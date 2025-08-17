use std::collections::HashSet;
use std::rc::Rc;

use once_cell::sync::Lazy;
use regex::Regex;
use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

// Regular inline links: [text](url) - but NOT images ![text](url)
static RE_INLINE_LINK: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?:^|[^!])\[([^\]]*)\]\(([^)]+)\)").expect("Failed to compile inline link regex")
});

// Reference links: [text][ref] - but NOT images ![text][ref]
static RE_REF_LINK: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?:^|[^!])\[([^\]]*)\]\[([^\]]+)\]")
        .expect("Failed to compile reference link regex")
});

// Collapsed reference links: [text][] - but NOT images ![text][]
static RE_COLLAPSED_REF_LINK: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?:^|[^!])\[([^\]]+)\]\[\]")
        .expect("Failed to compile collapsed reference link regex")
});

static RE_NORMALIZE_PUNCTUATION: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[\W_]+").expect("Failed to compile punctuation regex"));
static RE_NORMALIZE_WHITESPACE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s+").expect("Failed to compile whitespace regex"));

/// MD059 - Link text should be descriptive
///
/// This rule checks that link text provides meaningful description instead of generic phrases.
pub(crate) struct MD059Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
    prohibited_texts: HashSet<String>,
}

impl MD059Linter {
    pub fn new(context: Rc<Context>) -> Self {
        let prohibited_texts = context
            .config
            .linters
            .settings
            .descriptive_link_text
            .prohibited_texts
            .iter()
            .map(|text| normalize_text(text))
            .collect();

        Self {
            context,
            violations: Vec::new(),
            prohibited_texts,
        }
    }
}

impl RuleLinter for MD059Linter {
    fn feed(&mut self, node: &Node) {
        // Process different possible link node types
        match node.kind() {
            "link" => self.check_link_text(node),
            "inline" => self.check_inline_for_links(node),
            _ => {}
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

impl MD059Linter {
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
        if !link_text.is_empty() {
            self.check_text_for_link_patterns(&link_text, inline_node);
        }
    }

    fn check_text_for_link_patterns(&mut self, text: &str, node: &Node) {
        for caps in RE_INLINE_LINK.captures_iter(text) {
            if let Some(label_match) = caps.get(1) {
                let label_text = label_match.as_str();
                self.check_label_for_prohibited_text(label_text, node);
            }
        }

        for caps in RE_REF_LINK.captures_iter(text) {
            if let Some(label_match) = caps.get(1) {
                let label_text = label_match.as_str();
                self.check_label_for_prohibited_text(label_text, node);
            }
        }

        for caps in RE_COLLAPSED_REF_LINK.captures_iter(text) {
            if let Some(label_match) = caps.get(1) {
                let label_text = label_match.as_str();
                self.check_label_for_prohibited_text(label_text, node);
            }
        }
    }

    fn check_link_text(&mut self, link_node: &Node) {
        // Extract the link text content from tree-sitter link nodes
        if let Some(text) = self.extract_link_text(link_node) {
            // Check if the link contains code or HTML content - if so, skip validation
            if self.contains_allowed_elements(link_node) {
                return;
            }

            let normalized_text = normalize_text(&text);

            if self.prohibited_texts.contains(&normalized_text) {
                self.create_violation(link_node, &text);
            }
        }
    }

    fn check_label_for_prohibited_text(&mut self, label_text: &str, node: &Node) {
        // Check if label text contains code or HTML - if so, skip
        if label_text.contains('`') || label_text.contains('<') {
            return;
        }

        let normalized_text = normalize_text(label_text);

        if self.prohibited_texts.contains(&normalized_text) {
            self.create_violation(node, label_text);
        }
    }

    fn extract_link_text(&self, link_node: &Node) -> Option<String> {
        // Navigate the tree-sitter AST to find the link text
        // Links in markdown have structure like: link -> label -> [text content]
        let document_content = self.context.document_content.borrow();
        let document_bytes = document_content.as_bytes();

        // Look for label child node
        for child in link_node.children(&mut link_node.walk()) {
            if child.kind() == "label" {
                // Extract text from label, excluding the brackets
                let label_text = child.utf8_text(document_bytes).unwrap_or("");

                // Remove the surrounding brackets
                if label_text.starts_with('[') && label_text.ends_with(']') {
                    let inner_text = &label_text[1..label_text.len() - 1];
                    return Some(inner_text.to_string());
                }
            }
        }

        // Fallback: try to extract from the full link text
        let full_text = link_node.utf8_text(document_bytes).unwrap_or("");
        if let Some(start) = full_text.find('[') {
            if let Some(end) = full_text[start..].find(']') {
                let inner_text = &full_text[start + 1..start + end];
                return Some(inner_text.to_string());
            }
        }

        None
    }

    fn contains_allowed_elements(&self, link_node: &Node) -> bool {
        // Check if the link contains code or HTML elements, which are allowed.
        // This is an efficient, allocation-free, iterative pre-order traversal.
        let allowed_types: &[&str] = &["code_span", "html_tag", "inline_html"];
        let mut cursor = link_node.walk();
        loop {
            if allowed_types.contains(&cursor.node().kind()) {
                return true;
            }
            if !cursor.goto_first_child() {
                while !cursor.goto_next_sibling() {
                    if !cursor.goto_parent() {
                        return false;
                    }
                }
            }
        }
    }

    fn create_violation(&mut self, node: &Node, link_text: &str) {
        let message = format!("Link text should be descriptive: '{link_text}'");

        self.violations.push(RuleViolation::new(
            &MD059,
            message,
            self.context.file_path.clone(),
            range_from_tree_sitter(&node.range()),
        ));
    }
}

/// Normalizes text using the same algorithm as the original markdownlint
/// Removes punctuation and extra whitespace, converts to lowercase
fn normalize_text(text: &str) -> String {
    // Replace all non-word and underscore characters with spaces
    let step1 = RE_NORMALIZE_PUNCTUATION.replace_all(text, " ");

    // Replace multiple spaces with single space
    let step2 = RE_NORMALIZE_WHITESPACE.replace_all(&step1, " ");

    // Convert to lowercase and trim
    step2.to_lowercase().trim().to_string()
}

pub const MD059: Rule = Rule {
    id: "MD059",
    alias: "descriptive-link-text",
    tags: &["accessibility", "links"],
    description: "Link text should be descriptive",
    rule_type: RuleType::Token,
    required_nodes: &["link", "inline"],
    new_linter: |context| Box::new(MD059Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    use super::normalize_text;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![
            ("descriptive-link-text", RuleSeverity::Error),
            ("heading-style", RuleSeverity::Off),
            ("heading-increment", RuleSeverity::Off),
            ("line-length", RuleSeverity::Off),
        ])
    }

    #[test]
    fn test_normalize_text() {
        assert_eq!("click here", normalize_text("click here"));
        assert_eq!("click here", normalize_text("Click Here"));
        assert_eq!("click here", normalize_text("click   here"));
        assert_eq!("click here", normalize_text("click_here"));
        assert_eq!("click here", normalize_text("click-here"));
        assert_eq!("click here", normalize_text("  click here  "));
        assert_eq!("click here", normalize_text("click.here!"));
    }

    #[test]
    fn test_descriptive_link_passes() {
        let input = "[Download the budget document](https://example.com/budget.pdf)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_generic_link_text_fails() {
        let input = "[click here](https://example.com)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD059", violation.rule().id);
        assert!(violation
            .message()
            .contains("Link text should be descriptive"));
        assert!(violation.message().contains("click here"));
    }

    #[test]
    fn test_prohibited_texts() {
        let test_cases = vec![
            "[here](url)",
            "[link](url)",
            "[more](url)",
            "[click here](url)",
        ];

        for input in test_cases {
            let config = test_config();
            let mut linter =
                MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
            let violations = linter.analyze();

            assert_eq!(1, violations.len(), "Failed for input: {input}");
            let violation = &violations[0];
            assert_eq!("MD059", violation.rule().id);
        }
    }

    #[test]
    fn test_case_insensitive() {
        let input = "[CLICK HERE](https://example.com)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_punctuation_normalized() {
        let input = "[click-here!](https://example.com)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_extra_whitespace_normalized() {
        let input = "[  click   here  ](https://example.com)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_reference_links() {
        let input = r#"[click here][ref]

[ref]: https://example.com"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_multiple_links() {
        let input = "[good link](url1) and [click here](url2) and [another good](url3)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("click here"));
    }

    #[test]
    fn test_empty_link_text() {
        let input = "[](https://example.com)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Empty link text should not match prohibited texts
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_links_with_code_allowed() {
        let input = "[`click here`](https://example.com)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Links containing code should be allowed
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_image_links_ignored() {
        let input = "![click here](image.jpg)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Images should be ignored by this rule
        assert_eq!(0, violations.len());
    }
}
