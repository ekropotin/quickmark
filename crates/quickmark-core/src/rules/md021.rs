use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use std::rc::Rc;
use tree_sitter::Node;

use crate::linter::{range_from_tree_sitter, Context, RuleLinter, RuleViolation};

use super::{Rule, RuleType};

static CLOSED_ATX_REGEX: Lazy<Regex> = Lazy::new(|| {
    // Match closed ATX headings but exclude escaped hashes (consistent with original markdownlint)
    // The pattern ensures that the closing hashes are not escaped
    Regex::new(r"^(#+)([ \t]*)([^# \t\\]|[^# \t][^#]*?[^# \t\\])([ \t]*)(#+)(\s*)$")
        .expect("Invalid regex for MD021")
});

pub(crate) struct MD021Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD021Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    fn analyze_all_lines(&mut self) {
        let lines = self.context.lines.borrow();

        // Get line numbers that should be ignored (inside code blocks or HTML blocks)
        let ignore_lines = self.get_ignore_lines();

        for (line_index, line) in lines.iter().enumerate() {
            if ignore_lines.contains(&(line_index + 1)) {
                continue; // Skip lines in code blocks or HTML blocks
            }

            if let Some(mut line_violations) = self.check_line(line, line_index) {
                self.violations.append(&mut line_violations);
            }
        }
    }

    /// Get line numbers that should be ignored (inside code blocks or HTML blocks)
    fn get_ignore_lines(&self) -> HashSet<usize> {
        let mut ignore_lines = HashSet::new();
        let node_cache = self.context.node_cache.borrow();

        for node_type in ["fenced_code_block", "indented_code_block", "html_block"] {
            if let Some(blocks) = node_cache.get(node_type) {
                for node_info in blocks {
                    for line_num in (node_info.line_start + 1)..=(node_info.line_end + 1) {
                        ignore_lines.insert(line_num);
                    }
                }
            }
        }

        ignore_lines
    }

    fn check_line(&self, line: &str, line_index: usize) -> Option<Vec<RuleViolation>> {
        let mut violations = Vec::new();

        if let Some(captures) = CLOSED_ATX_REGEX.captures(line) {
            let opening_spaces = captures.get(2).unwrap().as_str();
            let closing_spaces = captures.get(4).unwrap().as_str();

            // Check for multiple spaces after opening hashes
            if opening_spaces.len() > 1 {
                let start_col = captures.get(2).unwrap().start();
                violations.push(RuleViolation::new(
                    &MD021,
                    format!(
                        "Multiple spaces inside hashes on closed atx style heading [Expected: 1; Actual: {}]",
                        opening_spaces.len()
                    ),
                    self.context.file_path.clone(),
                    // The location points to the second space, which is the beginning of the violation.
                    range_from_tree_sitter(&tree_sitter::Range {
                        start_byte: 0, // Not accurate, but line/col is used
                        end_byte: 0,
                        start_point: tree_sitter::Point { row: line_index, column: start_col + 2 },
                        end_point: tree_sitter::Point { row: line_index, column: start_col + 3 },
                    }),
                ));
            }

            // Check for multiple spaces before closing hashes
            if closing_spaces.len() > 1 {
                let start_col = captures.get(4).unwrap().start();
                violations.push(RuleViolation::new(
                    &MD021,
                    format!(
                        "Multiple spaces inside hashes on closed atx style heading [Expected: 1; Actual: {}]",
                        closing_spaces.len()
                    ),
                    self.context.file_path.clone(),
                    // The location points to the second space, which is the beginning of the violation.
                    range_from_tree_sitter(&tree_sitter::Range {
                        start_byte: 0, // Not accurate, but line/col is used
                        end_byte: 0,
                        start_point: tree_sitter::Point { row: line_index, column: start_col + 2 },
                        end_point: tree_sitter::Point { row: line_index, column: start_col + 3 },
                    }),
                ));
            }
        }

        if violations.is_empty() {
            None
        } else {
            Some(violations)
        }
    }
}

impl RuleLinter for MD021Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "document" {
            self.analyze_all_lines();
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD021: Rule = Rule {
    id: "MD021",
    alias: "no-multiple-space-closed-atx",
    tags: &["headings", "atx_closed", "spaces"],
    description: "Multiple spaces inside hashes on closed atx style heading",
    rule_type: RuleType::Line,
    required_nodes: &[],
    new_linter: |context| Box::new(MD021Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![
            ("no-multiple-space-closed-atx", RuleSeverity::Error),
            ("heading-style", RuleSeverity::Off),
            ("heading-increment", RuleSeverity::Off),
        ])
    }

    #[test]
    fn test_md021_multiple_spaces_after_opening_hashes() {
        let config = test_config();

        let input = "##  Heading with multiple spaces after opening ##\n###   Another heading ###\n####    Yet another heading ####\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect 3 violations for multiple spaces after opening hashes
        assert_eq!(violations.len(), 3);

        for violation in &violations {
            assert_eq!(violation.rule().id, "MD021");
        }
    }

    #[test]
    fn test_md021_multiple_spaces_before_closing_hashes() {
        let config = test_config();

        let input = "## Heading with multiple spaces before closing  ##\n### Another heading with spaces before closing   ###\n#### Yet another heading    ####\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect 3 violations for multiple spaces before closing hashes
        assert_eq!(violations.len(), 3);

        for violation in &violations {
            assert_eq!(violation.rule().id, "MD021");
        }
    }

    #[test]
    fn test_md021_multiple_spaces_both_sides() {
        let config = test_config();

        let input = "##  Heading with multiple spaces on both sides  ##\n###   Another heading with multiple spaces   ###\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect 4 violations: 2 for opening spaces, 2 for closing spaces
        assert_eq!(violations.len(), 4);

        for violation in &violations {
            assert_eq!(violation.rule().id, "MD021");
        }
    }

    #[test]
    fn test_md021_correct_single_spaces() {
        let config = test_config();

        let input = "# Heading with correct spacing #\n## Another heading with correct spacing ##\n### Third heading with correct spacing ###\n#### Fourth heading ####\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - single space is correct
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_md021_only_applies_to_closed_headings() {
        let config = test_config();

        let input = "# Regular ATX heading\n##  Regular ATX heading with multiple spaces\n### Regular ATX heading\n##  Closed heading with multiple spaces ##\n### Another closed heading with multiple spaces  ###\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should only detect violations for closed headings, not regular ATX headings
        // Expected: 2 violations (one for opening spaces, one for closing spaces)
        assert_eq!(violations.len(), 2);

        for violation in &violations {
            assert_eq!(violation.rule().id, "MD021");
        }
    }

    #[test]
    fn test_md021_no_spaces_around_hashes() {
        let config = test_config();

        let input = "##Heading with no spaces##\n###Another heading with no spaces###\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // MD021 only cares about multiple spaces, not missing spaces
        // No violations expected for this case
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_md021_mixed_tabs_and_spaces() {
        let config = test_config();

        let input = "##\t\tHeading with tabs after opening ##\n## Heading with spaces before closing\t\t##\n###  \tMixed tabs and spaces   ###\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect violations for any whitespace longer than 1 character
        assert_eq!(violations.len(), 4); // 2 + 1 + 1 = 4 violations

        for violation in &violations {
            assert_eq!(violation.rule().id, "MD021");
        }
    }

    #[test]
    fn test_md021_edge_case_single_hash() {
        let config = test_config();

        let input = "#  Heading with single hash and multiple spaces #\n#   Another single hash heading   #\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect 3 violations: 1 for first line opening, 1 for second line opening, 1 for second line closing
        assert_eq!(violations.len(), 3);

        for violation in &violations {
            assert_eq!(violation.rule().id, "MD021");
        }
    }

    #[test]
    fn test_md021_escaped_hash_not_detected() {
        let config = test_config();

        // These escaped hash headings should NOT trigger MD021 violations
        // (they should be ignored as they're not true closed ATX headings)
        let input = "## Multiple spaces before escaped hash  \\##\n### Multiple spaces with escaped hash  \\###\n####  Yet another escaped hash  \\####\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have NO violations - escaped hashes are not closed ATX headings for MD021
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_md021_column_positions_accuracy() {
        let config = test_config();

        // Test that column positions are reported correctly (1-based indexing)
        let input = "##  Two spaces after opening ##\n### Three spaces before closing   ###\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(violations.len(), 2);

        // First violation: opening spaces on line 1
        // Line: "##  Two spaces after opening ##"
        // Column should be 4 (the second space)
        assert_eq!(violations[0].location().range.start.line, 0);
        assert_eq!(violations[0].location().range.start.character, 4);

        // Second violation: closing spaces on line 2
        // Line: "### Three spaces before closing   ###"
        // Column should be 33 (the second space)
        assert_eq!(violations[1].location().range.start.line, 1);
        assert_eq!(violations[1].location().range.start.character, 33);
    }

    #[test]
    fn test_md021_mixed_tabs_spaces_comprehensive() {
        let config = test_config();

        // Test various combinations of tabs and spaces
        let input = "##\t\tTab after opening ##\n##  \tSpace then tab ##\n##\t Mixed tab and space\t##\n###\t  Tab and spaces  \t###\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Expected violations:
        // Line 1: 1 violation (opening: 2 tabs)
        // Line 2: 1 violation (opening: 2 spaces + 1 tab = 3 chars)
        // Line 3: 1 violation (opening: 1 tab + 1 space = 2 chars)
        // Line 4: 2 violations (opening: 1 tab + 2 spaces = 3 chars, closing: 2 spaces + 1 tab = 3 chars)
        assert_eq!(violations.len(), 5);

        for violation in &violations {
            assert_eq!(violation.rule().id, "MD021");
            // Each violation message should indicate the actual count > 1
            assert!(violation.message().contains("Actual:"));
            assert!(!violation.message().contains("Actual: 1]")); // None should be exactly 1
        }
    }

    #[test]
    fn test_md021_single_vs_multiple_hash_combinations() {
        let config = test_config();

        // Test different combinations of hash counts
        let input = "#  Single hash with multiple opening spaces #\n##   Double hash with multiple opening spaces ##\n###    Triple hash with multiple opening spaces ###\n# Single hash with multiple closing spaces  #\n##  Double hash with multiple closing spaces  ##\n###   Triple hash with multiple closing spaces   ###\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Expected violations:
        // Line 1: 1 violation (opening: 2 spaces)
        // Line 2: 1 violation (opening: 3 spaces)
        // Line 3: 1 violation (opening: 4 spaces)
        // Line 4: 1 violation (closing: 2 spaces)
        // Line 5: 2 violations (opening and closing: 2 spaces each)
        // Line 6: 2 violations (opening and closing: 3 spaces each)
        assert_eq!(violations.len(), 8);

        // Verify all are MD021 violations
        for violation in &violations {
            assert_eq!(violation.rule().id, "MD021");
        }
    }

    #[test]
    fn test_md021_boundary_conditions() {
        let config = test_config();

        // Test boundary conditions: exactly 1 space (valid) vs 2+ spaces (invalid)
        let input = "# Exactly one space on both sides #\n##  Exactly two spaces after opening ##\n## Exactly two spaces before closing  ##\n###   Three spaces both sides   ###\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // First line should have NO violations (exactly 1 space is correct)
        // Other lines should have violations
        assert_eq!(violations.len(), 4);

        // Verify that the single-space line is not included in violations
        for violation in &violations {
            assert_ne!(violation.location().range.start.line, 0); // First line should not have violations
        }
    }

    #[test]
    fn test_md021_violation_message_format() {
        let config = test_config();

        // Test that violation messages contain correct actual counts
        let input = "##  Two spaces ##\n###   Three spaces   ###\n####    Four spaces    ####\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(violations.len(), 5); // Line 1: 1 violation (opening), Line 2: 2 violations, Line 3: 2 violations

        // Check that messages contain the correct counts
        let messages: Vec<String> = violations.iter().map(|v| v.message().to_string()).collect();

        // Should have messages with different actual counts
        assert!(messages.iter().any(|m| m.contains("Actual: 2]")));
        assert!(messages.iter().any(|m| m.contains("Actual: 3]")));
        assert!(messages.iter().any(|m| m.contains("Actual: 4]")));
    }

    #[test]
    fn test_md021_regex_edge_cases() {
        let config = test_config();

        // Test edge cases that might confuse the regex
        let input = "## Normal heading ##\n##  Heading with  multiple  internal  spaces ##\n###   Heading with trailing hash###\n####    Heading with unmatched hashes ###\n##### Heading with content containing # symbols #####\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Expected violations:
        // Line 1: No violations (correct spacing)
        // Line 2: 1 violation (opening: 2 spaces)
        // Line 3: 1 violation (opening: 3 spaces, no closing violation due to no space before ###)
        // Line 4: 1 violation (opening: 4 spaces, but unbalanced hashes so no closing violation)
        // Line 5: No violations (this doesn't match our regex as a closed ATX heading)

        assert_eq!(violations.len(), 3);

        for violation in &violations {
            assert_eq!(violation.rule().id, "MD021");
        }
    }

    #[test]
    fn test_md021_parity_comprehensive() {
        let config = test_config();

        // Test cases that exactly match the comprehensive test file scenarios
        let input = "##  Two spaces after opening ##\n###   Three spaces after opening ###\n## Two spaces before closing  ##\n### Three spaces before closing   ###\n##  Both sides have multiple  ##\n#  Multiple spaces after single hash #\n##\tTab after opening\t##\n##    Many spaces    ##\n###     Even more spaces     ###\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Expected violations:
        // Line 1: 1 (opening: 2 spaces)
        // Line 2: 1 (opening: 3 spaces)
        // Line 3: 1 (closing: 2 spaces)
        // Line 4: 1 (closing: 3 spaces)
        // Line 5: 2 (opening: 2 spaces, closing: 2 spaces)
        // Line 6: 1 (opening: 2 spaces)
        // Line 7: 0 (exactly 1 tab on both sides is valid)
        // Line 8: 2 (opening: 4 spaces, closing: 4 spaces)
        // Line 9: 2 (opening: 5 spaces, closing: 5 spaces)
        assert_eq!(violations.len(), 11);

        // Verify all violations are MD021
        for violation in &violations {
            assert_eq!(violation.rule().id, "MD021");
            assert!(violation
                .message()
                .contains("Multiple spaces inside hashes on closed atx style heading"));
        }

        // Verify column positions are 1-based and accurate
        for violation in &violations {
            assert!(violation.location().range.start.character > 0); // Should be 1-based
            assert!(violation.location().range.start.character < 50); // Reasonable column range
        }
    }

    #[test]
    fn test_md021_only_closed_not_setext() {
        let config = test_config();

        let input = "Setext Heading 1\n================\n\nSetext Heading 2\n----------------\n\n##  Closed ATX heading  ##\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should only detect violations for the closed ATX heading
        assert_eq!(violations.len(), 2); // opening and closing spaces

        for violation in &violations {
            assert_eq!(violation.rule().id, "MD021");
        }
    }
}
