use anyhow::Context;
use clap::Parser;
use quickmark::config::config_in_path_or_default;
use quickmark::linter::MultiRuleLinter;
use quickmark::linter::RuleViolationSeverity::Error;
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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let file_path = cli.file;
    let file_content = fs::read_to_string(&file_path)
        .context(format!("Can't read file {}", &file_path.to_string_lossy()))?;

    let pwd = env::current_dir()?;
    let config = config_in_path_or_default(&pwd)?;

    let context = quickmark::linter::Context {
        file_path: file_path.clone(),
        config,
    };

    let mut linter = MultiRuleLinter::new(context);

    let (warns, errs) = linter
        .lint(&file_content)
        .iter()
        .fold((0, 0), |(warns, errs), v| {
            eprintln!("{}", v);
            match &v.severity {
                Error => (warns, errs + 1),
                _ => (warns + 1, errs),
            }
        });

    println!("\nErrors: {}", errs);
    println!("Warnings: {}", warns);

    let exit_code = min(errs, 1);
    exit(exit_code);
}
