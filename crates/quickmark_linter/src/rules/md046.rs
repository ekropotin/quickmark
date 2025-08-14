use std::rc::Rc;
use tree_sitter::Node;

use crate::config::CodeBlockStyle;
use crate::linter::{CharPosition, Context, Range, RuleLinter, RuleViolation};

use super::{Rule, RuleType};

const VIOLATION_MESSAGE: &str = "Code block style";

pub(crate) struct MD046Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
    expected_style: Option<CodeBlockStyle>,
}

impl MD046Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
            expected_style: None,
        }
    }

    fn analyze_all_code_blocks(&mut self) {
        let configured_style = self
            .context
            .config
            .linters
            .settings
            .code_block_style
            .style
            .clone();

        let all_code_blocks = {
            let node_cache = self.context.node_cache.borrow();
            let mut all_code_blocks = Vec::new();

            if let Some(fenced_blocks) = node_cache.get("fenced_code_block") {
                all_code_blocks.extend(
                    fenced_blocks
                        .iter()
                        .map(|n| (n.clone(), CodeBlockStyle::Fenced)),
                );
            }

            if let Some(indented_blocks) = node_cache.get("indented_code_block") {
                all_code_blocks.extend(
                    indented_blocks
                        .iter()
                        .map(|n| (n.clone(), CodeBlockStyle::Indented)),
                );
            }

            all_code_blocks.sort_by_key(|(node_info, _)| node_info.line_start);
            all_code_blocks
        };

        for (node_info, block_style) in all_code_blocks {
            self.check_code_block(&node_info, block_style, &configured_style);
        }
    }

    fn check_code_block(
        &mut self,
        node_info: &crate::linter::NodeInfo,
        block_style: CodeBlockStyle,
        configured_style: &CodeBlockStyle,
    ) {
        let expected_style = if *configured_style == CodeBlockStyle::Consistent {
            if self.expected_style.is_none() {
                self.expected_style = Some(block_style.clone());
            }
            self.expected_style.as_ref().unwrap()
        } else {
            configured_style
        };

        if block_style != *expected_style {
            let range = Range {
                start: CharPosition {
                    line: node_info.line_start,
                    character: 0,
                },
                end: CharPosition {
                    line: node_info.line_start,
                    character: 0, // Will be updated with actual content
                },
            };

            self.violations.push(RuleViolation::new(
                &MD046,
                VIOLATION_MESSAGE.to_string(),
                self.context.file_path.clone(),
                range,
            ));
        }
    }
}

impl RuleLinter for MD046Linter {
    fn feed(&mut self, _node: &Node) {
        // This is a document-level rule. All processing is in `finalize`.
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        self.analyze_all_code_blocks();
        std::mem::take(&mut self.violations)
    }
}

pub const MD046: Rule = Rule {
    id: "MD046",
    alias: "code-block-style",
    tags: &["code"],
    description: "Code block style",
    rule_type: RuleType::Document,
    required_nodes: &["fenced_code_block", "indented_code_block"],
    new_linter: |context| Box::new(MD046Linter::new(context)),
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
                ("code-block-style", RuleSeverity::Error),
                ("heading-style", RuleSeverity::Off),
                ("heading-increment", RuleSeverity::Off),
            ],
            Default::default(),
        )
    }

    #[test]
    fn test_violation_consistent_style_mixed() {
        let config = test_config();

        let input = "Some text.

    This is a
    code block.

And here is more text

```text
and here is a different
code block
```";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Code block style"));
    }

    #[test]
    fn test_no_violation_consistent_style_all_fenced() {
        let config = test_config();

        let input = "Some text.

```text
This is a fenced code block.
```

And here is more text

```text
and here is another fenced code block
```";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_no_violation_consistent_style_all_indented() {
        let config = test_config();

        let input = "Some text.

    This is an indented
    code block.

And here is more text

    And this is another
    indented code block";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_violation_fenced_style_with_indented() {
        use crate::config::{CodeBlockStyle, MD046CodeBlockStyleTable};

        let mut config = test_config();
        config.linters.settings.code_block_style = MD046CodeBlockStyleTable {
            style: CodeBlockStyle::Fenced,
        };

        let input = "Some text.

    This is an indented
    code block.

And here is more text

```text
and here is a fenced code block
```";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Code block style"));
        assert_eq!(violations[0].location().range.start.line, 2); // indented code block at line 3 (0-indexed)
    }

    #[test]
    fn test_violation_indented_style_with_fenced() {
        use crate::config::{CodeBlockStyle, MD046CodeBlockStyleTable};

        let mut config = test_config();
        config.linters.settings.code_block_style = MD046CodeBlockStyleTable {
            style: CodeBlockStyle::Indented,
        };

        let input = "Some text.

```text
This is a fenced code block
```

And here is more text

    This is an indented
    code block";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Code block style"));
        assert_eq!(violations[0].location().range.start.line, 2); // fenced code block at line 3 (0-indexed)
    }

    #[test]
    fn test_no_violation_single_code_block() {
        let config = test_config();

        let input = "Some text.

    This is an indented
    code block.

No other code blocks.";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_no_violation_no_code_blocks() {
        let config = test_config();

        let input = "Some text.

Just regular paragraphs.

No code blocks at all.";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_violation_multiple_inconsistent_blocks() {
        let config = test_config();

        let input = "Some text.

    First indented block

Text between

```text
First fenced block
```

More text

    Second indented block

```javascript
Second fenced block
```";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should have violations for the fenced blocks since indented was first
        assert_eq!(2, violations.len());
        // Both violations should be for fenced blocks
        assert_eq!(violations[0].location().range.start.line, 6); // first fenced block
        assert_eq!(violations[1].location().range.start.line, 14); // second fenced block
    }
}
