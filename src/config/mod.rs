use serde::Deserialize;
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

#[derive(Deserialize, Debug)]
pub struct MD003HeadingStyleTable {
    pub style: HeadingStyle,
}

#[derive(Deserialize, Debug)]
pub struct RulesTable {
    #[serde(rename = "heading-increment")]
    pub heading_increment: RuleSeverity,
    #[serde(rename = "heading-style")]
    pub heading_style: RuleSeverity,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub rules: RulesTable,
    #[serde(rename = "heading-style")]
    pub heading_style: MD003HeadingStyleTable,
}

pub fn parse_config(config_str: &str) -> Result<Config, toml::de::Error> {
    toml::from_str(config_str)
}

#[cfg(test)]
mod test {
    use crate::config::{HeadingStyle, RuleSeverity};

    use super::parse_config;

    #[test]
    pub fn test_deser() {
        let config = r#"
        [rules]
        heading-increment = 'warn'
        heading-style = 'err'

        [heading-style]
        style = 'atx'
        "#;

        let deserialized = parse_config(config).unwrap();
        assert_eq!(RuleSeverity::Warning, deserialized.rules.heading_increment);
        assert_eq!(RuleSeverity::Error, deserialized.rules.heading_style);
        assert_eq!(HeadingStyle::ATX, deserialized.heading_style.style)
    }
}
