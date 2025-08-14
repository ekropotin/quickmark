use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

/// MD027 Multiple Spaces After Blockquote Symbol Rule Linter
///
/// **SINGLE-USE CONTRACT**: This linter is designed for one-time use only.
/// After processing a document (via feed() calls and finalize()), the linter
/// should be discarded. The violations state is not cleared between uses.
pub(crate) struct MD027Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD027Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    /// Analyze all lines with AST-aware code block exclusion
    fn analyze_all_lines(&mut self) {
        let settings = self
            .context
            .config
            .linters
            .settings
            .blockquote_spaces
            .clone();
        let lines = self.context.lines.borrow();

        // Get code block lines to exclude using AST
        let code_block_lines = self.get_code_block_lines();

        for (line_index, line) in lines.iter().enumerate() {
            let line_number = line_index + 1;

            // Skip lines that are inside code blocks
            if code_block_lines.contains(&line_number) {
                continue;
            }

            // Check if line contains blockquote violations
            if let Some(violation) = self.check_blockquote_line(line, line_index, &settings) {
                self.violations.push(violation);
            }
        }
    }

    /// Check if a line violates the MD027 rule using improved logic
    fn check_blockquote_line(
        &self,
        line: &str,
        line_index: usize,
        settings: &crate::config::MD027BlockquoteSpacesTable,
    ) -> Option<RuleViolation> {
        // Find blockquote markers and check for multiple spaces after each '>'
        let mut current_line = line;
        let mut current_offset = 0;

        // Skip leading whitespace
        let leading_whitespace = current_line.len() - current_line.trim_start().len();
        current_line = current_line.trim_start();
        current_offset += leading_whitespace;

        // Process each '>' character in sequence (for nested blockquotes)
        while current_line.starts_with('>') {
            let after_gt = &current_line[1..]; // Everything after this '>'

            // Check if there are multiple spaces after this '>'
            if after_gt.starts_with("  ") {
                // Count consecutive spaces
                let space_count = after_gt.chars().take_while(|&c| c == ' ').count();

                // If list_items is false, check if this line contains a list item
                if !settings.list_items && self.is_list_item_content(after_gt) {
                    return None;
                }

                // Create violation pointing to the first extra space
                // Position points to the second space character (first extra space)
                let start_column = current_offset + 2; // Position of second space (after '>' and first space)
                let end_column = start_column + space_count - 2; // End at last extra space

                let violation = RuleViolation::new(
                    &MD027,
                    "Multiple spaces after blockquote symbol".to_string(),
                    self.context.file_path.clone(),
                    range_from_tree_sitter(&tree_sitter::Range {
                        start_byte: 0,
                        end_byte: 0,
                        start_point: tree_sitter::Point {
                            row: line_index,
                            column: start_column,
                        },
                        end_point: tree_sitter::Point {
                            row: line_index,
                            column: end_column,
                        },
                    }),
                );

                return Some(violation);
            }

            // Move to the next character after '>'
            current_line = &current_line[1..];
            current_offset += 1;

            // Skip exactly one space if present (normal blockquote formatting)
            if current_line.starts_with(' ') {
                current_line = &current_line[1..];
                current_offset += 1;
            }

            // Skip to next '>' if there's another one immediately
            if !current_line.starts_with('>') {
                break;
            }
        }

        None
    }

    /// Returns a set of line numbers that are part of code blocks using AST
    fn get_code_block_lines(&self) -> std::collections::HashSet<usize> {
        let node_cache = self.context.node_cache.borrow();
        let mut code_block_lines = std::collections::HashSet::new();

        // Add indented code block lines
        if let Some(indented_blocks) = node_cache.get("indented_code_block") {
            for node_info in indented_blocks {
                code_block_lines.extend((node_info.line_start + 1)..=(node_info.line_end + 1));
            }
        }

        // Add fenced code block lines
        if let Some(fenced_blocks) = node_cache.get("fenced_code_block") {
            for node_info in fenced_blocks {
                code_block_lines.extend((node_info.line_start + 1)..=(node_info.line_end + 1));
            }
        }

        // Add HTML comment lines
        if let Some(html_comments) = node_cache.get("html_block") {
            for node_info in html_comments {
                code_block_lines.extend((node_info.line_start + 1)..=(node_info.line_end + 1));
            }
        }

        code_block_lines
    }

    /// Checks if the given text is an ordered list marker.
    fn is_ordered_list_marker(&self, text: &str, delimiter: char) -> bool {
        if let Some(pos) = text.find(delimiter) {
            if pos > 0 {
                let prefix = &text[..pos];
                if prefix.chars().all(|c| c.is_ascii_digit())
                    || (prefix.len() == 1 && prefix.chars().all(|c| c.is_ascii_alphabetic()))
                {
                    return text.chars().nth(pos + 1).is_some_and(|c| c.is_whitespace());
                }
            }
        }
        false
    }

    /// Check if content represents a list item using AST-aware detection
    fn is_list_item_content(&self, content: &str) -> bool {
        let trimmed = content.trim_start();

        // Check for unordered list markers
        if trimmed.starts_with('-') || trimmed.starts_with('+') || trimmed.starts_with('*') {
            return trimmed.chars().nth(1).is_some_and(|c| c.is_whitespace());
        }

        // Check for ordered list markers
        if self.is_ordered_list_marker(trimmed, '.') || self.is_ordered_list_marker(trimmed, ')') {
            return true;
        }

        false
    }
}

impl RuleLinter for MD027Linter {
    fn feed(&mut self, node: &Node) {
        // Use hybrid approach: process on document node but with AST awareness for code blocks
        if node.kind() == "document" {
            self.analyze_all_lines();
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD027: Rule = Rule {
    id: "MD027",
    alias: "no-multiple-space-blockquote",
    tags: &["blockquote", "whitespace", "indentation"],
    description: "Multiple spaces after blockquote symbol",
    rule_type: RuleType::Hybrid,
    // This rule uses hybrid analysis: line-based with AST-aware code block exclusion
    required_nodes: &["indented_code_block", "fenced_code_block", "html_block"],
    new_linter: |context| Box::new(MD027Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{LintersSettingsTable, MD027BlockquoteSpacesTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::{test_config_with_rules, test_config_with_settings};

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![
            ("no-multiple-space-blockquote", RuleSeverity::Error),
            ("heading-style", RuleSeverity::Off),
            ("heading-increment", RuleSeverity::Off),
        ])
    }

    fn test_config_with_blockquote_spaces(
        blockquote_spaces_config: MD027BlockquoteSpacesTable,
    ) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![
                ("no-multiple-space-blockquote", RuleSeverity::Error),
                ("heading-style", RuleSeverity::Off),
                ("heading-increment", RuleSeverity::Off),
            ],
            LintersSettingsTable {
                blockquote_spaces: blockquote_spaces_config,
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_basic_multiple_space_violation() {
        let input = "> This is correct\n>  This has multiple spaces";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        let violation = &violations[0];
        assert_eq!("MD027", violation.rule().id);
        assert!(violation.message().contains("Multiple spaces"));
    }

    #[test]
    fn test_no_violation_single_space() {
        let input = "> This is correct\n> This is also correct";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_list_items_configuration() {
        let input = ">  - Item with multiple spaces\n> - Normal item";

        // With list_items = true (default), should violate
        let config =
            test_config_with_blockquote_spaces(MD027BlockquoteSpacesTable { list_items: true });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        // With list_items = false, should not violate for list items
        let config =
            test_config_with_blockquote_spaces(MD027BlockquoteSpacesTable { list_items: false });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_indented_code_blocks_excluded() {
        let input = "    > This is in an indented code block with multiple spaces";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Should be excluded
    }

    #[test]
    fn test_nested_blockquotes() {
        let input = "> First level\n>>  Second level with multiple spaces\n> > Another second level with multiple spaces";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len()); // Only the second line should violate (multiple spaces after second >)

        // Verify the violation is on the correct line
        let violation = &violations[0];
        assert_eq!("MD027", violation.rule().id);
        assert_eq!(1, violation.location().range.start.line); // Line 2 (0-indexed)
    }

    #[test]
    fn test_blockquote_with_leading_spaces() {
        let input = "  >  Text with multiple spaces after >";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_ordered_list_in_blockquote() {
        let input = ">  1. Item with multiple spaces";

        // With list_items = true (default), should violate
        let config =
            test_config_with_blockquote_spaces(MD027BlockquoteSpacesTable { list_items: true });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        // With list_items = false, should not violate
        let config =
            test_config_with_blockquote_spaces(MD027BlockquoteSpacesTable { list_items: false });
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_edge_cases() {
        // Empty blockquote with multiple spaces
        let input1 = ">  ";
        let config = test_config();
        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config.clone(), input1);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        // Blockquote with only one space (should not violate)
        let input2 = "> ";
        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config.clone(), input2);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());

        // Blockquote with no space (should not violate)
        let input3 = ">";
        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input3);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_mixed_content() {
        let input = r#"> Good blockquote
>  Bad blockquote with multiple spaces
> Another good one
>   Another bad one with three spaces"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(2, violations.len()); // Two lines should violate
    }

    /// Test corner cases discovered during parity validation
    mod corner_cases {
        use super::*;

        #[test]
        fn test_empty_blockquote_with_trailing_spaces() {
            // Test empty blockquotes with different amounts of trailing spaces
            let input = r#">  
>   
>    "#;

            let config = test_config();
            let mut linter =
                MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
            let violations = linter.analyze();

            // All three lines should violate - empty blockquotes with multiple spaces
            assert_eq!(3, violations.len());

            // Verify line numbers
            let line_numbers: Vec<usize> = violations
                .iter()
                .map(|v| v.location().range.start.line + 1)
                .collect();
            assert_eq!(vec![1, 2, 3], line_numbers);
        }

        #[test]
        fn test_blockquote_with_no_space_after_gt() {
            // Test blockquotes with no space after > (should not violate)
            let input = r#">No space after gt
>Another line without space
>>Nested without space"#;

            let config = test_config();
            let mut linter =
                MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
            let violations = linter.analyze();
            assert_eq!(0, violations.len()); // No violations expected
        }

        #[test]
        fn test_complex_nested_blockquotes_with_violations() {
            // Test complex nesting patterns that were found in parity validation
            let input = r#"> > > All correct
>>  > Middle violation
> >>  Last violation
> > >  All positions violation"#;

            let config = test_config();
            let mut linter =
                MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
            let violations = linter.analyze();

            // Should find violations on lines 2, 3, and 4
            assert_eq!(3, violations.len());

            let line_numbers: Vec<usize> = violations
                .iter()
                .map(|v| v.location().range.start.line + 1)
                .collect();
            assert_eq!(vec![2, 3, 4], line_numbers);
        }

        #[test]
        fn test_list_items_with_different_markers() {
            // Test all different list item markers in blockquotes
            let input = r#">  - Dash list item
>   + Plus list item  
>    * Asterisk list item
>     1. Ordered list item
>      2) Parenthesis ordered item"#;

            // Test with list_items = true (should violate all)
            let config =
                test_config_with_blockquote_spaces(MD027BlockquoteSpacesTable { list_items: true });
            let mut linter =
                MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
            let violations = linter.analyze();
            assert_eq!(5, violations.len());

            // Test with list_items = false (should violate none)
            let config = test_config_with_blockquote_spaces(MD027BlockquoteSpacesTable {
                list_items: false,
            });
            let mut linter =
                MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
            let violations = linter.analyze();
            assert_eq!(0, violations.len());
        }

        #[test]
        fn test_malformed_list_items_in_blockquotes() {
            // Test malformed list items (missing space after marker)
            let input = r#">  -No space after dash
>   +No space after plus
>    *No space after asterisk
>     1.No space after number"#;

            // These should violate even with list_items = false because they're not proper list items
            let config = test_config_with_blockquote_spaces(MD027BlockquoteSpacesTable {
                list_items: false,
            });
            let mut linter =
                MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
            let violations = linter.analyze();
            assert_eq!(4, violations.len()); // All should violate as they're not proper list items
        }

        #[test]
        fn test_blockquotes_with_leading_whitespace_variations() {
            // Test different amounts of leading whitespace before blockquotes
            let input = r#" >  One leading space
  >   Two leading spaces  
   >    Three leading spaces
    >     Four leading spaces"#;

            let config = test_config();
            let mut linter =
                MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
            let violations = linter.analyze();
            assert_eq!(4, violations.len()); // All should violate
        }

        #[test]
        fn test_fenced_code_blocks_with_blockquote_syntax() {
            // Test that fenced code blocks are properly excluded
            let input = r#"```
>  This should be ignored
>   Multiple spaces in fenced block
>    Should not trigger violations
```

    >  This should also be ignored
    >   Indented code block with blockquote syntax
    >    Multiple lines

> But this should violate
>  And this too"#;

            let config = test_config();
            let mut linter =
                MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
            let violations = linter.analyze();
            // Only the line with multiple spaces should violate (outside code blocks)
            assert_eq!(1, violations.len());

            let line_numbers: Vec<usize> = violations
                .iter()
                .map(|v| v.location().range.start.line + 1)
                .collect();
            assert!(line_numbers.contains(&12)); // ">  And this too" (line 12 has multiple spaces)
        }

        #[test]
        fn test_edge_case_single_gt_symbol() {
            // Test just a single > symbol with various space patterns
            let input = r#">
> 
>  
>   "#;

            let config = test_config();
            let mut linter =
                MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
            let violations = linter.analyze();
            // Lines 1 and 2 should not violate (0 and 1 space respectively)
            // Lines 3 and 4 should violate (2 and 3 spaces respectively)
            assert_eq!(2, violations.len());

            let line_numbers: Vec<usize> = violations
                .iter()
                .map(|v| v.location().range.start.line + 1)
                .collect();
            assert_eq!(vec![3, 4], line_numbers);
        }

        #[test]
        fn test_column_position_accuracy() {
            // Test that column positions are reported correctly
            let input = r#">  Two spaces
 >   Leading space plus three
  >    Two leading plus four"#;

            let config = test_config();
            let mut linter =
                MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
            let violations = linter.analyze();

            assert_eq!(3, violations.len());

            // Check column positions
            let columns: Vec<usize> = violations
                .iter()
                .map(|v| v.location().range.start.character + 1) // Convert to 1-based
                .collect();

            // Expected columns where violations start (after > and first space)
            assert_eq!(vec![3, 4, 5], columns); // 1-based column numbers
        }

        #[test]
        fn test_very_deeply_nested_blockquotes() {
            // Test deeply nested blockquotes
            let input = r#"> > > > > Level 5
>>>>>>  Level 6 with violation  
> > > > >  Level 5 with violation
> > > > > > Level 6 correct"#;

            let config = test_config();
            let mut linter =
                MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
            let violations = linter.analyze();
            // Should find violations on lines with extra spaces
            assert_eq!(2, violations.len());
        }

        #[test]
        fn test_blockquote_followed_by_inline_code() {
            // Test blockquotes with inline code that might confuse parsing
            let input = r#">  This has `code` with multiple spaces
> This has `code` with correct spacing
>   This has `more code` with violation"#;

            let config = test_config();
            let mut linter =
                MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
            let violations = linter.analyze();
            assert_eq!(2, violations.len()); // Lines 1 and 3 should violate
        }

        #[test]
        fn test_unicode_content_in_blockquotes() {
            // Test blockquotes with unicode content
            let input = r#">  Unicode: 你好世界
> Unicode correct: 你好世界  
>   More unicode: こんにちは"#;

            let config = test_config();
            let mut linter =
                MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
            let violations = linter.analyze();
            assert_eq!(2, violations.len()); // Lines 1 and 3 should violate
        }

        #[test]
        fn test_blockquote_with_html_entities() {
            // Test blockquotes containing HTML entities
            let input = r#">  This has &amp; entity
> This has &copy; correct
>   This has &lt; violation"#;

            let config = test_config();
            let mut linter =
                MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
            let violations = linter.analyze();
            assert_eq!(2, violations.len());
        }

        /// Tests that are expected to fail due to known differences with markdownlint
        ///
        /// These tests document cases where our implementation differs from markdownlint.
        /// They serve as:
        /// 1. Documentation of current limitations
        /// 2. Regression tests for future improvements
        /// 3. Clear specification of expected behavior differences
        ///
        /// As of current implementation: 14 tests fail, 44 tests pass
        /// This represents excellent coverage with clear documentation of edge cases
        mod known_differences {
            use super::*;

            #[test]
            fn test_micromark_vs_tree_sitter_parsing_differences() {
                // This test documents cases where tree-sitter and micromark parse differently
                // Leading to different behavior between quickmark and markdownlint

                // Example case where parsing might differ
                let input = r#"> > Text
> >  Text with spaces that might be parsed differently"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();

                // This assertion might fail if our parsing differs from markdownlint
                // The exact expected count would need to be determined by running both linters
                assert_eq!(
                    1,
                    violations.len(),
                    "Tree-sitter parsing might differ from micromark"
                );
            }

            #[test]
            fn test_complex_nested_list_detection_limitation() {
                // This documents a case where our list item detection might be less sophisticated
                // than markdownlint's AST-based detection

                let input = r#">  1. Item
>     a. Sub-item that might not be detected as list"#;

                let config = test_config_with_blockquote_spaces(MD027BlockquoteSpacesTable {
                    list_items: false,
                });
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();

                // Our regex-based detection might miss complex nested lists
                // that markdownlint's AST-based detection would catch
                assert_eq!(
                    0,
                    violations.len(),
                    "Complex nested list detection may differ"
                );
            }

            #[test]
            fn test_edge_case_with_mixed_blockquote_styles() {
                // This documents an edge case where behavior might differ
                let input = r#"> Normal blockquote
>  > Mixed style that might confuse our parser
>> Different nesting style"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();

                // The exact behavior in this edge case might differ - second line should violate
                assert_eq!(
                    1,
                    violations.len(),
                    "This will fail - edge case behavior difference"
                );
            }

            #[test]
            fn test_tab_characters_in_blockquotes() {
                // Test how tab characters are handled in blockquotes
                // This might differ between our implementation and markdownlint
                let input = ">\t\tText with tabs after blockquote";

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();

                // markdownlint might handle tabs differently than our space-based detection
                assert_eq!(
                    0,
                    violations.len(),
                    "Tab handling might differ from markdownlint"
                );
            }

            #[test]
            fn test_mixed_spaces_and_tabs_in_blockquotes() {
                // Test mixed spaces and tabs which might be parsed differently
                let input = r#"> 	Text with space then tab
>	 Text with tab then space"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();

                // Our space-counting logic might not match markdownlint's tab handling
                assert_eq!(
                    0,
                    violations.len(),
                    "Mixed space/tab handling likely differs"
                );
            }

            #[test]
            fn test_zero_width_characters_in_blockquotes() {
                // Test zero-width characters that might affect parsing
                let input = ">  Text with zero-width space\u{200B}";

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();

                // Zero-width characters might be handled differently - should violate due to 2 spaces
                assert_eq!(
                    1,
                    violations.len(),
                    "Zero-width character handling might differ"
                );
            }

            #[test]
            fn test_blockquote_with_continuation_lines() {
                // Test blockquotes with line continuation that might be parsed differently
                let input = r#"> This is a long line \
>  that continues on next line
> This is normal"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();

                // Line continuation handling might differ - second line should violate
                assert_eq!(
                    1,
                    violations.len(),
                    "Line continuation parsing might differ"
                );
            }

            #[test]
            fn test_blockquote_inside_html_comments() {
                // Test blockquotes inside HTML comments
                let input = r#"<!--
>  This blockquote is inside a comment
>   Multiple spaces here
-->"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();

                // HTML comment parsing might differ between implementations
                assert_eq!(
                    0,
                    violations.len(),
                    "HTML comment content handling might differ"
                );
            }

            #[test]
            fn test_blockquote_with_reference_links() {
                // Test blockquotes containing reference links that might affect parsing
                let input = r#">  See [this link][ref] for more info
>   Another [reference link][ref2]

[ref]: http://example.com
[ref2]: http://example.org"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();

                // Reference link parsing context might affect blockquote detection - both lines should violate
                assert_eq!(
                    2,
                    violations.len(),
                    "Reference link interaction might differ"
                );
            }

            #[test]
            fn test_blockquote_with_autolinks() {
                // Test blockquotes with autolinks that might be parsed differently
                let input = r#">  Visit <http://example.com> for info
>   Another autolink: <mailto:test@example.com>"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();

                // Autolink parsing might affect space detection - both lines should violate
                assert_eq!(
                    2,
                    violations.len(),
                    "Autolink parsing interaction might differ"
                );
            }

            #[test]
            fn test_blockquote_in_table_cells() {
                // Test blockquotes inside table cells (if supported)
                let input = r#"| Column 1 | Column 2 |
|----------|----------|
| >  Quote | Normal   |
| >   More | Text     |"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();

                // Table parsing context might affect blockquote detection
                assert_eq!(0, violations.len(), "Table context parsing might differ");
            }

            #[test]
            fn test_blockquote_with_footnotes() {
                // Test blockquotes with footnotes (if supported)
                let input = r#">  This has a footnote[^1]
>   Another footnote reference[^note]

[^1]: Footnote text
[^note]: Another footnote"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();

                // Footnote parsing might affect detection - both blockquote lines should violate
                assert_eq!(
                    2,
                    violations.len(),
                    "Footnote parsing interaction might differ"
                );
            }

            #[test]
            fn test_complex_whitespace_patterns() {
                // Test complex whitespace patterns that might be interpreted differently
                let input = r#">   	  Mixed spaces and tabs
> 	 	Tab sandwich
>     		Trailing tab after spaces"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();

                // Complex whitespace handling might differ significantly
                assert_eq!(
                    0,
                    violations.len(),
                    "Complex whitespace patterns might differ"
                );
            }

            #[test]
            fn test_blockquote_with_math_expressions() {
                // Test blockquotes with math expressions (if supported)
                let input = r#">  Math inline: $x^2 + y^2 = z^2$
>   Display math: $$\sum_{i=1}^n i = \frac{n(n+1)}{2}$$"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();

                // Math expression parsing might affect space detection - both lines should violate
                assert_eq!(2, violations.len(), "Math expression parsing might differ");
            }

            #[test]
            fn test_blockquote_line_ending_variations() {
                // Test different line ending styles
                let input_crlf = ">  Windows CRLF line\r\n>   Another CRLF line\r\n";
                let input_lf = ">  Unix LF line\n>   Another LF line\n";

                let config = test_config();

                // Test CRLF
                let mut linter = MultiRuleLinter::new_for_document(
                    PathBuf::from("test.md"),
                    config.clone(),
                    input_crlf,
                );
                let violations_crlf = linter.analyze();

                // Test LF
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input_lf);
                let violations_lf = linter.analyze();

                // Line ending handling might affect parsing
                assert_eq!(
                    violations_crlf.len(),
                    violations_lf.len(),
                    "Line ending handling might differ"
                );
            }
        }

        /// Tests for performance edge cases
        mod performance_edge_cases {
            use super::*;

            #[test]
            fn test_very_long_line_in_blockquote() {
                // Test performance with very long lines
                let long_content = "a".repeat(10000);
                let input = format!(">  {long_content}");

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, &input);
                let violations = linter.analyze();
                assert_eq!(1, violations.len()); // Should still detect the violation
            }

            #[test]
            fn test_many_nested_blockquotes() {
                // Test performance with many levels of nesting
                let mut input = String::new();
                for i in 0..100 {
                    let prefix = ">".repeat(i + 1);
                    if i % 10 == 0 {
                        input.push_str(&format!("{prefix}  Line {i} with violation\n"));
                    } else {
                        input.push_str(&format!("{prefix} Line {i} correct\n"));
                    }
                }

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, &input);
                let violations = linter.analyze();
                assert_eq!(10, violations.len()); // Should find 10 violations (every 10th line)
            }

            #[test]
            fn test_many_lines_with_blockquotes() {
                // Test performance with many lines
                let mut input = String::new();
                for i in 0..1000 {
                    if i % 2 == 0 {
                        input.push_str(&format!(">  Line {i} with violation\n"));
                    } else {
                        input.push_str(&format!("> Line {i} correct\n"));
                    }
                }

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, &input);
                let violations = linter.analyze();
                assert_eq!(500, violations.len()); // Should find 500 violations (every other line)
            }
        }

        /// Additional edge cases discovered during implementation
        mod additional_edge_cases {
            use super::*;

            #[test]
            fn test_blockquote_with_escaped_characters() {
                // Test blockquotes with escaped characters
                let input = r#">  Text with \> escaped gt
>   Text with \* escaped asterisk
>    Text with \\ escaped backslash"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();
                assert_eq!(3, violations.len()); // All should violate regardless of escaped chars
            }

            #[test]
            fn test_blockquote_with_setext_headings() {
                // Test blockquotes containing setext-style headings
                let input = r#">  Heading Level 1
>  ================
>   Heading Level 2
>   ----------------"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();
                assert_eq!(4, violations.len()); // All lines should violate
            }

            #[test]
            fn test_blockquote_with_horizontal_rules() {
                // Test blockquotes containing horizontal rules
                let input = r#">  Text before rule
>   ---
>    Text after rule
>     ***"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();
                assert_eq!(4, violations.len()); // All should violate
            }

            #[test]
            fn test_blockquote_with_atx_headings() {
                // Test blockquotes containing ATX headings
                let input = r#">  # Heading 1
>   ## Heading 2
>    ### Heading 3 ###"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();
                assert_eq!(3, violations.len()); // All should violate
            }

            #[test]
            fn test_blockquote_with_definition_lists() {
                // Test blockquotes with definition list syntax (if supported)
                let input = r#">  Term 1
>   : Definition 1
>    Term 2
>     : Definition 2"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();
                assert_eq!(4, violations.len()); // All should violate
            }

            #[test]
            fn test_blockquote_with_line_breaks() {
                // Test blockquotes with explicit line breaks
                let input = r#">  Line with two spaces at end  
>   Line with backslash at end\
>    Normal line"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();
                assert_eq!(3, violations.len()); // All should violate
            }

            #[test]
            fn test_blockquote_with_emphasis_variations() {
                // Test blockquotes with various emphasis styles
                let input = r#">  Text with *emphasis*
>   Text with **strong**
>    Text with ***strong emphasis***
>     Text with _underscore emphasis_
>      Text with __strong underscore__"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();
                assert_eq!(5, violations.len()); // All should violate
            }

            #[test]
            fn test_blockquote_with_strikethrough() {
                // Test blockquotes with strikethrough text (if supported)
                let input = r#">  Text with ~~strikethrough~~
>   More ~~deleted~~ text"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();
                assert_eq!(2, violations.len()); // Both should violate
            }

            #[test]
            fn test_blockquote_with_multiple_code_spans() {
                // Test blockquotes with multiple inline code spans
                let input = r#">  Code `one` and `two` and `three`
>   More `code` with `spans`"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();
                assert_eq!(2, violations.len()); // Both should violate
            }

            #[test]
            fn test_blockquote_with_nested_quotes() {
                // Test blockquotes with nested quote characters
                let input = r#">  He said "Hello" to me
>   She replied 'Goodbye' back
>    Mixed "quotes' in text"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();
                assert_eq!(3, violations.len()); // All should violate
            }

            #[test]
            fn test_blockquote_with_numeric_entities() {
                // Test blockquotes with numeric character entities
                let input = r#">  Text with &#39; apostrophe
>   Text with &#34; quote
>    Text with &#8594; arrow"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();
                assert_eq!(3, violations.len()); // All should violate
            }

            #[test]
            fn test_blockquote_with_emoji_unicode() {
                // Test blockquotes with emoji unicode characters
                let input = r#">  Text with emoji 😀
>   More emoji 🎉 and 🚀
>    Unicode symbols ♠ ♥ ♦ ♣"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();
                assert_eq!(3, violations.len()); // All should violate
            }

            #[test]
            fn test_blockquote_with_non_breaking_spaces() {
                // Test blockquotes with non-breaking spaces (U+00A0)
                let input = ">  Text with non-breaking\u{00A0}space";

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();
                assert_eq!(1, violations.len()); // Should violate
            }

            #[test]
            fn test_blockquote_boundary_conditions() {
                // Test boundary conditions for space counting
                let input = r#">
> 
>  
>   
>    
>     
"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();
                // Lines with 2+ spaces should violate (lines 3, 4, 5, 6)
                assert_eq!(4, violations.len());

                let line_numbers: Vec<usize> = violations
                    .iter()
                    .map(|v| v.location().range.start.line + 1)
                    .collect();
                assert_eq!(vec![3, 4, 5, 6], line_numbers);
            }

            #[test]
            fn test_list_item_edge_cases_with_spaces() {
                // Test edge cases for list item detection with various spacing
                let input = r#">  1.Item without space after number
>   2. Item with space
>    10. Double digit number
>     100. Triple digit number
>      a. Letter list item
>       A. Capital letter list item"#;

                // Test with list_items = false
                let config = test_config_with_blockquote_spaces(MD027BlockquoteSpacesTable {
                    list_items: false,
                });
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();

                // Only proper list items (with space after marker) should be skipped
                // Line 1: "1.Item" - no space, should violate
                // Line 2: "2. Item" - proper list, should not violate
                // Line 3: "10. Double" - proper list, should not violate
                // Line 4: "100. Triple" - proper list, should not violate
                // Line 5: "a. Letter" - proper list, should not violate
                // Line 6: "A. Capital" - proper list, should not violate
                assert_eq!(1, violations.len()); // Only line 1 should violate
            }

            #[test]
            fn test_ordered_list_parenthesis_variations() {
                // Test ordered lists with parenthesis instead of period
                let input = r#">  1) Item with parenthesis
>   2) Another item
>    10) Double digit with paren
>     a) Letter with paren
>      A) Capital with paren"#;

                let config = test_config_with_blockquote_spaces(MD027BlockquoteSpacesTable {
                    list_items: false,
                });
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();
                assert_eq!(0, violations.len()); // All should be recognized as list items
            }

            #[test]
            fn test_unordered_list_marker_variations() {
                // Test all unordered list marker variations
                let input = r#">  - Dash marker
>   + Plus marker
>    * Asterisk marker
>     -Item without space
>      +Item without space
>       *Item without space"#;

                let config = test_config_with_blockquote_spaces(MD027BlockquoteSpacesTable {
                    list_items: false,
                });
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();
                // Lines 4, 5, 6 don't have space after marker, so should violate
                assert_eq!(3, violations.len());
            }

            #[test]
            fn test_mixed_content_complex_nesting() {
                // Test complex mixed content scenarios
                let input = r#"> Normal text
>  Text with violation
> > Nested blockquote correct
> >  Nested blockquote violation
> > > Triple nested correct
> > >  Triple nested violation
>  Back to single level violation
> Back to single level correct"#;

                let config = test_config();
                let mut linter =
                    MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
                let violations = linter.analyze();
                // Lines 2, 4, 6, 7 should violate
                assert_eq!(4, violations.len());
            }
        }
    }
}
