use anyhow::Result;
use quickmark_linter::config::{
    normalize_severities, CodeBlockStyle, CodeFenceStyle, EmphasisStyle, HeadingStyle,
    LintersSettingsTable, LintersTable, MD003HeadingStyleTable, MD007UlIndentTable,
    MD013LineLengthTable, MD022HeadingsBlanksTable, MD024MultipleHeadingsTable, MD025SingleH1Table,
    MD033InlineHtmlTable, MD035HrStyleTable, MD046CodeBlockStyleTable, MD048CodeFenceStyleTable,
    MD049EmphasisStyleTable, QuickmarkConfig, RuleSeverity,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::{fs, path::Path};

#[derive(Deserialize)]
enum TomlRuleSeverity {
    #[serde(rename = "err")]
    Error,
    #[serde(rename = "warn")]
    Warning,
    #[serde(rename = "off")]
    Off,
}

#[derive(Deserialize)]
enum TomlHeadingStyle {
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
enum TomlUlStyle {
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
enum TomlCodeBlockStyle {
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "fenced")]
    Fenced,
    #[serde(rename = "indented")]
    Indented,
}

#[derive(Deserialize)]
enum TomlCodeFenceStyle {
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "backtick")]
    Backtick,
    #[serde(rename = "tilde")]
    Tilde,
}

#[derive(Deserialize)]
enum TomlEmphasisStyle {
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "asterisk")]
    Asterisk,
    #[serde(rename = "underscore")]
    Underscore,
}

#[derive(Deserialize)]
struct TomlMD003HeadingStyleTable {
    style: TomlHeadingStyle,
}

#[derive(Deserialize)]
struct TomlMD004UlStyleTable {
    style: TomlUlStyle,
}

#[derive(Deserialize)]
struct TomlMD046CodeBlockStyleTable {
    style: TomlCodeBlockStyle,
}

#[derive(Deserialize)]
struct TomlMD048CodeFenceStyleTable {
    style: TomlCodeFenceStyle,
}

#[derive(Deserialize)]
struct TomlMD049EmphasisStyleTable {
    style: TomlEmphasisStyle,
}

fn default_indent() -> usize {
    2
}

fn default_br_spaces() -> usize {
    2
}

fn default_spaces_per_tab() -> usize {
    1
}

fn default_one() -> usize {
    1
}

fn default_empty_code_languages() -> Vec<String> {
    Vec::new()
}

#[derive(Deserialize)]
struct TomlMD007UlIndentTable {
    #[serde(default = "default_indent")]
    indent: usize,
    #[serde(default = "default_indent")]
    start_indent: usize,
    #[serde(default = "default_false")]
    start_indented: bool,
}

impl Default for TomlMD007UlIndentTable {
    fn default() -> Self {
        Self {
            indent: 2,
            start_indent: 2,
            start_indented: false,
        }
    }
}

#[derive(Deserialize)]
struct TomlMD009TrailingSpacesTable {
    #[serde(default = "default_br_spaces")]
    br_spaces: usize,
    #[serde(default = "default_false")]
    list_item_empty_lines: bool,
    #[serde(default = "default_false")]
    strict: bool,
}

impl Default for TomlMD009TrailingSpacesTable {
    fn default() -> Self {
        Self {
            br_spaces: 2,
            list_item_empty_lines: false,
            strict: false,
        }
    }
}

#[derive(Deserialize)]
struct TomlMD010HardTabsTable {
    #[serde(default = "default_true")]
    code_blocks: bool,
    #[serde(default = "default_empty_code_languages")]
    ignore_code_languages: Vec<String>,
    #[serde(default = "default_spaces_per_tab")]
    spaces_per_tab: usize,
}

impl Default for TomlMD010HardTabsTable {
    fn default() -> Self {
        Self {
            code_blocks: true,
            ignore_code_languages: Vec::new(),
            spaces_per_tab: 1,
        }
    }
}

#[derive(Deserialize)]
struct TomlMD012MultipleBlankLinesTable {
    #[serde(default = "default_one")]
    maximum: usize,
}

impl Default for TomlMD012MultipleBlankLinesTable {
    fn default() -> Self {
        Self { maximum: 1 }
    }
}

#[derive(Deserialize, Default)]
struct TomlMD013LineLengthTable {
    #[serde(default = "default_line_length")]
    line_length: usize,
    #[serde(default = "default_code_block_line_length")]
    code_block_line_length: usize,
    #[serde(default = "default_heading_line_length")]
    heading_line_length: usize,
    #[serde(default = "default_true")]
    code_blocks: bool,
    #[serde(default = "default_true")]
    headings: bool,
    #[serde(default = "default_true")]
    tables: bool,
    #[serde(default = "default_false")]
    strict: bool,
    #[serde(default = "default_false")]
    stern: bool,
}

fn default_line_length() -> usize {
    80
}
fn default_code_block_line_length() -> usize {
    80
}
fn default_heading_line_length() -> usize {
    80
}
fn default_true() -> bool {
    true
}
fn default_false() -> bool {
    false
}
fn default_empty_string() -> String {
    String::new()
}

#[derive(Deserialize, Default)]
struct TomlMD051LinkFragmentsTable {
    #[serde(default = "default_false")]
    ignore_case: bool,
    #[serde(default = "default_empty_string")]
    ignored_pattern: String,
}

fn default_ignored_labels() -> Vec<String> {
    vec!["x".to_string()]
}
fn default_ignored_definitions() -> Vec<String> {
    vec!["//".to_string()]
}

#[derive(Deserialize, Default)]
struct TomlMD052ReferenceLinksImagesTable {
    #[serde(default = "default_false")]
    shortcut_syntax: bool,
    #[serde(default = "default_ignored_labels")]
    ignored_labels: Vec<String>,
}

#[derive(Deserialize, Default)]
struct TomlMD053LinkImageReferenceDefinitionsTable {
    #[serde(default = "default_ignored_definitions")]
    ignored_definitions: Vec<String>,
}

#[derive(Deserialize, Default)]
struct TomlMD024MultipleHeadingsTable {
    #[serde(default = "default_false")]
    siblings_only: bool,
    #[serde(default = "default_false")]
    allow_different_nesting: bool,
}

fn default_level_1() -> u8 {
    1
}

fn default_front_matter_title() -> String {
    r"^\s*title\s*[:=]".to_string()
}

fn default_allow_preamble() -> bool {
    false
}

fn default_trailing_punctuation() -> String {
    ".,;:!。，；：！".to_string() // Default punctuation without '?'
}

#[derive(Deserialize)]
struct TomlMD025SingleH1Table {
    #[serde(default = "default_level_1")]
    level: u8,
    #[serde(default = "default_front_matter_title")]
    front_matter_title: String,
}

impl Default for TomlMD025SingleH1Table {
    fn default() -> Self {
        Self {
            level: 1,
            front_matter_title: r"^\s*title\s*[:=]".to_string(),
        }
    }
}

#[derive(Deserialize)]
struct TomlMD041FirstLineHeadingTable {
    #[serde(default = "default_allow_preamble")]
    allow_preamble: bool,
    #[serde(default = "default_front_matter_title")]
    front_matter_title: String,
    #[serde(default = "default_level_1")]
    level: u8,
}

impl Default for TomlMD041FirstLineHeadingTable {
    fn default() -> Self {
        Self {
            allow_preamble: false,
            front_matter_title: r"^\s*title\s*[:=]".to_string(),
            level: 1,
        }
    }
}

#[derive(Deserialize)]
struct TomlMD026TrailingPunctuationTable {
    #[serde(default = "default_trailing_punctuation")]
    punctuation: String,
}

impl Default for TomlMD026TrailingPunctuationTable {
    fn default() -> Self {
        Self {
            punctuation: ".,;:!。，；：！".to_string(),
        }
    }
}

#[derive(Deserialize)]
struct TomlMD027BlockquoteSpacesTable {
    #[serde(default = "default_blockquote_list_items")]
    list_items: bool,
}
impl Default for TomlMD027BlockquoteSpacesTable {
    fn default() -> Self {
        Self { list_items: true }
    }
}

fn default_blockquote_list_items() -> bool {
    true
}

#[derive(Deserialize)]
struct TomlMD030ListMarkerSpaceTable {
    #[serde(default = "default_ul_single")]
    ul_single: usize,
    #[serde(default = "default_ol_single")]
    ol_single: usize,
    #[serde(default = "default_ul_multi")]
    ul_multi: usize,
    #[serde(default = "default_ol_multi")]
    ol_multi: usize,
}

impl Default for TomlMD030ListMarkerSpaceTable {
    fn default() -> Self {
        Self {
            ul_single: 1,
            ol_single: 1,
            ul_multi: 1,
            ol_multi: 1,
        }
    }
}

fn default_ul_single() -> usize {
    1
}

fn default_ol_single() -> usize {
    1
}

fn default_ul_multi() -> usize {
    1
}

fn default_ol_multi() -> usize {
    1
}

fn default_lines_config() -> Vec<i32> {
    vec![1]
}

fn default_list_items_true() -> bool {
    true
}

fn default_empty_headings() -> Vec<String> {
    Vec::new()
}

#[derive(Deserialize, Default)]
struct TomlMD043RequiredHeadingsTable {
    #[serde(default = "default_empty_headings")]
    headings: Vec<String>,
    #[serde(default = "default_false")]
    match_case: bool,
}

#[derive(Deserialize, Default)]
struct TomlMD022HeadingsBlanksTable {
    #[serde(default = "default_lines_config")]
    lines_above: Vec<i32>,
    #[serde(default = "default_lines_config")]
    lines_below: Vec<i32>,
}

#[derive(Deserialize)]
struct TomlMD031FencedCodeBlanksTable {
    #[serde(default = "default_list_items_true")]
    list_items: bool,
}

impl Default for TomlMD031FencedCodeBlanksTable {
    fn default() -> Self {
        Self { list_items: true }
    }
}

fn default_empty_vec() -> Vec<String> {
    Vec::new()
}

#[derive(Deserialize, Default)]
struct TomlMD033InlineHtmlTable {
    #[serde(default = "default_empty_vec")]
    allowed_elements: Vec<String>,
}

fn default_hr_style() -> String {
    "consistent".to_string()
}

#[derive(Deserialize)]
struct TomlMD035HrStyleTable {
    #[serde(default = "default_hr_style")]
    style: String,
}

impl Default for TomlMD035HrStyleTable {
    fn default() -> Self {
        Self {
            style: "consistent".to_string(),
        }
    }
}

#[derive(Deserialize, Default)]
struct TomlMD040FencedCodeLanguageTable {
    #[serde(default = "default_empty_vec")]
    allowed_languages: Vec<String>,
    #[serde(default = "default_false")]
    language_only: bool,
}

#[derive(Deserialize, Default)]
struct TomlMD036EmphasisAsHeadingTable {
    #[serde(default = "default_md036_punctuation")]
    punctuation: String,
}

fn default_md036_punctuation() -> String {
    ".,;:!?。，；：！？".to_string()
}

#[derive(Deserialize, Default)]
struct TomlLintersSettingsTable {
    #[serde(rename = "heading-style")]
    #[serde(default)]
    heading_style: TomlMD003HeadingStyleTable,
    #[serde(rename = "ul-style")]
    #[serde(default)]
    ul_style: TomlMD004UlStyleTable,
    #[serde(rename = "ul-indent")]
    #[serde(default)]
    ul_indent: TomlMD007UlIndentTable,
    #[serde(rename = "no-trailing-spaces")]
    #[serde(default)]
    trailing_spaces: TomlMD009TrailingSpacesTable,
    #[serde(rename = "no-hard-tabs")]
    #[serde(default)]
    hard_tabs: TomlMD010HardTabsTable,
    #[serde(rename = "no-multiple-blanks")]
    #[serde(default)]
    multiple_blank_lines: TomlMD012MultipleBlankLinesTable,
    #[serde(rename = "line-length")]
    #[serde(default)]
    line_length: TomlMD013LineLengthTable,
    #[serde(rename = "blanks-around-headings")]
    #[serde(default)]
    headings_blanks: TomlMD022HeadingsBlanksTable,
    #[serde(rename = "single-h1")]
    #[serde(default)]
    single_h1: TomlMD025SingleH1Table,
    #[serde(rename = "first-line-heading")]
    #[serde(default)]
    first_line_heading: TomlMD041FirstLineHeadingTable,
    #[serde(rename = "no-trailing-punctuation")]
    #[serde(default)]
    trailing_punctuation: TomlMD026TrailingPunctuationTable,
    #[serde(rename = "no-multiple-space-blockquote")]
    #[serde(default)]
    blockquote_spaces: TomlMD027BlockquoteSpacesTable,
    #[serde(rename = "list-marker-space")]
    #[serde(default)]
    list_marker_space: TomlMD030ListMarkerSpaceTable,
    #[serde(rename = "blanks-around-fences")]
    #[serde(default)]
    fenced_code_blanks: TomlMD031FencedCodeBlanksTable,
    #[serde(rename = "no-inline-html")]
    #[serde(default)]
    inline_html: TomlMD033InlineHtmlTable,
    #[serde(rename = "hr-style")]
    #[serde(default)]
    hr_style: TomlMD035HrStyleTable,
    #[serde(rename = "fenced-code-language")]
    #[serde(default)]
    fenced_code_language: TomlMD040FencedCodeLanguageTable,
    #[serde(rename = "code-block-style")]
    #[serde(default)]
    code_block_style: TomlMD046CodeBlockStyleTable,
    #[serde(rename = "code-fence-style")]
    #[serde(default)]
    code_fence_style: TomlMD048CodeFenceStyleTable,
    #[serde(rename = "emphasis-style")]
    #[serde(default)]
    emphasis_style: TomlMD049EmphasisStyleTable,
    #[serde(rename = "no-duplicate-heading")]
    #[serde(default)]
    multiple_headings: TomlMD024MultipleHeadingsTable,
    #[serde(rename = "required-headings")]
    #[serde(default)]
    required_headings: TomlMD043RequiredHeadingsTable,
    #[serde(rename = "link-fragments")]
    #[serde(default)]
    link_fragments: TomlMD051LinkFragmentsTable,
    #[serde(rename = "reference-links-images")]
    #[serde(default)]
    reference_links_images: TomlMD052ReferenceLinksImagesTable,
    #[serde(rename = "link-image-reference-definitions")]
    #[serde(default)]
    link_image_reference_definitions: TomlMD053LinkImageReferenceDefinitionsTable,
    #[serde(rename = "no-emphasis-as-heading")]
    #[serde(default)]
    emphasis_as_heading: TomlMD036EmphasisAsHeadingTable,
}

#[derive(Deserialize, Default)]
struct TomlLintersTable {
    #[serde(default)]
    severity: HashMap<String, TomlRuleSeverity>,
    #[serde(default)]
    settings: TomlLintersSettingsTable,
}

#[derive(Deserialize)]
struct TomlQuickmarkConfig {
    #[serde(default)]
    linters: TomlLintersTable,
}

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

fn convert_toml_severity(toml_severity: TomlRuleSeverity) -> RuleSeverity {
    match toml_severity {
        TomlRuleSeverity::Error => RuleSeverity::Error,
        TomlRuleSeverity::Warning => RuleSeverity::Warning,
        TomlRuleSeverity::Off => RuleSeverity::Off,
    }
}

fn convert_toml_heading_style(toml_style: TomlHeadingStyle) -> HeadingStyle {
    match toml_style {
        TomlHeadingStyle::Consistent => HeadingStyle::Consistent,
        TomlHeadingStyle::Atx => HeadingStyle::ATX,
        TomlHeadingStyle::Setext => HeadingStyle::Setext,
        TomlHeadingStyle::ATXClosed => HeadingStyle::ATXClosed,
        TomlHeadingStyle::SetextWithATX => HeadingStyle::SetextWithATX,
        TomlHeadingStyle::SetextWithATXClosed => HeadingStyle::SetextWithATXClosed,
    }
}

fn convert_toml_ul_style(toml_style: TomlUlStyle) -> quickmark_linter::config::UlStyle {
    match toml_style {
        TomlUlStyle::Asterisk => quickmark_linter::config::UlStyle::Asterisk,
        TomlUlStyle::Consistent => quickmark_linter::config::UlStyle::Consistent,
        TomlUlStyle::Dash => quickmark_linter::config::UlStyle::Dash,
        TomlUlStyle::Plus => quickmark_linter::config::UlStyle::Plus,
        TomlUlStyle::Sublist => quickmark_linter::config::UlStyle::Sublist,
    }
}

fn convert_toml_code_block_style(toml_style: TomlCodeBlockStyle) -> CodeBlockStyle {
    match toml_style {
        TomlCodeBlockStyle::Consistent => CodeBlockStyle::Consistent,
        TomlCodeBlockStyle::Fenced => CodeBlockStyle::Fenced,
        TomlCodeBlockStyle::Indented => CodeBlockStyle::Indented,
    }
}

fn convert_toml_code_fence_style(toml_style: TomlCodeFenceStyle) -> CodeFenceStyle {
    match toml_style {
        TomlCodeFenceStyle::Consistent => CodeFenceStyle::Consistent,
        TomlCodeFenceStyle::Backtick => CodeFenceStyle::Backtick,
        TomlCodeFenceStyle::Tilde => CodeFenceStyle::Tilde,
    }
}

fn convert_toml_emphasis_style(toml_style: TomlEmphasisStyle) -> EmphasisStyle {
    match toml_style {
        TomlEmphasisStyle::Consistent => EmphasisStyle::Consistent,
        TomlEmphasisStyle::Asterisk => EmphasisStyle::Asterisk,
        TomlEmphasisStyle::Underscore => EmphasisStyle::Underscore,
    }
}

/// Parse a TOML configuration string into a QuickmarkConfig
pub fn parse_toml_config(config_str: &str) -> Result<QuickmarkConfig> {
    let toml_config: TomlQuickmarkConfig = toml::from_str(config_str)?;
    let mut severity = toml_config
        .linters
        .severity
        .into_iter()
        .map(|(k, v)| (k, convert_toml_severity(v)))
        .collect();

    normalize_severities(&mut severity);

    Ok(QuickmarkConfig::new(LintersTable {
        severity,
        settings: LintersSettingsTable {
            heading_style: MD003HeadingStyleTable {
                style: convert_toml_heading_style(toml_config.linters.settings.heading_style.style),
            },
            ul_style: quickmark_linter::config::MD004UlStyleTable {
                style: convert_toml_ul_style(toml_config.linters.settings.ul_style.style),
            },
            ul_indent: MD007UlIndentTable {
                indent: toml_config.linters.settings.ul_indent.indent,
                start_indent: toml_config.linters.settings.ul_indent.start_indent,
                start_indented: toml_config.linters.settings.ul_indent.start_indented,
            },
            trailing_spaces: quickmark_linter::config::MD009TrailingSpacesTable {
                br_spaces: toml_config.linters.settings.trailing_spaces.br_spaces,
                list_item_empty_lines: toml_config
                    .linters
                    .settings
                    .trailing_spaces
                    .list_item_empty_lines,
                strict: toml_config.linters.settings.trailing_spaces.strict,
            },
            hard_tabs: quickmark_linter::config::MD010HardTabsTable {
                code_blocks: toml_config.linters.settings.hard_tabs.code_blocks,
                ignore_code_languages: toml_config.linters.settings.hard_tabs.ignore_code_languages,
                spaces_per_tab: toml_config.linters.settings.hard_tabs.spaces_per_tab,
            },
            multiple_blank_lines: quickmark_linter::config::MD012MultipleBlankLinesTable {
                maximum: toml_config.linters.settings.multiple_blank_lines.maximum,
            },
            line_length: MD013LineLengthTable {
                line_length: toml_config.linters.settings.line_length.line_length,
                code_block_line_length: toml_config
                    .linters
                    .settings
                    .line_length
                    .code_block_line_length,
                heading_line_length: toml_config.linters.settings.line_length.heading_line_length,
                code_blocks: toml_config.linters.settings.line_length.code_blocks,
                headings: toml_config.linters.settings.line_length.headings,
                tables: toml_config.linters.settings.line_length.tables,
                strict: toml_config.linters.settings.line_length.strict,
                stern: toml_config.linters.settings.line_length.stern,
            },
            headings_blanks: MD022HeadingsBlanksTable {
                lines_above: toml_config.linters.settings.headings_blanks.lines_above,
                lines_below: toml_config.linters.settings.headings_blanks.lines_below,
            },
            single_h1: MD025SingleH1Table {
                level: toml_config.linters.settings.single_h1.level,
                front_matter_title: toml_config.linters.settings.single_h1.front_matter_title,
            },
            first_line_heading: quickmark_linter::config::MD041FirstLineHeadingTable {
                allow_preamble: toml_config
                    .linters
                    .settings
                    .first_line_heading
                    .allow_preamble,
                front_matter_title: toml_config
                    .linters
                    .settings
                    .first_line_heading
                    .front_matter_title,
                level: toml_config.linters.settings.first_line_heading.level,
            },
            trailing_punctuation: quickmark_linter::config::MD026TrailingPunctuationTable {
                punctuation: toml_config
                    .linters
                    .settings
                    .trailing_punctuation
                    .punctuation,
            },
            blockquote_spaces: quickmark_linter::config::MD027BlockquoteSpacesTable {
                list_items: toml_config.linters.settings.blockquote_spaces.list_items,
            },
            list_marker_space: quickmark_linter::config::MD030ListMarkerSpaceTable {
                ul_single: toml_config.linters.settings.list_marker_space.ul_single,
                ol_single: toml_config.linters.settings.list_marker_space.ol_single,
                ul_multi: toml_config.linters.settings.list_marker_space.ul_multi,
                ol_multi: toml_config.linters.settings.list_marker_space.ol_multi,
            },
            fenced_code_blanks: quickmark_linter::config::MD031FencedCodeBlanksTable {
                list_items: toml_config.linters.settings.fenced_code_blanks.list_items,
            },
            inline_html: MD033InlineHtmlTable {
                allowed_elements: toml_config.linters.settings.inline_html.allowed_elements,
            },
            hr_style: MD035HrStyleTable {
                style: toml_config.linters.settings.hr_style.style,
            },
            emphasis_as_heading: quickmark_linter::config::MD036EmphasisAsHeadingTable {
                punctuation: toml_config.linters.settings.emphasis_as_heading.punctuation,
            },
            fenced_code_language: quickmark_linter::config::MD040FencedCodeLanguageTable {
                allowed_languages: toml_config
                    .linters
                    .settings
                    .fenced_code_language
                    .allowed_languages,
                language_only: toml_config
                    .linters
                    .settings
                    .fenced_code_language
                    .language_only,
            },
            code_block_style: MD046CodeBlockStyleTable {
                style: convert_toml_code_block_style(
                    toml_config.linters.settings.code_block_style.style,
                ),
            },
            code_fence_style: MD048CodeFenceStyleTable {
                style: convert_toml_code_fence_style(
                    toml_config.linters.settings.code_fence_style.style,
                ),
            },
            emphasis_style: MD049EmphasisStyleTable {
                style: convert_toml_emphasis_style(
                    toml_config.linters.settings.emphasis_style.style,
                ),
            },
            multiple_headings: MD024MultipleHeadingsTable {
                siblings_only: toml_config.linters.settings.multiple_headings.siblings_only,
                allow_different_nesting: toml_config
                    .linters
                    .settings
                    .multiple_headings
                    .allow_different_nesting,
            },
            link_fragments: quickmark_linter::config::MD051LinkFragmentsTable {
                ignore_case: toml_config.linters.settings.link_fragments.ignore_case,
                ignored_pattern: toml_config.linters.settings.link_fragments.ignored_pattern,
            },
            reference_links_images: quickmark_linter::config::MD052ReferenceLinksImagesTable {
                shortcut_syntax: toml_config
                    .linters
                    .settings
                    .reference_links_images
                    .shortcut_syntax,
                ignored_labels: toml_config
                    .linters
                    .settings
                    .reference_links_images
                    .ignored_labels,
            },
            required_headings: quickmark_linter::config::MD043RequiredHeadingsTable {
                headings: toml_config.linters.settings.required_headings.headings,
                match_case: toml_config.linters.settings.required_headings.match_case,
            },
            link_image_reference_definitions:
                quickmark_linter::config::MD053LinkImageReferenceDefinitionsTable {
                    ignored_definitions: toml_config
                        .linters
                        .settings
                        .link_image_reference_definitions
                        .ignored_definitions,
                },
        },
    }))
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
mod tests {
    use super::*;
    use quickmark_linter::config::{HeadingStyle, RuleSeverity};

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
        assert_eq!(
            quickmark_linter::config::UlStyle::Asterisk,
            parsed.linters.settings.ul_style.style
        );
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
        assert_eq!(
            quickmark_linter::config::UlStyle::Sublist,
            parsed.linters.settings.ul_style.style
        );
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
    fn test_parse_comprehensive_config() {
        let config_str = r#"
        [linters.severity]
        heading-increment = 'warn'
        heading-style = 'err'
        line-length = 'err'
        no-missing-space-closed-atx = 'err'
        no-bare-urls = 'err'
        no-duplicate-heading = 'err'
        no-multiple-space-atx = 'warn'
        no-multiple-space-closed-atx = 'err'
        single-h1 = 'warn'
        link-fragments = 'warn'
        reference-links-images = 'err'
        link-image-reference-definitions = 'warn'

        [linters.settings.heading-style]
        style = 'setext_with_atx_closed'

        [linters.settings.line-length]
        line_length = 120
        code_block_line_length = 100
        heading_line_length = 80
        code_blocks = false
        headings = true
        tables = false
        strict = true
        stern = false

        [linters.settings.blanks-around-headings]
        lines_above = [1, 2, 0]
        lines_below = [1, 1, 2]

        [linters.settings.blanks-around-fences]
        list_items = false

        [linters.settings.no-duplicate-heading]
        siblings_only = true
        allow_different_nesting = false

        [linters.settings.single-h1]
        level = 2
        front_matter_title = '^\s*custom_title\s*:'

        [linters.settings.link-fragments]
        ignore_case = true
        ignored_pattern = "external-.*"

        [linters.settings.reference-links-images]
        shortcut_syntax = true
        ignored_labels = ["custom", "todo", "note"]

        [linters.settings.link-image-reference-definitions]
        ignored_definitions = ["//", "comment", "note"]
        "#;

        let parsed = parse_toml_config(config_str).unwrap();

        // Test all rule severities
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("heading-increment").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("heading-style").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("line-length").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *parsed
                .linters
                .severity
                .get("no-missing-space-closed-atx")
                .unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("no-bare-urls").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("no-duplicate-heading").unwrap()
        );
        assert_eq!(
            RuleSeverity::Warning,
            *parsed
                .linters
                .severity
                .get("no-multiple-space-atx")
                .unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *parsed
                .linters
                .severity
                .get("no-multiple-space-closed-atx")
                .unwrap()
        );
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("single-h1").unwrap()
        );
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("link-fragments").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *parsed
                .linters
                .severity
                .get("reference-links-images")
                .unwrap()
        );
        assert_eq!(
            RuleSeverity::Warning,
            *parsed
                .linters
                .severity
                .get("link-image-reference-definitions")
                .unwrap()
        );

        // Test MD003 (heading-style) settings
        assert_eq!(
            HeadingStyle::SetextWithATXClosed,
            parsed.linters.settings.heading_style.style
        );

        // Test MD013 (line-length) settings
        assert_eq!(120, parsed.linters.settings.line_length.line_length);
        assert_eq!(
            100,
            parsed.linters.settings.line_length.code_block_line_length
        );
        assert_eq!(80, parsed.linters.settings.line_length.heading_line_length);
        assert!(!parsed.linters.settings.line_length.code_blocks);
        assert!(parsed.linters.settings.line_length.headings);
        assert!(!parsed.linters.settings.line_length.tables);
        assert!(parsed.linters.settings.line_length.strict);
        assert!(!parsed.linters.settings.line_length.stern);

        // Test MD022 (blanks-around-headings) settings
        assert_eq!(
            vec![1, 2, 0],
            parsed.linters.settings.headings_blanks.lines_above
        );
        assert_eq!(
            vec![1, 1, 2],
            parsed.linters.settings.headings_blanks.lines_below
        );

        // Test MD031 (blanks-around-fences) settings
        assert!(!parsed.linters.settings.fenced_code_blanks.list_items);

        // Test MD024 (no-duplicate-heading) settings
        assert!(parsed.linters.settings.multiple_headings.siblings_only);
        assert!(
            !parsed
                .linters
                .settings
                .multiple_headings
                .allow_different_nesting
        );

        // Test MD025 (single-h1) settings
        assert_eq!(2, parsed.linters.settings.single_h1.level);
        assert_eq!(
            r"^\s*custom_title\s*:",
            parsed.linters.settings.single_h1.front_matter_title
        );

        // Test MD051 (link-fragments) settings
        assert!(parsed.linters.settings.link_fragments.ignore_case);
        assert_eq!(
            "external-.*",
            parsed.linters.settings.link_fragments.ignored_pattern
        );

        // Test MD052 (reference-links-images) settings
        assert!(
            parsed
                .linters
                .settings
                .reference_links_images
                .shortcut_syntax
        );
        assert_eq!(
            vec!["custom".to_string(), "todo".to_string(), "note".to_string()],
            parsed
                .linters
                .settings
                .reference_links_images
                .ignored_labels
        );

        // Test MD053 (link-image-reference-definitions) settings
        assert_eq!(
            vec!["//".to_string(), "comment".to_string(), "note".to_string()],
            parsed
                .linters
                .settings
                .link_image_reference_definitions
                .ignored_definitions
        );
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

        fs::write(&config_path, config_content).unwrap();

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

    #[test]
    fn test_parse_md043_required_headings_config() {
        let config_str = "
        [linters.severity]
        required-headings = 'err'

        [linters.settings.required-headings]
        headings = [\"# Title\", \"## Section\", \"*\", \"## Conclusion\"]
        match_case = true
        ";

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("required-headings").unwrap()
        );
        assert_eq!(
            vec!["# Title", "## Section", "*", "## Conclusion"],
            parsed.linters.settings.required_headings.headings
        );
        assert!(parsed.linters.settings.required_headings.match_case);
    }

    #[test]
    fn test_parse_md043_default_values() {
        let config_str = r#"
        [linters.severity]
        required-headings = 'warn'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("required-headings").unwrap()
        );
        // Test default values
        assert!(parsed
            .linters
            .settings
            .required_headings
            .headings
            .is_empty());
        assert!(!parsed.linters.settings.required_headings.match_case);
    }

    #[test]
    fn test_parse_md025_single_h1_config() {
        let config_str = r#"
        [linters.severity]
        single-h1 = 'err'

        [linters.settings.single-h1]
        level = 2
        front_matter_title = '^\s*heading\s*:'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("single-h1").unwrap()
        );
        assert_eq!(2, parsed.linters.settings.single_h1.level);
        assert_eq!(
            r"^\s*heading\s*:",
            parsed.linters.settings.single_h1.front_matter_title
        );
    }

    #[test]
    fn test_parse_md025_default_values() {
        let config_str = r#"
        [linters.severity]
        single-h1 = 'warn'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("single-h1").unwrap()
        );
        // Test default values
        assert_eq!(1, parsed.linters.settings.single_h1.level);
        assert_eq!(
            r"^\s*title\s*[:=]",
            parsed.linters.settings.single_h1.front_matter_title
        );
    }

    #[test]
    fn test_parse_md033_inline_html_config() {
        let config_str = r#"
        [linters.severity]
        no-inline-html = 'err'

        [linters.settings.no-inline-html]
        allowed_elements = ["h1", "p", "br", "hr"]
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("no-inline-html").unwrap()
        );
        assert_eq!(
            vec![
                "h1".to_string(),
                "p".to_string(),
                "br".to_string(),
                "hr".to_string()
            ],
            parsed.linters.settings.inline_html.allowed_elements
        );
    }

    #[test]
    fn test_parse_md033_default_values() {
        let config_str = r#"
        [linters.severity]
        no-inline-html = 'warn'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("no-inline-html").unwrap()
        );
        // Test default values
        assert!(parsed
            .linters
            .settings
            .inline_html
            .allowed_elements
            .is_empty());
    }

    #[test]
    fn test_parse_md009_trailing_spaces_config() {
        let config_str = r#"
        [linters.severity]
        no-trailing-spaces = 'err'

        [linters.settings.no-trailing-spaces]
        br_spaces = 4
        list_item_empty_lines = true
        strict = true
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("no-trailing-spaces").unwrap()
        );
        assert_eq!(4, parsed.linters.settings.trailing_spaces.br_spaces);
        assert!(
            parsed
                .linters
                .settings
                .trailing_spaces
                .list_item_empty_lines
        );
        assert!(parsed.linters.settings.trailing_spaces.strict);
    }

    #[test]
    fn test_parse_md009_default_values() {
        let config_str = r#"
        [linters.severity]
        no-trailing-spaces = 'warn'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("no-trailing-spaces").unwrap()
        );
        // Test default values
        assert_eq!(2, parsed.linters.settings.trailing_spaces.br_spaces);
        assert!(
            !parsed
                .linters
                .settings
                .trailing_spaces
                .list_item_empty_lines
        );
        assert!(!parsed.linters.settings.trailing_spaces.strict);
    }

    #[test]
    fn test_parse_md010_hard_tabs_config() {
        let config_str = r#"
        [linters.severity]
        no-hard-tabs = 'err'

        [linters.settings.no-hard-tabs]
        code_blocks = false
        ignore_code_languages = ["python", "javascript", "bash"]
        spaces_per_tab = 4
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("no-hard-tabs").unwrap()
        );
        assert!(!parsed.linters.settings.hard_tabs.code_blocks);
        assert_eq!(
            vec![
                "python".to_string(),
                "javascript".to_string(),
                "bash".to_string()
            ],
            parsed.linters.settings.hard_tabs.ignore_code_languages
        );
        assert_eq!(4, parsed.linters.settings.hard_tabs.spaces_per_tab);
    }

    #[test]
    fn test_parse_md010_default_values() {
        let config_str = r#"
        [linters.severity]
        no-hard-tabs = 'warn'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("no-hard-tabs").unwrap()
        );
        // Test default values
        assert!(parsed.linters.settings.hard_tabs.code_blocks);
        assert!(parsed
            .linters
            .settings
            .hard_tabs
            .ignore_code_languages
            .is_empty());
        assert_eq!(1, parsed.linters.settings.hard_tabs.spaces_per_tab);
    }

    #[test]
    fn test_parse_md046_code_block_style_config() {
        let config_str = r#"
        [linters.severity]
        code-block-style = 'err'

        [linters.settings.code-block-style]
        style = 'fenced'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("code-block-style").unwrap()
        );
        assert_eq!(
            CodeBlockStyle::Fenced,
            parsed.linters.settings.code_block_style.style
        );
    }

    #[test]
    fn test_parse_md046_indented_style_config() {
        let config_str = r#"
        [linters.severity]
        code-block-style = 'warn'

        [linters.settings.code-block-style]
        style = 'indented'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("code-block-style").unwrap()
        );
        assert_eq!(
            CodeBlockStyle::Indented,
            parsed.linters.settings.code_block_style.style
        );
    }

    #[test]
    fn test_parse_md046_default_values() {
        let config_str = r#"
        [linters.severity]
        code-block-style = 'warn'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("code-block-style").unwrap()
        );
        // Test default values
        assert_eq!(
            CodeBlockStyle::Consistent,
            parsed.linters.settings.code_block_style.style
        );
    }

    #[test]
    fn test_parse_md048_code_fence_style_config() {
        let config_str = r#"
        [linters.severity]
        code-fence-style = 'err'

        [linters.settings.code-fence-style]
        style = 'backtick'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("code-fence-style").unwrap()
        );
        assert_eq!(
            CodeFenceStyle::Backtick,
            parsed.linters.settings.code_fence_style.style
        );
    }

    #[test]
    fn test_parse_md048_tilde_style_config() {
        let config_str = r#"
        [linters.severity]
        code-fence-style = 'warn'

        [linters.settings.code-fence-style]
        style = 'tilde'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("code-fence-style").unwrap()
        );
        assert_eq!(
            CodeFenceStyle::Tilde,
            parsed.linters.settings.code_fence_style.style
        );
    }

    #[test]
    fn test_parse_md048_default_values() {
        let config_str = r#"
        [linters.severity]
        code-fence-style = 'warn'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("code-fence-style").unwrap()
        );
        // Test default values
        assert_eq!(
            CodeFenceStyle::Consistent,
            parsed.linters.settings.code_fence_style.style
        );
    }

    #[test]
    fn test_parse_md023_heading_start_left_config() {
        let config_str = r#"
        [linters.severity]
        heading-start-left = 'warn'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("heading-start-left").unwrap()
        );
    }

    #[test]
    fn test_parse_md023_id_config() {
        let config_str = r#"
        [linters.severity]
        heading-start-left = 'err'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("heading-start-left").unwrap()
        );
    }

    #[test]
    fn test_parse_md026_trailing_punctuation_config() {
        let config_str = r#"
        [linters.severity]
        no-trailing-punctuation = 'warn'

        [linters.settings.no-trailing-punctuation]
        punctuation = '.,;:'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed
                .linters
                .severity
                .get("no-trailing-punctuation")
                .unwrap()
        );
        assert_eq!(
            ".,;:".to_string(),
            parsed.linters.settings.trailing_punctuation.punctuation
        );
    }

    #[test]
    fn test_parse_md026_empty_punctuation_config() {
        let config_str = r#"
        [linters.severity]
        no-trailing-punctuation = 'off'

        [linters.settings.no-trailing-punctuation]
        punctuation = ''
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Off,
            *parsed
                .linters
                .severity
                .get("no-trailing-punctuation")
                .unwrap()
        );
        assert_eq!(
            "".to_string(),
            parsed.linters.settings.trailing_punctuation.punctuation
        );
    }

    #[test]
    fn test_parse_md026_default_punctuation_config() {
        let config_str = r#"
        [linters.severity]
        no-trailing-punctuation = 'err'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed
                .linters
                .severity
                .get("no-trailing-punctuation")
                .unwrap()
        );
        // Should use default punctuation when not specified
        assert_eq!(
            ".,;:!。，；：！".to_string(),
            parsed.linters.settings.trailing_punctuation.punctuation
        );
    }

    #[test]
    fn test_parse_md030_list_marker_space_config() {
        let config_str = r#"
        [linters.severity]
        list-marker-space = 'warn'

        [linters.settings.list-marker-space]
        ul_single = 2
        ol_single = 1
        ul_multi = 3
        ol_multi = 2
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("list-marker-space").unwrap()
        );
        assert_eq!(2, parsed.linters.settings.list_marker_space.ul_single);
        assert_eq!(1, parsed.linters.settings.list_marker_space.ol_single);
        assert_eq!(3, parsed.linters.settings.list_marker_space.ul_multi);
        assert_eq!(2, parsed.linters.settings.list_marker_space.ol_multi);
    }

    #[test]
    fn test_parse_md030_list_marker_space_defaults() {
        let config_str = r#"
        [linters.severity]
        list-marker-space = 'err'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("list-marker-space").unwrap()
        );
        // Should use defaults when settings not specified
        assert_eq!(1, parsed.linters.settings.list_marker_space.ul_single);
        assert_eq!(1, parsed.linters.settings.list_marker_space.ol_single);
        assert_eq!(1, parsed.linters.settings.list_marker_space.ul_multi);
        assert_eq!(1, parsed.linters.settings.list_marker_space.ol_multi);
    }

    #[test]
    fn test_parse_md035_hr_style_config() {
        let config_str = r#"
        [linters.severity]
        hr-style = 'err'

        [linters.settings.hr-style]
        style = '---'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("hr-style").unwrap()
        );
        assert_eq!("---".to_string(), parsed.linters.settings.hr_style.style);
    }

    #[test]
    fn test_parse_md035_hr_style_defaults() {
        let config_str = r#"
        [linters.severity]
        hr-style = 'warn'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("hr-style").unwrap()
        );
        // Should use default when setting not specified
        assert_eq!(
            "consistent".to_string(),
            parsed.linters.settings.hr_style.style
        );
    }

    #[test]
    fn test_parse_md039_no_space_in_links_config() {
        let config_str = r#"
        [linters.severity]
        no-space-in-links = 'err'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("no-space-in-links").unwrap()
        );
    }

    #[test]
    fn test_parse_md039_no_space_in_links_warning() {
        let config_str = r#"
        [linters.severity]
        no-space-in-links = 'warn'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("no-space-in-links").unwrap()
        );
    }

    #[test]
    fn test_parse_md042_no_empty_links_config() {
        let config_str = r#"
        [linters.severity]
        no-empty-links = 'err'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("no-empty-links").unwrap()
        );
    }

    #[test]
    fn test_parse_md042_no_empty_links_warning() {
        let config_str = r#"
        [linters.severity]
        no-empty-links = 'warn'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("no-empty-links").unwrap()
        );
    }

    #[test]
    fn test_parse_md045_no_alt_text_config() {
        let config_str = r#"
        [linters.severity]
        no-alt-text = 'err'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("no-alt-text").unwrap()
        );
    }

    #[test]
    fn test_parse_md045_no_alt_text_warning() {
        let config_str = r#"
        [linters.severity]
        no-alt-text = 'warn'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("no-alt-text").unwrap()
        );
    }

    #[test]
    fn test_parse_md047_single_trailing_newline_config() {
        let config_str = r#"
        [linters.severity]
        single-trailing-newline = 'err'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed
                .linters
                .severity
                .get("single-trailing-newline")
                .unwrap()
        );
    }

    #[test]
    fn test_parse_md047_single_trailing_newline_warning() {
        let config_str = r#"
        [linters.severity]
        single-trailing-newline = 'warn'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed
                .linters
                .severity
                .get("single-trailing-newline")
                .unwrap()
        );
    }

    #[test]
    fn test_parse_md049_emphasis_style_asterisk_config() {
        let config_str = r#"
        [linters.severity]
        emphasis-style = 'err'

        [linters.settings.emphasis-style]
        style = 'asterisk'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("emphasis-style").unwrap()
        );
        assert_eq!(
            EmphasisStyle::Asterisk,
            parsed.linters.settings.emphasis_style.style
        );
    }

    #[test]
    fn test_parse_md049_emphasis_style_underscore_config() {
        let config_str = r#"
        [linters.severity]
        emphasis-style = 'warn'

        [linters.settings.emphasis-style]
        style = 'underscore'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("emphasis-style").unwrap()
        );
        assert_eq!(
            EmphasisStyle::Underscore,
            parsed.linters.settings.emphasis_style.style
        );
    }

    #[test]
    fn test_parse_md049_emphasis_style_consistent_config() {
        let config_str = r#"
        [linters.severity]
        emphasis-style = 'err'

        [linters.settings.emphasis-style]
        style = 'consistent'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("emphasis-style").unwrap()
        );
        assert_eq!(
            EmphasisStyle::Consistent,
            parsed.linters.settings.emphasis_style.style
        );
    }

    #[test]
    fn test_parse_md049_emphasis_style_default_values() {
        let config_str = r#"
        [linters.severity]
        emphasis-style = 'warn'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("emphasis-style").unwrap()
        );
        // Test default value
        assert_eq!(
            EmphasisStyle::Consistent,
            parsed.linters.settings.emphasis_style.style
        );
    }
}
