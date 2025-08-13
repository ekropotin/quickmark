use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::HashSet, rc::Rc};
use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, Context, RuleLinter, RuleViolation},
    rules::{Rule, RuleType},
};

// Memoized regex patterns for HTML tag detection
static HTML_TAG_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"<(/?)([a-zA-Z][a-zA-Z0-9]*)[^>]*/?>").expect("Invalid HTML tag regex")
});

static CODE_SPAN_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"`[^`]*`").expect("Invalid code span regex"));

pub(crate) struct MD033Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
    allowed_elements: HashSet<String>,
}

impl MD033Linter {
    pub fn new(context: Rc<Context>) -> Self {
        // Pre-process allowed elements into a HashSet for O(1) lookups
        let allowed_elements: HashSet<String> = context
            .config
            .linters
            .settings
            .inline_html
            .allowed_elements
            .iter()
            .map(|element| element.to_lowercase())
            .collect();

        Self {
            context,
            violations: Vec::new(),
            allowed_elements,
        }
    }

    fn is_allowed_element(&self, element_name: &str) -> bool {
        // O(1) lookup in pre-computed HashSet
        self.allowed_elements.contains(&element_name.to_lowercase())
    }

    fn is_in_code_context(&self, node: &Node) -> bool {
        // Check if this node is inside a code span or code block
        let mut current = node.parent();
        while let Some(parent) = current {
            match parent.kind() {
                "code_span" | "fenced_code_block" | "indented_code_block" => {
                    return true;
                }
                _ => {
                    current = parent.parent();
                }
            }
        }
        false
    }

    fn byte_to_line_col(&self, byte_pos: usize) -> (usize, usize) {
        let document_content = self.context.document_content.borrow();
        let content = document_content.as_bytes();

        let mut line = 0;
        let mut col = 0;

        for (i, &byte) in content.iter().enumerate() {
            if i >= byte_pos {
                break;
            }
            if byte == b'\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
        }

        (line, col)
    }

    fn process_html_in_node(&mut self, node: &Node) {
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let content = {
            let document_content = self.context.document_content.borrow();
            document_content[start_byte..end_byte].to_string()
        };

        // For inline nodes, we need to check for code spans within them
        if node.kind() == "inline" {
            self.process_html_in_inline_node(node);
        } else {
            // For html_block nodes, process directly
            self.process_html_with_regex(node, &content, start_byte);
        }
    }

    fn process_html_in_inline_node(&mut self, node: &Node) {
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let content = {
            let document_content = self.context.document_content.borrow();
            document_content[start_byte..end_byte].to_string()
        };

        // Find all code span ranges using memoized regex pattern
        let mut code_span_ranges = Vec::new();
        for cap in CODE_SPAN_REGEX.captures_iter(&content) {
            let span_start = cap.get(0).unwrap().start();
            let span_end = cap.get(0).unwrap().end();
            code_span_ranges.push((span_start, span_end));
        }

        // Now process HTML with regex, but exclude code span ranges
        self.process_html_with_regex_excluding_ranges(
            node,
            &content,
            start_byte,
            &code_span_ranges,
        );
    }

    fn process_html_with_regex_excluding_ranges(
        &mut self,
        _node: &Node,
        content: &str,
        start_byte: usize,
        exclude_ranges: &[(usize, usize)],
    ) {
        // Use memoized HTML tag regex pattern
        for cap in HTML_TAG_REGEX.captures_iter(content) {
            if let Some(element_name_match) = cap.get(2) {
                let tag_start = cap.get(0).unwrap().start();
                let tag_end = cap.get(0).unwrap().end();

                // Check if this tag is inside any excluded range (code span)
                let mut in_excluded_range = false;
                for &(exclude_start, exclude_end) in exclude_ranges {
                    if tag_start >= exclude_start && tag_end <= exclude_end {
                        in_excluded_range = true;
                        break;
                    }
                }

                if in_excluded_range {
                    continue;
                }

                let is_closing = cap.get(1).is_some_and(|m| m.as_str() == "/");

                // Skip closing tags - we only want to report opening/self-closing tags
                if is_closing {
                    continue;
                }

                let element_name = element_name_match.as_str();

                // Check if this element is allowed
                if !self.is_allowed_element(element_name) {
                    // Calculate precise position of the HTML tag
                    let tag_start_byte = start_byte + tag_start;
                    let tag_end_byte = start_byte + tag_end;
                    let (start_line, start_col) = self.byte_to_line_col(tag_start_byte);
                    let (end_line, end_col) = self.byte_to_line_col(tag_end_byte);

                    // Create precise tree_sitter::Range for this violation
                    let range = range_from_tree_sitter(&tree_sitter::Range {
                        start_byte: tag_start_byte,
                        end_byte: tag_end_byte,
                        start_point: tree_sitter::Point {
                            row: start_line,
                            column: start_col,
                        },
                        end_point: tree_sitter::Point {
                            row: end_line,
                            column: end_col,
                        },
                    });

                    let violation = RuleViolation::new(
                        &MD033,
                        format!("Inline HTML [Element: {element_name}]"),
                        self.context.file_path.clone(),
                        range,
                    );
                    self.violations.push(violation);
                }
            }
        }
    }

    fn process_html_with_regex(&mut self, _node: &Node, content: &str, start_byte: usize) {
        // Use memoized HTML tag regex pattern
        for cap in HTML_TAG_REGEX.captures_iter(content) {
            if let Some(element_name_match) = cap.get(2) {
                let tag_start = cap.get(0).unwrap().start();
                let tag_end = cap.get(0).unwrap().end();
                let is_closing = cap.get(1).is_some_and(|m| m.as_str() == "/");

                // Skip closing tags - we only want to report opening/self-closing tags
                if is_closing {
                    continue;
                }

                let element_name = element_name_match.as_str();

                // Check if this element is allowed
                if !self.is_allowed_element(element_name) {
                    // Calculate precise position of the HTML tag
                    let tag_start_byte = start_byte + tag_start;
                    let tag_end_byte = start_byte + tag_end;
                    let (start_line, start_col) = self.byte_to_line_col(tag_start_byte);
                    let (end_line, end_col) = self.byte_to_line_col(tag_end_byte);

                    // Create precise tree_sitter::Range for this violation
                    let range = range_from_tree_sitter(&tree_sitter::Range {
                        start_byte: tag_start_byte,
                        end_byte: tag_end_byte,
                        start_point: tree_sitter::Point {
                            row: start_line,
                            column: start_col,
                        },
                        end_point: tree_sitter::Point {
                            row: end_line,
                            column: end_col,
                        },
                    });

                    let violation = RuleViolation::new(
                        &MD033,
                        format!("Inline HTML [Element: {element_name}]"),
                        self.context.file_path.clone(),
                        range,
                    );
                    self.violations.push(violation);
                }
            }
        }
    }
}

impl RuleLinter for MD033Linter {
    fn feed(&mut self, node: &Node) {
        // Process inline and html_block nodes that may contain HTML
        match node.kind() {
            "inline" => {
                // Check if this inline node is inside a code span by looking at its parent
                if !self.is_in_code_context(node) {
                    self.process_html_in_node(node);
                }
            }
            "html_block" => {
                // HTML blocks should always be processed unless they are in code blocks
                // But html_block nodes are typically not inside code blocks by tree-sitter design
                self.process_html_in_node(node);
            }
            _ => (),
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD033: Rule = Rule {
    id: "MD033",
    alias: "no-inline-html",
    tags: &["html"],
    description: "Inline HTML",
    rule_type: RuleType::Token,
    required_nodes: &["inline", "html_block"],
    new_linter: |context| Box::new(MD033Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{LintersSettingsTable, MD033InlineHtmlTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_settings;

    fn test_config_default() -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![("no-inline-html", RuleSeverity::Error)],
            LintersSettingsTable {
                inline_html: MD033InlineHtmlTable {
                    allowed_elements: vec![],
                },
                ..Default::default()
            },
        )
    }

    fn test_config_with_allowed_elements(
        allowed_elements: Vec<&str>,
    ) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![("no-inline-html", RuleSeverity::Error)],
            LintersSettingsTable {
                inline_html: MD033InlineHtmlTable {
                    allowed_elements: allowed_elements.iter().map(|s| s.to_string()).collect(),
                },
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_no_inline_html_no_violations() {
        let config = test_config_default();
        let input = "# Regular heading

This is regular markdown with no HTML.

- List item 1
- List item 2

```text
<p>This should not trigger as it's in a code block</p>
```

Text `<code>` text (this should not trigger as it's in a code span)";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md033_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD033")
            .collect();
        assert_eq!(md033_violations.len(), 0);
    }

    #[test]
    fn test_basic_inline_html_violations() {
        let config = test_config_default();
        let input = "# Regular heading

<h1>Inline HTML Heading</h1>

<p>More inline HTML
but this time on multiple lines
</p>

Regular text";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md033_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD033")
            .collect();

        // Should find 2 violations: <h1> and <p> opening tags
        assert_eq!(md033_violations.len(), 2);

        // Check that the violations contain the element names
        assert!(md033_violations[0].message().contains("h1"));
        assert!(md033_violations[1].message().contains("p"));
    }

    #[test]
    fn test_self_closing_tags() {
        let config = test_config_default();
        let input = "# Heading

<hr>

<hr/>

<br />

<img src=\"test.jpg\" alt=\"test\"/>";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md033_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD033")
            .collect();

        // Should find 4 violations: <hr>, <hr/>, <br />, <img/>
        assert_eq!(md033_violations.len(), 4);

        // Check element names
        assert!(md033_violations.iter().any(|v| v.message().contains("hr")));
        assert!(md033_violations.iter().any(|v| v.message().contains("br")));
        assert!(md033_violations.iter().any(|v| v.message().contains("img")));
    }

    #[test]
    fn test_allowed_elements() {
        let config = test_config_with_allowed_elements(vec!["h1", "p", "hr"]);
        let input = "# Regular heading

<h1>This is allowed</h1>

<h2>This is not allowed</h2>

<p>This is allowed</p>

<div>This is not allowed</div>

<hr>

<hr/>

<br/>";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md033_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD033")
            .collect();

        // Should find 3 violations: <h2>, <div>, <br/>
        assert_eq!(md033_violations.len(), 3);

        // Check that only non-allowed elements are reported
        assert!(md033_violations.iter().any(|v| v.message().contains("h2")));
        assert!(md033_violations.iter().any(|v| v.message().contains("div")));
        assert!(md033_violations.iter().any(|v| v.message().contains("br")));

        // Check that allowed elements are not reported
        assert!(!md033_violations.iter().any(|v| v.message().contains("h1")));
        assert!(!md033_violations.iter().any(|v| v.message().contains("p")));
        assert!(!md033_violations.iter().any(|v| v.message().contains("hr")));
    }

    #[test]
    fn test_case_insensitive_allowed_elements() {
        let config = test_config_with_allowed_elements(vec!["h1", "P"]);
        let input = "# Regular heading

<h1>Lower case tag, lower case config - allowed</h1>

<H1>Upper case tag, lower case config - allowed</H1>

<p>Lower case tag, upper case config - allowed</p>

<P>Upper case tag, upper case config - allowed</P>

<h2>Not allowed</h2>";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md033_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD033")
            .collect();

        // Should find only 1 violation: <h2>
        assert_eq!(md033_violations.len(), 1);
        assert!(md033_violations[0].message().contains("h2"));
    }

    #[test]
    fn test_nested_html_tags() {
        let config = test_config_with_allowed_elements(vec!["h1"]);
        let input = "<h1>This <h2>is not</h2> allowed</h1>";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md033_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD033")
            .collect();

        // Should find 1 violation: <h2> (h1 is allowed)
        assert_eq!(md033_violations.len(), 1);
        assert!(md033_violations[0].message().contains("h2"));
    }

    #[test]
    fn test_html_in_code_blocks_ignored() {
        let config = test_config_default();
        let input = "# Heading

```html
<h1>This should not trigger</h1>
<p>Neither should this</p>
```

    <h1>This shouldn't trigger as it's inside an indented code block</h1>

But <p>this should trigger</p>";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md033_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD033")
            .collect();

        // Should find only 1 violation: the <p> outside code blocks
        assert_eq!(md033_violations.len(), 1);
        assert!(md033_violations[0].message().contains("p"));
    }

    #[test]
    fn test_html_in_code_spans_ignored() {
        let config = test_config_default();
        let input = "# Heading

Text `<code>` text should not trigger.

Text `<p>some text</p>` should not trigger.

But <span>this should trigger</span>.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md033_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD033")
            .collect();

        // Should find only 1 violation: <span>
        assert_eq!(md033_violations.len(), 1);
        assert!(md033_violations[0].message().contains("span"));
    }

    #[test]
    fn test_only_opening_tags_reported() {
        let config = test_config_default();
        let input = "# Heading

<p>Opening and closing tags</p>

<div>
Content
</div>";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md033_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD033")
            .collect();

        // Should find only 2 violations: <p> and <div> opening tags, not the closing tags
        assert_eq!(md033_violations.len(), 2);
        assert!(md033_violations.iter().any(|v| v.message().contains("p")));
        assert!(md033_violations.iter().any(|v| v.message().contains("div")));
    }
}
