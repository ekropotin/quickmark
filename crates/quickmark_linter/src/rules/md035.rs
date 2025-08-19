use serde::Deserialize;
use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

// MD035-specific configuration types
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD035HrStyleTable {
    #[serde(default)]
    pub style: String,
}

impl Default for MD035HrStyleTable {
    fn default() -> Self {
        Self {
            style: "consistent".to_string(),
        }
    }
}

pub(crate) struct MD035Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
    expected_style: Option<String>,
}

impl MD035Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
            expected_style: None,
        }
    }
}

impl RuleLinter for MD035Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "thematic_break" {
            let content = self.context.document_content.borrow();
            let text = match node.utf8_text(content.as_bytes()) {
                Ok(text) => text.trim(),
                Err(_) => return, // Ignore if text cannot be decoded
            };

            // Get the configured style from the context
            let config_style = &self.context.config.linters.settings.hr_style.style;

            // Determine or get the expected style
            let expected = self.expected_style.get_or_insert_with(|| {
                if config_style == "consistent" {
                    text.to_string() // First one sets the style
                } else {
                    config_style.clone() // Use the configured style
                }
            });

            // Check if the current style matches the expected one
            if text != expected.as_str() {
                self.violations.push(RuleViolation::new(
                    &MD035,
                    format!("Expected '{expected}', actual '{text}'"),
                    self.context.file_path.clone(),
                    range_from_tree_sitter(&node.range()),
                ));
            }
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD035: Rule = Rule {
    id: "MD035",
    alias: "hr-style",
    tags: &["hr"],
    description: "Horizontal rule style",
    rule_type: RuleType::Token,
    required_nodes: &["thematic_break"],
    new_linter: |context| Box::new(MD035Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![
            ("hr-style", RuleSeverity::Error),
            ("heading-increment", RuleSeverity::Off),
            ("heading-style", RuleSeverity::Off),
            ("line-length", RuleSeverity::Off),
        ])
    }

    #[test]
    fn test_consistent_horizontal_rules_no_violation() {
        let input = r#"# Heading

---

Some content

---

More content"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should not trigger violations for consistent styles
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_inconsistent_horizontal_rules_violation() {
        let input = r#"# Heading

---

Some content

***

More content"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should trigger violation for inconsistent style
        assert_eq!(1, violations.len());
        let violation = &violations[0];
        assert_eq!("MD035", violation.rule().id);
        assert!(violation.message().contains("Expected '---', actual '***'"));
    }

    #[test]
    fn test_multiple_inconsistent_styles() {
        let input = r#"# Heading

---

Content

***

More content

___

Final content"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should trigger violations for both inconsistent styles
        assert_eq!(2, violations.len());
        assert_eq!("MD035", violations[0].rule().id);
        assert_eq!("MD035", violations[1].rule().id);
        assert!(violations[0]
            .message()
            .contains("Expected '---', actual '***'"));
        assert!(violations[1]
            .message()
            .contains("Expected '---', actual '___'"));
    }

    #[test]
    fn test_asterisk_consistent_no_violation() {
        let input = r#"# Heading

***

Some content

***

More content"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should not trigger violations for consistent asterisk style
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_underscore_consistent_no_violation() {
        let input = r#"# Heading

___

Some content

___

More content"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should not trigger violations for consistent underscore style
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_spaced_horizontal_rules_consistent() {
        let input = r#"# Heading

* * *

Some content

* * *

More content"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should not trigger violations for consistent spaced style
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_spaced_vs_non_spaced_inconsistent() {
        let input = r#"# Heading

***

Some content

* * *

More content"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should trigger violation for inconsistent spacing
        assert_eq!(1, violations.len());
        assert!(violations[0]
            .message()
            .contains("Expected '***', actual '* * *'"));
    }

    #[test]
    fn test_single_horizontal_rule_no_violation() {
        let input = r#"# Heading

Some content

---

More content"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Single horizontal rule should not trigger any violations
        assert_eq!(0, violations.len());
    }
}
