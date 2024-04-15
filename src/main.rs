use anyhow::Context;
use clap::Parser;
use quickmark::linter::{MultiRuleLinter, Settings};
use std::{fs, path::PathBuf};

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

    let rules = vec![quickmark::rules::qm001::QM001];

    let context = quickmark::linter::Context {
        file_path: file_path.clone(),
        settings: Settings {},
    };

    let mut linter = MultiRuleLinter::new(&rules, context);
    linter
        .lint(&file_content)
        .iter()
        .for_each(|v| print!("{}", v));
    Ok(())
}
