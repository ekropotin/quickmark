use anyhow::Result;
use quickmark_linter::config::{
    normalize_severities, HeadingStyle, LintersSettingsTable, LintersTable, MD003HeadingStyleTable,
    MD013LineLengthTable, MD022HeadingsBlanksTable, MD024MultipleHeadingsTable, QuickmarkConfig,
    RuleSeverity,
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
struct TomlMD003HeadingStyleTable {
    style: TomlHeadingStyle,
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

fn default_lines_config() -> Vec<i32> {
    vec![1]
}

fn default_list_items_true() -> bool {
    true
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

#[derive(Deserialize, Default)]
struct TomlLintersSettingsTable {
    #[serde(rename = "heading-style")]
    #[serde(default)]
    heading_style: TomlMD003HeadingStyleTable,
    #[serde(rename = "line-length")]
    #[serde(default)]
    line_length: TomlMD013LineLengthTable,
    #[serde(rename = "blanks-around-headings")]
    #[serde(default)]
    headings_blanks: TomlMD022HeadingsBlanksTable,
    #[serde(rename = "blanks-around-fences")]
    #[serde(default)]
    fenced_code_blanks: TomlMD031FencedCodeBlanksTable,
    #[serde(rename = "no-duplicate-heading")]
    #[serde(default)]
    multiple_headings: TomlMD024MultipleHeadingsTable,
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
            fenced_code_blanks: quickmark_linter::config::MD031FencedCodeBlanksTable {
                list_items: toml_config.linters.settings.fenced_code_blanks.list_items,
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
}
