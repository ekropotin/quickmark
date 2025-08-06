//! Testing utilities for QuickMark linter rules.
//!
//! This module provides shared functions to create test configurations,
//! avoiding code duplication across individual rule tests.

#[cfg(any(test, feature = "testing"))]
pub mod test_helpers {
    use crate::config::{LintersSettingsTable, LintersTable, QuickmarkConfig, RuleSeverity};
    use std::collections::HashMap;

    /// Creates a basic test configuration with only the specified rules enabled.
    ///
    /// # Arguments
    /// * `enabled_rules` - Vector of (rule_name, severity) tuples to enable
    ///
    /// # Example
    /// ```
    /// use quickmark_linter::test_utils::test_helpers::test_config_with_rules;
    /// use quickmark_linter::config::RuleSeverity;
    ///
    /// let config = test_config_with_rules(vec![
    ///     ("heading-increment", RuleSeverity::Error),
    ///     ("heading-style", RuleSeverity::Warning),
    /// ]);
    /// ```
    pub fn test_config_with_rules(enabled_rules: Vec<(&str, RuleSeverity)>) -> QuickmarkConfig {
        let severity: HashMap<String, RuleSeverity> = enabled_rules
            .into_iter()
            .map(|(rule, sev)| (rule.to_string(), sev))
            .collect();

        QuickmarkConfig {
            linters: LintersTable {
                severity,
                settings: LintersSettingsTable::default(),
            },
        }
    }

    /// Creates a test configuration with custom settings.
    ///
    /// # Arguments
    /// * `enabled_rules` - Vector of (rule_name, severity) tuples to enable
    /// * `settings` - Custom linter settings
    ///
    /// # Example
    /// ```
    /// use quickmark_linter::test_utils::test_helpers::test_config_with_settings;
    /// use quickmark_linter::config::{RuleSeverity, LintersSettingsTable, MD003HeadingStyleTable, HeadingStyle};
    ///
    /// let config = test_config_with_settings(
    ///     vec![("heading-style", RuleSeverity::Error)],
    ///     LintersSettingsTable {
    ///         heading_style: MD003HeadingStyleTable { style: HeadingStyle::ATX },
    ///         ..Default::default()
    ///     }
    /// );
    /// ```
    pub fn test_config_with_settings(
        enabled_rules: Vec<(&str, RuleSeverity)>,
        settings: LintersSettingsTable,
    ) -> QuickmarkConfig {
        let severity: HashMap<String, RuleSeverity> = enabled_rules
            .into_iter()
            .map(|(rule, sev)| (rule.to_string(), sev))
            .collect();

        QuickmarkConfig {
            linters: LintersTable { severity, settings },
        }
    }
}
