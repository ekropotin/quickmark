use std::rc::Rc;

use once_cell::sync::Lazy;
use regex::Regex;
use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, Context, RuleViolation},
    rules::{Rule, RuleLinter, RuleType},
};

// Regex patterns to find emphasis markers with spaces
static ASTERISK_EMPHASIS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\*{1,3})(\s*)([^*\n]*?)(\s*)(\*{1,3})").expect("Invalid asterisk emphasis regex")
});

static UNDERSCORE_EMPHASIS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\_{1,3})(\s*)([^_\n]*?)(\s*)(\_{1,3})")
        .expect("Invalid underscore emphasis regex")
});

// Regex to find code spans
static CODE_SPAN_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"`[^`\n]*`").expect("Invalid code span regex"));

pub(crate) struct MD037Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD037Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    fn is_in_code_context(&self, node: &Node) -> bool {
        // Check if this node is inside a code span or code block
        let mut current = Some(*node);
        while let Some(node_to_check) = current {
            match node_to_check.kind() {
                "code_span" | "fenced_code_block" | "indented_code_block" => {
                    return true;
                }
                _ => {
                    current = node_to_check.parent();
                }
            }
        }
        false
    }

    fn find_emphasis_violations_in_text(&mut self, node: &Node) {
        if self.is_in_code_context(node) {
            return;
        }

        let start_byte = node.start_byte();
        let text = {
            let source = self.context.get_document_content();
            source[start_byte..node.end_byte()].to_string()
        };

        // Find code span ranges to exclude
        let code_span_ranges: Vec<(usize, usize)> = CODE_SPAN_REGEX
            .find_iter(&text)
            .map(|m| (m.start(), m.end()))
            .collect();

        // Check for asterisk emphasis violations
        self.check_emphasis_pattern(
            &text,
            start_byte,
            &ASTERISK_EMPHASIS_REGEX,
            &code_span_ranges,
        );

        // Check for underscore emphasis violations
        self.check_emphasis_pattern(
            &text,
            start_byte,
            &UNDERSCORE_EMPHASIS_REGEX,
            &code_span_ranges,
        );
    }

    fn check_emphasis_pattern(
        &mut self,
        text: &str,
        text_start_byte: usize,
        regex: &Regex,
        code_span_ranges: &[(usize, usize)],
    ) {
        for capture in regex.captures_iter(text) {
            if let (
                Some(opening_marker),
                Some(opening_space),
                Some(_content),
                Some(closing_space),
                Some(closing_marker),
            ) = (
                capture.get(1),
                capture.get(2),
                capture.get(3),
                capture.get(4),
                capture.get(5),
            ) {
                // Check if this match overlaps with any code span
                let match_start = capture.get(0).unwrap().start();
                let match_end = capture.get(0).unwrap().end();

                let in_code_span = code_span_ranges.iter().any(|(code_start, code_end)| {
                    // Check if the match overlaps with a code span
                    match_start < *code_end && match_end > *code_start
                });

                if in_code_span {
                    continue; // Skip this match as it's inside a code span
                }

                let opening_text = opening_marker.as_str();
                let closing_text = closing_marker.as_str();

                // Only process if markers match (same type and count)
                if opening_text == closing_text {
                    // Check for space after opening marker
                    if !opening_space.as_str().is_empty() {
                        self.create_opening_space_violation(
                            opening_marker,
                            opening_space,
                            text_start_byte,
                        );
                    }

                    // Check for space before closing marker
                    if !closing_space.as_str().is_empty() {
                        self.create_closing_space_violation(
                            closing_marker,
                            closing_space,
                            text_start_byte,
                        );
                    }
                }
            }
        }
    }

    fn create_opening_space_violation(
        &mut self,
        opening_marker: regex::Match,
        opening_space: regex::Match,
        text_start_byte: usize,
    ) {
        let marker = opening_marker.as_str();
        let space = opening_space.as_str();
        let violation_start = text_start_byte + opening_marker.end();
        let violation_end = text_start_byte + opening_space.end();

        let range = tree_sitter::Range {
            start_byte: violation_start,
            end_byte: violation_end,
            start_point: self.byte_to_point(violation_start),
            end_point: self.byte_to_point(violation_end),
        };

        self.violations.push(RuleViolation::new(
            &MD037,
            format!("{} [Context: \"{}{}\"]", MD037.description, marker, space),
            self.context.file_path.clone(),
            range_from_tree_sitter(&range),
        ));
    }

    fn create_closing_space_violation(
        &mut self,
        closing_marker: regex::Match,
        closing_space: regex::Match,
        text_start_byte: usize,
    ) {
        let marker = closing_marker.as_str();
        let space = closing_space.as_str();
        let violation_start = text_start_byte + closing_space.start();
        let violation_end = text_start_byte + closing_marker.end();

        let range = tree_sitter::Range {
            start_byte: violation_start,
            end_byte: violation_end,
            start_point: self.byte_to_point(violation_start),
            end_point: self.byte_to_point(violation_end),
        };

        self.violations.push(RuleViolation::new(
            &MD037,
            format!("{} [Context: \"{}{}\"]", MD037.description, space, marker),
            self.context.file_path.clone(),
            range_from_tree_sitter(&range),
        ));
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

impl RuleLinter for MD037Linter {
    fn feed(&mut self, node: &Node) {
        match node.kind() {
            // Look for text content that might contain emphasis markers with spaces
            "text" | "inline" => {
                self.find_emphasis_violations_in_text(node);
            }
            _ => {}
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD037: Rule = Rule {
    id: "MD037",
    alias: "no-space-in-emphasis",
    tags: &["whitespace", "emphasis"],
    description: "Spaces inside emphasis markers",
    rule_type: RuleType::Token,
    required_nodes: &["emphasis", "strong_emphasis"],
    new_linter: |context| Box::new(MD037Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![("no-space-in-emphasis", RuleSeverity::Error)])
    }

    #[test]
    fn test_no_violations_valid_emphasis() {
        let config = test_config();
        let input = "This has *valid emphasis* and **valid strong** text.
Also _valid emphasis_ and __valid strong__ text.
And ***valid strong emphasis*** and ___valid strong emphasis___ text.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md037_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD037")
            .collect();
        assert_eq!(md037_violations.len(), 0);
    }

    #[test]
    fn test_violations_spaces_inside_single_asterisk() {
        let config = test_config();
        let input = "This has * invalid emphasis * with spaces inside.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md037_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD037")
            .collect();

        // Should find 2 violations: one for opening space, one for closing space
        assert_eq!(md037_violations.len(), 2);
    }

    #[test]
    fn test_violations_spaces_inside_double_asterisk() {
        let config = test_config();
        let input = "This has ** invalid strong ** with spaces inside.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md037_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD037")
            .collect();

        // Should find 2 violations: one for opening space, one for closing space
        assert_eq!(md037_violations.len(), 2);
    }

    #[test]
    fn test_violations_spaces_inside_triple_asterisk() {
        let config = test_config();
        let input = "This has *** invalid strong emphasis *** with spaces inside.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md037_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD037")
            .collect();

        // Should find 2 violations: one for opening space, one for closing space
        assert_eq!(md037_violations.len(), 2);
    }

    #[test]
    fn test_violations_spaces_inside_single_underscore() {
        let config = test_config();
        let input = "This has _ invalid emphasis _ with spaces inside.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md037_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD037")
            .collect();

        // Should find 2 violations: one for opening space, one for closing space
        assert_eq!(md037_violations.len(), 2);
    }

    #[test]
    fn test_violations_spaces_inside_double_underscore() {
        let config = test_config();
        let input = "This has __ invalid strong __ with spaces inside.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md037_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD037")
            .collect();

        // Should find 2 violations: one for opening space, one for closing space
        assert_eq!(md037_violations.len(), 2);
    }

    #[test]
    fn test_violations_spaces_inside_triple_underscore() {
        let config = test_config();
        let input = "This has ___ invalid strong emphasis ___ with spaces inside.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md037_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD037")
            .collect();

        // Should find 2 violations: one for opening space, one for closing space
        assert_eq!(md037_violations.len(), 2);
    }

    #[test]
    fn test_violations_mixed_valid_and_invalid() {
        let config = test_config();
        let input = "Mix of *valid* and * invalid * emphasis.
Also **valid** and ** invalid ** strong.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md037_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD037")
            .collect();

        // Should find 4 violations: 2 from each invalid emphasis (opening and closing spaces)
        assert_eq!(md037_violations.len(), 4);
    }

    #[test]
    fn test_violations_one_sided_spaces() {
        let config = test_config();
        let input = "One sided *invalid * and * invalid* emphasis.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md037_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD037")
            .collect();

        // Should find 2 violations: one for each one-sided space
        assert_eq!(md037_violations.len(), 2);
    }

    #[test]
    fn test_no_violations_in_code_blocks() {
        let config = test_config();
        let input = "Regular text with *valid* emphasis.

```markdown
This should not trigger * invalid * emphasis in code blocks.
```

More text with _valid_ emphasis.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md037_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD037")
            .collect();
        assert_eq!(md037_violations.len(), 0);
    }

    #[test]
    fn test_no_violations_in_code_spans() {
        let config = test_config();
        let input = "Regular text with `* invalid * code spans` should not trigger violations.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md037_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD037")
            .collect();
        assert_eq!(md037_violations.len(), 0);
    }
}
