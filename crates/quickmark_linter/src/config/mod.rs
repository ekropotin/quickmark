use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LintersSettingsTable {
    pub heading_style: MD003HeadingStyleTable,
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::config::{
        HeadingStyle, LintersSettingsTable, LintersTable, MD003HeadingStyleTable, RuleSeverity,
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
