use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    config::TablePipeStyle,
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

/// MD055 - Table pipe style
///
/// This rule enforces consistent use of leading and trailing pipes in tables.
pub(crate) struct MD055Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
    first_table_style: Option<(bool, bool)>, // (has_leading, has_trailing)
}

struct ViolationInfo {
    message: String,
    column_offset: usize,
}

impl MD055Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
            first_table_style: None,
        }
    }
}

impl RuleLinter for MD055Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "pipe_table" {
            self.check_table(node);
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

impl MD055Linter {
    fn check_table(&mut self, table_node: &Node) {
        let mut table_rows = Vec::new();
        let mut cursor = table_node.walk();
        for child in table_node.children(&mut cursor) {
            if child.kind() == "pipe_table_header"
                || child.kind() == "pipe_table_row"
                || child.kind() == "pipe_table_delimiter_row"
            {
                table_rows.push(child);
            }
        }

        if table_rows.is_empty() {
            return;
        }

        let mut all_violation_infos = Vec::new();
        {
            // This scope limits the lifetime of `document_content`'s borrow
            let document_content = self.context.document_content.borrow();
            let config_style = &self.context.config.linters.settings.table_pipe_style.style;

            let expected_style = match config_style {
                TablePipeStyle::Consistent => {
                    if let Some(style) = self.first_table_style {
                        style
                    } else {
                        let first_row_text = table_rows[0]
                            .utf8_text(document_content.as_bytes())
                            .unwrap_or("")
                            .trim();
                        let has_leading = first_row_text.starts_with('|');
                        let has_trailing =
                            first_row_text.ends_with('|') && first_row_text.len() > 1;
                        let style = (has_leading, has_trailing);
                        self.first_table_style = Some(style);
                        style
                    }
                }
                TablePipeStyle::LeadingAndTrailing => (true, true),
                TablePipeStyle::LeadingOnly => (true, false),
                TablePipeStyle::TrailingOnly => (false, true),
                TablePipeStyle::NoLeadingOrTrailing => (false, false),
            };

            for row in &table_rows {
                let infos = self.check_row_pipe_style(row, expected_style, &document_content);
                if !infos.is_empty() {
                    all_violation_infos.push((*row, infos));
                }
            }
        }

        for (row, infos) in all_violation_infos {
            for info in infos {
                self.create_violation_at_position(&row, info.message, info.column_offset);
            }
        }
    }

    fn check_row_pipe_style(
        &self,
        row_node: &Node,
        expected: (bool, bool),
        document_content: &str,
    ) -> Vec<ViolationInfo> {
        let mut infos = Vec::new();
        let (expected_leading, expected_trailing) = expected;

        let row_text = row_node
            .utf8_text(document_content.as_bytes())
            .unwrap_or("");
        let leading_whitespace_len = row_text.len() - row_text.trim_start().len();
        let trimmed_text = row_text.trim();

        let actual_leading = trimmed_text.starts_with('|');
        let actual_trailing = trimmed_text.ends_with('|') && trimmed_text.len() > 1;

        // Check leading pipe
        if expected_leading != actual_leading {
            let message = if expected_leading {
                "Missing leading pipe"
            } else {
                "Unexpected leading pipe"
            };
            infos.push(ViolationInfo {
                message: message.to_string(),
                column_offset: leading_whitespace_len,
            });
        }

        // Check trailing pipe
        if expected_trailing != actual_trailing {
            let message = if expected_trailing {
                "Missing trailing pipe"
            } else {
                "Unexpected trailing pipe"
            };
            let pos = if actual_trailing {
                leading_whitespace_len + trimmed_text.len().saturating_sub(1)
            } else {
                leading_whitespace_len + trimmed_text.len()
            };
            infos.push(ViolationInfo {
                message: message.to_string(),
                column_offset: pos,
            });
        }
        infos
    }

    fn create_violation_at_position(&mut self, node: &Node, message: String, column_offset: usize) {
        let mut range = range_from_tree_sitter(&node.range());
        range.start.character += column_offset;
        range.end.character = range.start.character + 1;

        self.violations.push(RuleViolation::new(
            &MD055,
            message,
            self.context.file_path.clone(),
            range,
        ));
    }
}

pub const MD055: Rule = Rule {
    id: "MD055",
    alias: "table-pipe-style",
    tags: &["table"],
    description: "Table pipe style",
    rule_type: RuleType::Token,
    required_nodes: &["pipe_table"],
    new_linter: |context| Box::new(MD055Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{
        config::{MD055TablePipeStyleTable, RuleSeverity, TablePipeStyle},
        linter::MultiRuleLinter,
        test_utils::test_helpers::test_config_with_rules,
    };

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![("table-pipe-style", RuleSeverity::Error)])
    }

    fn test_config_with_style(style: TablePipeStyle) -> crate::config::QuickmarkConfig {
        let mut config = test_config();
        config.linters.settings.table_pipe_style = MD055TablePipeStyleTable { style };
        config
    }

    #[test]
    fn test_consistent_style_with_leading_and_trailing() {
        let input = r#"| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |"#;
        let config = test_config_with_style(TablePipeStyle::Consistent);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_consistent_style_with_leading_only() {
        let input = r#"| Header 1 | Header 2
| -------- | --------
| Cell 1   | Cell 2"#;
        let config = test_config_with_style(TablePipeStyle::Consistent);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_consistent_style_with_trailing_only() {
        let input = r#"Header 1 | Header 2 |
-------- | -------- |
Cell 1   | Cell 2   |"#;
        let config = test_config_with_style(TablePipeStyle::Consistent);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_consistent_style_with_no_leading_or_trailing() {
        let input = r#"Header 1 | Header 2
-------- | --------
Cell 1   | Cell 2"#;
        let config = test_config_with_style(TablePipeStyle::Consistent);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_consistent_style_violation() {
        let input = r#"| Header 1 | Header 2 |
| -------- | -------- |
Cell 1   | Cell 2   |"#; // Missing leading pipe in last row
        let config = test_config_with_style(TablePipeStyle::Consistent);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Missing leading pipe"));
    }

    #[test]
    fn test_leading_and_trailing_style_valid() {
        let input = r#"| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |"#;
        let config = test_config_with_style(TablePipeStyle::LeadingAndTrailing);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_leading_and_trailing_style_missing_leading() {
        let input = r#"Header 1 | Header 2 |
-------- | -------- |
Cell 1   | Cell 2   |"#;
        let config = test_config_with_style(TablePipeStyle::LeadingAndTrailing);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(3, violations.len()); // Header, delimiter, and data row missing leading pipe
        for violation in &violations {
            assert!(violation.message().contains("Missing leading pipe"));
        }
    }

    #[test]
    fn test_leading_and_trailing_style_missing_trailing() {
        let input = r#"| Header 1 | Header 2
| -------- | --------
| Cell 1   | Cell 2"#;
        let config = test_config_with_style(TablePipeStyle::LeadingAndTrailing);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(3, violations.len()); // Header, delimiter, and data row missing trailing pipe
        for violation in &violations {
            assert!(violation.message().contains("Missing trailing pipe"));
        }
    }

    #[test]
    fn test_leading_only_style_valid() {
        let input = r#"| Header 1 | Header 2
| -------- | --------
| Cell 1   | Cell 2"#;
        let config = test_config_with_style(TablePipeStyle::LeadingOnly);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_leading_only_style_unexpected_trailing() {
        let input = r#"| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |"#;
        let config = test_config_with_style(TablePipeStyle::LeadingOnly);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(3, violations.len()); // Header, delimiter, and data row have unexpected trailing pipe
        for violation in &violations {
            assert!(violation.message().contains("Unexpected trailing pipe"));
        }
    }

    #[test]
    fn test_trailing_only_style_valid() {
        let input = r#"Header 1 | Header 2 |
-------- | -------- |
Cell 1   | Cell 2   |"#;
        let config = test_config_with_style(TablePipeStyle::TrailingOnly);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_trailing_only_style_unexpected_leading() {
        let input = r#"| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |"#;
        let config = test_config_with_style(TablePipeStyle::TrailingOnly);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(3, violations.len()); // Header, delimiter, and data row have unexpected leading pipe
        for violation in &violations {
            assert!(violation.message().contains("Unexpected leading pipe"));
        }
    }

    #[test]
    fn test_no_leading_or_trailing_style_valid() {
        let input = r#"Header 1 | Header 2
-------- | --------
Cell 1   | Cell 2"#;
        let config = test_config_with_style(TablePipeStyle::NoLeadingOrTrailing);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_no_leading_or_trailing_style_unexpected_leading() {
        let input = r#"| Header 1 | Header 2
| -------- | --------
| Cell 1   | Cell 2"#;
        let config = test_config_with_style(TablePipeStyle::NoLeadingOrTrailing);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(3, violations.len()); // Header, delimiter, and data row have unexpected leading pipe
        for violation in &violations {
            assert!(violation.message().contains("Unexpected leading pipe"));
        }
    }

    #[test]
    fn test_no_leading_or_trailing_style_unexpected_trailing() {
        let input = r#"Header 1 | Header 2 |
-------- | -------- |
Cell 1   | Cell 2   |"#;
        let config = test_config_with_style(TablePipeStyle::NoLeadingOrTrailing);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(3, violations.len()); // Header, delimiter, and data row have unexpected trailing pipe
        for violation in &violations {
            assert!(violation.message().contains("Unexpected trailing pipe"));
        }
    }

    #[test]
    fn test_multiple_tables_consistent_style() {
        let input = r#"| Table 1 | Header |
| ------- | ------ |
| Cell    | Value  |

Header | Column |
------ | ------ |
Data   | Info   |"#;
        let config = test_config_with_style(TablePipeStyle::Consistent);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(3, violations.len()); // Second table (header, delimiter, data) should match first table's style
        for violation in &violations {
            assert!(violation.message().contains("Missing"));
        }
    }

    #[test]
    fn test_empty_table() {
        let input = "";
        let config = test_config_with_style(TablePipeStyle::Consistent);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    // Edge case tests discovered during parity validation

    #[test]
    fn test_delimiter_rows_are_checked() {
        // During parity validation, discovered that delimiter rows must also be checked
        let input = r#"| Header 1 | Header 2 |
-------- | -------- |
| Cell 1  | Cell 2   |"#; // Delimiter row missing leading/trailing pipes
        let config = test_config_with_style(TablePipeStyle::LeadingAndTrailing);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect violations on delimiter row
        assert!(!violations.is_empty()); // At least delimiter row violations
        let violation_lines: Vec<usize> = violations
            .iter()
            .map(|v| v.location().range.start.line)
            .collect();
        assert!(violation_lines.contains(&1)); // Line 1 is the delimiter row (0-indexed)

        // Verify that delimiter row violations are detected
        let delimiter_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.location().range.start.line == 1)
            .collect();
        assert!(!delimiter_violations.is_empty()); // Should have at least one violation on delimiter row
    }

    #[test]
    fn test_column_position_accuracy() {
        // During parity validation, discovered exact column positions matter
        let input = r#"Header 1 | Header 2
-------- | --------
Data 1   | Data 2"#; // Missing both leading and trailing pipes
        let config = test_config_with_style(TablePipeStyle::LeadingAndTrailing);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert!(violations.len() >= 2);

        // Leading pipe violations should be at column 0
        let leading_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.message().contains("Missing leading"))
            .collect();
        assert!(!leading_violations.is_empty());
        for violation in leading_violations {
            assert_eq!(0, violation.location().range.start.character);
        }

        // Trailing pipe violations should be at end of content
        let trailing_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.message().contains("Missing trailing"))
            .collect();
        assert!(!trailing_violations.is_empty());
        for violation in trailing_violations {
            // Each should be at the end of its respective line content
            assert!(violation.location().range.start.character > 0);
        }
    }

    #[test]
    fn test_single_row_table() {
        // Edge case: table with only header, no data rows
        let input = r#"| Header 1 | Header 2 |"#;
        let config = test_config_with_style(TablePipeStyle::LeadingAndTrailing);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Should be valid
    }

    #[test]
    fn test_consistent_style_with_first_table_no_pipes() {
        // Edge case: first table has no pipes, subsequent tables should match
        let input = r#"Header 1 | Header 2
-------- | --------
Data 1   | Data 2

| Another | Table |
| ------- | ----- |
| With    | Pipes |"#;
        let config = test_config_with_style(TablePipeStyle::Consistent);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Second table should violate because it has pipes when first doesn't
        assert!(!violations.is_empty());
        for violation in &violations {
            assert!(violation.message().contains("Unexpected"));
        }
    }

    #[test]
    fn test_mixed_violations_same_row() {
        // Edge case: row with both missing leading AND trailing pipes
        let input = r#"| Header 1 | Header 2 |
| -------- | -------- |
Cell 1    | Cell 2"#; // Missing both leading and trailing pipes
        let config = test_config_with_style(TablePipeStyle::LeadingAndTrailing);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should report both violations for the last row (0-indexed line 2)
        let row3_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.location().range.start.line == 2)
            .collect();
        assert_eq!(2, row3_violations.len()); // Both leading and trailing violations
    }

    #[test]
    fn test_table_with_empty_cells() {
        // Edge case: table with empty cells
        let input = r#"| Header |  |
| ------ |  |
| Value  |  |"#;
        let config = test_config_with_style(TablePipeStyle::LeadingAndTrailing);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Should be valid despite empty cells
    }

    #[test]
    fn test_table_with_escaped_pipes() {
        // Edge case: table with escaped pipes in content
        let input = r#"| Header | Content |
| ------ | ------- |
| Value  | \| pipe |"#;
        let config = test_config_with_style(TablePipeStyle::LeadingAndTrailing);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Escaped pipes shouldn't affect style detection
    }
}
