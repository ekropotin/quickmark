use std::collections::{HashMap, HashSet};

use crate::rules::ALL_RULES;

#[derive(Debug, PartialEq, Clone)]
pub enum RuleSeverity {
    Error,
    Warning,
    Off,
}

#[derive(Debug, PartialEq, Clone)]
pub enum HeadingStyle {
    Consistent,
    ATX,
    Setext,
    ATXClosed,
    SetextWithATX,
    SetextWithATXClosed,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UlStyle {
    Asterisk,
    Consistent,
    Dash,
    Plus,
    Sublist,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD003HeadingStyleTable {
    pub style: HeadingStyle,
}

impl Default for MD003HeadingStyleTable {
    fn default() -> Self {
        Self {
            style: HeadingStyle::Consistent,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD004UlStyleTable {
    pub style: UlStyle,
}

impl Default for MD004UlStyleTable {
    fn default() -> Self {
        Self {
            style: UlStyle::Consistent,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD013LineLengthTable {
    pub line_length: usize,
    pub code_block_line_length: usize,
    pub heading_line_length: usize,
    pub code_blocks: bool,
    pub headings: bool,
    pub tables: bool,
    pub strict: bool,
    pub stern: bool,
}

impl Default for MD013LineLengthTable {
    fn default() -> Self {
        Self {
            line_length: 80,
            code_block_line_length: 80,
            heading_line_length: 80,
            code_blocks: true,
            headings: true,
            tables: true,
            strict: false,
            stern: false,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MD051LinkFragmentsTable {
    pub ignore_case: bool,
    pub ignored_pattern: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD052ReferenceLinksImagesTable {
    pub shortcut_syntax: bool,
    pub ignored_labels: Vec<String>,
}

impl Default for MD052ReferenceLinksImagesTable {
    fn default() -> Self {
        Self {
            shortcut_syntax: false,
            ignored_labels: vec!["x".to_string()],
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD053LinkImageReferenceDefinitionsTable {
    pub ignored_definitions: Vec<String>,
}

impl Default for MD053LinkImageReferenceDefinitionsTable {
    fn default() -> Self {
        Self {
            ignored_definitions: vec!["//".to_string()],
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MD024MultipleHeadingsTable {
    pub siblings_only: bool,
    pub allow_different_nesting: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD025SingleH1Table {
    pub level: u8,
    pub front_matter_title: String,
}

impl Default for MD025SingleH1Table {
    fn default() -> Self {
        Self {
            level: 1,
            front_matter_title: r"^\s*title\s*[:=]".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD022HeadingsBlanksTable {
    pub lines_above: Vec<i32>,
    pub lines_below: Vec<i32>,
}

impl Default for MD022HeadingsBlanksTable {
    fn default() -> Self {
        Self {
            lines_above: vec![1],
            lines_below: vec![1],
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD007UlIndentTable {
    pub indent: usize,
    pub start_indent: usize,
    pub start_indented: bool,
}

impl Default for MD007UlIndentTable {
    fn default() -> Self {
        Self {
            indent: 2,
            start_indent: 2,
            start_indented: false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD031FencedCodeBlanksTable {
    pub list_items: bool,
}

impl Default for MD031FencedCodeBlanksTable {
    fn default() -> Self {
        Self { list_items: true }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LintersSettingsTable {
    pub heading_style: MD003HeadingStyleTable,
    pub ul_style: MD004UlStyleTable,
    pub ul_indent: MD007UlIndentTable,
    pub line_length: MD013LineLengthTable,
    pub headings_blanks: MD022HeadingsBlanksTable,
    pub single_h1: MD025SingleH1Table,
    pub fenced_code_blanks: MD031FencedCodeBlanksTable,
    pub multiple_headings: MD024MultipleHeadingsTable,
    pub link_fragments: MD051LinkFragmentsTable,
    pub reference_links_images: MD052ReferenceLinksImagesTable,
    pub link_image_reference_definitions: MD053LinkImageReferenceDefinitionsTable,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LintersTable {
    pub severity: HashMap<String, RuleSeverity>,
    pub settings: LintersSettingsTable,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct QuickmarkConfig {
    pub linters: LintersTable,
}

pub fn normalize_severities(severities: &mut HashMap<String, RuleSeverity>) {
    let rule_aliases: HashSet<&str> = ALL_RULES.iter().map(|r| r.alias).collect();
    severities.retain(|key, _| rule_aliases.contains(key.as_str()));
    for &rule in &rule_aliases {
        severities
            .entry(rule.to_string())
            .or_insert(RuleSeverity::Error);
    }
}

impl QuickmarkConfig {
    pub fn new(linters: LintersTable) -> Self {
        Self { linters }
    }

    pub fn default_with_normalized_severities() -> Self {
        let mut config = Self::default();
        normalize_severities(&mut config.linters.severity);
        config
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::config::{
        HeadingStyle, LintersSettingsTable, LintersTable, MD003HeadingStyleTable,
        MD004UlStyleTable, MD007UlIndentTable, MD013LineLengthTable, MD022HeadingsBlanksTable,
        MD024MultipleHeadingsTable, MD025SingleH1Table, MD031FencedCodeBlanksTable,
        MD051LinkFragmentsTable, MD052ReferenceLinksImagesTable,
        MD053LinkImageReferenceDefinitionsTable, RuleSeverity,
    };

    use super::{normalize_severities, QuickmarkConfig};

    #[test]
    pub fn test_normalize_severities() {
        let mut severity: HashMap<String, RuleSeverity> = vec![
            ("heading-style".to_string(), RuleSeverity::Error),
            ("some-bullshit".to_string(), RuleSeverity::Warning),
        ]
        .into_iter()
        .collect();

        normalize_severities(&mut severity);

        assert_eq!(
            RuleSeverity::Error,
            *severity.get("heading-increment").unwrap()
        );
        assert_eq!(RuleSeverity::Error, *severity.get("heading-style").unwrap());
        assert_eq!(RuleSeverity::Error, *severity.get("list-indent").unwrap());
        assert_eq!(None, severity.get("some-bullshit"));
    }

    #[test]
    pub fn test_default_with_normalized_severities() {
        let config = QuickmarkConfig::default_with_normalized_severities();
        assert_eq!(
            RuleSeverity::Error,
            *config.linters.severity.get("heading-increment").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *config.linters.severity.get("heading-style").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *config.linters.severity.get("list-indent").unwrap()
        );
        assert_eq!(
            HeadingStyle::Consistent,
            config.linters.settings.heading_style.style
        );
    }

    #[test]
    pub fn test_new_config() {
        let severity: HashMap<String, RuleSeverity> = vec![
            ("heading-increment".to_string(), RuleSeverity::Warning),
            ("heading-style".to_string(), RuleSeverity::Off),
        ]
        .into_iter()
        .collect();

        let config = QuickmarkConfig::new(LintersTable {
            severity,
            settings: LintersSettingsTable {
                heading_style: MD003HeadingStyleTable {
                    style: HeadingStyle::ATX,
                },
                ul_style: MD004UlStyleTable::default(),
                ul_indent: MD007UlIndentTable::default(),
                line_length: MD013LineLengthTable::default(),
                headings_blanks: MD022HeadingsBlanksTable::default(),
                single_h1: MD025SingleH1Table::default(),
                fenced_code_blanks: MD031FencedCodeBlanksTable::default(),
                multiple_headings: MD024MultipleHeadingsTable::default(),
                link_fragments: MD051LinkFragmentsTable::default(),
                reference_links_images: MD052ReferenceLinksImagesTable::default(),
                link_image_reference_definitions: MD053LinkImageReferenceDefinitionsTable::default(
                ),
            },
        });

        assert_eq!(
            RuleSeverity::Warning,
            *config.linters.severity.get("heading-increment").unwrap()
        );
        assert_eq!(
            RuleSeverity::Off,
            *config.linters.severity.get("heading-style").unwrap()
        );
        assert_eq!(
            HeadingStyle::ATX,
            config.linters.settings.heading_style.style
        );
    }
}
