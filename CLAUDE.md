# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

QuickMark is a lightning-fast Markdown/CommonMark linter written in Rust. It's inspired by markdownlint for Ruby and focuses on providing exceptional performance while integrating with development environments through LSP.

The original David Anson's markdownlint source code is available here: <https://github.com/DavidAnson/markdownlint>

## Project Structure

This is a Rust workspace with multiple crates implementing a clean separation of concerns:

```
quickmark/
├── Cargo.toml                 # Workspace configuration
├── crates/
│   ├── quickmark_linter/      # Core linting logic (format-agnostic)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── config/        # Configuration data structures
│   │   │   ├── linter.rs      # Linting engine
│   │   │   ├── rules/         # Individual linting rules
│   │   │   └── tree_sitter_walker.rs
│   │   └── tests/
│   ├── quickmark_config/      # Shared configuration parsing
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs         # TOML parsing and validation
│   ├── quickmark/             # CLI application
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs        # CLI interface
│   └── quickmark_server/      # Server application (LSP, etc.)
│       ├── Cargo.toml
│       └── src/
│           └── main.rs        # Server interface
├── docs/                      # Documentation
└── tests/                     # Integration tests
```

## Common Commands

### Build and Development

- **Build all crates**: `cargo build`
- **Build release**: `cargo build --release`
- **Run tests**: `cargo test`
- **Run CLI linter**: `cargo run --bin quickmark -- /path/to/file.md`
- **Run server**: `cargo run --bin quickmark_server`

### Configuration

- QuickMark looks for `quickmark.toml` in the current working directory
- If not found, default configuration is used
- See `crates/quickmark_linter/tests/fixtures/quickmark.toml` for example configuration

## Architecture

### Crate Responsibilities

**quickmark_linter** (Core Library):

- Pure linting logic with no configuration format dependencies
- Accepts `QuickmarkConfig` objects directly
- Tree-sitter based Markdown parsing
- Rule system with pluggable architecture
- Format-agnostic design for maximum reusability

**quickmark_config** (Shared Configuration):

- TOML configuration parsing and validation
- Converts TOML structures to `QuickmarkConfig` objects
- Rule severity normalization
- Used by both CLI and server applications
- Centralized configuration logic prevents code duplication

**quickmark** (CLI Application):

- Command-line interface using clap
- File I/O and user interaction
- Uses `quickmark_config` for configuration parsing
- Uses `quickmark_linter` for actual linting

**quickmark_server** (Server Application):

- Server interface for LSP integration
- Uses same configuration system as CLI
- Demonstrates shared configuration usage

### Core Components

**Linting Engine** (`quickmark_linter/src/linter.rs`):

- `MultiRuleLinter`: Orchestrates multiple rule linters
- `RuleViolation`: Represents a linting error with location and message
- `Context`: Shared context containing file path and configuration
- Uses tree-sitter for Markdown parsing with tree-sitter-md grammar
- Filters rules based on severity configuration (off/warn/err)

**Configuration System** (`quickmark_linter/src/config/mod.rs`):

- Format-agnostic configuration data structures
- `QuickmarkConfig`: Root configuration structure
- `RuleSeverity`: Enum for error/warning/off states
- `normalize_severities`: Validates and normalizes rule configurations
- No serialization dependencies - pure data structures

**TOML Configuration** (`quickmark_config/src/lib.rs`):

- `parse_toml_config`: Parses TOML strings into `QuickmarkConfig`
- `config_in_path_or_default`: Loads config from filesystem or defaults
- TOML-specific data structures with serde derives
- Conversion functions between TOML and core config types

**Rule System** (`quickmark_linter/src/rules/mod.rs`):

- `Rule`: Static metadata structure defining rule properties
- `ALL_RULES`: Registry of all available rules
- Each rule implements `RuleLinter` trait with `feed` method
- Rules are dynamically instantiated based on configuration

### Current Rules

- **MD001** (`heading-increment`): Ensures heading levels increment by one
- **MD003** (`heading-style`): Enforces consistent heading styles

### Key Design Patterns

**Separation of Concerns**: Each crate has a single, focused responsibility:

- Core linting logic is separate from configuration formats
- Configuration parsing is shared between applications
- Applications handle their specific interfaces (CLI, server)

**Plugin Architecture**: Rules are registered in `ALL_RULES` and dynamically loaded based on configuration.

**Shared Context**: `Rc<Context>` is passed to all rule linters, containing file path and configuration.

**AST Traversal**: Uses tree-sitter node iteration with each rule's `feed` method processing nodes.

**Configuration-Driven**: Rule severity and settings are externally configurable via TOML files.

**Format Agnostic Core**: The linting engine accepts configuration objects directly, making it easy to support multiple configuration formats in the future.

## Dependencies

### quickmark_linter

- `anyhow`: Error handling
- `tree-sitter`: AST parsing
- `tree-sitter-md`: Markdown grammar

### quickmark_config

- `anyhow`: Error handling
- `serde`: TOML deserialization
- `toml`: TOML parsing
- `quickmark_linter`: Core configuration types

### quickmark

- `anyhow`: Error handling
- `clap`: CLI parsing
- `quickmark_config`: Configuration parsing
- `quickmark_linter`: Linting engine

### quickmark_server

- `anyhow`: Error handling
- `quickmark_config`: Configuration parsing
- `quickmark_linter`: Linting engine

## Adding New Rules

1. Create a new rule module in `crates/quickmark_linter/src/rules/`
2. Implement the `RuleLinter` trait
3. Add the rule to `ALL_RULES` in `crates/quickmark_linter/src/rules/mod.rs`
4. Add any rule-specific configuration to the config structs
5. Update TOML parsing in `quickmark_config` if needed

## Adding New Configuration Formats

1. Create conversion functions in `quickmark_config`
2. Add new public functions following the pattern of `parse_toml_config`
3. Both CLI and server applications can immediately use the new format
