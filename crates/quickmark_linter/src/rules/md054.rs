use serde::Deserialize;
use std::collections::HashSet;
use std::rc::Rc;

use once_cell::sync::Lazy;
use regex::Regex;
use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

// MD054-specific configuration types
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD054LinkImageStyleTable {
    #[serde(default)]
    pub autolink: bool,
    #[serde(default)]
    pub inline: bool,
    #[serde(default)]
    pub full: bool,
    #[serde(default)]
    pub collapsed: bool,
    #[serde(default)]
    pub shortcut: bool,
    #[serde(default)]
    pub url_inline: bool,
}

impl Default for MD054LinkImageStyleTable {
    fn default() -> Self {
        Self {
            autolink: true,
            inline: true,
            full: true,
            collapsed: true,
            shortcut: true,
            url_inline: true,
        }
    }
}

// Combined regular expressions for detecting different link and image styles.
// This improves performance by reducing the number of passes over the text.
// Groups are used to differentiate between image and link matches.

// Style: [text](url)
static RE_INLINE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(!\[([^\]]*)\]\(([^)]*)\))|((?:^|[^!])\[([^\]]*)\]\(([^)]*)\))").unwrap()
});

// Style: [text][ref]
static RE_FULL_REFERENCE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(!\[([^\]]*)\]\[([^\]]+)\])|((?:^|[^!])\[([^\]]*)\]\[([^\]]+)\])").unwrap()
});

// Style: [ref][]
static RE_COLLAPSED_REFERENCE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(!\[([^\]]+)\]\[\])|((?:^|[^!])\[([^\]]+)\]\[\])").unwrap());

// Style: [ref]
static RE_SHORTCUT_REFERENCE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(!\[([^\]]+)\])|((?:^|[^!])\[([^\]]+)\])").unwrap());

// Style: <url>
static RE_AUTOLINK: Lazy<Regex> = Lazy::new(|| Regex::new(r"<(https?://[^>]+)>").unwrap());

/// MD054 - Link and image style
///
/// This rule controls which styles of links and images are allowed in the document.
pub(crate) struct MD054Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD054Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }
}

impl RuleLinter for MD054Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "inline" {
            self.check_inline_content(node);
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

impl MD054Linter {
    fn check_inline_content(&mut self, node: &Node) {
        let content = {
            let document_content = self.context.document_content.borrow();
            node.utf8_text(document_content.as_bytes())
                .unwrap_or("")
                .to_string()
        };

        if !content.is_empty() {
            self.check_content_for_violations(&content, node);
        }
    }

    fn check_content_for_violations(&mut self, content: &str, node: &Node) {
        let config = self
            .context
            .config
            .linters
            .settings
            .link_image_style
            .clone();
        let mut found_violations = HashSet::new();

        // Check for autolinks
        if !config.autolink {
            for caps in RE_AUTOLINK.captures_iter(content) {
                let start = caps.get(0).unwrap().start();
                if found_violations.insert(("autolink", start)) {
                    self.create_violation_at_offset(
                        node,
                        content,
                        start,
                        "Autolinks are not allowed".to_string(),
                    );
                }
            }
        }

        // Check for inline style
        for caps in RE_INLINE.captures_iter(content) {
            // Group 1: Image match `![]()`
            if let Some(image_match) = caps.get(1) {
                if !config.inline && found_violations.insert(("inline_image", image_match.start()))
                {
                    self.create_violation_at_offset(
                        node,
                        content,
                        image_match.start(),
                        "Inline images are not allowed".to_string(),
                    );
                }
            }
            // Group 4: Link match `[]()`
            else if let Some(link_match) = caps.get(4) {
                let mut start = link_match.start();
                if !link_match.as_str().starts_with('[') {
                    start += 1; // Adjust for `(?:^|[^!])`
                }

                if !config.inline {
                    if found_violations.insert(("inline_link", start)) {
                        self.create_violation_at_offset(
                            node,
                            content,
                            start,
                            "Inline links are not allowed".to_string(),
                        );
                    }
                    continue; // If disallowed, no need for further checks
                }

                // Check for url_inline: [https://...](https://...)
                if !config.url_inline {
                    if let (Some(text), Some(url)) = (caps.get(5), caps.get(6)) {
                        if text.as_str() == url.as_str()
                            && found_violations.insert(("url_inline", start))
                        {
                            self.create_violation_at_offset(
                                node,
                                content,
                                start,
                                "Inline links with matching URL text are not allowed".to_string(),
                            );
                        }
                    }
                }
            }
        }

        // Check for full reference style
        if !config.full {
            for caps in RE_FULL_REFERENCE.captures_iter(content) {
                if let Some(image_match) = caps.get(1) {
                    if found_violations.insert(("full_image", image_match.start())) {
                        self.create_violation_at_offset(
                            node,
                            content,
                            image_match.start(),
                            "Full reference images are not allowed".to_string(),
                        );
                    }
                } else if let Some(link_match) = caps.get(4) {
                    let mut start = link_match.start();
                    if !link_match.as_str().starts_with('[') {
                        start += 1;
                    }
                    if found_violations.insert(("full_link", start)) {
                        self.create_violation_at_offset(
                            node,
                            content,
                            start,
                            "Full reference links are not allowed".to_string(),
                        );
                    }
                }
            }
        }

        // Check for collapsed reference style
        if !config.collapsed {
            for caps in RE_COLLAPSED_REFERENCE.captures_iter(content) {
                if let Some(image_match) = caps.get(1) {
                    if found_violations.insert(("collapsed_image", image_match.start())) {
                        self.create_violation_at_offset(
                            node,
                            content,
                            image_match.start(),
                            "Collapsed reference images are not allowed".to_string(),
                        );
                    }
                } else if let Some(link_match) = caps.get(3) {
                    let mut start = link_match.start();
                    if !link_match.as_str().starts_with('[') {
                        start += 1;
                    }
                    if found_violations.insert(("collapsed_link", start)) {
                        self.create_violation_at_offset(
                            node,
                            content,
                            start,
                            "Collapsed reference links are not allowed".to_string(),
                        );
                    }
                }
            }
        }

        // Check for shortcut reference style
        if !config.shortcut {
            for caps in RE_SHORTCUT_REFERENCE.captures_iter(content) {
                let whole_match = caps.get(0).unwrap();
                let end_offset = whole_match.end();

                // Check character after match to avoid false positives for other link types
                if end_offset < content.len() {
                    if let Some(next_char) = content[end_offset..].chars().next() {
                        if next_char == '(' || next_char == '[' {
                            continue;
                        }
                    }
                }

                if let Some(image_match) = caps.get(1) {
                    if found_violations.insert(("shortcut_image", image_match.start())) {
                        self.create_violation_at_offset(
                            node,
                            content,
                            image_match.start(),
                            "Shortcut reference images are not allowed".to_string(),
                        );
                    }
                } else if let Some(link_match) = caps.get(3) {
                    let mut start = link_match.start();
                    if !link_match.as_str().starts_with('[') {
                        start += 1;
                    }
                    if found_violations.insert(("shortcut_link", start)) {
                        self.create_violation_at_offset(
                            node,
                            content,
                            start,
                            "Shortcut reference links are not allowed".to_string(),
                        );
                    }
                }
            }
        }
    }

    fn create_violation_at_offset(
        &mut self,
        node: &Node,
        content: &str,
        offset: usize,
        message: String,
    ) {
        // Calculate the line number within the content where the violation occurs
        let lines_before_offset = content[..offset].matches('\n').count();
        let node_start_row = node.start_position().row;
        let violation_row = node_start_row + lines_before_offset;

        // Create a custom range for this specific violation
        let violation_range = tree_sitter::Range {
            start_byte: node.start_byte() + offset,
            end_byte: node.start_byte() + offset + 1, // Just mark the start of the violation
            start_point: tree_sitter::Point {
                row: violation_row,
                column: if lines_before_offset == 0 {
                    node.start_position().column + offset
                } else {
                    // Calculate column position for this line
                    let line_start = content[..offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    offset - line_start
                },
            },
            end_point: tree_sitter::Point {
                row: violation_row,
                column: if lines_before_offset == 0 {
                    node.start_position().column + offset + 1
                } else {
                    let line_start = content[..offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    offset - line_start + 1
                },
            },
        };

        self.violations.push(RuleViolation::new(
            &MD054,
            message,
            self.context.file_path.clone(),
            range_from_tree_sitter(&violation_range),
        ));
    }
}

pub const MD054: Rule = Rule {
    id: "MD054",
    alias: "link-image-style",
    tags: &["links", "images"],
    description: "Link and image style",
    rule_type: RuleType::Token,
    required_nodes: &["inline"],
    new_linter: |context| Box::new(MD054Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{MD054LinkImageStyleTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![
            ("link-image-style", RuleSeverity::Error),
            ("heading-style", RuleSeverity::Off),
            ("heading-increment", RuleSeverity::Off),
            ("line-length", RuleSeverity::Off),
        ])
    }

    fn test_config_with_settings(
        settings: MD054LinkImageStyleTable,
    ) -> crate::config::QuickmarkConfig {
        let mut config = test_config();
        config.linters.settings.link_image_style = settings;
        config
    }

    // Test cases for autolinks
    #[test]
    fn test_autolink_allowed() {
        let input = "<https://example.com>";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            autolink: true,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_autolink_disallowed() {
        let input = "<https://example.com>";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            autolink: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD054", violation.rule().id);
        assert!(violation.message().to_lowercase().contains("autolink"));
    }

    // Test cases for inline links
    #[test]
    fn test_inline_link_allowed() {
        let input = "[example](https://example.com)";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            inline: true,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_inline_link_disallowed() {
        let input = "[example](https://example.com)";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            inline: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD054", violation.rule().id);
        assert!(violation.message().to_lowercase().contains("inline"));
    }

    // Test cases for inline images
    #[test]
    fn test_inline_image_allowed() {
        let input = "![alt text](https://example.com/image.jpg)";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            inline: true,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_inline_image_disallowed() {
        let input = "![alt text](https://example.com/image.jpg)";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            inline: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD054", violation.rule().id);
        assert!(violation.message().to_lowercase().contains("inline"));
    }

    // Test cases for full reference links
    #[test]
    fn test_full_reference_link_allowed() {
        let input = "[example][ref]\n\n[ref]: https://example.com";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            full: true,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_full_reference_link_disallowed() {
        let input = "[example][ref]\n\n[ref]: https://example.com";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            full: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD054", violation.rule().id);
        assert!(violation
            .message()
            .to_lowercase()
            .contains("full reference"));
    }

    // Test cases for full reference images
    #[test]
    fn test_full_reference_image_allowed() {
        let input = "![alt text][ref]\n\n[ref]: https://example.com/image.jpg";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            full: true,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_full_reference_image_disallowed() {
        let input = "![alt text][ref]\n\n[ref]: https://example.com/image.jpg";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            full: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD054", violation.rule().id);
        assert!(violation
            .message()
            .to_lowercase()
            .contains("full reference"));
    }

    // Test cases for collapsed reference links
    #[test]
    fn test_collapsed_reference_link_allowed() {
        let input = "[example][]\n\n[example]: https://example.com";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            collapsed: true,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_collapsed_reference_link_disallowed() {
        let input = "[example][]\n\n[example]: https://example.com";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            collapsed: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD054", violation.rule().id);
        assert!(violation
            .message()
            .to_lowercase()
            .contains("collapsed reference"));
    }

    // Test cases for collapsed reference images
    #[test]
    fn test_collapsed_reference_image_allowed() {
        let input = "![example][]\n\n[example]: https://example.com/image.jpg";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            collapsed: true,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_collapsed_reference_image_disallowed() {
        let input = "![example][]\n\n[example]: https://example.com/image.jpg";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            collapsed: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD054", violation.rule().id);
        assert!(violation
            .message()
            .to_lowercase()
            .contains("collapsed reference"));
    }

    // Test cases for shortcut reference links
    #[test]
    fn test_shortcut_reference_link_allowed() {
        let input = "[example]\n\n[example]: https://example.com";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            shortcut: true,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_shortcut_reference_link_disallowed() {
        let input = "[example]\n\n[example]: https://example.com";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            shortcut: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD054", violation.rule().id);
        assert!(violation
            .message()
            .to_lowercase()
            .contains("shortcut reference"));
    }

    // Test cases for shortcut reference images
    #[test]
    fn test_shortcut_reference_image_allowed() {
        let input = "![example]\n\n[example]: https://example.com/image.jpg";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            shortcut: true,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_shortcut_reference_image_disallowed() {
        let input = "![example]\n\n[example]: https://example.com/image.jpg";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            shortcut: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD054", violation.rule().id);
        assert!(violation
            .message()
            .to_lowercase()
            .contains("shortcut reference"));
    }

    // Test cases for url_inline
    #[test]
    fn test_url_inline_link_allowed() {
        let input = "[https://example.com](https://example.com)";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            url_inline: true,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_url_inline_link_disallowed() {
        let input = "[https://example.com](https://example.com)";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            url_inline: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD054", violation.rule().id);
        assert!(violation
            .message()
            .to_lowercase()
            .contains("matching url text"));
    }

    // Test multiple configuration options disabled
    #[test]
    fn test_multiple_styles_disallowed() {
        let input = r#"
[inline link](https://example.com)
<https://example.com>
[reference][ref]

[ref]: https://example.com
"#;
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            autolink: false,
            inline: false,
            full: false,
            collapsed: true,
            shortcut: true,
            url_inline: true,
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(3, violations.len()); // Should catch inline, autolink, and full reference
        for violation in &violations {
            assert_eq!("MD054", violation.rule().id);
        }
    }

    // Test all styles allowed (default)
    #[test]
    fn test_all_styles_allowed() {
        let input = r#"
[inline link](https://example.com)
<https://example.com>
[reference][ref]
[collapsed][]
[shortcut]
[https://example.com](https://example.com)

[ref]: https://example.com
[collapsed]: https://example.com
[shortcut]: https://example.com
"#;
        let config = test_config(); // Default config allows all
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    // Test line number accuracy for multiline content
    #[test]
    fn test_line_numbers_multiline_content() {
        let input = "Here is some text.\n\n[Link 1](https://example.com)\n\nSome more text here.\n\n[Link 2](https://github.com)";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            inline: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len());

        // Verify line numbers match exactly what original markdownlint reports
        assert_eq!(2, violations[0].location().range.start.line); // Line 3 (0-indexed)
        assert_eq!(6, violations[1].location().range.start.line); // Line 7 (0-indexed)
    }

    // Test bracket offset calculation with preceding text
    #[test]
    fn test_bracket_offset_with_preceding_text() {
        let input = "Text [link](url) more text";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            inline: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        // Should point to the '[' character, not any preceding character
        assert_eq!(5, violations[0].location().range.start.character); // Column should be at start of line (tree-sitter groups text)
    }

    // Test newline handling in regex patterns
    #[test]
    fn test_newline_before_link() {
        let input = "\n[Link text](https://example.com)\n[GitHub](https://github.com)";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            inline: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len());

        // Line numbers should be correct even with leading newlines
        assert_eq!(1, violations[0].location().range.start.line); // Second line (0-indexed)
        assert_eq!(2, violations[1].location().range.start.line); // Third line (0-indexed)
    }

    // Test multiple links on same line
    #[test]
    fn test_multiple_links_same_line() {
        let input = "[Link1](url1) and [Link2](url2)";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            inline: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len());

        // Both violations should be on line 1
        assert_eq!(0, violations[0].location().range.start.line);
        assert_eq!(0, violations[1].location().range.start.line);
    }

    // Test reference link bracket offset calculation
    #[test]
    fn test_reference_link_bracket_offset() {
        let input = "Text [ref link][reference] more";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            full: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        // Should correctly identify the full reference link
        assert!(violations[0].message().contains("Full reference"));
    }

    // Test collapsed reference bracket offset
    #[test]
    fn test_collapsed_reference_bracket_offset() {
        let input = "Text [collapsed][] more text";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            collapsed: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        // Should correctly identify the collapsed reference link
        assert!(violations[0].message().contains("Collapsed reference"));
    }

    // Test shortcut reference bracket offset
    #[test]
    fn test_shortcut_reference_bracket_offset() {
        let input = "Text [shortcut] more text\n\n[shortcut]: https://example.com";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            shortcut: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        // Should correctly identify the shortcut reference link
        assert!(violations[0].message().contains("Shortcut reference"));
    }

    // Test regex patterns that start with non-bracket characters
    #[test]
    fn test_regex_non_bracket_start() {
        let input = "!\n[Link after exclamation](url)";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            inline: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        // Should be reported on line 2 where the actual link is, not line 1 with the !
        assert_eq!(1, violations[0].location().range.start.line);
    }

    // Test line number accuracy parity with original markdownlint
    #[test]
    fn test_parity_line_numbers() {
        // This input matches what we tested against original markdownlint
        let input = "# MD054 Violations Test Cases\n\nThis file contains examples.\n\n<https://example.com>\n[Link text](https://example.com)";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            autolink: false,
            inline: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len());

        // These line numbers should match exactly what original markdownlint reports
        assert_eq!(4, violations[0].location().range.start.line); // Autolink on line 5 (0-indexed: 4)
        assert_eq!(5, violations[1].location().range.start.line); // Inline link on line 6 (0-indexed: 5)
    }

    // Test image bracket offset calculation (images don't use the (?:^|[^!]) pattern)
    #[test]
    fn test_image_bracket_offset() {
        let input = "Text ![alt](image.jpg) more text";
        let config = test_config_with_settings(MD054LinkImageStyleTable {
            inline: false,
            ..Default::default()
        });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        // Should correctly identify the inline image
        assert!(violations[0].message().contains("Inline images"));
        assert_eq!(0, violations[0].location().range.start.line);
    }
}
