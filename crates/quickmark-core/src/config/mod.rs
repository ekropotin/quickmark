use anyhow::Result;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::rules::ALL_RULES;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum RuleSeverity {
    #[serde(rename = "err")]
    Error,
    #[serde(rename = "warn")]
    Warning,
    #[serde(rename = "off")]
    Off,
}

pub use crate::rules::md003::{HeadingStyle, MD003HeadingStyleTable};
pub use crate::rules::md004::{MD004UlStyleTable, UlStyle};
pub use crate::rules::md007::MD007UlIndentTable;
pub use crate::rules::md009::MD009TrailingSpacesTable;
pub use crate::rules::md010::MD010HardTabsTable;
pub use crate::rules::md012::MD012MultipleBlankLinesTable;
pub use crate::rules::md013::MD013LineLengthTable;
pub use crate::rules::md022::MD022HeadingsBlanksTable;
pub use crate::rules::md024::MD024MultipleHeadingsTable;
pub use crate::rules::md025::MD025SingleH1Table;
pub use crate::rules::md026::MD026TrailingPunctuationTable;
pub use crate::rules::md027::MD027BlockquoteSpacesTable;
pub use crate::rules::md029::{MD029OlPrefixTable, OlPrefixStyle};
pub use crate::rules::md030::MD030ListMarkerSpaceTable;
pub use crate::rules::md031::MD031FencedCodeBlanksTable;
pub use crate::rules::md033::MD033InlineHtmlTable;
pub use crate::rules::md035::MD035HrStyleTable;
pub use crate::rules::md036::MD036EmphasisAsHeadingTable;
pub use crate::rules::md040::MD040FencedCodeLanguageTable;
pub use crate::rules::md041::MD041FirstLineHeadingTable;
pub use crate::rules::md043::MD043RequiredHeadingsTable;
pub use crate::rules::md044::MD044ProperNamesTable;
pub use crate::rules::md046::{CodeBlockStyle, MD046CodeBlockStyleTable};
pub use crate::rules::md048::{CodeFenceStyle, MD048CodeFenceStyleTable};
pub use crate::rules::md049::{EmphasisStyle, MD049EmphasisStyleTable};
pub use crate::rules::md050::{MD050StrongStyleTable, StrongStyle};
pub use crate::rules::md051::MD051LinkFragmentsTable;
pub use crate::rules::md052::MD052ReferenceLinksImagesTable;
pub use crate::rules::md053::MD053LinkImageReferenceDefinitionsTable;
pub use crate::rules::md054::MD054LinkImageStyleTable;
pub use crate::rules::md055::{MD055TablePipeStyleTable, TablePipeStyle};
pub use crate::rules::md059::MD059DescriptiveLinkTextTable;

#[derive(Debug, Default, PartialEq, Clone, Deserialize)]
pub struct LintersSettingsTable {
    #[serde(rename = "heading-style")]
    #[serde(default)]
    pub heading_style: MD003HeadingStyleTable,
    #[serde(rename = "ul-style")]
    #[serde(default)]
    pub ul_style: MD004UlStyleTable,
    #[serde(rename = "ol-prefix")]
    #[serde(default)]
    pub ol_prefix: MD029OlPrefixTable,
    #[serde(rename = "ul-indent")]
    #[serde(default)]
    pub ul_indent: MD007UlIndentTable,
    #[serde(rename = "no-trailing-spaces")]
    #[serde(default)]
    pub trailing_spaces: MD009TrailingSpacesTable,
    #[serde(rename = "no-hard-tabs")]
    #[serde(default)]
    pub hard_tabs: MD010HardTabsTable,
    #[serde(rename = "no-multiple-blanks")]
    #[serde(default)]
    pub multiple_blank_lines: MD012MultipleBlankLinesTable,
    #[serde(rename = "line-length")]
    #[serde(default)]
    pub line_length: MD013LineLengthTable,
    #[serde(rename = "blanks-around-headings")]
    #[serde(default)]
    pub headings_blanks: MD022HeadingsBlanksTable,
    #[serde(rename = "single-h1")]
    #[serde(default)]
    pub single_h1: MD025SingleH1Table,
    #[serde(rename = "first-line-heading")]
    #[serde(default)]
    pub first_line_heading: MD041FirstLineHeadingTable,
    #[serde(rename = "no-trailing-punctuation")]
    #[serde(default)]
    pub trailing_punctuation: MD026TrailingPunctuationTable,
    #[serde(rename = "no-multiple-space-blockquote")]
    #[serde(default)]
    pub blockquote_spaces: MD027BlockquoteSpacesTable,
    #[serde(rename = "list-marker-space")]
    #[serde(default)]
    pub list_marker_space: MD030ListMarkerSpaceTable,
    #[serde(rename = "blanks-around-fences")]
    #[serde(default)]
    pub fenced_code_blanks: MD031FencedCodeBlanksTable,
    #[serde(rename = "no-inline-html")]
    #[serde(default)]
    pub inline_html: MD033InlineHtmlTable,
    #[serde(rename = "hr-style")]
    #[serde(default)]
    pub hr_style: MD035HrStyleTable,
    #[serde(rename = "no-emphasis-as-heading")]
    #[serde(default)]
    pub emphasis_as_heading: MD036EmphasisAsHeadingTable,
    #[serde(rename = "fenced-code-language")]
    #[serde(default)]
    pub fenced_code_language: MD040FencedCodeLanguageTable,
    #[serde(rename = "code-block-style")]
    #[serde(default)]
    pub code_block_style: MD046CodeBlockStyleTable,
    #[serde(rename = "code-fence-style")]
    #[serde(default)]
    pub code_fence_style: MD048CodeFenceStyleTable,
    #[serde(rename = "emphasis-style")]
    #[serde(default)]
    pub emphasis_style: MD049EmphasisStyleTable,
    #[serde(rename = "strong-style")]
    #[serde(default)]
    pub strong_style: MD050StrongStyleTable,
    #[serde(rename = "no-duplicate-heading")]
    #[serde(default)]
    pub multiple_headings: MD024MultipleHeadingsTable,
    #[serde(rename = "required-headings")]
    #[serde(default)]
    pub required_headings: MD043RequiredHeadingsTable,
    #[serde(rename = "proper-names")]
    #[serde(default)]
    pub proper_names: MD044ProperNamesTable,
    #[serde(rename = "link-fragments")]
    #[serde(default)]
    pub link_fragments: MD051LinkFragmentsTable,
    #[serde(rename = "reference-links-images")]
    #[serde(default)]
    pub reference_links_images: MD052ReferenceLinksImagesTable,
    #[serde(rename = "link-image-reference-definitions")]
    #[serde(default)]
    pub link_image_reference_definitions: MD053LinkImageReferenceDefinitionsTable,
    #[serde(rename = "link-image-style")]
    #[serde(default)]
    pub link_image_style: MD054LinkImageStyleTable,
    #[serde(rename = "table-pipe-style")]
    #[serde(default)]
    pub table_pipe_style: MD055TablePipeStyleTable,
    #[serde(rename = "descriptive-link-text")]
    #[serde(default)]
    pub descriptive_link_text: MD059DescriptiveLinkTextTable,
}

#[derive(Debug, Default, PartialEq, Clone, Deserialize)]
pub struct LintersTable {
    #[serde(default)]
    pub severity: HashMap<String, RuleSeverity>,
    #[serde(default)]
    pub settings: LintersSettingsTable,
}

#[derive(Debug, Default, PartialEq, Clone, Deserialize)]
pub struct QuickmarkConfig {
    #[serde(default)]
    pub linters: LintersTable,
}

pub fn normalize_severities(severities: &mut HashMap<String, RuleSeverity>) {
    let rule_aliases: HashSet<&str> = ALL_RULES.iter().map(|r| r.alias).collect();

    // Extract default severity if present, then remove it from the map
    let default_severity = severities.remove("default").unwrap_or(RuleSeverity::Error);

    // Remove invalid rules (keep only recognized rule aliases)
    severities.retain(|key, _| rule_aliases.contains(key.as_str()));

    // Apply default severity to all rules that don't have explicit configuration
    for &rule in &rule_aliases {
        severities
            .entry(rule.to_string())
            .or_insert(default_severity.clone());
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

/// Result of searching for a configuration file
#[derive(Debug, PartialEq, Clone)]
pub enum ConfigSearchResult {
    /// Configuration file found and successfully parsed
    Found {
        path: PathBuf,
        config: Box<QuickmarkConfig>,
    },
    /// No configuration file found during search
    NotFound { searched_paths: Vec<PathBuf> },
    /// Configuration file found but failed to parse
    Error { path: PathBuf, error: String },
}

/// Hierarchical config discovery with workspace root stopping point
pub struct ConfigDiscovery {
    workspace_roots: Vec<PathBuf>,
}

impl Default for ConfigDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigDiscovery {
    /// Create a new ConfigDiscovery for CLI usage (no workspace roots)
    pub fn new() -> Self {
        Self {
            workspace_roots: Vec::new(),
        }
    }

    /// Create a new ConfigDiscovery for LSP usage with workspace roots
    pub fn with_workspace_roots(roots: Vec<PathBuf>) -> Self {
        Self {
            workspace_roots: roots,
        }
    }

    /// Find configuration file starting from the given file path
    pub fn find_config(&self, file_path: &Path) -> ConfigSearchResult {
        let start_dir = if file_path.is_file() {
            file_path.parent().unwrap_or(file_path)
        } else {
            file_path
        };

        let mut searched_paths = Vec::new();
        let mut current_dir = start_dir;

        loop {
            let config_path = current_dir.join("quickmark.toml");
            searched_paths.push(config_path.clone());

            if config_path.is_file() {
                match fs::read_to_string(&config_path) {
                    Ok(config_str) => match parse_toml_config(&config_str) {
                        Ok(config) => {
                            return ConfigSearchResult::Found {
                                path: config_path,
                                config: Box::new(config),
                            }
                        }
                        Err(e) => {
                            return ConfigSearchResult::Error {
                                path: config_path,
                                error: e.to_string(),
                            }
                        }
                    },
                    Err(e) => {
                        return ConfigSearchResult::Error {
                            path: config_path,
                            error: e.to_string(),
                        }
                    }
                }
            }

            // Check if we should stop searching at this directory
            if self.should_stop_search(current_dir) {
                break;
            }

            // Move to parent directory
            match current_dir.parent() {
                Some(parent) => current_dir = parent,
                None => break, // Reached filesystem root
            }
        }

        ConfigSearchResult::NotFound { searched_paths }
    }

    /// Determine if search should stop at the current directory
    fn should_stop_search(&self, dir: &Path) -> bool {
        // 1. IDE Workspace Root (highest priority)
        for workspace_root in &self.workspace_roots {
            if dir == workspace_root.as_path() {
                return true;
            }
        }

        // 2. Git Repository Root
        if dir.join(".git").exists() {
            return true;
        }

        // 3. Common Project Root Markers
        let project_markers = [
            "package.json",
            "Cargo.toml",
            "pyproject.toml",
            "go.mod",
            ".vscode",
            ".idea",
            ".sublime-project",
        ];

        for marker in &project_markers {
            if dir.join(marker).exists() {
                return true;
            }
        }

        false
    }
}

/// Parse a TOML configuration string into a QuickmarkConfig
pub fn parse_toml_config(config_str: &str) -> Result<QuickmarkConfig> {
    let mut config: QuickmarkConfig = toml::from_str(config_str)?;
    normalize_severities(&mut config.linters.severity);
    Ok(config)
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
    eprintln!(
        "Config file was not found at {}. Default config will be used.",
        config_file.to_string_lossy()
    );
    Ok(QuickmarkConfig::default_with_normalized_severities())
}

/// Convenience function that uses ConfigDiscovery to find config or return default
pub fn discover_config_or_default(file_path: &Path) -> Result<QuickmarkConfig> {
    let discovery = ConfigDiscovery::new();
    match discovery.find_config(file_path) {
        ConfigSearchResult::Found { config, .. } => Ok(*config),
        ConfigSearchResult::NotFound { .. } => {
            Ok(QuickmarkConfig::default_with_normalized_severities())
        }
        ConfigSearchResult::Error { path, error } => {
            eprintln!(
                "Error loading config from {}: {}. Default config will be used.",
                path.to_string_lossy(),
                error
            );
            Ok(QuickmarkConfig::default_with_normalized_severities())
        }
    }
}

/// Convenience function for LSP usage with workspace roots
pub fn discover_config_with_workspace_or_default(
    file_path: &Path,
    workspace_roots: Vec<PathBuf>,
) -> Result<QuickmarkConfig> {
    let discovery = ConfigDiscovery::with_workspace_roots(workspace_roots);
    match discovery.find_config(file_path) {
        ConfigSearchResult::Found { config, .. } => Ok(*config),
        ConfigSearchResult::NotFound { .. } => {
            Ok(QuickmarkConfig::default_with_normalized_severities())
        }
        ConfigSearchResult::Error { path, error } => {
            eprintln!(
                "Error loading config from {}: {}. Default config will be used.",
                path.to_string_lossy(),
                error
            );
            Ok(QuickmarkConfig::default_with_normalized_severities())
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::path::Path;
    use tempfile::TempDir;

    use crate::config::{
        config_from_env_path_or_default, discover_config_or_default,
        discover_config_with_workspace_or_default, parse_toml_config, ConfigDiscovery,
        ConfigSearchResult, HeadingStyle, LintersSettingsTable, LintersTable,
        MD003HeadingStyleTable, MD004UlStyleTable, MD007UlIndentTable, MD009TrailingSpacesTable,
        MD010HardTabsTable, MD012MultipleBlankLinesTable, MD013LineLengthTable,
        MD022HeadingsBlanksTable, MD024MultipleHeadingsTable, MD025SingleH1Table,
        MD026TrailingPunctuationTable, MD027BlockquoteSpacesTable, MD029OlPrefixTable,
        MD030ListMarkerSpaceTable, MD031FencedCodeBlanksTable, MD033InlineHtmlTable,
        MD035HrStyleTable, MD036EmphasisAsHeadingTable, MD040FencedCodeLanguageTable,
        MD041FirstLineHeadingTable, MD043RequiredHeadingsTable, MD044ProperNamesTable,
        MD046CodeBlockStyleTable, MD048CodeFenceStyleTable, MD049EmphasisStyleTable,
        MD050StrongStyleTable, MD051LinkFragmentsTable, MD052ReferenceLinksImagesTable,
        MD053LinkImageReferenceDefinitionsTable, MD054LinkImageStyleTable,
        MD055TablePipeStyleTable, MD059DescriptiveLinkTextTable, RuleSeverity,
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
        assert_eq!(RuleSeverity::Error, *severity.get("list-indent").unwrap());
        assert_eq!(
            RuleSeverity::Error,
            *severity.get("no-reversed-links").unwrap()
        );
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
            RuleSeverity::Error,
            *config.linters.severity.get("list-indent").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *config.linters.severity.get("no-reversed-links").unwrap()
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
                ul_style: MD004UlStyleTable::default(),
                ol_prefix: MD029OlPrefixTable::default(),
                list_marker_space: MD030ListMarkerSpaceTable::default(),
                ul_indent: MD007UlIndentTable::default(),
                trailing_spaces: MD009TrailingSpacesTable::default(),
                hard_tabs: MD010HardTabsTable::default(),
                multiple_blank_lines: MD012MultipleBlankLinesTable::default(),
                line_length: MD013LineLengthTable::default(),
                headings_blanks: MD022HeadingsBlanksTable::default(),
                single_h1: MD025SingleH1Table::default(),
                first_line_heading: MD041FirstLineHeadingTable::default(),
                trailing_punctuation: MD026TrailingPunctuationTable::default(),
                blockquote_spaces: MD027BlockquoteSpacesTable::default(),
                fenced_code_blanks: MD031FencedCodeBlanksTable::default(),
                inline_html: MD033InlineHtmlTable::default(),
                hr_style: MD035HrStyleTable::default(),
                emphasis_as_heading: MD036EmphasisAsHeadingTable::default(),
                fenced_code_language: MD040FencedCodeLanguageTable::default(),
                code_block_style: MD046CodeBlockStyleTable::default(),
                code_fence_style: MD048CodeFenceStyleTable::default(),
                emphasis_style: MD049EmphasisStyleTable::default(),
                strong_style: MD050StrongStyleTable::default(),
                multiple_headings: MD024MultipleHeadingsTable::default(),
                required_headings: MD043RequiredHeadingsTable::default(),
                proper_names: MD044ProperNamesTable::default(),
                link_fragments: MD051LinkFragmentsTable::default(),
                reference_links_images: MD052ReferenceLinksImagesTable::default(),
                link_image_reference_definitions: MD053LinkImageReferenceDefinitionsTable::default(
                ),
                link_image_style: MD054LinkImageStyleTable::default(),
                table_pipe_style: MD055TablePipeStyleTable::default(),
                descriptive_link_text: MD059DescriptiveLinkTextTable::default(),
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
    fn test_config_from_env_fallback_to_local() {
        // Create a local config in a temp directory
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("quickmark.toml");
        let config_content = r#"
        [linters.severity]
        heading-increment = 'err'
        heading-style = 'off'
        "#;

        std::fs::write(&config_path, config_content).unwrap();

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
    fn test_parse_full_config_with_custom_parameters() {
        let config_str = r#"
        [linters.severity]
        heading-style = 'warn'
        ul-style = 'off'
        line-length = 'err'
        
        [linters.settings.heading-style]
        style = 'atx'
        
        [linters.settings.ul-style]
        style = 'asterisk'
        
        [linters.settings.ol-prefix]
        style = 'one'
        
        [linters.settings.ul-indent]
        indent = 4
        start_indent = 3
        start_indented = true
        
        [linters.settings.no-trailing-spaces]
        br_spaces = 3
        list_item_empty_lines = true
        strict = true
        
        [linters.settings.no-hard-tabs]
        code_blocks = false
        ignore_code_languages = ["python", "go"]
        spaces_per_tab = 8
        
        [linters.settings.no-multiple-blanks]
        maximum = 3
        
        [linters.settings.line-length]
        line_length = 120
        code_block_line_length = 100
        heading_line_length = 90
        code_blocks = false
        headings = false
        tables = false
        strict = true
        stern = true
        
        [linters.settings.blanks-around-headings]
        lines_above = [2, 1, 1, 1, 1, 1]
        lines_below = [2, 1, 1, 1, 1, 1]
        
        [linters.settings.single-h1]
        level = 2
        front_matter_title = "^title:"
        
        [linters.settings.first-line-heading]
        allow_preamble = true
        
        [linters.settings.no-trailing-punctuation]
        punctuation = ".,;:!?"
        
        [linters.settings.no-multiple-space-blockquote]
        list_items = false
        
        [linters.settings.list-marker-space]
        ul_single = 2
        ol_single = 3
        ul_multi = 3
        ol_multi = 4
        
        [linters.settings.blanks-around-fences]
        list_items = false
        
        [linters.settings.no-inline-html]
        allowed_elements = ["br", "img"]
        
        [linters.settings.hr-style]
        style = "asterisk"
        
        [linters.settings.no-emphasis-as-heading]
        punctuation = ".,;:!?"
        
        [linters.settings.fenced-code-language]
        allowed_languages = ["rust", "python"]
        language_only = true
        
        [linters.settings.code-block-style]
        style = 'fenced'
        
        [linters.settings.code-fence-style]
        style = 'backtick'
        
        [linters.settings.emphasis-style]
        style = 'asterisk'
        
        [linters.settings.strong-style]
        style = 'underscore'
        
        [linters.settings.no-duplicate-heading]
        siblings_only = false
        allow_different_nesting = false
        
        [linters.settings.required-headings]
        headings = ["Introduction", "Usage", "Examples"]
        match_case = true
        
        [linters.settings.proper-names]
        names = ["JavaScript", "GitHub", "API"]
        code_blocks = false
        html_elements = false
        
        [linters.settings.link-fragments]
        
        [linters.settings.reference-links-images]
        ignored_labels = ["x", "skip"]
        
        [linters.settings.link-image-reference-definitions]
        ignored_definitions = ["//", "skip"]
        
        [linters.settings.link-image-style]
        autolink = false
        inline = true
        full = true
        collapsed = false
        shortcut = false
        url_inline = false
        
        [linters.settings.table-pipe-style]
        style = 'leading_and_trailing'
        
        [linters.settings.descriptive-link-text]
        prohibited_texts = ["click here", "read more", "see here"]
        "#;

        let parsed = parse_toml_config(config_str).unwrap();

        // Verify severities
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("heading-style").unwrap()
        );
        assert_eq!(
            RuleSeverity::Off,
            *parsed.linters.severity.get("ul-style").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("line-length").unwrap()
        );

        // Verify heading-style settings
        assert_eq!(
            HeadingStyle::ATX,
            parsed.linters.settings.heading_style.style
        );

        // Verify ul-style settings
        use crate::rules::md004::UlStyle;
        assert_eq!(UlStyle::Asterisk, parsed.linters.settings.ul_style.style);

        // Verify ul-indent settings
        assert_eq!(4, parsed.linters.settings.ul_indent.indent);
        assert_eq!(3, parsed.linters.settings.ul_indent.start_indent);
        assert!(parsed.linters.settings.ul_indent.start_indented);

        // Verify trailing-spaces settings
        assert_eq!(3, parsed.linters.settings.trailing_spaces.br_spaces);
        assert!(
            parsed
                .linters
                .settings
                .trailing_spaces
                .list_item_empty_lines
        );
        assert!(parsed.linters.settings.trailing_spaces.strict);

        // Verify line-length settings
        assert_eq!(120, parsed.linters.settings.line_length.line_length);
        assert_eq!(
            100,
            parsed.linters.settings.line_length.code_block_line_length
        );
        assert_eq!(90, parsed.linters.settings.line_length.heading_line_length);
        assert!(!parsed.linters.settings.line_length.code_blocks);
        assert!(!parsed.linters.settings.line_length.headings);
        assert!(!parsed.linters.settings.line_length.tables);
        assert!(parsed.linters.settings.line_length.strict);
        assert!(parsed.linters.settings.line_length.stern);

        // Verify single-h1 settings
        assert_eq!(2, parsed.linters.settings.single_h1.level);
        assert_eq!(
            "^title:",
            parsed.linters.settings.single_h1.front_matter_title
        );

        // Verify ol-prefix settings
        use crate::rules::md029::OlPrefixStyle;
        assert_eq!(OlPrefixStyle::One, parsed.linters.settings.ol_prefix.style);

        // Verify hard-tabs settings
        assert!(!parsed.linters.settings.hard_tabs.code_blocks);
        assert_eq!(
            vec!["python", "go"],
            parsed.linters.settings.hard_tabs.ignore_code_languages
        );
        assert_eq!(8, parsed.linters.settings.hard_tabs.spaces_per_tab);

        // Verify multiple-blank-lines settings
        assert_eq!(3, parsed.linters.settings.multiple_blank_lines.maximum);

        // Verify headings-blanks settings
        assert_eq!(
            vec![2, 1, 1, 1, 1, 1],
            parsed.linters.settings.headings_blanks.lines_above
        );
        assert_eq!(
            vec![2, 1, 1, 1, 1, 1],
            parsed.linters.settings.headings_blanks.lines_below
        );

        // Verify first-line-heading settings
        assert!(parsed.linters.settings.first_line_heading.allow_preamble);

        // Verify trailing-punctuation settings
        assert_eq!(
            ".,;:!?",
            parsed.linters.settings.trailing_punctuation.punctuation
        );

        // Verify blockquote-spaces settings
        assert!(!parsed.linters.settings.blockquote_spaces.list_items);

        // Verify list-marker-space settings
        assert_eq!(2, parsed.linters.settings.list_marker_space.ul_single);
        assert_eq!(3, parsed.linters.settings.list_marker_space.ol_single);
        assert_eq!(3, parsed.linters.settings.list_marker_space.ul_multi);
        assert_eq!(4, parsed.linters.settings.list_marker_space.ol_multi);

        // Verify fenced-code-blanks settings
        assert!(!parsed.linters.settings.fenced_code_blanks.list_items);

        // Verify inline-html settings
        assert_eq!(
            vec!["br", "img"],
            parsed.linters.settings.inline_html.allowed_elements
        );

        // Verify hr-style settings
        assert_eq!("asterisk", parsed.linters.settings.hr_style.style);

        // Verify emphasis-as-heading settings
        assert_eq!(
            ".,;:!?",
            parsed.linters.settings.emphasis_as_heading.punctuation
        );

        // Verify fenced-code-language settings
        assert_eq!(
            vec!["rust", "python"],
            parsed
                .linters
                .settings
                .fenced_code_language
                .allowed_languages
        );
        assert!(parsed.linters.settings.fenced_code_language.language_only);

        // Verify code-block-style settings
        use crate::rules::md046::CodeBlockStyle;
        assert_eq!(
            CodeBlockStyle::Fenced,
            parsed.linters.settings.code_block_style.style
        );

        // Verify code-fence-style settings
        use crate::rules::md048::CodeFenceStyle;
        assert_eq!(
            CodeFenceStyle::Backtick,
            parsed.linters.settings.code_fence_style.style
        );

        // Verify emphasis-style settings
        use crate::rules::md049::EmphasisStyle;
        assert_eq!(
            EmphasisStyle::Asterisk,
            parsed.linters.settings.emphasis_style.style
        );

        // Verify strong-style settings
        use crate::rules::md050::StrongStyle;
        assert_eq!(
            StrongStyle::Underscore,
            parsed.linters.settings.strong_style.style
        );

        // Verify multiple-headings settings
        assert!(!parsed.linters.settings.multiple_headings.siblings_only);
        assert!(
            !parsed
                .linters
                .settings
                .multiple_headings
                .allow_different_nesting
        );

        // Verify required-headings settings
        assert_eq!(
            vec!["Introduction", "Usage", "Examples"],
            parsed.linters.settings.required_headings.headings
        );
        assert!(parsed.linters.settings.required_headings.match_case);

        // Verify proper-names settings
        assert_eq!(
            vec!["JavaScript", "GitHub", "API"],
            parsed.linters.settings.proper_names.names
        );
        assert!(!parsed.linters.settings.proper_names.code_blocks);
        assert!(!parsed.linters.settings.proper_names.html_elements);

        // Verify reference-links-images settings
        assert_eq!(
            vec!["x", "skip"],
            parsed
                .linters
                .settings
                .reference_links_images
                .ignored_labels
        );

        // Verify link-image-reference-definitions settings
        assert_eq!(
            vec!["//", "skip"],
            parsed
                .linters
                .settings
                .link_image_reference_definitions
                .ignored_definitions
        );

        // Verify link-image-style settings
        assert!(!parsed.linters.settings.link_image_style.autolink);
        assert!(parsed.linters.settings.link_image_style.inline);
        assert!(parsed.linters.settings.link_image_style.full);
        assert!(!parsed.linters.settings.link_image_style.collapsed);
        assert!(!parsed.linters.settings.link_image_style.shortcut);
        assert!(!parsed.linters.settings.link_image_style.url_inline);

        // Verify table-pipe-style settings
        use crate::rules::md055::TablePipeStyle;
        assert_eq!(
            TablePipeStyle::LeadingAndTrailing,
            parsed.linters.settings.table_pipe_style.style
        );

        // Verify descriptive-link-text settings
        assert_eq!(
            vec!["click here", "read more", "see here"],
            parsed
                .linters
                .settings
                .descriptive_link_text
                .prohibited_texts
        );
    }

    #[test]
    fn test_parse_empty_config_uses_defaults() {
        let config_str = r#"
        # Empty config - should use all defaults
        "#;

        let parsed = parse_toml_config(config_str).unwrap();

        // Verify all rules have Error severity (normalized default)
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("heading-style").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("ul-style").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("line-length").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("ul-indent").unwrap()
        );

        // Verify heading-style defaults
        assert_eq!(
            HeadingStyle::Consistent,
            parsed.linters.settings.heading_style.style
        );

        // Verify ul-style defaults
        use crate::rules::md004::UlStyle;
        assert_eq!(UlStyle::Consistent, parsed.linters.settings.ul_style.style);

        // Verify ul-indent defaults
        assert_eq!(2, parsed.linters.settings.ul_indent.indent);
        assert_eq!(2, parsed.linters.settings.ul_indent.start_indent);
        assert!(!parsed.linters.settings.ul_indent.start_indented);

        // Verify trailing-spaces defaults
        assert_eq!(2, parsed.linters.settings.trailing_spaces.br_spaces);
        assert!(
            !parsed
                .linters
                .settings
                .trailing_spaces
                .list_item_empty_lines
        );
        assert!(!parsed.linters.settings.trailing_spaces.strict);

        // Verify line-length defaults
        assert_eq!(80, parsed.linters.settings.line_length.line_length);
        assert_eq!(
            80,
            parsed.linters.settings.line_length.code_block_line_length
        );
        assert_eq!(80, parsed.linters.settings.line_length.heading_line_length);
        assert!(parsed.linters.settings.line_length.code_blocks);
        assert!(parsed.linters.settings.line_length.headings);
        assert!(parsed.linters.settings.line_length.tables);
        assert!(!parsed.linters.settings.line_length.strict);
        assert!(!parsed.linters.settings.line_length.stern);

        // Verify single-h1 defaults
        assert_eq!(1, parsed.linters.settings.single_h1.level);
        assert_eq!(
            r"^\s*title\s*[:=]",
            parsed.linters.settings.single_h1.front_matter_title
        );

        // Verify ol-prefix defaults
        use crate::rules::md029::OlPrefixStyle;
        assert_eq!(
            OlPrefixStyle::OneOrOrdered,
            parsed.linters.settings.ol_prefix.style
        );

        // Verify multiple-blank-lines defaults
        assert_eq!(1, parsed.linters.settings.multiple_blank_lines.maximum);

        // Verify hard-tabs defaults
        assert_eq!(1, parsed.linters.settings.hard_tabs.spaces_per_tab);
        assert!(parsed.linters.settings.hard_tabs.code_blocks);

        // Verify first-line-heading defaults
        assert!(!parsed.linters.settings.first_line_heading.allow_preamble);
    }

    #[test]
    fn test_default_severity_error() {
        let config_str = r#"
        [linters.severity]
        default = "err"
        heading-style = "warn"
        ul-style = "off"
        "#;

        let parsed = parse_toml_config(config_str).unwrap();

        // Rules with explicit configuration should use that
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("heading-style").unwrap()
        );
        assert_eq!(
            RuleSeverity::Off,
            *parsed.linters.severity.get("ul-style").unwrap()
        );

        // Rules without explicit configuration should use default (Error)
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("line-length").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("ul-indent").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("no-hard-tabs").unwrap()
        );

        // Default should not appear in final severities map
        assert_eq!(None, parsed.linters.severity.get("default"));
    }

    #[test]
    fn test_default_severity_warning() {
        let config_str = r#"
        [linters.severity]
        default = "warn"
        heading-style = "err"
        "#;

        let parsed = parse_toml_config(config_str).unwrap();

        // Explicit rule should use Error
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("heading-style").unwrap()
        );

        // All other rules should use default (Warning)
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("ul-style").unwrap()
        );
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("line-length").unwrap()
        );
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("ul-indent").unwrap()
        );

        // Default should not appear in final severities map
        assert_eq!(None, parsed.linters.severity.get("default"));
    }

    #[test]
    fn test_default_severity_off() {
        let config_str = r#"
        [linters.severity]
        default = "off"
        heading-style = "err"
        line-length = "warn"
        "#;

        let parsed = parse_toml_config(config_str).unwrap();

        // Explicit rules should use their configured severities
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("heading-style").unwrap()
        );
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("line-length").unwrap()
        );

        // All other rules should be disabled (Off)
        assert_eq!(
            RuleSeverity::Off,
            *parsed.linters.severity.get("ul-style").unwrap()
        );
        assert_eq!(
            RuleSeverity::Off,
            *parsed.linters.severity.get("ul-indent").unwrap()
        );
        assert_eq!(
            RuleSeverity::Off,
            *parsed.linters.severity.get("no-hard-tabs").unwrap()
        );

        // Default should not appear in final severities map
        assert_eq!(None, parsed.linters.severity.get("default"));
    }

    #[test]
    fn test_default_severity_with_invalid_rules() {
        let config_str = r#"
        [linters.severity]
        default = "warn"
        heading-style = "err"
        invalid-rule = "off"
        another-invalid = "warn"
        ul-style = "off"
        "#;

        let parsed = parse_toml_config(config_str).unwrap();

        // Valid explicit rules should be preserved
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("heading-style").unwrap()
        );
        assert_eq!(
            RuleSeverity::Off,
            *parsed.linters.severity.get("ul-style").unwrap()
        );

        // Invalid rules should be removed
        assert_eq!(None, parsed.linters.severity.get("invalid-rule"));
        assert_eq!(None, parsed.linters.severity.get("another-invalid"));

        // Valid rules without explicit config should use default
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("line-length").unwrap()
        );
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("ul-indent").unwrap()
        );

        // Default should not appear in final severities map
        assert_eq!(None, parsed.linters.severity.get("default"));
    }

    #[test]
    fn test_no_default_uses_error() {
        let config_str = r#"
        [linters.severity]
        heading-style = "warn"
        ul-style = "off"
        "#;

        let parsed = parse_toml_config(config_str).unwrap();

        // Explicit rules should use their configured severities
        assert_eq!(
            RuleSeverity::Warning,
            *parsed.linters.severity.get("heading-style").unwrap()
        );
        assert_eq!(
            RuleSeverity::Off,
            *parsed.linters.severity.get("ul-style").unwrap()
        );

        // Rules without explicit config should default to Error
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("line-length").unwrap()
        );
        assert_eq!(
            RuleSeverity::Error,
            *parsed.linters.severity.get("ul-indent").unwrap()
        );
    }

    #[test]
    fn test_config_discovery_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.md");

        let discovery = ConfigDiscovery::new();
        let result = discovery.find_config(&file_path);

        match result {
            ConfigSearchResult::NotFound { searched_paths } => {
                assert!(!searched_paths.is_empty());
                // Should have searched in temp_dir
                assert!(searched_paths
                    .iter()
                    .any(|p| p.parent() == Some(temp_dir.path())));
            }
            _ => panic!("Expected NotFound result"),
        }
    }

    #[test]
    fn test_config_discovery_found() {
        let temp_dir = TempDir::new().unwrap();

        // Create a config file
        let config_path = temp_dir.path().join("quickmark.toml");
        let config_content = r#"
        [linters.severity]
        heading-style = 'warn'
        "#;
        std::fs::write(&config_path, config_content).unwrap();

        // Create a file in the same directory
        let file_path = temp_dir.path().join("test.md");
        std::fs::write(&file_path, "# Test").unwrap();

        let discovery = ConfigDiscovery::new();
        let result = discovery.find_config(&file_path);

        match result {
            ConfigSearchResult::Found { path, config } => {
                assert_eq!(path, config_path);
                assert_eq!(
                    *config.linters.severity.get("heading-style").unwrap(),
                    RuleSeverity::Warning
                );
            }
            _ => panic!("Expected Found result, got: {:?}", result),
        }
    }

    #[test]
    fn test_config_discovery_hierarchical_search() {
        let temp_dir = TempDir::new().unwrap();

        // Create nested directories: temp_dir/project/src/
        let project_dir = temp_dir.path().join("project");
        let src_dir = project_dir.join("src");
        std::fs::create_dir_all(&src_dir).unwrap();

        // Create config in project root
        let config_path = project_dir.join("quickmark.toml");
        let config_content = r#"
        [linters.severity]
        heading-style = 'off'
        "#;
        std::fs::write(&config_path, config_content).unwrap();

        // Create a file in src/
        let file_path = src_dir.join("test.md");
        std::fs::write(&file_path, "# Test").unwrap();

        let discovery = ConfigDiscovery::new();
        let result = discovery.find_config(&file_path);

        match result {
            ConfigSearchResult::Found { path, config } => {
                assert_eq!(path, config_path);
                assert_eq!(
                    *config.linters.severity.get("heading-style").unwrap(),
                    RuleSeverity::Off
                );
            }
            _ => panic!("Expected Found result, got: {:?}", result),
        }
    }

    #[test]
    fn test_config_discovery_stops_at_git_root() {
        let temp_dir = TempDir::new().unwrap();

        // Create nested directories: temp_dir/repo/src/
        let repo_dir = temp_dir.path().join("repo");
        let src_dir = repo_dir.join("src");
        std::fs::create_dir_all(&src_dir).unwrap();

        // Create .git directory to mark as repo root
        std::fs::create_dir(repo_dir.join(".git")).unwrap();

        // Create config outside repo (should not be found)
        let outer_config = temp_dir.path().join("quickmark.toml");
        std::fs::write(&outer_config, "[linters.severity]\nheading-style = 'warn'").unwrap();

        // Create file in src/
        let file_path = src_dir.join("test.md");
        std::fs::write(&file_path, "# Test").unwrap();

        let discovery = ConfigDiscovery::new();
        let result = discovery.find_config(&file_path);

        match result {
            ConfigSearchResult::NotFound { searched_paths } => {
                // Should have searched in src/ and repo/ but not in temp_dir (stopped at .git)
                let searched_dirs: Vec<_> =
                    searched_paths.iter().filter_map(|p| p.parent()).collect();
                assert!(searched_dirs.contains(&src_dir.as_path()));
                assert!(searched_dirs.contains(&repo_dir.as_path()));
                assert!(!searched_dirs.contains(&temp_dir.path()));
            }
            _ => panic!("Expected NotFound result, got: {:?}", result),
        }
    }

    #[test]
    fn test_config_discovery_stops_at_workspace_root() {
        let temp_dir = TempDir::new().unwrap();

        // Create nested directories: temp_dir/workspace/project/src/
        let workspace_dir = temp_dir.path().join("workspace");
        let project_dir = workspace_dir.join("project");
        let src_dir = project_dir.join("src");
        std::fs::create_dir_all(&src_dir).unwrap();

        // Create config outside workspace (should not be found)
        let outer_config = temp_dir.path().join("quickmark.toml");
        std::fs::write(&outer_config, "[linters.severity]\nheading-style = 'warn'").unwrap();

        // Create file in src/
        let file_path = src_dir.join("test.md");
        std::fs::write(&file_path, "# Test").unwrap();

        let discovery = ConfigDiscovery::with_workspace_roots(vec![workspace_dir.clone()]);
        let result = discovery.find_config(&file_path);

        match result {
            ConfigSearchResult::NotFound { searched_paths } => {
                // Should have searched in src/, project/, and workspace/ but not temp_dir
                let searched_dirs: Vec<_> =
                    searched_paths.iter().filter_map(|p| p.parent()).collect();
                assert!(searched_dirs.contains(&src_dir.as_path()));
                assert!(searched_dirs.contains(&project_dir.as_path()));
                assert!(searched_dirs.contains(&workspace_dir.as_path()));
                assert!(!searched_dirs.contains(&temp_dir.path()));
            }
            _ => panic!("Expected NotFound result, got: {:?}", result),
        }
    }

    #[test]
    fn test_config_discovery_stops_at_cargo_toml() {
        let temp_dir = TempDir::new().unwrap();

        // Create nested directories: temp_dir/project/src/
        let project_dir = temp_dir.path().join("project");
        let src_dir = project_dir.join("src");
        std::fs::create_dir_all(&src_dir).unwrap();

        // Create Cargo.toml to mark as project root
        std::fs::write(project_dir.join("Cargo.toml"), "[package]\nname = \"test\"").unwrap();

        // Create config outside project (should not be found)
        let outer_config = temp_dir.path().join("quickmark.toml");
        std::fs::write(&outer_config, "[linters.severity]\nheading-style = 'warn'").unwrap();

        // Create file in src/
        let file_path = src_dir.join("test.md");
        std::fs::write(&file_path, "# Test").unwrap();

        let discovery = ConfigDiscovery::new();
        let result = discovery.find_config(&file_path);

        match result {
            ConfigSearchResult::NotFound { searched_paths } => {
                // Should have searched in src/ and project/ but not in temp_dir (stopped at Cargo.toml)
                let searched_dirs: Vec<_> =
                    searched_paths.iter().filter_map(|p| p.parent()).collect();
                assert!(searched_dirs.contains(&src_dir.as_path()));
                assert!(searched_dirs.contains(&project_dir.as_path()));
                assert!(!searched_dirs.contains(&temp_dir.path()));
            }
            _ => panic!("Expected NotFound result, got: {:?}", result),
        }
    }

    #[test]
    fn test_config_discovery_error() {
        let temp_dir = TempDir::new().unwrap();

        // Create invalid config file
        let config_path = temp_dir.path().join("quickmark.toml");
        let invalid_config = "invalid toml content [[[";
        std::fs::write(&config_path, invalid_config).unwrap();

        // Create a file in the same directory
        let file_path = temp_dir.path().join("test.md");
        std::fs::write(&file_path, "# Test").unwrap();

        let discovery = ConfigDiscovery::new();
        let result = discovery.find_config(&file_path);

        match result {
            ConfigSearchResult::Error { path, error } => {
                assert_eq!(path, config_path);
                assert!(error.contains("expected")); // TOML parse error
            }
            _ => panic!("Expected Error result, got: {:?}", result),
        }
    }

    #[test]
    fn test_discover_config_or_default_found() {
        let temp_dir = TempDir::new().unwrap();

        // Create a config file
        let config_path = temp_dir.path().join("quickmark.toml");
        let config_content = r#"
        [linters.severity]
        heading-style = 'warn'
        "#;
        std::fs::write(&config_path, config_content).unwrap();

        // Create a file in the same directory
        let file_path = temp_dir.path().join("test.md");
        std::fs::write(&file_path, "# Test").unwrap();

        let result = discover_config_or_default(&file_path).unwrap();
        assert_eq!(
            *result.linters.severity.get("heading-style").unwrap(),
            RuleSeverity::Warning
        );
    }

    #[test]
    fn test_discover_config_or_default_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.md");
        std::fs::write(&file_path, "# Test").unwrap();

        let result = discover_config_or_default(&file_path).unwrap();
        // Should return default config with normalized severities
        assert_eq!(
            *result.linters.severity.get("heading-style").unwrap(),
            RuleSeverity::Error
        );
    }

    #[test]
    fn test_discover_config_with_workspace_or_default() {
        let temp_dir = TempDir::new().unwrap();

        // Create workspace directory
        let workspace_dir = temp_dir.path().join("workspace");
        let project_dir = workspace_dir.join("project");
        std::fs::create_dir_all(&project_dir).unwrap();

        // Create config in workspace
        let config_path = workspace_dir.join("quickmark.toml");
        let config_content = r#"
        [linters.severity]
        heading-style = 'off'
        "#;
        std::fs::write(&config_path, config_content).unwrap();

        // Create file in project
        let file_path = project_dir.join("test.md");
        std::fs::write(&file_path, "# Test").unwrap();

        let result =
            discover_config_with_workspace_or_default(&file_path, vec![workspace_dir.clone()])
                .unwrap();

        assert_eq!(
            *result.linters.severity.get("heading-style").unwrap(),
            RuleSeverity::Off
        );
    }

    #[test]
    fn test_should_stop_search_workspace_priority() {
        let temp_dir = TempDir::new().unwrap();

        // Create structure: temp_dir/workspace/.git/project/
        let workspace_dir = temp_dir.path().join("workspace");
        let git_dir = workspace_dir.join(".git");
        let project_dir = git_dir.join("project");
        std::fs::create_dir_all(&project_dir).unwrap();

        // ConfigDiscovery with workspace root should stop at workspace, not .git
        let discovery = ConfigDiscovery::with_workspace_roots(vec![workspace_dir.clone()]);

        // Should stop at workspace (highest priority)
        assert!(discovery.should_stop_search(&workspace_dir));
        // Should not stop at .git when workspace root is set
        assert!(!discovery.should_stop_search(&git_dir));
    }
}
