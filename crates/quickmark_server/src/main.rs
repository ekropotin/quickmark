use quickmark_config::config_in_path_or_default;
use std::env;

fn main() -> anyhow::Result<()> {
    println!("Quickmark Server starting...");

    // Demonstrate that the server can also use the shared config parsing
    let pwd = env::current_dir()?;
    let config = config_in_path_or_default(&pwd)?;

    println!(
        "Loaded configuration with {} rules",
        config.linters.severity.len()
    );

    // Show some config details
    for (rule, severity) in &config.linters.severity {
        println!("Rule '{}' is set to {:?}", rule, severity);
    }

    println!("Server would run with this configuration...");

    Ok(())
}
