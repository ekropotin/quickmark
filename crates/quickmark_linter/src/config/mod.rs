use std::collections::{HashMap, HashSet};
use serde::Deserialize;

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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OlPrefixStyle {
    One,
    Ordered,
    OneOrOrdered,
    Zero,
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
pub struct MD029OlPrefixTable {
    pub style: OlPrefixStyle,
}

impl Default for MD029OlPrefixTable {
    fn default() -> Self {
        Self {
            style: OlPrefixStyle::OneOrOrdered,
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

#[derive(Debug, PartialEq, Clone)]
pub struct MD054LinkImageStyleTable {
    pub autolink: bool,
    pub inline: bool,
    pub full: bool,
    pub collapsed: bool,
    pub shortcut: bool,
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

#[derive(Debug, PartialEq, Clone)]
pub enum TablePipeStyle {
    Consistent,
    LeadingAndTrailing,
    LeadingOnly,
    TrailingOnly,
    NoLeadingOrTrailing,
}

impl Default for TablePipeStyle {
    fn default() -> Self {
        Self::Consistent
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD055TablePipeStyleTable {
    pub style: TablePipeStyle,
}

impl Default for MD055TablePipeStyleTable {
    fn default() -> Self {
        Self {
            style: TablePipeStyle::Consistent,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD059DescriptiveLinkTextTable {
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

#[derive(Debug, PartialEq, Clone)]
pub struct MD044ProperNamesTable {
    pub names: Vec<String>,
    pub code_blocks: bool,
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
pub struct MD041FirstLineHeadingTable {
    pub allow_preamble: bool,
    pub front_matter_title: String,
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
pub struct MD009TrailingSpacesTable {
    pub br_spaces: usize,
    pub list_item_empty_lines: bool,
    pub strict: bool,
}

impl Default for MD009TrailingSpacesTable {
    fn default() -> Self {
        Self {
            br_spaces: 2,
            list_item_empty_lines: false,
            strict: false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD010HardTabsTable {
    pub code_blocks: bool,
    pub ignore_code_languages: Vec<String>,
    pub spaces_per_tab: usize,
}

impl Default for MD010HardTabsTable {
    fn default() -> Self {
        Self {
            code_blocks: true,
            ignore_code_languages: Vec::new(),
            spaces_per_tab: 1,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD012MultipleBlankLinesTable {
    pub maximum: usize,
}

impl Default for MD012MultipleBlankLinesTable {
    fn default() -> Self {
        Self { maximum: 1 }
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

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MD043RequiredHeadingsTable {
    pub headings: Vec<String>,
    pub match_case: bool,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MD026TrailingPunctuationTable {
    pub punctuation: String,
}

impl MD026TrailingPunctuationTable {
    pub fn with_default_punctuation() -> Self {
        Self {
            punctuation: ".,;:!。，；：！".to_string(), // Default without '?' chars
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD036EmphasisAsHeadingTable {
    pub punctuation: String,
}

impl Default for MD036EmphasisAsHeadingTable {
    fn default() -> Self {
        Self {
            punctuation: ".,;:!?。，；：！？".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD027BlockquoteSpacesTable {
    pub list_items: bool,
}

impl Default for MD027BlockquoteSpacesTable {
    fn default() -> Self {
        Self { list_items: true }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MD033InlineHtmlTable {
    pub allowed_elements: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD030ListMarkerSpaceTable {
    pub ul_single: usize,
    pub ol_single: usize,
    pub ul_multi: usize,
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

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MD040FencedCodeLanguageTable {
    pub allowed_languages: Vec<String>,
    pub language_only: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CodeBlockStyle {
    Consistent,
    Fenced,
    Indented,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CodeFenceStyle {
    Consistent,
    Backtick,
    Tilde,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EmphasisStyle {
    Consistent,
    Asterisk,
    Underscore,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StrongStyle {
    Consistent,
    Asterisk,
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

#[derive(Debug, PartialEq, Clone)]
pub struct MD046CodeBlockStyleTable {
    pub style: CodeBlockStyle,
}

impl Default for MD046CodeBlockStyleTable {
    fn default() -> Self {
        Self {
            style: CodeBlockStyle::Consistent,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD048CodeFenceStyleTable {
    pub style: CodeFenceStyle,
}

impl Default for MD048CodeFenceStyleTable {
    fn default() -> Self {
        Self {
            style: CodeFenceStyle::Consistent,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD049EmphasisStyleTable {
    pub style: EmphasisStyle,
}

impl Default for MD049EmphasisStyleTable {
    fn default() -> Self {
        Self {
            style: EmphasisStyle::Consistent,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD050StrongStyleTable {
    pub style: StrongStyle,
}

impl Default for MD050StrongStyleTable {
    fn default() -> Self {
        Self {
            style: StrongStyle::Consistent,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MD035HrStyleTable {
    pub style: String,
}

impl Default for MD035HrStyleTable {
    fn default() -> Self {
        Self {
            style: "consistent".to_string(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LintersSettingsTable {
    pub heading_style: MD003HeadingStyleTable,
    pub ul_style: MD004UlStyleTable,
    pub ol_prefix: MD029OlPrefixTable,
    pub ul_indent: MD007UlIndentTable,
    pub trailing_spaces: MD009TrailingSpacesTable,
    pub hard_tabs: MD010HardTabsTable,
    pub multiple_blank_lines: MD012MultipleBlankLinesTable,
    pub line_length: MD013LineLengthTable,
    pub headings_blanks: MD022HeadingsBlanksTable,
    pub single_h1: MD025SingleH1Table,
    pub first_line_heading: MD041FirstLineHeadingTable,
    pub trailing_punctuation: MD026TrailingPunctuationTable,
    pub blockquote_spaces: MD027BlockquoteSpacesTable,
    pub list_marker_space: MD030ListMarkerSpaceTable,
    pub fenced_code_blanks: MD031FencedCodeBlanksTable,
    pub inline_html: MD033InlineHtmlTable,
    pub hr_style: MD035HrStyleTable,
    pub emphasis_as_heading: MD036EmphasisAsHeadingTable,
    pub fenced_code_language: MD040FencedCodeLanguageTable,
    pub code_block_style: MD046CodeBlockStyleTable,
    pub code_fence_style: MD048CodeFenceStyleTable,
    pub emphasis_style: MD049EmphasisStyleTable,
    pub strong_style: MD050StrongStyleTable,
    pub multiple_headings: MD024MultipleHeadingsTable,
    pub required_headings: MD043RequiredHeadingsTable,
    pub proper_names: MD044ProperNamesTable,
    pub link_fragments: MD051LinkFragmentsTable,
    pub reference_links_images: MD052ReferenceLinksImagesTable,
    pub link_image_reference_definitions: MD053LinkImageReferenceDefinitionsTable,
    pub link_image_style: MD054LinkImageStyleTable,
    pub table_pipe_style: MD055TablePipeStyleTable,
    pub descriptive_link_text: MD059DescriptiveLinkTextTable,
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

// TOML configuration structures for serialization/deserialization
#[derive(Deserialize)]
pub enum TomlRuleSeverity {
    #[serde(rename = "err")]
    Error,
    #[serde(rename = "warn")]
    Warning,
    #[serde(rename = "off")]
    Off,
}

#[derive(Deserialize)]
pub enum TomlHeadingStyle {
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "atx")]
    Atx,
    #[serde(rename = "setext")]
    Setext,
    #[serde(rename = "atx_closed")]
    ATXClosed,
    #[serde(rename = "setext_with_atx")]
    SetextWithATX,
    #[serde(rename = "setext_with_atx_closed")]
    SetextWithATXClosed,
}

#[derive(Deserialize)]
pub enum TomlUlStyle {
    #[serde(rename = "asterisk")]
    Asterisk,
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "dash")]
    Dash,
    #[serde(rename = "plus")]
    Plus,
    #[serde(rename = "sublist")]
    Sublist,
}

#[derive(Deserialize)]
pub enum TomlOlPrefixStyle {
    #[serde(rename = "one")]
    One,
    #[serde(rename = "ordered")]
    Ordered,
    #[serde(rename = "one_or_ordered")]
    OneOrOrdered,
    #[serde(rename = "zero")]
    Zero,
}

#[derive(Deserialize)]
pub enum TomlCodeBlockStyle {
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "fenced")]
    Fenced,
    #[serde(rename = "indented")]
    Indented,
}

#[derive(Deserialize)]
pub enum TomlCodeFenceStyle {
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "backtick")]
    Backtick,
    #[serde(rename = "tilde")]
    Tilde,
}

#[derive(Deserialize)]
pub enum TomlEmphasisStyle {
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "asterisk")]
    Asterisk,
    #[serde(rename = "underscore")]
    Underscore,
}

#[derive(Deserialize)]
pub enum TomlStrongStyle {
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "asterisk")]
    Asterisk,
    #[serde(rename = "underscore")]
    Underscore,
}

#[derive(Deserialize)]
pub enum TomlTablePipeStyle {
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

// TOML table structures for specific rules
#[derive(Deserialize)]
pub struct TomlMD003HeadingStyleTable {
    pub style: TomlHeadingStyle,
}

#[derive(Deserialize)]
pub struct TomlMD004UlStyleTable {
    pub style: TomlUlStyle,
}

#[derive(Deserialize)]
pub struct TomlMD029OlPrefixTable {
    pub style: TomlOlPrefixStyle,
}

#[derive(Deserialize)]
pub struct TomlMD046CodeBlockStyleTable {
    pub style: TomlCodeBlockStyle,
}

#[derive(Deserialize)]
pub struct TomlMD048CodeFenceStyleTable {
    pub style: TomlCodeFenceStyle,
}

#[derive(Deserialize)]
pub struct TomlMD049EmphasisStyleTable {
    pub style: TomlEmphasisStyle,
}

#[derive(Deserialize)]
pub struct TomlMD050StrongStyleTable {
    pub style: TomlStrongStyle,
}

#[derive(Deserialize)]
pub struct TomlMD055TablePipeStyleTable {
    pub style: TomlTablePipeStyle,
}

// Default functions for TOML parsing
pub fn default_indent() -> usize {
    MD007UlIndentTable::default().indent
}

pub fn default_br_spaces() -> usize {
    MD009TrailingSpacesTable::default().br_spaces
}

pub fn default_spaces_per_tab() -> usize {
    MD010HardTabsTable::default().spaces_per_tab
}

pub fn default_one() -> usize {
    MD012MultipleBlankLinesTable::default().maximum
}

pub fn default_empty_code_languages() -> Vec<String> {
    MD010HardTabsTable::default().ignore_code_languages
}

pub fn default_line_length() -> usize {
    MD013LineLengthTable::default().line_length
}

pub fn default_code_block_line_length() -> usize {
    MD013LineLengthTable::default().code_block_line_length
}

pub fn default_heading_line_length() -> usize {
    MD013LineLengthTable::default().heading_line_length
}

pub fn default_true() -> bool {
    MD013LineLengthTable::default().code_blocks // any boolean field that defaults to true
}

pub fn default_false() -> bool {
    MD013LineLengthTable::default().strict // any boolean field that defaults to false
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

// Complex TOML rule configuration structures
#[derive(Deserialize)]
pub struct TomlMD007UlIndentTable {
    #[serde(default = "default_indent")]
    pub indent: usize,
    #[serde(default = "default_indent")]
    pub start_indent: usize,
    #[serde(default = "default_false")]
    pub start_indented: bool,
}

impl Default for TomlMD007UlIndentTable {
    fn default() -> Self {
        let default_config = MD007UlIndentTable::default();
        Self {
            indent: default_config.indent,
            start_indent: default_config.start_indent,
            start_indented: default_config.start_indented,
        }
    }
}

#[derive(Deserialize)]
pub struct TomlMD009TrailingSpacesTable {
    #[serde(default = "default_br_spaces")]
    pub br_spaces: usize,
    #[serde(default = "default_false")]
    pub list_item_empty_lines: bool,
    #[serde(default = "default_false")]
    pub strict: bool,
}

impl Default for TomlMD009TrailingSpacesTable {
    fn default() -> Self {
        let default_config = MD009TrailingSpacesTable::default();
        Self {
            br_spaces: default_config.br_spaces,
            list_item_empty_lines: default_config.list_item_empty_lines,
            strict: default_config.strict,
        }
    }
}

#[derive(Deserialize)]
pub struct TomlMD010HardTabsTable {
    #[serde(default = "default_true")]
    pub code_blocks: bool,
    #[serde(default = "default_empty_code_languages")]
    pub ignore_code_languages: Vec<String>,
    #[serde(default = "default_spaces_per_tab")]
    pub spaces_per_tab: usize,
}

impl Default for TomlMD010HardTabsTable {
    fn default() -> Self {
        let default_config = MD010HardTabsTable::default();
        Self {
            code_blocks: default_config.code_blocks,
            ignore_code_languages: default_config.ignore_code_languages,
            spaces_per_tab: default_config.spaces_per_tab,
        }
    }
}

#[derive(Deserialize)]
pub struct TomlMD012MultipleBlankLinesTable {
    #[serde(default = "default_one")]
    pub maximum: usize,
}

impl Default for TomlMD012MultipleBlankLinesTable {
    fn default() -> Self {
        let default_config = MD012MultipleBlankLinesTable::default();
        Self { 
            maximum: default_config.maximum 
        }
    }
}

#[derive(Deserialize)]
pub struct TomlMD013LineLengthTable {
    #[serde(default = "default_line_length")]
    pub line_length: usize,
    #[serde(default = "default_code_block_line_length")]
    pub code_block_line_length: usize,
    #[serde(default = "default_heading_line_length")]
    pub heading_line_length: usize,
    #[serde(default = "default_true")]
    pub code_blocks: bool,
    #[serde(default = "default_true")]
    pub headings: bool,
    #[serde(default = "default_true")]
    pub tables: bool,
    #[serde(default = "default_false")]
    pub strict: bool,
    #[serde(default = "default_false")]
    pub stern: bool,
}

impl Default for TomlMD013LineLengthTable {
    fn default() -> Self {
        let default_config = MD013LineLengthTable::default();
        Self {
            line_length: default_config.line_length,
            code_block_line_length: default_config.code_block_line_length,
            heading_line_length: default_config.heading_line_length,
            code_blocks: default_config.code_blocks,
            headings: default_config.headings,
            tables: default_config.tables,
            strict: default_config.strict,
            stern: default_config.stern,
        }
    }
}

#[derive(Deserialize, Default)]
pub struct TomlMD051LinkFragmentsTable {
    #[serde(default = "default_false")]
    pub ignore_case: bool,
    #[serde(default = "default_empty_string")]
    pub ignored_pattern: String,
}

#[derive(Deserialize, Default)]
pub struct TomlMD052ReferenceLinksImagesTable {
    #[serde(default = "default_false")]
    pub shortcut_syntax: bool,
    #[serde(default = "default_ignored_labels")]
    pub ignored_labels: Vec<String>,
}

#[derive(Deserialize, Default)]
pub struct TomlMD053LinkImageReferenceDefinitionsTable {
    #[serde(default = "default_ignored_definitions")]
    pub ignored_definitions: Vec<String>,
}

#[derive(Deserialize, Default)]
pub struct TomlMD054LinkImageStyleTable {
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

#[derive(Deserialize, Default)]
pub struct TomlMD024MultipleHeadingsTable {
    #[serde(default = "default_false")]
    pub siblings_only: bool,
    #[serde(default = "default_false")]
    pub allow_different_nesting: bool,
}

#[derive(Deserialize)]
pub struct TomlMD025SingleH1Table {
    #[serde(default = "default_level_1")]
    pub level: u8,
    #[serde(default = "default_front_matter_title")]
    pub front_matter_title: String,
}

impl Default for TomlMD025SingleH1Table {
    fn default() -> Self {
        let default_config = MD025SingleH1Table::default();
        Self {
            level: default_config.level,
            front_matter_title: default_config.front_matter_title,
        }
    }
}

#[derive(Deserialize)]
pub struct TomlMD041FirstLineHeadingTable {
    #[serde(default = "default_allow_preamble")]
    pub allow_preamble: bool,
    #[serde(default = "default_front_matter_title")]
    pub front_matter_title: String,
    #[serde(default = "default_level_1")]
    pub level: u8,
}

impl Default for TomlMD041FirstLineHeadingTable {
    fn default() -> Self {
        let default_config = MD041FirstLineHeadingTable::default();
        Self {
            allow_preamble: default_config.allow_preamble,
            front_matter_title: default_config.front_matter_title,
            level: default_config.level,
        }
    }
}

#[derive(Deserialize)]
pub struct TomlMD026TrailingPunctuationTable {
    #[serde(default = "default_trailing_punctuation")]
    pub punctuation: String,
}

impl Default for TomlMD026TrailingPunctuationTable {
    fn default() -> Self {
        Self {
            punctuation: ".,;:!。，；：！".to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct TomlMD027BlockquoteSpacesTable {
    #[serde(default = "default_blockquote_list_items")]
    pub list_items: bool,
}

impl Default for TomlMD027BlockquoteSpacesTable {
    fn default() -> Self {
        let default_config = MD027BlockquoteSpacesTable::default();
        Self { 
            list_items: default_config.list_items 
        }
    }
}

#[derive(Deserialize)]
pub struct TomlMD030ListMarkerSpaceTable {
    #[serde(default = "default_ul_single")]
    pub ul_single: usize,
    #[serde(default = "default_ol_single")]
    pub ol_single: usize,
    #[serde(default = "default_ul_multi")]
    pub ul_multi: usize,
    #[serde(default = "default_ol_multi")]
    pub ol_multi: usize,
}

impl Default for TomlMD030ListMarkerSpaceTable {
    fn default() -> Self {
        let default_config = MD030ListMarkerSpaceTable::default();
        Self {
            ul_single: default_config.ul_single,
            ol_single: default_config.ol_single,
            ul_multi: default_config.ul_multi,
            ol_multi: default_config.ol_multi,
        }
    }
}

#[derive(Deserialize, Default)]
pub struct TomlMD043RequiredHeadingsTable {
    #[serde(default = "default_empty_headings")]
    pub headings: Vec<String>,
    #[serde(default = "default_false")]
    pub match_case: bool,
}

#[derive(Deserialize)]
pub struct TomlMD044ProperNamesTable {
    #[serde(default = "default_empty_vec")]
    pub names: Vec<String>,
    #[serde(default = "default_true")]
    pub code_blocks: bool,
    #[serde(default = "default_true")]
    pub html_elements: bool,
}

impl Default for TomlMD044ProperNamesTable {
    fn default() -> Self {
        Self {
            names: default_empty_vec(),
            code_blocks: default_true(),
            html_elements: default_true(),
        }
    }
}

#[derive(Deserialize, Default)]
pub struct TomlMD022HeadingsBlanksTable {
    #[serde(default = "default_lines_config")]
    pub lines_above: Vec<i32>,
    #[serde(default = "default_lines_config")]
    pub lines_below: Vec<i32>,
}

#[derive(Deserialize)]
pub struct TomlMD031FencedCodeBlanksTable {
    #[serde(default = "default_list_items_true")]
    pub list_items: bool,
}

impl Default for TomlMD031FencedCodeBlanksTable {
    fn default() -> Self {
        Self { list_items: true }
    }
}

#[derive(Deserialize, Default)]
pub struct TomlMD033InlineHtmlTable {
    #[serde(default = "default_empty_vec")]
    pub allowed_elements: Vec<String>,
}

#[derive(Deserialize)]
pub struct TomlMD035HrStyleTable {
    #[serde(default = "default_hr_style")]
    pub style: String,
}

impl Default for TomlMD035HrStyleTable {
    fn default() -> Self {
        Self {
            style: "consistent".to_string(),
        }
    }
}

#[derive(Deserialize, Default)]
pub struct TomlMD040FencedCodeLanguageTable {
    #[serde(default = "default_empty_vec")]
    pub allowed_languages: Vec<String>,
    #[serde(default = "default_false")]
    pub language_only: bool,
}

#[derive(Deserialize, Default)]
pub struct TomlMD036EmphasisAsHeadingTable {
    #[serde(default = "default_md036_punctuation")]
    pub punctuation: String,
}

#[derive(Deserialize)]
pub struct TomlMD059DescriptiveLinkTextTable {
    #[serde(default = "default_prohibited_texts")]
    pub prohibited_texts: Vec<String>,
}

impl Default for TomlMD059DescriptiveLinkTextTable {
    fn default() -> Self {
        Self {
            prohibited_texts: default_prohibited_texts(),
        }
    }
}

// Main TOML configuration structures
#[derive(Deserialize, Default)]
pub struct TomlLintersSettingsTable {
    #[serde(rename = "heading-style")]
    #[serde(default)]
    pub heading_style: TomlMD003HeadingStyleTable,
    #[serde(rename = "ul-style")]
    #[serde(default)]
    pub ul_style: TomlMD004UlStyleTable,
    #[serde(rename = "ol-prefix")]
    #[serde(default)]
    pub ol_prefix: TomlMD029OlPrefixTable,
    #[serde(rename = "ul-indent")]
    #[serde(default)]
    pub ul_indent: TomlMD007UlIndentTable,
    #[serde(rename = "no-trailing-spaces")]
    #[serde(default)]
    pub trailing_spaces: TomlMD009TrailingSpacesTable,
    #[serde(rename = "no-hard-tabs")]
    #[serde(default)]
    pub hard_tabs: TomlMD010HardTabsTable,
    #[serde(rename = "no-multiple-blanks")]
    #[serde(default)]
    pub multiple_blank_lines: TomlMD012MultipleBlankLinesTable,
    #[serde(rename = "line-length")]
    #[serde(default)]
    pub line_length: TomlMD013LineLengthTable,
    #[serde(rename = "blanks-around-headings")]
    #[serde(default)]
    pub headings_blanks: TomlMD022HeadingsBlanksTable,
    #[serde(rename = "single-h1")]
    #[serde(default)]
    pub single_h1: TomlMD025SingleH1Table,
    #[serde(rename = "first-line-heading")]
    #[serde(default)]
    pub first_line_heading: TomlMD041FirstLineHeadingTable,
    #[serde(rename = "no-trailing-punctuation")]
    #[serde(default)]
    pub trailing_punctuation: TomlMD026TrailingPunctuationTable,
    #[serde(rename = "no-multiple-space-blockquote")]
    #[serde(default)]
    pub blockquote_spaces: TomlMD027BlockquoteSpacesTable,
    #[serde(rename = "list-marker-space")]
    #[serde(default)]
    pub list_marker_space: TomlMD030ListMarkerSpaceTable,
    #[serde(rename = "blanks-around-fences")]
    #[serde(default)]
    pub fenced_code_blanks: TomlMD031FencedCodeBlanksTable,
    #[serde(rename = "no-inline-html")]
    #[serde(default)]
    pub inline_html: TomlMD033InlineHtmlTable,
    #[serde(rename = "hr-style")]
    #[serde(default)]
    pub hr_style: TomlMD035HrStyleTable,
    #[serde(rename = "fenced-code-language")]
    #[serde(default)]
    pub fenced_code_language: TomlMD040FencedCodeLanguageTable,
    #[serde(rename = "code-block-style")]
    #[serde(default)]
    pub code_block_style: TomlMD046CodeBlockStyleTable,
    #[serde(rename = "code-fence-style")]
    #[serde(default)]
    pub code_fence_style: TomlMD048CodeFenceStyleTable,
    #[serde(rename = "emphasis-style")]
    #[serde(default)]
    pub emphasis_style: TomlMD049EmphasisStyleTable,
    #[serde(rename = "strong-style")]
    #[serde(default)]
    pub strong_style: TomlMD050StrongStyleTable,
    #[serde(rename = "no-duplicate-heading")]
    #[serde(default)]
    pub multiple_headings: TomlMD024MultipleHeadingsTable,
    #[serde(rename = "required-headings")]
    #[serde(default)]
    pub required_headings: TomlMD043RequiredHeadingsTable,
    #[serde(rename = "proper-names")]
    #[serde(default)]
    pub proper_names: TomlMD044ProperNamesTable,
    #[serde(rename = "link-fragments")]
    #[serde(default)]
    pub link_fragments: TomlMD051LinkFragmentsTable,
    #[serde(rename = "reference-links-images")]
    #[serde(default)]
    pub reference_links_images: TomlMD052ReferenceLinksImagesTable,
    #[serde(rename = "link-image-reference-definitions")]
    #[serde(default)]
    pub link_image_reference_definitions: TomlMD053LinkImageReferenceDefinitionsTable,
    #[serde(rename = "link-image-style")]
    #[serde(default)]
    pub link_image_style: TomlMD054LinkImageStyleTable,
    #[serde(rename = "table-pipe-style")]
    #[serde(default)]
    pub table_pipe_style: TomlMD055TablePipeStyleTable,
    #[serde(rename = "no-emphasis-as-heading")]
    #[serde(default)]
    pub emphasis_as_heading: TomlMD036EmphasisAsHeadingTable,
    #[serde(rename = "descriptive-link-text")]
    #[serde(default)]
    pub descriptive_link_text: TomlMD059DescriptiveLinkTextTable,
}

#[derive(Deserialize, Default)]
pub struct TomlLintersTable {
    #[serde(default)]
    pub severity: HashMap<String, TomlRuleSeverity>,
    #[serde(default)]
    pub settings: TomlLintersSettingsTable,
}

#[derive(Deserialize)]
pub struct TomlQuickmarkConfig {
    #[serde(default)]
    pub linters: TomlLintersTable,
}

// Default implementations for simple TOML structures
impl Default for TomlMD003HeadingStyleTable {
    fn default() -> Self {
        Self {
            style: TomlHeadingStyle::Consistent,
        }
    }
}

impl Default for TomlMD004UlStyleTable {
    fn default() -> Self {
        Self {
            style: TomlUlStyle::Consistent,
        }
    }
}

impl Default for TomlMD029OlPrefixTable {
    fn default() -> Self {
        Self {
            style: TomlOlPrefixStyle::OneOrOrdered,
        }
    }
}

impl Default for TomlMD046CodeBlockStyleTable {
    fn default() -> Self {
        Self {
            style: TomlCodeBlockStyle::Consistent,
        }
    }
}

impl Default for TomlMD048CodeFenceStyleTable {
    fn default() -> Self {
        Self {
            style: TomlCodeFenceStyle::Consistent,
        }
    }
}

impl Default for TomlMD049EmphasisStyleTable {
    fn default() -> Self {
        Self {
            style: TomlEmphasisStyle::Consistent,
        }
    }
}

impl Default for TomlMD050StrongStyleTable {
    fn default() -> Self {
        Self {
            style: TomlStrongStyle::Consistent,
        }
    }
}

impl Default for TomlMD055TablePipeStyleTable {
    fn default() -> Self {
        Self {
            style: TomlTablePipeStyle::Consistent,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::config::{
        HeadingStyle, LintersSettingsTable, LintersTable, MD003HeadingStyleTable,
        MD004UlStyleTable, MD007UlIndentTable, MD009TrailingSpacesTable, MD010HardTabsTable,
        MD012MultipleBlankLinesTable, MD013LineLengthTable, MD022HeadingsBlanksTable,
        MD024MultipleHeadingsTable, MD025SingleH1Table, MD026TrailingPunctuationTable,
        MD027BlockquoteSpacesTable, MD029OlPrefixTable, MD030ListMarkerSpaceTable,
        MD031FencedCodeBlanksTable, MD033InlineHtmlTable, MD035HrStyleTable,
        MD036EmphasisAsHeadingTable, MD040FencedCodeLanguageTable, MD041FirstLineHeadingTable,
        MD043RequiredHeadingsTable, MD044ProperNamesTable, MD046CodeBlockStyleTable,
        MD048CodeFenceStyleTable, MD049EmphasisStyleTable, MD050StrongStyleTable,
        MD051LinkFragmentsTable, MD052ReferenceLinksImagesTable,
        MD053LinkImageReferenceDefinitionsTable, MD054LinkImageStyleTable,
        MD055TablePipeStyleTable, MD059DescriptiveLinkTextTable, RuleSeverity,
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
}
