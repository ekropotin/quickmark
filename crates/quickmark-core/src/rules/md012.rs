use serde::Deserialize;
use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

// MD012-specific configuration types
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD012MultipleBlankLinesTable {
    #[serde(default)]
    pub maximum: usize,
}

impl Default for MD012MultipleBlankLinesTable {
    fn default() -> Self {
        Self { maximum: 1 }
    }
}

/// MD012 Multiple Consecutive Blank Lines Rule Linter
///
/// **SINGLE-USE CONTRACT**: This linter is designed for one-time use only.
/// After processing a document (via feed() calls and finalize()), the linter
/// should be discarded. The violations state is not cleared between uses.
pub(crate) struct MD012Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD012Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    /// Analyze all lines and store all violations for reporting via finalize()
    /// Context cache is already initialized by MultiRuleLinter
    fn analyze_all_lines(&mut self) {
        let settings = &self.context.config.linters.settings.multiple_blank_lines;
        let lines = self.context.lines.borrow();
        let maximum = settings.maximum;

        // Create a boolean mask for lines that are part of code blocks.
        // This is more performant than a HashSet for dense data like line numbers
        // due to better cache locality and no hashing overhead.
        let mut code_block_mask = vec![false; lines.len()];
        self.populate_code_block_mask(&mut code_block_mask);

        let mut consecutive_blanks = 0;

        for (line_index, line) in lines.iter().enumerate() {
            let is_blank = line.trim().is_empty();
            // Use the boolean mask for an O(1) lookup.
            let is_in_code_block = code_block_mask.get(line_index).copied().unwrap_or(false);

            if is_blank && !is_in_code_block {
                consecutive_blanks += 1;

                // Report violation immediately when maximum is exceeded
                // This matches markdownlint behavior of reporting each position
                if consecutive_blanks > maximum {
                    let violation = self.create_violation(line_index, consecutive_blanks, maximum);
                    self.violations.push(violation);
                }
            } else {
                consecutive_blanks = 0;
            }
        }

        // Note: No additional end-of-document check needed because violations
        // are reported immediately during the loop when each blank line is processed
    }

    /// Populates a boolean slice indicating which lines are part of code blocks.
    ///
    /// This is performant as it uses the pre-parsed node cache and a contiguous
    /// memory block (`Vec<bool>`) for marking lines, leading to better cache
    /// performance than a `HashSet`. It uses 0-based indexing consistently.
    ///
    /// Note: Works around a tree-sitter-md issue where fenced code blocks
    /// incorrectly include a blank line immediately after the closing fence.
    fn populate_code_block_mask(&self, mask: &mut [bool]) {
        let node_cache = self.context.node_cache.borrow();
        let lines = self.context.lines.borrow();

        // Handle indented code blocks
        if let Some(indented_blocks) = node_cache.get("indented_code_block") {
            for node_info in indented_blocks {
                for line_num in node_info.line_start..=node_info.line_end {
                    if let Some(is_in_block) = mask.get_mut(line_num) {
                        *is_in_block = true;
                    }
                }
            }
        }

        // Handle fenced code blocks with workaround for tree-sitter issue
        if let Some(fenced_blocks) = node_cache.get("fenced_code_block") {
            for node_info in fenced_blocks {
                let mut end_line = node_info.line_end;

                // Workaround: If the last line in the range is blank and doesn't contain
                // a closing fence, exclude it (it's likely incorrectly included by tree-sitter)
                if let Some(last_line) = lines.get(end_line) {
                    if last_line.trim().is_empty() {
                        // Check if the previous line contains a closing fence
                        if let Some(prev_line) = lines.get(end_line.saturating_sub(1)) {
                            if prev_line.trim().starts_with("```") {
                                // The previous line is the closing fence, so this blank line
                                // should not be part of the code block
                                end_line = end_line.saturating_sub(1);
                            }
                        }
                    }
                }

                for line_num in node_info.line_start..=end_line {
                    if let Some(is_in_block) = mask.get_mut(line_num) {
                        *is_in_block = true;
                    }
                }
            }
        }
    }

    /// Creates a RuleViolation with a correctly calculated range.
    fn create_violation(
        &self,
        line_index: usize,
        consecutive_blanks: usize,
        maximum: usize,
    ) -> RuleViolation {
        let message = format!(
            "Multiple consecutive blank lines [Expected: {maximum} or fewer; Actual: {consecutive_blanks}]"
        );

        RuleViolation::new(
            &MD012,
            message,
            self.context.file_path.clone(),
            range_from_tree_sitter(&tree_sitter::Range {
                // FIXME: Byte offsets are not correctly calculated because line start offsets are
                // unavailable here. To fix this, the `Context` should provide a way to resolve
                // a line index to its starting byte offset in the source file.
                // The current implementation of `0` is incorrect and may result in
                // incorrect highlighting in some tools.
                start_byte: 0,
                end_byte: 0,
                start_point: tree_sitter::Point {
                    row: line_index,
                    column: 0,
                },
                end_point: tree_sitter::Point {
                    row: line_index,
                    column: 0,
                },
            }),
        )
    }
}

impl RuleLinter for MD012Linter {
    fn feed(&mut self, node: &Node) {
        // This rule is line-based and only needs to run once.
        // We trigger the analysis on seeing the top-level `document` node.
        if node.kind() == "document" {
            self.analyze_all_lines();
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD012: Rule = Rule {
    id: "MD012",
    alias: "no-multiple-blanks",
    tags: &["blank_lines", "whitespace"],
    description: "Multiple consecutive blank lines",
    rule_type: RuleType::Line,
    // This is a line-based rule and does not require specific nodes from the AST.
    // The logic runs once for the entire file content.
    required_nodes: &[],
    new_linter: |context| Box::new(MD012Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{LintersSettingsTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::{test_config_with_rules, test_config_with_settings};

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![
            ("no-multiple-blanks", RuleSeverity::Error),
            ("heading-style", RuleSeverity::Off),
            ("heading-increment", RuleSeverity::Off),
        ])
    }

    fn test_config_with_multiple_blanks(
        multiple_blanks_config: crate::config::MD012MultipleBlankLinesTable,
    ) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![
                ("no-multiple-blanks", RuleSeverity::Error),
                ("heading-style", RuleSeverity::Off),
                ("heading-increment", RuleSeverity::Off),
            ],
            LintersSettingsTable {
                multiple_blank_lines: multiple_blanks_config,
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_no_violations_single_line() {
        let input = "Single line document";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_no_violations_no_blank_lines() {
        let input = r#"Line one
Line two
Line three"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_no_violations_single_blank_line() {
        let input = r#"Line one

Line two"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_violation_two_consecutive_blank_lines() {
        let input = r#"Line one


Line two"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        let violation = &violations[0];
        assert_eq!("MD012", violation.rule().id);
        assert!(violation
            .message()
            .contains("Multiple consecutive blank lines"));
    }

    #[test]
    fn test_violation_three_consecutive_blank_lines() {
        let input = r#"Line one



Line two"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should have 2 violations: one at 2nd blank, one at 3rd blank (markdownlint behavior)
        assert_eq!(2, violations.len());

        for violation in &violations {
            assert_eq!("MD012", violation.rule().id);
        }
    }

    #[test]
    fn test_violation_multiple_locations() {
        let input = r#"Line one


Line two


Line three"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len());

        for violation in &violations {
            assert_eq!("MD012", violation.rule().id);
        }
    }

    #[test]
    fn test_custom_maximum_two() {
        let config =
            test_config_with_multiple_blanks(crate::config::MD012MultipleBlankLinesTable {
                maximum: 2,
            });

        // Two blank lines should be allowed
        let input_allowed = r#"Line one


Line two"#;
        let mut linter = MultiRuleLinter::new_for_document(
            PathBuf::from("test.md"),
            config.clone(),
            input_allowed,
        );
        let violations = linter.analyze();
        assert_eq!(0, violations.len());

        // Three blank lines should violate
        let input_violation = r#"Line one



Line two"#;
        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input_violation);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_custom_maximum_zero() {
        let config =
            test_config_with_multiple_blanks(crate::config::MD012MultipleBlankLinesTable {
                maximum: 0,
            });

        // Any blank line should violate
        let input = r#"Line one

Line two"#;
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_code_blocks_excluded() {
        // Indented code block
        let input_indented = r#"Normal text

    Code line 1


    Code line 2

Normal text again"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(
            PathBuf::from("test.md"),
            config.clone(),
            input_indented,
        );
        let violations = linter.analyze();
        // Should not violate for blank lines inside code blocks
        assert_eq!(0, violations.len());

        // Fenced code block
        let input_fenced = r#"Normal text

```
Code line 1


Code line 2
```

Normal text again"#;

        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input_fenced);
        let violations = linter.analyze();
        // Should not violate for blank lines inside fenced code blocks
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_code_blocks_with_surrounding_violations() {
        let input = r#"Normal text


```
Code with blank lines


Inside
```


More normal text"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should violate for multiple blank lines outside code blocks
        assert_eq!(2, violations.len());
    }

    #[test]
    fn test_blank_lines_with_spaces() {
        // Blank lines with only spaces should still count as blank
        let input = "Line one\n\n  \n\nLine two"; // Second blank line has 2 spaces

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // 3 consecutive blank lines = 2 violations (when we reach 2nd and 3rd blank)
        assert_eq!(2, violations.len());
    }

    #[test]
    fn test_trailing_newline_edge_case() {
        // This test specifically covers the edge case where a file ends with newlines
        // that create an implicit empty line. This was the root cause of the parity
        // issue with markdownlint - markdownlint counts the implicit line created by
        // a trailing newline, but Rust's str.lines() doesn't include it.

        // File ending with single newline - should not violate (no blank lines)
        let input_single = "Line one\nLine two\n";
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(
            PathBuf::from("test.md"),
            config.clone(),
            input_single,
        );
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Single trailing newline should not violate"
        );

        // File ending with two newlines - creates one explicit blank + one implicit blank = 2 consecutive blanks
        // This should violate because it exceeds maximum of 1
        let input_double = "Line one\nLine two\n\n";
        let mut linter = MultiRuleLinter::new_for_document(
            PathBuf::from("test.md"),
            config.clone(),
            input_double,
        );
        let violations = linter.analyze();
        assert_eq!(
            1,
            violations.len(),
            "Double trailing newline (two consecutive blanks) should violate"
        );

        // File ending with three newlines - creates two explicit blanks + one implicit blank = 3 consecutive blanks
        // This should create 2 violations (one at 2nd blank, one at 3rd blank)
        let input_triple = "Line one\nLine two\n\n\n";
        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input_triple);
        let violations = linter.analyze();
        assert_eq!(
            2,
            violations.len(),
            "Triple trailing newline (three consecutive blanks) should create 2 violations"
        );

        for violation in &violations {
            assert_eq!("MD012", violation.rule().id);
            assert!(violation
                .message()
                .contains("Multiple consecutive blank lines"));
        }
    }

    #[test]
    fn test_beginning_and_end_of_document() {
        // Multiple blank lines at the beginning should violate
        let input_beginning = "\n\nLine one\nLine two";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(
            PathBuf::from("test.md"),
            config.clone(),
            input_beginning,
        );
        let violations = linter.analyze();
        // 2 blank lines = 1 violation (when 2nd blank line is reached)
        assert_eq!(1, violations.len());

        // Multiple blank lines at the end should violate
        let input_end = "Line one\nLine two\n\n\n";

        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input_end);
        let violations = linter.analyze();
        // 3 blank lines (including the implicit one from trailing newline) = 2 violations
        assert_eq!(2, violations.len());
    }
}
