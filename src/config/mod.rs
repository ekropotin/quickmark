use serde::Deserialize;
use std::{fs, path::Path};
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

#[derive(Deserialize, Debug, PartialEq)]
pub struct SeveritiesTable {
    #[serde(rename = "heading-increment")]
    pub heading_increment: RuleSeverity,
    #[serde(rename = "heading-style")]
    pub heading_style: RuleSeverity,
}

impl Default for SeveritiesTable {
    fn default() -> Self {
        Self {
            heading_increment: RuleSeverity::Error,
            heading_style: RuleSeverity::Error,
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
    pub severity: SeveritiesTable,
    pub settings: LintersSettingsTable,
}

#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct Config {
    pub linters: LintersTable,
}

pub fn parse_config(config_str: &str) -> anyhow::Result<Config> {
    let res = toml::from_str(config_str)?;
    Ok(res)
}

pub fn config_in_path_or_default(path: &Path) -> anyhow::Result<Config> {
    let config_file = path.join("quickmark.toml");
    if config_file.is_file() {
        let config = fs::read_to_string(config_file)?;
        return parse_config(&config);
    };
    println!(
        "Config file was not found at {}. Default config will be used.",
        config_file.to_string_lossy()
    );
    Ok(Config::default())
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::config::{Config, HeadingStyle, RuleSeverity};

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
            parsed.linters.severity.heading_increment
        );
        assert_eq!(RuleSeverity::Error, parsed.linters.severity.heading_style);
        assert_eq!(
            HeadingStyle::ATX,
            parsed.linters.settings.heading_style.style
        )
    }

    #[test]
    pub fn test_get_config_from_file() {
        let config = config_in_path_or_default(Path::new(FIXTURES_PATH))
            .expect("get_config_in_pwd_or_default() failed");
        assert_eq!(
            RuleSeverity::Warning,
            config.linters.severity.heading_increment
        );
        assert_eq!(RuleSeverity::Off, config.linters.severity.heading_style);
        assert_eq!(
            HeadingStyle::ATX,
            config.linters.settings.heading_style.style
        );
    }

    #[test]
    pub fn test_get_config_default() {
        let config = config_in_path_or_default(Path::new("tests"))
            .expect("get_config_in_pwd_or_default() failed");
        assert_eq!(Config::default(), config);
    }
}
