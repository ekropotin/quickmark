use serde::Deserialize;
use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::{range_from_tree_sitter, Context, RuleLinter, RuleViolation},
    rules::{Rule, RuleType},
};

// MD024-specific configuration types
#[derive(Debug, PartialEq, Clone, Deserialize, Default)]
pub struct MD024MultipleHeadingsTable {
    #[serde(default)]
    pub siblings_only: bool,
    #[serde(default)]
    pub allow_different_nesting: bool,
}

pub(crate) struct MD024Linter {
    context: Rc<Context>,
    violations: Vec<RuleViolation>,
    headings: Vec<HeadingInfo>,
}

#[derive(Debug, Clone)]
struct HeadingInfo {
    content: String,
    level: u8,
    node_range: tree_sitter::Range,
    parent_path: Vec<String>, // Path from root to parent heading
}

impl MD024Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            violations: Vec::new(),
            headings: Vec::new(),
        }
    }

    fn extract_heading_content(&self, node: &Node) -> String {
        // Extract text content from heading node
        let source = self.context.get_document_content();
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let full_text = &source[start_byte..end_byte];

        // Remove markdown syntax and trim
        match node.kind() {
            "atx_heading" => {
                // Remove leading #s and trailing #s if present
                let text = full_text
                    .trim_start_matches('#')
                    .trim()
                    .trim_end_matches('#')
                    .trim();
                // Normalize whitespace: replace multiple spaces with single space
                text.split_whitespace().collect::<Vec<_>>().join(" ")
            }
            "setext_heading" => {
                // For setext, take first line (before underline)
                if let Some(line) = full_text.lines().next() {
                    let trimmed = line.trim();
                    // Normalize whitespace: replace multiple spaces with single space
                    trimmed.split_whitespace().collect::<Vec<_>>().join(" ")
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }

    fn extract_heading_level(&self, node: &Node) -> u8 {
        match node.kind() {
            "atx_heading" => {
                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();
                    if child.kind().starts_with("atx_h") && child.kind().ends_with("_marker") {
                        return child.kind().chars().nth(5).unwrap().to_digit(10).unwrap() as u8;
                    }
                }
                1 // fallback
            }
            "setext_heading" => {
                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();
                    if child.kind() == "setext_h1_underline" {
                        return 1;
                    } else if child.kind() == "setext_h2_underline" {
                        return 2;
                    }
                }
                1 // fallback
            }
            _ => 1,
        }
    }

    fn build_parent_path(&self, current_level: u8) -> Vec<String> {
        let mut parent_path = Vec::new();

        // Find all headings at levels less than current_level, in reverse order
        for heading in self.headings.iter().rev() {
            if heading.level < current_level {
                parent_path.insert(0, heading.content.clone());
                // Continue looking for higher-level parents
                if heading.level == 1 {
                    break; // Reached the top level
                }
            }
        }

        parent_path
    }

    fn check_for_duplicate(&mut self, current_heading: &HeadingInfo) {
        let config = &self.context.config.linters.settings.multiple_headings;

        for existing_heading in &self.headings {
            if existing_heading.content == current_heading.content {
                let is_violation = if config.siblings_only {
                    // Only report duplicates within same parent
                    existing_heading.parent_path == current_heading.parent_path
                } else if config.allow_different_nesting {
                    // Allow duplicates at different levels
                    existing_heading.level == current_heading.level
                } else {
                    // Standard behavior: any duplicate is a violation
                    true
                };

                if is_violation {
                    self.violations.push(RuleViolation::new(
                        &MD024,
                        format!(
                            "{} [Duplicate heading: '{}']",
                            MD024.description, current_heading.content
                        ),
                        self.context.file_path.clone(),
                        range_from_tree_sitter(&current_heading.node_range),
                    ));
                    break; // Only report once per duplicate
                }
            }
        }
    }
}

impl RuleLinter for MD024Linter {
    fn feed(&mut self, node: &Node) {
        if node.kind() == "atx_heading" || node.kind() == "setext_heading" {
            let content = self.extract_heading_content(node);
            let level = self.extract_heading_level(node);
            let parent_path = self.build_parent_path(level);

            let heading_info = HeadingInfo {
                content: content.clone(),
                level,
                node_range: node.range(),
                parent_path,
            };

            self.check_for_duplicate(&heading_info);
            self.headings.push(heading_info);
        }
    }

    fn finalize(&mut self) -> Vec<RuleViolation> {
        std::mem::take(&mut self.violations)
    }
}

pub const MD024: Rule = Rule {
    id: "MD024",
    alias: "no-duplicate-heading",
    tags: &["headings"],
    description: "Multiple headings with the same content",
    rule_type: RuleType::Document,
    required_nodes: &["atx_heading", "setext_heading"],
    new_linter: |context| Box::new(MD024Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::{LintersSettingsTable, MD024MultipleHeadingsTable, RuleSeverity};
    use crate::linter::MultiRuleLinter;
    use crate::test_utils::test_helpers::test_config_with_settings;

    fn test_config(
        siblings_only: bool,
        allow_different_nesting: bool,
    ) -> crate::config::QuickmarkConfig {
        test_config_with_settings(
            vec![("no-duplicate-heading", RuleSeverity::Error)],
            LintersSettingsTable {
                multiple_headings: MD024MultipleHeadingsTable {
                    siblings_only,
                    allow_different_nesting,
                },
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_basic_duplicate_headings() {
        let config = test_config(false, false);
        let input = "# Introduction

Some text

## Section 1

Content

## Section 1

More content";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Section 1"));
    }

    #[test]
    fn test_no_duplicates() {
        let config = test_config(false, false);
        let input = "# Introduction

## Section 1

### Subsection A

## Section 2

### Subsection B";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_siblings_only_different_parents() {
        let config = test_config(true, false);
        let input = "# Chapter 1

## Introduction

# Chapter 2

## Introduction";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0); // Should allow duplicate under different parents
    }

    #[test]
    fn test_siblings_only_same_parent() {
        let config = test_config(true, false);
        let input = "# Chapter 1

## Introduction

## Introduction";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1); // Should detect duplicate under same parent
    }

    #[test]
    fn test_allow_different_nesting_levels() {
        let config = test_config(false, true);
        let input = "# Introduction

## Introduction";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 0); // Should allow duplicate at different levels
    }

    #[test]
    fn test_allow_different_nesting_same_level() {
        let config = test_config(false, true);
        let input = "# Introduction

# Introduction";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1); // Should detect duplicate at same level
    }

    #[test]
    fn test_setext_headings() {
        let config = test_config(false, false);
        let input = "Introduction
============

Section 1
---------

Section 1
---------
";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Section 1"));
    }

    #[test]
    fn test_mixed_heading_styles() {
        let config = test_config(false, false);
        let input = "Introduction
============

## Section 1

Section 1
---------
";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Section 1"));
    }

    #[test]
    fn test_complex_hierarchy() {
        let config = test_config(true, false);
        let input = "# Part 1

## Chapter 1

### Introduction

## Chapter 2

### Introduction

# Part 2

## Chapter 1

### Introduction";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // With siblings_only: "Introduction" under different chapters should be allowed
        // "Chapter 1" under different parts should be allowed
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_whitespace_normalization() {
        let config = test_config(false, false);
        let input = "#   Section   1   

##  Section 1  ";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1); // Should normalize whitespace and detect duplicate
    }

    #[test]
    fn test_empty_headings() {
        let config = test_config(false, false);
        let input = "# 

##

##";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1); // Empty headings should be treated as duplicates
    }

    #[test]
    fn test_atx_closed_headings() {
        let config = test_config(false, false);
        let input = "# Introduction #

## Section 1 ##

## Section 1 ##";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message().contains("Section 1"));
    }

    #[test]
    fn test_both_options_enabled() {
        let config = test_config(true, true);
        let input = "# Chapter 1

## Introduction

# Chapter 2 

## Introduction

### Introduction";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
        // siblings_only=true allows "Introduction" under different parents
        // allow_different_nesting=true allows same name at different levels within same parent
        assert_eq!(violations.len(), 0);
    }
}
