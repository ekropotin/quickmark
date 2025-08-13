use anyhow::Result;
use quickmark_linter::config::{
    normalize_severities, HeadingStyle, LintersSettingsTable, LintersTable, MD003HeadingStyleTable,
    MD007UlIndentTable, MD013LineLengthTable, MD022HeadingsBlanksTable, MD024MultipleHeadingsTable,
    MD025SingleH1Table, MD033InlineHtmlTable, QuickmarkConfig, RuleSeverity,
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
struct TomlMD003HeadingStyleTable {
    style: TomlHeadingStyle,
}

#[derive(Deserialize)]
struct TomlMD004UlStyleTable {
    style: TomlUlStyle,
}

fn default_indent() -> usize {
    2
}

fn default_br_spaces() -> usize {
    2
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
    #[serde(rename = "line-length")]
    #[serde(default)]
    line_length: TomlMD013LineLengthTable,
    #[serde(rename = "blanks-around-headings")]
    #[serde(default)]
    headings_blanks: TomlMD022HeadingsBlanksTable,
    #[serde(rename = "single-h1")]
    #[serde(default)]
    single_h1: TomlMD025SingleH1Table,
    #[serde(rename = "blanks-around-fences")]
    #[serde(default)]
    fenced_code_blanks: TomlMD031FencedCodeBlanksTable,
    #[serde(rename = "no-inline-html")]
    #[serde(default)]
    inline_html: TomlMD033InlineHtmlTable,
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
            fenced_code_blanks: quickmark_linter::config::MD031FencedCodeBlanksTable {
                list_items: toml_config.linters.settings.fenced_code_blanks.list_items,
            },
            inline_html: MD033InlineHtmlTable {
                allowed_elements: toml_config.linters.settings.inline_html.allowed_elements,
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
}
