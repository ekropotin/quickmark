use anyhow::Context;
use clap::Parser;
use quickmark::config::config_in_path_or_default;
use quickmark::linter::{print_linting_errors, MultiRuleLinter};
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
        config: config.clone(),
    };

    let mut linter = MultiRuleLinter::new(context);

    let lint_res = linter.lint(&file_content);
    let (errs, _) = print_linting_errors(&lint_res, &config);
    let exit_code = min(errs, 1);
    exit(exit_code);
}
