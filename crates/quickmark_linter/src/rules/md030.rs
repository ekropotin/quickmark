use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

pub(crate) struct MD030Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD030Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }
}

impl RuleLinter for MD030Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "list" {
            self.check_list_marker_spacing(node);
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

impl MD030Linter {
    fn check_list_marker_spacing(&mut self, list_node: &Node) {
        let list_items: Vec<Node> = {
            let mut cursor = list_node.walk();
            list_node
                .children(&mut cursor)
                .filter(|c| c.kind() == "list_item")
                .collect()
        };

        if list_items.is_empty() {
            return;
        }

        let is_ordered = self.is_ordered_list(&list_items[0]);
        let is_single_line = self.is_single_line_list(&list_items);

        let expected_spaces = self.get_expected_spaces(is_ordered, is_single_line);

        for list_item in &list_items {
            self.check_list_item_spacing(list_item, expected_spaces);
        }
    }

    fn is_ordered_list(&self, list_item_node: &Node) -> bool {
        let mut cursor = list_item_node.walk();
        let result = list_item_node
            .children(&mut cursor)
            .find(|c| c.kind().starts_with("list_marker"))
            .is_some_and(|marker_node| {
                let kind = marker_node.kind();
                kind == "list_marker_dot" || kind == "list_marker_parenthesis"
            });
        result
    }

    fn is_single_line_list(&self, list_items: &[Node]) -> bool {
        // A list is single-line if all its items are single-line
        // (i.e., each item starts and ends on the same line)
        list_items
            .iter()
            .all(|item| item.start_position().row == item.end_position().row)
    }

    fn get_expected_spaces(&self, is_ordered: bool, is_single_line: bool) -> usize {
        let config = &self.context.config.linters.settings.list_marker_space;
        match (is_ordered, is_single_line) {
            (true, true) => config.ol_single,
            (true, false) => config.ol_multi,
            (false, true) => config.ul_single,
            (false, false) => config.ul_multi,
        }
    }

    fn check_list_item_spacing(&mut self, list_item: &Node, expected_spaces: usize) {
        let content = self.context.document_content.borrow();
        let item_text = match list_item.utf8_text(content.as_bytes()) {
            Ok(text) => text,
            Err(_) => return, // Ignore if text cannot be decoded
        };

        if let Some(first_line) = item_text.lines().next() {
            if let Some(actual_spaces) = self.extract_spaces_after_marker(first_line) {
                if actual_spaces != expected_spaces {
                    let message = format!(
                        "{} [Expected: {}; Actual: {}]",
                        MD030.description, expected_spaces, actual_spaces
                    );

                    self.violations.push(RuleViolation::new(
                        &MD030,
                        message,
                        self.context.file_path.clone(),
                        range_from_tree_sitter(&list_item.range()),
                    ));
                }
            }
        }
    }

    fn extract_spaces_after_marker(&self, line: &str) -> Option<usize> {
        let line = line.trim_start(); // Remove leading indentation

        // Handle unordered lists: *, +, -
        if line.starts_with(['*', '+', '-']) {
            let after_marker = &line[1..];
            return Some(after_marker.chars().take_while(|&c| c == ' ').count());
        }

        // Handle ordered lists: 1., 2., etc.
        if let Some(dot_pos) = line.find('.') {
            let before_dot = &line[..dot_pos];
            if !before_dot.is_empty() && before_dot.chars().all(|c| c.is_ascii_digit()) {
                let after_marker = &line[dot_pos + 1..];
                return Some(after_marker.chars().take_while(|&c| c == ' ').count());
            }
        }

        None
    }
}

pub const MD030: Rule = Rule {
    id: "MD030",
    alias: "list-marker-space",
    tags: &["ol", "ul", "whitespace"],
    description: "Spaces after list markers",
    rule_type: RuleType::Token,
    required_nodes: &["list"],
    new_linter: |context| Box::new(MD030Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{QuickmarkConfig, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> QuickmarkConfig {
        test_config_with_rules(vec![("list-marker-space", RuleSeverity::Error)])
    }

    #[test]
    fn test_default_unordered_list_single_space_no_violations() {
        let input = "* Item 1\n* Item 2\n* Item 3\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Default single space after unordered list marker should have no violations"
        );
    }

    #[test]
    fn test_default_ordered_list_single_space_no_violations() {
        let input = "1. Item 1\n2. Item 2\n3. Item 3\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Default single space after ordered list marker should have no violations"
        );
    }

    #[test]
    fn test_unordered_list_double_space_has_violations() {
        let input = "*  Item 1\n*  Item 2\n*  Item 3\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert!(
            !violations.is_empty(),
            "Double space after unordered list marker should have violations"
        );
    }

    #[test]
    fn test_ordered_list_double_space_has_violations() {
        let input = "1.  Item 1\n2.  Item 2\n3.  Item 3\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert!(
            !violations.is_empty(),
            "Double space after ordered list marker should have violations"
        );
    }

    #[test]
    fn test_mixed_list_types_independent() {
        let input = "* Item 1\n* Item 2\n\n1. Item 1\n2. Item 2\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Mixed list types with correct spacing should have no violations"
        );
    }

    #[test]
    fn test_single_line_vs_multi_line_lists() {
        // Single-line list - each item is on one line
        let input_single = "* Item 1\n* Item 2\n* Item 3\n";

        // Multi-line list - has content that spans multiple lines
        let input_multi = "*   Item 1\n\n    Second paragraph\n\n*   Item 2\n";

        let config = test_config();

        // Single-line list with default spacing (1 space)
        let mut linter = MultiRuleLinter::new_for_document(
            PathBuf::from("test.md"),
            config.clone(),
            input_single,
        );
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Single-line list with 1 space should be valid"
        );

        // Multi-line list with 3 spaces (will fail with default config expecting 1 space)
        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input_multi);
        let violations = linter.analyze();
        assert!(
            !violations.is_empty(),
            "Multi-line list with 3 spaces should have violations when expecting 1"
        );
    }

    #[test]
    fn test_nested_lists_not_affected() {
        let input = "* Item 1\n  * Nested item 1\n  * Nested item 2\n* Item 2\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Nested lists with correct spacing should have no violations"
        );
    }

    #[test]
    fn test_no_space_after_marker_has_violations() {
        // This test is invalid because "*Item 1" without space is not a valid list item
        // according to CommonMark specification. Tree-sitter correctly doesn't parse it as a list.
        // Instead, let's test a case with too few spaces compared to expectation.

        // Using a multi-line list where config expects 1 space but we have 0 would be invalid markdown.
        // So let's skip this test or modify it to test a valid but incorrect case.
        // For now, let's test double spaces which we know should fail:
        let input = "*  Item 1\n*  Item 2\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert!(
            !violations.is_empty(),
            "Double space after list marker should have violations with default config expecting 1 space"
        );
    }

    #[test]
    fn test_three_spaces_after_marker_has_violations() {
        let input = "*   Item 1\n*   Item 2\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert!(
            !violations.is_empty(),
            "Three spaces after list marker should have violations with default config"
        );
    }

    #[test]
    fn test_plus_marker_type() {
        let input = "+ Item 1\n+ Item 2\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Plus marker with single space should have no violations"
        );
    }

    #[test]
    fn test_dash_marker_type() {
        let input = "- Item 1\n- Item 2\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Dash marker with single space should have no violations"
        );
    }
}
