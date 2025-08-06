use std::collections::HashSet;
use std::rc::Rc;
use tree_sitter::Node;
use regex::Regex;
use once_cell::sync::Lazy;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

// Pre-compiled regex patterns for performance
static FULL_REFERENCE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\[([^\]]*)\]\[([^\]]*)\]").unwrap()
});

static COLLAPSED_REFERENCE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\[([^\]]+)\]\[\]").unwrap()
});

static SHORTCUT_REFERENCE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\[([^\]]+)\]").unwrap()
});

static REFERENCE_DEFINITION_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^\s*\[([^\]]+)\]:\s*").unwrap()
});

#[derive(Debug, Clone)]
struct ReferenceLink {
    label: String,
    range: tree_sitter::Range,
    is_shortcut: bool,
}

pub(crate) struct MD052Linter {
    context: Rc<Context>,
    definitions: HashSet<String>,
    references: Vec<ReferenceLink>,
}

impl MD052Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            definitions: HashSet::new(),
            references: Vec::new(),
        }
    }

    fn normalize_reference(&self, label: &str) -> String {
        // Normalize reference labels according to CommonMark spec:
        // - Convert to lowercase
        // - Trim whitespace
        // - Collapse consecutive whitespace to single spaces
        label.to_lowercase().split_whitespace().collect::<Vec<_>>().join(" ")
    }

    fn extract_reference_definition(&self, node: &Node) -> Vec<String> {
        // Extract the label from reference definition nodes
        // [label]: url "title"
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let document_content = self.context.document_content.borrow();
        let content = &document_content[start_byte..end_byte];
        
        let mut definitions = Vec::new();
        for cap in REFERENCE_DEFINITION_PATTERN.captures_iter(content) {
            if let Some(label) = cap.get(1) {
                definitions.push(self.normalize_reference(label.as_str()));
            }
        }
        definitions
    }

    fn extract_reference_links(&self, node: &Node) -> Vec<(String, bool)> {
        // Extract reference links in different formats:
        // Full: [text][label]
        // Collapsed: [label][]  
        // Shortcut: [label]
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let document_content = self.context.document_content.borrow();
        let content = &document_content[start_byte..end_byte];

        let mut links = Vec::new();

        // Check for inline links first (contain parentheses - not reference links)
        if content.contains('(') && content.contains(')') {
            return links; // This is an inline link, not a reference link
        }

        // Full reference: [text][label]
        for cap in FULL_REFERENCE_PATTERN.captures_iter(content) {
            if let Some(label) = cap.get(2) {
                let label_str = label.as_str();
                if !label_str.is_empty() {
                    links.push((self.normalize_reference(label_str), false));
                }
            }
        }

        // Collapsed reference: [label][]
        for cap in COLLAPSED_REFERENCE_PATTERN.captures_iter(content) {
            if let Some(label) = cap.get(1) {
                links.push((self.normalize_reference(label.as_str()), false));
            }
        }

        // Shortcut reference: [label] (only if not caught by other patterns and not inline links)
        if links.is_empty() {
            for cap in SHORTCUT_REFERENCE_PATTERN.captures_iter(content) {
                if let Some(label) = cap.get(1) {
                    // Only consider it a shortcut if it doesn't look like a full/collapsed reference
                    // and there's no second bracket pair after this one
                    let match_end = cap.get(0).unwrap().end();
                    let remaining = &content[match_end..];
                    if !remaining.trim_start().starts_with('[') {
                        links.push((self.normalize_reference(label.as_str()), true));
                    }
                }
            }
        }

        links
    }
}

impl RuleLinter for MD052Linter {
    fn feed(&mut self, node: &Node) {
        match node.kind() {
            // Handle reference definitions like [label]: url
            "paragraph" => {
                let definitions = self.extract_reference_definition(node);
                for definition in definitions {
                    self.definitions.insert(definition);
                }
                
                // Also check for reference links in paragraphs
                let links = self.extract_reference_links(node);
                for (label, is_shortcut) in links {
                    self.references.push(ReferenceLink {
                        label,
                        range: node.range(),
                        is_shortcut,
                    });
                }
            }
            // Handle reference links [text][label], [label][], [label]  
            "link" | "image" => {
                let links = self.extract_reference_links(node);
                for (label, is_shortcut) in links {
                    self.references.push(ReferenceLink {
                        label,
                        range: node.range(),
                        is_shortcut,
                    });
                }
            }
            _ => {
                // Check all other node types for reference definitions
                let definitions = self.extract_reference_definition(node);
                for definition in definitions {
                    self.definitions.insert(definition);
                }
            }
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        let mut violations = Vec::new();
        let config = &self.context.config.linters.settings.reference_links_images;
        let ignored_labels: HashSet<String> = config.ignored_labels.iter()
            .map(|label| self.normalize_reference(label))
            .collect();

        for reference in &self.references {
            // Skip shortcut syntax unless explicitly enabled
            if reference.is_shortcut && !config.shortcut_syntax {
                continue;
            }

            let normalized_label = self.normalize_reference(&reference.label);
            
            // Skip if label is in ignored list
            if ignored_labels.contains(&normalized_label) {
                continue;
            }

            // Check if definition exists
            if !self.definitions.contains(&normalized_label) {
                violations.push(RuleViolation::new(
                    &MD052,
                    format!("Missing link or image reference definition: \"{}\"", reference.label),
                    self.context.file_path.clone(),
                    range_from_tree_sitter(&reference.range),
                ));
            }
        }

        violations
    }
}

pub const MD052: Rule = Rule {
    id: "MD052",
    alias: "reference-links-images",
    tags: &["links", "images"],
    description: "Reference links and images should use a label that is defined",
    rule_type: RuleType::Document,
    required_nodes: &["link", "image", "paragraph"],
    new_linter: |context| Box::new(MD052Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{LintersSettingsTable, MD052ReferenceLinksImagesTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![("reference-links-images", RuleSeverity::Error)])
    }

    fn test_config_with_settings(shortcut_syntax: bool, ignored_labels: Vec<String>) -> crate::config::QuickmarkConfig {
        crate::test_utils::test_helpers::test_config_with_settings(
            vec![("reference-links-images", RuleSeverity::Error)],
            LintersSettingsTable {
                reference_links_images: MD052ReferenceLinksImagesTable {
                    shortcut_syntax,
                    ignored_labels,
                },
                ..Default::default()
            },
        )
    }


    #[test]
    fn test_valid_full_reference() {
        let input = "[Good link][label]

[label]: https://example.com
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have no violations - valid reference
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_invalid_full_reference() {
        let input = "[Bad link][missing]

[label]: https://example.com
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have 1 violation - missing reference definition
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Missing link or image reference definition: \"missing\""));
    }

    #[test]
    fn test_valid_collapsed_reference() {
        let input = "[label][]

[label]: https://example.com
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have no violations - valid collapsed reference
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_invalid_collapsed_reference() {
        let input = "[missing][]

[label]: https://example.com
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have 1 violation - missing reference definition
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Missing link or image reference definition: \"missing\""));
    }

    #[test]
    fn test_shortcut_syntax_disabled_by_default() {
        let input = "[undefined]

[label]: https://example.com
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have no violations - shortcut syntax ignored by default
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_shortcut_syntax_enabled() {
        let input = "[undefined]

[label]: https://example.com
";

        let config = test_config_with_settings(true, vec!["x".to_string()]);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have 1 violation - shortcut syntax enabled and undefined
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Missing link or image reference definition: \"undefined\""));
    }

    #[test]
    fn test_valid_shortcut_syntax_enabled() {
        let input = "[label]

[label]: https://example.com
";

        let config = test_config_with_settings(true, vec!["x".to_string()]);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have no violations - shortcut syntax enabled and defined
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_ignored_labels_default_x() {
        let input = "[x] Task item

[label]: https://example.com
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have no violations - 'x' is ignored by default (GitHub task list)
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_custom_ignored_labels() {
        let input = "[custom] Some text
[another] More text

[label]: https://example.com
";

        let config = test_config_with_settings(true, vec!["custom".to_string(), "another".to_string()]);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have no violations - custom labels are ignored
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_case_insensitive_matching() {
        let input = "[Good Link][LABEL]

[label]: https://example.com
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have no violations - case insensitive matching per CommonMark
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_whitespace_normalization() {
        let input = "[Good Link][  label   with   spaces  ]

[label with spaces]: https://example.com
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have no violations - whitespace is normalized per CommonMark
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_images_full_reference() {
        let input = "![Alt text][image]

[image]: https://example.com/image.png
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have no violations - valid image reference
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_images_invalid_reference() {
        let input = "![Alt text][missing]

[image]: https://example.com/image.png
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have 1 violation - missing image reference definition
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Missing link or image reference definition: \"missing\""));
    }

    #[test]
    fn test_multiple_violations() {
        let input = "[Bad link][missing1]
[Another bad][missing2]
[Good link][valid]

[valid]: https://example.com
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have 2 violations - two missing reference definitions
        assert_eq!(2, violations.len());
    }

    #[test]
    fn test_mixed_link_types() {
        let input = "[Full][label1]
[Collapsed][]
[Shortcut]
![Image][image1]
![Collapsed image][]

[label1]: https://example.com/1
[collapsed]: https://example.com/2
[shortcut]: https://example.com/3
[image1]: https://example.com/image1.png
[collapsed image]: https://example.com/image2.png
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have no violations - all references defined (shortcut ignored by default)
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_duplicate_definitions() {
        let input = "[Good link][label]

[label]: https://example.com/1
[label]: https://example.com/2
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have no violations - first definition wins per CommonMark spec
        assert_eq!(0, violations.len());
    }
}