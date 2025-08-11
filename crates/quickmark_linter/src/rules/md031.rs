use std::rc::Rc;
use tree_sitter::Node;

use crate::linter::{range_from_tree_sitter, Context, RuleLinter, RuleViolation};

use super::{Rule, RuleType};

// Pre-computed violation messages to avoid format! allocations
const MISSING_BLANK_BEFORE: &str =
    "Fenced code blocks should be surrounded by blank lines [Missing blank line before]";
const MISSING_BLANK_AFTER: &str =
    "Fenced code blocks should be surrounded by blank lines [Missing blank line after]";

pub(crate) struct MD031Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD031Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    /// Check if a line is blank, handling out-of-bounds safely.
    /// Out-of-bounds lines are considered blank to avoid false violations at document boundaries.
    #[inline]
    fn is_line_blank_cached(&self, line_number: usize, lines: &[String]) -> bool {
        if line_number < lines.len() {
            lines[line_number].trim().is_empty()
        } else {
            true // Consider out-of-bounds lines as blank
        }
    }

    /// Check if a node is within a list structure by traversing up the AST.
    #[inline]
    fn is_in_list(&self, node: &Node) -> bool {
        let mut current = node.parent();
        while let Some(parent) = current {
            match parent.kind() {
                "list_item" | "list" => return true,
                _ => current = parent.parent(),
            }
        }
        false
    }

    /// Check if content represents a fence closing marker.
    #[inline]
    fn is_fence_marker(content: &str) -> bool {
        content.starts_with("```") || content.starts_with("~~~")
    }

    /// Determine if the code block ends at the document boundary with a fence marker.
    #[inline]
    fn is_at_document_end_with_fence(end_line: usize, total_lines: usize, content: &str) -> bool {
        end_line >= total_lines - 1 && Self::is_fence_marker(content)
    }

    fn check_fenced_code_block(&mut self, node: &Node) {
        let config = &self.context.config.linters.settings.fenced_code_blanks;

        // Skip if list_items is false and this code block is in a list
        if !config.list_items && self.is_in_list(node) {
            return;
        }

        let start_line = node.start_position().row;
        let end_line = node.end_position().row;
        // Single borrow for the entire function to avoid multiple RefCell runtime checks
        let lines = self.context.lines.borrow();
        let total_lines = lines.len();

        // Check blank line above (only if not at document start)
        if start_line > 0 {
            let line_above = start_line - 1;
            if !self.is_line_blank_cached(line_above, &lines) {
                self.violations.push(RuleViolation::new(
                    &MD031,
                    MISSING_BLANK_BEFORE.to_string(),
                    self.context.file_path.clone(),
                    range_from_tree_sitter(&node.range()),
                ));
            }
        }

        // Check blank line below using optimized logic
        // Original markdownlint: !isBlankLine(lines[codeBlock.endLine]) && !isBlankLine(lines[codeBlock.endLine - 1])

        // Fast path: Early return if we're at document end with a fence marker
        if end_line >= total_lines {
            return; // Beyond document bounds
        }

        let end_line_content = lines[end_line].trim();
        if Self::is_at_document_end_with_fence(end_line, total_lines, end_line_content) {
            return; // At document end with fence closing - no violation
        }

        // Check for violation using cached line access
        let end_line_blank = self.is_line_blank_cached(end_line, &lines);
        let prev_line_blank = self.is_line_blank_cached(end_line.saturating_sub(1), &lines);

        if !end_line_blank && !prev_line_blank {
            self.violations.push(RuleViolation::new(
                &MD031,
                MISSING_BLANK_AFTER.to_string(),
                self.context.file_path.clone(),
                range_from_tree_sitter(&node.range()),
            ));
        }
    }
}

impl RuleLinter for MD031Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "fenced_code_block" {
            self.check_fenced_code_block(node);
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD031: Rule = Rule {
    id: "MD031",
    alias: "blanks-around-fences",
    tags: &["blank_lines", "code"],
    description: "Fenced code blocks should be surrounded by blank lines",
    rule_type: RuleType::Hybrid,
    required_nodes: &["fenced_code_block"],
    new_linter: |context| Box::new(MD031Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{LintersSettingsTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_settings;

    fn test_config_with_list_items(list_items: bool) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![
                ("blanks-around-fences", RuleSeverity::Error),
                ("heading-style", RuleSeverity::Off),
                ("heading-increment", RuleSeverity::Off),
            ],
            LintersSettingsTable {
                fenced_code_blanks: crate::config::MD031FencedCodeBlanksTable { list_items },
                ..Default::default()
            },
        )
    }

    fn test_config_default() -> crate::config::QuickmarkConfig {
        test_config_with_list_items(true)
    }

    #[test]
    fn test_no_violation_proper_blanks() {
        let config = test_config_default();

        let input = "Some text

```javascript
const x = 1;
```

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_violation_missing_blank_above() {
        let config = test_config_default();

        let input = "Some text
```javascript
const x = 1;
```

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("blank line"));
    }

    #[test]
    fn test_violation_missing_blank_below() {
        let config = test_config_default();

        let input = "Some text

```javascript
const x = 1;
```
More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("blank line"));
    }

    #[test]
    fn test_violation_missing_both_blanks() {
        let config = test_config_default();

        let input = "Some text
```javascript
const x = 1;
```
More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len());
        assert!(violations[0].message().contains("blank line"));
        assert!(violations[1].message().contains("blank line"));
    }

    #[test]
    fn test_no_violation_at_document_start() {
        let config = test_config_default();

        let input = "```javascript
const x = 1;
```

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_no_violation_at_document_end() {
        let config = test_config_default();

        let input = "Some text

```javascript
const x = 1;
```";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_tilde_fences() {
        let config = test_config_default();

        let input = "Some text
~~~javascript
const x = 1;
~~~
More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len());
    }

    #[test]
    fn test_violation_in_lists_when_enabled() {
        let config = test_config_with_list_items(true);

        let input = "1. First item
   ```javascript
   const x = 1;
   ```
2. Second item";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len()); // Should have violations in list items
    }

    #[test]
    fn test_no_violation_in_lists_when_disabled() {
        let config = test_config_with_list_items(false);

        let input = "1. First item
   ```javascript
   const x = 1;
   ```
2. Second item";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Should NOT have violations in list items
    }

    #[test]
    fn test_violation_outside_lists_when_list_items_disabled() {
        let config = test_config_with_list_items(false);

        let input = "Some text
```javascript
const x = 1;
```
More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len()); // Should still have violations outside lists
    }

    #[test]
    fn test_blockquote_fences() {
        let config = test_config_default();

        let input = "> Some text
> ```javascript
> const x = 1;
> ```
> More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len()); // Should detect violations in blockquotes
    }

    #[test]
    fn test_nested_blockquote_lists() {
        let config = test_config_with_list_items(true);

        let input = "> 1. Item
>    ```javascript
>    const x = 1;
>    ```
> 2. Item";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len()); // Should detect violations in nested structures
    }
}
