use regex::Regex;
use std::rc::Rc;
use tree_sitter::Node;

use crate::linter::{CharPosition, Context, Range, RuleLinter, RuleViolation};

use super::{Rule, RuleType};

const VIOLATION_MESSAGE: &str = "Dollar signs used before commands without showing output";

pub(crate) struct MD014Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
    dollar_regex: Regex,
}

impl MD014Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
            dollar_regex: Regex::new(r"^(\s*)\$\s+").unwrap(),
        }
    }

    /// Analyze all code blocks using cached nodes
    fn analyze_all_code_blocks(&mut self) {
        let node_cache = self.context.node_cache.borrow();
        let lines = self.context.lines.borrow();

        // Check fenced code blocks
        if let Some(fenced_blocks) = node_cache.get("fenced_code_block") {
            for node_info in fenced_blocks {
                if let Some(violation) = self.check_code_block_info(node_info, &lines, true) {
                    self.violations.push(violation);
                }
            }
        }

        // Check indented code blocks
        if let Some(indented_blocks) = node_cache.get("indented_code_block") {
            for node_info in indented_blocks {
                if let Some(violation) = self.check_code_block_info(node_info, &lines, false) {
                    self.violations.push(violation);
                }
            }
        }
    }

    fn check_code_block_info(
        &self,
        node_info: &crate::linter::NodeInfo,
        lines: &[String],
        is_fenced: bool,
    ) -> Option<RuleViolation> {
        let start_line = node_info.line_start;
        let end_line = node_info.line_end;

        // Extract content lines from the code block
        let mut content_lines = Vec::new();

        // For fenced code blocks, skip the fence lines
        let (content_start, content_end) = if is_fenced {
            // Skip first and last line (fence markers)
            (start_line + 1, end_line.saturating_sub(1))
        } else {
            // For indented code blocks, include all lines
            (start_line, end_line)
        };

        // Collect non-empty lines
        for line_idx in content_start..=content_end {
            if line_idx < lines.len() {
                let line = &lines[line_idx];
                if !line.trim().is_empty() {
                    // For indented code blocks, filter lines that don't have proper indentation
                    // This works around tree-sitter-md parsing inconsistencies
                    if !is_fenced {
                        // Check if line starts with at least 4 spaces (indented code block requirement)
                        if !line.starts_with("    ") && !line.starts_with(' ') {
                            continue;
                        }
                    }
                    content_lines.push((line_idx, line));
                }
            }
        }

        // If no non-empty lines, no violation
        if content_lines.is_empty() {
            return None;
        }

        // Check if ALL non-empty lines start with dollar sign
        let all_have_dollar = content_lines
            .iter()
            .all(|(_, line)| self.dollar_regex.is_match(line));

        if all_have_dollar {
            // Report violation on the first line with dollar sign
            if let Some((first_line_idx, first_line)) = content_lines.first() {
                let range = Range {
                    start: CharPosition {
                        line: *first_line_idx,
                        character: 0,
                    },
                    end: CharPosition {
                        line: *first_line_idx,
                        character: first_line.len(),
                    },
                };

                return Some(RuleViolation::new(
                    &MD014,
                    VIOLATION_MESSAGE.to_string(),
                    self.context.file_path.clone(),
                    range,
                ));
            }
        }

        None
    }
}

impl RuleLinter for MD014Linter {
    fn feed(&mut self, node: &Node) {
        // This is a document-level rule, so we run the analysis when we see the document node.
        if node.kind() == "document" {
            self.analyze_all_code_blocks();
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD014: Rule = Rule {
    id: "MD014",
    alias: "commands-show-output",
    tags: &["code"],
    description: "Dollar signs used before commands without showing output",
    rule_type: RuleType::Document,
    required_nodes: &["fenced_code_block", "indented_code_block"],
    new_linter: |context| Box::new(MD014Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_settings;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![
                ("commands-show-output", RuleSeverity::Error),
                ("heading-style", RuleSeverity::Off),
                ("heading-increment", RuleSeverity::Off),
            ],
            Default::default(),
        )
    }

    #[test]
    fn test_violation_all_lines_with_dollar_signs() {
        let config = test_config();

        let input = "```bash
$ git status
$ ls -la
$ pwd
```";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Dollar signs"));
    }

    #[test]
    fn test_no_violation_with_command_output() {
        let config = test_config();

        let input = "```bash
$ git status
On branch main
nothing to commit

$ ls -la
total 8
drwxr-xr-x 2 user user 4096 Jan 1 00:00 .
```";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_no_violation_no_dollar_signs() {
        let config = test_config();

        let input = "```bash
git status
ls -la
pwd
```";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_violation_indented_code_block() {
        let config = test_config();

        let input = "Some text:

    $ git status
    $ ls -la
    $ pwd

More text.";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Dollar signs"));
    }

    #[test]
    fn test_no_violation_mixed_dollar_signs() {
        let config = test_config();

        let input = "```bash
$ git status
ls -la
$ pwd
```";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_violation_with_whitespace_before_dollar() {
        let config = test_config();

        let input = "```bash
  $ git status
  $ ls -la
  $ pwd
```";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Dollar signs"));
    }

    #[test]
    fn test_no_violation_empty_code_block() {
        let config = test_config();

        let input = "```bash
```";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_no_violation_blank_lines_only() {
        let config = test_config();

        let input = "```bash



```";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_violation_with_blank_lines_between_commands() {
        let config = test_config();

        let input = "```bash
$ git status

$ ls -la

$ pwd
```";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Dollar signs"));
    }
}
