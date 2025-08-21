use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, Context, RuleLinter, RuleViolation},
    rules::{Rule, RuleType},
};

/// MD058 - Tables should be surrounded by blank lines
///
/// This rule checks that tables have blank lines before and after them,
/// except when the table is at the very beginning or end of the document.
pub(crate) struct MD058Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD058Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    fn check_table_blanks(&mut self, table_node: &Node) {
        let start_line = table_node.start_position().row;
        let lines = self.context.lines.borrow();

        // Find the actual last row of the table.
        // tree-sitter can sometimes identify nodes as table rows even if they are not
        // part of the table's syntax (e.g., surrounding text).
        // We filter for children that are actual table components and contain a pipe character.
        let mut cursor = table_node.walk();
        let Some(last_row) = table_node
            .children(&mut cursor)
            .filter(|child| {
                matches!(
                    child.kind(),
                    "pipe_table_header" | "pipe_table_row" | "pipe_table_delimiter_row"
                )
            })
            .filter(|row| {
                let row_line = row.start_position().row;
                lines.get(row_line).is_some_and(|l| l.contains('|'))
            })
            .last()
        else {
            return; // No valid rows in table, nothing to check.
        };

        let actual_end_line = last_row.end_position().row;

        // Check for a blank line above the table if it's not at the document start.
        if start_line > 0 {
            // A blank line is required only if there is non-blank content somewhere above the table.
            let has_content_above = (0..start_line).any(|i| !lines[i].trim().is_empty());

            if has_content_above && !lines[start_line - 1].trim().is_empty() {
                self.violations.push(RuleViolation::new(
                    &MD058,
                    format!("{} [Above]", MD058.description),
                    self.context.file_path.clone(),
                    range_from_tree_sitter(&table_node.range()),
                ));
            }
        }

        // Check for a blank line below the table if it's not at the document end.
        if actual_end_line + 1 < lines.len() {
            // A blank line is required only if there is non-blank content somewhere below the table.
            let has_content_below =
                ((actual_end_line + 1)..lines.len()).any(|i| !lines[i].trim().is_empty());

            if has_content_below && !lines[actual_end_line + 1].trim().is_empty() {
                self.violations.push(RuleViolation::new(
                    &MD058,
                    format!("{} [Below]", MD058.description),
                    self.context.file_path.clone(),
                    range_from_tree_sitter(&table_node.range()),
                ));
            }
        }
    }
}

impl RuleLinter for MD058Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "pipe_table" {
            self.check_table_blanks(node);
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD058: Rule = Rule {
    id: "MD058",
    alias: "blanks-around-tables",
    tags: &["table", "blank_lines"],
    description: "Tables should be surrounded by blank lines",
    rule_type: RuleType::Token,
    required_nodes: &["pipe_table"],
    new_linter: |context| Box::new(MD058Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{
        config::RuleSeverity, linter::MultiRuleLinter,
        test_utils::test_helpers::test_config_with_rules,
    };

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![("blanks-around-tables", RuleSeverity::Error)])
    }

    #[test]
    fn test_table_with_proper_blank_lines() {
        let input = r#"Some text

| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |

More text"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_table_missing_blank_line_above() {
        let input = r#"Some text
| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |

More text"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("[Above]"));
    }

    #[test]
    fn test_table_missing_blank_line_below() {
        let input = r#"Some text

| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |
More text"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("[Below]"));
    }

    #[test]
    fn test_table_missing_both_blank_lines() {
        let input = r#"Some text
| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |
More text"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len());
        assert!(violations[0].message().contains("[Above]"));
        assert!(violations[1].message().contains("[Below]"));
    }

    #[test]
    fn test_table_at_start_of_document() {
        let input = r#"| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |

More text"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should not violate - no content above to require blank line
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_table_at_end_of_document() {
        let input = r#"Some text

| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should not violate - no content below to require blank line
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_table_alone_in_document() {
        let input = r#"| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should not violate - no content above or below
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_multiple_tables_proper_spacing() {
        let input = r#"Some text

| Table 1 | Header |
| ------- | ------ |
| Cell    | Value  |

Text between tables

| Table 2 | Header |
| ------- | ------ |
| Cell    | Value  |

Final text"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_multiple_tables_improper_spacing() {
        let input = r#"Some text
| Table 1 | Header |
| ------- | ------ |
| Cell    | Value  |
Text between tables
| Table 2 | Header |
| ------- | ------ |
| Cell    | Value  |
Final text"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(4, violations.len()); // 2 tables × 2 violations each (above and below)
    }

    #[test]
    fn test_table_with_only_blank_lines_above_and_below() {
        let input = r#"


| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |


"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should not violate - no actual content above or below
        assert_eq!(0, violations.len());
    }
}
