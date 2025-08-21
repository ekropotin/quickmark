use std::rc::Rc;

use once_cell::sync::Lazy;
use regex::Regex;
use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

// Regular inline links: [text](url) - but NOT images ![text](url)
static RE_INLINE_LINK: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?:^|[^!])\[([^\]]*)\]\(([^)]*)\)").unwrap());

/// MD042 - No empty links
///
/// This rule checks for links that have no destination or only a fragment identifier.
pub(crate) struct MD042Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl RuleLinter for MD042Linter {
    fn feed(&mut self, node: &Node) {
        // Process different possible link node types
        match node.kind() {
            "link" => self.check_link_for_empty_destination(node),
            "inline" => self.check_inline_for_links(node),
            _ => {}
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

impl MD042Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    fn check_inline_for_links(&mut self, inline_node: &Node) {
        let link_text = {
            let document_content = self.context.document_content.borrow();
            inline_node
                .utf8_text(document_content.as_bytes())
                .unwrap_or_default()
                .to_string()
        };
        self.check_text_for_link_patterns(&link_text, inline_node);
    }

    fn check_text_for_link_patterns(&mut self, text: &str, node: &Node) {
        // Check inline links: [text](url)
        for caps in RE_INLINE_LINK.captures_iter(text) {
            if let Some(url_match) = caps.get(2) {
                if self.is_empty_link_destination(url_match.as_str()) {
                    self.create_empty_link_violation(node);
                }
            }
        }
    }

    fn check_link_for_empty_destination(&mut self, link_node: &Node) {
        let link_text = {
            let document_content = self.context.document_content.borrow();
            link_node
                .utf8_text(document_content.as_bytes())
                .unwrap_or_default()
                .to_string()
        };
        // Use the same regex-based checker for robustness and consistency
        self.check_text_for_link_patterns(&link_text, link_node);
    }

    fn is_empty_link_destination(&self, url: &str) -> bool {
        let trimmed = url.trim();
        trimmed.is_empty() || trimmed == "#"
    }

    fn create_empty_link_violation(&mut self, node: &Node) {
        self.violations.push(RuleViolation::new(
            &MD042,
            MD042.description.to_string(),
            self.context.file_path.clone(),
            range_from_tree_sitter(&node.range()),
        ));
    }
}

pub const MD042: Rule = Rule {
    id: "MD042",
    alias: "no-empty-links",
    tags: &["links"],
    description: "No empty links",
    rule_type: RuleType::Token,
    required_nodes: &["link", "inline"], // We need link nodes and inline nodes that might contain links
    new_linter: |context| Box::new(MD042Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![
            ("no-empty-links", RuleSeverity::Error),
            ("heading-style", RuleSeverity::Off),
            ("heading-increment", RuleSeverity::Off),
            ("line-length", RuleSeverity::Off),
        ])
    }

    #[test]
    fn test_valid_link() {
        let input = "[link text](https://example.com)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_empty_link_url() {
        let input = "[empty link]()";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD042", violation.rule().id);
        assert_eq!("No empty links", violation.message());
    }

    #[test]
    fn test_fragment_only_link() {
        let input = "[fragment only](#)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD042", violation.rule().id);
        assert_eq!("No empty links", violation.message());
    }

    #[test]
    fn test_valid_fragment_link() {
        let input = "[section link](#section)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_empty_reference_link() {
        let input = "[link text][]";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Reference links would need document-level analysis to verify if the reference exists
        // For now, we don't flag collapsed reference links as empty
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_image_not_affected() {
        let input = "![image alt]()";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Images should not be affected by this rule
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_multiple_links_with_one_empty() {
        let input = "[good link](https://example.com) and [empty link]() and [another good](https://other.com)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should only detect the empty link
        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD042", violation.rule().id);
    }

    #[test]
    fn test_mixed_empty_links() {
        let input = "[empty1]() and [fragment](#) and [valid](https://example.com)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect both empty links
        assert_eq!(2, violations.len());
        for violation in &violations {
            assert_eq!("MD042", violation.rule().id);
        }
    }

    #[test]
    fn test_sequential_links_bug_prevention() {
        // This test is based on issue #308 - ensure that after finding an empty link,
        // subsequent valid links are not incorrectly flagged as empty
        let input = "[link1](https://example.com)\n[link2]()\n[link3](https://example.com)\n[link4](https://example.com)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should only detect link2 as empty, not link3 or link4
        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD042", violation.rule().id);
    }

    #[test]
    fn test_footnote_style_empty_links() {
        // Test case from issue #370 - footnote-style links with empty destinations
        let input = "[^gh-md]: <> \"Like here on GitHub.\"";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // This is a complex case - for now we may or may not detect this
        // The original issue suggests this might be valid footnote syntax
        // Let's see what our current implementation does
        println!("Footnote test violations: {}", violations.len());
        for violation in &violations {
            println!("  {}: {}", violation.rule().id, violation.message());
        }
    }

    #[test]
    fn test_empty_link_with_title() {
        // Test links with empty URL but with title attribute
        let input = "[link text]( \"title\")";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // According to original markdownlint behavior, title-only URLs are NOT considered empty
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_fragment_with_content() {
        // Test that fragments with actual content are not flagged
        let input = "[section](#introduction)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should not be flagged as empty
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_whitespace_only_urls() {
        // Test URLs that are only whitespace
        let input = "[empty]( ) and [tabs](\t) and [newline](\n)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect all three whitespace-only URLs as empty
        assert_eq!(3, violations.len());
        for violation in &violations {
            assert_eq!("MD042", violation.rule().id);
        }
    }
}
