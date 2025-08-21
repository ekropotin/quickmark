use std::rc::Rc;
use tree_sitter::Node;

use crate::linter::{range_from_tree_sitter, Context, RuleLinter, RuleViolation};

use super::{Rule, RuleType};

// Pre-computed violation messages to avoid format! allocations
const MISSING_BLANK_BEFORE: &str =
    "Lists should be surrounded by blank lines [Missing blank line before]";
const MISSING_BLANK_AFTER: &str =
    "Lists should be surrounded by blank lines [Missing blank line after]";

pub(crate) struct MD032Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD032Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    /// Check if a line is blank, handling out-of-bounds safely and considering blockquote context.
    /// Out-of-bounds lines are considered blank to avoid false violations at document boundaries.
    /// Lines containing only blockquote markers (e.g., "> " or ">") are considered blank.
    #[inline]
    fn is_line_blank_cached(&self, line_number: usize, lines: &[String]) -> bool {
        if line_number < lines.len() {
            let line = &lines[line_number];
            let trimmed = line.trim();

            // Regular blank line
            if trimmed.is_empty() {
                return true;
            }

            // Check if this is a blockquote marker line (just >, >>, etc.)
            if trimmed == ">" || trimmed.chars().all(|c| c == '>') {
                return true;
            }

            // Check if this is a blockquote with only spaces ("> ", ">> ", etc.)
            if trimmed.starts_with('>') && trimmed.trim_start_matches('>').trim().is_empty() {
                return true;
            }

            false
        } else {
            true // Consider out-of-bounds lines as blank
        }
    }

    /// Check if a node is within another list structure by traversing up the AST.
    /// This helps identify top-level lists vs nested lists.
    /// Lists within blockquotes are still considered "top-level" for MD032 purposes.
    #[inline]
    fn is_top_level_list(&self, node: &Node) -> bool {
        let mut current = node.parent();
        while let Some(parent) = current {
            match parent.kind() {
                "list" => return false, // Found parent list, so this is nested
                // Stop searching when we hit document-level containers
                "document" | "block_quote" => return true,
                _ => current = parent.parent(),
            }
        }
        true // No parent list found, this is top-level
    }

    /// Find the visual end line of the list by examining actual content
    /// This approach looks at the lines themselves rather than relying solely on tree-sitter boundaries
    fn find_visual_end_line(&self, node: &Node) -> usize {
        let start_line = node.start_position().row;
        let tree_sitter_end_line = node.end_position().row;

        // Borrow lines to examine content
        let lines = self.context.lines.borrow();

        // For blockquoted lists, we need to handle them differently
        // If this is a blockquoted list, trust tree-sitter more
        if lines
            .get(start_line)
            .is_some_and(|line| line.trim_start().starts_with('>'))
        {
            // This is a blockquoted list - be more conservative with tree-sitter boundaries
            // but still exclude trailing blank blockquote lines
            for line_idx in (start_line..=tree_sitter_end_line).rev() {
                if line_idx < lines.len() {
                    let line = &lines[line_idx];
                    let after_quote = line.trim_start_matches('>').trim();

                    // If this line has meaningful content within the blockquote
                    if !after_quote.is_empty() {
                        return line_idx;
                    }
                }
            }
        } else {
            // Regular list - use the existing content-based detection
            for line_idx in (start_line..=tree_sitter_end_line).rev() {
                if line_idx < lines.len() {
                    let line = &lines[line_idx];
                    let trimmed = line.trim();

                    // If this line has content and looks like it could be part of a list item
                    if !trimmed.is_empty() {
                        // Check if it's definitely NOT a block element
                        let is_thematic_break = trimmed.len() >= 3
                            && (trimmed.chars().all(|c| c == '-')
                                || trimmed.chars().all(|c| c == '*')
                                || trimmed.chars().all(|c| c == '_'));

                        let is_block_element = trimmed.starts_with('#') || // headings
                            trimmed.starts_with("```") || trimmed.starts_with("~~~") || // code blocks
                            is_thematic_break; // thematic breaks

                        if !is_block_element {
                            return line_idx;
                        }
                    }
                }
            }
        }

        // Fallback to node's start line if no content found
        start_line
    }

    fn check_list(&mut self, node: &Node) {
        // Only check top-level lists
        if !self.is_top_level_list(node) {
            return;
        }

        let start_line = node.start_position().row;
        let end_line = self.find_visual_end_line(node);

        // Single borrow for the entire function to avoid multiple RefCell runtime checks
        let lines = self.context.lines.borrow();
        let total_lines = lines.len();

        // Check blank line above (only if not at document start)
        if start_line > 0 {
            let line_above = start_line - 1;
            if !self.is_line_blank_cached(line_above, &lines) {
                self.violations.push(RuleViolation::new(
                    &MD032,
                    MISSING_BLANK_BEFORE.to_string(),
                    self.context.file_path.clone(),
                    range_from_tree_sitter(&node.range()),
                ));
            }
        }

        // Check blank line below (following original markdownlint logic)
        // The original checks lines[lastLineNumber] where lastLineNumber is the line after the list
        let line_after_list_idx = end_line + 1;
        if line_after_list_idx < total_lines {
            let is_blank = self.is_line_blank_cached(line_after_list_idx, &lines);

            // If the line immediately after the list is not blank, report a violation
            // This matches the original markdownlint behavior exactly
            if !is_blank {
                self.violations.push(RuleViolation::new(
                    &MD032,
                    MISSING_BLANK_AFTER.to_string(),
                    self.context.file_path.clone(),
                    range_from_tree_sitter(&node.range()),
                ));
            }
        }
    }
}

impl RuleLinter for MD032Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "list" {
            self.check_list(node);
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD032: Rule = Rule {
    id: "MD032",
    alias: "blanks-around-lists",
    tags: &["blank_lines", "bullet", "ol", "ul"],
    description: "Lists should be surrounded by blank lines",
    rule_type: RuleType::Hybrid,
    required_nodes: &["list"],
    new_linter: |context| Box::new(MD032Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_settings;

    fn test_config_default() -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![
                ("blanks-around-lists", RuleSeverity::Error),
                ("heading-style", RuleSeverity::Off),
                ("heading-increment", RuleSeverity::Off),
            ],
            Default::default(),
        )
    }

    #[test]
    fn test_no_violation_proper_blanks() {
        let config = test_config_default();

        let input = "Some text

* List item
* List item

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_violation_missing_blank_above() {
        let config = test_config_default();

        let input = "Some text
* List item
* List item

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("blank line before"));
    }

    #[test]
    fn test_violation_missing_blank_below() {
        let config = test_config_default();

        // Use a thematic break instead of paragraph text to avoid lazy continuation
        let input = "Some text

* List item
* List item
---";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("blank line after"));
    }

    #[test]
    fn test_violation_missing_both_blanks() {
        let config = test_config_default();

        // Use a thematic break to avoid lazy continuation
        let input = "Some text
* List item
* List item
---";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len());
        assert!(violations[0].message().contains("blank line"));
        assert!(violations[1].message().contains("blank line"));
    }

    #[test]
    fn test_no_violation_at_document_start() {
        let config = test_config_default();

        let input = "* List item
* List item

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_no_violation_at_document_end() {
        let config = test_config_default();

        let input = "Some text

* List item
* List item";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_ordered_list_violations() {
        let config = test_config_default();

        let input = "Some text
1. List item
2. List item
---";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len()); // Both missing blank above and below
    }

    #[test]
    fn test_mixed_list_markers() {
        let config = test_config_default();

        let input = "Some text
+ List item
- List item
More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Original markdownlint detects 3 violations:
        // + List item (missing blank before and after), - List item (missing blank before)
        assert_eq!(3, violations.len());
    }

    #[test]
    fn test_nested_lists_no_violation() {
        let config = test_config_default();

        let input = "Some text

* List item
  * Nested item
  * Nested item
* List item

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should not report violations for nested lists, only top-level
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_lists_in_blockquotes() {
        let config = test_config_default();

        let input = "> Some text
>
> * List item
> * List item
>
> More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should handle blockquote context properly
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_lists_in_blockquotes_violation() {
        let config = test_config_default();

        let input = "> Some text
> * List item
> * List item
> More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should detect violations even in blockquotes (only missing blank before due to lazy continuation)
        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_list_with_horizontal_rule_before() {
        let config = test_config_default();

        let input = "Some text

---
* List item
* List item

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // HR immediately before list should trigger violation
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("blank line before"));
    }

    #[test]
    fn test_list_with_horizontal_rule_after() {
        let config = test_config_default();

        let input = "Some text

* List item
* List item
---

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // HR immediately after list should trigger violation
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("blank line after"));
    }

    #[test]
    fn test_list_with_code_block_before() {
        let config = test_config_default();

        let input = "Some text

```
code
```
* List item
* List item

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Code block immediately before list should trigger violation
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("blank line before"));
    }

    #[test]
    fn test_list_with_code_block_after() {
        let config = test_config_default();

        let input = "Some text

* List item
* List item
```
code
```

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Code block immediately after list should trigger violation
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("blank line after"));
    }

    #[test]
    fn test_lazy_continuation_line() {
        let config = test_config_default();

        let input = "Some text

1. List item
   More item 1
2. List item
More item 2

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // "More item 2" is a lazy continuation line, should not trigger violation
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_list_at_document_boundaries_complete() {
        let config = test_config_default();

        let input = "* List item
* List item";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // List spans entire document - no violations expected
        assert_eq!(0, violations.len());
    }
}
