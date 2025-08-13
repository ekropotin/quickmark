use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

pub(crate) struct MD007Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD007Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }
}

impl RuleLinter for MD007Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "list" && self.is_unordered_list(node) {
            self.check_list_indentation(node);
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

impl MD007Linter {
    /// Check if a list node is an unordered list by examining its first marker
    fn is_unordered_list(&self, list_node: &Node) -> bool {
        let mut list_cursor = list_node.walk();
        if let Some(first_item) = list_node
            .children(&mut list_cursor)
            .find(|c| c.kind() == "list_item")
        {
            let mut item_cursor = first_item.walk();
            for child in first_item.children(&mut item_cursor) {
                if child.kind().starts_with("list_marker") {
                    let content = self.context.document_content.borrow();
                    if let Ok(text) = child.utf8_text(content.as_bytes()) {
                        // Check if it's an unordered list marker
                        if let Some(marker_char) = text.trim().chars().next() {
                            return matches!(marker_char, '*' | '+' | '-');
                        }
                    }
                    // If marker is found but unreadable, assume not unordered
                    return false;
                }
            }
        }
        false
    }

    fn check_list_indentation(&mut self, list_node: &Node) {
        let nesting_level = self.calculate_nesting_level(list_node);

        // Only check unordered sublists if all parent lists are also unordered
        if nesting_level > 0 && !self.all_parents_unordered(list_node) {
            return;
        }

        let mut cursor = list_node.walk();
        for list_item in list_node.children(&mut cursor) {
            if list_item.kind() == "list_item" {
                // List items are indented at the same level as their parent list
                // The nesting level of a list item is the number of ancestor lists it has
                let item_nesting_level = self.calculate_list_item_nesting_level(&list_item);
                self.check_list_item_indentation(list_item, item_nesting_level);
            }
        }
    }

    fn check_list_item_indentation(&mut self, list_item: Node, nesting_level: usize) {
        let config = &self.context.config.linters.settings.ul_indent;
        let actual_indent = self.get_list_item_indentation(&list_item);
        let expected_indent = self.calculate_expected_indent(nesting_level, config);

        if actual_indent != expected_indent {
            let message = format!(
                "{} [Expected: {}; Actual: {}]",
                MD007.description, expected_indent, actual_indent
            );

            self.violations.push(RuleViolation::new(
                &MD007,
                message,
                self.context.file_path.clone(),
                range_from_tree_sitter(&list_item.range()),
            ));
        }
    }

    fn get_list_item_indentation(&self, list_item: &Node) -> usize {
        let content = self.context.document_content.borrow();
        let start_line = list_item.start_position().row;

        if let Some(line) = content.lines().nth(start_line) {
            // Count leading spaces/tabs (treating tabs as single characters for now)
            line.chars().take_while(|&c| c == ' ' || c == '\t').count()
        } else {
            0
        }
    }

    fn calculate_expected_indent(
        &self,
        nesting_level: usize,
        config: &crate::config::MD007UlIndentTable,
    ) -> usize {
        if nesting_level == 0 {
            // Top level
            if config.start_indented {
                config.start_indent
            } else {
                0
            }
        } else {
            // Nested levels
            let base_indent = if config.start_indented {
                config.start_indent
            } else {
                0
            };
            base_indent + (nesting_level * config.indent)
        }
    }

    fn calculate_nesting_level(&self, list_node: &Node) -> usize {
        let mut nesting_level = 0;
        let mut current_node = *list_node;

        // Walk up the tree looking for parent list nodes (any kind)
        while let Some(parent) = current_node.parent() {
            if parent.kind() == "list" {
                nesting_level += 1;
            }
            current_node = parent;
        }

        nesting_level
    }

    fn calculate_list_item_nesting_level(&self, list_item: &Node) -> usize {
        let mut nesting_level: usize = 0;
        let mut current_node = *list_item;

        // Walk up the tree looking for ancestor list nodes (any kind)
        while let Some(parent) = current_node.parent() {
            if parent.kind() == "list" {
                nesting_level += 1;
            }
            current_node = parent;
        }

        // List items are indented one level less than the number of ancestor lists
        // because the immediate parent list determines the indentation level
        nesting_level.saturating_sub(1)
    }

    fn all_parents_unordered(&self, list_node: &Node) -> bool {
        let mut current_node = *list_node;

        // Walk up the tree checking all parent list nodes
        while let Some(parent) = current_node.parent() {
            if parent.kind() == "list" && !self.is_unordered_list(&parent) {
                return false;
            }
            current_node = parent;
        }

        true
    }
}

pub const MD007: Rule = Rule {
    id: "MD007",
    alias: "ul-indent",
    tags: &["bullet", "indentation", "ul"],
    description: "Unordered list indentation",
    rule_type: RuleType::Token,
    required_nodes: &["list"],
    new_linter: |context| Box::new(MD007Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{
        LintersSettingsTable, LintersTable, MD007UlIndentTable, QuickmarkConfig, RuleSeverity,
    };
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;
    use std::collections::HashMap;

    fn test_config() -> QuickmarkConfig {
        test_config_with_rules(vec![("ul-indent", RuleSeverity::Error)])
    }

    fn test_config_custom(
        indent: usize,
        start_indent: usize,
        start_indented: bool,
    ) -> QuickmarkConfig {
        let severity: HashMap<String, RuleSeverity> =
            vec![("ul-indent".to_string(), RuleSeverity::Error)]
                .into_iter()
                .collect();

        QuickmarkConfig::new(LintersTable {
            severity,
            settings: LintersSettingsTable {
                ul_indent: MD007UlIndentTable {
                    indent,
                    start_indent,
                    start_indented,
                },
                ..Default::default()
            },
        })
    }

    #[test]
    fn test_default_settings_values() {
        let config = test_config();
        assert_eq!(2, config.linters.settings.ul_indent.indent);
        assert_eq!(2, config.linters.settings.ul_indent.start_indent);
        assert!(!config.linters.settings.ul_indent.start_indented);
    }

    #[test]
    fn test_custom_settings_values() {
        let config = test_config_custom(4, 3, true);
        assert_eq!(4, config.linters.settings.ul_indent.indent);
        assert_eq!(3, config.linters.settings.ul_indent.start_indent);
        assert!(config.linters.settings.ul_indent.start_indented);
    }

    #[test]
    fn test_proper_indentation_default_settings() {
        let input = "* Item 1
  * Item 2
    * Item 3
  * Item 4
* Item 5
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_improper_indentation_default_settings() {
        let input = "* Item 1
 * Item 2 (1 space, should be 2)
   * Item 3 (3 spaces, should be 2)
    * Item 4 (4 spaces, should be 4 for level 2)
* Item 5
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert!(
            !violations.is_empty(),
            "Should have violations for improper indentation"
        );
    }

    #[test]
    fn test_start_indented_false_default() {
        let input = "* Item 1
  * Item 2
* Item 3
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Top-level items should not be indented by default"
        );
    }

    #[test]
    fn test_start_indented_true() {
        let input = "  * Item 1
    * Item 2
  * Item 3
";

        let config = test_config_custom(2, 2, true);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Top-level items should be indented when start_indented=true"
        );
    }

    #[test]
    fn test_start_indented_true_wrong_indentation() {
        let input = "* Item 1 (should be indented by start_indent=2)
  * Item 2
";

        let config = test_config_custom(2, 2, true);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert!(
            !violations.is_empty(),
            "Should have violations when start_indented=true but top-level not indented"
        );
    }

    #[test]
    fn test_different_start_indent_value() {
        let input = "   * Item 1
     * Item 2
   * Item 3
";

        let config = test_config_custom(2, 3, true);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Should use start_indent=3 for first level when start_indented=true"
        );
    }

    #[test]
    fn test_custom_indent_value() {
        let input = "* Item 1
    * Item 2 (4 spaces for indent=4)
        * Item 3 (8 spaces for level 2 with indent=4)
    * Item 4
* Item 5
";

        let config = test_config_custom(4, 2, false);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len(), "Should accept custom indent=4");
    }

    #[test]
    fn test_mixed_lists_only_ul() {
        let input = "* Unordered item 1
  * Unordered item 2

1. Ordered item 1
   2. Ordered item 2 (this should be ignored)
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Should only check unordered lists, ignore ordered lists"
        );
    }

    #[test]
    fn test_nested_unordered_in_ordered() {
        let input = "1. Ordered item
   * Unordered nested (should be checked for indentation)
     * Deeper unordered nested
2. Another ordered item
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // The rule should only check unordered sublists if all parent lists are unordered
        // In this case, the parent is ordered, so it should be ignored
        assert_eq!(
            0,
            violations.len(),
            "Should ignore unordered lists nested in ordered lists"
        );
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
    fn test_multiple_list_blocks() {
        let input = "* List 1 item 1
  * List 1 item 2

Some text

* List 2 item 1
  * List 2 item 2
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }
}
