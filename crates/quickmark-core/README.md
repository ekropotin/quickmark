# quickmark-core

Lightning-fast Markdown/CommonMark linter core library with tree-sitter based parsing.

## Overview

`quickmark-core` is the foundational library for QuickMark, providing high-performance Markdown linting capabilities. It features an integrated configuration system, tree-sitter based parsing, and a pluggable rule architecture designed for speed and extensibility.

## Features

- **Tree-sitter Parsing**: Uses tree-sitter-md for robust Markdown AST generation
- **Integrated Configuration**: Built-in TOML configuration parsing and validation
- **Rule System**: Pluggable architecture with 5 rule types for optimal performance
- **Single-Pass Architecture**: Efficient processing with cached node filtering
- **Configuration-Driven**: Externally configurable rule severity and settings

## Usage

```rust
use quickmark_core::{config_in_path_or_default, MultiRuleLinter, Context};

// Load configuration
let config = config_in_path_or_default(".")?;

// Create linter and context
let linter = MultiRuleLinter::new(&config);
let context = Context::new("example.md", &config);

// Lint markdown content
let violations = linter.lint(&context, markdown_content)?;
```

## Rule Types

- **Line-Based**: Analyze raw text lines (e.g., line length limits)
- **Token-Based**: Work with specific AST node types (e.g., headings, lists)
- **Document-Wide**: Require full document analysis (e.g., duplicate detection)
- **Hybrid**: Need both AST and line context (e.g., spacing rules)
- **Special**: Unique requirements (e.g., external dictionaries)

## License

MIT