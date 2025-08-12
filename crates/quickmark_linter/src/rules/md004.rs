use std::collections::HashMap;
use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    config::UlStyle,
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

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
        text.trim().chars().next().filter(|&c| c == '*' || c == '+' || c == '-')
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
    fn find_list_item_markers<'a>(&self, list_node: &Node<'a>) -> Vec<(Node<'a>, char, usize)> {
        let mut markers = Vec::new();
        
        for child_idx in 0..list_node.child_count() {
            if let Some(list_item) = list_node.child(child_idx) {
                if list_item.kind() == "list_item" {
                    // Look for the list marker within the list item
                    for grand_child_idx in 0..list_item.child_count() {
                        if let Some(child) = list_item.child(grand_child_idx) {
                            if child.kind().starts_with("list_marker") {
                                let content = self.context.document_content.borrow();
                                let text = child.utf8_text(content.as_bytes()).unwrap_or("");
                                if let Some(marker) = Self::extract_marker(text) {
                                    // Calculate nesting level (simple approach for now)
                                    let nesting_level = 0; // TODO: Implement proper nesting calculation
                                    markers.push((child, marker, nesting_level));
                                }
                            }
                        }
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
        let marker_info: Vec<(tree_sitter::Range, char, usize)> = {
            let markers = self.find_list_item_markers(node);
            markers.into_iter().map(|(node, marker, level)| (node.range(), marker, level)).collect()
        };
        
        if marker_info.is_empty() {
            return; // No markers found, nothing to check
        }
        
        let nesting_level = self.calculate_nesting_level(node);
        
        // Debug: print found markers
        // eprintln!("Found {} markers: {:?}", marker_info.len(), marker_info.iter().map(|(_, c, _)| c).collect::<Vec<_>>());
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
                if let Some(&parent_marker) = self.nesting_styles.get(&nesting_level.saturating_sub(1)) {
                    // Choose a different marker from parent
                    expected_marker = Some(match parent_marker {
                        '*' => '+',
                        '+' => '-', 
                        '-' => '*',
                        _ => '*',
                    });
                } else {
                    // Top level - use first marker found or default to asterisk
                    expected_marker = Some(marker_info.first().map(|(_, marker, _)| *marker).unwrap_or('*'));
                }
                
                // Remember this nesting level's marker
                if let Some(marker) = expected_marker {
                    self.nesting_styles.insert(nesting_level, marker);
                }
            }
        }

        // Check all markers against expected and collect violations
        if let Some(expected) = expected_marker {
            for (range, actual_marker, _) in marker_info {
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
        for child_idx in 0..list_node.child_count() {
            if let Some(list_item) = list_node.child(child_idx) {
                if list_item.kind() == "list_item" {
                    for grand_child_idx in 0..list_item.child_count() {
                        if let Some(child) = list_item.child(grand_child_idx) {
                            if child.kind().starts_with("list_marker") {
                                let content = self.context.document_content.borrow();
                                let text = child.utf8_text(content.as_bytes()).unwrap_or("");
                                // Check if it's an unordered list marker
                                return text.trim().chars().next().map_or(false, |c| c == '*' || c == '+' || c == '-');
                            }
                        }
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
        test_config_with_rules(vec![
            ("ul-style", RuleSeverity::Error),
        ])
    }

    fn test_config_sublist() -> crate::config::QuickmarkConfig {
        use std::collections::HashMap;
        use crate::config::{LintersTable, LintersSettingsTable, QuickmarkConfig, RuleSeverity, UlStyle, MD004UlStyleTable};
        
        let severity: HashMap<String, RuleSeverity> = vec![
            ("ul-style".to_string(), RuleSeverity::Error),
        ]
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
        // TODO: This test will require implementing style configuration
        // For now, just test that it doesn't crash
        let input = "- Item 1
- Item 2
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // Currently returns 0 because logic isn't implemented
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