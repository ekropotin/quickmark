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

### Rules Implementation Checklist

This section tracks the progress of porting all markdownlint rules to QuickMark. Rules are categorized by their implementation requirements:

#### Line-Based Rules (5 rules)

*Work primarily with raw text lines - high performance, direct text analysis*

- [ ] **MD009** (`no-trailing-spaces`): Trailing spaces at end of lines
- [ ] **MD010** (`no-hard-tabs`): Hard tabs should not be used
- [ ] **MD012** (`no-multiple-blanks`): Multiple consecutive blank lines
- [x] **MD013** (`line-length`): Line length limits with configurable exceptions ✅
- [ ] **MD047** (`single-trailing-newline`): Files should end with a single newline

#### Token-Based Rules (32 rules)

*Work with specific AST node types - cached node filtering for efficiency*

**Heading Rules (8 rules):**

- [x] **MD001** (`heading-increment`): Heading levels increment by one ✅
- [x] **MD003** (`heading-style`): Consistent heading styles ✅
- [ ] **MD018** (`no-missing-space-atx`): Space after hash in ATX headings
- [ ] **MD019** (`no-multiple-space-atx`): Multiple spaces after hash in ATX headings
- [ ] **MD020** (`no-missing-space-closed-atx`): Space inside closed ATX headings
- [ ] **MD021** (`no-multiple-space-closed-atx`): Multiple spaces in closed ATX headings
- [ ] **MD023** (`heading-start-left`): Headings start at beginning of line
- [ ] **MD026** (`no-trailing-punctuation`): Trailing punctuation in headings

**List Rules (6 rules):**

- [ ] **MD004** (`ul-style`): Unordered list style consistency
- [ ] **MD005** (`list-indent`): List item indentation at same level
- [ ] **MD006** (`ul-start-left`): Bulleted lists start at beginning of line
- [ ] **MD007** (`ul-indent`): Unordered list indentation consistency
- [ ] **MD029** (`ol-prefix`): Ordered list item prefix consistency
- [ ] **MD030** (`list-marker-space`): Spaces after list markers

**Link Rules (3 rules):**

- [ ] **MD011** (`no-reversed-links`): Reversed link syntax
- [ ] **MD034** (`no-bare-urls`): Bare URLs without proper formatting
- [ ] **MD042** (`no-empty-links`): Empty links

**Code Rules (4 rules):**

- [ ] **MD014** (`commands-show-output`): Dollar signs before shell commands
- [ ] **MD040** (`fenced-code-language`): Language specified for fenced code blocks
- [ ] **MD046** (`code-block-style`): Code block style consistency
- [ ] **MD048** (`code-fence-style`): Code fence style consistency

**Formatting Rules (11 rules):**

- [ ] **MD027** (`no-multiple-space-blockquote`): Multiple spaces after blockquote
- [ ] **MD028** (`no-blanks-blockquote`): Blank lines inside blockquotes
- [ ] **MD033** (`no-inline-html`): Inline HTML usage
- [ ] **MD035** (`hr-style`): Horizontal rule style consistency
- [ ] **MD036** (`no-emphasis-as-heading`): Emphasis used instead of heading
- [ ] **MD037** (`no-space-in-emphasis`): Spaces inside emphasis markers
- [ ] **MD038** (`no-space-in-code`): Spaces inside code span elements
- [ ] **MD039** (`no-space-in-links`): Spaces inside link text
- [ ] **MD045** (`no-alt-text`): Images should have alternate text
- [ ] **MD049** (`emphasis-style`): Emphasis style consistency
- [ ] **MD050** (`strong-style`): Strong style consistency

#### Document-Wide Rules (7 rules)

*Require full document analysis - global state tracking*

- [ ] **MD024** (`no-duplicate-heading`): Multiple headings with same content
- [ ] **MD025** (`single-title`): Multiple top-level headings
- [ ] **MD041** (`first-line-heading`): First line should be top-level heading
- [ ] **MD043** (`required-headings`): Required heading structure
- [x] **MD051** (`link-fragments`): Link fragments should be valid
- [ ] **MD052** (`reference-links-images`): Reference links should be defined
- [ ] **MD053** (`link-image-reference-definitions`): Reference definitions should be needed

#### Hybrid Rules (3 rules)

*Need both AST analysis and line context - structural elements with spacing*

- [ ] **MD022** (`blanks-around-headings`): Headings surrounded by blank lines
- [ ] **MD031** (`blanks-around-fences`): Fenced code blocks surrounded by blank lines
- [ ] **MD032** (`blanks-around-lists`): Lists surrounded by blank lines

#### Special Rules (1 rule)

*Unique implementation requirements*

- [ ] **MD044** (`proper-names`): Proper names with correct capitalization (requires external dictionaries)

**Implementation Progress: 3/48 rules completed (6.25%)**

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
