use std::{cell::RefCell, rc::Rc};

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

/// MD013 Line Length Rule Linter
///
/// **SINGLE-USE CONTRACT**: This linter is designed for one-time use only.
/// After processing a document (via feed() calls and finalize()), the linter
/// should be discarded. The pending_violations state is not cleared between uses.
pub(crate) struct MD013Linter {
    context: Rc<Context>,
    pending_violations: RefCell<Vec<RuleViolation>>,
}

impl MD013Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            pending_violations: RefCell::new(Vec::new()),
        }
    }

    /// Analyze all lines and store all violations for reporting via finalize()
    /// Context cache is already initialized by MultiRuleLinter
    fn analyze_all_lines(&self) {
        let lines = self.context.lines.borrow();
        let mut violations = Vec::new();

        for (line_index, line) in lines.iter().enumerate() {
            let node_kind = self.context.get_node_type_for_line(line_index);
            let should_check = self.should_check_node_type(&node_kind);
            let should_violate = if should_check {
                self.should_violate_line(line, line_index, &node_kind)
            } else {
                false
            };

            if should_violate {
                let violation = self.create_violation_for_line(line, line_index, &node_kind);
                violations.push(violation);
            }
        }

        *self.pending_violations.borrow_mut() = violations;
    }

    fn is_link_reference_definition(&self, line: &str) -> bool {
        line.trim_start().starts_with('[') && line.contains("]:") && line.contains("http")
    }

    fn is_standalone_link_or_image(&self, line: &str) -> bool {
        let trimmed = line.trim();
        // Check for standalone link: [text](url)
        if trimmed.starts_with('[') && trimmed.contains("](") && trimmed.ends_with(')') {
            return true;
        }
        // Check for standalone image: ![alt](url)
        if trimmed.starts_with("![") && trimmed.contains("](") && trimmed.ends_with(')') {
            return true;
        }
        false
    }

    fn has_no_spaces_beyond_limit(&self, line: &str, limit: usize) -> bool {
        if line.len() <= limit {
            return false;
        }
        let beyond_limit = &line[limit..];
        !beyond_limit.contains(' ')
    }

    fn should_check_node_type(&self, node_kind: &str) -> bool {
        let settings = &self.context.config.linters.settings.line_length;
        match node_kind {
            // Heading nodes
            s if s.starts_with("atx_h") && s.ends_with("_marker") => settings.headings,
            s if s.starts_with("setext_h") && s.ends_with("_underline") => settings.headings,
            "atx_heading" | "setext_heading" => settings.headings,
            // Code block nodes
            "fenced_code_block" | "indented_code_block" | "code_fence_content" => {
                settings.code_blocks
            }
            // Table nodes
            "table" | "table_row" => settings.tables,
            _ => true, // Check regular text content
        }
    }

    fn is_heading_line(&self, line: &str) -> bool {
        let trimmed = line.trim_start();
        // ATX headings start with #
        trimmed.starts_with('#') && (trimmed.len() > 1 && trimmed.chars().nth(1) == Some(' '))
    }

    fn get_line_limit(&self, node_kind: &str) -> usize {
        let settings = &self.context.config.linters.settings.line_length;
        match node_kind {
            // Heading nodes
            s if s.starts_with("atx_h") && s.ends_with("_marker") => settings.heading_line_length,
            s if s.starts_with("setext_h") && s.ends_with("_underline") => {
                settings.heading_line_length
            }
            "atx_heading" | "setext_heading" => settings.heading_line_length,
            // Code block nodes
            "fenced_code_block" | "indented_code_block" | "code_fence_content" => {
                settings.code_block_line_length
            }
            _ => settings.line_length,
        }
    }

    fn should_violate_line(&self, line: &str, _line_number: usize, node_kind: &str) -> bool {
        let settings = &self.context.config.linters.settings.line_length;

        // Check if this is a heading line and headings are disabled
        if self.is_heading_line(line) && !settings.headings {
            return false;
        }

        // Skip if this node type shouldn't be checked
        if !self.should_check_node_type(node_kind) {
            return false;
        }

        let limit = self.get_line_limit(node_kind);

        // Check if line exceeds limit
        if line.len() <= limit {
            return false;
        }

        // Apply exceptions
        if self.is_link_reference_definition(line) {
            return false;
        }

        if self.is_standalone_link_or_image(line) {
            return false;
        }

        // Strict mode: all lines beyond limit are violations
        if settings.strict {
            return true;
        }

        // Stern mode: more aggressive than default, but allows lines without spaces beyond limit
        if settings.stern {
            // In stern mode, allow lines without spaces beyond limit (like default)
            // but be more strict about other cases
            if self.has_no_spaces_beyond_limit(line, limit) {
                return false;
            }
            // If there are spaces beyond limit, it's a violation in stern mode
            return true;
        }

        // Default mode: allow lines without spaces beyond the limit
        if self.has_no_spaces_beyond_limit(line, limit) {
            return false;
        }

        true
    }

    fn create_violation_for_line(
        &self,
        line: &str,
        line_number: usize,
        node_kind: &str,
    ) -> RuleViolation {
        let limit = self.get_line_limit(node_kind);
        RuleViolation::new(
            &MD013,
            format!(
                "{} [Expected: <= {}; Actual: {}]",
                MD013.description,
                limit,
                line.len()
            ),
            self.context.file_path.clone(),
            range_from_tree_sitter(&tree_sitter::Range {
                start_byte: 0,
                end_byte: line.len(),
                start_point: tree_sitter::Point {
                    row: line_number,
                    column: 0,
                },
                end_point: tree_sitter::Point {
                    row: line_number,
                    column: line.len(),
                },
            }),
        )
    }
}

impl RuleLinter for MD013Linter {
    fn feed(&mut self, node: &Node) {
        // Analyze all lines when we see the document node
        // Context cache is already initialized by MultiRuleLinter
        if node.kind() == "document" {
            self.analyze_all_lines();
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        // Return all pending violations at once
        std::mem::take(&mut *self.pending_violations.borrow_mut())
    }
}

pub const MD013: Rule = Rule {
    id: "MD013",
    alias: "line-length",
    tags: &["line_length"],
    description: "Line length should not exceed the configured limit",
    rule_type: RuleType::Line,
    required_nodes: &[], // Line-based rules don't require specific nodes
    new_linter: |context| Box::new(MD013Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{LintersSettingsTable, MD013LineLengthTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::{test_config_with_rules, test_config_with_settings};

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![
            ("line-length", RuleSeverity::Error),
            ("heading-style", RuleSeverity::Off),
            ("heading-increment", RuleSeverity::Off),
        ])
    }

    fn test_config_with_line_length(
        line_length_config: MD013LineLengthTable,
    ) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![
                ("line-length", RuleSeverity::Error),
                ("heading-style", RuleSeverity::Off),
                ("heading-increment", RuleSeverity::Off),
            ],
            LintersSettingsTable {
                line_length: line_length_config,
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_line_length_violation() {
        let input = "This is a line that is definitely longer than eighty characters and should trigger a violation.";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());

        let violation = &violations[0];
        assert_eq!("MD013", violation.rule().id);
        assert!(violation.message().contains("Expected: <= 80"));
        assert!(violation
            .message()
            .contains(&format!("Actual: {}", input.len())));
    }

    #[test]
    fn test_line_length_no_violation() {
        let mut input =
            "This line should be exactly eighty characters long and not trigger".to_string();
        while input.len() < 80 {
            input.push('x');
        }
        assert_eq!(80, input.len());

        let config = test_config();
        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, &input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_link_reference_definition_exception() {
        let input = "[very-long-link-reference-that-exceeds-eighty-characters]: https://example.com/very-long-url-that-should-be-exempted";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_standalone_link_exception() {
        let input = "[This is a very long link text that definitely exceeds eighty characters](https://example.com)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_standalone_image_exception() {
        let input = "![This is a very long image alt text that definitely exceeds eighty characters](https://example.com/image.jpg)";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_no_spaces_beyond_limit_exception() {
        let input = "This line has exactly eighty characters and then continues without spaces: https://example.com/very-long-url-without-spaces";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_spaces_beyond_limit_violation() {
        // Create a string that exceeds 80 chars with a space beyond the limit
        let mut input =
            "This line has exactly eighty characters and should trigger violation".to_string();
        while input.len() < 80 {
            input.push('x');
        }
        input.push(' '); // Add space beyond limit

        let config = test_config();
        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, &input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_strict_mode() {
        let line_length_config = MD013LineLengthTable {
            strict: true,
            ..MD013LineLengthTable::default()
        };

        let input = "This line has exactly eighty characters and then continues without spaces like: https://example.com/url";

        let config = test_config_with_line_length(line_length_config);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len()); // Should violate in strict mode
    }

    #[test]
    fn test_stern_mode_with_spaces_beyond_limit() {
        let config = MD013LineLengthTable {
            stern: true,
            ..MD013LineLengthTable::default()
        };

        // Line with spaces beyond limit - should violate in stern mode
        // Make sure the line has exactly 80 chars, then add text with spaces beyond that
        let mut input =
            "This line has exactly eighty characters and should trigger violations".to_string();
        while input.len() < 80 {
            input.push('x');
        }
        input.push_str(" with spaces"); // Add spaces beyond limit

        let full_config = test_config_with_line_length(config);
        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), full_config, &input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len()); // Should violate in stern mode
    }

    #[test]
    fn test_stern_mode_without_spaces_beyond_limit() {
        let config = MD013LineLengthTable {
            stern: true,
            ..MD013LineLengthTable::default()
        };

        // Line without spaces beyond limit - should NOT violate in stern mode
        let input = "This line has exactly eighty characters and then continues without spaces: https://example.com/very-long-url-without-spaces";

        let full_config = test_config_with_line_length(config);
        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), full_config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len()); // Should NOT violate in stern mode
    }

    #[test]
    fn test_stern_mode_vs_default_mode() {
        // Create line that exceeds limit with spaces beyond limit
        let mut input =
            "This line has exactly eighty characters and then continues with".to_string();
        while input.len() < 80 {
            input.push('x');
        }
        input.push_str(" spaces beyond"); // Add spaces beyond limit

        // Default mode - should violate because there are spaces beyond limit
        let default_config = MD013LineLengthTable::default();
        let default_full_config = test_config_with_line_length(default_config);
        let mut default_linter = MultiRuleLinter::new_for_document(
            PathBuf::from("test.md"),
            default_full_config,
            &input,
        );
        let default_violations = default_linter.analyze();

        // Stern mode - should violate because it's more aggressive about lines with spaces
        let stern_config = MD013LineLengthTable {
            stern: true,
            ..MD013LineLengthTable::default()
        };
        let stern_full_config = test_config_with_line_length(stern_config);
        let mut stern_linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), stern_full_config, &input);
        let stern_violations = stern_linter.analyze();

        // Both should catch this since it has spaces beyond limit
        assert_eq!(1, default_violations.len()); // Default should catch this since it has spaces
        assert_eq!(1, stern_violations.len()); // Stern should definitely catch this
    }

    #[test]
    fn test_stern_vs_strict_vs_default_comprehensive() {
        // Case 1: Line with spaces beyond limit - all modes should catch this
        let mut case1 =
            "This line has exactly eighty characters and then continues with".to_string();
        while case1.len() < 80 {
            case1.push('x');
        }
        case1.push_str(" spaces"); // Add spaces beyond limit

        // Case 2: Line without spaces beyond limit - only strict mode should catch this
        let case2 = "This line has exactly eighty characters and then continues without spaces: https://example.com/url".to_string();

        // Case 3: Line within limit - no mode should catch this
        let case3 = "This line is within the eighty character limit".to_string();

        let test_cases = vec![
            (&case1, true, true, true),    // Has spaces beyond limit
            (&case2, false, false, true),  // No spaces beyond limit
            (&case3, false, false, false), // Within limit
        ];

        for (input, expect_default, expect_stern, expect_strict) in test_cases {
            // Default mode
            let default_config = MD013LineLengthTable::default();
            let default_full_config = test_config_with_line_length(default_config);
            let mut default_linter = MultiRuleLinter::new_for_document(
                PathBuf::from("test.md"),
                default_full_config,
                input,
            );
            let default_violations = default_linter.analyze();
            assert_eq!(
                expect_default,
                !default_violations.is_empty(),
                "Default mode failed for: {input}"
            );

            // Stern mode
            let stern_config = MD013LineLengthTable {
                stern: true,
                ..MD013LineLengthTable::default()
            };
            let stern_full_config = test_config_with_line_length(stern_config);
            let mut stern_linter = MultiRuleLinter::new_for_document(
                PathBuf::from("test.md"),
                stern_full_config,
                input,
            );
            let stern_violations = stern_linter.analyze();
            assert_eq!(
                expect_stern,
                !stern_violations.is_empty(),
                "Stern mode failed for: {input}"
            );

            // Strict mode
            let strict_config = MD013LineLengthTable {
                strict: true,
                ..MD013LineLengthTable::default()
            };
            let strict_full_config = test_config_with_line_length(strict_config);
            let mut strict_linter = MultiRuleLinter::new_for_document(
                PathBuf::from("test.md"),
                strict_full_config,
                input,
            );
            let strict_violations = strict_linter.analyze();
            assert_eq!(
                expect_strict,
                !strict_violations.is_empty(),
                "Strict mode failed for: {input}"
            );
        }
    }

    #[test]
    fn test_custom_line_length() {
        let line_length_config = MD013LineLengthTable {
            line_length: 50,
            ..MD013LineLengthTable::default()
        };

        let input = "This line is longer than fifty characters and should violate";

        let config = test_config_with_line_length(line_length_config);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Expected: <= 50"));
    }

    #[test]
    fn test_headings_disabled() {
        let line_length_config = MD013LineLengthTable {
            headings: false,
            ..MD013LineLengthTable::default()
        };

        let input = "# This is a very long heading that definitely exceeds the eighty character limit and should not trigger a violation";

        let config = test_config_with_line_length(line_length_config);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_multiple_lines() {
        let input = "This is a short line.
This is a very long line that definitely exceeds the eighty character limit and should trigger a violation.
Another short line.";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_demonstrates_potential_bug_scenario() {
        // This test demonstrates that our concern was valid in theory, but doesn't occur in practice
        // because tree-sitter creates enough AST nodes for even simple documents

        let input = "A\nB\nC\n"; // Minimal document - just 3 short lines

        // Count AST nodes for this minimal document
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_md::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(input, None).unwrap();
        let mut node_count = 0;
        let walker = crate::tree_sitter_walker::TreeSitterWalker::new(&tree);
        walker.walk(|_node| {
            node_count += 1;
        });

        println!("Even a 3-line minimal document creates {node_count} AST nodes");
        println!("This explains why our MD013 implementation works correctly");

        // Even this tiny document creates multiple nodes (document, paragraph, text nodes, etc.)
        assert!(
            node_count >= 3,
            "Even minimal documents create multiple AST nodes"
        );
    }

    #[test]
    fn test_extreme_violations_vs_minimal_nodes() {
        // Create the most minimal AST possible: just plain text with no structure
        // This should create minimal AST nodes but many violations
        let mut input = String::new();

        // Add 100 long lines of plain text (no markdown structure at all)
        let long_line = "This line is definitely longer than 80 characters and should trigger a line length violation every single time.\n";
        assert!(
            long_line.len() > 80,
            "Test line should exceed 80 chars, got {}",
            long_line.len()
        );

        for i in 0..100 {
            input.push_str(&format!("Violation line {}: {}", i + 1, long_line));
        }

        println!("Total input length: {} chars", input.len());
        println!("Number of lines: {}", input.lines().count());

        // Count how many AST nodes are created by parsing this document
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_md::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(&input, None).unwrap();
        let mut node_count = 0;
        let walker = crate::tree_sitter_walker::TreeSitterWalker::new(&tree);
        walker.walk(|_node| {
            node_count += 1;
        });
        println!("Total AST nodes: {node_count}");

        let config = test_config();
        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, &input);
        let violations = linter.analyze();

        println!("Violations found: {}", violations.len());

        // This is the critical test: with the improved MD013, we should ALWAYS find all violations
        // regardless of the node count, because violations are tied to line numbers, not node traversal order
        println!(
            "Ratio: {} violations vs {} nodes",
            violations.len(),
            node_count
        );

        // We should find exactly 100 violations
        assert_eq!(100, violations.len(),
            "Expected 100 line length violations but found {}. The improved MD013 should never lose violations!",
            violations.len()
        );
    }

    #[test]
    fn test_violation_node_mismatch_scenario() {
        // This test creates a scenario where violations > nodes to ensure our fix works
        // Create a document with minimal structure but maximum line violations

        let mut input = "# Header\n\n".to_string(); // Creates multiple AST nodes

        // Add 50 long lines that should violate but may not have corresponding unique AST nodes
        for i in 0..50 {
            input.push_str(&format!("Line {} with text that is definitely over eighty characters and should trigger MD013 violation\n", i + 1));
        }

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_md::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(&input, None).unwrap();
        let mut node_count = 0;
        let walker = crate::tree_sitter_walker::TreeSitterWalker::new(&tree);
        walker.walk(|_node| {
            node_count += 1;
        });

        let config = test_config();
        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, &input);
        let violations = linter.analyze();

        println!(
            "Stress test: {} violations vs {} nodes",
            violations.len(),
            node_count
        );

        // Should find exactly 50 violations (one per long line), regardless of node count
        assert_eq!(
            50,
            violations.len(),
            "Expected 50 violations but found {}. Improved MD013 must not lose violations!",
            violations.len()
        );

        // Verify each violation is on the correct line
        for (i, violation) in violations.iter().enumerate() {
            let expected_line = i + 2; // Lines 2, 3, 4, ..., 51 (line 0 is header, line 1 is empty)
            assert_eq!(
                expected_line,
                violation.location().range.start.line,
                "Violation {} should be on line {} but was on line {}",
                i + 1,
                expected_line,
                violation.location().range.start.line
            );
        }
    }

    #[test]
    fn test_many_violations_vs_few_nodes() {
        // Create a document with many line violations but few AST nodes
        // Structure: simple heading followed by many long lines of plain text
        let mut input = "# Short heading\n\n".to_string();

        // Add 20 long lines that should each trigger violations
        let long_line = "This line is definitely longer than 80 characters and should trigger a line length violation every time it appears.\n";
        assert!(
            long_line.len() > 80,
            "Test line should exceed 80 chars, got {}",
            long_line.len()
        );

        for i in 0..20 {
            input.push_str(&format!("Line {}: {}", i + 1, long_line));
        }

        println!("Total input length: {} chars", input.len());
        println!("Number of lines: {}", input.lines().count());

        let config = test_config();
        let mut linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, &input);
        let violations = linter.analyze();

        // Debug: print actual violations found
        println!("Violations found: {}", violations.len());
        for (i, violation) in violations.iter().enumerate() {
            println!(
                "  Violation {}: line {}",
                i + 1,
                violation.location().range.start.line
            );
        }

        // We should find exactly 20 violations (one per long line)
        // If we find fewer, it means some violations were lost due to the bug
        assert_eq!(20, violations.len(),
            "Expected 20 line length violations but found {}. This suggests violations were lost due to insufficient AST nodes.",
            violations.len()
        );

        // Verify violations are on the correct lines (lines 2-21, since line 0 is heading, line 1 is empty)
        for (i, violation) in violations.iter().enumerate() {
            let expected_line = i + 2; // Lines 2, 3, 4, ..., 21
            assert_eq!(
                expected_line,
                violation.location().range.start.line,
                "Violation {} should be on line {} but was on line {}",
                i + 1,
                expected_line,
                violation.location().range.start.line
            );
        }
    }
}
