use anyhow::Context;
use clap::Parser;
use quickmark_config::config_in_path_or_default;
use quickmark_linter::config::{QuickmarkConfig, RuleSeverity};
use quickmark_linter::linter::{MultiRuleLinter, RuleViolation};
use std::cmp::min;
use std::env;
use std::{fs, path::PathBuf, process::exit};

#[derive(Parser, Debug)]
#[command(version, about = "Quickmark: An extremely fast CommonMark linter")]
struct Cli {
    /// Path to the markdown file
    #[arg(required = true)]



    file: PathBuf,
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
        // Convert 0-based line numbers to 1-based for CLI display
        eprintln!(
            "{}: {}:{}:{} {}/{} {}",
            prefix,
            v.location().file_path.to_string_lossy(),
            v.location().range.start.line + 1,
            v.location().range.start.character,
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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let file_path = cli.file;
    let file_content = fs::read_to_string(&file_path)
        .context(format!("Can't read file {}", &file_path.to_string_lossy()))?;

    let pwd = env::current_dir()?;
    let config = config_in_path_or_default(&pwd)?;

    let mut linter = MultiRuleLinter::new_for_document(file_path, config.clone(), &file_content);

    let lint_res = linter.analyze();
    let (errs, _) = print_cli_errors(&lint_res, &config);
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
    use std::path::PathBuf;

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
}
