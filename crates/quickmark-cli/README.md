# quickmark-cli

Lightning-fast Markdown/CommonMark linter CLI tool with tree-sitter based parsing.

## Overview

`quickmark-cli` provides a command-line interface for QuickMark, enabling fast Markdown linting from the terminal with parallel file processing and comprehensive file pattern support.

## Installation

```bash
cargo install quickmark-cli
```

## Usage

```bash
# Lint a single file
qmark document.md

# Lint multiple files
qmark *.md

# Lint directory recursively
qmark docs/

# Use specific configuration
qmark --config quickmark.toml src/
```

## Features

- **Parallel Processing**: Uses rayon for concurrent file linting
- **File Pattern Matching**: Supports glob patterns and gitignore-style filtering
- **Configurable**: Loads `quickmark.toml` configuration files
- **Fast**: Built on the high-performance quickmark-core library
- **Cross-platform**: Works on Linux, macOS, and Windows

## Configuration

QuickMark looks for `quickmark.toml` in the current directory. If not found, default configuration is used.

Example configuration:
```toml
[rules]
MD013 = "warn"  # Line length
MD024 = "error" # Multiple headings with same content
```

## Binary Name

The CLI tool is installed as `qmark` for quick access.

## License

MIT