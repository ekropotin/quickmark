use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

pub(crate) struct MD005Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD005Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }
}

impl RuleLinter for MD005Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "list" {
            self.check_list_indentation(node);
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

impl MD005Linter {
    fn check_list_indentation(&mut self, list_node: &Node) {
        let list_items = Self::get_direct_list_items_static(list_node);
        if list_items.len() < 2 {
            // Need at least 2 items to compare indentation
            return;
        }

        let is_ordered = Self::is_ordered_list_static(
            list_node,
            self.context.document_content.borrow().as_bytes(),
        );

        if is_ordered {
            self.check_ordered_list_indentation(list_node, &list_items);
        } else {
            self.check_unordered_list_indentation(list_node, &list_items);
        }
    }

    fn get_direct_list_items_static<'a>(list_node: &Node<'a>) -> Vec<Node<'a>> {
        let mut cursor = list_node.walk();
        list_node
            .children(&mut cursor)
            .filter(|c| c.kind() == "list_item")
            .collect()
    }

    fn is_ordered_list_static(list_node: &Node, content: &[u8]) -> bool {
        let mut list_cursor = list_node.walk();
        if let Some(first_item) = list_node
            .children(&mut list_cursor)
            .find(|c| c.kind() == "list_item")
        {
            let mut item_cursor = first_item.walk();
            // Use a for loop to make lifetimes explicit and avoid borrow checker issues.
            for child in first_item.children(&mut item_cursor) {
                if child.kind().starts_with("list_marker") {
                    if let Ok(text) = child.utf8_text(content) {
                        return text.contains('.');
                    }
                    // If a marker is found but its text cannot be read, assume it's not an ordered list.
                    return false;
                }
            }
        }
        false
    }

    fn check_unordered_list_indentation(&mut self, _list_node: &Node, list_items: &[Node]) {
        let expected_indent = self.get_list_item_indentation(&list_items[0]);

        for item in list_items.iter().skip(1) {
            let actual_indent = self.get_list_item_indentation(item);

            if actual_indent != expected_indent {
                let message = format!(
                    "{} [Expected: {}; Actual: {}]",
                    MD005.description, expected_indent, actual_indent
                );

                self.violations.push(RuleViolation::new(
                    &MD005,
                    message,
                    self.context.file_path.clone(),
                    range_from_tree_sitter(&item.range()),
                ));
            }
        }
    }

    fn check_ordered_list_indentation(&mut self, _list_node: &Node, list_items: &[Node]) {
        // Mimic the original markdownlint algorithm more closely
        let expected_indent = self.get_list_item_indentation(&list_items[0]);
        let mut expected_end = 0;
        let mut end_matching = false;

        for item in list_items {
            let actual_indent = self.get_list_item_indentation(item);
            let marker_length = self.get_list_marker_text_length(item);
            let actual_end = actual_indent + marker_length;

            expected_end = if expected_end == 0 {
                actual_end
            } else {
                expected_end
            };

            if expected_indent != actual_indent || end_matching {
                if expected_end == actual_end {
                    end_matching = true;
                } else {
                    let detail = if end_matching {
                        format!("Expected: ({expected_end}); Actual: ({actual_end})")
                    } else {
                        format!("Expected: {expected_indent}; Actual: {actual_indent}")
                    };

                    self.violations.push(RuleViolation::new(
                        &MD005,
                        format!("{} [{}]", MD005.description, detail),
                        self.context.file_path.clone(),
                        range_from_tree_sitter(&item.range()),
                    ));
                }
            }
        }
    }

    fn get_list_marker_text_length(&self, list_item: &Node) -> usize {
        let mut cursor = list_item.walk();
        if let Some(marker_node) = list_item
            .children(&mut cursor)
            .find(|c| c.kind().starts_with("list_marker"))
        {
            let content = self.context.document_content.borrow();
            if let Ok(text) = marker_node.utf8_text(content.as_bytes()) {
                return text.trim().len();
            }
        }
        0
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
}

pub const MD005: Rule = Rule {
    id: "MD005",
    alias: "list-indent",
    tags: &["bullet", "ul", "indentation"],
    description: "Inconsistent indentation for list items at the same level",
    rule_type: RuleType::Token,
    required_nodes: &["list"],
    new_linter: |context| Box::new(MD005Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{QuickmarkConfig, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> QuickmarkConfig {
        test_config_with_rules(vec![("list-indent", RuleSeverity::Error)])
    }

    #[test]
    fn test_consistent_unordered_list_indentation_no_violations() {
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
            "Consistent indentation should have no violations"
        );
    }

    #[test]
    fn test_inconsistent_unordered_list_indentation_has_violations() {
        let input = "* Item 1
 * Item 2 (1 space instead of 0)
* Item 3
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert!(
            !violations.is_empty(),
            "Inconsistent indentation should have violations"
        );
    }

    #[test]
    fn test_consistent_ordered_list_left_aligned_no_violations() {
        let input = "1. Item 1
2. Item 2
10. Item 10
11. Item 11
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Left-aligned ordered list should have no violations"
        );
    }

    #[test]
    fn test_consistent_ordered_list_right_aligned_no_violations() {
        let input = " 1. Item 1
 2. Item 2
10. Item 10
11. Item 11
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Right-aligned ordered list should have no violations"
        );
    }

    #[test]
    fn test_inconsistent_ordered_list_has_violations() {
        let input = "1. Item 1
 2. Item 2 (should be at same indent as item 1)
3. Item 3
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert!(
            !violations.is_empty(),
            "Inconsistent ordered list indentation should have violations"
        );
    }

    #[test]
    fn test_nested_lists_different_levels_no_violations() {
        let input = "* Item 1
  * Nested item 1
  * Nested item 2
* Item 2
  * Nested item 3
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Items at different nesting levels should not be compared"
        );
    }

    #[test]
    fn test_nested_lists_same_level_inconsistent() {
        let input = "* Item 1
  * Nested item 1
   * Nested item 2 (should be 2 spaces like item 1)
* Item 2
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert!(
            !violations.is_empty(),
            "Nested items at same level with inconsistent indent should have violations"
        );
    }

    #[test]
    fn test_mixed_ordered_unordered_lists() {
        let input = "1. Ordered item 1
2. Ordered item 2

* Unordered item 1  
* Unordered item 2
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Different list types should not interfere with each other"
        );
    }

    #[test]
    fn test_single_item_list_no_violations() {
        let input = "* Single item
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Single item lists should not have violations"
        );
    }

    #[test]
    fn test_empty_document_no_violations() {
        let input = "";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Empty documents should not have violations"
        );
    }

    #[test]
    fn test_ordered_list_with_different_number_lengths() {
        let input = " 1. Item 1
 2. Item 2
 3. Item 3
 4. Item 4
 5. Item 5
 6. Item 6
 7. Item 7
 8. Item 8
 9. Item 9
10. Item 10
11. Item 11
12. Item 12
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Right-aligned numbers should be consistent"
        );
    }

    #[test]
    fn test_ordered_list_inconsistent_right_alignment() {
        let input = " 1. Item 1
 2. Item 2
10. Item 10
 11. Item 11 (should align with 10, not with 1/2)
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert!(
            !violations.is_empty(),
            "Inconsistent right alignment should have violations"
        );
    }
}
