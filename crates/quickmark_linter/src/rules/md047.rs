use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

/// MD047 Single Trailing Newline Rule Linter
///
/// **SINGLE-USE CONTRACT**: This linter is designed for one-time use only.
/// After processing a document (via feed() calls and finalize()), the linter
/// should be discarded. The violations state is not cleared between uses.
pub(crate) struct MD047Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD047Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    /// Analyze the last line to check if file ends with newline
    fn analyze_last_line(&mut self) {
        let lines = self.context.lines.borrow();

        if lines.is_empty() {
            return;
        }

        let last_line_index = lines.len() - 1;
        let last_line = &lines[last_line_index];

        if !self.is_blank_line(last_line) {
            let violation = self.create_violation(last_line_index, last_line);
            self.violations.push(violation);
        }
    }

    /// Check if a line is "blank" according to markdownlint's logic.
    /// A line is blank if it's empty or consists of only whitespace,
    /// blockquote markers (`>`), or HTML comments (`<!-- ... -->`).
    /// This implementation is optimized to avoid string allocations.
    fn is_blank_line(&self, mut line: &str) -> bool {
        loop {
            line = line.trim_start(); // Skips leading whitespace

            if line.is_empty() {
                return true;
            }

            if line.starts_with('>') {
                line = &line[1..];
                continue;
            }

            if line.starts_with("<!--") {
                if let Some(end_index) = line.find("-->") {
                    line = &line[end_index + 3..];
                    continue;
                }
                // Unmatched "<!--" means the rest of the line is a comment
                return true;
            }

            // Anything else is considered content
            return false;
        }
    }

    /// Create a violation for a line that doesn't end with newline
    fn create_violation(&self, line_index: usize, line: &str) -> RuleViolation {
        RuleViolation::new(
            &MD047,
            MD047.description.to_string(),
            self.context.file_path.clone(),
            range_from_tree_sitter(&tree_sitter::Range {
                start_byte: 0,
                end_byte: line.len(),
                start_point: tree_sitter::Point {
                    row: line_index,
                    column: line.len(),
                },
                end_point: tree_sitter::Point {
                    row: line_index,
                    column: line.len() + 1,
                },
            }),
        )
    }
}

impl RuleLinter for MD047Linter {
    fn feed(&mut self, node: &Node) {
        // This rule is line-based and only needs to run once.
        // We trigger the analysis on seeing the top-level `document` node.
        if node.kind() == "document" {
            self.analyze_last_line();
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD047: Rule = Rule {
    id: "MD047",
    alias: "single-trailing-newline",
    tags: &["blank_lines"],
    description: "Files should end with a single newline character",
    rule_type: RuleType::Line,
    required_nodes: &[], // Line-based rules don't require specific nodes
    new_linter: |context| Box::new(MD047Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![
            ("single-trailing-newline", RuleSeverity::Error),
            ("heading-style", RuleSeverity::Off),
            ("heading-increment", RuleSeverity::Off),
        ])
    }

    #[test]
    fn test_file_without_trailing_newline() {
        let input = "This file does not end with a newline";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        let violation = &violations[0];
        assert_eq!("MD047", violation.rule().id);
        assert_eq!(
            "Files should end with a single newline character",
            violation.message()
        );
    }

    #[test]
    fn test_file_with_trailing_newline() {
        let input = "This file ends with a newline\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_file_with_multiple_trailing_newlines() {
        let input = "This file has multiple newlines\n\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Should not violate - ends with newline
    }

    #[test]
    fn test_empty_file() {
        let input = "";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Empty file shouldn't violate
    }

    #[test]
    fn test_file_with_only_newline() {
        let input = "\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Single newline should not violate
    }

    #[test]
    fn test_file_with_whitespace_last_line() {
        let input = "Content\n   \n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Whitespace-only last line should not violate
    }

    #[test]
    fn test_file_ending_with_html_comment() {
        let input = "Content\n<!-- This is a comment -->\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // HTML comment on last line should not violate if ends with newline
    }

    #[test]
    fn test_file_ending_with_html_comment_no_newline() {
        let input = "Content\n<!-- This is a comment -->";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // HTML comment only should be considered blank
    }

    #[test]
    fn test_file_ending_with_blockquote_markers() {
        let input = "Content\n>>>\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Blockquote markers only should not violate
    }

    #[test]
    fn test_file_ending_with_blockquote_markers_no_newline() {
        let input = "Content\n>>>";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Blockquote markers only should be considered blank
    }

    #[test]
    fn test_file_ending_with_mixed_comments_and_blockquotes() {
        let input = "Content\n<!-- comment -->>\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Mixed comments and blockquotes should not violate
    }

    #[test]
    fn test_multiple_lines_last_without_newline() {
        let input = "Line 1\nLine 2\nLast line without newline";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        let violation = &violations[0];
        assert_eq!("MD047", violation.rule().id);
        // Should point to the end of the last line
        assert_eq!(2, violation.location().range.start.line); // 0-indexed, so line 2 = third line
    }
}
