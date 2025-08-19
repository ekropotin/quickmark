use serde::Deserialize;
use std::rc::Rc;
use tree_sitter::Node;

use crate::linter::{range_from_tree_sitter, Context, RuleLinter, RuleViolation};

use super::{Rule, RuleType};

// MD022-specific configuration types
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD022HeadingsBlanksTable {
    #[serde(default)]
    pub lines_above: Vec<i32>,
    #[serde(default)]
    pub lines_below: Vec<i32>,
}

impl Default for MD022HeadingsBlanksTable {
    fn default() -> Self {
        Self {
            lines_above: vec![1],
            lines_below: vec![1],
        }
    }
}

pub(crate) struct MD022Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD022Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    fn get_lines_above(&self, heading_level: usize) -> i32 {
        let config = &self.context.config.linters.settings.headings_blanks;
        if heading_level > 0 && heading_level <= config.lines_above.len() {
            config.lines_above[heading_level - 1]
        } else if !config.lines_above.is_empty() {
            config.lines_above[0]
        } else {
            1 // Default
        }
    }

    fn get_lines_below(&self, heading_level: usize) -> i32 {
        let config = &self.context.config.linters.settings.headings_blanks;
        if heading_level > 0 && heading_level <= config.lines_below.len() {
            config.lines_below[heading_level - 1]
        } else if !config.lines_below.is_empty() {
            config.lines_below[0]
        } else {
            1 // Default
        }
    }

    fn get_heading_level(&self, node: &Node) -> usize {
        match node.kind() {
            "atx_heading" => {
                // Look for atx_hX_marker
                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();
                    if child.kind().starts_with("atx_h") && child.kind().ends_with("_marker") {
                        // "atx_h3_marker" => 3
                        return child.kind().chars().nth(5).unwrap().to_digit(10).unwrap() as usize;
                    }
                }
                1 // fallback
            }
            "setext_heading" => {
                // Look for setext_h1_underline or setext_h2_underline
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

    fn is_line_blank(&self, line_number: usize) -> bool {
        let lines = self.context.lines.borrow();
        if line_number < lines.len() {
            lines[line_number].trim().is_empty()
        } else {
            true // Consider out-of-bounds lines as blank
        }
    }

    fn count_blank_lines_above(&self, start_line: usize) -> usize {
        if start_line == 0 {
            return 0; // No lines above first line
        }

        let mut count = 0;
        let mut line_idx = start_line - 1;

        loop {
            if self.is_line_blank(line_idx) {
                count += 1;
                if line_idx == 0 {
                    break;
                }
                line_idx -= 1;
            } else {
                break;
            }
        }

        count
    }

    fn count_blank_lines_below(&self, end_line: usize) -> usize {
        let lines = self.context.lines.borrow();
        let mut count = 0;
        let mut line_idx = end_line + 1;

        while line_idx < lines.len() && self.is_line_blank(line_idx) {
            count += 1;
            line_idx += 1;
        }

        count
    }

    fn check_heading(&mut self, node: &Node) {
        let level = self.get_heading_level(node);
        let required_above = self.get_lines_above(level);
        let required_below = self.get_lines_below(level);

        let start_line = node.start_position().row;
        let end_line = node.end_position().row;

        // For setext headings, tree-sitter sometimes includes preceding content
        // We need to find the actual heading text line
        let actual_start_line = if node.kind() == "setext_heading" {
            // For setext headings, find the paragraph child which contains the heading text
            let mut heading_text_line = start_line;
            for i in 0..node.child_count() {
                let child = node.child(i).unwrap();
                if child.kind() == "paragraph" {
                    heading_text_line = child.start_position().row;
                    break;
                }
            }
            heading_text_line
        } else {
            start_line
        };

        let lines = self.context.lines.borrow();

        // Check lines above (only if required_above >= 0 and there's content above)
        if required_above >= 0 && actual_start_line > 0 {
            // Check if there's actual content above (not just blank lines)
            let has_content_above = (0..actual_start_line).any(|i| !self.is_line_blank(i));

            if has_content_above {
                let actual_above = self.count_blank_lines_above(actual_start_line);
                if (actual_above as i32) < required_above {
                    self.violations.push(RuleViolation::new(
                        &MD022,
                        format!(
                            "{} [Above: Expected: {}; Actual: {}]",
                            MD022.description, required_above, actual_above
                        ),
                        self.context.file_path.clone(),
                        range_from_tree_sitter(&node.range()),
                    ));
                }
            }
        }

        // Check lines below (only if required_below >= 0 and there's content below)
        // For ATX headings, they span one line (start_line)
        // For setext headings, they span two lines (text line + underline line)
        let effective_end_line = match node.kind() {
            "atx_heading" => actual_start_line,
            "setext_heading" => {
                // Find the underline line (setext_h1_underline or setext_h2_underline)
                let mut underline_line = end_line;
                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();
                    if child.kind() == "setext_h1_underline"
                        || child.kind() == "setext_h2_underline"
                    {
                        underline_line = child.start_position().row;
                        break;
                    }
                }
                underline_line
            }
            _ => end_line,
        };

        if required_below >= 0 && effective_end_line + 1 < lines.len() {
            // Check if there's actual content below (not just blank lines)
            let has_content_below =
                ((effective_end_line + 1)..lines.len()).any(|i| !self.is_line_blank(i));

            if has_content_below {
                let actual_below = self.count_blank_lines_below(effective_end_line);
                if (actual_below as i32) < required_below {
                    self.violations.push(RuleViolation::new(
                        &MD022,
                        format!(
                            "{} [Below: Expected: {}; Actual: {}]",
                            MD022.description, required_below, actual_below
                        ),
                        self.context.file_path.clone(),
                        range_from_tree_sitter(&node.range()),
                    ));
                }
            }
        }
    }
}

impl RuleLinter for MD022Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "atx_heading" || node.kind() == "setext_heading" {
            self.check_heading(node);
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD022: Rule = Rule {
    id: "MD022",
    alias: "blanks-around-headings",
    tags: &["headings", "blank_lines"],
    description: "Headings should be surrounded by blank lines",
    rule_type: RuleType::Hybrid,
    required_nodes: &["atx_heading", "setext_heading"],
    new_linter: |context| Box::new(MD022Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{LintersSettingsTable, MD022HeadingsBlanksTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_settings;

    fn test_config_with_blanks(
        blanks_config: MD022HeadingsBlanksTable,
    ) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![
                ("blanks-around-headings", RuleSeverity::Error),
                ("heading-style", RuleSeverity::Off),
                ("heading-increment", RuleSeverity::Off),
            ],
            LintersSettingsTable {
                headings_blanks: blanks_config,
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_default_config() {
        let config = test_config_with_blanks(MD022HeadingsBlanksTable::default());

        // Test violation: missing blank line above
        let input = "Some text
# Heading 1
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0]
            .message()
            .contains("Above: Expected: 1; Actual: 0"));
    }

    #[test]
    fn test_no_violation_with_correct_blanks() {
        let config = test_config_with_blanks(MD022HeadingsBlanksTable::default());

        let input = "Some text

# Heading 1

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_missing_blank_line_above() {
        let config = test_config_with_blanks(MD022HeadingsBlanksTable::default());

        let input = "Some text
# Heading 1

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0]
            .message()
            .contains("Above: Expected: 1; Actual: 0"));
    }

    #[test]
    fn test_missing_blank_line_below() {
        let config = test_config_with_blanks(MD022HeadingsBlanksTable::default());

        let input = "Some text

# Heading 1
More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0]
            .message()
            .contains("Below: Expected: 1; Actual: 0"));
    }

    #[test]
    fn test_both_missing_blank_lines() {
        let config = test_config_with_blanks(MD022HeadingsBlanksTable::default());

        let input = "Some text
# Heading 1
More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len());
        assert!(violations[0]
            .message()
            .contains("Above: Expected: 1; Actual: 0"));
        assert!(violations[1]
            .message()
            .contains("Below: Expected: 1; Actual: 0"));
    }

    #[test]
    fn test_setext_headings() {
        let config = test_config_with_blanks(MD022HeadingsBlanksTable::default());

        let input = "Some text
Heading 1
=========
More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Original markdownlint only finds the "Below" violation for this case
        // because tree-sitter includes preceding content in setext heading
        assert_eq!(1, violations.len());
        assert!(violations[0]
            .message()
            .contains("Below: Expected: 1; Actual: 0"));
    }

    #[test]
    fn test_custom_lines_above() {
        let config = test_config_with_blanks(MD022HeadingsBlanksTable {
            lines_above: vec![2],
            lines_below: vec![1],
        });

        let input = "Some text

# Heading 1

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0]
            .message()
            .contains("Above: Expected: 2; Actual: 1"));
    }

    #[test]
    fn test_custom_lines_below() {
        let config = test_config_with_blanks(MD022HeadingsBlanksTable {
            lines_above: vec![1],
            lines_below: vec![2],
        });

        let input = "Some text

# Heading 1

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0]
            .message()
            .contains("Below: Expected: 2; Actual: 1"));
    }

    #[test]
    fn test_heading_at_start_of_document() {
        let config = test_config_with_blanks(MD022HeadingsBlanksTable::default());

        let input = "# Heading 1

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should not violate - no content above to require blank line
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_heading_at_end_of_document() {
        let config = test_config_with_blanks(MD022HeadingsBlanksTable::default());

        let input = "Some text

# Heading 1";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should not violate - no content below to require blank line
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_disable_with_negative_one() {
        let config = test_config_with_blanks(MD022HeadingsBlanksTable {
            lines_above: vec![-1], // -1 means allow any number of blank lines
            lines_below: vec![1],
        });

        let input = "Some text
# Heading 1

More text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should not violate for lines above since -1 allows any number
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_per_heading_level_config() {
        let config = test_config_with_blanks(MD022HeadingsBlanksTable {
            lines_above: vec![1, 2, 0], // Level 1: 1 line, Level 2: 2 lines, Level 3: 0 lines
            lines_below: vec![1, 1, 1],
        });

        let input = "Text

# Level 1 - good


## Level 2 - good

### Level 3 - good

Text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_per_heading_level_violations() {
        let config = test_config_with_blanks(MD022HeadingsBlanksTable {
            lines_above: vec![1, 2, 0], // Level 1: 1 line, Level 2: 2 lines, Level 3: 0 lines
            lines_below: vec![1, 1, 1],
        });

        let input = "Text

# Level 1 - good

## Level 2 - bad (needs 2 above)

### Level 3 - good

Text";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0]
            .message()
            .contains("Above: Expected: 2; Actual: 1"));
    }
}
