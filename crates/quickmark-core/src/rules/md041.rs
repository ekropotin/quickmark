use serde::Deserialize;
use std::rc::Rc;

use regex::Regex;
use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, Context, RuleLinter, RuleViolation},
    rules::{Rule, RuleType},
};

// MD041-specific configuration types
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD041FirstLineHeadingTable {
    #[serde(default)]
    pub allow_preamble: bool,
    #[serde(default)]
    pub front_matter_title: String,
    #[serde(default)]
    pub level: u8,
}

impl Default for MD041FirstLineHeadingTable {
    fn default() -> Self {
        Self {
            allow_preamble: false,
            front_matter_title: r"^\s*title\s*[:=]".to_string(),
            level: 1,
        }
    }
}

#[derive(Debug)]
enum FirstElement {
    Heading(u8, tree_sitter::Range), // level, range
    Content(tree_sitter::Range),
    None,
}

pub(crate) struct MD041Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
    first_element: FirstElement,
    front_matter_end_byte: Option<usize>,
    title_regex: Option<Regex>,
}

impl MD041Linter {
    pub fn new(context: Rc<Context>) -> Self {
        let content = context.get_document_content();
        let front_matter_end_byte = Self::calculate_front_matter_end_byte(&content);

        let config = &context.config.linters.settings.first_line_heading;
        let title_regex = if !config.front_matter_title.is_empty() {
            Some(
                Regex::new(&config.front_matter_title)
                    .unwrap_or_else(|_| Regex::new(r"^\s*title\s*[:=]").unwrap()),
            )
        } else {
            None
        };

        Self {
            context: context.clone(),
            violations: Vec::new(),
            first_element: FirstElement::None,
            front_matter_end_byte,
            title_regex,
        }
    }

    /// Calculates the end byte of the front matter, including the final newline.
    /// This is done by iterating through the lines of the content.
    fn calculate_front_matter_end_byte(content: &str) -> Option<usize> {
        if !content.starts_with("---") {
            return None;
        }

        let mut byte_pos = 0;
        let mut found_start = false;

        let mut remaining = content;
        while let Some(newline_pos) = remaining.find('\n') {
            let line = &remaining[..newline_pos];
            let line_to_check = line.trim_end_matches('\r');

            if line_to_check.trim() == "---" {
                if !found_start {
                    found_start = true;
                } else {
                    return Some(byte_pos + newline_pos + 1);
                }
            }
            byte_pos += newline_pos + 1;
            remaining = &remaining[newline_pos + 1..];
        }

        // Check last line if no newline at end
        if !remaining.is_empty() && remaining.trim() == "---" && found_start {
            return Some(content.len());
        }

        None
    }

    fn extract_heading_level(&self, node: &Node) -> u8 {
        match node.kind() {
            "atx_heading" => {
                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();
                    let kind = child.kind();
                    if kind.starts_with("atx_h") && kind.ends_with("_marker") {
                        let level_str = &kind["atx_h".len()..kind.len() - "_marker".len()];
                        return level_str.parse::<u8>().unwrap_or(1);
                    }
                }
                1 // fallback
            }
            "setext_heading" => {
                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();
                    if child.kind() == "setext_h1_underline" {
                        return 1;
                    } else if child.kind() == "setext_h2_underline" {
                        return 2;
                    }
                }
                1 // fallback
            }
            _ => 1,
        }
    }

    fn check_front_matter_has_title(&self) -> bool {
        let Some(title_regex) = &self.title_regex else {
            return false; // Front matter title checking disabled
        };

        let Some(fm_end) = self.front_matter_end_byte else {
            return false; // No front matter found
        };

        let content = self.context.get_document_content();
        let front_matter_content = &content[..fm_end];

        front_matter_content
            .lines()
            .skip(1) // Skip the initial "---"
            .take_while(|line| line.trim() != "---")
            .any(|line| title_regex.is_match(line))
    }

    fn is_html_comment(&self, node: &Node) -> bool {
        if node.kind() == "html_flow" {
            let source = self.context.get_document_content();
            let content = &source[node.start_byte()..node.end_byte()];
            content.trim_start().starts_with("<!--")
        } else {
            false
        }
    }

    fn is_in_front_matter(&self, node: &Node) -> bool {
        if let Some(fm_end) = self.front_matter_end_byte {
            node.start_byte() < fm_end
        } else {
            false
        }
    }

    fn should_ignore_node(&self, node: &Node) -> bool {
        // Ignore front matter nodes
        if self.is_in_front_matter(node) {
            return true;
        }

        // Ignore HTML comments
        if self.is_html_comment(node) {
            return true;
        }

        false
    }

    fn is_content_node(&self, node: &Node) -> bool {
        matches!(
            node.kind(),
            "paragraph"
                | "list"
                | "list_item"
                | "code_block"
                | "fenced_code_block"
                | "blockquote"
                | "table"
                | "thematic_break"
        )
    }
}

impl RuleLinter for MD041Linter {
    fn feed(&mut self, node: &Node) {
        // Skip if we already processed the first element
        if !matches!(self.first_element, FirstElement::None) {
            return;
        }

        // Skip nodes that should be ignored
        if self.should_ignore_node(node) {
            return;
        }

        // Check if this is a heading
        if node.kind() == "atx_heading" || node.kind() == "setext_heading" {
            let level = self.extract_heading_level(node);
            self.first_element = FirstElement::Heading(level, node.range());
            return;
        }

        // Check if this is content
        if self.is_content_node(node) {
            self.first_element = FirstElement::Content(node.range());
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        // Check if front matter has title - if so, no violation
        if self.check_front_matter_has_title() {
            return Vec::new();
        }

        let config = &self.context.config.linters.settings.first_line_heading;

        match &self.first_element {
            FirstElement::Heading(level, range) => {
                // First element is a heading - check if it has the correct level
                if *level != config.level {
                    self.violations.push(RuleViolation::new(
                        &MD041,
                        format!(
                            "Expected first heading to be level {}, but found level {}",
                            config.level, level
                        ),
                        self.context.file_path.clone(),
                        range_from_tree_sitter(range),
                    ));
                }
            }
            FirstElement::Content(range) => {
                // First element is content - only a violation if preamble is not allowed
                if !config.allow_preamble {
                    self.violations.push(RuleViolation::new(
                        &MD041,
                        "First line in a file should be a top-level heading".to_string(),
                        self.context.file_path.clone(),
                        range_from_tree_sitter(range),
                    ));
                }
            }
            FirstElement::None => {
                // No content found - this is valid (empty document)
            }
        }

        std::mem::take(&mut self.violations)
    }
}

pub const MD041: Rule = Rule {
    id: "MD041",
    alias: "first-line-heading",
    tags: &["headings"],
    description: "First line in a file should be a top-level heading",
    rule_type: RuleType::Document,
    required_nodes: &[
        "atx_heading",
        "setext_heading",
        "paragraph",
        "list",
        "list_item",
        "code_block",
        "fenced_code_block",
        "blockquote",
        "table",
        "thematic_break",
    ],
    new_linter: |context| Box::new(MD041Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{LintersSettingsTable, MD041FirstLineHeadingTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_settings;

    fn test_config(
        level: u8,
        front_matter_title: &str,
        allow_preamble: bool,
    ) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![("first-line-heading", RuleSeverity::Error)],
            LintersSettingsTable {
                first_line_heading: MD041FirstLineHeadingTable {
                    level,
                    front_matter_title: front_matter_title.to_string(),
                    allow_preamble,
                },
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_valid_first_line_heading() {
        let config = test_config(1, r"^\s*title\s*[:=]", false);
        let input = "# Title

Some content

## Section 1

Content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_no_first_line_heading() {
        let config = test_config(1, r"^\s*title\s*[:=]", false);
        let input = "This is some text

# Title

Content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0]
            .message()
            .contains("First line in a file should be a top-level heading"));
    }

    #[test]
    fn test_wrong_level_first_heading() {
        let config = test_config(1, r"^\s*title\s*[:=]", false);
        let input = "## Title

Content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0]
            .message()
            .contains("Expected first heading to be level 1, but found level 2"));
    }

    #[test]
    fn test_custom_level() {
        let config = test_config(2, r"^\s*title\s*[:=]", false);
        let input = "## Title

Content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_custom_level_wrong_level() {
        let config = test_config(2, r"^\s*title\s*[:=]", false);
        let input = "# Title

Content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0]
            .message()
            .contains("Expected first heading to be level 2, but found level 1"));
    }

    #[test]
    fn test_setext_heading_valid() {
        let config = test_config(1, r"^\s*title\s*[:=]", false);
        let input = "Title
=====

Content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_setext_heading_wrong_level() {
        let config = test_config(1, r"^\s*title\s*[:=]", false);
        let input = "Title
-----

Content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0]
            .message()
            .contains("Expected first heading to be level 1, but found level 2"));
    }

    #[test]
    fn test_allow_preamble_true() {
        let config = test_config(1, r"^\s*title\s*[:=]", true);
        let input = "This is some preamble text

# Title

Content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_allow_preamble_false() {
        let config = test_config(1, r"^\s*title\s*[:=]", false);
        let input = "This is some preamble text

# Title

Content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0]
            .message()
            .contains("First line in a file should be a top-level heading"));
    }

    #[test]
    fn test_front_matter_with_title() {
        let config = test_config(1, r"^\s*title\s*[:=]", false);
        let input = "---
layout: post
title: \"Welcome to Jekyll!\"
date: 2015-11-17 16:16:01 -0600
---

This is content without a heading";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_front_matter_without_title() {
        let config = test_config(1, r"^\s*title\s*[:=]", false);
        let input = "---
layout: post
author: John Doe
date: 2015-11-17 16:16:01 -0600
---

This is content without a heading";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_front_matter_title_disabled() {
        let config = test_config(1, "", false); // Empty pattern disables front matter checking
        let input = "---
title: \"Welcome to Jekyll!\"
---

This is content without a heading";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_custom_front_matter_title_regex() {
        let config = test_config(1, r"^\s*heading\s*:", false);
        let input = "---
layout: post
heading: \"My Custom Title\"
---

This is content without a heading";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_comments_before_heading() {
        let config = test_config(1, r"^\s*title\s*[:=]", false);
        let input = "<!-- This is a comment -->

# Title

Content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_empty_document() {
        let config = test_config(1, r"^\s*title\s*[:=]", false);
        let input = "";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_whitespace_only() {
        let config = test_config(1, r"^\s*title\s*[:=]", false);
        let input = "   \n\n  \n\n# Title\n\nContent";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }
}
