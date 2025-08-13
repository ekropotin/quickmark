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
    fn get_ignore_lines(&self) -> std::collections::HashSet<usize> {
        let mut ignore_lines = std::collections::HashSet::new();
        let node_cache = self.context.node_cache.borrow();

        // Get cached nodes for code blocks and HTML blocks
        if let Some(fenced_blocks) = node_cache.get("fenced_code_block") {
            for node_info in fenced_blocks {
                let start_line = node_info.line_start + 1;
                let end_line = node_info.line_end + 1;
                for line_num in start_line..=end_line {
                    ignore_lines.insert(line_num);
                }
            }
        }

        if let Some(indented_blocks) = node_cache.get("indented_code_block") {
            for node_info in indented_blocks {
                let start_line = node_info.line_start + 1;
                let end_line = node_info.line_end + 1;
                for line_num in start_line..=end_line {
                    ignore_lines.insert(line_num);
                }
            }
        }

        if let Some(html_blocks) = node_cache.get("html_block") {
            for node_info in html_blocks {
                let start_line = node_info.line_start + 1;
                let end_line = node_info.line_end + 1;
                for line_num in start_line..=end_line {
                    ignore_lines.insert(line_num);
                }
            }
        }

        ignore_lines
    }

    fn is_md018_violation(&self, line: &str) -> bool {
        // Pattern from original: /^#+[^# \t]/.test(line) && !/#\s*$/.test(line) && !line.startsWith("#️⃣")

        // Check if line starts with one or more # followed by non-space, non-tab, non-# character
        let trimmed = line.trim_start();

        if !trimmed.starts_with('#') {
            return false;
        }

        // Find the end of the hash sequence
        let hash_end = trimmed.chars().take_while(|&c| c == '#').count();
        if hash_end == 0 {
            return false;
        }

        // Check if line is only hashes and whitespace
        if trimmed.trim_end().chars().all(|c| c == '#') {
            return false;
        }

        // Get the character immediately after the hashes
        let chars: Vec<char> = trimmed.chars().collect();
        if hash_end >= chars.len() {
            return false; // Line ends with hashes only
        }

        let char_after_hashes = chars[hash_end];

        // Check if the character after hashes is NOT a space or tab
        if char_after_hashes != ' ' && char_after_hashes != '\t' && char_after_hashes != '#' {
            // Additional check: ignore emoji hashtag pattern #️⃣ (not ##️⃣ or others)
            if line.starts_with("#️⃣") {
                return false;
            }
            return true;
        }

        false
    }

    fn create_violation_for_line(&self, line: &str, line_number: usize) -> RuleViolation {
        RuleViolation::new(
            &MD018,
            MD018.description.to_string(),
            self.context.file_path.clone(),
            range_from_tree_sitter(&tree_sitter::Range {
                start_byte: 0,
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
        test_config_with_rules(vec![
            ("no-missing-space-atx", RuleSeverity::Error),
            ("heading-style", RuleSeverity::Off),
        ])
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
        let input = "#\n##\n###";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_hash_with_only_whitespace_ignored() {
        let input = "#   \n##  \n### \t";

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
        let input = "```\n#NoSpaceHere\n```";

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
        let input = "<div>\n#NoSpaceHere\n</div>";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_multiple_violations() {
        let input = "#Heading 1\n##Heading 2\n### Proper heading\n####Heading 4";

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
        let input = "# Valid heading 1\n#Invalid heading\n## Valid heading 2\n###Also invalid";

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
