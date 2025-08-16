use once_cell::sync::Lazy;
use regex::Regex;
use std::rc::Rc;
use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

// Pre-compiled regex patterns for image parsing
static IMG_TAG_REGEX: Lazy<Regex> = Lazy::new(|| {
    // Use DOTALL flag to match across newlines and case-insensitive flag
    Regex::new(r"(?si)<(/?)img\b[^>]*>").expect("Invalid img tag regex")
});

static ALT_ATTRIBUTE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?si)\balt\s*=\s*(?:[\"']([^\"']*)['"]|([^\s>]+))"#)
        .expect("Invalid alt attribute regex")
});

static ARIA_HIDDEN_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?si)aria-hidden\s*=\s*(?:[\"']([^\"']*)['"]|([^\s>]+))"#)
        .expect("Invalid aria-hidden regex")
});

// Regex patterns for Markdown images
static MARKDOWN_IMAGE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"!\[([^\]]*)\]\([^)]+\)").expect("Invalid markdown image regex"));

static MARKDOWN_REFERENCE_IMAGE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"!\[([^\]]*)\]\[([^\]]*)\]").expect("Invalid markdown reference image regex")
});

static MARKDOWN_REFERENCE_IMAGE_SHORTCUT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"!\[([^\]]*)\]\[]").expect("Invalid markdown reference image shortcut regex")
});

pub(crate) struct MD045Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
    line_starts: Vec<usize>,
}

impl MD045Linter {
    pub fn new(context: Rc<Context>) -> Self {
        // Pre-calculate line starts for efficient line/col lookup
        let line_starts: Vec<usize> = std::iter::once(0)
            .chain(
                context
                    .document_content
                    .borrow()
                    .match_indices('\n')
                    .map(|(i, _)| i + 1),
            )
            .collect();

        Self {
            context,
            violations: Vec::new(),
            line_starts,
        }
    }

    fn is_in_code_context(&self, node: &Node) -> bool {
        // Check if this node is inside a code span or code block
        let mut current = node.parent();
        while let Some(parent) = current {
            match parent.kind() {
                "code_span" | "fenced_code_block" | "indented_code_block" => {
                    return true;
                }
                _ => {
                    current = parent.parent();
                }
            }
        }
        false
    }

    fn contains_inline_code_with_images(&self, content: &str) -> bool {
        // Check if the entire content is a single inline code span containing images
        static CODE_SPAN_WITH_IMG_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^`[^`]*(?:<img|!\[)[^`]*`\s*(?:and\s*`[^`]*(?:<img|!\[)[^`]*`\s*)*$")
                .expect("Invalid code span with image regex")
        });
        CODE_SPAN_WITH_IMG_REGEX.is_match(content.trim())
    }

    fn find_markdown_image_violations(&self, content: &str) -> Vec<(usize, usize)> {
        let mut ranges = Vec::new();

        // Check inline images: ![alt](url)
        for captures in MARKDOWN_IMAGE_REGEX.captures_iter(content) {
            if let (Some(alt_text), Some(full_match)) = (captures.get(1), captures.get(0)) {
                if alt_text.as_str().is_empty() {
                    ranges.push((full_match.start(), full_match.end()));
                }
            }
        }

        // Check reference images: ![alt][ref]
        for captures in MARKDOWN_REFERENCE_IMAGE_REGEX.captures_iter(content) {
            if let (Some(alt_text), Some(full_match)) = (captures.get(1), captures.get(0)) {
                if alt_text.as_str().is_empty() {
                    ranges.push((full_match.start(), full_match.end()));
                }
            }
        }

        // Check shortcut reference images: ![alt][]
        for captures in MARKDOWN_REFERENCE_IMAGE_SHORTCUT_REGEX.captures_iter(content) {
            if let (Some(alt_text), Some(full_match)) = (captures.get(1), captures.get(0)) {
                if alt_text.as_str().is_empty() {
                    ranges.push((full_match.start(), full_match.end()));
                }
            }
        }

        ranges
    }

    fn find_html_image_violations(&self, content: &str) -> Vec<(usize, usize)> {
        let mut ranges = Vec::new();
        for img_match in IMG_TAG_REGEX.find_iter(content) {
            let img_tag = img_match.as_str();

            // Skip closing tags
            if img_tag.starts_with("</") {
                continue;
            }

            // Check for aria-hidden="true" first
            if let Some(aria_cap) = ARIA_HIDDEN_REGEX.captures(img_tag) {
                let value = aria_cap.get(1).or(aria_cap.get(2));
                if let Some(value_match) = value {
                    if value_match.as_str().to_lowercase() == "true" {
                        continue; // Skip images with aria-hidden="true"
                    }
                }
            }

            // Check for alt attribute with value
            let has_valid_alt = ALT_ATTRIBUTE_REGEX.captures(img_tag).is_some();

            if !has_valid_alt {
                ranges.push((img_match.start(), img_match.end()));
            }
        }
        ranges
    }

    fn add_violation(&mut self, node: &Node, start_offset: usize, end_offset: usize) {
        let start_byte = node.start_byte() + start_offset;
        let end_byte = node.start_byte() + end_offset;

        let (start_line, start_col) = self.byte_to_line_col(start_byte);
        let (end_line, end_col) = self.byte_to_line_col(end_byte);

        let range = range_from_tree_sitter(&tree_sitter::Range {
            start_byte,
            end_byte,
            start_point: tree_sitter::Point {
                row: start_line,
                column: start_col,
            },
            end_point: tree_sitter::Point {
                row: end_line,
                column: end_col,
            },
        });

        let violation = RuleViolation::new(
            &MD045,
            MD045.description.to_string(),
            self.context.file_path.clone(),
            range,
        );
        self.violations.push(violation);
    }

    fn byte_to_line_col(&self, byte_pos: usize) -> (usize, usize) {
        let line = match self.line_starts.binary_search(&byte_pos) {
            Ok(line) => line,
            Err(line) => line - 1,
        };
        let line_start = self.line_starts[line];
        let col = byte_pos - line_start;
        (line, col)
    }
}

pub const MD045: Rule = Rule {
    id: "MD045",
    alias: "no-alt-text",
    tags: &["accessibility", "images"],
    description: "Images should have alternate text (alt text)",
    rule_type: RuleType::Token,
    required_nodes: &["inline", "html_block"],
    new_linter: |context| Box::new(MD045Linter::new(context)),
};

impl RuleLinter for MD045Linter {
    fn feed(&mut self, node: &Node) {
        match node.kind() {
            "inline" | "html_block" => {
                if self.is_in_code_context(node) {
                    return;
                }

                let (markdown_ranges, html_ranges) = {
                    let document_content = self.context.document_content.borrow();
                    let content = &document_content[node.start_byte()..node.end_byte()];

                    if self.contains_inline_code_with_images(content) {
                        (vec![], vec![])
                    } else if node.kind() == "inline" {
                        (
                            self.find_markdown_image_violations(content),
                            self.find_html_image_violations(content),
                        )
                    } else {
                        // html_block
                        (vec![], self.find_html_image_violations(content))
                    }
                };

                for (start, end) in markdown_ranges {
                    self.add_violation(node, start, end);
                }

                for (start, end) in html_ranges {
                    self.add_violation(node, start, end);
                }
            }
            _ => {}
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::RuleSeverity;
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![
            ("no-alt-text", RuleSeverity::Error),
            ("no-inline-html", RuleSeverity::Off),
        ])
    }

    #[test]
    fn test_markdown_images_with_alt_text_no_violations() {
        let input = "# Test\n\n![Valid alt text](image.jpg)\n\n![Another valid image](image.jpg \"Title\")\n\n![Reference image with alt][ref]\n\nReference image with alt text ![Alt text reference][ref2]\n\n[ref]: image.jpg\n[ref2]: image.jpg \"Title\"\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md045_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD045")
            .collect();
        assert_eq!(md045_violations.len(), 0);
    }

    #[test]
    fn test_markdown_images_without_alt_text_violations() {
        let input = "# Test\n\n![](image.jpg)\n\n![](image.jpg \"Title\")\n\n![Empty alt](image.jpg) and ![](inline-image.jpg) in text\n\nReference image without alt ![][ref]\n\n[ref]: image.jpg\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md045_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD045")
            .collect();

        // Should find 4 violations:
        // Line 2: ![](image.jpg)
        // Line 4: ![](image.jpg "Title")
        // Line 6: ![](inline-image.jpg)
        // Line 8: ![][ref]
        assert_eq!(md045_violations.len(), 4);
    }

    #[test]
    fn test_html_images_with_alt_attribute_no_violations() {
        let input = "# Test\n\n<img src=\"image.jpg\" alt=\"Valid alt text\" />\n\n<img src=\"image.jpg\" alt=\"Another valid\" >\n\n<IMG SRC=\"image.jpg\" ALT=\"Case insensitive\" />\n\n<img \n  src=\"image.jpg\" \n  alt=\"Multi-line\" \n  />\n\n<img src=\"image.jpg\" alt=\"\" />\n\n<img src=\"image.jpg\" alt='' />\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md045_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD045")
            .collect();
        assert_eq!(md045_violations.len(), 0);
    }

    #[test]
    fn test_html_images_without_alt_attribute_violations() {
        let input = "# Test\n\n<img src=\"image.jpg\" />\n\n<img src=\"image.jpg\" alt>\n\n<IMG SRC=\"image.jpg\" />\n\n<img \n  src=\"image.jpg\" \n  title=\"Title only\" />\n\n<p><img src=\"nested.jpg\"></p>\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md045_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD045")
            .collect();

        // Should find 4 violations:
        // Line 2: <img src="image.jpg" />
        // Line 4: <img src="image.jpg" alt>
        // Line 6: <IMG SRC="image.jpg" />
        // Line 8-10: Multi-line img tag
        // Line 12: nested img tag
        assert_eq!(md045_violations.len(), 5);
    }

    #[test]
    fn test_html_images_with_aria_hidden_no_violations() {
        let input = "# Test\n\n<img src=\"image.jpg\" aria-hidden=\"true\" />\n\n<img src=\"image.jpg\" ARIA-HIDDEN=\"TRUE\" />\n\n<img \n  src=\"image.jpg\" \n  aria-hidden=\"true\"
  />\n\n<img src=\"image.jpg\" aria-hidden='true' />\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md045_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD045")
            .collect();
        assert_eq!(md045_violations.len(), 0);
    }

    #[test]
    fn test_html_images_with_aria_hidden_false_violations() {
        let input = "# Test\n\n<img src=\"image.jpg\" aria-hidden=\"false\" />\n\n<img src=\"image.jpg\" aria-hidden=\"\" />\n\n<img src=\"image.jpg\" aria-hidden=\"other\" />\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md045_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD045")
            .collect();

        // Should find 3 violations (aria-hidden != \"true\")
        assert_eq!(md045_violations.len(), 3);
    }

    #[test]
    fn test_mixed_image_types() {
        let input = "# Test\n\n![Valid alt](image.jpg)\n\n![](no-alt.jpg)\n\n<img src=\"valid.jpg\" alt=\"Valid\" />\n\n<img src=\"no-alt.jpg\" />\n\n<img src=\"hidden.jpg\" aria-hidden=\"true\" />\n\n![Reference valid][ref1]\n\n![][ref2]\n\n[ref1]: image.jpg\n[ref2]: image.jpg\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md045_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD045")
            .collect();

        // Should find 3 violations:
        // Line 4: ![](no-alt.jpg)
        // Line 8: <img src="no-alt.jpg" />
        // Line 14: ![][ref2]
        assert_eq!(md045_violations.len(), 3);
    }

    #[test]
    fn test_multiline_markdown_images() {
        let input = "# Test\n\n![Alt text](image.jpg 
\"Title\")\n\n![](image.jpg 
\"Title\")\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md045_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD045")
            .collect();

        // Should find 1 violation (the second image without alt text)
        assert_eq!(md045_violations.len(), 1);
    }

    #[test]
    fn test_images_in_links() {
        let input = "# Test\n\n[![Alt text](image.jpg)](link.html)\n\n[![](no-alt.jpg)](link.html)\n\n[<img src=\"alt.jpg\" alt=\"Alt\" />](link.html)\n\n[<img src=\"no-alt.jpg\" />](link.html)\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md045_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD045")
            .collect();

        // Should find 2 violations:
        // Line 4: [![](no-alt.jpg)](link.html) - markdown image without alt
        // Line 8: [<img src="no-alt.jpg" />](link.html) - HTML img without alt
        assert_eq!(md045_violations.len(), 2);
    }

    #[test]
    fn test_no_false_positives_in_code_blocks() {
        let input = "# Test\n\n```html\n![](image.jpg)\n<img src=\"image.jpg\" />\n```\n\n    ![](indented-code.jpg)\n    <img src=\"indented.jpg\" />\n\n`![](inline-code.jpg)` and `<img src=\"inline.jpg\" />`\n\nRegular text with ![](actual-image.jpg) should trigger.\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        let md045_violations: Vec<_> = violations
            .iter()
            .filter(|v| v.rule().id == "MD045")
            .collect();

        // Should only find 1 violation (the actual image outside code blocks)
        assert_eq!(md045_violations.len(), 1);
    }
}
