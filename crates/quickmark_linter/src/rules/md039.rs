use std::rc::Rc;

use once_cell::sync::Lazy;
use regex::Regex;
use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

// Using once_cell::sync::Lazy for safe, one-time compilation of regexes.
// Regular inline links: [text](url) - but NOT images ![text](url)
static RE_INLINE_LINK: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?:^|[^!])\[([^\]]*)\]\(([^)]+)\)").unwrap());

// Reference links: [text][ref] - but NOT images ![text][ref]
static RE_REF_LINK: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?:^|[^!])\[([^\]]*)\]\[([^\]]+)\]").unwrap());

// Collapsed reference links: [text][] - but NOT images ![text][]
static RE_COLLAPSED_REF_LINK: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?:^|[^!])\[([^\]]+)\]\[\]").unwrap());

/// MD039 - Spaces inside link text
///
/// This rule checks for unnecessary spaces at the beginning or end of link text.
pub(crate) struct MD039Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD039Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }
}

impl RuleLinter for MD039Linter {
    fn feed(&mut self, node: &Node) {
        // Process different possible link node types
        if node.kind() == "link" {
            self.check_link_for_spaces(node);
        } else if node.kind() == "inline" {
            // Check if this inline node contains links
            self.check_inline_for_links(node);
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

impl MD039Linter {
    fn check_inline_for_links(&mut self, inline_node: &Node) {
        // Look for links within inline content using the text
        let link_text = {
            let document_content = self.context.document_content.borrow();
            inline_node
                .utf8_text(document_content.as_bytes())
                .unwrap_or("")
                .to_string()
        };

        // Parse the inline content for markdown links
        // Look for patterns like [text](url), [text][ref], [ref][], [ref]
        self.check_text_for_link_patterns(&link_text, inline_node);
    }

    fn check_text_for_link_patterns(&mut self, text: &str, node: &Node) {
        for caps in RE_INLINE_LINK.captures_iter(text) {
            if let Some(label_match) = caps.get(1) {
                let label_text = label_match.as_str();
                self.check_label_for_spaces(label_text, node);
            }
        }

        for caps in RE_REF_LINK.captures_iter(text) {
            if let Some(label_match) = caps.get(1) {
                let label_text = label_match.as_str();
                self.check_label_for_spaces(label_text, node);
            }
        }

        for caps in RE_COLLAPSED_REF_LINK.captures_iter(text) {
            if let Some(label_match) = caps.get(1) {
                let label_text = label_match.as_str();
                self.check_label_for_spaces(label_text, node);
            }
        }

        // Shortcut reference links: [text] - but only if there's a matching reference definition
        // We need to be careful here to not match arbitrary brackets
        // For now, let's only process shortcut links in specific contexts or skip them
        // since they require document-level analysis to verify the reference exists
    }

    fn check_label_for_spaces(&mut self, label_text: &str, node: &Node) {
        // Check for leading spaces
        if label_text.len() != label_text.trim_start().len() {
            self.create_space_violation(node, true);
        }

        // Check for trailing spaces
        if label_text.len() != label_text.trim_end().len() {
            self.create_space_violation(node, false);
        }
    }

    fn check_link_for_spaces(&mut self, link_node: &Node) {
        // Look for the link text within the link node
        // In tree-sitter markdown, links have different structures
        // We need to find the text content and check for leading/trailing spaces

        let link_text = {
            let document_content = self.context.document_content.borrow();
            link_node
                .utf8_text(document_content.as_bytes())
                .unwrap_or("")
                .to_string()
        };

        // Find the bracket part [text] in the link
        if let Some(bracket_start) = link_text.find('[') {
            if let Some(bracket_end) = link_text.find(']') {
                if bracket_end > bracket_start {
                    let label_text = &link_text[bracket_start + 1..bracket_end];

                    // Check for leading spaces
                    if label_text.len() != label_text.trim_start().len() {
                        self.create_space_violation(link_node, true);
                    }

                    // Check for trailing spaces
                    if label_text.len() != label_text.trim_end().len() {
                        self.create_space_violation(link_node, false);
                    }
                }
            }
        }
    }

    fn create_space_violation(&mut self, node: &Node, is_leading: bool) {
        let space_type = if is_leading { "leading" } else { "trailing" };
        let message = format!("Spaces inside link text ({space_type})");

        self.violations.push(RuleViolation::new(
            &MD039,
            message,
            self.context.file_path.clone(),
            range_from_tree_sitter(&node.range()),
        ));
    }
}

pub const MD039: Rule = Rule {
    id: "MD039",
    alias: "no-space-in-links",
    tags: &["whitespace", "links"],
    description: "Spaces inside link text",
    rule_type: RuleType::Token,
    required_nodes: &["link", "inline"], // We need link nodes to check for spaces in link text
    new_linter: |context| Box::new(MD039Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![
            ("no-space-in-links", RuleSeverity::Error),
            ("heading-style", RuleSeverity::Off),
            ("heading-increment", RuleSeverity::Off),
            ("line-length", RuleSeverity::Off),
        ])
    }

    #[test]
    fn test_no_spaces_in_link_text() {
        let input = "[link text](https://example.com)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_leading_space_in_link_text() {
        let input = "[ link text](https://example.com)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD039", violation.rule().id);
        assert!(violation.message().contains("Spaces inside link text"));
    }

    #[test]
    fn test_trailing_space_in_link_text() {
        let input = "[link text ](https://example.com)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD039", violation.rule().id);
        assert!(violation.message().contains("Spaces inside link text"));
    }

    #[test]
    fn test_both_leading_and_trailing_spaces() {
        let input = "[ link text ](https://example.com)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should report both leading and trailing space violations
        assert_eq!(2, violations.len());
        for violation in &violations {
            assert_eq!("MD039", violation.rule().id);
            assert!(violation.message().contains("Spaces inside link text"));
        }
    }

    #[test]
    fn test_reference_link_with_spaces() {
        let input = "[ link text ][ref]\n\n[ref]: https://example.com";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect spaces in reference link text
        assert_eq!(2, violations.len());
        for violation in &violations {
            assert_eq!("MD039", violation.rule().id);
        }
    }

    #[test]
    fn test_shortcut_reference_link_with_spaces() {
        let input = "[ link text ][]\n\n[link text]: https://example.com";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect spaces in collapsed reference link
        assert_eq!(2, violations.len());
        for violation in &violations {
            assert_eq!("MD039", violation.rule().id);
        }
    }

    #[test]
    fn test_image_not_affected() {
        let input = "![ image alt text ](image.jpg)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Images should not be affected by this rule
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_empty_link_text_with_spaces() {
        let input = "[ ](https://example.com)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect spaces in empty link text
        assert_eq!(2, violations.len());
        for violation in &violations {
            assert_eq!("MD039", violation.rule().id);
        }
    }

    #[test]
    fn test_multiple_links() {
        let input = "[good link](url1) and [ bad link ](url2) and [another good](url3)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should only detect violations in the bad link
        assert_eq!(2, violations.len());
        for violation in &violations {
            assert_eq!("MD039", violation.rule().id);
        }
    }
}
