use std::collections::HashSet;
use std::rc::Rc;
use regex::Regex;
use once_cell::sync::Lazy;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, RuleViolation},
    rules::{Context, Rule, RuleLinter, RuleType},
};

#[derive(Debug, Clone)]
struct LinkFragment {
    fragment: String,
    node: Node<'static>,
}

// Pre-compiled regex patterns for performance
static LINK_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\[([^\]]*)\]\(([^)]*#[^)]*)\)").unwrap()
});

static RANGE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^L\d+C\d+-L\d+C\d+$").unwrap()
});

static ID_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"id\s*=\s*["']([^"']+)["']"#).unwrap()
});

static NAME_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"name\s*=\s*["']([^"']+)["']"#).unwrap()
});

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
        // Get the text content of the heading, excluding markers
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let document_content = self.context.document_content.borrow();
        let _heading_content = &document_content[start_byte..end_byte];
        
        // For ATX headings, remove the # markers and trim
        if node.kind() == "atx_heading" {
            // Find the heading text by looking for inline content
            for i in 0..node.child_count() {
                let child = node.child(i).unwrap();
                if child.kind() == "inline" {
                    let child_start = child.start_byte();
                    let child_end = child.end_byte();
                    let text = &document_content[child_start..child_end].trim();
                    return Some(text.to_string());
                }
            }
        }
        
        // For setext headings, look for paragraph containing inline content
        if node.kind() == "setext_heading" {
            for i in 0..node.child_count() {
                let child = node.child(i).unwrap();
                if child.kind() == "paragraph" {
                    // Look for inline content within the paragraph
                    for j in 0..child.child_count() {
                        let grandchild = child.child(j).unwrap();
                        if grandchild.kind() == "inline" {
                            let grandchild_start = grandchild.start_byte();
                            let grandchild_end = grandchild.end_byte();
                            let text = &document_content[grandchild_start..grandchild_end].trim();
                            return Some(text.to_string());
                        }
                    }
                }
            }
        }
        
        None
    }

    fn generate_github_fragment(&self, heading_text: &str) -> String {
        // Implementation of GitHub's heading algorithm:
        // 1. Convert to lowercase
        // 2. Remove punctuation (keep alphanumeric, spaces, hyphens)
        // 3. Replace spaces with hyphens
        // 4. Remove multiple consecutive hyphens
        
        let mut result = heading_text.to_lowercase();
        
        // Remove punctuation, keeping only alphanumeric, spaces, and hyphens
        result = result.chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-')
            .collect();
        
        // Replace spaces with hyphens
        result = result.replace(' ', "-");
        
        // Remove multiple consecutive hyphens efficiently
        let chars: Vec<char> = result.chars().collect();
        let mut filtered = Vec::new();
        let mut prev_was_dash = false;
        
        for ch in chars {
            if ch == '-' {
                if !prev_was_dash {
                    filtered.push(ch);
                    prev_was_dash = true;
                }
            } else {
                filtered.push(ch);
                prev_was_dash = false;
            }
        }
        result = filtered.into_iter().collect();
        
        // Trim leading/trailing hyphens
        result = result.trim_matches('-').to_string();
        
        result
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

    fn extract_link_fragments(&self, node: &Node) -> Vec<String> {
        // Extract all fragments from link nodes
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let document_content = self.context.document_content.borrow();
        let content = &document_content[start_byte..end_byte];
        
        let mut fragments = Vec::new();
        
        for cap in LINK_PATTERN.captures_iter(content) {
            if let Some(url_with_fragment) = cap.get(2) {
                let url_text = url_with_fragment.as_str();
                if let Some(hash_pos) = url_text.rfind('#') {
                    let fragment = &url_text[hash_pos + 1..];
                    // Only process non-empty fragments that don't contain spaces
                    // Fragments with spaces are likely malformed and handled by other rules
                    if !fragment.is_empty() && !fragment.contains(' ') {
                        fragments.push(fragment.to_string());
                    }
                }
            }
        }
        
        fragments
    }

    fn is_github_special_fragment(&self, fragment: &str) -> bool {
        // GitHub special fragments according to GitHub specification
        // Reference: https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/creating-a-permanent-link-to-a-code-snippet
        
        if fragment == "top" {
            return true;
        }
        
        // Line number patterns: L followed by one or more digits (L123, L1, etc.)
        if fragment.starts_with('L') && fragment.len() > 1 && fragment[1..].chars().all(|c| c.is_ascii_digit()) {
            return true;
        }
        
        // Range patterns: L19C5-L21C11 (GitHub's official format for line ranges with column numbers)
        if RANGE_PATTERN.is_match(fragment) {
            return true;
        }
        
        // Note: L10-L20 format is NOT valid according to GitHub spec
        // GitHub requires column numbers: L10C1-L20C5
        
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
                        self.valid_fragments_lowercase.insert(custom_anchor.to_lowercase());
                        // Also generate the default fragment from the heading text without the anchor
                        let clean_text = heading_text.replace(&format!("{{#{custom_anchor}}}"), "").trim().to_string();
                        if !clean_text.is_empty() {
                            let fragment = self.generate_github_fragment(&clean_text);
                            if !fragment.is_empty() {
                                self.valid_fragments.insert(fragment.clone());
                                self.valid_fragments_lowercase.insert(fragment.to_lowercase());
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
                            self.valid_fragments_lowercase.insert(unique_fragment.to_lowercase());
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
                let fragments = self.extract_link_fragments(node);
                for fragment in fragments {
                    // We need to store the node for later violation reporting
                    // Note: This is a simplified approach. In a real implementation,
                    // we'd need to handle the lifetime properly
                    self.link_fragments.push(LinkFragment {
                        fragment,
                        // SAFETY: This transmute extends the lifetime of the Node.
                        // It's safe because the node is only used during document analysis
                        // and the MultiRuleLinter ensures the tree lives as long as the linter.
                        node: unsafe { std::mem::transmute::<tree_sitter::Node<'_>, tree_sitter::Node<'_>>(*node) },
                    });
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
                    range_from_tree_sitter(&link_fragment.node.range()),
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

    fn test_config_with_settings(ignore_case: bool, ignored_pattern: String) -> crate::config::QuickmarkConfig {
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
        let input = "# Test Heading

<div id=\"my-custom-id\">Content</div>

[Valid Link](#my-custom-id)
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have no violations - HTML id is valid
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_html_name_attribute() {
        let input = "# Test Heading

<a name=\"my-anchor\">Anchor</a>

[Valid Link](#my-anchor)
";

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
";

        let config = test_config();
        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        
        // Should have 1 violation - L10-L20 is invalid per GitHub spec
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Link fragment 'L10-L20'"));
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
        // Fragments with spaces and empty fragments are ignored (consistent with markdownlint)
        assert_eq!(1, violations.len());
        assert!(violations[0].message().contains("Link fragment 'L'"));
    }
}
