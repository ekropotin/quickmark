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
│   ├── quickmark-core/        # Core linting logic with integrated configuration
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── config/        # Configuration data structures and TOML parsing
│   │   │   ├── linter.rs      # Linting engine
│   │   │   ├── rules/         # Individual linting rules
│   │   │   ├── test_utils.rs  # Testing utilities
│   │   │   └── tree_sitter_walker.rs  # Tree-sitter AST traversal utilities
│   │   └── tests/
│   ├── quickmark-cli/         # CLI application
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs        # CLI interface
│   └── quickmark-server/      # Server application (LSP, etc.)
│       ├── Cargo.toml
│       └── src/
│           └── main.rs        # Server interface
├── docs/                      # Documentation
├── test-samples/              # Test files and configurations
└── vscode-quickmark/          # VSCode extension
```

## Common Commands

### Build and Development

- **Build all crates**: `cargo build`
- **Build release**: `cargo build --release`
- **Run tests**: `cargo test`
- **Run CLI linter**: `cargo run --bin qmark -- /path/to/file.md`
- **Run server**: `cargo run --bin quickmark-server`

### Configuration

- QuickMark looks for `quickmark.toml` in the current working directory
- If not found, default configuration is used

## Architecture

### Crate Responsibilities

**quickmark-core** (Core Library):

- Core linting logic with integrated configuration system
- TOML configuration parsing and validation
- Converts TOML structures to `QuickmarkConfig` objects  
- Tree-sitter based Markdown parsing
- Rule system with pluggable architecture
- Rule severity normalization and validation
- Self-contained design eliminates external configuration dependencies

**quickmark-cli** (CLI Application):

- Command-line interface using clap
- File I/O and user interaction
- Uses `quickmark-core` for configuration parsing and linting
- Parallel file processing with rayon
- File glob and ignore pattern support

**quickmark-server** (Server Application):

- LSP server interface for editor integration
- Uses `quickmark-core` for configuration and linting
- Async processing with tokio
- Real-time document analysis

### Core Components

**Linting Engine** (`quickmark-core/src/linter.rs`):

- `MultiRuleLinter`: Orchestrates multiple rule linters
- `RuleViolation`: Represents a linting error with location and message
- `Context`: Shared context containing file path and configuration
- Uses tree-sitter for Markdown parsing with tree-sitter-md grammar
- Filters rules based on severity configuration (off/warn/err)

**Configuration System** (`quickmark-core/src/config/mod.rs`):

- Format-agnostic configuration data structures
- `QuickmarkConfig`: Root configuration structure
- `RuleSeverity`: Enum for error/warning/off states
- `normalize_severities`: Validates and normalizes rule configurations
- No serialization dependencies - pure data structures

**TOML Configuration** (`quickmark-core/src/config/mod.rs`):

- Integrated TOML parsing within the core library
- `parse_toml_config`: Parses TOML strings into `QuickmarkConfig`
- `config_in_path_or_default`: Loads config from filesystem or defaults
- TOML-specific data structures with serde derives
- Direct conversion to core configuration types

**Rule System** (`quickmark-core/src/rules/mod.rs`):

- `Rule`: Static metadata structure defining rule properties
- `ALL_RULES`: Registry of all available rules
- Each rule implements `RuleLinter` trait with `feed` method
- Rules are dynamically instantiated based on configuration

### Linting Architecture Evolution

**Performance-Optimized Single-Pass Design**:

QuickMark has evolved from a simple node-based traversal to a sophisticated single-pass architecture that efficiently handles different rule types while maintaining exceptional performance. This design is inspired by the original markdownlint's architecture but leverages Rust's performance advantages and tree-sitter's robust parsing.

**Rule Type Classification**:

Rules are categorized into five types for optimal performance and implementation strategy:

- **Line-Based Rules** (e.g., MD013): Operate directly on raw text lines with AST context for configuration
- **Token-Based Rules** (e.g., MD001, MD003): Work with specific cached AST node types
- **Document-Wide Rules** (e.g., MD024, MD025): Require full document state analysis
- **Hybrid Rules** (e.g., MD022): Need both AST analysis and line context for structural spacing
- **Special Rules** (e.g., MD044): Unique implementation requirements like external dictionaries

**Enhanced Context System**:

The `Context` provides multiple optimized data views:

- Raw text lines for line-based analysis
- Cached filtered AST nodes by type (headings, code blocks, etc.)
- Configuration-driven rule execution with lazy evaluation

**Motivation for Single-Pass Architecture**:

1. **Performance**: Avoids multiple document parsing passes that would compromise QuickMark's speed promise
2. **Memory Efficiency**: Caches commonly-used node types rather than re-filtering AST repeatedly
3. **Scalability**: Supports complex rules (cross-document validation, word analysis) without architectural changes
4. **Compatibility**: Maintains the existing rule interface while enabling performance optimizations

This architecture allows rules like MD013 to work efficiently with raw text while still having access to AST context for proper configuration handling (e.g., different limits for headings vs. code blocks).

### Key Design Patterns

**Separation of Concerns**: Each crate has a single, focused responsibility:

- Core library integrates linting logic with configuration parsing
- Applications handle their specific interfaces (CLI, server)
- Clean dependency hierarchy with core as the foundation

**Plugin Architecture**: Rules are registered in `ALL_RULES` and dynamically loaded based on configuration.

**Shared Context**: `Rc<Context>` is passed to all rule linters, containing file path and configuration.

**Hybrid AST + Line Processing**: Uses tree-sitter for structural analysis with cached node filtering, plus direct text line access for line-based rules. Rules receive an enhanced context with multiple optimized data views.

**Configuration-Driven**: Rule severity and settings are externally configurable via TOML files.

**Integrated Configuration**: The core library includes TOML configuration parsing while maintaining clean separation between configuration structures and linting logic.

## Dependencies

### quickmark-core

- `anyhow`: Error handling
- `tree-sitter`: AST parsing  
- `tree-sitter-md`: Markdown grammar
- `serde`: TOML deserialization
- `toml`: TOML parsing and configuration
- `regex`: Pattern matching
- `linkify`: URL detection
- `once_cell`: Lazy statics

### quickmark-cli

- `anyhow`: Error handling
- `clap`: CLI parsing with derive features
- `quickmark-core` (path = "../quickmark-core"): Linting engine and configuration
- `glob`: File pattern matching
- `rayon`: Parallel processing
- `ignore`: Gitignore-style file filtering
- `walkdir`: Directory traversal

### quickmark-server

- `anyhow`: Error handling
- `quickmark-core` (path = "../quickmark-core"): Linting engine and configuration
- `tower-lsp`: LSP server implementation
- `tokio`: Async runtime

## Adding New Rules

1. Create a new rule module in `crates/quickmark-core/src/rules/`
2. Implement the `RuleLinter` trait with appropriate `RuleType` classification
3. Add the rule to `ALL_RULES` in `crates/quickmark-core/src/rules/mod.rs`
4. Add any rule-specific configuration to the config structs
5. Update TOML parsing in `quickmark-core/src/config/` if needed

**Rule Type Guidelines**:

- Use `RuleType::Line` for rules that primarily analyze text content (line length, whitespace, etc.)
- Use `RuleType::Token` for rules that analyze document structure (headings, lists, code blocks)
- Use `RuleType::Document` for rules requiring full document analysis (duplicate headings, cross-references)
- Use `RuleType::Hybrid` for rules needing both AST nodes and line context (blank line spacing around elements)
- Use `RuleType::Special` for rules with unique requirements (external dictionaries, complex text analysis)

## Adding New Configuration Formats

1. Add conversion functions to `quickmark-core/src/config/`
2. Implement parsing functions following the pattern of `parse_toml_config`
3. Both CLI and server applications inherit the new format support automatically
4. Extend the configuration module with new format-specific dependencies as needed

## Code Guidelines

### General principles

1. **Follow idiomatic Rust**: Use the latest Rust best practices and conventions. Code should feel natural to an experienced Rust developer.
2. **No unsafe unless required**: Do not use unsafe blocks unless absolutely necessary for performance or interoperability. If used, justify with a comment and encapsulate safely.
3. **Zero compiler warnings**:
   - Code must compile with `#![deny(warnings)]`.
   - Suppress only known false positives with clearly scoped `#[allow(...)]` attributes and documented reasons.
4. **Speed over memory**:
   - Optimize for CPU performance, even at the cost of increased memory usage.
   - Avoid unnecessary allocations, but favor speed in algorithms and data access patterns.
5. **Cloning is expensive**: Avoid cloning (`.clone()`) unless it is proven to be more efficient than passing a reference or performing in-place mutation.
6. **Use modern language features**:
   - Prefer `let else`, `if let`, match ergonomics, Iterator combinators, `?` operator, and Result-based error handling.
   - Consider `Cow` or `Arc` where applicable to avoid unnecessary clones.

### Optimization practices

1. **Prefer move semantics** when data is no longer needed.
2. **Use references wisely**: Use `&T` or `&mut T` rather than `T` or `T.clone()` when ownership is not required.
3. **Inline strategically**: Inline functions where beneficial using `#[inline]` (but measure if in doubt).
4. **Use zero-cost abstractions**: From the standard library or crates like `itertools`, `smallvec`, `rayon` (for parallelism), etc.
5. **Choose fast data structures**: For hot paths, prefer faster data structures, even if more memory is consumed (e.g., `HashMap` over `BTreeMap` when ordering is unnecessary).

### Safety and correctness

1. **Use strong typing**: Use the type system to enforce invariants.
2. **Avoid panics**: In library code (`unwrap()`, `expect()`) unless in clearly unreachable branches.
3. **Mark important results**: Use `#[must_use]` to mark important results when appropriate.
4. **Document assumptions**: Document all `TODO`, `FIXME`, and assumptions in code.

### Linting & Tooling

**Code must pass**:

1. `cargo check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo fmt --check`
4. `cargo test --all`

**Additional practices**:

1. **Use Clippy lints**: That enforce performance best practices (e.g., `clippy::redundant_clone`, `clippy::needless_collect`, `clippy::manual_memcpy`, etc.)
2. **Use performance attributes**: `#[inline(always)]`, `#[cold]`, or `#[no_mangle]` where profiling/FFI suggests it makes sense — but only after benchmarking.

### Testing & Validation

**Tests must**:

1. **Cover edge cases**: And performance regressions.
2. **Use `#[should_panic]`**: Where panics are expected.
3. **Prefer property-based testing**: Use `proptest` or fuzzing where inputs are highly variable.

# Git and Commit Guidelines

## Branch Naming and Workflow

Follow the branch naming conventions defined in [CONTRIBUTING.md](CONTRIBUTING.md).

## Commit Messages

When working on a branch that follows the naming convention, always include the issue reference in commit messages:

**If the branch name contains an issue number** (e.g., `feature/123-add-rule`), include `fixes #123` in the commit message to automatically link and close the issue when merged.

**Format:**
```
Brief description of changes

Longer explanation if needed.

Fixes #123
```

**Example:**
```
Add MD025 single H1 rule implementation

Implements the MD025 rule that ensures documents have only one H1 heading.
Includes comprehensive tests and documentation.

Fixes #123
```
