use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, Context, RuleLinter, RuleViolation},
    rules::{Rule, RuleType},
};

#[derive(Debug, Clone)]
struct HeadingInfo {
    content: String,
    level: u8,
    range: tree_sitter::Range,
}

pub(crate) struct MD043Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
    headings: Vec<HeadingInfo>,
}

impl MD043Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
            headings: Vec::new(),
        }
    }

    fn extract_heading_content(&self, node: &Node) -> String {
        let source = self.context.get_document_content();
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let full_text = &source[start_byte..end_byte];

        match node.kind() {
            "atx_heading" => {
                // Remove leading #s and trailing #s if present
                let text = full_text
                    .trim_start_matches('#')
                    .trim()
                    .trim_end_matches('#')
                    .trim();
                text.to_string()
            }
            "setext_heading" => {
                // For setext, take first line (before underline)
                if let Some(line) = full_text.lines().next() {
                    line.trim().to_string()
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }

    fn extract_heading_level(&self, node: &Node) -> u8 {
        match node.kind() {
            "atx_heading" => {
                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();
                    if child.kind().starts_with("atx_h") && child.kind().ends_with("_marker") {
                        return child.kind().chars().nth(5).unwrap().to_digit(10).unwrap() as u8;
                    }
                }
                1 // fallback
            }
            "setext_heading" => {
                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();
                    if child.kind() == "setext_h1_underline" {
                        return 1;
                    } else if child.kind() == "setext_h2_underline" {
                        return 2;
                    }
                }
                1 // fallback
            }
            _ => 1,
        }
    }

    fn format_heading(&self, content: &str, level: u8) -> String {
        format!("{} {}", "#".repeat(level as usize), content)
    }

    fn compare_headings(&self, expected: &str, actual: &str) -> bool {
        let config = &self.context.config.linters.settings.required_headings;
        if config.match_case {
            expected == actual
        } else {
            expected.to_lowercase() == actual.to_lowercase()
        }
    }

    fn check_required_headings(&mut self) {
        let config = &self.context.config.linters.settings.required_headings;

        if config.headings.is_empty() {
            return; // Nothing to check
        }

        let mut required_index = 0;
        let mut match_any = false;
        let mut has_error = false;
        let any_headings = !self.headings.is_empty();

        for heading in &self.headings {
            if has_error {
                break;
            }

            let actual = self.format_heading(&heading.content, heading.level);

            if required_index >= config.headings.len() {
                // No more required headings, but we have more actual headings
                break;
            }

            let expected = &config.headings[required_index];

            match expected.as_str() {
                "*" => {
                    // Zero or more unspecified headings
                    if required_index + 1 < config.headings.len() {
                        let next_expected = &config.headings[required_index + 1];
                        if self.compare_headings(next_expected, &actual) {
                            required_index += 2; // Skip "*" and match the next
                            match_any = false;
                        } else {
                            match_any = true;
                        }
                    } else {
                        match_any = true;
                    }
                }
                "+" => {
                    // One or more unspecified headings
                    match_any = true;
                    required_index += 1;
                }
                "?" => {
                    // Exactly one unspecified heading
                    required_index += 1;
                }
                _ => {
                    // Specific heading required
                    if self.compare_headings(expected, &actual) {
                        required_index += 1;
                        match_any = false;
                    } else if match_any {
                        // We're in a "match any" state, so continue without advancing
                        continue;
                    } else {
                        // Expected specific heading but got something else
                        self.violations.push(RuleViolation::new(
                            &MD043,
                            format!("Expected: {expected}; Actual: {actual}"),
                            self.context.file_path.clone(),
                            range_from_tree_sitter(&heading.range),
                        ));
                        has_error = true;
                    }
                }
            }
        }

        // Check if there are unmatched required headings at the end
        let extra_headings = config.headings.len() - required_index;
        if !has_error
            && ((extra_headings > 1)
                || ((extra_headings == 1) && (config.headings[required_index] != "*")))
            && (any_headings || !config.headings.iter().all(|h| h == "*"))
        {
            // Report missing heading at end of file
            let last_line = self.context.get_document_content().lines().count();
            let missing_heading = &config.headings[required_index];

            // Create a range for the end of file
            let end_range = tree_sitter::Range {
                start_byte: self.context.get_document_content().len(),
                end_byte: self.context.get_document_content().len(),
                start_point: tree_sitter::Point {
                    row: last_line,
                    column: 0,
                },
                end_point: tree_sitter::Point {
                    row: last_line,
                    column: 0,
                },
            };

            self.violations.push(RuleViolation::new(
                &MD043,
                format!("Missing heading: {missing_heading}"),
                self.context.file_path.clone(),
                range_from_tree_sitter(&end_range),
            ));
        }
    }
}

impl RuleLinter for MD043Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "atx_heading" || node.kind() == "setext_heading" {
            let content = self.extract_heading_content(node);
            let level = self.extract_heading_level(node);

            self.headings.push(HeadingInfo {
                content,
                level,
                range: node.range(),
            });
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        self.check_required_headings();
        std::mem::take(&mut self.violations)
    }
}

pub const MD043: Rule = Rule {
    id: "MD043",
    alias: "required-headings",
    tags: &["headings"],
    description: "Required heading structure",
    rule_type: RuleType::Document,
    required_nodes: &["atx_heading", "setext_heading"],
    new_linter: |context| Box::new(MD043Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{LintersSettingsTable, MD043RequiredHeadingsTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_settings;

    fn test_config(headings: Vec<String>, match_case: bool) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![("required-headings", RuleSeverity::Error)],
            LintersSettingsTable {
                required_headings: MD043RequiredHeadingsTable {
                    headings,
                    match_case,
                },
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_no_required_headings() {
        let config = test_config(vec![], false);
        let input = "# Title\n\n## Section\n\nContent";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_exact_match() {
        let config = test_config(
            vec![
                "# Title".to_string(),
                "## Section".to_string(),
                "### Details".to_string(),
            ],
            false,
        );
        let input = "# Title\n\n## Section\n\n### Details\n\nContent";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_missing_heading() {
        let config = test_config(
            vec![
                "# Title".to_string(),
                "## Section".to_string(),
                "### Details".to_string(),
            ],
            false,
        );
        let input = "# Title\n\n### Details\n\nContent";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Expected: ## Section"));
    }

    #[test]
    fn test_wrong_heading() {
        let config = test_config(vec!["# Title".to_string(), "## Section".to_string()], false);
        let input = "# Title\n\n## Wrong Section\n\nContent";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Expected: ## Section"));
        assert!(violations[0].message().contains("Actual: ## Wrong Section"));
    }

    #[test]
    fn test_case_insensitive_match() {
        let config = test_config(vec!["# Title".to_string(), "## Section".to_string()], false);
        let input = "# TITLE\n\n## section\n\nContent";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_case_sensitive_match() {
        let config = test_config(vec!["# Title".to_string(), "## Section".to_string()], true);
        let input = "# TITLE\n\n## section\n\nContent";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1); // Only reports the first mismatch
        assert!(violations[0].message().contains("Expected: # Title"));
        assert!(violations[0].message().contains("Actual: # TITLE"));
    }

    #[test]
    fn test_zero_or_more_wildcard() {
        let config = test_config(
            vec![
                "# Title".to_string(),
                "*".to_string(),
                "## Important".to_string(),
            ],
            false,
        );
        let input = "# Title\n\n## Random\n\n### Sub\n\n## Important\n\nContent";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_one_or_more_wildcard() {
        let config = test_config(
            vec![
                "# Title".to_string(),
                "+".to_string(),
                "## Important".to_string(),
            ],
            false,
        );
        let input = "# Title\n\n## Random\n\n### Sub\n\n## Important\n\nContent";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_question_mark_wildcard() {
        let config = test_config(vec!["?".to_string(), "## Section".to_string()], false);
        let input = "# Any Title\n\n## Section\n\nContent";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_missing_heading_at_end() {
        let config = test_config(
            vec![
                "# Title".to_string(),
                "## Section".to_string(),
                "### Details".to_string(),
            ],
            false,
        );
        let input = "# Title\n\n## Section\n\nContent";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0]
            .message()
            .contains("Missing heading: ### Details"));
    }

    #[test]
    fn test_setext_headings() {
        let config = test_config(vec!["# Title".to_string(), "## Section".to_string()], false);
        let input = "Title\n=====\n\nSection\n-------\n\nContent";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_mixed_heading_styles() {
        let config = test_config(vec!["# Title".to_string(), "## Section".to_string()], false);
        let input = "Title\n=====\n\n## Section\n\nContent";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_closed_atx_headings() {
        let config = test_config(vec!["# Title".to_string(), "## Section".to_string()], false);
        let input = "# Title #\n\n## Section ##\n\nContent";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }
}
