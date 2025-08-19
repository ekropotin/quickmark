use serde::Deserialize;
use std::rc::Rc;

use once_cell::sync::Lazy;
use regex::Regex;
use tree_sitter::Node;

use crate::linter::{range_from_tree_sitter, Context, RuleLinter, RuleViolation};

use super::{Rule, RuleType};

// MD026-specific configuration types
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD026TrailingPunctuationTable {
    #[serde(default)]
    pub punctuation: String,
}

impl Default for MD026TrailingPunctuationTable {
    fn default() -> Self {
        Self {
            punctuation: ".,;:!。，；：！".to_string(),
        }
    }
}

impl MD026TrailingPunctuationTable {
    pub fn with_default_punctuation() -> Self {
        Self {
            punctuation: ".,;:!。，；：！".to_string(), // Default without '?' chars
        }
    }
}

pub(crate) struct MD026Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD026Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    fn extract_heading_text<'a>(&self, node: &Node, source: &'a str) -> &'a str {
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let full_text = &source[start_byte..end_byte];

        match node.kind() {
            "atx_heading" => full_text
                .trim_start_matches('#')
                .trim()
                .trim_end_matches('#')
                .trim(),
            "setext_heading" => {
                if let Some(line) = full_text.lines().next() {
                    line.trim()
                } else {
                    ""
                }
            }
            _ => "",
        }
    }

    fn check_trailing_punctuation(&mut self, node: &Node) {
        let source = self.context.get_document_content();
        let heading_text = self.extract_heading_text(node, &source);
        if heading_text.is_empty() {
            return;
        }

        let config = &self.context.config.linters.settings.trailing_punctuation;

        // Handle configuration: if punctuation is empty, the rule is effectively disabled
        let punctuation_chars = if config.punctuation.is_empty() {
            return; // Empty punctuation = rule disabled, allow all
        } else {
            &config.punctuation
        };

        // Check if the heading ends with any of the specified punctuation characters
        if let Some(trailing_char) = heading_text.chars().last() {
            if punctuation_chars.contains(trailing_char) {
                // Check if this is an HTML entity (ends with ;)
                if trailing_char == ';' && is_html_entity(heading_text) {
                    return; // Skip HTML entities
                }

                // Check if this is a gemoji code (ends with :)
                if trailing_char == ':' && is_gemoji_code(heading_text) {
                    return; // Skip gemoji codes
                }

                // Create a violation
                let range = tree_sitter::Range {
                    start_byte: 0, // Not used by range_from_tree_sitter
                    end_byte: 0,   // Not used by range_from_tree_sitter
                    start_point: tree_sitter::Point {
                        row: node.start_position().row,
                        column: 0,
                    },
                    end_point: tree_sitter::Point {
                        row: node.end_position().row,
                        column: node.end_position().column,
                    },
                };

                self.violations.push(RuleViolation::new(
                    &MD026,
                    format!("Punctuation: '{trailing_char}'"),
                    self.context.file_path.clone(),
                    range_from_tree_sitter(&range),
                ));
            }
        }
    }
}

impl RuleLinter for MD026Linter {
    fn feed(&mut self, node: &Node) {
        match node.kind() {
            "atx_heading" | "setext_heading" => self.check_trailing_punctuation(node),
            _ => {
                // Ignore other nodes
            }
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

// Helper function to detect HTML entities
fn is_html_entity(text: &str) -> bool {
    static HTML_ENTITY_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"&(?:[a-zA-Z\d]+|#\d+|#x[0-9a-fA-F]+);$").unwrap());
    HTML_ENTITY_RE.is_match(text.trim())
}

// Helper function to detect GitHub emoji codes (gemoji)
fn is_gemoji_code(text: &str) -> bool {
    static GEMOJI_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r":(?:[abmovx]|[-+]1|100|1234|(?:1st|2nd|3rd)_place_medal|8ball|clock\d{1,4}|e-mail|non-potable_water|o2|t-rex|u5272|u5408|u55b6|u6307|u6708|u6709|u6e80|u7121|u7533|u7981|u7a7a|[a-z]{2,15}2?|[a-z]{1,14}(?:_[a-z\d]{1,16})+):$").unwrap()
    });
    GEMOJI_RE.is_match(text.trim())
}

pub const MD026: Rule = Rule {
    id: "MD026",
    alias: "no-trailing-punctuation",
    tags: &["headings"],
    description: "Trailing punctuation in heading",
    rule_type: RuleType::Token,
    required_nodes: &["atx_heading", "setext_heading"],
    new_linter: |context| Box::new(MD026Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{LintersSettingsTable, MD026TrailingPunctuationTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_settings;

    fn test_config(punctuation: &str) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![("no-trailing-punctuation", RuleSeverity::Error)],
            LintersSettingsTable {
                trailing_punctuation: MD026TrailingPunctuationTable {
                    punctuation: punctuation.to_string(),
                },
                ..Default::default()
            },
        )
    }

    fn test_default_config() -> crate::config::QuickmarkConfig {
        test_config(".,;:!。，；：！")
    }

    #[test]
    fn test_atx_heading_with_period() {
        let config = test_default_config();
        let input = "# This is a heading.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Punctuation: '.'"));
    }

    #[test]
    fn test_atx_heading_with_exclamation() {
        let config = test_default_config();
        let input = "# This is a heading!";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Punctuation: '!'"));
    }

    #[test]
    fn test_atx_heading_with_comma() {
        let config = test_default_config();
        let input = "## This is a heading,";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Punctuation: ','"));
    }

    #[test]
    fn test_atx_heading_with_semicolon() {
        let config = test_default_config();
        let input = "### This is a heading;";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Punctuation: ';'"));
    }

    #[test]
    fn test_atx_heading_with_colon() {
        let config = test_default_config();
        let input = "#### This is a heading:";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Punctuation: ':'"));
    }

    #[test]
    fn test_atx_heading_with_question_mark_allowed() {
        let config = test_default_config();
        let input = "# This is a heading?";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0); // '?' is not in default punctuation
    }

    #[test]
    fn test_atx_heading_without_punctuation() {
        let config = test_default_config();
        let input = "# This is a heading";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_setext_heading_with_period() {
        let config = test_default_config();
        let input = "# Document\n\nThis is a heading.\n==================\n\nContent here";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Punctuation: '.'"));
    }

    #[test]
    fn test_setext_heading_with_exclamation() {
        let config = test_default_config();
        let input = "# Document\n\nThis is a heading!\n------------------\n\nContent here";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Punctuation: '!'"));
    }

    #[test]
    fn test_setext_heading_without_punctuation() {
        let config = test_default_config();
        let input = "# Document\n\nThis is a heading\n=================\n\nContent here";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_full_width_punctuation() {
        let config = test_default_config();
        let input = "# Heading with full-width period。";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Punctuation: '。'"));
    }

    #[test]
    fn test_full_width_comma() {
        let config = test_default_config();
        let input = "# Heading with full-width comma，";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Punctuation: '，'"));
    }

    #[test]
    fn test_custom_punctuation() {
        let config = test_config(".,;:");
        let input = "# This heading has exclamation!";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0); // '!' not in custom punctuation
    }

    #[test]
    fn test_custom_punctuation_with_violation() {
        let config = test_config(".,;:");
        let input = "# This heading has period.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Punctuation: '.'"));
    }

    #[test]
    fn test_empty_punctuation_allows_all() {
        let config = test_config("");
        let input =
            "# This heading has period.\n## This heading has exclamation!\n### This has comma,";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0); // Empty punctuation = allow all
    }

    #[test]
    fn test_html_entity_ignored() {
        let config = test_default_config();
        let input = "# Copyright &copy;\n## Registered &reg;";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0); // HTML entities should be ignored
    }

    #[test]
    fn test_numeric_html_entity_ignored() {
        let config = test_default_config();
        let input = "# Copyright &#169;\n## Registered &#174;";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0); // Numeric HTML entities should be ignored
    }

    #[test]
    fn test_hex_html_entity_ignored() {
        let config = test_default_config();
        let input = "# Copyright &#x000A9;\n## Registered &#xAE;";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0); // Hex HTML entities should be ignored
    }

    #[test]
    fn test_mixed_valid_and_invalid() {
        let config = test_default_config();
        let input =
            "# Good heading\n## Bad heading.\n### Another good heading\n#### Another bad heading!";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 2);
        assert!(violations[0].message().contains("Punctuation: '.'"));
        assert!(violations[1].message().contains("Punctuation: '!'"));
    }

    #[test]
    fn test_atx_closed_style_heading() {
        let config = test_default_config();
        let input = "# This is a heading. #";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Punctuation: '.'"));
    }

    #[test]
    fn test_multiple_trailing_punctuation() {
        let config = test_default_config();
        let input = "# This is a heading...";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Punctuation: '.'"));
    }

    #[test]
    fn test_empty_heading() {
        let config = test_default_config();
        let input = "#\n==";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0); // Empty headings should not trigger violations
    }
}
