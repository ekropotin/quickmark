use serde::Deserialize;
use std::collections::HashMap;
use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

// MD004-specific configuration types
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum UlStyle {
    #[serde(rename = "asterisk")]
    Asterisk,
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "dash")]
    Dash,
    #[serde(rename = "plus")]
    Plus,
    #[serde(rename = "sublist")]
    Sublist,
}

impl Default for UlStyle {
    fn default() -> Self {
        Self::Consistent
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD004UlStyleTable {
    #[serde(default)]
    pub style: UlStyle,
}

impl Default for MD004UlStyleTable {
    fn default() -> Self {
        Self {
            style: UlStyle::Consistent,
        }
    }
}

pub(crate) struct MD004Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
    nesting_styles: HashMap<usize, char>, // Track expected markers by nesting level for sublist style
    document_expected_style: Option<char>, // Track expected style for the entire document in consistent mode
}

impl MD004Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
            nesting_styles: HashMap::new(),
            document_expected_style: None,
        }
    }

    /// Extract marker character from a text node
    fn extract_marker(text: &str) -> Option<char> {
        text.trim()
            .chars()
            .next()
            .filter(|&c| c == '*' || c == '+' || c == '-')
    }

    /// Convert marker character to style name for error messages
    fn marker_to_style_name(marker: char) -> &'static str {
        match marker {
            '*' => "asterisk",
            '+' => "plus",
            '-' => "dash",
            _ => "unknown",
        }
    }

    /// Get expected marker for a given style
    fn style_to_marker(style: &UlStyle) -> Option<char> {
        match style {
            UlStyle::Asterisk => Some('*'),
            UlStyle::Dash => Some('-'),
            UlStyle::Plus => Some('+'),
            UlStyle::Consistent | UlStyle::Sublist => None, // These are determined dynamically
        }
    }

    /// Find list item markers within a list node
    fn find_list_item_markers<'a>(&self, list_node: &Node<'a>) -> Vec<(Node<'a>, char)> {
        let mut markers = Vec::new();
        let content = self.context.document_content.borrow();
        let source_bytes = content.as_bytes();
        let mut list_cursor = list_node.walk();

        for list_item in list_node.children(&mut list_cursor) {
            if list_item.kind() == "list_item" {
                // This is the key: we need a new cursor for the sub-iteration
                let mut item_cursor = list_item.walk();
                for child in list_item.children(&mut item_cursor) {
                    if child.kind().starts_with("list_marker") {
                        if let Some(marker_char) = child
                            .utf8_text(source_bytes)
                            .ok()
                            .and_then(Self::extract_marker)
                        {
                            markers.push((child, marker_char));
                        }
                        // Once we find a marker for a list_item, we can stop searching its children.
                        break;
                    }
                }
            }
        }
        markers
    }

    /// Calculate nesting level of a list within other lists
    fn calculate_nesting_level(&self, list_node: &Node) -> usize {
        let mut nesting_level = 0;
        let mut current_node = *list_node;

        // Walk up the tree looking for parent list nodes
        while let Some(parent) = current_node.parent() {
            if parent.kind() == "list" {
                nesting_level += 1;
            }
            current_node = parent;
        }

        nesting_level
    }

    fn check_list(&mut self, node: &Node) {
        let style = &self.context.config.linters.settings.ul_style.style;

        // Extract marker information immediately to avoid lifetime issues
        let marker_info: Vec<(tree_sitter::Range, char)> = {
            let markers = self.find_list_item_markers(node);
            markers
                .into_iter()
                .map(|(node, marker)| (node.range(), marker))
                .collect()
        };

        if marker_info.is_empty() {
            return; // No markers found, nothing to check
        }

        let nesting_level = self.calculate_nesting_level(node);

        // Debug: print found markers
        // eprintln!("Found {} markers: {:?}", marker_info.len(), marker_info.iter().map(|(_, c)| c).collect::<Vec<_>>());
        // eprintln!("Nesting level: {}", nesting_level);
        let expected_marker: Option<char>;

        match style {
            UlStyle::Consistent => {
                // For consistent style, first marker in document sets the expected style
                if let Some(document_style) = self.document_expected_style {
                    expected_marker = Some(document_style);
                } else {
                    // First list in document - set the expected style
                    expected_marker = Some(marker_info[0].1);
                    self.document_expected_style = expected_marker;
                }
            }
            UlStyle::Asterisk | UlStyle::Dash | UlStyle::Plus => {
                expected_marker = Self::style_to_marker(style);
            }
            UlStyle::Sublist => {
                // Handle sublist style - each nesting level should differ from its parent
                if let Some(&parent_marker) =
                    self.nesting_styles.get(&nesting_level.saturating_sub(1))
                {
                    // Choose a different marker from parent
                    expected_marker = Some(match parent_marker {
                        '*' => '+',
                        '+' => '-',
                        '-' => '*',
                        _ => '*',
                    });
                } else {
                    // Top level - use first marker found or default to asterisk
                    expected_marker = Some(
                        marker_info
                            .first()
                            .map(|(_, marker)| *marker)
                            .unwrap_or('*'),
                    );
                }

                // Remember this nesting level's marker
                if let Some(marker) = expected_marker {
                    self.nesting_styles.insert(nesting_level, marker);
                }
            }
        }

        // Check all markers against expected and collect violations
        if let Some(expected) = expected_marker {
            for (range, actual_marker) in marker_info {
                if actual_marker != expected {
                    let message = format!(
                        "{} [Expected: {}; Actual: {}]",
                        MD004.description,
                        Self::marker_to_style_name(expected),
                        Self::marker_to_style_name(actual_marker)
                    );

                    self.violations.push(RuleViolation::new(
                        &MD004,
                        message,
                        self.context.file_path.clone(),
                        range_from_tree_sitter(&range),
                    ));
                }
            }
        }
    }
}

impl RuleLinter for MD004Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "list" {
            // Only check unordered lists, not ordered lists
            if self.is_unordered_list(node) {
                self.check_list(node);
            }
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

impl MD004Linter {
    /// Check if a list node is an unordered list by examining its first marker
    fn is_unordered_list(&self, list_node: &Node) -> bool {
        let mut list_cursor = list_node.walk();
        for list_item in list_node.children(&mut list_cursor) {
            if list_item.kind() == "list_item" {
                let mut item_cursor = list_item.walk();
                for child in list_item.children(&mut item_cursor) {
                    if child.kind().starts_with("list_marker") {
                        let content = self.context.document_content.borrow();
                        if let Ok(text) = child.utf8_text(content.as_bytes()) {
                            if let Some(marker_char) = text.trim().chars().next() {
                                return matches!(marker_char, '*' | '+' | '-');
                            }
                        }
                        return false; // Found marker, but failed to parse
                    }
                }
            }
        }
        false
    }
}

pub const MD004: Rule = Rule {
    id: "MD004",
    alias: "ul-style",
    tags: &["bullet", "ul"],
    description: "Unordered list style",
    rule_type: RuleType::Token,
    required_nodes: &["list"],
    new_linter: |context| Box::new(MD004Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![("ul-style", RuleSeverity::Error)])
    }

    fn test_config_sublist() -> crate::config::QuickmarkConfig {
        use super::{MD004UlStyleTable, UlStyle}; // Local import
        use crate::config::{LintersSettingsTable, LintersTable, QuickmarkConfig, RuleSeverity};
        use std::collections::HashMap;

        let severity: HashMap<String, RuleSeverity> =
            vec![("ul-style".to_string(), RuleSeverity::Error)]
                .into_iter()
                .collect();

        QuickmarkConfig::new(LintersTable {
            severity,
            settings: LintersSettingsTable {
                ul_style: MD004UlStyleTable {
                    style: UlStyle::Sublist,
                },
                ..Default::default()
            },
        })
    }

    fn test_config_asterisk() -> crate::config::QuickmarkConfig {
        use super::{MD004UlStyleTable, UlStyle}; // Local import
        use crate::config::{LintersSettingsTable, LintersTable, QuickmarkConfig, RuleSeverity};
        use std::collections::HashMap;

        let severity: HashMap<String, RuleSeverity> =
            vec![("ul-style".to_string(), RuleSeverity::Error)]
                .into_iter()
                .collect();

        QuickmarkConfig::new(LintersTable {
            severity,
            settings: LintersSettingsTable {
                ul_style: MD004UlStyleTable {
                    style: UlStyle::Asterisk,
                },
                ..Default::default()
            },
        })
    }

    fn test_config_dash() -> crate::config::QuickmarkConfig {
        use super::{MD004UlStyleTable, UlStyle}; // Local import
        use crate::config::{LintersSettingsTable, LintersTable, QuickmarkConfig, RuleSeverity};
        use std::collections::HashMap;

        let severity: HashMap<String, RuleSeverity> =
            vec![("ul-style".to_string(), RuleSeverity::Error)]
                .into_iter()
                .collect();

        QuickmarkConfig::new(LintersTable {
            severity,
            settings: LintersSettingsTable {
                ul_style: MD004UlStyleTable {
                    style: UlStyle::Dash,
                },
                ..Default::default()
            },
        })
    }

    fn test_config_plus() -> crate::config::QuickmarkConfig {
        use super::{MD004UlStyleTable, UlStyle}; // Local import
        use crate::config::{LintersSettingsTable, LintersTable, QuickmarkConfig, RuleSeverity};
        use std::collections::HashMap;

        let severity: HashMap<String, RuleSeverity> =
            vec![("ul-style".to_string(), RuleSeverity::Error)]
                .into_iter()
                .collect();

        QuickmarkConfig::new(LintersTable {
            severity,
            settings: LintersSettingsTable {
                ul_style: MD004UlStyleTable {
                    style: UlStyle::Plus,
                },
                ..Default::default()
            },
        })
    }

    #[test]
    fn test_consistent_asterisk_passes() {
        let input = "* Item 1
* Item 2
* Item 3
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_consistent_dash_passes() {
        let input = "- Item 1
- Item 2
- Item 3
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_consistent_plus_passes() {
        let input = "+ Item 1
+ Item 2
+ Item 3
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_inconsistent_mixed_fails() {
        let input = "* Item 1
+ Item 2
- Item 3
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should have violations for items 2 and 3 (inconsistent with item 1's asterisk)
        assert_eq!(2, violations.len());
    }

    #[test]
    fn test_asterisk_style_enforced() {
        let input = "- Item 1
- Item 2
";

        let config = test_config_asterisk();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should have violations for both items using dash instead of asterisk
        assert_eq!(2, violations.len());
        assert!(violations[0].message().contains("Expected: asterisk"));
        assert!(violations[0].message().contains("Actual: dash"));
    }

    #[test]
    fn test_dash_style_enforced() {
        let input = "* Item 1
+ Item 2
* Item 3
";

        let config = test_config_dash();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should have violations for all items not using dash
        assert_eq!(3, violations.len());
        assert!(violations[0].message().contains("Expected: dash"));
        assert!(violations[0].message().contains("Actual: asterisk"));
    }

    #[test]
    fn test_plus_style_enforced() {
        let input = "- Item 1
* Item 2
";

        let config = test_config_plus();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Should have violations for both items not using plus
        assert_eq!(2, violations.len());
        assert!(violations[0].message().contains("Expected: plus"));
        assert!(violations[0].message().contains("Actual: dash"));
    }

    #[test]
    fn test_asterisk_style_passes() {
        let input = "* Item 1
* Item 2
* Item 3
";

        let config = test_config_asterisk();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_dash_style_passes() {
        let input = "- Item 1
- Item 2
- Item 3
";

        let config = test_config_dash();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_plus_style_passes() {
        let input = "+ Item 1
+ Item 2
+ Item 3
";

        let config = test_config_plus();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_nested_lists_sublist_style() {
        let input = "* Item 1
  + Item 2
    - Item 3
  + Item 4
* Item 5
  + Item 6
";

        let config = test_config_sublist();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // This should be valid for sublist style - each level uses different markers
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_nested_lists_consistent_within_level() {
        let input = "* Item 1
  * Item 2  
    * Item 3
  * Item 4
* Item 5
  * Item 6
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_nested_lists_inconsistent_within_level_fails() {
        let input = "* Item 1
  + Item 2  
    - Item 3
  + Item 4
* Item 5
  - Item 6  // This should fail - inconsistent with level 1 asterisks
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // In consistent mode, all non-asterisk markers should violate (4 total: 2 plus, 1 dash, 1 dash)
        assert_eq!(4, violations.len());
    }

    #[test]
    fn test_single_item_list() {
        let input = "* Single item
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_empty_document() {
        let input = "";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_lists_separated_by_content() {
        let input = "* Item 1
* Item 2

Some paragraph text

- Item 3
- Item 4
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // In consistent mode, all lists in document should use same style
        // First list uses asterisk, so second list using dash should violate
        assert_eq!(2, violations.len());
    }
}
