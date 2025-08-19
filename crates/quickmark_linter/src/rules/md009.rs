use serde::Deserialize;
use std::collections::HashSet;
use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

// MD009-specific configuration types
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD009TrailingSpacesTable {
    #[serde(default)]
    pub br_spaces: usize,
    #[serde(default)]
    pub list_item_empty_lines: bool,
    #[serde(default)]
    pub strict: bool,
}

impl Default for MD009TrailingSpacesTable {
    fn default() -> Self {
        Self {
            br_spaces: 2,
            list_item_empty_lines: false,
            strict: false,
        }
    }
}

/// MD009 Trailing Spaces Rule Linter
///
/// **SINGLE-USE CONTRACT**: This linter is designed for one-time use only.
/// After processing a document (via feed() calls and finalize()), the linter
/// should be discarded. The violations state is not cleared between uses.
pub(crate) struct MD009Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD009Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    /// Analyze all lines and store all violations for reporting via finalize()
    /// Context cache is already initialized by MultiRuleLinter
    fn analyze_all_lines(&mut self) {
        let settings = &self.context.config.linters.settings.trailing_spaces;
        let lines = self.context.lines.borrow();

        // Determine effective br_spaces (< 2 becomes 0)
        let expected_spaces = if settings.br_spaces < 2 {
            0
        } else {
            settings.br_spaces
        };

        // Build sets of line numbers to exclude
        let code_block_lines = self.get_code_block_lines();
        let list_item_empty_lines = if settings.list_item_empty_lines {
            self.get_list_item_empty_lines()
        } else {
            HashSet::new()
        };

        for (line_index, line) in lines.iter().enumerate() {
            let line_number = line_index + 1;
            let trailing_spaces = line.len() - line.trim_end().len();

            if trailing_spaces > 0
                && !code_block_lines.contains(&line_number)
                && !list_item_empty_lines.contains(&line_number)
            {
                let followed_by_blank_line = lines
                    .get(line_index + 1)
                    .is_some_and(|next_line| next_line.trim().is_empty());

                if self.should_violate(
                    trailing_spaces,
                    expected_spaces,
                    settings.strict,
                    settings.br_spaces,
                    followed_by_blank_line,
                ) {
                    let violation =
                        self.create_violation(line_index, line, trailing_spaces, expected_spaces);
                    self.violations.push(violation);
                }
            }
        }
    }

    /// Returns a set of line numbers that are part of code blocks.
    /// This is performant as it uses the pre-parsed node cache.
    fn get_code_block_lines(&self) -> HashSet<usize> {
        let node_cache = self.context.node_cache.borrow();
        ["indented_code_block", "fenced_code_block"]
            .iter()
            .filter_map(|kind| node_cache.get(*kind))
            .flatten()
            .flat_map(|node_info| (node_info.line_start + 1)..=(node_info.line_end + 1))
            .collect()
    }

    /// Returns a set of line numbers for empty lines within list items.
    /// This is more robust and performant than manual parsing, as it relies on the AST.
    fn get_list_item_empty_lines(&self) -> HashSet<usize> {
        let node_cache = self.context.node_cache.borrow();
        let lines = self.context.lines.borrow();

        node_cache.get("list").map_or_else(HashSet::new, |lists| {
            lists
                .iter()
                .flat_map(|node_info| (node_info.line_start + 1)..=(node_info.line_end + 1))
                .filter(|&line_num| {
                    let line_index = line_num - 1;
                    lines
                        .get(line_index)
                        .is_some_and(|line| line.trim().is_empty())
                })
                .collect()
        })
    }

    /// Determines if a line with trailing spaces constitutes a violation.
    fn should_violate(
        &self,
        trailing_spaces: usize,
        expected_spaces: usize,
        strict: bool,
        br_spaces: usize,
        followed_by_blank_line: bool,
    ) -> bool {
        if strict {
            // In strict mode, there's an exception for `br_spaces` followed by a blank line.
            if br_spaces >= 2 && trailing_spaces == br_spaces && followed_by_blank_line {
                return false;
            }
            // Otherwise, any trailing space is a violation in strict mode.
            return true;
        }

        // In non-strict mode, a violation occurs if the number of trailing spaces
        // is not the amount expected for a hard line break.
        trailing_spaces != expected_spaces
    }

    /// Creates a RuleViolation with a correctly calculated range.
    fn create_violation(
        &self,
        line_index: usize,
        line: &str,
        trailing_spaces: usize,
        expected_spaces: usize,
    ) -> RuleViolation {
        let message = if expected_spaces == 0 {
            format!("Expected: 0 trailing spaces; Actual: {trailing_spaces}")
        } else {
            format!("Expected: 0 or {expected_spaces} trailing spaces; Actual: {trailing_spaces}")
        };

        let start_column = line.trim_end().len();
        let end_column = line.len();

        RuleViolation::new(
            &MD009,
            message,
            self.context.file_path.clone(),
            range_from_tree_sitter(&tree_sitter::Range {
                // FIXME: Byte offsets are not correctly calculated as line start offset is unavailable here.
                // This may result in incorrect highlighting in some tools.
                // The primary information is in the points (row/column).
                start_byte: 0,
                end_byte: 0,
                start_point: tree_sitter::Point {
                    row: line_index,
                    column: start_column,
                },
                end_point: tree_sitter::Point {
                    row: line_index,
                    column: end_column,
                },
            }),
        )
    }
}

impl RuleLinter for MD009Linter {
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

pub const MD009: Rule = Rule {
    id: "MD009",
    alias: "no-trailing-spaces",
    tags: &["whitespace"],
    description: "Trailing spaces",
    rule_type: RuleType::Line,
    // This is a line-based rule and does not require specific nodes from the AST.
    // The logic runs once for the entire file content.
    required_nodes: &[],
    new_linter: |context| Box::new(MD009Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{LintersSettingsTable, MD009TrailingSpacesTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::{test_config_with_rules, test_config_with_settings};

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![
            ("no-trailing-spaces", RuleSeverity::Error),
            ("heading-style", RuleSeverity::Off),
            ("heading-increment", RuleSeverity::Off),
        ])
    }

    fn test_config_with_trailing_spaces(
        trailing_spaces_config: MD009TrailingSpacesTable,
    ) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![
                ("no-trailing-spaces", RuleSeverity::Error),
                ("heading-style", RuleSeverity::Off),
                ("heading-increment", RuleSeverity::Off),
            ],
            LintersSettingsTable {
                trailing_spaces: trailing_spaces_config,
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_basic_trailing_space_violation() {
        #[rustfmt::skip]
        let input = "This line has trailing spaces   "; // 3 spaces should violate

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        let violation = &violations[0];
        assert_eq!("MD009", violation.rule().id);
        assert!(violation.message().contains("Expected:"));
        assert!(violation.message().contains("Actual: 3"));
    }

    #[test]
    fn test_no_trailing_spaces() {
        let input = "This line has no trailing spaces";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_single_trailing_space() {
        #[rustfmt::skip]
        let input = "This line has one trailing space ";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_two_spaces_allowed_by_default() {
        #[rustfmt::skip]
        let input = "This line has two trailing spaces for line break  ";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Default br_spaces = 2, so this should be allowed
    }

    #[test]
    fn test_three_spaces_violation() {
        #[rustfmt::skip]
        let input = "This line has three trailing spaces   ";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_custom_br_spaces() {
        let config = test_config_with_trailing_spaces(MD009TrailingSpacesTable {
            br_spaces: 4,
            list_item_empty_lines: false,
            strict: false,
        });

        #[rustfmt::skip]
        let input_allowed = "This line has four trailing spaces    ";
        let mut linter = MultiRuleLinter::new_for_document(
            PathBuf::from("test.md"),
            config.clone(),
            input_allowed,
        );
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Should be allowed

        #[rustfmt::skip]
        let input_violation = "This line has five trailing spaces     ";
        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input_violation);
        let violations = linter.analyze();
        assert_eq!(1, violations.len()); // Should violate
    }

    #[test]
    fn test_strict_mode() {
        let config = test_config_with_trailing_spaces(MD009TrailingSpacesTable {
            br_spaces: 2,
            list_item_empty_lines: false,
            strict: true,
        });

        // In strict mode, even allowed trailing spaces should be violations
        // if they don't actually create a line break
        #[rustfmt::skip]
        let input = "This line has two trailing spaces but no line break after  ";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len()); // Should violate in strict mode
    }

    #[test]
    fn test_br_spaces_less_than_two() {
        let config = test_config_with_trailing_spaces(MD009TrailingSpacesTable {
            br_spaces: 1,
            list_item_empty_lines: false,
            strict: false,
        });

        // When br_spaces < 2, it should behave like br_spaces = 0
        #[rustfmt::skip]
        let input = "Single trailing space ";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len()); // Should violate
    }

    #[test]
    fn test_indented_code_block_excluded() {
        #[rustfmt::skip]
        let input = "    This is an indented code block with trailing spaces  ";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Code blocks should be excluded
    }

    #[test]
    fn test_fenced_code_block_excluded() {
        #[rustfmt::skip]
        let input = r#"```rust
fn main() {
    println!("Hello");  
}
```"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Fenced code blocks should be excluded
    }

    #[test]
    fn test_list_item_empty_lines() {
        let config = test_config_with_trailing_spaces(MD009TrailingSpacesTable {
            br_spaces: 2,
            list_item_empty_lines: true,
            strict: false,
        });

        #[rustfmt::skip]
        let input = r#"- item 1
 
  - item 2"#; // Empty line with 1 space in list

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Should be allowed when list_item_empty_lines = true
    }

    #[test]
    fn test_list_item_empty_lines_disabled() {
        let config = test_config(); // Default has list_item_empty_lines = false

        #[rustfmt::skip]
        let input = r#"- item 1
 
  - item 2"#; // Empty line with 1 space in list

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len()); // Should violate when list_item_empty_lines = false
    }

    #[test]
    fn test_multiple_lines_mixed() {
        #[rustfmt::skip]
        let input = r#"Line without trailing spaces
Line with single space 
Line with two spaces  
Line with three spaces   
Normal line again"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len()); // Single space and three spaces should violate
    }

    #[test]
    fn test_empty_line_with_spaces() {
        // Test with 2 spaces (default br_spaces) - should NOT violate
        #[rustfmt::skip]
        let input_2_spaces = r#"Line one
  
Line three"#; // Middle line has 2 spaces

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(
            PathBuf::from("test.md"),
            config.clone(),
            input_2_spaces,
        );
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // 2 spaces should be allowed by default

        // Test with 3 spaces - should violate
        #[rustfmt::skip]
        let input_3_spaces = r#"Line one
   
Line three"#; // Middle line has 3 spaces

        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input_3_spaces);
        let violations = linter.analyze();
        assert_eq!(1, violations.len()); // 3 spaces should violate
    }

    #[test]
    fn test_strict_mode_paragraph_detection_parity() {
        // This test captures a discrepancy found between quickmark and markdownlint
        // In strict mode, markdownlint only flags trailing spaces that don't create actual line breaks

        let config = test_config_with_trailing_spaces(MD009TrailingSpacesTable {
            br_spaces: 2,
            list_item_empty_lines: false,
            strict: true,
        });

        // NOTE: The trailing spaces are significant in this input string.
        #[rustfmt::skip]
        let input = r#"This line has no trailing spaces
This line has two trailing spaces for line break  

Paragraph with proper line break  
Next line continues the paragraph.

Normal paragraph without any trailing spaces."#;

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Based on markdownlint behavior:
        // - Line 2: has 2 spaces followed by empty line - creates actual line break, should NOT violate in strict
        // - Line 4: has 2 spaces followed by continuation - does NOT create line break, SHOULD violate in strict
        assert_eq!(
            1,
            violations.len(),
            "Expected 1 violation (line 4 only) to match markdownlint behavior"
        );

        // Verify the violation is on the correct line
        let line_numbers: Vec<usize> = violations
            .iter()
            .map(|v| v.location().range.start.line + 1)
            .collect();

        // Only line 4 should be reported (trailing spaces that don't create actual line breaks)
        assert!(
            line_numbers.contains(&4),
            "Line 4 should be reported (trailing spaces before paragraph continuation)"
        );
        assert!(!line_numbers.contains(&2), "Line 2 should NOT be reported (trailing spaces before empty line create actual line break)");
    }
}
