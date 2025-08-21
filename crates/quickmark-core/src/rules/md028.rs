use std::collections::HashSet;
use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

/// MD028 Blank lines inside blockquote Rule Linter
///
/// **SINGLE-USE CONTRACT**: This linter is designed for one-time use only.
/// After processing a document (via feed() calls and finalize()), the linter
/// should be discarded. The violations state is not cleared between uses.
pub(crate) struct MD028Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD028Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    fn analyze_all_lines(&mut self) {
        let code_block_lines = self.get_code_block_lines();
        let lines = self.context.lines.borrow();

        let mut last_line_was_blockquote = false;
        let mut blank_line_sequence_start: Option<usize> = None;

        for (i, line) in lines.iter().enumerate() {
            if code_block_lines.contains(&(i + 1)) {
                last_line_was_blockquote = false;
                blank_line_sequence_start = None;
                continue;
            }

            if self.is_blockquote_line(line) {
                if let Some(blank_idx) = blank_line_sequence_start {
                    self.violations.push(RuleViolation::new(
                        &MD028,
                        "Blank line inside blockquote".to_string(),
                        self.context.file_path.clone(),
                        range_from_tree_sitter(&tree_sitter::Range {
                            start_byte: 0,
                            end_byte: 0,
                            start_point: tree_sitter::Point {
                                row: blank_idx,
                                column: 0,
                            },
                            end_point: tree_sitter::Point {
                                row: blank_idx,
                                column: lines[blank_idx].len(),
                            },
                        }),
                    ));
                }
                last_line_was_blockquote = true;
                blank_line_sequence_start = None;
            } else if self.is_blank_line(line) {
                if last_line_was_blockquote && blank_line_sequence_start.is_none() {
                    blank_line_sequence_start = Some(i);
                }
            } else {
                last_line_was_blockquote = false;
                blank_line_sequence_start = None;
            }
        }
    }

    fn get_code_block_lines(&self) -> HashSet<usize> {
        let node_cache = self.context.node_cache.borrow();
        let mut code_block_lines = HashSet::new();
        let node_types = ["indented_code_block", "fenced_code_block", "html_block"];
        for node_type in &node_types {
            if let Some(nodes) = node_cache.get(*node_type) {
                for node_info in nodes {
                    code_block_lines.extend((node_info.line_start + 1)..=(node_info.line_end + 1));
                }
            }
        }
        code_block_lines
    }

    fn is_blockquote_line(&self, line: &str) -> bool {
        line.trim_start().starts_with('>')
    }

    fn is_blank_line(&self, line: &str) -> bool {
        line.trim().is_empty()
    }
}

impl RuleLinter for MD028Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "document" {
            self.analyze_all_lines();
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD028: Rule = Rule {
    id: "MD028",
    alias: "no-blanks-blockquote",
    tags: &["blockquote", "whitespace"],
    description: "Blank lines inside blockquotes",
    rule_type: RuleType::Hybrid,
    required_nodes: &[
        "document",
        "indented_code_block",
        "fenced_code_block",
        "html_block",
    ],
    new_linter: |context| Box::new(MD028Linter::new(context)),
};

#[cfg(test)]
mod tests {
    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;
    use std::path::PathBuf;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![
            ("no-blanks-blockquote", RuleSeverity::Error),
            ("heading-style", RuleSeverity::Off),
            ("heading-increment", RuleSeverity::Off),
        ])
    }

    #[test]
    fn test_md028_violation_basic() {
        let input = r#"> First blockquote

> Second blockquote"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // This test should fail initially (TDD approach)
        assert!(
            !violations.is_empty(),
            "Should detect blank line inside blockquote"
        );
    }

    #[test]
    fn test_md028_valid_continuous_blockquote() {
        let input = r#"> First line
> Second line"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // This should not violate - continuous blockquote
        assert!(
            violations.is_empty(),
            "Should not violate for continuous blockquote"
        );
    }

    #[test]
    fn test_md028_valid_separated_with_content() {
        let input = r#"> First blockquote

Some text here.

> Second blockquote"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // This should not violate - properly separated with content
        assert!(
            violations.is_empty(),
            "Should not violate when blockquotes are separated with content"
        );
    }

    #[test]
    fn test_md028_valid_continuous_with_blank_line_marker() {
        let input = r#"> First line
>
> Second line"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // This should not violate - blank line with blockquote marker
        assert!(
            violations.is_empty(),
            "Should not violate when blank line has blockquote marker"
        );
    }

    #[test]
    fn test_md028_violation_multiple_blank_lines() {
        let input = r#"> First blockquote


> Second blockquote"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // This should violate - multiple blank lines between blockquotes
        assert!(
            !violations.is_empty(),
            "Should detect multiple blank lines inside blockquote"
        );
    }

    #[test]
    fn test_md028_violation_nested_blockquotes() {
        let input = r#"> First level
> > Second level

> > Another second level"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // This should violate - blank line in nested blockquotes
        assert!(
            !violations.is_empty(),
            "Should detect blank lines in nested blockquotes"
        );
    }
}
