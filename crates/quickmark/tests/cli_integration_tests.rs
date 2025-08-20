use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::path::PathBuf;

/// Helper function to get the path to test sample files
fn test_sample_path(filename: &str) -> String {
    // Use the CARGO_MANIFEST_DIR environment variable to find the project root
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

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
    let mut cmd = Command::cargo_bin("qmark").unwrap();
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
    let mut cmd = Command::cargo_bin("qmark").unwrap();
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
    let mut cmd = Command::cargo_bin("qmark").unwrap();
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
    let mut cmd = Command::cargo_bin("qmark").unwrap();
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
    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.arg("nonexistent_file.md");

    cmd.assert()
        .success() // Should succeed with no files found message
        .stderr(predicates::str::contains(
            "No markdown files found to lint.",
        ));
}

/// Test CLI error output format
#[test]
fn test_cli_error_format() {
    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.arg(test_sample_path("test_md001_violations.md"));

    let output = cmd.assert().failure().get_output().clone();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Check that error format includes expected components:
    // ERR: file_path:line:column MD001/heading-increment message
    let error_lines: Vec<&str> = stderr
        .lines()
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

    // Create a test markdown file in the same directory as the config
    let md_content = r#"
# Heading 1

Heading 2
=========

## Heading 3
"#;
    let md_file = temp_dir.child("test.md");
    md_file.write_str(md_content).unwrap();

    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.arg(md_file.path());

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
    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.arg(test_sample_path("test_md001_violations.md"));

    let output = cmd.assert().failure().get_output().clone();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Find error lines and check line numbers
    let error_lines: Vec<&str> = stderr
        .lines()
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
                    assert!(
                        line_num >= 1,
                        "Line number should be 1-based, got: {line_num}"
                    );
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

    // Create a test markdown file that will trigger both rules in the same directory as the config
    let md_content = r#"# Heading 1

### Heading 3 (violates MD001 - skipped level 2)

Heading 2 (setext style - violates MD003 mixed styles)
=========

## Another heading (ATX style)"#;
    let md_file = temp_dir.child("test_violations.md");
    md_file.write_str(md_content).unwrap();

    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.arg(md_file.path());

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
    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.arg(test_sample_path("test_md003_atx_only.md"));

    cmd.assert()
        .success() // Should succeed since all headings are consistent ATX style
        .stdout(predicates::str::contains("Errors: 0"))
        .stdout(predicates::str::contains("Warnings: 0"));
}

/// Test CLI with setext-only style file
#[test]
fn test_cli_setext_only_file() {
    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.arg(test_sample_path("test_md003_setext_only.md"));

    cmd.assert()
        .success() // Should succeed since all headings are consistent setext style
        .stdout(predicates::str::contains("Errors: 0"))
        .stdout(predicates::str::contains("Warnings: 0"));
}

/// Test CLI with ATX-closed style file
#[test]
fn test_cli_atx_closed_file() {
    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.arg(test_sample_path("test_md003_atx_closed.md"));

    cmd.assert()
        .success() // Should succeed since all headings are consistent ATX-closed style
        .stdout(predicates::str::contains("Errors: 0"))
        .stdout(predicates::str::contains("Warnings: 0"));
}

/// Test CLI with setext-atx violations file
#[test]
fn test_cli_setext_atx_violations() {
    let mut cmd = Command::cargo_bin("qmark").unwrap();
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

    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg(test_sample_path("test_md003_setext_atx_violations.md"));

    cmd.assert()
        .failure() // Should fail due to style violations
        .stderr(predicates::str::contains("MD003"))
        .stderr(predicates::str::contains("heading-style"));
}

/// Test hierarchical config discovery with mass linting
/// This test verifies both the multiple file linting capability and proper
/// hierarchical configuration discovery for each file based on its location
#[test]
fn test_cli_hierarchical_config_discovery() {
    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.arg(test_sample_path("hierarchical-test/"));

    let output = cmd.assert().failure().get_output().clone();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Verify that multiple markdown files were processed
    let processed_files: std::collections::HashSet<_> = stderr
        .lines()
        .filter(|line| line.contains(".md:"))
        .filter_map(|line| {
            // Extract file path: ERR: file_path:line:col rule message
            if let Some(colon_pos) = line.find(": ") {
                let after_prefix = &line[colon_pos + 2..];
                if let Some(md_pos) = after_prefix.find(".md:") {
                    return Some(&after_prefix[..md_pos + 3]);
                }
            }
            None
        })
        .collect();

    // Should process at least 4 markdown files (README.md, api.md, guide.md, integration.md, lib.md)
    assert!(
        processed_files.len() >= 4,
        "Should process multiple markdown files, found: {:?}",
        processed_files
    );

    // Verify files from different directories are processed
    let has_project_root_files = processed_files
        .iter()
        .any(|f| f.contains("project-root/README.md"));
    let has_src_files = processed_files
        .iter()
        .any(|f| f.contains("project-root/src/api.md"));
    let has_nested_files = processed_files
        .iter()
        .any(|f| f.contains("project-root/src/docs/guide.md"));
    let has_tests_files = processed_files
        .iter()
        .any(|f| f.contains("project-root/tests/integration.md"));
    let has_cargo_project_files = processed_files
        .iter()
        .any(|f| f.contains("cargo-project/src/lib.md"));

    assert!(has_project_root_files, "Should process project root files");
    assert!(has_src_files, "Should process src directory files");
    assert!(has_nested_files, "Should process nested docs files");
    assert!(has_tests_files, "Should process tests directory files");
    assert!(
        has_cargo_project_files,
        "Should process cargo-project files"
    );

    // Test hierarchical config application by checking rule behavior per file location

    // 1. Project root files should have MD001 disabled (project-root/quickmark.toml)
    let project_root_md001_violations: Vec<_> = stderr
        .lines()
        .filter(|line| line.contains("project-root/README.md:") && line.contains("MD001"))
        .collect();
    assert!(
        project_root_md001_violations.is_empty(),
        "MD001 should be disabled at project root level, but found: {:?}",
        project_root_md001_violations
    );

    // 2. Verify different configs are applied by checking line length limits
    // Src files should have stricter line length (80 chars) vs project root (100 chars)
    let src_line_length_violations: Vec<_> = stderr
        .lines()
        .filter(|line| {
            (line.contains("project-root/src/api.md:")
                || line.contains("project-root/src/docs/guide.md:"))
                && line.contains("MD013")
                && line.contains("80")
        })
        .collect();

    let root_line_length_violations: Vec<_> = stderr
        .lines()
        .filter(|line| {
            (line.contains("project-root/README.md:")
                || line.contains("project-root/tests/integration.md:"))
                && line.contains("MD013")
                && line.contains("100")
        })
        .collect();

    assert!(
        !src_line_length_violations.is_empty(),
        "Src files should use 80 char line length limit, but none found"
    );

    assert!(
        !root_line_length_violations.is_empty(),
        "Root files should use 100 char line length limit, but none found"
    );

    // 3. Cargo project files should use their own config
    let cargo_project_config_applied = stderr
        .lines()
        .any(|line| line.contains("cargo-project/src/lib.md:"));
    assert!(
        cargo_project_config_applied,
        "Cargo project files should be processed with their own config"
    );

    println!("Processed files: {:?}", processed_files);
    println!(
        "Total violations found: {}",
        stderr
            .lines()
            .filter(|l| l.contains("ERR:") || l.contains("WARN:"))
            .count()
    );
}

/// Test that config discovery stops at git repository boundaries
#[test]
fn test_cli_config_discovery_git_boundary() {
    // Create a temporary git repository structure
    let temp_dir = TempDir::new().unwrap();

    // Create outer directory with config (should NOT be found due to git boundary)
    let outer_config = temp_dir.child("quickmark.toml");
    outer_config
        .write_str(
            r#"
[linters.severity]
heading-increment = 'off'
"#,
        )
        .unwrap();

    // Create git repository subdirectory
    let git_repo = temp_dir.child("repo");
    git_repo.create_dir_all().unwrap();

    let git_dir = git_repo.child(".git");
    git_dir.create_dir_all().unwrap();

    // Create markdown file in git repo (should use default config, not outer config)
    let md_content = r#"# Title

### Skipped Level 2 (should trigger MD001 with default config)
"#;
    let md_file = git_repo.child("README.md");
    md_file.write_str(md_content).unwrap();

    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.arg(md_file.path());

    let output = cmd.assert().failure().get_output().clone();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should trigger MD001 error because default config is used, not outer config
    assert!(
        stderr.contains("MD001"),
        "Should use default config and detect MD001 violation, not outer config"
    );
}

/// Test CLI with QUICKMARK_CONFIG environment variable pointing to valid config
#[test]
fn test_cli_quickmark_config_env_valid() {
    let temp_dir = TempDir::new().unwrap();

    // Create a config file with specific settings
    let config_content = r#"
[linters.severity]
heading-increment = 'warn'
heading-style = 'off'
line-length = 'err'

[linters.settings.line-length]
line_length = 50
"#;

    let config_file = temp_dir.child("custom_config.toml");
    config_file.write_str(config_content).unwrap();

    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.env("QUICKMARK_CONFIG", config_file.path())
        .arg(test_sample_path("test_md001_violations.md"));

    let output = cmd.assert().failure().get_output().clone();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should only show MD001 warnings (heading-style is off, line-length is on)
    assert!(stderr.contains("WARN:"));
    assert!(stderr.contains("MD001"));
    assert!(!stderr.contains("MD003")); // heading-style is off
}

/// Test CLI with QUICKMARK_CONFIG environment variable pointing to invalid path
#[test]
fn test_cli_quickmark_config_env_invalid() {
    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.env("QUICKMARK_CONFIG", "/nonexistent/path/config.toml")
        .arg(test_sample_path("test_md001_valid.md"));

    let output = cmd.assert().failure().get_output().clone();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should show error about invalid config path but continue with default config
    assert!(
        stderr.contains("Config file was not found") || stderr.contains("Error loading config")
    );
    // Should still process the file with default config
    assert!(stderr.contains("MD003")); // Default config should catch MD003 violations
}

/// Test CLI with QUICKMARK_CONFIG environment variable taking precedence over local config
#[test]
fn test_cli_quickmark_config_env_precedence() {
    let temp_dir = TempDir::new().unwrap();

    // Create a local quickmark.toml that would normally be used
    let local_config_content = r#"
[linters.severity]
heading-increment = 'off'
heading-style = 'off'
"#;

    let local_config_file = temp_dir.child("quickmark.toml");
    local_config_file.write_str(local_config_content).unwrap();

    // Create a different config file for QUICKMARK_CONFIG
    let env_config_content = r#"
[linters.severity]
heading-increment = 'err'
heading-style = 'err'
"#;

    let env_config_file = temp_dir.child("env_config.toml");
    env_config_file.write_str(env_config_content).unwrap();

    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.current_dir(temp_dir.path())
        .env("QUICKMARK_CONFIG", env_config_file.path())
        .arg(test_sample_path("test_md001_violations.md"));

    let output = cmd.assert().failure().get_output().clone();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should use env config (errors) not local config (off)
    assert!(stderr.contains("ERR:"));
    assert!(stderr.contains("MD001"));
    assert!(stderr.contains("MD003"));
}

/// Test CLI with multiple files
#[test]
fn test_cli_multiple_files() {
    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.arg(test_sample_path("test_md001_violations.md"))
        .arg(test_sample_path("test_md003_mixed_styles.md"));

    let output = cmd.assert().failure().get_output().clone();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should contain violations from both files
    assert!(stderr.contains("test_md001_violations.md"));
    assert!(stderr.contains("test_md003_mixed_styles.md"));
    assert!(stderr.contains("MD001"));
    assert!(stderr.contains("MD003"));

    // Should have multiple errors
    assert!(stdout.contains("Errors:"));
    // Extract error count and verify it's greater than 1
    let error_count: i32 = stdout
        .lines()
        .find(|line| line.starts_with("Errors:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|count| count.parse().ok())
        .unwrap_or(0);
    assert!(
        error_count > 1,
        "Should have multiple errors from multiple files"
    );
}

/// Test CLI with directory traversal
#[test]
fn test_cli_directory_traversal() {
    use std::env;

    // Get the project root directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let test_samples_dir = std::path::PathBuf::from(manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("test-samples");

    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.arg(test_samples_dir);

    let output = cmd.assert().failure().get_output().clone();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should find violations from multiple files in the directory
    assert!(stderr.contains(".md"));
    assert!(stdout.contains("Errors:"));

    // Should process multiple files - count unique file paths
    let unique_files: std::collections::HashSet<_> = stderr
        .lines()
        .filter(|line| line.contains(".md:"))
        .filter_map(|line| {
            // Extract file path: ERR: file_path:line:col rule message
            // Skip "ERR: " or "WARN: " prefix
            if let Some(colon_pos) = line.find(": ") {
                let after_prefix = &line[colon_pos + 2..];
                // Find the path part before the line number
                if let Some(md_pos) = after_prefix.find(".md:") {
                    return Some(&after_prefix[..md_pos + 3]); // Include .md
                }
            }
            None
        })
        .collect();
    assert!(
        unique_files.len() > 1,
        "Should process multiple markdown files in directory, found: {:?}",
        unique_files
    );
}

/// Test CLI with non-markdown files (should be ignored)
#[test]
fn test_cli_non_markdown_files_ignored() {
    let temp_dir = TempDir::new().unwrap();

    // Create a non-markdown file
    let txt_file = temp_dir.child("README.txt");
    txt_file.write_str("This is not a markdown file").unwrap();

    // Create a markdown file for comparison
    let md_file = temp_dir.child("test.md");
    md_file.write_str("# Title\n\n### Skipped H2").unwrap();

    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.arg(temp_dir.path());

    let output = cmd.assert().failure().get_output().clone();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should only process the markdown file
    assert!(stderr.contains("test.md"));
    assert!(!stderr.contains("README.txt"));
    assert!(stderr.contains("MD001")); // Should find violations in the .md file
}

/// Test CLI with no markdown files found
#[test]
fn test_cli_no_markdown_files() {
    let temp_dir = TempDir::new().unwrap();

    // Create only non-markdown files
    let txt_file = temp_dir.child("README.txt");
    txt_file.write_str("This is not markdown").unwrap();

    let rs_file = temp_dir.child("main.rs");
    rs_file.write_str("fn main() {}").unwrap();

    let mut cmd = Command::cargo_bin("qmark").unwrap();
    cmd.arg(temp_dir.path());

    cmd.assert()
        .success() // Should exit successfully but with no files
        .stderr(predicates::str::contains(
            "No markdown files found to lint.",
        ));
}
