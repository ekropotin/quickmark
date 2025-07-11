use anyhow::Context;
use clap::Parser;
use quickmark_config::config_in_path_or_default;
use quickmark_linter::linter::{print_linting_errors, Context as LintContext, MultiRuleLinter};
use std::cmp::min;
use std::env;
use std::rc::Rc;
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

    let context = Rc::new(LintContext { file_path, config });

    let mut linter = MultiRuleLinter::new(context.clone());

    let lint_res = linter.lint(&file_content);
    let (errs, _) = print_linting_errors(&lint_res, &context.config);
    let exit_code = min(errs, 1);
    exit(exit_code);
}
