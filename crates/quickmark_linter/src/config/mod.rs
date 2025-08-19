use anyhow::Result;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::{fs, path::Path};

use crate::rules::ALL_RULES;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum RuleSeverity {
    #[serde(rename = "err")]
    Error,
    #[serde(rename = "warn")]
    Warning,
    #[serde(rename = "off")]
    Off,
}

// Re-export MD003 configuration types for backward compatibility
pub use crate::rules::md003::{HeadingStyle, MD003HeadingStyleTable};

// Re-export MD004 configuration types for backward compatibility
pub use crate::rules::md004::{MD004UlStyleTable, UlStyle};

// Re-export MD007 configuration types for backward compatibility
pub use crate::rules::md007::MD007UlIndentTable;

// Re-export MD009 configuration types for backward compatibility
pub use crate::rules::md009::MD009TrailingSpacesTable;

// Re-export MD010 configuration types for backward compatibility
pub use crate::rules::md010::MD010HardTabsTable;

// Re-export MD012 configuration types for backward compatibility
pub use crate::rules::md012::MD012MultipleBlankLinesTable;

// Re-export MD013 configuration types for backward compatibility
pub use crate::rules::md013::MD013LineLengthTable;

#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub enum OlPrefixStyle {
    #[serde(rename = "one")]
    One,
    #[serde(rename = "ordered")]
    Ordered,
    #[serde(rename = "one_or_ordered")]
    OneOrOrdered,
    #[serde(rename = "zero")]
    Zero,
}

impl Default for OlPrefixStyle {
    fn default() -> Self {
        Self::OneOrOrdered
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD029OlPrefixTable {
    #[serde(default)]
    pub style: OlPrefixStyle,
}

impl Default for MD029OlPrefixTable {
    fn default() -> Self {
        Self {
            style: OlPrefixStyle::OneOrOrdered,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize)]
pub struct MD051LinkFragmentsTable {
    #[serde(default = "default_false")]
    pub ignore_case: bool,
    #[serde(default = "default_empty_string")]
    pub ignored_pattern: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD052ReferenceLinksImagesTable {
    #[serde(default = "default_false")]
    pub shortcut_syntax: bool,
    #[serde(default = "default_ignored_labels")]
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

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD053LinkImageReferenceDefinitionsTable {
    #[serde(default = "default_ignored_definitions")]
    pub ignored_definitions: Vec<String>,
}

impl Default for MD053LinkImageReferenceDefinitionsTable {
    fn default() -> Self {
        Self {
            ignored_definitions: vec!["//".to_string()],
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD054LinkImageStyleTable {
    #[serde(default = "default_true")]
    pub autolink: bool,
    #[serde(default = "default_true")]
    pub inline: bool,
    #[serde(default = "default_true")]
    pub full: bool,
    #[serde(default = "default_true")]
    pub collapsed: bool,
    #[serde(default = "default_true")]
    pub shortcut: bool,
    #[serde(default = "default_true")]
    pub url_inline: bool,
}

impl Default for MD054LinkImageStyleTable {
    fn default() -> Self {
        Self {
            autolink: true,
            inline: true,
            full: true,
            collapsed: true,
            shortcut: true,
            url_inline: true,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum TablePipeStyle {
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "leading_and_trailing")]
    LeadingAndTrailing,
    #[serde(rename = "leading_only")]
    LeadingOnly,
    #[serde(rename = "trailing_only")]
    TrailingOnly,
    #[serde(rename = "no_leading_or_trailing")]
    NoLeadingOrTrailing,
}

impl Default for TablePipeStyle {
    fn default() -> Self {
        Self::Consistent
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD055TablePipeStyleTable {
    #[serde(default)]
    pub style: TablePipeStyle,
}

impl Default for MD055TablePipeStyleTable {
    fn default() -> Self {
        Self {
            style: TablePipeStyle::Consistent,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD059DescriptiveLinkTextTable {
    #[serde(default = "default_prohibited_texts")]
    pub prohibited_texts: Vec<String>,
}

impl Default for MD059DescriptiveLinkTextTable {
    fn default() -> Self {
        Self {
            prohibited_texts: vec![
                "click here".to_string(),
                "here".to_string(),
                "link".to_string(),
                "more".to_string(),
            ],
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD044ProperNamesTable {
    #[serde(default = "default_empty_vec")]
    pub names: Vec<String>,
    #[serde(default = "default_true")]
    pub code_blocks: bool,
    #[serde(default = "default_true")]
    pub html_elements: bool,
}

impl Default for MD044ProperNamesTable {
    fn default() -> Self {
        Self {
            names: Vec::new(),
            code_blocks: true,
            html_elements: true,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize)]
pub struct MD024MultipleHeadingsTable {
    #[serde(default = "default_false")]
    pub siblings_only: bool,
    #[serde(default = "default_false")]
    pub allow_different_nesting: bool,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD025SingleH1Table {
    #[serde(default = "default_level_1")]
    pub level: u8,
    #[serde(default = "default_front_matter_title")]
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

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD041FirstLineHeadingTable {
    #[serde(default = "default_allow_preamble")]
    pub allow_preamble: bool,
    #[serde(default = "default_front_matter_title")]
    pub front_matter_title: String,
    #[serde(default = "default_level_1")]
    pub level: u8,
}

impl Default for MD041FirstLineHeadingTable {
    fn default() -> Self {
        Self {
            allow_preamble: false,
            front_matter_title: r"^\s*title\s*[:=]".to_string(),
            level: 1,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD022HeadingsBlanksTable {
    #[serde(default = "default_lines_config")]
    pub lines_above: Vec<i32>,
    #[serde(default = "default_lines_config")]
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

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD031FencedCodeBlanksTable {
    #[serde(default = "default_list_items_true")]
    pub list_items: bool,
}

impl Default for MD031FencedCodeBlanksTable {
    fn default() -> Self {
        Self { list_items: true }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize)]
pub struct MD043RequiredHeadingsTable {
    #[serde(default = "default_empty_headings")]
    pub headings: Vec<String>,
    #[serde(default = "default_false")]
    pub match_case: bool,
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize)]
pub struct MD026TrailingPunctuationTable {
    #[serde(default = "default_trailing_punctuation")]
    pub punctuation: String,
}

impl MD026TrailingPunctuationTable {
    pub fn with_default_punctuation() -> Self {
        Self {
            punctuation: ".,;:!。，；：！".to_string(), // Default without '?' chars
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD036EmphasisAsHeadingTable {
    #[serde(default = "default_md036_punctuation")]
    pub punctuation: String,
}

impl Default for MD036EmphasisAsHeadingTable {
    fn default() -> Self {
        Self {
            punctuation: ".,;:!?。，；：！？".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD027BlockquoteSpacesTable {
    #[serde(default = "default_blockquote_list_items")]
    pub list_items: bool,
}

impl Default for MD027BlockquoteSpacesTable {
    fn default() -> Self {
        Self { list_items: true }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize)]
pub struct MD033InlineHtmlTable {
    #[serde(default = "default_empty_vec")]
    pub allowed_elements: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD030ListMarkerSpaceTable {
    #[serde(default = "default_ul_single")]
    pub ul_single: usize,
    #[serde(default = "default_ol_single")]
    pub ol_single: usize,
    #[serde(default = "default_ul_multi")]
    pub ul_multi: usize,
    #[serde(default = "default_ol_multi")]
    pub ol_multi: usize,
}

impl Default for MD030ListMarkerSpaceTable {
    fn default() -> Self {
        Self {
            ul_single: 1,
            ol_single: 1,
            ul_multi: 1,
            ol_multi: 1,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize)]
pub struct MD040FencedCodeLanguageTable {
    #[serde(default = "default_empty_vec")]
    pub allowed_languages: Vec<String>,
    #[serde(default = "default_false")]
    pub language_only: bool,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum CodeBlockStyle {
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "fenced")]
    Fenced,
    #[serde(rename = "indented")]
    Indented,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum CodeFenceStyle {
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "backtick")]
    Backtick,
    #[serde(rename = "tilde")]
    Tilde,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum EmphasisStyle {
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "asterisk")]
    Asterisk,
    #[serde(rename = "underscore")]
    Underscore,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum StrongStyle {
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "asterisk")]
    Asterisk,
    #[serde(rename = "underscore")]
    Underscore,
}

impl Default for CodeBlockStyle {
    fn default() -> Self {
        Self::Consistent
    }
}

impl Default for CodeFenceStyle {
    fn default() -> Self {
        Self::Consistent
    }
}

impl Default for EmphasisStyle {
    fn default() -> Self {
        Self::Consistent
    }
}

impl Default for StrongStyle {
    fn default() -> Self {
        Self::Consistent
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD046CodeBlockStyleTable {
    #[serde(default)]
    pub style: CodeBlockStyle,
}

impl Default for MD046CodeBlockStyleTable {
    fn default() -> Self {
        Self {
            style: CodeBlockStyle::Consistent,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD048CodeFenceStyleTable {
    #[serde(default)]
    pub style: CodeFenceStyle,
}

impl Default for MD048CodeFenceStyleTable {
    fn default() -> Self {
        Self {
            style: CodeFenceStyle::Consistent,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD049EmphasisStyleTable {
    #[serde(default)]
    pub style: EmphasisStyle,
}

impl Default for MD049EmphasisStyleTable {
    fn default() -> Self {
        Self {
            style: EmphasisStyle::Consistent,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD050StrongStyleTable {
    #[serde(default)]
    pub style: StrongStyle,
}

impl Default for MD050StrongStyleTable {
    fn default() -> Self {
        Self {
            style: StrongStyle::Consistent,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MD035HrStyleTable {
    #[serde(default = "default_hr_style")]
    pub style: String,
}

impl Default for MD035HrStyleTable {
    fn default() -> Self {
        Self {
            style: "consistent".to_string(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Deserialize)]
pub struct LintersSettingsTable {
    #[serde(rename = "heading-style")]
    #[serde(default)]
    pub heading_style: MD003HeadingStyleTable,
    #[serde(rename = "ul-style")]
    #[serde(default)]
    pub ul_style: MD004UlStyleTable,
    #[serde(rename = "ol-prefix")]
    #[serde(default)]
    pub ol_prefix: MD029OlPrefixTable,
    #[serde(rename = "ul-indent")]
    #[serde(default)]
    pub ul_indent: MD007UlIndentTable,
    #[serde(rename = "no-trailing-spaces")]
    #[serde(default)]
    pub trailing_spaces: MD009TrailingSpacesTable,
    #[serde(rename = "no-hard-tabs")]
    #[serde(default)]
    pub hard_tabs: MD010HardTabsTable,
    #[serde(rename = "no-multiple-blanks")]
    #[serde(default)]
    pub multiple_blank_lines: MD012MultipleBlankLinesTable,
    #[serde(rename = "line-length")]
    #[serde(default)]
    pub line_length: MD013LineLengthTable,
    #[serde(rename = "blanks-around-headings")]
    #[serde(default)]
    pub headings_blanks: MD022HeadingsBlanksTable,
    #[serde(rename = "single-h1")]
    #[serde(default)]
    pub single_h1: MD025SingleH1Table,
    #[serde(rename = "first-line-heading")]
    #[serde(default)]
    pub first_line_heading: MD041FirstLineHeadingTable,
    #[serde(rename = "no-trailing-punctuation")]
    #[serde(default)]
    pub trailing_punctuation: MD026TrailingPunctuationTable,
    #[serde(rename = "no-multiple-space-blockquote")]
    #[serde(default)]
    pub blockquote_spaces: MD027BlockquoteSpacesTable,
    #[serde(rename = "list-marker-space")]
    #[serde(default)]
    pub list_marker_space: MD030ListMarkerSpaceTable,
    #[serde(rename = "blanks-around-fences")]
    #[serde(default)]
    pub fenced_code_blanks: MD031FencedCodeBlanksTable,
    #[serde(rename = "no-inline-html")]
    #[serde(default)]
    pub inline_html: MD033InlineHtmlTable,
    #[serde(rename = "hr-style")]
    #[serde(default)]
    pub hr_style: MD035HrStyleTable,
    #[serde(rename = "no-emphasis-as-heading")]
    #[serde(default)]
    pub emphasis_as_heading: MD036EmphasisAsHeadingTable,
    #[serde(rename = "fenced-code-language")]
    #[serde(default)]
    pub fenced_code_language: MD040FencedCodeLanguageTable,
    #[serde(rename = "code-block-style")]
    #[serde(default)]
    pub code_block_style: MD046CodeBlockStyleTable,
    #[serde(rename = "code-fence-style")]
    #[serde(default)]
    pub code_fence_style: MD048CodeFenceStyleTable,
    #[serde(rename = "emphasis-style")]
    #[serde(default)]
    pub emphasis_style: MD049EmphasisStyleTable,
    #[serde(rename = "strong-style")]
    #[serde(default)]
    pub strong_style: MD050StrongStyleTable,
    #[serde(rename = "no-duplicate-heading")]
    #[serde(default)]
    pub multiple_headings: MD024MultipleHeadingsTable,
    #[serde(rename = "required-headings")]
    #[serde(default)]
    pub required_headings: MD043RequiredHeadingsTable,
    #[serde(rename = "proper-names")]
    #[serde(default)]
    pub proper_names: MD044ProperNamesTable,
    #[serde(rename = "link-fragments")]
    #[serde(default)]
    pub link_fragments: MD051LinkFragmentsTable,
    #[serde(rename = "reference-links-images")]
    #[serde(default)]
    pub reference_links_images: MD052ReferenceLinksImagesTable,
    #[serde(rename = "link-image-reference-definitions")]
    #[serde(default)]
    pub link_image_reference_definitions: MD053LinkImageReferenceDefinitionsTable,
    #[serde(rename = "link-image-style")]
    #[serde(default)]
    pub link_image_style: MD054LinkImageStyleTable,
    #[serde(rename = "table-pipe-style")]
    #[serde(default)]
    pub table_pipe_style: MD055TablePipeStyleTable,
    #[serde(rename = "descriptive-link-text")]
    #[serde(default)]
    pub descriptive_link_text: MD059DescriptiveLinkTextTable,
}

#[derive(Debug, Default, PartialEq, Clone, Deserialize)]
pub struct LintersTable {
    #[serde(default)]
    pub severity: HashMap<String, RuleSeverity>,
    #[serde(default)]
    pub settings: LintersSettingsTable,
}

#[derive(Debug, Default, PartialEq, Clone, Deserialize)]
pub struct QuickmarkConfig {
    #[serde(default)]
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

// Default functions for TOML deserialization
pub fn default_indent() -> usize {
    2 // MD007UlIndentTable default
}

pub fn default_br_spaces() -> usize {
    2 // MD009TrailingSpacesTable default
}

pub fn default_spaces_per_tab() -> usize {
    1 // MD010HardTabsTable default
}

pub fn default_one() -> usize {
    1 // MD012MultipleBlankLinesTable default
}

pub fn default_empty_code_languages() -> Vec<String> {
    Vec::new() // MD010HardTabsTable default
}

pub fn default_line_length() -> usize {
    80 // MD013LineLengthTable default
}

pub fn default_code_block_line_length() -> usize {
    80 // MD013LineLengthTable default
}

pub fn default_heading_line_length() -> usize {
    80 // MD013LineLengthTable default
}

pub fn default_true() -> bool {
    true
}

pub fn default_false() -> bool {
    false
}

pub fn default_empty_string() -> String {
    String::new()
}

pub fn default_level_1() -> u8 {
    MD025SingleH1Table::default().level
}

pub fn default_front_matter_title() -> String {
    MD025SingleH1Table::default().front_matter_title
}

pub fn default_allow_preamble() -> bool {
    MD041FirstLineHeadingTable::default().allow_preamble
}

pub fn default_trailing_punctuation() -> String {
    ".,;:!。，；：！".to_string()
}

pub fn default_blockquote_list_items() -> bool {
    MD027BlockquoteSpacesTable::default().list_items
}

pub fn default_ul_single() -> usize {
    MD030ListMarkerSpaceTable::default().ul_single
}

pub fn default_ol_single() -> usize {
    MD030ListMarkerSpaceTable::default().ol_single
}

pub fn default_ul_multi() -> usize {
    MD030ListMarkerSpaceTable::default().ul_multi
}

pub fn default_ol_multi() -> usize {
    MD030ListMarkerSpaceTable::default().ol_multi
}

pub fn default_lines_config() -> Vec<i32> {
    vec![1]
}

pub fn default_list_items_true() -> bool {
    true
}

pub fn default_empty_headings() -> Vec<String> {
    Vec::new()
}

pub fn default_empty_vec() -> Vec<String> {
    Vec::new()
}

pub fn default_hr_style() -> String {
    "consistent".to_string()
}

pub fn default_md036_punctuation() -> String {
    ".,;:!?。，；：！？".to_string()
}

pub fn default_prohibited_texts() -> Vec<String> {
    vec![
        "click here".to_string(),
        "here".to_string(),
        "link".to_string(),
        "more".to_string(),
    ]
}

pub fn default_ignored_labels() -> Vec<String> {
    vec!["x".to_string()]
}

pub fn default_ignored_definitions() -> Vec<String> {
    vec!["//".to_string()]
}

/// Parse a TOML configuration string into a QuickmarkConfig
pub fn parse_toml_config(config_str: &str) -> Result<QuickmarkConfig> {
    let mut config: QuickmarkConfig = toml::from_str(config_str)?;
    normalize_severities(&mut config.linters.severity);
    Ok(config)
}

/// Load configuration from QUICKMARK_CONFIG environment variable, path, or default
pub fn config_from_env_path_or_default(path: &Path) -> Result<QuickmarkConfig> {
    // First check if QUICKMARK_CONFIG environment variable is set
    if let Ok(env_config_path) = std::env::var("QUICKMARK_CONFIG") {
        let env_config_file = Path::new(&env_config_path);
        if env_config_file.is_file() {
            match fs::read_to_string(env_config_file) {
                Ok(config) => return parse_toml_config(&config),
                Err(e) => {
                    eprintln!(
                        "Error loading config from QUICKMARK_CONFIG path {env_config_path}: {e}. Default config will be used."
                    );
                    return Ok(QuickmarkConfig::default_with_normalized_severities());
                }
            }
        } else {
            eprintln!(
                "Config file was not found at QUICKMARK_CONFIG path {env_config_path}. Default config will be used."
            );
            return Ok(QuickmarkConfig::default_with_normalized_severities());
        }
    }

    // Fallback to existing behavior - check for quickmark.toml in path
    config_in_path_or_default(path)
}

/// Load configuration from a path, or return default if not found
pub fn config_in_path_or_default(path: &Path) -> Result<QuickmarkConfig> {
    let config_file = path.join("quickmark.toml");
    if config_file.is_file() {
        let config = fs::read_to_string(config_file)?;
        return parse_toml_config(&config);
    }
    println!(
        "Config file was not found at {}. Default config will be used.",
        config_file.to_string_lossy()
    );
    Ok(QuickmarkConfig::default_with_normalized_severities())
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::path::Path;

    use crate::config::{
        config_from_env_path_or_default, parse_toml_config, HeadingStyle, LintersSettingsTable,
        LintersTable, MD003HeadingStyleTable, MD004UlStyleTable, MD007UlIndentTable,
        MD009TrailingSpacesTable, MD010HardTabsTable, MD012MultipleBlankLinesTable,
        MD013LineLengthTable, MD022HeadingsBlanksTable, MD024MultipleHeadingsTable,
        MD025SingleH1Table, MD026TrailingPunctuationTable, MD027BlockquoteSpacesTable,
        MD029OlPrefixTable, MD030ListMarkerSpaceTable, MD031FencedCodeBlanksTable,
        MD033InlineHtmlTable, MD035HrStyleTable, MD036EmphasisAsHeadingTable,
        MD040FencedCodeLanguageTable, MD041FirstLineHeadingTable, MD043RequiredHeadingsTable,
        MD044ProperNamesTable, MD046CodeBlockStyleTable, MD048CodeFenceStyleTable,
        MD049EmphasisStyleTable, MD050StrongStyleTable, MD051LinkFragmentsTable,
        MD052ReferenceLinksImagesTable, MD053LinkImageReferenceDefinitionsTable,
        MD054LinkImageStyleTable, MD055TablePipeStyleTable, MD059DescriptiveLinkTextTable,
        RuleSeverity, UlStyle,
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
        assert_eq!(
            RuleSeverity::Error,
            *severity.get("no-reversed-links").unwrap()
        );
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
            RuleSeverity::Error,
            *config.linters.severity.get("no-reversed-links").unwrap()
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
                ol_prefix: MD029OlPrefixTable::default(),
                list_marker_space: MD030ListMarkerSpaceTable::default(),
                ul_indent: MD007UlIndentTable::default(),
                trailing_spaces: MD009TrailingSpacesTable::default(),
                hard_tabs: MD010HardTabsTable::default(),
                multiple_blank_lines: MD012MultipleBlankLinesTable::default(),
                line_length: MD013LineLengthTable::default(),
                headings_blanks: MD022HeadingsBlanksTable::default(),
                single_h1: MD025SingleH1Table::default(),
                first_line_heading: MD041FirstLineHeadingTable::default(),
                trailing_punctuation: MD026TrailingPunctuationTable::default(),
                blockquote_spaces: MD027BlockquoteSpacesTable::default(),
                fenced_code_blanks: MD031FencedCodeBlanksTable::default(),
                inline_html: MD033InlineHtmlTable::default(),
                hr_style: MD035HrStyleTable::default(),
                emphasis_as_heading: MD036EmphasisAsHeadingTable::default(),
                fenced_code_language: MD040FencedCodeLanguageTable::default(),
                code_block_style: MD046CodeBlockStyleTable::default(),
                code_fence_style: MD048CodeFenceStyleTable::default(),
                emphasis_style: MD049EmphasisStyleTable::default(),
                strong_style: MD050StrongStyleTable::default(),
                multiple_headings: MD024MultipleHeadingsTable::default(),
                required_headings: MD043RequiredHeadingsTable::default(),
                proper_names: MD044ProperNamesTable::default(),
                link_fragments: MD051LinkFragmentsTable::default(),
                reference_links_images: MD052ReferenceLinksImagesTable::default(),
                link_image_reference_definitions: MD053LinkImageReferenceDefinitionsTable::default(
                ),
                link_image_style: MD054LinkImageStyleTable::default(),
                table_pipe_style: MD055TablePipeStyleTable::default(),
                descriptive_link_text: MD059DescriptiveLinkTextTable::default(),
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

    // TOML parsing tests
    #[test]
    fn test_parse_md028_config() {
        let config_str = r#"
        [linters.severity]
        no-blanks-blockquote = 'warn'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("no-blanks-blockquote").unwrap()
        );
    }

    #[test]
    fn test_parse_toml_config_with_invalid_rules() {
        let config_str = r#"
        [linters.severity]
        heading-style = 'err'
        some-invalid-rule = 'warn'

        [linters.settings.heading-style]
        style = 'atx'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("heading-increment").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("heading-style").unwrap()
        );
        assert_eq!(None, parsed.linters.severity.get("some-invalid-rule"));
    }

    #[test]
    fn test_parse_md004_ul_style_config() {
        let config_str = r#"
        [linters.severity]
        ul-style = 'err'

        [linters.settings.ul-style]
        style = 'asterisk'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("ul-style").unwrap()
        );
        assert_eq!(UlStyle::Asterisk, parsed.linters.settings.ul_style.style);
    }

    #[test]
    fn test_parse_md004_sublist_style_config() {
        let config_str = r#"
        [linters.severity]
        ul-style = 'warn'

        [linters.settings.ul-style]
        style = 'sublist'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("ul-style").unwrap()
        );
        assert_eq!(UlStyle::Sublist, parsed.linters.settings.ul_style.style);
    }

    #[test]
    fn test_parse_md007_ul_indent_config() {
        let config_str = r#"
        [linters.severity]
        ul-indent = 'err'

        [linters.settings.ul-indent]
        indent = 4
        start_indent = 3
        start_indented = true
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("ul-indent").unwrap()
        );
        assert_eq!(4, parsed.linters.settings.ul_indent.indent);
        assert_eq!(3, parsed.linters.settings.ul_indent.start_indent);
        assert!(parsed.linters.settings.ul_indent.start_indented);
    }

    #[test]
    fn test_parse_md007_default_values() {
        let config_str = r#"
        [linters.severity]
        ul-indent = 'warn'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("ul-indent").unwrap()
        );
        // Test default values
        assert_eq!(2, parsed.linters.settings.ul_indent.indent);
        assert_eq!(2, parsed.linters.settings.ul_indent.start_indent);
        assert!(!parsed.linters.settings.ul_indent.start_indented);
    }

    #[test]
    fn test_config_from_env_fallback_to_local() {
        // Create a local config in a temp directory
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("quickmark.toml");
        let config_content = r#"
        [linters.severity]
        heading-increment = 'err'
        heading-style = 'off'
        "#;

        std::fs::write(&config_path, config_content).unwrap();

        // Load config - should fall back to checking the provided path
        let config = config_from_env_path_or_default(temp_dir.path()).unwrap();

        assert_eq!(
            RuleSeverity::Error,
            *config.linters.severity.get("heading-increment").unwrap()
        );
        assert_eq!(
            RuleSeverity::Off,
            *config.linters.severity.get("heading-style").unwrap()
        );
    }

    #[test]
    fn test_config_from_env_default_when_no_config() {
        let dummy_path = Path::new("/tmp");
        let config = config_from_env_path_or_default(dummy_path).unwrap();

        // Should use default configuration
        assert_eq!(
            RuleSeverity::Error,
            *config.linters.severity.get("heading-increment").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *config.linters.severity.get("heading-style").unwrap()
        );
    }
}
