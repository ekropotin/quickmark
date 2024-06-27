use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

use crate::rules::ALL_RULES;
#[derive(Deserialize, Debug, PartialEq)]
pub enum RuleSeverity {
    #[serde(rename = "err")]
    Error,
    #[serde(rename = "warn")]
    Warning,
    #[serde(rename = "off")]
    Off,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum HeadingStyle {
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "atx")]
    ATX,
    #[serde(rename = "setext")]
    Setext,
}

#[derive(Deserialize, Debug, PartialEq)]
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

#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct LintersSettingsTable {
    #[serde(rename = "heading-style")]
    pub heading_style: MD003HeadingStyleTable,
}

#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct LintersTable {
    pub severity: HashMap<String, RuleSeverity>,
    pub settings: LintersSettingsTable,
}

#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct QuickmarkConfig {
    pub linters: LintersTable,
}

fn normalize_severities(severities: &mut HashMap<String, RuleSeverity>) {
    let rule_aliases: HashSet<&str> = ALL_RULES.iter().map(|r| r.alias).collect();
    severities.retain(|key, _| rule_aliases.contains(key.as_str()));
    for &rule in &rule_aliases {
        severities
            .entry(rule.to_string())
            .or_insert(RuleSeverity::Error);
    }
}

pub fn parse_config(config_str: &str) -> anyhow::Result<QuickmarkConfig> {
    let mut res: QuickmarkConfig = toml::from_str(config_str)?;
    normalize_severities(&mut res.linters.severity);
    Ok(res)
}

pub fn config_in_path_or_default(path: &Path) -> anyhow::Result<QuickmarkConfig> {
    let config_file = path.join("quickmark.toml");
    if config_file.is_file() {
        let config = fs::read_to_string(config_file)?;
        return parse_config(&config);
    };
    println!(
        "Config file was not found at {}. Default config will be used.",
        config_file.to_string_lossy()
    );
    let mut config = QuickmarkConfig::default();
    normalize_severities(&mut config.linters.severity);
    Ok(config)
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::config::{HeadingStyle, RuleSeverity};

    use super::{config_in_path_or_default, parse_config};

    const FIXTURES_PATH: &str = "tests/fixtures";

    #[test]
    pub fn test_deser() {
        let config = r#"
        [linters.severity]
        heading-increment = 'warn'
        heading-style = 'err'

        [linters.settings.heading-style]
        style = 'atx'
        "#;

        let parsed = parse_config(config).unwrap();
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
        )
    }

    #[test]
    pub fn test_normalize_severities() {
        let config = r#"
        [linters.severity]
        heading-style = 'err'
        some-bullshit = 'warn'

        [linters.settings.heading-style]
        style = 'atx'
        "#;

        let parsed = parse_config(config).unwrap();
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("heading-increment").unwrap()
        );
        assert_eq!(None, parsed.linters.severity.get("some-bullshit"));
    }
    #[test]
    pub fn test_get_config_from_file() {
        let config = config_in_path_or_default(Path::new(FIXTURES_PATH))
            .expect("get_config_in_pwd_or_default() failed");
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

    #[test]
    pub fn test_get_config_default() {
        let config = config_in_path_or_default(Path::new("tests"))
            .expect("get_config_in_pwd_or_default() failed");
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
