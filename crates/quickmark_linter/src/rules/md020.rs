use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use std::rc::Rc;
use tree_sitter::Node;

use crate::linter::{range_from_tree_sitter, Context, RuleLinter, RuleViolation};

use super::{Rule, RuleType};

static CLOSED_ATX_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(#+)([ \t]*)([^# \t\\]|[^# \t][^#]*?[^# \t\\])([ \t]*)((?:\\#)?)(#+)(\s*)$")
        .expect("Invalid regex for MD020")
});

pub(crate) struct MD020Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD020Linter {
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

            if let Some(violation) = self.check_line(line, line_index) {
                self.violations.push(violation);
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

    fn check_line(&self, line: &str, line_index: usize) -> Option<RuleViolation> {
        if let Some(captures) = CLOSED_ATX_REGEX.captures(line) {
            let left_space = captures.get(2).unwrap().as_str();
            let right_space = captures.get(4).unwrap().as_str();
            let right_escape = captures.get(5).unwrap().as_str();

            let missing_left_space = left_space.is_empty();
            let missing_right_space = right_space.is_empty() || !right_escape.is_empty();

            if missing_left_space || missing_right_space {
                return Some(self.create_violation_for_line(line, line_index));
            }
        }
        None
    }

    fn create_violation_for_line(&self, line: &str, line_index: usize) -> RuleViolation {
        RuleViolation::new(
            &MD020,
            MD020.description.to_string(),
            self.context.file_path.clone(),
            range_from_tree_sitter(&tree_sitter::Range {
                start_byte: 0,
                end_byte: line.len(),
                start_point: tree_sitter::Point {
                    row: line_index,
                    column: 0,
                },
                end_point: tree_sitter::Point {
                    row: line_index,
                    column: line.len(),
                },
            }),
        )
    }
}

impl RuleLinter for MD020Linter {
    fn feed(&mut self, node: &Node) {
        // For line-based rules, we analyze all lines at once when we see the document node.
        if node.kind() == "document" {
            self.analyze_all_lines();
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD020: Rule = Rule {
    id: "MD020",
    alias: "no-missing-space-closed-atx",
    tags: &["headings", "atx_closed", "spaces"],
    description: "No space inside hashes on closed atx style heading",
    rule_type: RuleType::Line,
    required_nodes: &[], // Line-based rules don't require specific nodes
    new_linter: |context| Box::new(MD020Linter::new(context)),
};

#[cfg(test)]
mod test {
    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;
    use std::path::PathBuf;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![("no-missing-space-closed-atx", RuleSeverity::Error)])
    }

    #[test]
    fn test_md020_missing_space_left_side() {
        let config = test_config();
        let input = "#Heading 1#";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("No space inside hashes"));
    }

    #[test]
    fn test_md020_missing_space_right_side() {
        let config = test_config();
        let input = "# Heading 1#";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("No space inside hashes"));
    }

    #[test]
    fn test_md020_missing_space_both_sides() {
        let config = test_config();
        let input = "##Heading 2##";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("No space inside hashes"));
    }

    #[test]
    fn test_md020_correct_spacing() {
        let config = test_config();
        let input = "# Heading 1 #\n## Heading 2 ##\n### Heading 3 ###";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_md020_open_atx_headings_ignored() {
        let config = test_config();
        let input = "# Open Heading 1\n## Open Heading 2\n### Open Heading 3";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_md020_setext_headings_ignored() {
        let config = test_config();
        let input = "Setext Heading 1\n================\n\nSetext Heading 2\n----------------";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_md020_escaped_hash() {
        let config = test_config();
        let input = "## Heading \\##";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("No space inside hashes"));
    }

    #[test]
    fn test_md020_escaped_hash_with_space() {
        let config = test_config();
        let input = "## Heading \\# ##";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_md020_multiple_violations_in_file() {
        let config = test_config();
        let input = "#Heading 1#\n\n## Heading 2##\n\n###Heading 3###\n\n#### Correct Heading ####";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 3);
    }

    #[test]
    fn test_md020_code_blocks_ignored() {
        let config = test_config();
        let input =
            "```\n#BadHeading#\n##AnotherBad##\n```\n\n    #IndentedCodeBad#\n\n# Good Heading #";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_md020_html_flow_ignored() {
        let config = test_config();
        let input = "<div>\n#BadHeading#\n##AnotherBad##\n</div>\n\n# Good Heading #";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_md020_trailing_spaces() {
        let config = test_config();
        let input = "# Heading 1 #   \n## Heading 2 ##\t\n### Heading 3 ###\n";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_md020_unbalanced_closing_hashes() {
        let config = test_config();
        let input = "# Heading 1 ########\n## Heading 2##########\n### Heading 3 #";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1); // Only the second one violates (missing space before #)
    }

    #[test]
    fn test_md020_tabs_as_spaces() {
        let config = test_config();
        let input = "#\tHeading 1\t#\n##\t\tHeading 2\t##\n###   Heading 3   ###";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_md020_mixed_whitespace() {
        let config = test_config();
        let input = "# \tHeading 1 \t#\n##  Heading 2\t ##\n### \t Heading 3 \t ###";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_md020_content_with_hashes() {
        let config = test_config();
        let input = "# Heading with # hash #\n## Another # heading ##\n### Multiple ## hashes ###";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_md020_empty_heading() {
        let config = test_config();
        let input = "# #\n## ##\n### ###";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        // Empty headings should be ignored or handled by other rules
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_md020_complex_content() {
        let config = test_config();
        let input = "# Complex *italic* **bold** `code` content #\n## Link [text](url) content ##\n### Image ![alt](src) content ###";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        assert_eq!(linter.analyze().len(), 0);
    }
}