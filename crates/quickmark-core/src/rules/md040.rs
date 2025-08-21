use serde::Deserialize;
use std::collections::HashSet;
use std::rc::Rc;
use tree_sitter::Node;

use crate::{
    linter::{CharPosition, Context, Range, RuleLinter, RuleViolation},
    rules::{Rule, RuleType},
};

// MD040-specific configuration types
#[derive(Debug, PartialEq, Clone, Deserialize, Default)]
pub struct MD040FencedCodeLanguageTable {
    #[serde(default)]
    pub allowed_languages: Vec<String>,
    #[serde(default)]
    pub language_only: bool,
}

pub(crate) struct MD040Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD040Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    /// Extracts the language identifier from a fenced code block's first line.
    /// This handles common variations like attributes (e.g., ```rust{{...}}).
    /// Returns `(Option<language>, has_extra_info)`. The language is a slice
    /// of the input line to avoid allocations.
    fn extract_code_block_language<'a>(&self, line: &'a str) -> (Option<&'a str>, bool) {
        let trimmed = line.trim_start();
        let marker = if trimmed.starts_with("```") {
            "```"
        } else if trimmed.starts_with("~~~") {
            "~~~"
        } else {
            return (None, false);
        };

        let info_string = trimmed[marker.len()..].trim();

        if info_string.is_empty() {
            return (None, false);
        }

        let mut parts = info_string.split_whitespace();
        // The unwrap is safe because we've checked that info_string is not empty.
        let language_part = parts.next().unwrap();
        let has_extra_info = parts.next().is_some();

        // The unwrap is safe because split always returns an iterator with at least one element.
        let language = language_part.split('{').next().unwrap();

        if language.is_empty() {
            (None, has_extra_info)
        } else {
            (Some(language), has_extra_info)
        }
    }
}

impl RuleLinter for MD040Linter {
    fn feed(&mut self, _node: &Node) {
        // MD040 uses Document pattern, not Token pattern
        // All processing happens in finalize()
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        let config = &self.context.config.linters.settings.fenced_code_language;
        let node_cache = self.context.node_cache.borrow();
        let lines = self.context.lines.borrow();

        // For performance, convert allowed_languages to a HashSet if it's not empty.
        let allowed_languages_set: Option<HashSet<&str>> = if !config.allowed_languages.is_empty() {
            Some(
                config
                    .allowed_languages
                    .iter()
                    .map(String::as_str)
                    .collect(),
            )
        } else {
            None
        };

        if let Some(fenced_code_blocks) = node_cache.get("fenced_code_block") {
            for node_info in fenced_code_blocks {
                if let Some(first_line) = lines.get(node_info.line_start) {
                    let (language_opt, has_extra_info) =
                        self.extract_code_block_language(first_line);

                    let range = Range {
                        start: CharPosition {
                            line: node_info.line_start,
                            character: 0,
                        },
                        end: CharPosition {
                            line: node_info.line_start,
                            character: first_line.len(),
                        },
                    };

                    let language = match language_opt {
                        Some(lang) => lang,
                        None => {
                            self.violations.push(RuleViolation::new(
                                &MD040,
                                "Fenced code blocks should have a language specified".to_string(),
                                self.context.file_path.clone(),
                                range,
                            ));
                            continue;
                        }
                    };

                    if let Some(set) = &allowed_languages_set {
                        if !set.contains(language) {
                            self.violations.push(RuleViolation::new(
                                &MD040,
                                format!("\"{language}\" is not allowed"),
                                self.context.file_path.clone(),
                                range,
                            ));
                            continue;
                        }
                    }

                    // Check if language_only is true and there's extra metadata
                    if config.language_only && has_extra_info {
                        let range = Range {
                            start: CharPosition {
                                line: node_info.line_start,
                                character: 0,
                            },
                            end: CharPosition {
                                line: node_info.line_start,
                                character: first_line.len(),
                            },
                        };
                        let violation = RuleViolation::new(
                            &MD040,
                            format!(
                                "Info string contains more than language: \"{}\"",
                                first_line.trim()
                            ),
                            self.context.file_path.clone(),
                            range,
                        );
                        self.violations.push(violation);
                    }
                }
            }
        }

        std::mem::take(&mut self.violations)
    }
}

pub const MD040: Rule = Rule {
    id: "MD040",
    alias: "fenced-code-language",
    tags: &["code", "language"],
    description: "Fenced code blocks should have a language specified",
    rule_type: RuleType::Document,
    required_nodes: &["fenced_code_block"],
    new_linter: |context| Box::new(MD040Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{LintersSettingsTable, MD040FencedCodeLanguageTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_settings;

    fn test_config_default() -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![("fenced-code-language", RuleSeverity::Error)],
            LintersSettingsTable {
                fenced_code_language: MD040FencedCodeLanguageTable {
                    allowed_languages: vec![],
                    language_only: false,
                },
                ..Default::default()
            },
        )
    }

    fn test_config_with_allowed_languages(
        allowed_languages: Vec<&str>,
    ) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![("fenced-code-language", RuleSeverity::Error)],
            LintersSettingsTable {
                fenced_code_language: MD040FencedCodeLanguageTable {
                    allowed_languages: allowed_languages.iter().map(|s| s.to_string()).collect(),
                    language_only: false,
                },
                ..Default::default()
            },
        )
    }

    fn test_config_with_language_only(language_only: bool) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![("fenced-code-language", RuleSeverity::Error)],
            LintersSettingsTable {
                fenced_code_language: MD040FencedCodeLanguageTable {
                    allowed_languages: vec![],
                    language_only,
                },
                ..Default::default()
            },
        )
    }

    fn test_config_with_both_options(
        allowed_languages: Vec<&str>,
        language_only: bool,
    ) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![("fenced-code-language", RuleSeverity::Error)],
            LintersSettingsTable {
                fenced_code_language: MD040FencedCodeLanguageTable {
                    allowed_languages: allowed_languages.iter().map(|s| s.to_string()).collect(),
                    language_only,
                },
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_fenced_code_with_language_no_violations() {
        let config = test_config_default();
        let input = "# Test

```rust
fn main() {
    println!(\"Hello, World!\");
}
```

```javascript
console.log('Hello, World!');
```

```text
Plain text content
```";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md040_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD040")
            .collect();
        assert_eq!(md040_violations.len(), 0);
    }

    #[test]
    fn test_fenced_code_without_language_violations() {
        let config = test_config_default();
        let input = "# Test

```
def hello():
    print(\"Hello, World!\")
```

```rust
fn main() {
    println!(\"Hello, World!\");
}
```

```
console.log('Hello, World!');
```";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md040_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD040")
            .collect();

        // Should find 2 violations: the two fenced code blocks without languages
        assert_eq!(md040_violations.len(), 2);
    }

    #[test]
    fn test_allowed_languages_specific_list() {
        let config = test_config_with_allowed_languages(vec!["rust", "python"]);
        let input = "# Test

```rust
fn main() {}
```

```python
def hello(): pass
```

```javascript
console.log('not allowed');
```

```
no language specified
```";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md040_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD040")
            .collect();

        // Should find 2 violations: javascript (not in allowed list) and no language
        assert_eq!(md040_violations.len(), 2);
        assert!(md040_violations
            .iter()
            .any(|v| v.message().contains("javascript")));
    }

    #[test]
    fn test_language_only_option_no_extra_info() {
        let config = test_config_with_language_only(true);
        let input = "# Test

```rust
fn main() {}
```

```python {.line-numbers}
def hello(): pass
```

```javascript copy
console.log('Hello');
```";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md040_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD040")
            .collect();

        // Should find 2 violations: python and javascript have extra info beyond language
        assert_eq!(md040_violations.len(), 2);
    }

    #[test]
    fn test_language_only_option_language_only_allowed() {
        let config = test_config_with_language_only(true);
        let input = "# Test

```rust
fn main() {}
```

```python
def hello(): pass
```";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md040_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD040")
            .collect();

        // Should find no violations: both have only language specified
        assert_eq!(md040_violations.len(), 0);
    }

    #[test]
    fn test_combined_options() {
        let config = test_config_with_both_options(vec!["rust", "python"], true);
        let input = "# Test

```rust
fn main() {}
```

```python copy
def hello(): pass
```

```javascript
console.log('Hello');
```

```
no language
```";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md040_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD040")
            .collect();

        // Should find 3 violations:
        // 1. python has extra info (violates language_only)
        // 2. javascript not in allowed list
        // 3. no language specified
        assert_eq!(md040_violations.len(), 3);
    }

    #[test]
    fn test_indented_code_blocks_ignored() {
        let config = test_config_default();
        let input = "# Test

    def hello():
        print(\"This is indented code\")

```
def hello():
    print(\"This is fenced code without language\")
```";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md040_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD040")
            .collect();

        // Should find only 1 violation: the fenced code block without language
        // Indented code blocks should be ignored
        assert_eq!(md040_violations.len(), 1);
    }

    #[test]
    fn test_case_sensitivity_in_languages() {
        let config = test_config_with_allowed_languages(vec!["rust", "PYTHON"]);
        let input = "# Test

```Rust
fn main() {}
```

```python
def hello(): pass
```

```PYTHON
def hello(): pass
```";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md040_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD040")
            .collect();

        // Should find 2 violations: "Rust" and "python" don't match case-sensitive allowed list
        assert_eq!(md040_violations.len(), 2);
    }

    #[test]
    fn test_empty_fenced_code_blocks() {
        let config = test_config_default();
        let input = "# Test

```

```

```rust

```";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md040_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD040")
            .collect();

        // Should find 1 violation: the first block has no language
        assert_eq!(md040_violations.len(), 1);
    }

    #[test]
    fn test_tildes_fenced_code_blocks() {
        let config = test_config_default();
        let input = "# Test

~~~
def hello():
    print(\"Hello\")
~~~

~~~python
def hello():
    print(\"Hello\")
~~~";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md040_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD040")
            .collect();

        // Should find 1 violation: the first block has no language
        assert_eq!(md040_violations.len(), 1);
    }
}
