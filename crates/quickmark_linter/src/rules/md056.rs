use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

/// MD056 - Table column count
///
/// This rule checks that all rows in a table have the same number of columns.
pub(crate) struct MD056Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD056Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    fn check_table_column_count(&mut self, table_node: &Node) {
        let mut cursor = table_node.walk();
        let mut table_rows = table_node.children(&mut cursor).filter(|child| {
            matches!(
                child.kind(),
                "pipe_table_header" | "pipe_table_row" | "pipe_table_delimiter_row"
            )
        });

        let Some(first_row) = table_rows.next() else {
            return;
        };

        let expected_column_count = self.count_table_cells(&first_row);

        // The first row determines the expected count, so we only need to check subsequent rows.
        for row in table_rows {
            let actual_column_count = self.count_table_cells(&row);

            if actual_column_count == expected_column_count {
                continue;
            }

            let (message, column_offset) = if actual_column_count < expected_column_count {
                (
                    format!(
                        "Too few cells, row will be missing data (expected {expected_column_count}, got {actual_column_count})"
                    ),
                    self.get_row_end_position(&row),
                )
            } else {
                (
                    format!(
                        "Too many cells, extra data will be missing (expected {expected_column_count}, got {actual_column_count})"
                    ),
                    self.get_extra_cells_position(&row, expected_column_count),
                )
            };

            let mut range = range_from_tree_sitter(&row.range());
            range.start.character += column_offset;
            range.end.character = range.start.character + 1;

            self.violations.push(RuleViolation::new(
                &MD056,
                message,
                self.context.file_path.clone(),
                range,
            ));
        }
    }

    fn count_table_cells(&self, row_node: &Node) -> usize {
        row_node
            .children(&mut row_node.walk())
            .filter(|child| {
                matches!(
                    child.kind(),
                    "pipe_table_cell" | "pipe_table_delimiter_cell"
                )
            })
            .count()
    }

    fn get_row_end_position(&self, row_node: &Node) -> usize {
        let document_content = self.context.document_content.borrow();
        let row_text = row_node
            .utf8_text(document_content.as_bytes())
            .unwrap_or("");

        // Find the end of the actual content (excluding trailing whitespace) minus 1 to match original
        row_text.trim_end().len().saturating_sub(1)
    }

    fn get_extra_cells_position(&self, row_node: &Node, expected_count: usize) -> usize {
        row_node
            .children(&mut row_node.walk())
            .filter(|child| {
                matches!(
                    child.kind(),
                    "pipe_table_cell" | "pipe_table_delimiter_cell"
                )
            })
            .nth(expected_count)
            .map(|extra_cell| extra_cell.start_position().column - row_node.start_position().column)
            .unwrap_or(0)
    }
}

impl RuleLinter for MD056Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "pipe_table" {
            self.check_table_column_count(node);
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD056: Rule = Rule {
    id: "MD056",
    alias: "table-column-count",
    tags: &["table"],
    description: "Table column count",
    rule_type: RuleType::Token,
    required_nodes: &["pipe_table"],
    new_linter: |context| Box::new(MD056Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{
        config::RuleSeverity, linter::MultiRuleLinter,
        test_utils::test_helpers::test_config_with_rules,
    };

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![("table-column-count", RuleSeverity::Error)])
    }

    #[test]
    fn test_table_with_consistent_column_count() {
        let input = r#"| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   |"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_table_with_too_few_cells() {
        let input = r#"| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |
| Cell 3   |"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Too few cells"));
    }

    #[test]
    fn test_table_with_too_many_cells() {
        let input = r#"| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   | Cell 5 |"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Too many cells"));
    }

    #[test]
    fn test_table_with_mixed_column_counts() {
        let input = r#"| Header 1 | Header 2 | Header 3 |
| -------- | -------- | -------- |
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   | Cell 5   | Cell 6 |"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len());
        assert!(violations[0].message().contains("Too few cells"));
        assert!(violations[1].message().contains("Too many cells"));
    }

    #[test]
    fn test_table_header_only() {
        let input = r#"| Header 1 | Header 2 |"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_table_with_delimiter_row_only() {
        let input = r#"| Header 1 | Header 2 |
| -------- | -------- |"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_empty_cells_in_table() {
        let input = r#"| Header 1 | Header 2 |
| -------- | -------- |
|          | Cell 2   |
| Cell 3   |          |"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_table_with_one_column() {
        let input = r#"| Header |
| ------ |
| Cell 1 |
| Cell 2 |"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_table_with_one_column_violation() {
        let input = r#"| Header |
| ------ |
| Cell 1 | Cell 2 |"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Too many cells"));
    }

    #[test]
    fn test_multiple_tables_independent() {
        let input = r#"| Table 1 | Header |
| ------- | ------ |
| Cell    | Value  |

| Different | Table | Headers |
| --------- | ----- | ------- |
| More      | Data  | Here    |"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_multiple_tables_with_violations() {
        let input = r#"| Table 1 | Header |
| ------- | ------ |
| Cell    |

| Different | Table |
| --------- | ----- |
| More      | Data  | Extra |"#;
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len());
        assert!(violations[0].message().contains("Too few cells"));
        assert!(violations[1].message().contains("Too many cells"));
    }
}
