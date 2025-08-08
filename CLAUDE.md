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
- **Run CLI linter**: `cargo run --bin qmark -- /path/to/file.md`
- **Run server**: `cargo run --bin quickmark_server`

### Configuration

- QuickMark looks for `quickmark.toml` in the current working directory
- If not found, default configuration is used

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

- Core linting logic is separate from configuration formats
- Configuration parsing is shared between applications
- Applications handle their specific interfaces (CLI, server)

**Plugin Architecture**: Rules are registered in `ALL_RULES` and dynamically loaded based on configuration.

**Shared Context**: `Rc<Context>` is passed to all rule linters, containing file path and configuration.

**Hybrid AST + Line Processing**: Uses tree-sitter for structural analysis with cached node filtering, plus direct text line access for line-based rules. Rules receive an enhanced context with multiple optimized data views.

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
2. Implement the `RuleLinter` trait with appropriate `RuleType` classification
3. Add the rule to `ALL_RULES` in `crates/quickmark_linter/src/rules/mod.rs`
4. Add any rule-specific configuration to the config structs
5. Update TOML parsing in `quickmark_config` if needed

**Rule Type Guidelines**:

- Use `RuleType::Line` for rules that primarily analyze text content (line length, whitespace, etc.)
- Use `RuleType::Token` for rules that analyze document structure (headings, lists, code blocks)
- Use `RuleType::Document` for rules requiring full document analysis (duplicate headings, cross-references)
- Use `RuleType::Hybrid` for rules needing both AST nodes and line context (blank line spacing around elements)
- Use `RuleType::Special` for rules with unique requirements (external dictionaries, complex text analysis)

## Adding New Configuration Formats

1. Create conversion functions in `quickmark_config`
2. Add new public functions following the pattern of `parse_toml_config`
3. Both CLI and server applications can immediately use the new format

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
