use anyhow::Result;
use quickmark_linter::config::{
    normalize_severities, HeadingStyle, LintersSettingsTable, LintersTable, MD003HeadingStyleTable, MD013LineLengthTable, QuickmarkConfig, RuleSeverity,
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

#[derive(Deserialize)]
#[derive(Default)]
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

fn default_line_length() -> usize { 80 }
fn default_code_block_line_length() -> usize { 80 }
fn default_heading_line_length() -> usize { 80 }
fn default_true() -> bool { true }
fn default_false() -> bool { false }
fn default_empty_string() -> String { String::new() }

#[derive(Deserialize, Default)]
struct TomlMD051LinkFragmentsTable {
    #[serde(default = "default_false")]
    ignore_case: bool,
    #[serde(default = "default_empty_string")]
    ignored_pattern: String,
}

fn default_ignored_labels() -> Vec<String> { vec!["x".to_string()] }

#[derive(Deserialize, Default)]
struct TomlMD052ReferenceLinksImagesTable {
    #[serde(default = "default_false")]
    shortcut_syntax: bool,
    #[serde(default = "default_ignored_labels")]
    ignored_labels: Vec<String>,
}

#[derive(Deserialize)]
#[derive(Default)]
struct TomlLintersSettingsTable {
    #[serde(rename = "heading-style")]
    #[serde(default)]
    heading_style: TomlMD003HeadingStyleTable,
    #[serde(rename = "line-length")]
    #[serde(default)]
    line_length: TomlMD013LineLengthTable,
    #[serde(rename = "link-fragments")]
    #[serde(default)]
    link_fragments: TomlMD051LinkFragmentsTable,
    #[serde(rename = "reference-links-images")]
    #[serde(default)]
    reference_links_images: TomlMD052ReferenceLinksImagesTable,
}

#[derive(Deserialize)]
#[derive(Default)]
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
                code_block_line_length: toml_config.linters.settings.line_length.code_block_line_length,
                heading_line_length: toml_config.linters.settings.line_length.heading_line_length,
                code_blocks: toml_config.linters.settings.line_length.code_blocks,
                headings: toml_config.linters.settings.line_length.headings,
                tables: toml_config.linters.settings.line_length.tables,
                strict: toml_config.linters.settings.line_length.strict,
                stern: toml_config.linters.settings.line_length.stern,
            },
            link_fragments: quickmark_linter::config::MD051LinkFragmentsTable {
                ignore_case: toml_config.linters.settings.link_fragments.ignore_case,
                ignored_pattern: toml_config.linters.settings.link_fragments.ignored_pattern,
            },
            reference_links_images: quickmark_linter::config::MD052ReferenceLinksImagesTable {
                shortcut_syntax: toml_config.linters.settings.reference_links_images.shortcut_syntax,
                ignored_labels: toml_config.linters.settings.reference_links_images.ignored_labels,
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
        link-fragments = 'warn'
        reference-links-images = 'err'

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

        [linters.settings.link-fragments]
        ignore_case = true
        ignored_pattern = "external-.*"

        [linters.settings.reference-links-images]
        shortcut_syntax = true
        ignored_labels = ["custom", "todo", "note"]
        "#;

        let parsed = parse_toml_config(config_str).unwrap();

        // Test all rule severities
        assert_eq!(RuleSeverity::Warning, *parsed.linters.severity.get("heading-increment").unwrap());
        assert_eq!(RuleSeverity::Error, *parsed.linters.severity.get("heading-style").unwrap());
        assert_eq!(RuleSeverity::Error, *parsed.linters.severity.get("line-length").unwrap());
        assert_eq!(RuleSeverity::Warning, *parsed.linters.severity.get("link-fragments").unwrap());
        assert_eq!(RuleSeverity::Error, *parsed.linters.severity.get("reference-links-images").unwrap());

        // Test MD003 (heading-style) settings
        assert_eq!(HeadingStyle::SetextWithATXClosed, parsed.linters.settings.heading_style.style);

        // Test MD013 (line-length) settings
        assert_eq!(120, parsed.linters.settings.line_length.line_length);
        assert_eq!(100, parsed.linters.settings.line_length.code_block_line_length);
        assert_eq!(80, parsed.linters.settings.line_length.heading_line_length);
        assert!(!parsed.linters.settings.line_length.code_blocks);
        assert!(parsed.linters.settings.line_length.headings);
        assert!(!parsed.linters.settings.line_length.tables);
        assert!(parsed.linters.settings.line_length.strict);
        assert!(!parsed.linters.settings.line_length.stern);

        // Test MD051 (link-fragments) settings
        assert!(parsed.linters.settings.link_fragments.ignore_case);
        assert_eq!("external-.*", parsed.linters.settings.link_fragments.ignored_pattern);

        // Test MD052 (reference-links-images) settings
        assert!(parsed.linters.settings.reference_links_images.shortcut_syntax);
        assert_eq!(
            vec!["custom".to_string(), "todo".to_string(), "note".to_string()],
            parsed.linters.settings.reference_links_images.ignored_labels
        );
    }
}
