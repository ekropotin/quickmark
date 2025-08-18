use regex::Regex;
use std::collections::HashSet;
use std::rc::Rc;

use crate::{
    linter::{range_from_tree_sitter, Context, RuleLinter, RuleViolation},
    rules::{Rule, RuleType},
};

pub(crate) struct MD044Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
    name_regexes: Vec<(String, Regex)>, // (original_name, compiled_regex)
    all_names: HashSet<String>,         // Added for performance
}

impl MD044Linter {
    pub fn new(context: Rc<Context>) -> Self {
        let config = &context.config.linters.settings.proper_names;
        let mut name_regexes = Vec::new();

        // Use a HashSet for efficient lookups of correct names
        let all_names: HashSet<String> = config.names.iter().cloned().collect();

        // Sort names by length (longest first) to handle overlapping matches
        let mut names = config.names.clone();
        names.sort_by(|a, b| b.len().cmp(&a.len()).then_with(|| a.cmp(b)));

        for name in names {
            if !name.is_empty() {
                // The original name is the "expected" name in case of a violation
                if let Ok(regex) = create_name_regex(&name) {
                    name_regexes.push((name, regex));
                }
            }
        }

        Self {
            context,
            violations: Vec::new(),
            name_regexes,
            all_names,
        }
    }

    fn should_check_node(&self, node_kind: &str) -> bool {
        let config = &self.context.config.linters.settings.proper_names;

        match node_kind {
            // Code blocks and inline code
            "fenced_code_block" | "indented_code_block" | "code_span" => config.code_blocks,
            // HTML elements
            "html_block" | "html_inline" => config.html_elements,
            // Regular text content
            "text" | "paragraph" => true,
            _ => false,
        }
    }

    // This function is now immutable with respect to self and returns violations.
    // This improves performance by allowing borrows of self.context in the caller (`feed`).
    fn check_text_content(
        &self,
        text: &str,
        start_line: usize,
        start_column: usize,
    ) -> Vec<RuleViolation> {
        if self.name_regexes.is_empty() {
            return Vec::new();
        }

        let mut violations = Vec::new();
        let mut exclusion_ranges: Vec<(usize, usize)> = Vec::new(); // (start, end) byte ranges

        for (expected_name, regex) in &self.name_regexes {
            for match_result in regex.find_iter(text) {
                let matched_text = match_result.as_str();
                let match_start = match_result.start();
                let match_end = match_result.end();

                // Check if this range overlaps with any exclusion range
                let overlaps = exclusion_ranges
                    .iter()
                    .any(|(start, end)| !(match_end <= *start || match_start >= *end));

                if overlaps {
                    continue;
                }

                // Performance: Use HashSet for O(1) average lookup and avoid String allocation.
                if self.all_names.contains(matched_text) {
                    // Add to exclusions even if it's valid to prevent overlaps with shorter, incorrect names
                    exclusion_ranges.push((match_start, match_end));
                    continue;
                }

                // Create violation range
                let range = tree_sitter::Range {
                    start_byte: match_start,
                    end_byte: match_end,
                    start_point: tree_sitter::Point {
                        row: start_line,
                        column: start_column + match_start,
                    },
                    end_point: tree_sitter::Point {
                        row: start_line,
                        column: start_column + match_end,
                    },
                };

                violations.push(RuleViolation::new(
                    &MD044,
                    format!("Expected: {expected_name}; Actual: {matched_text}"),
                    self.context.file_path.clone(),
                    range_from_tree_sitter(&range),
                ));

                // Add violation range to exclusions to prevent multiple reports on the same text
                exclusion_ranges.push((match_start, match_end));
            }
        }
        violations
    }
}

impl RuleLinter for MD044Linter {
    fn feed(&mut self, node: &tree_sitter::Node) {
        if !self.should_check_node(node.kind()) {
            return;
        }

        let source = self.context.get_document_content();
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();

        if end_byte <= source.len() {
            // Performance: Avoid allocating a new String for each node.
            // Pass a string slice directly. This is possible because check_text_content
            // no longer needs a mutable borrow of `self`, resolving the borrow checker conflict.
            let text_slice = &source[start_byte..end_byte];
            let start_line = node.start_position().row;
            let start_column = node.start_position().column;

            let new_violations = self.check_text_content(text_slice, start_line, start_column);
            self.violations.extend(new_violations);
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

// Helper function to create a case-insensitive regex for a proper name.
fn create_name_regex(name: &str) -> Result<Regex, regex::Error> {
    let escaped_name = regex::escape(name);

    // Word boundaries for the pattern, following original markdownlint logic.
    // This ensures we match whole words.
    let starts_with_word_char = name.chars().next().is_some_and(is_word_char);
    let ends_with_word_char = name.chars().last().is_some_and(is_word_char);

    let start_boundary = if starts_with_word_char { "\\b_*" } else { "" };
    let end_boundary = if ends_with_word_char { "_*\\b" } else { "" };

    // Performance: Use non-capturing groups (?:...) as we only need the full match.
    let pattern = format!("(?i){start_boundary}{escaped_name}{end_boundary}");
    Regex::new(&pattern)
}

// Helper function to check if a character is a word character (equivalent to \w in regex)
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

pub const MD044: Rule = Rule {
    id: "MD044",
    alias: "proper-names",
    tags: &["spelling"],
    description: "Proper names should have the correct capitalization",
    rule_type: RuleType::Token, // Changed from Special to Token as it processes specific node types
    required_nodes: &[
        "text",
        "paragraph",
        "fenced_code_block",
        "indented_code_block",
        "code_span",
        "html_block",
        "html_inline",
    ],
    new_linter: |context| Box::new(MD044Linter::new(context)),
};

#[cfg(test)]
mod test {
    use crate::config::{LintersSettingsTable, MD044ProperNamesTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_settings;
    use std::path::PathBuf;

    fn test_config(
        names: Vec<String>,
        code_blocks: bool,
        html_elements: bool,
    ) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![("proper-names", RuleSeverity::Error)],
            LintersSettingsTable {
                proper_names: MD044ProperNamesTable {
                    names,
                    code_blocks,
                    html_elements,
                },
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_no_names_configured() {
        let config = test_config(vec![], true, true);
        let input = "This contains javascript and GitHub text.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_exact_match_no_violations() {
        let config = test_config(
            vec!["JavaScript".to_string(), "GitHub".to_string()],
            true,
            true,
        );
        let input = "This text contains JavaScript and GitHub properly capitalized.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_incorrect_capitalization() {
        let config = test_config(vec!["JavaScript".to_string()], true, true);
        let input = "This text contains javascript with incorrect capitalization.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Expected: JavaScript"));
        assert!(violations[0].message().contains("Actual: javascript"));
    }

    #[test]
    fn test_multiple_violations() {
        let config = test_config(
            vec!["JavaScript".to_string(), "GitHub".to_string()],
            true,
            true,
        );
        let input = "We use javascript and github for development.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_code_blocks_enabled() {
        let config = test_config(vec!["JavaScript".to_string()], true, true);
        let input = "```\nlet x = javascript;\n```";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_code_blocks_disabled() {
        let config = test_config(vec!["JavaScript".to_string()], false, true);
        let input = "```\nlet x = javascript;\n```";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_html_elements_enabled() {
        let config = test_config(vec!["JavaScript".to_string()], true, true);
        let input = "<p>We use javascript here</p>";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_html_elements_disabled() {
        let config = test_config(vec!["JavaScript".to_string()], true, false);
        let input = "<p>We use javascript here</p>";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_word_boundaries() {
        let config = test_config(vec!["JavaScript".to_string()], true, true);
        let input = "The javascriptish language is not javascript.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1); // Should only match whole word "javascript", not "javascriptish"
    }

    #[test]
    fn test_sorting_by_length() {
        // Test that longer names match first to avoid partial matches
        let config = test_config(vec!["GitHub".to_string(), "git".to_string()], true, true);
        let input = "We use github for version control.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Expected: GitHub"));
    }

    #[test]
    fn test_mixed_case_names() {
        let config = test_config(
            vec!["GitHub".to_string(), "github.com".to_string()],
            true,
            true,
        );
        let input = "Visit github.com or use GITHUB for repos.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1); // "github.com" is correct, "GITHUB" should be "GitHub"
        assert!(violations[0].message().contains("Expected: GitHub"));
        assert!(violations[0].message().contains("Actual: GITHUB"));
    }
}
