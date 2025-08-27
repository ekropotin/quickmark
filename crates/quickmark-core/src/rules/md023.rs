use std::rc::Rc;
use tree_sitter::Node;

use crate::linter::{range_from_tree_sitter, Context, RuleLinter, RuleViolation};

use super::{Rule, RuleType};

pub(crate) struct MD023Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD023Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    fn check_atx_heading_indentation(&mut self, node: &Node) {
        let lines = self.context.lines.borrow();
        if let Some(violation) = self.check_line_for_indentation(node.start_position().row, &lines)
        {
            self.violations.push(violation);
        }
    }

    fn check_setext_heading_indentation(&mut self, node: &Node) {
        let lines = self.context.lines.borrow();

        let mut cursor = node.walk();
        let mut text_line_num = None;
        let mut underline_line_num = None;

        for child in node.children(&mut cursor) {
            match child.kind() {
                "paragraph" => {
                    text_line_num = Some(child.start_position().row);
                }
                "setext_h1_underline" | "setext_h2_underline" => {
                    underline_line_num = Some(child.start_position().row);
                }
                _ => {}
            }
        }

        if let Some(line_num) = text_line_num {
            if let Some(violation) = self.check_line_for_indentation(line_num, &lines) {
                self.violations.push(violation);
                return; // Report one violation per heading
            }
        }

        if let Some(line_num) = underline_line_num {
            if let Some(violation) = self.check_line_for_indentation(line_num, &lines) {
                self.violations.push(violation);
            }
        }
    }

    /// Checks a single line for indentation and returns a RuleViolation if it's indented.
    fn check_line_for_indentation(
        &self,
        line_num: usize,
        lines: &[String],
    ) -> Option<RuleViolation> {
        if let Some(line) = lines.get(line_num) {
            let leading_spaces = line.len() - line.trim_start().len();

            if leading_spaces > 0 {
                let range = tree_sitter::Range {
                    start_byte: 0, // Not used by range_from_tree_sitter
                    end_byte: 0,   // Not used by range_from_tree_sitter
                    start_point: tree_sitter::Point {
                        row: line_num,
                        column: 0,
                    },
                    end_point: tree_sitter::Point {
                        row: line_num,
                        column: leading_spaces,
                    },
                };

                return Some(RuleViolation::new(
                    &MD023,
                    MD023.description.to_string(),
                    self.context.file_path.clone(),
                    range_from_tree_sitter(&range),
                ));
            }
        }
        None
    }
}

impl RuleLinter for MD023Linter {
    fn feed(&mut self, node: &Node) {
        match node.kind() {
            "atx_heading" => self.check_atx_heading_indentation(node),
            "setext_heading" => self.check_setext_heading_indentation(node),
            _ => {
                // Ignore other nodes. It seems the linter is not filtering nodes
                // based on `required_nodes` before feeding them to the rule.
            }
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD023: Rule = Rule {
    id: "MD023",
    alias: "heading-start-left",
    tags: &["headings", "spaces"],
    description: "Headings must start at the beginning of the line",
    rule_type: RuleType::Hybrid,
    required_nodes: &["atx_heading", "setext_heading"],
    new_linter: |context| Box::new(MD023Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![
            ("heading-start-left", RuleSeverity::Error),
            ("heading-style", RuleSeverity::Off),
            ("heading-increment", RuleSeverity::Off),
        ])
    }

    #[test]
    fn test_atx_heading_indented() {
        let input = "Some text

 # Indented heading

More text";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        let violation = &violations[0];
        assert_eq!(2, violation.location().range.start.line);
        assert_eq!(0, violation.location().range.start.character);
        assert_eq!(2, violation.location().range.end.line);
        assert_eq!(1, violation.location().range.end.character);
    }

    #[test]
    fn test_atx_heading_not_indented() {
        let input = "Some text

# Not indented heading

More text";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_multiple_spaces_indentation() {
        let input = "Some text

   # Heading with 3 spaces

More text";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        let violation = &violations[0];
        assert_eq!(2, violation.location().range.start.line);
        assert_eq!(0, violation.location().range.start.character);
        assert_eq!(2, violation.location().range.end.line);
        assert_eq!(3, violation.location().range.end.character);
    }

    #[test]
    fn test_setext_heading_indented_text() {
        let input = "Some text

 Indented setext heading
========================

More text";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_setext_heading_indented_underline() {
        let input = "Some text

Setext heading
 ==============

More text";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_setext_heading_both_indented() {
        let input = "Some text

 Setext heading
 ==============

More text";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_setext_heading_not_indented() {
        let input = "Some text

Setext heading
==============

More text";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_heading_in_list_item() {
        let input = "* List item
  # Heading in list (should trigger)

* Another item";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_heading_in_blockquote() {
        let input = "> # Heading in blockquote (should NOT trigger)

> More blockquote content";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_hash_in_code_block() {
        let input = "```
# This is code, not a heading
   # This should also not trigger
```";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_hash_in_inline_code() {
        let input = "Text with `# inline code` and more text";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_multiple_indented_headings() {
        let input = " # First indented heading

 ## Second indented heading

### Not indented

   #### Third indented heading";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(3, violations.len());

        // First violation
        assert_eq!(0, violations[0].location().range.start.line);

        // Second violation
        assert_eq!(2, violations[1].location().range.start.line);

        // Third violation
        assert_eq!(6, violations[2].location().range.start.line);
    }
}
