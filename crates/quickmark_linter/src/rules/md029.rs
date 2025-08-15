use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    config::OlPrefixStyle,
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

pub(crate) struct MD029Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
    // Document-wide state for one_or_ordered mode
    document_style: Option<OlPrefixStyle>,
}

impl MD029Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
            document_style: None,
        }
    }

    /// Extract the numeric value from an ordered list item prefix
    fn extract_list_item_value(&self, list_item_node: &Node) -> Option<u32> {
        let content = self.context.document_content.borrow();
        let source_bytes = content.as_bytes();

        // Find the list marker within this list item
        let mut cursor = list_item_node.walk();
        for child in list_item_node.children(&mut cursor) {
            if child.kind() == "list_marker_dot" {
                if let Ok(text) = child.utf8_text(source_bytes) {
                    // Extract the number from text like "1. " or "42. "
                    // First trim whitespace, then remove the dot
                    let trimmed = text.trim();
                    let number_str = trimmed.trim_end_matches('.');
                    return number_str.parse::<u32>().ok();
                }
            }
        }
        None
    }

    /// Check if a list node is an ordered list by examining its first marker
    fn is_ordered_list(&self, list_node: &Node) -> bool {
        let mut cursor = list_node.walk();
        for list_item in list_node.children(&mut cursor) {
            if list_item.kind() == "list_item" {
                let mut item_cursor = list_item.walk();
                for child in list_item.children(&mut item_cursor) {
                    if child.kind() == "list_marker_dot" {
                        return true; // Found dot marker, it's ordered
                    }
                }
            }
        }
        false
    }

    /// Get style examples for error messages
    fn get_style_example(style: &OlPrefixStyle) -> &'static str {
        match style {
            OlPrefixStyle::One => "1/1/1",
            OlPrefixStyle::Ordered => "1/2/3",
            OlPrefixStyle::OneOrOrdered => "1/1/1 or 1/2/3",
            OlPrefixStyle::Zero => "0/0/0",
        }
    }

    fn check_list(&mut self, node: &Node) {
        let configured_style = self.context.config.linters.settings.ol_prefix.style;

        // Extract list items and their values with position information
        let mut list_items_with_values = Vec::new();
        let mut cursor = node.walk();

        for list_item in node.children(&mut cursor) {
            if list_item.kind() == "list_item" {
                if let Some(value) = self.extract_list_item_value(&list_item) {
                    list_items_with_values.push((list_item, value));
                }
            }
        }

        if list_items_with_values.is_empty() {
            return; // No items, nothing to check
        }

        // Split the continuous list into logical separate lists like markdownlint
        let logical_lists = self.split_into_logical_lists(&list_items_with_values);

        for logical_list in logical_lists {
            match configured_style {
                OlPrefixStyle::OneOrOrdered => {
                    self.check_list_with_document_style(&logical_list);
                }
                OlPrefixStyle::One => {
                    self.check_list_with_fixed_style(&logical_list, OlPrefixStyle::One);
                }
                OlPrefixStyle::Zero => {
                    self.check_list_with_fixed_style(&logical_list, OlPrefixStyle::Zero);
                }
                OlPrefixStyle::Ordered => {
                    self.check_list_with_fixed_style(&logical_list, OlPrefixStyle::Ordered);
                }
            }
        }
    }

    /// Split tree-sitter's continuous list into logical separate lists like markdownlint
    fn split_into_logical_lists<'a>(
        &self,
        list_items_with_values: &[(Node<'a>, u32)],
    ) -> Vec<Vec<(Node<'a>, u32)>> {
        if list_items_with_values.len() <= 1 {
            return vec![list_items_with_values.to_vec()];
        }

        let content = self.context.document_content.borrow();
        let lines: Vec<&str> = content.lines().collect();

        let mut logical_lists = Vec::new();
        let mut current_list = Vec::new();

        for (i, (list_item, value)) in list_items_with_values.iter().enumerate() {
            current_list.push((*list_item, *value));

            // Check if this should end the current logical list
            let should_split = if i == list_items_with_values.len() - 1 {
                false // Don't split on the last item
            } else {
                let current_start_line = list_item.start_position().row;
                let next_start_line = list_items_with_values[i + 1].0.start_position().row;

                let has_separation = self.has_list_separation_between_starts(
                    current_start_line,
                    next_start_line,
                    &lines,
                );

                has_separation
            };

            if should_split {
                logical_lists.push(std::mem::take(&mut current_list));
            }
        }

        // Add the final list if it has items
        if !current_list.is_empty() {
            logical_lists.push(current_list);
        }

        logical_lists
    }

    /// Check if there's a logical separation between two list items based on their start positions
    fn has_list_separation_between_starts(
        &self,
        current_start: usize,
        next_start: usize,
        lines: &[&str],
    ) -> bool {
        // Check all lines between the start of current item and start of next item
        for line_idx in (current_start + 1)..next_start {
            if line_idx < lines.len() {
                let line = lines[line_idx].trim();

                // Empty line indicates separation
                if line.is_empty() {
                    return true;
                }

                // Non-empty line between items also indicates separation
                if !line.is_empty() {
                    // Check for headings (# title)
                    if line.starts_with('#') {
                        return true;
                    }
                    // Check for horizontal rules (--- or ***)
                    if line.starts_with("---") || line.starts_with("***") {
                        return true;
                    }
                    // Any other non-empty content (paragraph text, etc.)
                    if !line.starts_with(' ') && !line.starts_with('\t') {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn check_list_with_document_style(&mut self, list_items_with_values: &[(Node, u32)]) {
        // For OneOrOrdered mode, establish document-wide style from the first logical list
        if self.document_style.is_none() && list_items_with_values.len() >= 2 {
            // Determine document style from the first list
            let first_value = list_items_with_values[0].1;
            let second_value = list_items_with_values[1].1;

            if second_value != 1 || first_value == 0 {
                // Ordered style
                self.document_style = Some(OlPrefixStyle::Ordered);
            } else {
                // One style (1/1/...)
                self.document_style = Some(OlPrefixStyle::One);
            }
        }

        // If we still don't have a style (single item in first list), assume ordered style
        let effective_style = self.document_style.unwrap_or(OlPrefixStyle::Ordered);

        // For document-wide style, each logical list should follow the style
        match effective_style {
            OlPrefixStyle::One => {
                // One style: all items should be "1."
                for (list_item, actual_value) in list_items_with_values {
                    if actual_value != &1 {
                        let message = format!(
                            "{} [Expected: 1; Actual: {}; Style: {}]",
                            MD029.description,
                            actual_value,
                            Self::get_style_example(&effective_style)
                        );

                        self.violations.push(RuleViolation::new(
                            &MD029,
                            message,
                            self.context.file_path.clone(),
                            range_from_tree_sitter(&list_item.range()),
                        ));
                    }
                }
            }
            OlPrefixStyle::Ordered => {
                // Ordered style: in one_or_ordered mode, separate lists should follow ordered pattern
                // Lists starting with 0 or 1 are acceptable, others should be corrected
                if list_items_with_values.len() >= 1 {
                    let list_start_value = list_items_with_values[0].1;

                    // Determine expected start value (0 or 1 based on the actual start)
                    let expected_start = if list_start_value == 0 { 0 } else { 1 };

                    // Check each item in the list for proper ordering
                    let mut expected_value = expected_start;

                    for (list_item, actual_value) in list_items_with_values {
                        if actual_value != &expected_value {
                            let message = format!(
                                "{} [Expected: {}; Actual: {}; Style: {}]",
                                MD029.description,
                                expected_value,
                                actual_value,
                                Self::get_style_example(&effective_style)
                            );

                            self.violations.push(RuleViolation::new(
                                &MD029,
                                message,
                                self.context.file_path.clone(),
                                range_from_tree_sitter(&list_item.range()),
                            ));
                        }
                        expected_value += 1;
                    }
                }
            }
            _ => {} // Other styles not relevant here
        }
    }

    fn check_list_with_fixed_style(
        &mut self,
        list_items_with_values: &[(Node, u32)],
        style: OlPrefixStyle,
    ) {
        // For fixed styles, each list is independent (original behavior)
        if list_items_with_values.len() < 2 {
            return; // Single item lists are always valid
        }

        let (effective_style, mut expected_value) = match style {
            OlPrefixStyle::One => (OlPrefixStyle::One, 1),
            OlPrefixStyle::Zero => (OlPrefixStyle::Zero, 0),
            OlPrefixStyle::Ordered => (OlPrefixStyle::Ordered, list_items_with_values[0].1),
            OlPrefixStyle::OneOrOrdered => unreachable!(), // Handled separately
        };

        // Check each list item against the expected pattern
        for (list_item, actual_value) in list_items_with_values {
            let should_report = match effective_style {
                OlPrefixStyle::One => actual_value != &1,
                OlPrefixStyle::Zero => actual_value != &0,
                OlPrefixStyle::Ordered => actual_value != &expected_value,
                OlPrefixStyle::OneOrOrdered => unreachable!(),
            };

            if should_report {
                let message = format!(
                    "{} [Expected: {}; Actual: {}; Style: {}]",
                    MD029.description,
                    expected_value,
                    actual_value,
                    Self::get_style_example(&effective_style)
                );

                self.violations.push(RuleViolation::new(
                    &MD029,
                    message,
                    self.context.file_path.clone(),
                    range_from_tree_sitter(&list_item.range()),
                ));
            }

            // For ordered style, increment expected value (within this list only)
            if matches!(effective_style, OlPrefixStyle::Ordered) {
                expected_value += 1;
            }
        }
    }
}

impl RuleLinter for MD029Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "list" && self.is_ordered_list(node) {
            self.check_list(node);
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD029: Rule = Rule {
    id: "MD029",
    alias: "ol-prefix",
    tags: &["ol"],
    description: "Ordered list item prefix",
    rule_type: RuleType::Document,
    required_nodes: &["list"],
    new_linter: |context| Box::new(MD029Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{
        LintersSettingsTable, LintersTable, MD029OlPrefixTable, OlPrefixStyle, QuickmarkConfig,
        RuleSeverity,
    };
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;
    use std::collections::HashMap;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![("ol-prefix", RuleSeverity::Error)])
    }

    fn test_config_style(style: OlPrefixStyle) -> crate::config::QuickmarkConfig {
        let severity: HashMap<String, RuleSeverity> =
            vec![("ol-prefix".to_string(), RuleSeverity::Error)]
                .into_iter()
                .collect();

        QuickmarkConfig::new(LintersTable {
            severity,
            settings: LintersSettingsTable {
                ol_prefix: MD029OlPrefixTable { style },
                ..Default::default()
            },
        })
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
    fn test_single_item_list() {
        let input = "1. Single item\n";
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len(), "Single item lists should not violate");
    }

    #[test]
    fn test_no_ordered_lists() {
        let input = "* Unordered item\n- Another item\n+ Plus item\n";
        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len(), "Should not check unordered lists");
    }

    // Test one_or_ordered style (default)
    #[test]
    fn test_one_or_ordered_detects_one_style() {
        let input = "1. Item one\n1. Item two\n1. Item three\n";
        let config = test_config_style(OlPrefixStyle::OneOrOrdered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len(), "Should detect and allow 'one' style");
    }

    #[test]
    fn test_one_or_ordered_detects_ordered_style() {
        let input = "1. Item one\n2. Item two\n3. Item three\n";
        let config = test_config_style(OlPrefixStyle::OneOrOrdered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Should detect and allow 'ordered' style"
        );
    }

    #[test]
    fn test_one_or_ordered_detects_zero_based() {
        let input = "0. Item zero\n1. Item one\n2. Item two\n";
        let config = test_config_style(OlPrefixStyle::OneOrOrdered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Should detect and allow zero-based ordered style"
        );
    }

    #[test]
    fn test_one_or_ordered_violates_mixed_style() {
        let input = "1. Item one\n1. Item two\n3. Item three\n";
        let config = test_config_style(OlPrefixStyle::OneOrOrdered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len(), "Should violate inconsistent numbering");
        // In "one_or_ordered" mode with pattern 1/1/3, this should be detected as "one" style
        // So the violation should be that item 3 has "3" instead of "1"
        assert!(violations[0].message().contains("Expected: 1; Actual: 3"));
    }

    // Test "one" style
    #[test]
    fn test_one_style_passes() {
        let input = "1. Item one\n1. Item two\n1. Item three\n";
        let config = test_config_style(OlPrefixStyle::One);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len(), "All '1.' should pass");
    }

    #[test]
    fn test_one_style_violates_ordered() {
        let input = "1. Item one\n2. Item two\n3. Item three\n";
        let config = test_config_style(OlPrefixStyle::One);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len(), "Should violate items 2 and 3");
        assert!(violations[0].message().contains("Expected: 1; Actual: 2"));
        assert!(violations[1].message().contains("Expected: 1; Actual: 3"));
    }

    #[test]
    fn test_one_style_violates_zero_start() {
        let input = "0. Item zero\n1. Item one\n2. Item two\n";
        let config = test_config_style(OlPrefixStyle::One);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            2,
            violations.len(),
            "Should violate items with 0 and 2, but not 1"
        );
        assert!(violations[0].message().contains("Expected: 1; Actual: 0"));
        assert!(violations[1].message().contains("Expected: 1; Actual: 2"));
    }

    // Test "ordered" style
    #[test]
    fn test_ordered_style_passes_one_based() {
        let input = "1. Item one\n2. Item two\n3. Item three\n";
        let config = test_config_style(OlPrefixStyle::Ordered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len(), "Incrementing 1/2/3 should pass");
    }

    #[test]
    fn test_ordered_style_passes_zero_based() {
        let input = "0. Item zero\n1. Item one\n2. Item two\n";
        let config = test_config_style(OlPrefixStyle::Ordered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len(), "Incrementing 0/1/2 should pass");
    }

    #[test]
    fn test_ordered_style_violates_all_ones() {
        let input = "1. Item one\n1. Item two\n1. Item three\n";
        let config = test_config_style(OlPrefixStyle::Ordered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len(), "Should violate items 2 and 3");
        assert!(violations[0].message().contains("Expected: 2; Actual: 1"));
        assert!(violations[1].message().contains("Expected: 3; Actual: 1"));
    }

    #[test]
    fn test_ordered_style_violates_skip() {
        let input = "1. Item one\n2. Item two\n4. Item four\n";
        let config = test_config_style(OlPrefixStyle::Ordered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len(), "Should violate skipped number");
        assert!(violations[0].message().contains("Expected: 3; Actual: 4"));
    }

    // Test "zero" style
    #[test]
    fn test_zero_style_passes() {
        let input = "0. Item zero\n0. Item zero\n0. Item zero\n";
        let config = test_config_style(OlPrefixStyle::Zero);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len(), "All '0.' should pass");
    }

    #[test]
    fn test_zero_style_violates_ones() {
        let input = "1. Item one\n1. Item two\n1. Item three\n";
        let config = test_config_style(OlPrefixStyle::Zero);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(3, violations.len(), "Should violate all items");
        assert!(violations[0].message().contains("Expected: 0; Actual: 1"));
    }

    #[test]
    fn test_zero_style_violates_ordered() {
        let input = "0. Item zero\n1. Item one\n2. Item two\n";
        let config = test_config_style(OlPrefixStyle::Zero);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len(), "Should violate incrementing items");
        assert!(violations[0].message().contains("Expected: 0; Actual: 1"));
        assert!(violations[1].message().contains("Expected: 0; Actual: 2"));
    }

    // Test separate lists with document-wide consistency
    #[test]
    fn test_separate_lists_document_consistency() {
        let input = "1. First list item\n2. Second list item\n\nSome text\n\n1. New list item\n3. Should violate - expected 2\n";
        let config = test_config_style(OlPrefixStyle::OneOrOrdered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // First list establishes ordered style, second list should increment properly within itself
        assert_eq!(1, violations.len(), "Second list should increment properly");
        assert!(violations[0].message().contains("Expected: 2; Actual: 3"));
    }

    // Test zero-padded numbers (should work)
    #[test]
    fn test_zero_padded_ordered() {
        let input = "08. Item eight\n09. Item nine\n10. Item ten\n";
        let config = test_config_style(OlPrefixStyle::Ordered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(
            0,
            violations.len(),
            "Zero-padded ordered numbers should work"
        );
    }

    // Test edge case: large numbers
    #[test]
    fn test_large_numbers() {
        let input = "100. Item hundred\n101. Item hundred-one\n102. Item hundred-two\n";
        let config = test_config_style(OlPrefixStyle::Ordered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len(), "Large numbers should work");
    }

    // Test nested lists - each nesting level is independent
    #[test]
    fn test_nested_lists() {
        let input = "1. Outer item\n   1. Inner item\n   2. Inner item\n2. Outer item\n";
        let config = test_config_style(OlPrefixStyle::Ordered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len(), "Nested lists should be independent");
    }

    // Test mixed ordered and unordered
    #[test]
    fn test_mixed_list_types() {
        let input = "1. Ordered item\n* Unordered item\n2. Another ordered\n";
        let config = test_config_style(OlPrefixStyle::Ordered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let _violations = linter.analyze();
        // This depends on how tree-sitter parses this - if it creates separate lists, it should pass
        // We'll adjust based on actual tree-sitter behavior
    }

    // Test document-wide style consistency (markdownlint behavior)
    #[test]
    fn test_document_wide_style_consistency() {
        // First list establishes "ordered" style (1/2/3)
        // Subsequent lists in ordered style should start with 1 and increment
        let input = "# First section\n\n1. First item\n2. Second item\n3. Third item\n\n# Second section\n\n100. Should violate - expected 1\n102. Should violate - expected 2\n103. Should violate - expected 3\n";
        let config = test_config_style(OlPrefixStyle::OneOrOrdered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect violations where second list doesn't start with 1 in ordered style
        assert_eq!(
            3,
            violations.len(),
            "Should have 3 violations for wrong start in ordered style"
        );
        assert!(violations[0].message().contains("Expected: 1; Actual: 100"));
        assert!(violations[1].message().contains("Expected: 2; Actual: 102"));
        assert!(violations[2].message().contains("Expected: 3; Actual: 103"));
    }

    #[test]
    fn test_document_wide_zero_based_style() {
        // First list establishes "zero-based ordered" style (0/1/2)
        // Subsequent lists should follow ordered style, can start with 0 or 1
        let input = "# First section\n\n0. First item\n1. Second item\n2. Third item\n\n# Second section\n\n5. Should violate - expected 1\n5. Should violate - expected 2\n";
        let config = test_config_style(OlPrefixStyle::OneOrOrdered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(
            2,
            violations.len(),
            "Should have 2 violations for wrong start in ordered style"
        );
        assert!(violations[0].message().contains("Expected: 1; Actual: 5"));
        assert!(violations[1].message().contains("Expected: 2; Actual: 5"));
    }

    #[test]
    fn test_document_wide_one_style() {
        // First list establishes "one" style (1/1/1)
        // Subsequent lists should also use all 1s
        let input = "# First section\n\n1. First item\n1. Second item\n1. Third item\n\n# Second section\n\n1. Should pass\n2. Should violate - expected 1\n";
        let config = test_config_style(OlPrefixStyle::OneOrOrdered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        assert_eq!(
            1,
            violations.len(),
            "Should have 1 violation for not following 'one' style"
        );
        assert!(violations[0].message().contains("Expected: 1; Actual: 2"));
    }

    #[test]
    fn test_fixed_style_modes_ignore_document_consistency() {
        // When using fixed styles (not one_or_ordered), each list should be independent
        let input = "# First section\n\n1. First item\n2. Second item\n\n# Second section\n\n1. Different style OK\n1. In ordered mode\n";
        let config = test_config_style(OlPrefixStyle::Ordered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // In "ordered" mode, the second list should violate because it's not incrementing
        assert_eq!(
            1,
            violations.len(),
            "Should have 1 violation in second list for not incrementing"
        );
        assert!(violations[0].message().contains("Expected: 2; Actual: 1"));
    }

    // Tests for 100% markdownlint parity
    #[test]
    fn test_markdownlint_parity_blank_separated_lists() {
        // Markdownlint treats lists separated by blank lines as separate lists
        // Each should start with 1 in ordered style
        let input = "1. First list\n2. Second item\n\n100. Second list should violate\n101. Should also violate\n";
        let config = test_config_style(OlPrefixStyle::OneOrOrdered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have 2 violations: 100 (expected 1) and 101 (expected 2)
        assert_eq!(
            2,
            violations.len(),
            "Should treat blank-separated lists as separate"
        );
        assert!(violations[0].message().contains("Expected: 1; Actual: 100"));
        assert!(violations[1].message().contains("Expected: 2; Actual: 101"));
    }

    #[test]
    fn test_markdownlint_parity_zero_padded_separate() {
        // Zero-padded numbers in separate list should violate
        let input = "1. First\n2. Second\n\n08. Zero-padded start\n09. Next\n10. Third\n";
        let config = test_config_style(OlPrefixStyle::OneOrOrdered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have 3 violations for the zero-padded list not starting with 1
        assert_eq!(
            3,
            violations.len(),
            "Zero-padded separate list should violate"
        );
        assert!(violations[0].message().contains("Expected: 1; Actual: 8"));
        assert!(violations[1].message().contains("Expected: 2; Actual: 9"));
        assert!(violations[2].message().contains("Expected: 3; Actual: 10"));
    }

    #[test]
    fn test_markdownlint_parity_single_item_style_detection() {
        // Single items in separate lists should be checked against document style
        let input = "1. First\n2. Second\n\n42. Single item should violate\n";
        let config = test_config_style(OlPrefixStyle::OneOrOrdered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have 1 violation - single item doesn't match established ordered style
        assert_eq!(
            1,
            violations.len(),
            "Single item should follow document style"
        );
        // Note: markdownlint shows "Style: 1/1/1" for single items, suggesting different logic
        assert!(violations[0].message().contains("Expected: 1; Actual: 42"));
    }

    #[test]
    fn test_markdownlint_parity_mixed_with_headings() {
        // Lists separated by headings are definitely separate
        let input = "# Section 1\n\n1. First\n2. Second\n\n## Section 2\n\n5. Should violate\n6. Also violate\n\n### Section 3\n\n0. Zero start\n1. Should pass\n";
        let config = test_config_style(OlPrefixStyle::OneOrOrdered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have 2 violations for section 2 not starting with 1
        // Section 3 with 0/1 should be OK as it establishes ordered pattern
        assert_eq!(
            2,
            violations.len(),
            "Lists in different sections should be separate"
        );
        assert!(violations[0].message().contains("Expected: 1; Actual: 5"));
        assert!(violations[1].message().contains("Expected: 2; Actual: 6"));
    }

    #[test]
    fn test_markdownlint_parity_continuous_vs_separate() {
        // This tests the core difference: what markdownlint considers one list vs separate lists
        let input = "1. Item one\n2. Item two\n3. Item three\n\n1. Should this be separate?\n2. Or continuous?\n";
        let config = test_config_style(OlPrefixStyle::OneOrOrdered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Markdownlint would treat the second part as a separate list
        // So no violations expected (both lists follow ordered pattern correctly)
        assert_eq!(
            0,
            violations.len(),
            "Lists with proper ordered pattern should not violate"
        );
    }

    #[test]
    fn test_markdownlint_parity_text_separation() {
        // Lists separated by paragraph text should be separate
        let input = "1. First list\n2. Second item\n\nSome paragraph text here.\n\n5. Different start\n6. Should violate\n";
        let config = test_config_style(OlPrefixStyle::OneOrOrdered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have 2 violations for not starting with 1
        assert_eq!(
            2,
            violations.len(),
            "Text-separated lists should be independent"
        );
        assert!(violations[0].message().contains("Expected: 1; Actual: 5"));
        assert!(violations[1].message().contains("Expected: 2; Actual: 6"));
    }

    #[test]
    fn test_markdownlint_parity_one_style_detection() {
        // Test that 1/1/1 pattern is detected as "one" style and enforced
        let input = "1. All ones\n1. Pattern\n1. Here\n\n2. Should violate\n2. Different pattern\n";
        let config = test_config_style(OlPrefixStyle::OneOrOrdered);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should detect "one" style from first list, second list should violate for using 2s
        assert_eq!(2, violations.len(), "Should enforce one style globally");
        assert!(violations[0].message().contains("Expected: 1; Actual: 2"));
        assert!(violations[1].message().contains("Expected: 1; Actual: 2"));
    }
}
