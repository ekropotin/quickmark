use anyhow::Context;
use clap::Parser;
use glob::glob;
use ignore::{
    types::TypesBuilder, ParallelVisitor, ParallelVisitorBuilder, WalkBuilder, WalkState,
};
use quickmark_core::config::{
    config_from_env_path_or_default, discover_config_or_default, QuickmarkConfig, RuleSeverity,
};
use quickmark_core::linter::{MultiRuleLinter, RuleViolation};
use rayon::prelude::*;
use std::cmp::min;
use std::env;
use std::path::{Path, PathBuf};
use std::{
    fs,
    process::exit,
    sync::{Arc, Mutex},
};

#[derive(Parser, Debug)]
#[command(version, about = "Quickmark: An extremely fast CommonMark linter")]
struct Cli {
    /// Files, directories, or glob patterns to check
    #[arg(help = "Files, directories, or glob patterns to check [default: .]")]
    files: Vec<PathBuf>,
}

struct FileCollector {
    files: Arc<Mutex<Vec<PathBuf>>>,
}

impl FileCollector {
    fn new(files: Arc<Mutex<Vec<PathBuf>>>) -> Self {
        Self { files }
    }
}

impl ParallelVisitor for FileCollector {
    fn visit(&mut self, entry: Result<ignore::DirEntry, ignore::Error>) -> WalkState {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                // The type filtering in WalkBuilder should ensure we only get markdown files
                if let Ok(mut files) = self.files.lock() {
                    files.push(path.to_path_buf());
                }
            }
        }
        WalkState::Continue
    }
}

/// Builder for FileCollector that implements ParallelVisitorBuilder
struct FileCollectorBuilder {
    files: Arc<Mutex<Vec<PathBuf>>>,
}

impl FileCollectorBuilder {
    fn new(files: Arc<Mutex<Vec<PathBuf>>>) -> Self {
        Self { files }
    }
}

impl<'s> ParallelVisitorBuilder<'s> for FileCollectorBuilder {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
        Box::new(FileCollector::new(Arc::clone(&self.files)))
    }
}

fn discover_markdown_files(paths: &[PathBuf]) -> anyhow::Result<Vec<PathBuf>> {
    let files = Arc::new(Mutex::new(Vec::new()));

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
                files.lock().unwrap().push(path);
            }
        } else if path.is_dir() {
            let mut types_builder = TypesBuilder::new();
            types_builder.add_defaults();
            types_builder.select("markdown");
            let types = types_builder.build()?;

            let walker = WalkBuilder::new(&path)
                .hidden(false)
                .git_ignore(true)
                .git_exclude(true)
                .git_global(true)
                .types(types)
                .build_parallel();

            let mut builder = FileCollectorBuilder::new(Arc::clone(&files));
            walker.visit(&mut builder);
        } else {
            // Try as glob pattern
            let pattern = path.to_string_lossy();
            for entry in glob(&pattern)? {
                let file_path = entry?;
                if file_path.is_file() && is_markdown_file(&file_path) {
                    files.lock().unwrap().push(file_path);
                }
            }
        }
    }

    let files = Arc::try_unwrap(files).unwrap().into_inner().unwrap();
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
fn print_cli_errors(results: &[RuleViolation]) -> (i32, i32) {
    let res = results.iter().fold((0, 0), |(errs, warns), v| {
        let severity = v.severity();
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

/// Lint a single file with a pre-loaded config and return its violations
fn lint_file_with_config(
    file_path: &Path,
    config: &QuickmarkConfig,
) -> anyhow::Result<Vec<RuleViolation>> {
    // Early exit optimization: Check if any rules are enabled before file I/O
    let has_active_rules = config
        .linters
        .severity
        .values()
        .any(|severity| *severity != RuleSeverity::Off);

    if !has_active_rules {
        // No rules are active, skip file reading and processing entirely
        return Ok(Vec::new());
    }

    let file_content = fs::read_to_string(file_path)
        .context(format!("Can't read file {}", file_path.to_string_lossy()))?;

    let mut linter =
        MultiRuleLinter::new_for_document(file_path.to_path_buf(), config.clone(), &file_content);
    Ok(linter.analyze())
}

/// Lint a single file with hierarchical config discovery and return its violations
/// This function is kept for backward compatibility but should be avoided for performance
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

    // Use optimized single config loading only when QUICKMARK_CONFIG is set
    // Otherwise, preserve hierarchical config discovery for correctness
    let (all_violations, _config) = if std::env::var("QUICKMARK_CONFIG").is_ok() {
        // Performance optimization: Load config once when using environment config
        let pwd = env::current_dir()?;
        let config = config_from_env_path_or_default(&pwd)?;

        let violations: Vec<RuleViolation> = files
            .par_iter()
            .map(|file_path| {
                lint_file_with_config(file_path, &config).unwrap_or_else(|e| {
                    eprintln!("Error linting {}: {}", file_path.display(), e);
                    Vec::new()
                })
            })
            .flatten()
            .collect();

        (violations, config)
    } else {
        // Preserve hierarchical config discovery for correctness
        let violations: Vec<RuleViolation> = files
            .par_iter()
            .map(|file_path| {
                lint_file_with_config_discovery(file_path, false).unwrap_or_else(|e| {
                    eprintln!("Error linting {}: {}", file_path.display(), e);
                    Vec::new()
                })
            })
            .flatten()
            .collect();

        // For hierarchical discovery, use default config for error display
        let default_path = PathBuf::from(".");
        let config_path = files.first().unwrap_or(&default_path);
        let config = discover_config_or_default(config_path)?;

        (violations, config)
    };

    let (errs, _) = print_cli_errors(&all_violations);
    let exit_code = min(errs, 1);
    exit(exit_code);
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickmark_core::config::{HeadingStyle, LintersSettingsTable, MD003HeadingStyleTable};
    use quickmark_core::test_utils::test_helpers::test_config_with_settings;
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

        let file_path = PathBuf::from("test.md");
        let file_content = "# Heading 1\n\n### Heading 3\n\nHeading 1\n=========\n";
        let mut linter = MultiRuleLinter::new_for_document(file_path, config.clone(), file_content);
        let results = linter.analyze();

        let (errs, warns) = print_cli_errors(&results);
        assert_eq!(1, errs);
        assert_eq!(1, warns);
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
