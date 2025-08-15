use std::rc::Rc;

use tree_sitter::Node;

use crate::linter::{range_from_tree_sitter, Context, RuleLinter, RuleViolation};

use super::{Rule, RuleType};

const VIOLATION_MESSAGE: &str = "Spaces inside code span elements";

pub(crate) struct MD038Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD038Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    fn check_inline_content(&mut self, node: &Node) {
        let text = {
            let content = self.context.get_document_content();
            node.utf8_text(content.as_bytes()).unwrap_or("").to_string()
        };
        let node_start_byte = node.start_byte();

        // Find all code spans using a proper parser
        let code_spans = self.find_code_spans(&text);
        for (content, start, len) in code_spans {
            self.check_code_span_content(&content, node_start_byte + start, len);
        }
    }

    fn find_code_spans(&self, text: &str) -> Vec<(String, usize, usize)> {
        let mut spans = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = text.chars().collect();

        while i < chars.len() {
            if chars[i] == '`' {
                // Count opening backticks
                let start_pos = i;
                let mut backtick_count = 0;
                while i < chars.len() && chars[i] == '`' {
                    backtick_count += 1;
                    i += 1;
                }

                // Look for closing backticks of the same count
                let content_start = i;
                let mut found_closing = false;

                while i < chars.len() {
                    if chars[i] == '`' {
                        let closing_start = i;
                        let mut closing_count = 0;
                        while i < chars.len() && chars[i] == '`' {
                            closing_count += 1;
                            i += 1;
                        }

                        if closing_count == backtick_count {
                            // Found matching closing backticks
                            let content_end = closing_start;
                            let content: String =
                                chars[content_start..content_end].iter().collect();
                            let content_byte_start = text
                                .char_indices()
                                .nth(content_start)
                                .map(|(i, _)| i)
                                .unwrap_or(0);
                            let content_len = content.len();
                            spans.push((content, content_byte_start, content_len));
                            found_closing = true;
                            break;
                        }
                        // Continue looking if backtick count doesn't match
                    } else {
                        i += 1;
                    }
                }

                // If we didn't find a closing sequence, backtrack and continue
                if !found_closing {
                    i = start_pos + 1;
                }
            } else {
                i += 1;
            }
        }

        spans
    }

    fn check_code_span_content(
        &mut self,
        code_content: &str,
        content_start_byte: usize,
        content_len: usize,
    ) {
        // If the content is only whitespace, allow it (per recent clarification)
        if code_content.trim().is_empty() {
            return;
        }

        // Check for leading whitespace violations
        let leading_whitespace: String = code_content
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect();
        let leading_is_violation = match leading_whitespace.as_str() {
            "" => false,  // No leading whitespace - OK
            " " => false, // Single space - OK per CommonMark spec
            _ => true,    // Multiple spaces, tabs, or other whitespace - violation
        };

        if leading_is_violation {
            let leading_byte_len = leading_whitespace.len();
            let violation_range = tree_sitter::Range {
                start_byte: content_start_byte,
                end_byte: content_start_byte + leading_byte_len,
                start_point: self.byte_to_point(content_start_byte),
                end_point: self.byte_to_point(content_start_byte + leading_byte_len),
            };

            self.violations.push(RuleViolation::new(
                &MD038,
                format!("{} [Context: leading whitespace]", VIOLATION_MESSAGE),
                self.context.file_path.clone(),
                range_from_tree_sitter(&violation_range),
            ));
        }

        // Check for trailing whitespace violations
        let trailing_whitespace: String = code_content
            .chars()
            .rev()
            .take_while(|c| c.is_whitespace())
            .collect::<String>()
            .chars()
            .rev()
            .collect();
        let trailing_is_violation = match trailing_whitespace.as_str() {
            "" => false,  // No trailing whitespace - OK
            " " => false, // Single space - OK per CommonMark spec
            _ => true,    // Multiple spaces, tabs, or other whitespace - violation
        };

        if trailing_is_violation {
            let trailing_byte_len = trailing_whitespace.len();
            let violation_end_byte = content_start_byte + content_len;
            let violation_start_byte = violation_end_byte - trailing_byte_len;

            let violation_range = tree_sitter::Range {
                start_byte: violation_start_byte,
                end_byte: violation_end_byte,
                start_point: self.byte_to_point(violation_start_byte),
                end_point: self.byte_to_point(violation_end_byte),
            };

            self.violations.push(RuleViolation::new(
                &MD038,
                format!("{} [Context: trailing whitespace]", VIOLATION_MESSAGE),
                self.context.file_path.clone(),
                range_from_tree_sitter(&violation_range),
            ));
        }
    }

    fn byte_to_point(&self, byte_pos: usize) -> tree_sitter::Point {
        let source = self.context.get_document_content();
        let mut line = 0;
        let mut column = 0;

        for (i, ch) in source.char_indices() {
            if i >= byte_pos {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        }

        tree_sitter::Point { row: line, column }
    }
}

impl RuleLinter for MD038Linter {
    fn feed(&mut self, node: &Node) {
        match node.kind() {
            "inline" => {
                self.check_inline_content(node);
            }
            _ => {}
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD038: Rule = Rule {
    id: "MD038",
    alias: "no-space-in-code",
    tags: &["whitespace", "code"],
    description: "Spaces inside code span elements",
    rule_type: RuleType::Token,
    required_nodes: &["inline"],
    new_linter: |context| Box::new(MD038Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![("no-space-in-code", RuleSeverity::Error)])
    }

    #[test]
    fn test_no_violations_valid_code_spans() {
        let config = test_config();
        let input = "This has `valid code` spans.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md038_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD038")
            .collect();
        assert_eq!(md038_violations.len(), 0);
    }

    #[test]
    fn test_no_violations_single_space_padding() {
        // Single leading and trailing space is allowed by CommonMark spec
        let config = test_config();
        let input = "This has ` code ` spans with single space padding.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md038_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD038")
            .collect();
        assert_eq!(md038_violations.len(), 0);
    }

    #[test]
    fn test_no_violations_code_spans_only_spaces() {
        // Code spans containing only spaces should be allowed
        let config = test_config();
        let input = "This has `   ` code spans with only spaces.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md038_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD038")
            .collect();
        assert_eq!(md038_violations.len(), 0);
    }

    #[test]
    fn test_violations_multiple_leading_spaces() {
        let config = test_config();
        let input = "This has `  code` with multiple leading spaces.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md038_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD038")
            .collect();
        assert_eq!(md038_violations.len(), 1);
    }

    #[test]
    fn test_violations_multiple_trailing_spaces() {
        let config = test_config();
        let input = "This has `code  ` with multiple trailing spaces.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md038_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD038")
            .collect();
        assert_eq!(md038_violations.len(), 1);
    }

    #[test]
    fn test_violations_multiple_leading_and_trailing_spaces() {
        let config = test_config();
        let input = "This has `  code  ` with multiple leading and trailing spaces.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md038_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD038")
            .collect();
        assert_eq!(md038_violations.len(), 2);
    }

    #[test]
    fn test_violations_tabs_instead_of_spaces() {
        let config = test_config();
        let input = "This has `\tcode\t` with tabs.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md038_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD038")
            .collect();
        assert_eq!(md038_violations.len(), 2);
    }

    #[test]
    fn test_violations_mixed_whitespace() {
        let config = test_config();
        let input = "This has ` \tcode \t` with mixed whitespace.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md038_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD038")
            .collect();
        assert_eq!(md038_violations.len(), 2);
    }

    #[test]
    fn test_violations_only_leading_spaces() {
        let config = test_config();
        let input = "This has `  code` with only leading spaces.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md038_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD038")
            .collect();
        assert_eq!(md038_violations.len(), 1);
    }

    #[test]
    fn test_violations_only_trailing_spaces() {
        let config = test_config();
        let input = "This has `code  ` with only trailing spaces.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md038_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD038")
            .collect();
        assert_eq!(md038_violations.len(), 1);
    }

    #[test]
    fn test_no_violations_double_backtick_code_spans() {
        let config = test_config();
        let input = "This has ``valid code`` with double backticks.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md038_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD038")
            .collect();
        assert_eq!(md038_violations.len(), 0);
    }

    #[test]
    fn test_violations_double_backtick_with_spaces() {
        let config = test_config();
        let input = "This has ``  code  `` with double backticks and spaces.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md038_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD038")
            .collect();
        assert_eq!(md038_violations.len(), 2);
    }

    #[test]
    fn test_multiple_code_spans_on_same_line() {
        let config = test_config();
        let input = "This has `valid` and `  invalid  ` code spans.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md038_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD038")
            .collect();
        assert_eq!(md038_violations.len(), 2);
    }

    #[test]
    fn test_code_spans_in_different_contexts() {
        let config = test_config();
        let input = "# Heading with `  invalid  ` code span

Paragraph with `valid` and `  invalid  ` spans.

- List item with `  invalid  ` code span
- Another item with `valid` span

> Blockquote with `  invalid  ` code span";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md038_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD038")
            .collect();
        assert_eq!(md038_violations.len(), 8); // 2 violations per invalid span (leading + trailing)
    }

    #[test]
    fn test_no_violations_empty_code_span() {
        let config = test_config();
        let input = "This has `` empty code spans.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md038_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD038")
            .collect();
        assert_eq!(md038_violations.len(), 0);
    }

    #[test]
    fn test_code_span_with_backtick_content() {
        // Test code span that contains backticks - should use double backticks
        let config = test_config();
        let input = "This shows `` ` `` a backtick character.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md038_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD038")
            .collect();
        // Single space padding is allowed in this case
        assert_eq!(md038_violations.len(), 0);
    }

    #[test]
    fn test_code_span_with_backtick_content_extra_spaces() {
        // Test code span that contains backticks with extra spaces
        let config = test_config();
        let input = "This shows ``  `  `` a backtick with extra spaces.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md038_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD038")
            .collect();
        assert_eq!(md038_violations.len(), 2);
    }
}
