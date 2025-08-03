use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::path::PathBuf;

/// Helper function to get the path to test sample files
fn test_sample_path(filename: &str) -> String {
    // Use the CARGO_MANIFEST_DIR environment variable to find the project root
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");

    PathBuf::from(manifest_dir)
        .parent() // Go up from crates/quickmark
        .unwrap()
        .parent() // Go up from crates
        .unwrap()
        .join("test-samples") // Join with test-samples
        .join(filename)
        .to_string_lossy()
        .to_string()
}

/// Test the CLI with a file that has no MD001 violations but has MD003 violations
#[test]
fn test_cli_no_md001_violations() {
    let mut cmd = Command::cargo_bin("quickmark").unwrap();
    cmd.arg(test_sample_path("test_md001_valid.md"));

    cmd.assert()
        .failure() // Should fail due to MD003 violations (mixed styles)
        .stderr(predicates::str::contains("MD003"))
        .stderr(predicates::str::contains("heading-style"))
        .stderr(predicates::str::contains("ERR:"))
        .stdout(predicates::str::contains("Errors: 2"))
        .stdout(predicates::str::contains("Warnings: 0"));
}

/// Test the CLI with a file that has MD001 violations
#[test]
fn test_cli_md001_violations() {
    let mut cmd = Command::cargo_bin("quickmark").unwrap();
    cmd.arg(test_sample_path("test_md001_violations.md"));

    cmd.assert()
        .failure() // Should fail due to violations
        .stderr(predicates::str::contains("MD001"))
        .stderr(predicates::str::contains("heading-increment"))
        .stderr(predicates::str::contains("ERR:"))
        .stdout(predicates::str::contains("Errors:"))
        .stdout(predicates::str::contains("Errors: 0").not());
}

/// Test the CLI with a file that has MD003 violations
#[test]
fn test_cli_md003_violations() {
    let mut cmd = Command::cargo_bin("quickmark").unwrap();
    cmd.arg(test_sample_path("test_md003_mixed_styles.md"));

    cmd.assert()
        .failure() // Should fail due to violations
        .stderr(predicates::str::contains("MD003"))
        .stderr(predicates::str::contains("heading-style"))
        .stderr(predicates::str::contains("ERR:"))
        .stdout(predicates::str::contains("Errors:"))
        .stdout(predicates::str::contains("Errors: 0").not());
}

/// Test the CLI with a comprehensive file that triggers all rules
#[test]
fn test_cli_all_rules_violations() {
    let mut cmd = Command::cargo_bin("quickmark").unwrap();
    cmd.arg(test_sample_path("test_all_rules_violations.md"));

    cmd.assert()
        .failure() // Should fail due to violations
        .stderr(predicates::str::contains("MD001"))
        .stderr(predicates::str::contains("heading-increment"))
        .stderr(predicates::str::contains("MD003"))
        .stderr(predicates::str::contains("heading-style"))
        .stderr(predicates::str::contains("ERR:"))
        .stdout(predicates::str::contains("Errors:"))
        .stdout(predicates::str::contains("Errors: 0").not());
}

/// Test the CLI with a non-existent file
#[test]
fn test_cli_nonexistent_file() {
    let mut cmd = Command::cargo_bin("quickmark").unwrap();
    cmd.arg("nonexistent_file.md");

    cmd.assert()
        .failure() // Should fail due to missing file
        .stderr(predicates::str::contains("Can't read file"));
}

/// Test CLI error output format
#[test]
fn test_cli_error_format() {
    let mut cmd = Command::cargo_bin("quickmark").unwrap();
    cmd.arg(test_sample_path("test_md001_violations.md"));

    let output = cmd.assert().failure().get_output().clone();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Check that error format includes expected components:
    // ERR: file_path:line:column MD001/heading-increment message
    let error_lines: Vec<&str> = stderr.lines()
        .filter(|line| line.starts_with("ERR:") || line.starts_with("WARN:"))
        .collect();

    assert!(!error_lines.is_empty());

    for error_line in error_lines {
        // Should have format: ERR: file:line:column MD001/heading-increment message
        assert!(error_line.contains("test_md001_violations.md"));
        assert!(error_line.contains(":"));
        // Should contain either MD001 or MD003
        assert!(error_line.contains("MD001") || error_line.contains("MD003"));
    }
}

/// Test CLI with different configurations using temporary files
#[test]
fn test_cli_with_custom_config() {
    let temp_dir = TempDir::new().unwrap();

    // Create a temporary config file
    let config_content = r#"
[linters.severity]
heading-increment = 'off'
heading-style = 'err'

[linters.settings.heading-style]
style = 'atx'
"#;

    let config_file = temp_dir.child("quickmark.toml");
    config_file.write_str(config_content).unwrap();

    let mut cmd = Command::cargo_bin("quickmark").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg(test_sample_path("test_md003_mixed_styles.md"));

    let output = cmd.assert().failure().get_output().clone();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // MD001 should be disabled, only MD003 should appear
    assert!(!stderr.contains("MD001"));
    assert!(stderr.contains("MD003"));

    // Temp directory will be automatically cleaned up
}

/// Test that line numbers are 1-based in CLI output
#[test]
fn test_cli_line_numbers_are_one_based() {
    let mut cmd = Command::cargo_bin("quickmark").unwrap();
    cmd.arg(test_sample_path("test_md001_violations.md"));

    let output = cmd.assert().failure().get_output().clone();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Find error lines and check line numbers
    let error_lines: Vec<&str> = stderr.lines()
        .filter(|line| line.starts_with("ERR:") || line.starts_with("WARN:"))
        .collect();

    for error_line in error_lines {
        // Extract line number (format: ERR: file:line:column ...)
        if let Some(colon_pos) = error_line.find(".md:") {
            let after_file = &error_line[colon_pos + 4..];
            if let Some(second_colon) = after_file.find(':') {
                let line_num_str = &after_file[..second_colon];
                if let Ok(line_num) = line_num_str.parse::<u32>() {
                    // Line numbers should be 1-based, not 0-based
                    assert!(line_num >= 1, "Line number should be 1-based, got: {line_num}");
                }
            }
        }
    }
}

/// Test CLI with mixed severity levels using temporary config
#[test]
fn test_cli_mixed_severities() {
    let temp_dir = TempDir::new().unwrap();

    // Create a config with mixed severities
    let config_content = r#"
[linters.severity]
heading-increment = 'warn'
heading-style = 'err'

[linters.settings.heading-style]
style = 'consistent'
"#;

    let config_file = temp_dir.child("quickmark.toml");
    config_file.write_str(config_content).unwrap();

    let mut cmd = Command::cargo_bin("quickmark").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg(test_sample_path("test_all_rules_violations.md"));

    let output = cmd.assert().failure().get_output().clone();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should contain both ERR and WARN prefixes
    assert!(stderr.contains("ERR:"));
    assert!(stderr.contains("WARN:"));

    // Should report both errors and warnings
    assert!(stdout.contains("Errors:"));
    assert!(stdout.contains("Warnings:"));
}

/// Test CLI with ATX-only style configuration
#[test]
fn test_cli_atx_only_file() {
    let mut cmd = Command::cargo_bin("quickmark").unwrap();
    cmd.arg(test_sample_path("test_md003_atx_only.md"));

    cmd.assert()
        .success() // Should succeed since all headings are consistent ATX style
        .stdout(predicates::str::contains("Errors: 0"))
        .stdout(predicates::str::contains("Warnings: 0"));
}

/// Test CLI with setext-only style file
#[test]
fn test_cli_setext_only_file() {
    let mut cmd = Command::cargo_bin("quickmark").unwrap();
    cmd.arg(test_sample_path("test_md003_setext_only.md"));

    cmd.assert()
        .success() // Should succeed since all headings are consistent setext style
        .stdout(predicates::str::contains("Errors: 0"))
        .stdout(predicates::str::contains("Warnings: 0"));
}

/// Test CLI with ATX-closed style file
#[test]
fn test_cli_atx_closed_file() {
    let mut cmd = Command::cargo_bin("quickmark").unwrap();
    cmd.arg(test_sample_path("test_md003_atx_closed.md"));

    cmd.assert()
        .success() // Should succeed since all headings are consistent ATX-closed style
        .stdout(predicates::str::contains("Errors: 0"))
        .stdout(predicates::str::contains("Warnings: 0"));
}

/// Test CLI with setext-atx violations file
#[test]
fn test_cli_setext_atx_violations() {
    let mut cmd = Command::cargo_bin("quickmark").unwrap();
    cmd.arg(test_sample_path("test_md003_setext_atx_violations.md"));

    cmd.assert()
        .failure() // Should fail due to style violations
        .stderr(predicates::str::contains("MD003"))
        .stderr(predicates::str::contains("heading-style"))
        .stdout(predicates::str::contains("Errors:"))
        .stdout(predicates::str::contains("Errors: 0").not());
}

/// Test CLI with custom configuration for setext_with_atx style
#[test]
fn test_cli_setext_with_atx_config() {
    let temp_dir = TempDir::new().unwrap();

    // Create a config with setext_with_atx style
    let config_content = r#"
[linters.severity]
heading-increment = 'off'
heading-style = 'err'

[linters.settings.heading-style]
style = 'setext_with_atx'
"#;

    let config_file = temp_dir.child("quickmark.toml");
    config_file.write_str(config_content).unwrap();

    let mut cmd = Command::cargo_bin("quickmark").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg(test_sample_path("test_md003_setext_atx_violations.md"));

    cmd.assert()
        .failure() // Should fail due to style violations
        .stderr(predicates::str::contains("MD003"))
        .stderr(predicates::str::contains("heading-style"));
}
