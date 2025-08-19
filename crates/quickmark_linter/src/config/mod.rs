use anyhow::Result;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::{fs, path::Path};

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
    println!(
        "Config file was not found at {}. Default config will be used.",
        config_file.to_string_lossy()
    );
    Ok(QuickmarkConfig::default_with_normalized_severities())
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::path::Path;

    use crate::config::{
        config_from_env_path_or_default, parse_toml_config, HeadingStyle, LintersSettingsTable,
        LintersTable, MD003HeadingStyleTable, MD004UlStyleTable, MD007UlIndentTable,
        MD009TrailingSpacesTable, MD010HardTabsTable, MD012MultipleBlankLinesTable,
        MD013LineLengthTable, MD022HeadingsBlanksTable, MD024MultipleHeadingsTable,
        MD025SingleH1Table, MD026TrailingPunctuationTable, MD027BlockquoteSpacesTable,
        MD029OlPrefixTable, MD030ListMarkerSpaceTable, MD031FencedCodeBlanksTable,
        MD033InlineHtmlTable, MD035HrStyleTable, MD036EmphasisAsHeadingTable,
        MD040FencedCodeLanguageTable, MD041FirstLineHeadingTable, MD043RequiredHeadingsTable,
        MD044ProperNamesTable, MD046CodeBlockStyleTable, MD048CodeFenceStyleTable,
        MD049EmphasisStyleTable, MD050StrongStyleTable, MD051LinkFragmentsTable,
        MD052ReferenceLinksImagesTable, MD053LinkImageReferenceDefinitionsTable,
        MD054LinkImageStyleTable, MD055TablePipeStyleTable, MD059DescriptiveLinkTextTable,
        RuleSeverity,
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
}
