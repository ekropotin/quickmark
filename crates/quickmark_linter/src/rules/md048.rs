use serde::Deserialize;
use std::rc::Rc;
use tree_sitter::Node;

use crate::linter::{CharPosition, Context, Range, RuleLinter, RuleViolation};

use super::{Rule, RuleType};

// MD048-specific configuration types
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum CodeFenceStyle {
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "backtick")]
    Backtick,
    #[serde(rename = "tilde")]
    Tilde,
}

impl Default for CodeFenceStyle {
    fn default() -> Self {
        Self::Consistent
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD048CodeFenceStyleTable {
    #[serde(default)]
    pub style: CodeFenceStyle,
}

impl Default for MD048CodeFenceStyleTable {
    fn default() -> Self {
        Self {
            style: CodeFenceStyle::Consistent,
        }
    }
}

const VIOLATION_MESSAGE: &str = "Code fence style";

pub(crate) struct MD048Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
    expected_style: Option<CodeFenceStyle>,
}

impl MD048Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
            expected_style: None,
        }
    }

    fn analyze_all_fenced_code_blocks(&mut self) {
        let configured_style = self
            .context
            .config
            .linters
            .settings
            .code_fence_style
            .style
            .clone();

        self.context
            .node_cache
            .borrow_mut()
            .entry("fenced_code_block".to_string())
            .or_default()
            .sort_by_key(|node_info| node_info.line_start);

        let fenced_blocks = self
            .context
            .node_cache
            .borrow()
            .get("fenced_code_block")
            .cloned()
            .unwrap_or_default();

        for node_info in &fenced_blocks {
            self.check_fenced_code_block(node_info, &configured_style);
        }
    }

    fn check_fenced_code_block(
        &mut self,
        node_info: &crate::linter::NodeInfo,
        configured_style: &CodeFenceStyle,
    ) {
        // Get the fence marker from the first line of the fenced code block
        let line_start = node_info.line_start;
        if let Some(line) = self.context.lines.borrow().get(line_start) {
            let trimmed_line = line.trim_start();
            let fence_marker = if trimmed_line.starts_with("```") {
                CodeFenceStyle::Backtick
            } else if trimmed_line.starts_with("~~~") {
                CodeFenceStyle::Tilde
            } else {
                return;
            };

            let expected_style = match configured_style {
                CodeFenceStyle::Consistent => self
                    .expected_style
                    .get_or_insert_with(|| fence_marker.clone()),
                _ => configured_style,
            };

            if &fence_marker != expected_style {
                let range = Range {
                    start: CharPosition {
                        line: line_start,
                        character: 0,
                    },
                    end: CharPosition {
                        line: line_start,
                        character: 0, // Will be updated with actual content
                    },
                };

                self.violations.push(RuleViolation::new(
                    &MD048,
                    VIOLATION_MESSAGE.to_string(),
                    self.context.file_path.clone(),
                    range,
                ));
            }
        }
    }
}

impl RuleLinter for MD048Linter {
    fn feed(&mut self, _node: &Node) {
        // This is a document-level rule. All processing is in `finalize`.
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        self.analyze_all_fenced_code_blocks();
        std::mem::take(&mut self.violations)
    }
}

pub const MD048: Rule = Rule {
    id: "MD048",
    alias: "code-fence-style",
    tags: &["code"],
    description: "Code fence style",
    rule_type: RuleType::Document,
    required_nodes: &["fenced_code_block"],
    new_linter: |context| Box::new(MD048Linter::new(context)),
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
                ("code-fence-style", RuleSeverity::Error),
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

```python
# First fenced block with backticks
```

More text.

~~~javascript
// Second fenced block with tildes
~~~";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Code fence style"));
        assert_eq!(violations[0].location().range.start.line, 8); // tilde block line
    }

    #[test]
    fn test_no_violation_consistent_style_all_backticks() {
        let config = test_config();

        let input = "Some text.

```python
# First fenced block with backticks
```

More text.

```javascript
// Second fenced block with backticks
```";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_no_violation_consistent_style_all_tildes() {
        let config = test_config();

        let input = "Some text.

~~~python
# First fenced block with tildes
~~~

More text.

~~~javascript
// Second fenced block with tildes
~~~";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_violation_backtick_style_with_tildes() {
        use crate::config::{CodeFenceStyle, MD048CodeFenceStyleTable};

        let mut config = test_config();
        config.linters.settings.code_fence_style = MD048CodeFenceStyleTable {
            style: CodeFenceStyle::Backtick,
        };

        let input = "Some text.

~~~python
# Tilde fenced block when backticks expected
~~~

More text.

```javascript
// Backtick fenced block - this is ok
```";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Code fence style"));
        assert_eq!(violations[0].location().range.start.line, 2); // tilde block line
    }

    #[test]
    fn test_violation_tilde_style_with_backticks() {
        use crate::config::{CodeFenceStyle, MD048CodeFenceStyleTable};

        let mut config = test_config();
        config.linters.settings.code_fence_style = MD048CodeFenceStyleTable {
            style: CodeFenceStyle::Tilde,
        };

        let input = "Some text.

```python
# Backtick fenced block when tildes expected
```

More text.

~~~javascript
// Tilde fenced block - this is ok
~~~";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Code fence style"));
        assert_eq!(violations[0].location().range.start.line, 2); // backtick block line
    }

    #[test]
    fn test_no_violation_single_code_block() {
        let config = test_config();

        let input = "Some text.

```python
# Single fenced block with backticks
```

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

```python
# First backtick block
```

Text between

~~~javascript
# First tilde block
~~~

More text

```rust
# Second backtick block
```

~~~go
# Second tilde block
~~~";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should have violations for the tilde blocks since backticks were first
        assert_eq!(2, violations.len());
        // Both violations should be for tilde blocks
        assert_eq!(violations[0].location().range.start.line, 8); // first tilde block
        assert_eq!(violations[1].location().range.start.line, 18); // second tilde block
    }
}
