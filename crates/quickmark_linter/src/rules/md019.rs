use std::rc::Rc;
use tree_sitter::Node;

use crate::linter::{Context, RuleLinter, RuleViolation};

use super::{Rule, RuleType};

pub(crate) struct MD019Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD019Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    fn check_heading_spaces(&mut self, node: &Node) {
        let source = self.context.get_document_content();

        // Different approach: analyze the raw text between marker and content
        if let (Some(marker_child), Some(content_child)) = (node.child(0), node.child(1)) {
            if marker_child.kind().starts_with("atx_h") && marker_child.kind().ends_with("_marker")
            {
                let marker_end = marker_child.end_byte();
                let content_start = content_child.start_byte();

                // Extract the whitespace between marker and content
                if content_start > marker_end {
                    let whitespace_text = &source[marker_end..content_start];

                    // Check if more than one whitespace character
                    if whitespace_text.len() > 1 {
                        // Create a range for the excess whitespace (after the first character)
                        let line_start = source[..marker_end]
                            .rfind('\n')
                            .map(|pos| pos + 1)
                            .unwrap_or(0);
                        let line_num = source[..marker_end].matches('\n').count();
                        let start_col = marker_end - line_start + 1; // +1 for the first valid space

                        self.violations.push(RuleViolation::new(
                            &MD019,
                            format!(
                                "Multiple spaces after hash on atx style heading [Expected: 1; Actual: {}]",
                                whitespace_text.len()
                            ),
                            self.context.file_path.clone(),
                            crate::linter::Range {
                                start: crate::linter::CharPosition { line: line_num, character: start_col },
                                end: crate::linter::CharPosition { line: line_num, character: start_col + whitespace_text.len() - 1 },
                            },
                        ));
                    }
                }
            }
        }
    }
}

impl RuleLinter for MD019Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "atx_heading" {
            self.check_heading_spaces(node);
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD019: Rule = Rule {
    id: "MD019",
    alias: "no-multiple-space-atx",
    tags: &["headings", "atx", "spaces"],
    description: "Multiple spaces after hash on atx style heading",
    rule_type: RuleType::Token,
    required_nodes: &["atx_heading"],
    new_linter: |context| Box::new(MD019Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![
            ("no-multiple-space-atx", RuleSeverity::Error),
            ("heading-style", RuleSeverity::Off),
            ("heading-increment", RuleSeverity::Off),
        ])
    }

    #[test]
    fn test_md019_multiple_spaces_violations() {
        let config = test_config();

        let input = "##  Heading 2
###   Heading 3
####    Heading 4
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect 3 violations for multiple spaces after hash
        assert_eq!(violations.len(), 3);

        for violation in &violations {
            assert_eq!(violation.rule().id, "MD019");
        }
    }

    #[test]
    fn test_md019_single_space_no_violations() {
        let config = test_config();

        let input = "# Heading 1
## Heading 2
### Heading 3
#### Heading 4
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - single space after hash is correct
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_md019_tabs_and_spaces_violations() {
        let config = test_config();

        let input = "##\t\tHeading with tabs
###  \tHeading with space and tab
####   Heading with multiple spaces
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect 3 violations for multiple whitespace chars after hash
        assert_eq!(violations.len(), 3);

        for violation in &violations {
            assert_eq!(violation.rule().id, "MD019");
        }
    }

    #[test]
    fn test_md019_mixed_valid_and_invalid() {
        let config = test_config();

        let input = "# Valid heading 1
##  Invalid heading 2
### Valid heading 3
####   Invalid heading 4
##### Valid heading 5
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect 2 violations (lines 2 and 4)
        assert_eq!(violations.len(), 2);

        for violation in &violations {
            assert_eq!(violation.rule().id, "MD019");
        }
    }

    #[test]
    fn test_md019_no_space_violations() {
        let config = test_config();

        let input = "#Heading with no space
##Heading with no space
###Heading with no space
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - MD019 only cares about multiple spaces, not missing spaces
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_md019_closed_atx_violations() {
        let config = test_config();

        let input = "##  Closed heading with multiple spaces ##
###   Another closed heading ###
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect 2 violations for multiple spaces after opening hash
        assert_eq!(violations.len(), 2);

        for violation in &violations {
            assert_eq!(violation.rule().id, "MD019");
        }
    }

    #[test]
    fn test_md019_only_atx_headings() {
        let config = test_config();

        let input = "Setext Heading 1
================

Setext Heading 2
----------------

##  ATX heading with multiple spaces
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should only detect 1 violation for the ATX heading, not setext headings
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule().id, "MD019");
    }
}
