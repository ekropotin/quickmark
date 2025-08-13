use std::collections::HashSet;
use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

pub(crate) struct MD018Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD018Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    /// Analyze all lines and store all violations for reporting via finalize()
    fn analyze_all_lines(&mut self) {
        let lines = self.context.lines.borrow();

        // We need to identify lines that are in code blocks or HTML blocks to ignore them
        let ignore_lines = self.get_ignore_lines();

        for (line_index, line) in lines.iter().enumerate() {
            if ignore_lines.contains(&(line_index + 1)) {
                continue; // Skip lines in code blocks or HTML blocks
            }

            if self.is_md018_violation(line) {
                let violation = self.create_violation_for_line(line, line_index);
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

    fn is_md018_violation(&self, line: &str) -> bool {
        let trimmed = line.trim_start();

        if !trimmed.starts_with('#') {
            return false;
        }

        if trimmed.starts_with("#️⃣") {
            return false;
        }

        let hash_count = trimmed.chars().take_while(|&c| c == '#').count();
        if hash_count == 0 {
            return false;
        }

        match trimmed.chars().nth(hash_count) {
            None => false,                  // Line consists only of hashes (e.g., "###")
            Some(' ') | Some('\t') => false, // Correctly formatted with a space or tab
            Some(_) => true,                // Any other character indicates a missing space
        }
    }

    fn create_violation_for_line(&self, line: &str, line_number: usize) -> RuleViolation {
        RuleViolation::new(
            &MD018,
            MD018.description.to_string(),
            self.context.file_path.clone(),
            range_from_tree_sitter(&tree_sitter::Range {
                start_byte: 0, // Note: byte offsets are not correctly handled here
                end_byte: line.len(),
                start_point: tree_sitter::Point {
                    row: line_number,
                    column: 0,
                },
                end_point: tree_sitter::Point {
                    row: line_number,
                    column: line.len(),
                },
            }),
        )
    }
}

impl RuleLinter for MD018Linter {
    fn feed(&mut self, node: &Node) {
        // Analyze all lines when we see the document node
        if node.kind() == "document" {
            self.analyze_all_lines();
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD018: Rule = Rule {
    id: "MD018",
    alias: "no-missing-space-atx",
    tags: &["atx", "headings", "spaces"],
    description: "No space after hash on atx style heading",
    rule_type: RuleType::Line,
    required_nodes: &[], // Line-based rules don't require specific nodes
    new_linter: |context| Box::new(MD018Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![("no-missing-space-atx", RuleSeverity::Error), ("heading-style", RuleSeverity::Off)])
    }

    #[test]
    fn test_missing_space_after_hash() {
        let input = "#Heading 1";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        let violation = &violations[0];
        assert_eq!("MD018", violation.rule().id);
        assert!(violation.message().contains("No space after hash"));
    }

    #[test]
    fn test_missing_space_after_multiple_hashes() {
        let input = "##Heading 2";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_proper_space_after_hash() {
        let input = "# Heading 1";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_proper_space_after_multiple_hashes() {
        let input = "## Heading 2";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_hash_only_lines_ignored() {
        let input = "#
##
###";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_hash_with_only_whitespace_ignored() {
        let input = "#   
##  
### 	";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_emoji_hashtag_ignored() {
        let input = "#️⃣ This should not trigger";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_code_blocks_ignored() {
        let input = "```
#NoSpaceHere
```";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_indented_code_blocks_ignored() {
        let input = "    #NoSpaceHere";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_html_blocks_ignored() {
        let input = "<div>
#NoSpaceHere
</div>";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_multiple_violations() {
        let input = "#Heading 1
##Heading 2
### Proper heading
####Heading 4";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(3, violations.len());

        // Check violation line numbers
        assert_eq!(0, violations[0].location().range.start.line);
        assert_eq!(1, violations[1].location().range.start.line);
        assert_eq!(3, violations[2].location().range.start.line);
    }

    #[test]
    fn test_mixed_valid_invalid() {
        let input = "# Valid heading 1
#Invalid heading
## Valid heading 2
###Also invalid";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len());

        // Check violation line numbers
        assert_eq!(1, violations[0].location().range.start.line);
        assert_eq!(3, violations[1].location().range.start.line);
    }

    #[test]
    fn test_hash_not_at_start_of_line() {
        let input = "Some text #NotAHeading";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }
}