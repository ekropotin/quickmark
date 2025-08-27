use core::fmt;
use serde::Deserialize;
use std::rc::Rc;
use tree_sitter::Node;

use crate::linter::{range_from_tree_sitter, Context, RuleLinter, RuleViolation};

use super::{Rule, RuleType};

// MD003-specific configuration types
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum HeadingStyle {
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "atx")]
    ATX,
    #[serde(rename = "setext")]
    Setext,
    #[serde(rename = "atx_closed")]
    ATXClosed,
    #[serde(rename = "setext_with_atx")]
    SetextWithATX,
    #[serde(rename = "setext_with_atx_closed")]
    SetextWithATXClosed,
}

impl Default for HeadingStyle {
    fn default() -> Self {
        Self::Consistent
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD003HeadingStyleTable {
    #[serde(default)]
    pub style: HeadingStyle,
}

impl Default for MD003HeadingStyleTable {
    fn default() -> Self {
        Self {
            style: HeadingStyle::Consistent,
        }
    }
}

#[derive(PartialEq, Debug)]
enum Style {
    Setext,
    Atx,
    AtxClosed,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Style::Setext => write!(f, "setext"),
            Style::Atx => write!(f, "atx"),
            Style::AtxClosed => write!(f, "atx_closed"),
        }
    }
}

pub(crate) struct MD003Linter {
    context: Rc<Context>,
    enforced_style: Option<Style>,
    violations: Vec<RuleViolation>,
}

impl MD003Linter {
    pub fn new(context: Rc<Context>) -> Self {
        // Access MD003 config through the centralized config structure
        let md003_config = &context.config.linters.settings.heading_style;
        let enforced_style = match md003_config.style {
            HeadingStyle::ATX => Some(Style::Atx),
            HeadingStyle::Setext => Some(Style::Setext),
            HeadingStyle::ATXClosed => Some(Style::AtxClosed),
            HeadingStyle::SetextWithATX => None, // Allow both setext and atx
            HeadingStyle::SetextWithATXClosed => None, // Allow setext and atx_closed
            _ => None,
        };
        Self {
            context,
            enforced_style,
            violations: Vec::new(),
        }
    }

    fn get_heading_level(&self, node: &Node) -> u8 {
        let mut cursor = node.walk();
        match node.kind() {
            "atx_heading" => node
                .children(&mut cursor)
                .find_map(|child| {
                    let kind = child.kind();
                    if kind.starts_with("atx_h") && kind.ends_with("_marker") {
                        // "atx_h3_marker" -> 3
                        kind.get(5..6)?.parse::<u8>().ok()
                    } else {
                        None
                    }
                })
                .unwrap_or(1),
            "setext_heading" => node
                .children(&mut cursor)
                .find_map(|child| match child.kind() {
                    "setext_h1_underline" => Some(1),
                    "setext_h2_underline" => Some(2),
                    _ => None,
                })
                .unwrap_or(1),
            _ => 1,
        }
    }

    fn is_atx_closed(&self, node: &Node) -> bool {
        // Use the idiomatic tree-sitter way to get the node's text.
        // This is more efficient than slicing the whole document manually.
        if let Ok(heading_text) = node.utf8_text(self.context.get_document_content().as_bytes()) {
            // Trim trailing whitespace and check if the heading ends with '#'.
            heading_text.trim_end().ends_with('#')
        } else {
            false
        }
    }

    fn add_violation(&mut self, node: &Node, expected: &str, actual: &Style) {
        self.violations.push(RuleViolation::new(
            &MD003,
            format!(
                "{} [Expected: {}; Actual: {}]",
                MD003.description, expected, actual
            ),
            self.context.file_path.clone(),
            range_from_tree_sitter(&node.range()),
        ));
    }
}

impl RuleLinter for MD003Linter {
    fn feed(&mut self, node: &Node) {
        let style = match node.kind() {
            "atx_heading" => {
                // Check if it's closed (has closing hashes)
                if self.is_atx_closed(node) {
                    Some(Style::AtxClosed)
                } else {
                    Some(Style::Atx)
                }
            }
            "setext_heading" => Some(Style::Setext),
            _ => None,
        };

        if let Some(style) = style {
            let level = self.get_heading_level(node);
            let config_style = &self.context.config.linters.settings.heading_style.style;

            match config_style {
                HeadingStyle::SetextWithATX => {
                    // Levels 1-2: must be setext, Levels 3+: must be atx (open), not atx_closed
                    if level <= 2 {
                        if style != Style::Setext {
                            self.add_violation(node, "setext", &style);
                        }
                    } else if style != Style::Atx {
                        self.add_violation(node, "atx", &style);
                    }
                }
                HeadingStyle::SetextWithATXClosed => {
                    // Levels 1-2: must be setext, Levels 3+: must be atx_closed, not plain atx
                    if level <= 2 {
                        if style != Style::Setext {
                            self.add_violation(node, "setext", &style);
                        }
                    } else if style != Style::AtxClosed {
                        self.add_violation(node, "atx_closed", &style);
                    }
                }
                _ => {
                    // For single-style configurations, check against enforced style
                    if let Some(enforced_style) = &self.enforced_style {
                        if style != *enforced_style {
                            self.add_violation(node, &enforced_style.to_string(), &style);
                        }
                    } else {
                        self.enforced_style = Some(style);
                    }
                }
            }
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD003: Rule = Rule {
    id: "MD003",
    alias: "heading-style",
    tags: &["headings"],
    description: "Heading style",
    rule_type: RuleType::Token,
    required_nodes: &["atx_heading", "setext_heading"],
    new_linter: |context| Box::new(MD003Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::{HeadingStyle, MD003HeadingStyleTable};
    use crate::config::{LintersSettingsTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_settings;

    fn test_config(style: HeadingStyle) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![
                ("heading-style", RuleSeverity::Error),
                ("heading-increment", RuleSeverity::Off),
            ],
            LintersSettingsTable {
                heading_style: MD003HeadingStyleTable { style },
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_heading_style_consistent_positive() {
        let config = test_config(HeadingStyle::Consistent);

        let input = "
Setext level 1
--------------
Setext level 2
==============
### ATX header level 3
#### ATX header level 4
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_heading_style_consistent_negative_setext() {
        let config = test_config(HeadingStyle::Consistent);

        let input = "
Setext level 1
--------------
Setext level 2
==============
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_heading_style_consistent_negative_atx() {
        let config = test_config(HeadingStyle::Consistent);

        let input = "
# Atx heading 1
## Atx heading 2
### Atx heading 3
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_heading_style_atx_positive() {
        let config = test_config(HeadingStyle::ATX);

        let input = "
Setext heading 1
----------------
Setext heading 2
================
### Atx heading 3
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_heading_style_atx_negative() {
        let config = test_config(HeadingStyle::ATX);

        let input = "
# Atx heading 1
## Atx heading 2
### Atx heading 3
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_heading_style_setext_positive() {
        let config = test_config(HeadingStyle::Setext);

        let input = "
# Atx heading 1
Setext heading 1
----------------
Setext heading 2
================
### Atx heading 3
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_heading_style_setext_negative() {
        let config = test_config(HeadingStyle::Setext);

        let input = "
Setext heading 1
----------------
Setext heading 2
================
Setext heading 2
================
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_heading_style_atx_closed_positive() {
        let config = test_config(HeadingStyle::ATXClosed);

        let input = "
# Open ATX heading 1
## Open ATX heading 2 ##
### ATX closed heading 3 ###
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_heading_style_atx_closed_negative() {
        let config = test_config(HeadingStyle::ATXClosed);

        let input = "
# ATX closed heading 1 #
## ATX closed heading 2 ##
### ATX closed heading 3 ###
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_heading_style_setext_with_atx_positive() {
        let config = test_config(HeadingStyle::SetextWithATX);

        let input = "
Setext heading 1
----------------
# Open ATX heading 2
## ATX closed heading 3 ##
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Level-based: setext h2 should be used for level 2, open ATX for level 3
        // Violations: ATX heading at level 2, closed ATX at level 3
        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_heading_style_setext_with_atx_negative() {
        let config = test_config(HeadingStyle::SetextWithATX);

        let input = "
Setext heading 1
----------------
Setext heading 2
----------------
### Open ATX heading 3
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Level-based: setext for 1-2, open ATX for 3+ - all correct
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_heading_style_setext_with_atx_closed_positive() {
        let config = test_config(HeadingStyle::SetextWithATXClosed);

        let input = "
Setext heading 1
----------------
# Open ATX heading 2
### Open ATX heading 3
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Level-based: setext for 1-2, closed ATX for 3+
        // Violations: open ATX at level 2, open ATX at level 3 (should be closed)
        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_heading_style_setext_with_atx_closed_negative() {
        let config = test_config(HeadingStyle::SetextWithATXClosed);

        let input = "
Setext heading 1
----------------
Setext heading 2
----------------
### ATX closed heading 3 ###
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Level-based: setext for 1-2, closed ATX for 3+ - all correct
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_setext_with_atx_level_violations_comprehensive() {
        let config = test_config(HeadingStyle::SetextWithATX);

        let input = "
# Level 1 ATX (should be setext)
## Level 2 ATX (should be setext)
### Level 3 ATX closed (should be open ATX) ###
#### Level 4 ATX closed (should be open ATX) ####
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Expect 4 violations: 2 for wrong style at levels 1-2, 2 for closed ATX at levels 3-4
        assert_eq!(violations.len(), 4);

        // Check specific violation messages
        assert!(violations[0]
            .message()
            .contains("Expected: setext; Actual: atx"));
        assert!(violations[1]
            .message()
            .contains("Expected: setext; Actual: atx"));
        assert!(violations[2]
            .message()
            .contains("Expected: atx; Actual: atx_closed"));
        assert!(violations[3]
            .message()
            .contains("Expected: atx; Actual: atx_closed"));
    }

    #[test]
    fn test_setext_with_atx_correct_level_usage() {
        let config = test_config(HeadingStyle::SetextWithATX);

        let input = "
Main Title
==========

Subtitle
--------

### Level 3 Open ATX
#### Level 4 Open ATX
##### Level 5 Open ATX
###### Level 6 Open ATX
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should have no violations - correct level-based usage
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_setext_with_atx_closed_level_violations_comprehensive() {
        let config = test_config(HeadingStyle::SetextWithATXClosed);

        let input = "
# Level 1 ATX (should be setext)
## Level 2 ATX (should be setext)
### Level 3 open ATX (should be closed ATX)
#### Level 4 open ATX (should be closed ATX)
##### Level 5 closed ATX is correct #####
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Expect 4 violations: 2 for wrong style at levels 1-2, 2 for open ATX at levels 3-4
        assert_eq!(violations.len(), 4);

        // Check specific violation messages
        assert!(violations[0]
            .message()
            .contains("Expected: setext; Actual: atx"));
        assert!(violations[1]
            .message()
            .contains("Expected: setext; Actual: atx"));
        assert!(violations[2]
            .message()
            .contains("Expected: atx_closed; Actual: atx"));
        assert!(violations[3]
            .message()
            .contains("Expected: atx_closed; Actual: atx"));
    }

    #[test]
    fn test_setext_with_atx_closed_correct_level_usage() {
        let config = test_config(HeadingStyle::SetextWithATXClosed);

        let input = "
Main Title
==========

Subtitle
--------

### Level 3 Closed ATX ###
#### Level 4 Closed ATX ####
##### Level 5 Closed ATX #####
###### Level 6 Closed ATX ######
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should have no violations - correct level-based usage
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_mixed_atx_styles_comprehensive() {
        let config = test_config(HeadingStyle::ATXClosed);

        let input = "
# Open ATX 1
## Closed ATX 2 ##
### Open ATX 3
#### Closed ATX 4 ####
##### Open ATX 5
###### Closed ATX 6 ######
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Expect 3 violations for open ATX headings (levels 1, 3, 5)
        assert_eq!(violations.len(), 3);

        for violation in &violations {
            assert!(violation
                .message()
                .contains("Expected: atx_closed; Actual: atx"));
        }
    }

    #[test]
    fn test_consistent_style_with_mixed_atx_variations() {
        let config = test_config(HeadingStyle::Consistent);

        let input = "
# First heading (sets the standard)
## Open ATX 2
### Closed ATX 3 ###
#### Open ATX 4
Setext heading
==============
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Expect 2 violations: closed ATX and setext (both different from first open ATX)
        assert_eq!(violations.len(), 2);

        assert!(violations[0]
            .message()
            .contains("Expected: atx; Actual: atx_closed"));
        assert!(violations[1]
            .message()
            .contains("Expected: atx; Actual: setext"));
    }

    #[test]
    fn test_file_without_trailing_newline_edge_case() {
        let config = test_config(HeadingStyle::Setext);

        // Test string without trailing newline (like our original issue)
        let input = "# ATX heading 1
## ATX heading 2
Final setext heading
--------------------";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should catch all 3 violations, including the final setext heading
        assert_eq!(violations.len(), 2); // Only ATX headings violate setext rule

        for violation in &violations {
            assert!(violation
                .message()
                .contains("Expected: setext; Actual: atx"));
        }
    }

    #[test]
    fn test_mix_of_styles() {
        let config = test_config(HeadingStyle::SetextWithATX);

        let input = "# Open ATX heading level 1

## Open ATX heading level 2

### Open ATX heading level 3 ###

#### Closed ATX heading level 4 ####

Setext heading level 1
======================

Setext heading level 2
----------------------

Another setext heading
======================

# Another open ATX

## Another closed ATX ##

Final setext heading
--------------------
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // - Level 1 ATX should be setext (1 violation)
        // - Level 2 ATX should be setext (2 violations)
        // - Level 3+ closed ATX should be open ATX (2 violations)
        // - Level 2 closed ATX should be setext (1 violation)
        // Total: 6 violations
        assert_eq!(violations.len(), 6);
    }

    #[test]
    fn test_atx_closed_detection_comprehensive() {
        let config = test_config(HeadingStyle::ATXClosed);

        let input = "# Open ATX
# Open ATX with spaces
## Open ATX level 2
### Closed ATX level 3 ###
#### Closed ATX with spaces ####
##### Closed ATX no spaces #####
###### Mixed closing hashes ##########
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect 3 open ATX violations (lines 1, 2, 3)
        assert_eq!(violations.len(), 3);

        for violation in &violations {
            assert!(violation
                .message()
                .contains("Expected: atx_closed; Actual: atx"));
        }
    }

    #[test]
    fn test_atx_closed_detection_edge_cases() {
        let config = test_config(HeadingStyle::ATX);

        let input = "# Regular ATX
## Closed ATX ##
### Unbalanced closing ########
#### Text with hash # in middle
##### Text ending with hash#
###### Actually closed ######
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Lines ending with # are considered closed: 2, 3, 5, 6
        // So we expect 4 violations for closed ATX when expecting open ATX
        assert_eq!(violations.len(), 4);

        for violation in &violations {
            assert!(violation
                .message()
                .contains("Expected: atx; Actual: atx_closed"));
        }
    }

    #[test]
    fn test_whitespace_handling_in_atx_closed_detection() {
        let config = test_config(HeadingStyle::ATXClosed);

        let input = "# Open ATX
## Closed with trailing spaces ##
### Closed with tabs ##
#### Open with trailing spaces
##### Closed no spaces #####
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect 2 open ATX violations (lines 1 and 4)
        assert_eq!(violations.len(), 2);

        for violation in &violations {
            assert!(violation
                .message()
                .contains("Expected: atx_closed; Actual: atx"));
        }
    }

    #[test]
    fn test_setext_only_supports_levels_1_and_2() {
        let config = test_config(HeadingStyle::Setext);

        let input = "Setext Level 1
==============

Setext Level 2
--------------

### Level 3 must be ATX ###
#### Level 4 must be ATX ####
";
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect 2 violations for ATX headings at levels 3-4
        assert_eq!(violations.len(), 2);

        for violation in &violations {
            assert!(violation
                .message()
                .contains("Expected: setext; Actual: atx_closed"));
        }
    }
}
