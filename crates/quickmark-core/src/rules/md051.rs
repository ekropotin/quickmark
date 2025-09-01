use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashSet;
use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

// MD051-specific configuration types
#[derive(Debug, PartialEq, Clone, Deserialize, Default)]
pub struct MD051LinkFragmentsTable {
    #[serde(default)]
    pub ignore_case: bool,
    #[serde(default)]
    pub ignored_pattern: String,
}

#[derive(Debug, Clone)]
struct LinkFragment {
    fragment: String,
    range: tree_sitter::Range,
}

// GitHub line fragment regex matching:
// ^#(?:L\d+(?:C\d+)?-L\d+(?:C\d+)?|L\d+)$
// This allows: L123, L12C5-L34C10. It also matches L12-L34.
static LINE_FRAGMENT_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^L\d+(?:C\d+)?-L\d+(?:C\d+)?$|^L\d+$").unwrap());

static ID_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r#"id\s*=\s*["']([^"']+)["']"#).unwrap());

static NAME_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"name\s*=\s*["']([^"']+)["']"#).unwrap());

pub(crate) struct MD051Linter {
    context: Rc<Context>,
    valid_fragments: HashSet<String>,
    valid_fragments_lowercase: HashSet<String>, // Pre-computed lowercase for case-insensitive lookups
    link_fragments: Vec<LinkFragment>,
}

impl MD051Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            valid_fragments: HashSet::new(),
            valid_fragments_lowercase: HashSet::new(),
            link_fragments: Vec::new(),
        }
    }

    fn extract_heading_text(&self, node: &Node) -> Option<String> {
        let inline_node = if node.kind() == "atx_heading" {
            node.children(&mut node.walk())
                .find(|c| c.kind() == "inline")
        } else if node.kind() == "setext_heading" {
            node.children(&mut node.walk())
                .find(|c| c.kind() == "paragraph")
                .and_then(|p| {
                    p.children(&mut p.walk())
                        .find(|gc| gc.kind() == "inline")
                })
        } else {
            None
        };

        inline_node.map(|n| {
            let document_content = self.context.document_content.borrow();
            document_content[n.start_byte()..n.end_byte()]
                .trim()
                .to_string()
        })
    }

    fn generate_github_fragment(&self, heading_text: &str) -> String {
        // GitHub fragment generation rules based on reverse engineering:
        // 1. Convert to lowercase
        // 2. Replace spaces with hyphens
        // 3. Keep only alphanumeric, hyphens, and underscores
        // 4. Remove leading/trailing hyphens
        let lower = heading_text.trim().to_lowercase().replace(' ', "-");
        let mut fragment = String::with_capacity(lower.len());

        for c in lower.chars() {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                fragment.push(c);
            }
        }

        // Remove leading and trailing hyphens
        fragment.trim_matches('-').to_string()
    }

    fn extract_custom_anchor(&self, heading_text: &str) -> Option<String> {
        // Look for {#custom-anchor} syntax
        if let Some(start) = heading_text.rfind("{#") {
            if let Some(end) = heading_text[start..].find('}') {
                let anchor = &heading_text[start + 2..start + end];
                return Some(anchor.to_string());
            }
        }
        None
    }

    fn extract_link_fragments(&self, node: &Node) -> Vec<LinkFragment> {
        // Extract link fragments using tree-sitter's native link structure
        // Look for pattern: [ text ] ( URL ) where URL contains #fragment
        let mut link_fragments = Vec::new();

        // Traverse child nodes looking for link patterns
        let mut i = 0;
        while i < node.child_count() {
            if let Some(child) = node.child(i) {
                if child.kind() == "[" {
                    // Found potential link start, look for complete link pattern
                    if let Some((fragment_info, end_index)) = self.parse_link_at_position(node, i) {
                        link_fragments.push(fragment_info);
                        i = end_index;
                    }
                }
                i += 1;
            }
        }

        link_fragments
    }

    fn parse_link_at_position(&self, parent: &Node, start_idx: usize) -> Option<(LinkFragment, usize)> {
        // Parse link pattern: [ text ] ( URL# fragment )
        let document_content = self.context.document_content.borrow();

        // Look for the sequence: [ ... ] ( ... )
        let mut bracket_close_idx = None;
        let mut paren_open_idx = None;
        let mut paren_close_idx = None;

        // Find ] after [
        for i in start_idx + 1..parent.child_count() {
            if let Some(child) = parent.child(i) {
                if child.kind() == "]" {
                    bracket_close_idx = Some(i);
                    break;
                }
            }
        }

        if let Some(bracket_close) = bracket_close_idx {
            // Find ( after ]
            for i in bracket_close + 1..parent.child_count() {
                if let Some(child) = parent.child(i) {
                    if child.kind() == "(" {
                        paren_open_idx = Some(i);
                        break;
                    }
                }
            }
        }

        if let Some(paren_open) = paren_open_idx {
            // Find ) after (
            for i in paren_open + 1..parent.child_count() {
                if let Some(child) = parent.child(i) {
                    if child.kind() == ")" {
                        paren_close_idx = Some(i);
                        break;
                    }
                }
            }
        }

        // If we found a complete link pattern
        if let (Some(paren_open), Some(paren_close)) = (paren_open_idx, paren_close_idx) {
            // Extract URL content between ( and ) by getting the text span
            if let (Some(paren_open_node), Some(paren_close_node)) =
                (parent.child(paren_open), parent.child(paren_close))
            {
                let start_byte = paren_open_node.end_byte(); // After the (
                let end_byte = paren_close_node.start_byte(); // Before the )
                let url_parts = &document_content[start_byte..end_byte];
                // Only process internal fragments (URLs starting with #)
                if url_parts.starts_with('#') {
                    if let Some(hash_pos) = url_parts.rfind('#') {
                        let fragment = &url_parts[hash_pos + 1..];
                        // Only process non-empty fragments that don't contain spaces
                        if !fragment.is_empty() && !fragment.contains(' ') {
                            // Get the range of the entire link for position reporting
                            if let (Some(start_node), Some(end_node)) =
                                (parent.child(start_idx), parent.child(paren_close))
                            {
                                let link_range = tree_sitter::Range {
                                    start_byte: start_node.start_byte(),
                                    end_byte: end_node.end_byte(),
                                    start_point: start_node.range().start_point,
                                    end_point: end_node.range().end_point,
                                };

                                return Some((
                                    LinkFragment {
                                        fragment: fragment.to_string(),
                                        range: link_range,
                                    },
                                    paren_close,
                                ));
                            }
                        }
                    }
                }
            }
        }

        None
    }

    fn is_github_special_fragment(&self, fragment: &str) -> bool {
        // GitHub special fragments according to GitHub specification
        // Reference: https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/creating-a-permanent-link-to-a-code-snippet

        if fragment == "top" {
            return true;
        }

        if LINE_FRAGMENT_PATTERN.is_match(fragment) {
            return true;
        }

        false
    }

    fn extract_html_id_or_name(&self, node: &Node) -> Vec<String> {
        // Extract id and name attributes from HTML elements
        let mut ids = Vec::new();
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let document_content = self.context.document_content.borrow();
        let html_content = &document_content[start_byte..end_byte];

        for cap in ID_PATTERN.captures_iter(html_content) {
            if let Some(id) = cap.get(1) {
                ids.push(id.as_str().to_string());
            }
        }

        for cap in NAME_PATTERN.captures_iter(html_content) {
            if let Some(name) = cap.get(1) {
                ids.push(name.as_str().to_string());
            }
        }

        ids
    }
}

impl RuleLinter for MD051Linter {
    fn feed(&mut self, node: &Node) {
        match node.kind() {
            "atx_heading" | "setext_heading" => {
                if let Some(heading_text) = self.extract_heading_text(node) {
                    // Check for custom anchor first
                    if let Some(custom_anchor) = self.extract_custom_anchor(&heading_text) {
                        self.valid_fragments.insert(custom_anchor.clone());
                        self.valid_fragments_lowercase
                            .insert(custom_anchor.to_lowercase());
                        // Also generate the default fragment from the heading text without the anchor
                        let clean_text = heading_text
                            .replace(&format!("{{#{custom_anchor}}}"), "")
                            .trim()
                            .to_string();
                        if !clean_text.is_empty() {
                            let fragment = self.generate_github_fragment(&clean_text);
                            if !fragment.is_empty() {
                                self.valid_fragments.insert(fragment.clone());
                                self.valid_fragments_lowercase
                                    .insert(fragment.to_lowercase());
                            }
                        }
                    } else {
                        // Generate GitHub-style fragment
                        let fragment = self.generate_github_fragment(&heading_text);
                        if !fragment.is_empty() {
                            // Handle duplicate headings by checking if fragment already exists
                            let mut unique_fragment = fragment.clone();
                            let mut counter = 1;
                            while self.valid_fragments.contains(&unique_fragment) {
                                unique_fragment = format!("{fragment}-{counter}");
                                counter += 1;
                            }
                            self.valid_fragments.insert(unique_fragment.clone());
                            self.valid_fragments_lowercase
                                .insert(unique_fragment.to_lowercase());
                        }
                    }
                }
            }
            "inline" | "html_block" => {
                // Extract HTML id and name attributes
                let ids = self.extract_html_id_or_name(node);
                for id in ids {
                    self.valid_fragments.insert(id.clone());
                    self.valid_fragments_lowercase.insert(id.to_lowercase());
                }

                // Also look for links in inline content
                let link_fragments = self.extract_link_fragments(node);
                for link_fragment in link_fragments {
                    self.link_fragments.push(link_fragment);
                }
            }
            _ => {
                // For other nodes, do nothing to avoid duplicates
            }
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        let mut violations = Vec::new();
        let config = &self.context.config.linters.settings.link_fragments;

        // Compile ignored pattern regex if provided
        let ignored_regex = if !config.ignored_pattern.is_empty() {
            Regex::new(&config.ignored_pattern).ok()
        } else {
            None
        };

        for link_fragment in &self.link_fragments {
            let fragment = &link_fragment.fragment;
            let mut is_valid = false;

            // Check if it's a GitHub special fragment
            if self.is_github_special_fragment(fragment) {
                is_valid = true;
            }

            // Check if it matches ignored pattern
            if !is_valid {
                if let Some(ref regex) = ignored_regex {
                    if regex.is_match(fragment) {
                        is_valid = true;
                    }
                }
            }

            // Check if it matches any valid fragment
            if !is_valid {
                if config.ignore_case {
                    let fragment_lower = fragment.to_lowercase();
                    is_valid = self.valid_fragments_lowercase.contains(&fragment_lower);
                } else {
                    is_valid = self.valid_fragments.contains(fragment);
                }
            }

            if !is_valid {
                violations.push(RuleViolation::new(
                    &MD051,
                    format!("Link fragment '{fragment}' does not match any heading or anchor in the document"),
                    self.context.file_path.clone(),
                    range_from_tree_sitter(&link_fragment.range),
                ));
            }
        }

        violations
    }
}

pub const MD051: Rule = Rule {
    id: "MD051",
    alias: "link-fragments",
    tags: &["links"],
    description: "Link fragments should be valid",
    rule_type: RuleType::Document,
    required_nodes: &["link", "atx_heading", "setext_heading"],
    new_linter: |context| Box::new(MD051Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{LintersSettingsTable, MD051LinkFragmentsTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_rules;

    fn test_config() -> crate::config::QuickmarkConfig {
        test_config_with_rules(vec![("link-fragments", RuleSeverity::Error)])
    }

    fn test_config_with_settings(
        ignore_case: bool,
        ignored_pattern: String,
    ) -> crate::config::QuickmarkConfig {
        crate::test_utils::test_helpers::test_config_with_settings(
            vec![("link-fragments", RuleSeverity::Error)],
            LintersSettingsTable {
                link_fragments: MD051LinkFragmentsTable {
                    ignore_case,
                    ignored_pattern,
                },
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_basic_valid_fragment() {
        let input = "# Test Heading

[Valid Link](#test-heading)
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - valid fragment
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_basic_invalid_fragment() {
        let input = "# Test Heading

[Invalid Link](#nonexistent-heading)
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have 1 violation - invalid fragment
        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_case_sensitive_default() {
        let input = "# Test Heading

[Invalid Link](#Test-Heading)
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have 1 violation - case mismatch
        assert_eq!(1, violations.len());
    }

    #[test]
    fn test_ignore_case_option() {
        let input = "# Test Heading

[Valid Link](#Test-Heading)
";

        let config = test_config_with_settings(true, String::new());
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - case ignored
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_punctuation_removal() {
        let input = "# Test: Heading! With? Punctuation.

[Valid Link](#test-heading-with-punctuation)
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - punctuation correctly removed
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_duplicate_headings() {
        let input = "# Test Heading

## Test Heading

[Link 1](#test-heading)
[Link 2](#test-heading-1)
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - both fragments are valid
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_custom_anchor() {
        let input = "# Test Heading {#custom-anchor}

[Valid Link](#custom-anchor)
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - custom anchor is valid
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_html_id_attribute() {
        let input = "# Test Heading\n\n<div id=\"my-custom-id\">Content</div>\n\n[Valid Link](#my-custom-id)\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - HTML id is valid
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_html_name_attribute() {
        let input = "# Test Heading\n\n<a name=\"my-anchor\">Anchor</a>\n\n[Valid Link](#my-anchor)\n";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - HTML name is valid
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_ignored_pattern() {
        let input = "# Test Heading

[Link to external](#external-fragment)
";

        let config = test_config_with_settings(false, "external-.*".to_string());
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - fragment matches ignored pattern
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_github_special_fragments() {
        let input = "# Test Heading

[Link to top](#top)
[Link to line](#L20)
[Link to range](#L19C5-L21C11)
[Invalid range](#L10-L20)
[Actually invalid](#L)
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have 1 violation - #L is invalid, but L10-L20 is ignored
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Link fragment 'L'"));
    }

    #[test]
    fn test_multiple_violations() {
        let input = "# Valid Heading

[Valid Link](#valid-heading)
[Invalid Link 1](#invalid-one)
[Invalid Link 2](#invalid-two)
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have 2 violations - two invalid fragments
        assert_eq!(2, violations.len());
    }

    #[test]
    fn test_setext_headings() {
        let input = "Test Heading
============

Another Heading
---------------

[Valid Link 1](#test-heading)
[Valid Link 2](#another-heading)
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have no violations - both setext headings are valid
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_edge_cases_for_consistency() {
        let input = "# Test Heading

[Valid link](#test-heading)
[Fragment with spaces](#test heading)
[Empty fragment](#)
[Invalid single L](#L)
[Valid L with number](#L123)
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Should have 1 violation - only #L should be reported
        // Fragments with spaces and empty fragments are ignored
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Link fragment 'L'"));
    }

    #[test]
    fn test_comprehensive() {
        let input = r#"# Test MD051 Comprehensive

This file tests various MD051 features and configuration options.

## Basic Headings

### Test Heading One

### Test Heading Two

## Case Sensitivity Tests

[Valid lowercase link](#test-heading-one)
[Invalid uppercase link](#test-heading-one)
[Mixed case invalid](#test-heading-two)

## Custom Anchors

### Heading with Custom Anchor {#custom-test-anchor}

[Valid custom anchor link](#custom-test-anchor)
[Invalid custom anchor link](#wrong-custom-anchor)

## Punctuation in Headings

### Heading: With? Special! Characters

[Valid punctuation link](#heading-with-special-characters)
[Invalid punctuation link](#heading-with-special-characters!)

## HTML Elements

<div id="test-html-id">HTML content</div>
<a name="test-html-name">Named anchor</a>

[Valid HTML id link](#test-html-id)
[Valid HTML name link](#test-html-name)
[Invalid HTML link](#wrong-html-id)

## GitHub Special Cases

[Valid top link](#top)
[Valid line link](#L123)
[Valid range link](#L10C1-L20C5)
[Invalid line format](#L)
[Invalid range format](#L10-L20)

## Setext Headings

First Setext Heading
====================

Second Setext Heading
---------------------

[Valid setext h1 link](#first-setext-heading)
[Valid setext h2 link](#second-setext-heading)
[Invalid setext link](#wrong-setext-heading)

## Duplicate Headings

### Duplicate Name

### Duplicate Name

[Link to first duplicate](#duplicate-name)
[Link to second duplicate](#duplicate-name-1)
[Invalid duplicate link](#duplicate-name-2)

## Multiple Links in Same Paragraph

This paragraph has [valid link](#test-heading-one) and [invalid link](#nonexistent) and [another valid](#custom-test-anchor).

## Edge Cases

[Empty fragment link](#)
[Fragment with spaces](#test heading one)
[Fragment with underscores](#test_heading_one)
[Fragment with numbers](#test-heading-123)

### Should not trigger

[Fragment with external link](https://developer.hashicorp.com/vault/api-docs/auth/jwt#default_role)
[Fragment with relative link](../../project/issues/managing_issues.md#add-an-issue-to-an-iteration-starter)
"#;

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();

        // Expected violations:
        // 1. Line 22: wrong-custom-anchor
        // 2. Line 29: heading-with-special-characters!
        // 3. Line 38: wrong-html-id
        // 4. Line 45: L
        // 5. Line 58: wrong-setext-heading
        // 6. Line 68: duplicate-name-2
        // 7. Line 72: nonexistent (in middle of line at position 56)
        // 8. Line 78: test_heading_one
        // 9. Line 79: test-heading-123

        assert_eq!(9, violations.len(), "Expected exactly 9 violations");

        // Specific checks for parity issues:
        let violation_messages: Vec<String> =
            violations.iter().map(|v| v.message().to_string()).collect();

        // Should NOT contain violations for external links
        assert!(
            !violation_messages
                .iter()
                .any(|msg| msg.contains("default_role")),
            "Should not report violations for external links"
        );
        assert!(
            !violation_messages
                .iter()
                .any(|msg| msg.contains("add-an-issue-to-an-iteration-starter")),
            "Should not report violations for relative links with external fragments"
        );

        // Should contain the expected invalid fragments
        assert!(violation_messages
            .iter()
            .any(|msg| msg.contains("wrong-custom-anchor")));
        assert!(violation_messages
            .iter()
            .any(|msg| msg.contains("heading-with-special-characters!")));
        assert!(violation_messages
            .iter()
            .any(|msg| msg.contains("wrong-html-id")));
        assert!(violation_messages.iter().any(|msg| msg.contains("'L'")));
        assert!(violation_messages
            .iter()
            .any(|msg| msg.contains("wrong-setext-heading")));
        assert!(violation_messages
            .iter()
            .any(|msg| msg.contains("duplicate-name-2")));
        assert!(violation_messages
            .iter()
            .any(|msg| msg.contains("nonexistent")));
        assert!(violation_messages
            .iter()
            .any(|msg| msg.contains("test_heading_one")));
        assert!(violation_messages
            .iter()
            .any(|msg| msg.contains("test-heading-123")));
    }

    #[test]
    fn test_colons() {
        let input = "
## `header:with:colons_in_it`

[should be ok](#headerwithcolons_in_it)
";

        let config = test_config();
        let mut multi_linter =
            MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = multi_linter.analyze();

        // Should have no violations - colons should be removed per GitHub spec
        assert_eq!(0, violations.len());
    }
}