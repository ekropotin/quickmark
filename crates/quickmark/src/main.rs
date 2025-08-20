use anyhow::Context;
use clap::Parser;
use glob::glob;
use ignore::WalkBuilder;
use quickmark_linter::config::{
    config_from_env_path_or_default, discover_config_or_default, QuickmarkConfig, RuleSeverity,
};
use quickmark_linter::linter::{MultiRuleLinter, RuleViolation};
use rayon::prelude::*;
use std::cmp::min;
use std::env;
use std::path::{Path, PathBuf};
use std::{fs, process::exit};

#[derive(Parser, Debug)]
#[command(version, about = "Quickmark: An extremely fast CommonMark linter")]
struct Cli {
    /// Files, directories, or glob patterns to check
    #[arg(help = "Files, directories, or glob patterns to check [default: .]")]
    files: Vec<PathBuf>,
}

/// Discover markdown files from the given paths
fn discover_markdown_files(paths: &[PathBuf]) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    // If no paths provided, default to current directory
    let search_paths = if paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        paths.to_vec()
    };

    for path in search_paths {
        if path.is_file() {
            // Single file
            if is_markdown_file(&path) {
                files.push(path);
            }
        } else if path.is_dir() {
            // Directory - use ignore crate for efficient directory traversal
            let walker = WalkBuilder::new(&path)
                .hidden(false)
                .git_ignore(true)
                .git_exclude(true)
                .git_global(true)
                .build();

            for entry in walker {
                let entry = entry?;
                let file_path = entry.path();

                if file_path.is_file() && is_markdown_file(file_path) {
                    files.push(file_path.to_path_buf());
                }
            }
        } else {
            // Try as glob pattern
            let pattern = path.to_string_lossy();
            for entry in glob(&pattern)? {
                let file_path = entry?;
                if file_path.is_file() && is_markdown_file(&file_path) {
                    files.push(file_path);
                }
            }
        }
    }

    // Sort files for consistent output
    files.sort();
    files.dedup();

    Ok(files)
}

/// Check if a file is a markdown file based on extension
fn is_markdown_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        matches!(ext.as_str(), "md" | "markdown" | "mdown" | "mkd" | "mkdn")
    } else {
        false
    }
}

/// Print linting errors with 1-based line numbering for CLI display
fn print_cli_errors(results: &[RuleViolation], config: &QuickmarkConfig) -> (i32, i32) {
    let severities = &config.linters.severity;

    let res = results.iter().fold((0, 0), |(errs, warns), v| {
        let severity = severities.get(v.rule().alias).unwrap();
        let prefix;
        let mut new_err = errs;
        let mut new_warns = warns;
        match severity {
            RuleSeverity::Error => {
                prefix = "ERR";
                new_err += 1;
            }
            _ => {
                prefix = "WARN";
                new_warns += 1;
            }
        };
        // Convert 0-based line and character numbers to 1-based for CLI display
        eprintln!(
            "{}: {}:{}:{} {}/{} {}",
            prefix,
            v.location().file_path.to_string_lossy(),
            v.location().range.start.line + 1,
            v.location().range.start.character + 1,
            v.rule().id,
            v.rule().alias,
            v.message()
        );
        (new_err, new_warns)
    });

    println!("\nErrors: {}", res.0);
    println!("Warnings: {}", res.1);
    res
}

/// Lint a single file with hierarchical config discovery and return its violations
fn lint_file_with_config_discovery(
    file_path: &Path,
    use_env_config: bool,
) -> anyhow::Result<Vec<RuleViolation>> {
    let file_content = fs::read_to_string(file_path)
        .context(format!("Can't read file {}", file_path.to_string_lossy()))?;

    // Discover configuration for each file individually for proper hierarchical discovery
    let config = if use_env_config {
        let pwd = env::current_dir()?;
        config_from_env_path_or_default(&pwd)?
    } else {
        discover_config_or_default(file_path)?
    };

    let mut linter =
        MultiRuleLinter::new_for_document(file_path.to_path_buf(), config, &file_content);
    Ok(linter.analyze())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Discover all markdown files to process
    let files = discover_markdown_files(&cli.files)?;

    if files.is_empty() {
        eprintln!("No markdown files found to lint.");
        exit(0);
    }

    // Check if we should use environment config (same for all files) or hierarchical discovery (per file)
    let use_env_config = std::env::var("QUICKMARK_CONFIG").is_ok();

    // Process files in parallel using rayon with per-file config discovery
    let all_violations: Vec<_> = files
        .par_iter()
        .map(|file_path| {
            lint_file_with_config_discovery(file_path, use_env_config).unwrap_or_else(|e| {
                eprintln!("Error linting {}: {}", file_path.display(), e);
                Vec::new()
            })
        })
        .flatten()
        .collect();

    // For error reporting, use a default config (the specific config doesn't matter for display)
    let display_config = if use_env_config {
        let pwd = env::current_dir()?;
        config_from_env_path_or_default(&pwd)?
    } else {
        let default_path = PathBuf::from(".");
        let config_path = files.first().unwrap_or(&default_path);
        discover_config_or_default(config_path)?
    };

    let (errs, _) = print_cli_errors(&all_violations, &display_config);
    let exit_code = min(errs, 1);
    exit(exit_code);
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickmark_linter::config::{HeadingStyle, LintersSettingsTable, MD003HeadingStyleTable};
    use quickmark_linter::linter::{CharPosition, Range};
    use quickmark_linter::rules::{md001::MD001, md003::MD003};
    use quickmark_linter::test_utils::test_helpers::test_config_with_settings;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_print_cli_errors() {
        let config = test_config_with_settings(
            vec![
                ("heading-increment", RuleSeverity::Error),
                ("heading-style", RuleSeverity::Warning),
            ],
            LintersSettingsTable {
                heading_style: MD003HeadingStyleTable {
                    style: HeadingStyle::Consistent,
                },
                ..Default::default()
            },
        );
        let range = Range {
            start: CharPosition {
                line: 1,
                character: 1,
            },
            end: CharPosition {
                line: 1,
                character: 5,
            },
        };
        let file = PathBuf::default();
        let results = vec![
            RuleViolation::new(
                &MD001,
                "all is bad".to_string(),
                file.clone(),
                range.clone(),
            ),
            RuleViolation::new(
                &MD003,
                "all is even worse".to_string(),
                file.clone(),
                range.clone(),
            ),
            RuleViolation::new(
                &MD003,
                "all is even worse2".to_string(),
                file.clone(),
                range,
            ),
        ];

        let (errs, warns) = print_cli_errors(&results, &config);
        assert_eq!(1, errs);
        assert_eq!(2, warns);
    }

    #[test]
    fn test_is_markdown_file() {
        assert!(is_markdown_file(Path::new("test.md")));
        assert!(is_markdown_file(Path::new("test.markdown")));
        assert!(is_markdown_file(Path::new("test.mdown")));
        assert!(is_markdown_file(Path::new("test.mkd")));
        assert!(is_markdown_file(Path::new("test.mkdn")));
        assert!(!is_markdown_file(Path::new("test.txt")));
        assert!(!is_markdown_file(Path::new("test.rs")));
        assert!(!is_markdown_file(Path::new("test")));
    }
}
