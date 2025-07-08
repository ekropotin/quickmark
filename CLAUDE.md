# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

QuickMark is a lightning-fast Markdown/CommonMark linter written in Rust. It's inspired by markdownlint for Ruby and focuses on providing exceptional performance while integrating with development environments through LSP.

## Common Commands

### Build and Development
- **Build release binary**: `cargo build --release` (binary will be in `./target/release/quickmark`)
- **Run tests**: `cargo test`
- **Run the linter**: `./target/release/quickmark /path/to/file.md` or `cargo run -- /path/to/file.md`

### Configuration
- QuickMark looks for `quickmark.toml` in the current working directory
- If not found, default configuration is used
- See `tests/fixtures/quickmark.toml` for example configuration

## Architecture

### Core Components

**Main Entry Point** (`src/main.rs`):
- CLI parsing using clap
- Loads configuration from current directory or uses defaults
- Creates shared context and runs the multi-rule linter
- Exits with error code based on linting results

**Linting Engine** (`src/linter.rs`):
- `MultiRuleLinter`: Orchestrates multiple rule linters
- `RuleViolation`: Represents a linting error with location and message
- `Context`: Shared context containing file path and configuration
- Uses tree-sitter for Markdown parsing with tree-sitter-md grammar
- Filters rules based on severity configuration (off/warn/err)

**Configuration System** (`src/config/mod.rs`):
- TOML-based configuration with serde deserialization
- `QuickmarkConfig`: Root configuration structure
- `RuleSeverity`: Enum for error/warning/off states
- Normalizes rule severities and filters unknown rules
- Supports rule-specific settings (e.g., heading style preferences)

**Rule System** (`src/rules/mod.rs`):
- `Rule`: Static metadata structure defining rule properties
- `ALL_RULES`: Registry of all available rules
- Each rule implements `RuleLinter` trait with `feed` method
- Rules are dynamically instantiated based on configuration

### Current Rules
- **MD001** (`heading-increment`): Ensures heading levels increment by one
- **MD003** (`heading-style`): Enforces consistent heading styles

### Key Design Patterns

**Plugin Architecture**: Rules are registered in `ALL_RULES` and dynamically loaded based on configuration.

**Shared Context**: `Rc<Context>` is passed to all rule linters, containing file path and configuration.

**AST Traversal**: Uses tree-sitter node iteration with each rule's `feed` method processing nodes.

**Configuration-Driven**: Rule severity and settings are externally configurable via TOML files.