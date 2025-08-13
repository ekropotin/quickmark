use once_cell::sync::Lazy;
use regex::Regex;
use std::rc::Rc;
use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

static REVERSED_LINK_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(^|[^\\])\(([^()]+)\)\[([^\]^][^\]]*)\]").unwrap());

static INLINE_CODE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"`([^`]+)`").unwrap());

/// MD011 Reversed Link Syntax Rule Linter
///
/// **SINGLE-USE CONTRACT**: This linter is designed for one-time use only.
/// After processing a document (via feed() calls and finalize()), the linter
/// should be discarded. The violations state is not cleared between uses.
pub(crate) struct MD011Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
    line_offsets: Vec<usize>,
}

impl MD011Linter {
    pub fn new(context: Rc<Context>) -> Self {
        let line_offsets = context
            .lines
            .borrow()
            .iter()
            .scan(0, |state, line| {
                let offset = *state;
                // Assuming LF line endings. The +1 accounts for the newline character.
                *state += line.len() + 1;
                Some(offset)
            })
            .collect();

        Self {
            context,
            violations: Vec::new(),
            line_offsets,
        }
    }

    /// Analyze all lines and store all violations for reporting via finalize().
    /// Context cache is already initialized by MultiRuleLinter.
    fn analyze_all_lines(&mut self) {
        let lines = self.context.lines.borrow();
        let excluded_lines = self.get_excluded_lines();

        for (line_index, line) in lines.iter().enumerate() {
            let line_number = line_index + 1;

            if excluded_lines.contains(&line_number) {
                continue;
            }

            // Find all reversed link patterns in the line and create violations.
            for caps in REVERSED_LINK_REGEX.captures_iter(line) {
                let full_match = caps.get(0).unwrap();
                let pre_char = caps.get(1).unwrap().as_str();
                let link_text = caps.get(2).unwrap().as_str();
                let link_destination = caps.get(3).unwrap().as_str();

                // Skip if either link text or destination ends with backslash (escaped)
                if link_text.ends_with("\\") || link_destination.ends_with("\\") {
                    continue;
                }

                // Manual negative lookahead: skip if followed by opening parenthesis
                let match_end_byte = full_match.end();
                if line.as_bytes().get(match_end_byte) == Some(&b'(') {
                    continue;
                }

                // Calculate position accounting for pre_char
                let match_start_byte = full_match.start() + pre_char.len();
                let match_length_byte = full_match.len() - pre_char.len();

                // Check if this match overlaps with any inline code spans
                if self.overlaps_with_inline_code(line_index, match_start_byte, match_length_byte) {
                    continue;
                }

                let violation =
                    self.create_violation(line_index, match_start_byte, match_length_byte);
                self.violations.push(violation);
            }
        }
    }

    /// Returns a set of line numbers that should be excluded from checking.
    /// This includes code blocks.
    fn get_excluded_lines(&self) -> std::collections::HashSet<usize> {
        let node_cache = self.context.node_cache.borrow();

        ["indented_code_block", "fenced_code_block"]
            .iter()
            .filter_map(|block_type| node_cache.get(*block_type))
            .flatten()
            .flat_map(|node_info| (node_info.line_start + 1)..=(node_info.line_end + 1))
            .collect()
    }

    /// Check if a match overlaps with any inline code spans on the same line.
    fn overlaps_with_inline_code(
        &self,
        line_index: usize,
        match_start: usize,
        match_length: usize,
    ) -> bool {
        let lines = self.context.lines.borrow();
        if let Some(line) = lines.get(line_index) {
            let match_end = match_start + match_length;

            for code_match in INLINE_CODE_REGEX.find_iter(line) {
                let code_start = code_match.start();
                let code_end = code_match.end();

                if match_start < code_end && match_end > code_start {
                    return true;
                }
            }
        }

        false
    }

    /// Creates a RuleViolation for a reversed link at the specified position.
    fn create_violation(
        &self,
        line_index: usize,
        match_start: usize,
        match_length: usize,
    ) -> RuleViolation {
        let message = "Reversed link syntax".to_string();
        let line_start_byte = self.line_offsets[line_index];
        let start_byte = line_start_byte + match_start;
        let end_byte = line_start_byte + match_start + match_length;

        RuleViolation::new(
            &MD011,
            message,
            self.context.file_path.clone(),
            range_from_tree_sitter(&tree_sitter::Range {
                start_byte,
                end_byte,
                start_point: tree_sitter::Point {
                    row: line_index,
                    column: match_start,
                },
                end_point: tree_sitter::Point {
                    row: line_index,
                    column: match_start + match_length,
                },
            }),
        )
    }
}

impl RuleLinter for MD011Linter {
    fn feed(&mut self, node: &Node) {
        // This rule is line-based and only needs to run once.
        // We trigger the analysis on seeing the top-level `document` node.
        if node.kind() == "document" {
            self.analyze_all_lines();
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD011: Rule = Rule {
    id: "MD011",
    alias: "no-reversed-links",
    tags: &["links"],
    description: "Reversed link syntax",
    rule_type: RuleType::Line,
    required_nodes: &["indented_code_block", "fenced_code_block"],
    new_linter: |context| Box::new(MD011Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![
            ("no-reversed-links", RuleSeverity::Error),
            ("heading-style", RuleSeverity::Off),
            ("heading-increment", RuleSeverity::Off),
        ])
    }

    #[test]
    fn test_basic_reversed_link_violation() {
        let input = "This is a (reversed)[link] example.";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        let violation = &violations[0];
        assert_eq!("MD011", violation.rule().id);
        assert_eq!("Reversed link syntax", violation.message());
    }

    #[test]
    fn test_no_violations_correct_syntax() {
        let input = "This is a [correct](link) example.";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_multiple_reversed_links() {
        let input = "Here is (one)[link] and (another)[example].";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len());

        for violation in &violations {
            assert_eq!("MD011", violation.rule().id);
            assert_eq!("Reversed link syntax", violation.message());
        }
    }

    #[test]
    fn test_escaped_reversed_link_not_flagged() {
        let input = r"This is an escaped \(not)[a-link] example.";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_link_text_ending_with_backslash() {
        let input = r"(text\)[link] should not be flagged.";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_link_destination_ending_with_backslash() {
        let input = r"(text)[link\\] should not be flagged.";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_reversed_link_in_fenced_code_block_ignored() {
        let input = r###"```
This (reversed)[link] should be ignored in code block.
```"###;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_reversed_link_in_indented_code_block_ignored() {
        let input = "    This (reversed)[link] should be ignored in indented code block.";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_mixed_content_with_some_violations() {
        let input = r###"# Heading

This is a (reversed)[link] example.

```
This (code)[link] should be ignored.
```

And another [correct](link).

Another (bad)[example] here."###;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len()); // Only the two reversed links outside code blocks

        for violation in &violations {
            assert_eq!("MD011", violation.rule().id);
            assert_eq!("Reversed link syntax", violation.message());
        }
    }

    #[test]
    fn test_markdown_extra_footnote_style() {
        // Footnote references like [^1] should not be flagged
        let input = "For (example)[^1] this should not be flagged.";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_complex_urls() {
        let input = "Visit (GitHub)[https://github.com/user/repo#section] for more info.";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        let violation = &violations[0];
        assert_eq!("MD011", violation.rule().id);
        assert_eq!("Reversed link syntax", violation.message());
    }

    #[test]
    fn test_at_start_of_line() {
        let input = "(reversed)[link] at start of line.";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        let violation = &violations[0];
        assert_eq!("MD011", violation.rule().id);
        assert_eq!("Reversed link syntax", violation.message());
    }

    #[test]
    fn test_nested_parentheses_not_matched() {
        let input = "This (text (with parens))[link] should not match because of nested parens.";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Regex excludes nested parentheses
    }

    #[test]
    fn test_link_destination_starting_with_caret_or_bracket() {
        // Link destinations starting with ] or ^ should not match
        let input1 = "(text)[^footnote] should not match.";
        let input2 = "(text)[]bracket] should not match.";

        let config = test_config();

        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config.clone(), input1);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());

        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input2);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_followed_by_parenthesis_not_matched() {
        // Pattern followed by opening parenthesis should not match
        let input = "(text)[link](more) should not match.";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_reversed_link_in_inline_code_ignored() {
        let input = "This is `a (reversed)[link]` in inline code.";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_reversed_link_partially_in_inline_code_ignored() {
        let input = "This is `a (reversed`)[link] in inline code.";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }
}
