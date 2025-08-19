use serde::Deserialize;
use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, Context, RuleLinter, RuleViolation},
    rules::{Rule, RuleType},
};

// MD025-specific configuration types
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD025SingleH1Table {
    #[serde(default)]
    pub level: u8,
    #[serde(default)]
    pub front_matter_title: String,
}

impl Default for MD025SingleH1Table {
    fn default() -> Self {
        Self {
            level: 1,
            front_matter_title: r"^\s*title\s*[:=]".to_string(),
        }
    }
}

#[derive(Debug)]
struct HeadingInfo {
    content: String,
    range: tree_sitter::Range,
    is_first_content_heading: bool,
}

pub(crate) struct MD025Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
    matching_headings: Vec<HeadingInfo>,
    has_front_matter_title: Option<bool>,
}

impl MD025Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
            matching_headings: Vec::new(),
            has_front_matter_title: None,
        }
    }

    fn extract_heading_level(&self, node: &Node) -> u8 {
        match node.kind() {
            "atx_heading" => {
                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();
                    if child.kind().starts_with("atx_h") && child.kind().ends_with("_marker") {
                        return child.kind().chars().nth(5).unwrap().to_digit(10).unwrap() as u8;
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

    fn extract_heading_content(&self, node: &Node) -> String {
        let source = self.context.get_document_content();
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let full_text = &source[start_byte..end_byte];

        match node.kind() {
            "atx_heading" => full_text
                .trim_start_matches('#')
                .trim()
                .trim_end_matches('#')
                .trim()
                .to_string(),
            "setext_heading" => {
                if let Some(line) = full_text.lines().next() {
                    line.trim().to_string()
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }

    fn check_front_matter_has_title(&mut self) -> bool {
        if self.has_front_matter_title.is_some() {
            return self.has_front_matter_title.unwrap();
        }

        let config = &self.context.config.linters.settings.single_h1;
        if config.front_matter_title.is_empty() {
            self.has_front_matter_title = Some(false);
            return false; // Front matter checking disabled
        }

        let content = self.context.get_document_content();

        // Check if document starts with front matter (---)
        if !content.starts_with("---") {
            self.has_front_matter_title = Some(false);
            return false;
        }

        // Find the end of front matter
        let lines: Vec<&str> = content.lines().collect();
        if lines.len() < 3 {
            self.has_front_matter_title = Some(false);
            return false; // Too short to have valid front matter
        }

        let mut end_index = None;
        for (i, line) in lines.iter().enumerate().skip(1) {
            if line.trim() == "---" {
                end_index = Some(i);
                break;
            }
        }

        let end_index = match end_index {
            Some(idx) => idx,
            None => {
                self.has_front_matter_title = Some(false);
                return false; // No closing front matter delimiter
            }
        };

        // Check for title in front matter
        let front_matter_lines = &lines[1..end_index];
        let title_regex = regex::Regex::new(&config.front_matter_title).unwrap_or_else(|_| {
            // Fallback to default regex if invalid
            regex::Regex::new(r"^\s*title\s*[:=]").unwrap()
        });

        let has_title = front_matter_lines
            .iter()
            .any(|line| title_regex.is_match(line));
        self.has_front_matter_title = Some(has_title);
        has_title
    }

    fn is_first_content_heading(&self, node: &Node) -> bool {
        let content = self.context.get_document_content();
        let node_start_byte = node.start_byte();
        let target_level = self.context.config.linters.settings.single_h1.level;

        // Get text before this heading
        let text_before = &content[..node_start_byte];

        // Check if there's only whitespace, comments, front matter,
        // or headings above the target level before this heading
        let mut in_front_matter = false;

        for line in text_before.lines() {
            let trimmed = line.trim();

            if trimmed == "---" {
                if !in_front_matter {
                    in_front_matter = true;
                    continue;
                } else {
                    // End of front matter
                    in_front_matter = false;
                    continue;
                }
            }

            if in_front_matter {
                continue; // Skip front matter content
            }

            // Check if this line is a heading above target level
            if trimmed.starts_with('#') {
                let heading_level = trimmed.chars().take_while(|&c| c == '#').count() as u8;
                if heading_level < target_level {
                    continue; // Ignore headings above target level
                }
                if heading_level == target_level {
                    // Found another heading at target level before this one
                    return false;
                }
                // Headings below target level count as content
                return false;
            }

            // Check for setext headings
            if trimmed.chars().all(|c| c == '=' || c == '-') && !trimmed.is_empty() {
                // This might be a setext underline - need to check previous line for content
                // For simplicity, we'll consider all setext underlines as potential headings
                let setext_level = if trimmed.chars().all(|c| c == '=') {
                    1
                } else {
                    2
                };
                if setext_level < target_level {
                    continue; // Ignore headings above target level
                }
                return false; // Setext heading at or below target level
            }

            // After front matter is closed or if no front matter
            if !trimmed.is_empty() && !trimmed.starts_with("<!--") && !trimmed.starts_with("-->") {
                // Found non-whitespace, non-comment, non-heading content before heading
                return false;
            }
        }

        true
    }
}

impl RuleLinter for MD025Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "atx_heading" || node.kind() == "setext_heading" {
            let level = self.extract_heading_level(node);
            let config = &self.context.config.linters.settings.single_h1;

            if level != config.level {
                return; // Not the level we're checking
            }

            let content = self.extract_heading_content(node);
            let is_first_content = self.is_first_content_heading(node);

            // Store the heading info for processing in finalize
            self.matching_headings.push(HeadingInfo {
                content,
                range: node.range(),
                is_first_content_heading: is_first_content,
            });
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        if self.matching_headings.is_empty() {
            return Vec::new();
        }

        let has_front_matter_title = self.check_front_matter_has_title();

        // Determine if we have a "top-level heading" scenario
        let has_top_level_heading = has_front_matter_title
            || (!self.matching_headings.is_empty()
                && self.matching_headings[0].is_first_content_heading);

        if has_top_level_heading {
            // Determine which headings are violations
            let start_index = if has_front_matter_title { 0 } else { 1 };

            for heading in self.matching_headings.iter().skip(start_index) {
                self.violations.push(RuleViolation::new(
                    &MD025,
                    format!("{} [{}]", MD025.description, heading.content),
                    self.context.file_path.clone(),
                    range_from_tree_sitter(&heading.range),
                ));
            }
        }

        std::mem::take(&mut self.violations)
    }
}

pub const MD025: Rule = Rule {
    id: "MD025",
    alias: "single-h1",
    tags: &["headings"],
    description: "Multiple top-level headings in the same document",
    rule_type: RuleType::Document,
    required_nodes: &["atx_heading", "setext_heading"],
    new_linter: |context| Box::new(MD025Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{LintersSettingsTable, MD025SingleH1Table, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_settings;

    fn test_config(level: u8, front_matter_title: &str) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![("single-h1", RuleSeverity::Error)],
            LintersSettingsTable {
                single_h1: MD025SingleH1Table {
                    level,
                    front_matter_title: front_matter_title.to_string(),
                },
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_single_h1_no_violations() {
        let config = test_config(1, r"^\s*title\s*[:=]");
        let input = "# Title

Some content

## Section 1

Content

## Section 2

More content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_multiple_h1_violations() {
        let config = test_config(1, r"^\s*title\s*[:=]");
        let input = "# First Title

Some content

# Second Title

More content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Second Title"));
    }

    #[test]
    fn test_front_matter_with_title_and_h1() {
        let config = test_config(1, r"^\s*title\s*[:=]");
        let input = "---
layout: post
title: \"Welcome to Jekyll!\"
date: 2015-11-17 16:16:01 -0600
---
# Top level heading

Content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Top level heading"));
    }

    #[test]
    fn test_front_matter_without_title() {
        let config = test_config(1, r"^\s*title\s*[:=]");
        let input = "---
layout: post
author: John Doe
date: 2015-11-17 16:16:01 -0600
---
# Title

Content

## Section";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_custom_level() {
        let config = test_config(2, r"^\s*title\s*[:=]");
        let input = "# Title (level 1, should be ignored)

## First H2

Content

## Second H2

More content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Second H2"));
    }

    #[test]
    fn test_setext_headings() {
        let config = test_config(1, r"^\s*title\s*[:=]");
        let input = "First Title
===========

Content

Second Title
============

More content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Second Title"));
    }

    #[test]
    fn test_mixed_heading_styles() {
        let config = test_config(1, r"^\s*title\s*[:=]");
        let input = "First Title
===========

Content

# Second Title

More content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Second Title"));
    }

    #[test]
    fn test_h1_not_first_content() {
        let config = test_config(1, r"^\s*title\s*[:=]");
        let input = "Some intro paragraph

# Title

Content

# Another Title";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // No violations because first H1 is not the first content
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_front_matter_title_disabled() {
        let config = test_config(1, ""); // Empty pattern disables front matter checking
        let input = "---
title: \"Welcome to Jekyll!\"
---
# Top level heading

Content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_custom_front_matter_title_regex() {
        let config = test_config(1, r"^\s*heading\s*:");
        let input = "---
layout: post
heading: \"My Custom Title\"
---
# Top level heading

Content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Top level heading"));
    }

    #[test]
    fn test_comments_before_heading() {
        let config = test_config(1, r"^\s*title\s*[:=]");
        let input = "<!-- This is a comment -->

# Title

Content

# Another Title";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Another Title"));
    }

    #[test]
    fn test_empty_document() {
        let config = test_config(1, r"^\s*title\s*[:=]");
        let input = "";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_only_lower_level_headings() {
        let config = test_config(1, r"^\s*title\s*[:=]");
        let input = "## Section 1

Content

### Subsection

More content

## Section 2

Final content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }
}
