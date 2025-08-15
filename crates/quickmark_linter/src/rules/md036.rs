use std::rc::Rc;

use tree_sitter::Node;

use crate::linter::{range_from_tree_sitter, Context, RuleLinter, RuleViolation};

use super::{Rule, RuleType};

pub(crate) struct MD036Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
}

impl MD036Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
        }
    }

    fn is_meaningful_node(node: &Node) -> bool {
        match node.kind() {
            "text" | "emphasis" | "strong_emphasis" | "inline" => true,
            _ => false,
        }
    }

    fn extract_text_content(&self, node: &Node) -> String {
        let source = self.context.get_document_content();
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        source[start_byte..end_byte].to_string()
    }

    fn check_inline_for_emphasis_heading(&mut self, inline_node: &Node) {
        // Get the text content of the inline node
        let inline_text = self.extract_text_content(inline_node);
        let trimmed_text = inline_text.trim();

        // Check if the entire inline text is emphasis (starts and ends with * or ** or _ or __)
        if (trimmed_text.starts_with("**")
            && trimmed_text.ends_with("**")
            && trimmed_text.len() > 4)
            || (trimmed_text.starts_with("__")
                && trimmed_text.ends_with("__")
                && trimmed_text.len() > 4)
            || (trimmed_text.starts_with("*")
                && trimmed_text.ends_with("*")
                && trimmed_text.len() > 2
                && !trimmed_text.starts_with("**"))
            || (trimmed_text.starts_with("_")
                && trimmed_text.ends_with("_")
                && trimmed_text.len() > 2
                && !trimmed_text.starts_with("__"))
        {
            // Extract the text inside the emphasis markers
            let inner_text = if trimmed_text.starts_with("**") || trimmed_text.starts_with("__") {
                &trimmed_text[2..trimmed_text.len() - 2]
            } else {
                &trimmed_text[1..trimmed_text.len() - 1]
            };

            // Process this as emphasis
            self.process_emphasis_text(inner_text, inline_node);
        }
    }

    fn process_emphasis_text(&mut self, inner_text: &str, source_node: &Node) {
        // Skip if text is empty
        if inner_text.trim().is_empty() {
            return;
        }

        // Check if text is single line (no newlines)
        if inner_text.contains('\n') {
            return;
        }

        // Check if text contains links - if so, allow it
        if inner_text.contains("[") && inner_text.contains("](") {
            return;
        }

        // Get punctuation configuration
        let punctuation_chars = &self
            .context
            .config
            .linters
            .settings
            .emphasis_as_heading
            .punctuation;

        // Check if text ends with punctuation
        if let Some(last_char) = inner_text.trim().chars().last() {
            if punctuation_chars.contains(last_char) {
                return; // Allow if ends with punctuation
            }
        }

        // Create violation
        let range = tree_sitter::Range {
            start_byte: 0, // Not used by range_from_tree_sitter
            end_byte: 0,   // Not used by range_from_tree_sitter
            start_point: tree_sitter::Point {
                row: source_node.start_position().row,
                column: source_node.start_position().column,
            },
            end_point: tree_sitter::Point {
                row: source_node.end_position().row,
                column: source_node.end_position().column,
            },
        };

        self.violations.push(RuleViolation::new(
            &MD036,
            format!("Emphasis used instead of heading: '{}'", inner_text.trim()),
            self.context.file_path.clone(),
            range_from_tree_sitter(&range),
        ));
    }

    fn process_emphasis_node(&mut self, emphasis_node: &Node) {
        let text_content = self.extract_text_content(emphasis_node);
        let trimmed_text = text_content.trim();

        // Skip if text is empty
        if trimmed_text.is_empty() {
            return;
        }

        // Check if text is single line (no newlines)
        if trimmed_text.contains('\n') {
            return;
        }

        // Check if the emphasis contains only text (no links, etc.)
        let mut has_only_text = true;
        let mut inner_cursor = emphasis_node.walk();
        if inner_cursor.goto_first_child() {
            loop {
                let inner_child = inner_cursor.node();
                if inner_child.kind() != "text" && !inner_child.kind().is_empty() {
                    has_only_text = false;
                    break;
                }
                if !inner_cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        if !has_only_text {
            return;
        }

        // Get punctuation configuration
        let punctuation_chars = &self
            .context
            .config
            .linters
            .settings
            .emphasis_as_heading
            .punctuation;

        // Check if text ends with punctuation
        if let Some(last_char) = trimmed_text.chars().last() {
            if punctuation_chars.contains(last_char) {
                return; // Allow if ends with punctuation
            }
        }

        // Create violation
        let range = tree_sitter::Range {
            start_byte: 0, // Not used by range_from_tree_sitter
            end_byte: 0,   // Not used by range_from_tree_sitter
            start_point: tree_sitter::Point {
                row: emphasis_node.start_position().row,
                column: emphasis_node.start_position().column,
            },
            end_point: tree_sitter::Point {
                row: emphasis_node.end_position().row,
                column: emphasis_node.end_position().column,
            },
        };

        self.violations.push(RuleViolation::new(
            &MD036,
            format!("Emphasis used instead of heading: '{trimmed_text}'"),
            self.context.file_path.clone(),
            range_from_tree_sitter(&range),
        ));
    }

    fn is_inside_list_item(&self, node: &Node) -> bool {
        let mut current = node.parent();
        while let Some(parent) = current {
            match parent.kind() {
                "list_item" => return true,
                "document" => return false, // Reached document root
                _ => current = parent.parent(),
            }
        }
        false
    }

    fn check_paragraph_for_emphasis_heading(&mut self, paragraph_node: &Node) {
        // Check if this paragraph is inside a list item - if so, skip it
        if self.is_inside_list_item(paragraph_node) {
            return;
        }

        // Check if paragraph contains only emphasis or strong emphasis
        let mut meaningful_children = Vec::new();
        let mut cursor = paragraph_node.walk();

        // Get all children of the paragraph
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if Self::is_meaningful_node(&child) {
                    meaningful_children.push(child);
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        // Check if paragraph has exactly one meaningful child that is an inline node
        if meaningful_children.len() == 1 {
            let child = meaningful_children[0];
            match child.kind() {
                "inline" => {
                    // Look inside the inline node for emphasis or strong emphasis
                    self.check_inline_for_emphasis_heading(&child);
                }
                "emphasis" | "strong_emphasis" => {
                    // Direct emphasis node (shouldn't happen with markdown structure, but handle it)
                    self.process_emphasis_node(&child);
                }
                _ => {
                    // Not an emphasis node, skip
                }
            }
        }
    }
}

impl RuleLinter for MD036Linter {
    fn feed(&mut self, node: &Node) {
        match node.kind() {
            "paragraph" => self.check_paragraph_for_emphasis_heading(node),
            _ => {
                // Ignore other nodes
            }
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD036: Rule = Rule {
    id: "MD036",
    alias: "no-emphasis-as-heading",
    tags: &["headings", "emphasis"],
    description: "Emphasis used instead of a heading",
    rule_type: RuleType::Token,
    required_nodes: &["paragraph"],
    new_linter: |context| Box::new(MD036Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{LintersSettingsTable, MD036EmphasisAsHeadingTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_settings;

    fn test_config(punctuation: &str) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![("no-emphasis-as-heading", RuleSeverity::Error)],
            LintersSettingsTable {
                emphasis_as_heading: MD036EmphasisAsHeadingTable {
                    punctuation: punctuation.to_string(),
                },
                ..Default::default()
            },
        )
    }

    fn test_default_config() -> crate::config::QuickmarkConfig {
        test_config(".,;:!?。，；：！？")
    }

    #[test]
    fn test_emphasis_as_heading_violation() {
        let config = test_default_config();
        let input = "**Section 1**\n\nSome content here.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Section 1"));
    }

    #[test]
    fn test_italic_emphasis_as_heading_violation() {
        let config = test_default_config();
        let input = "*Section 1*\n\nSome content here.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Section 1"));
    }

    #[test]
    fn test_valid_emphasis_in_paragraph() {
        let config = test_default_config();
        let input = "This is a normal paragraph with **some emphasis** in it.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_emphasis_with_punctuation_allowed() {
        let config = test_default_config();
        let input = "**This ends with punctuation.**\n\nSome content.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_multiline_emphasis_allowed() {
        let config = test_default_config();
        let input = "**This is an entire paragraph that has been emphasized\nand spans multiple lines**\n\nContent.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_custom_punctuation() {
        let config = test_config(".,;:");
        let input = "**This heading has exclamation!**\n\nContent.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1); // '!' not in custom punctuation
    }

    #[test]
    fn test_custom_punctuation_with_allowed() {
        let config = test_config(".,;:");
        let input = "**This heading has period.**\n\nContent.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_mixed_emphasis_and_normal_text() {
        let config = test_default_config();
        let input = "**Violation here**\n\nThis is a normal paragraph\n**that just happens to have emphasized text in**\neven though the emphasized text is on its own line.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1); // Only the first one should be flagged
    }

    #[test]
    fn test_emphasis_with_link() {
        let config = test_default_config();
        let input = "**[This is a link](https://example.com)**\n\nContent.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0); // Links should be allowed
    }

    #[test]
    fn test_full_width_punctuation() {
        let config = test_default_config();
        let input = "**Section with full-width punctuation。**\n\nContent.";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }
}
