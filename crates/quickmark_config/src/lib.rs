use anyhow::Result;
use quickmark_linter::config::{
    normalize_severities, HeadingStyle, LintersSettingsTable, LintersTable, MD003HeadingStyleTable,
    QuickmarkConfig, RuleSeverity,
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
    ATX,
    #[serde(rename = "setext")]
    Setext,
}

#[derive(Deserialize)]
struct TomlMD003HeadingStyleTable {
    style: TomlHeadingStyle,
}

#[derive(Deserialize)]
struct TomlLintersSettingsTable {
    #[serde(rename = "heading-style")]
    heading_style: TomlMD003HeadingStyleTable,
}

#[derive(Deserialize)]
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

impl Default for TomlLintersSettingsTable {
    fn default() -> Self {
        Self {
            heading_style: TomlMD003HeadingStyleTable::default(),
        }
    }
}

impl Default for TomlLintersTable {
    fn default() -> Self {
        Self {
            severity: HashMap::new(),
            settings: TomlLintersSettingsTable::default(),
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
        TomlHeadingStyle::ATX => HeadingStyle::ATX,
        TomlHeadingStyle::Setext => HeadingStyle::Setext,
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
    fn test_parse_toml_config() {
        let config_str = r#"
        [linters.severity]
        heading-increment = 'warn'
        heading-style = 'err'

        [linters.settings.heading-style]
        style = 'atx'
        "#;

        let parsed = parse_toml_config(config_str).unwrap();
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("heading-increment").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("heading-style").unwrap()
        );
        assert_eq!(
            HeadingStyle::ATX,
            parsed.linters.settings.heading_style.style
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
    fn test_default_config() {
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
            HeadingStyle::Consistent,
            config.linters.settings.heading_style.style
        );
    }
}
