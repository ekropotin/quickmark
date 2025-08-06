use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

// Pre-compiled regex patterns for performance
static FULL_REFERENCE_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[([^\]]*)\]\[([^\]]*)\]").unwrap());

static COLLAPSED_REFERENCE_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[([^\]]+)\]\[\]").unwrap());

static SHORTCUT_REFERENCE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[([^\]]+)\]").unwrap());

static REFERENCE_DEFINITION_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\s*\[([^\]]+)\]:\s*").unwrap());

#[derive(Debug, Clone)]
struct ReferenceDefinition {
    label: String,
    range: tree_sitter::Range,
}

pub(crate) struct MD053Linter {
    context: Rc<Context>,
    definitions: HashMap<String, Vec<ReferenceDefinition>>, // Track multiple definitions per label
    references: HashSet<String>,                            // All referenced labels
}

impl MD053Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            definitions: HashMap::new(),
            references: HashSet::new(),
        }
    }

    fn normalize_reference(&self, label: &str) -> String {
        // Normalize reference labels according to CommonMark spec:
        // - Convert to lowercase
        // - Trim whitespace
        // - Collapse consecutive whitespace to single spaces
        let mut result = String::with_capacity(label.len());
        let mut prev_was_space = false;

        for ch in label.chars() {
            if ch.is_whitespace() {
                if !prev_was_space && !result.is_empty() {
                    result.push(' ');
                    prev_was_space = true;
                }
            } else {
                result.push(ch.to_lowercase().next().unwrap_or(ch));
                prev_was_space = false;
            }
        }

        // Remove trailing space if present
        if result.ends_with(' ') {
            result.pop();
        }

        result
    }

    fn extract_reference_definition(&self, node: &Node) -> Vec<ReferenceDefinition> {
        // Extract the label from reference definition nodes
        // [label]: url "title"
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let document_content = self.context.document_content.borrow();
        let content = &document_content[start_byte..end_byte];

        REFERENCE_DEFINITION_PATTERN
            .captures_iter(content)
            .filter_map(|cap| {
                cap.get(1).map(|label| {
                    let normalized_label = self.normalize_reference(label.as_str());
                    ReferenceDefinition {
                        label: normalized_label,
                        range: node.range(),
                    }
                })
            })
            .collect()
    }

    fn extract_reference_links(&self, node: &Node) -> Vec<String> {
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
                    links.push(self.normalize_reference(label_str));
                }
            }
        }

        // Collapsed reference: [label][]
        for cap in COLLAPSED_REFERENCE_PATTERN.captures_iter(content) {
            if let Some(label) = cap.get(1) {
                links.push(self.normalize_reference(label.as_str()));
            }
        }

        // Shortcut reference: [label] - check all potential shortcuts
        // We need to be careful not to double-count references that were already caught by full/collapsed patterns
        let mut shortcut_candidates = Vec::new();
        for cap in SHORTCUT_REFERENCE_PATTERN.captures_iter(content) {
            if let Some(label) = cap.get(1) {
                let full_match = cap.get(0).expect("regex match should have group 0");
                let start = full_match.start();
                let end = full_match.end();
                let remaining = &content[end..];

                // Check if this looks like a shortcut (not immediately followed by [] or [label])
                // We only reject if immediately followed by brackets, not if there's whitespace/newline first
                let immediately_followed_by_bracket = remaining.starts_with('[');
                if !immediately_followed_by_bracket {
                    shortcut_candidates.push((
                        start,
                        end,
                        self.normalize_reference(label.as_str()),
                    ));
                }
            }
        }

        // Filter out shortcut candidates that overlap with already found full/collapsed references
        // Use a HashSet for O(1) lookup performance
        let mut existing_labels: HashSet<String> = links.iter().cloned().collect();
        for (_start, _end, normalized_label) in shortcut_candidates {
            // Check if this shortcut overlaps with any full/collapsed reference we already found
            // For now, we'll use a simple heuristic: if we didn't find this as a full/collapsed reference,
            // and it's not followed by brackets, treat it as a shortcut
            if !existing_labels.contains(&normalized_label) {
                existing_labels.insert(normalized_label.clone());
                links.push(normalized_label);
            }
        }

        links
    }
}

impl RuleLinter for MD053Linter {
    fn feed(&mut self, node: &Node) {
        match node.kind() {
            // Handle reference definitions like [label]: url
            "link_reference_definition" => {
                let definitions = self.extract_reference_definition(node);
                for definition in definitions {
                    self.definitions
                        .entry(definition.label.clone())
                        .or_default()
                        .push(definition);
                }
            }
            // Handle paragraphs for reference links
            "paragraph" => {
                let links = self.extract_reference_links(node);
                for link in links {
                    self.references.insert(link);
                }
            }
            // Handle reference links [text][label], [label][], [label]
            "link" | "image" => {
                let links = self.extract_reference_links(node);
                for link in links {
                    self.references.insert(link);
                }
            }
            _ => {
                // Ignore other node types
            }
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        let mut violations = Vec::new();
        let config = &self
            .context
            .config
            .linters
            .settings
            .link_image_reference_definitions;
        let ignored_definitions: HashSet<String> = config
            .ignored_definitions
            .iter()
            .map(|label| self.normalize_reference(label))
            .collect();

        // Check for unused definitions and duplicates
        for (label, definitions) in &self.definitions {
            // Skip if label is in ignored list
            if ignored_definitions.contains(label) {
                continue;
            }

            // Check if definition is unused (no references to it)
            let is_unused = !self.references.contains(label);

            if definitions.len() > 1 {
                // Handle duplicate definitions
                if is_unused {
                    // If unused, report the first definition as unused
                    let first_def = &definitions[0];
                    violations.push(RuleViolation::new(
                        &MD053,
                        format!(
                            "Unused link or image reference definition: \"{}\"",
                            first_def.label
                        ),
                        self.context.file_path.clone(),
                        range_from_tree_sitter(&first_def.range),
                    ));
                }
                // Report all subsequent definitions as duplicates (first definition wins per CommonMark)
                for definition in &definitions[1..] {
                    violations.push(RuleViolation::new(
                        &MD053,
                        format!(
                            "Duplicate link or image reference definition: \"{}\"",
                            definition.label
                        ),
                        self.context.file_path.clone(),
                        range_from_tree_sitter(&definition.range),
                    ));
                }
            } else if is_unused {
                // Single definition that is unused
                let def = &definitions[0];
                violations.push(RuleViolation::new(
                    &MD053,
                    format!(
                        "Unused link or image reference definition: \"{}\"",
                        def.label
                    ),
                    self.context.file_path.clone(),
                    range_from_tree_sitter(&def.range),
                ));
            }
        }

        violations
    }
}

pub const MD053: Rule = Rule {
    id: "MD053",
    alias: "link-image-reference-definitions",
    tags: &["links", "images"],
    description: "Link and image reference definitions should be needed",
    rule_type: RuleType::Document,
    required_nodes: &["link", "image", "paragraph", "link_reference_definition"],
    new_linter: |context| Box::new(MD053Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{
        LintersSettingsTable, MD053LinkImageReferenceDefinitionsTable, RuleSeverity,
    };
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![(
            "link-image-reference-definitions",
            RuleSeverity::Error,
        )])
    }

    fn test_config_with_ignored_definitions(
        ignored_definitions: Vec<String>,
    ) -> crate::config::QuickmarkConfig {
        crate::test_utils::test_helpers::test_config_with_settings(
            vec![("link-image-reference-definitions", RuleSeverity::Error)],
            LintersSettingsTable {
                link_image_reference_definitions: MD053LinkImageReferenceDefinitionsTable {
                    ignored_definitions,
                },
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_unused_definition_basic() {
        let input = "[unused]: https://example.com

Some text.
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have 1 violation - unused reference definition
        assert_eq!(1, violations.len());
        assert!(violations[0]
            .message()
            .contains("Unused link or image reference definition: \"unused\""));
    }

    #[test]
    fn test_used_definition_basic() {
        let input = "[label]: https://example.com

[Good link][label]
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - definition is used
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_duplicate_definitions() {
        let input = "[label]: https://example.com/1
[label]: https://example.com/2

[Good link][label]
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have 1 violation - duplicate definition (second one)
        assert_eq!(1, violations.len());
        assert!(violations[0]
            .message()
            .contains("Duplicate link or image reference definition: \"label\""));
    }

    #[test]
    fn test_unused_and_duplicate() {
        let input = "[unused1]: https://example.com/1
[unused2]: https://example.com/2
[duplicate]: https://example.com/3
[duplicate]: https://example.com/4

Some text.
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have 4 violations: 2 unused + 1 duplicate + 1 unused (both duplicates are unused)
        assert_eq!(4, violations.len());

        // Check violation types
        let messages: Vec<&str> = violations.iter().map(|v| v.message()).collect();
        let unused_count = messages.iter().filter(|m| m.contains("Unused")).count();
        let duplicate_count = messages.iter().filter(|m| m.contains("Duplicate")).count();

        assert_eq!(3, unused_count); // unused1, unused2, and both duplicate entries are unused
        assert_eq!(1, duplicate_count); // second duplicate entry
    }

    #[test]
    fn test_collapsed_reference_format() {
        let input = "[label]: https://example.com

[label][]
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - collapsed reference is used
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_shortcut_reference_format() {
        let input = "[label]: https://example.com

[label]
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - shortcut reference is used
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_image_references() {
        let input = "[image]: https://example.com/image.png
[unused-image]: https://example.com/unused.png

![Alt text][image]
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have 1 violation - unused image reference
        assert_eq!(1, violations.len());
        assert!(violations[0]
            .message()
            .contains("Unused link or image reference definition: \"unused-image\""));
    }

    #[test]
    fn test_case_insensitive_matching() {
        let input = "[Label]: https://example.com

[Good link][LABEL]
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - case insensitive matching per CommonMark
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_whitespace_normalization() {
        let input = "[  label   with   spaces  ]: https://example.com

[Good link][label with spaces]
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - whitespace is normalized per CommonMark
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_ignored_definitions_default() {
        let input = "[//]: # (This is a comment)
[unused]: https://example.com

Some text.
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have 1 violation - '//' is ignored by default, but 'unused' is not
        assert_eq!(1, violations.len());
        assert!(violations[0]
            .message()
            .contains("Unused link or image reference definition: \"unused\""));
    }

    #[test]
    fn test_custom_ignored_definitions() {
        let input = "[custom]: https://example.com
[another]: https://example.com
[regular]: https://example.com

[Good link][regular]
";

        let config =
            test_config_with_ignored_definitions(vec!["custom".to_string(), "another".to_string()]);
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - custom and another are ignored, regular is used
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_mixed_scenarios_comprehensive() {
        let input = "[used-full]: https://example.com/1
[used-collapsed]: https://example.com/2
[used-shortcut]: https://example.com/3
[unused]: https://example.com/4
[duplicate-used]: https://example.com/5
[duplicate-used]: https://example.com/6
[duplicate-unused]: https://example.com/7
[duplicate-unused]: https://example.com/8
[//]: # (Ignored comment)

[Link 1][used-full]
[used-collapsed][]
[used-shortcut]
[Link 2][duplicate-used]
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Expected violations:
        // - unused: unused
        // - duplicate-used (second): duplicate
        // - duplicate-unused (first): unused
        // - duplicate-unused (second): duplicate
        assert_eq!(4, violations.len());

        let messages: Vec<&str> = violations.iter().map(|v| v.message()).collect();
        let unused_count = messages.iter().filter(|m| m.contains("Unused")).count();
        let duplicate_count = messages.iter().filter(|m| m.contains("Duplicate")).count();

        assert_eq!(2, unused_count); // unused + duplicate_unused (first)
        assert_eq!(2, duplicate_count); // duplicate-used (second) + duplicate-unused (second)
    }

    #[test]
    fn test_inline_links_ignored() {
        let input = "[unused]: https://example.com

[Inline link](https://example.com) and [another](https://example.com).
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have 1 violation - unused definition, inline links don't count as references
        assert_eq!(1, violations.len());
        assert!(violations[0]
            .message()
            .contains("Unused link or image reference definition: \"unused\""));
    }
}
