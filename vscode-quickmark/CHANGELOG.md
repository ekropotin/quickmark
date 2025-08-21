# Change Log

All notable changes to the QuickMark VSCode extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.1] - 2024-01-XX

### Added
- Initial release of QuickMark VSCode extension
- LSP client integration with quickmark_server
- **Bundled binaries** for all major platforms (Windows, macOS, Linux)
- **Cross-platform support** with automatic platform detection
- Support for all Markdown file extensions (.md, .markdown, etc.)
- Configuration settings for server path, tracing, and linting behavior
- Commands for restarting server and showing output
- File system watcher for quickmark.toml configuration changes
- Comprehensive error handling and user feedback
- Development configuration with debugging support
- Build scripts for binary management and cross-compilation

### Features
- **Zero-dependency installation** - works out of the box
- Real-time Markdown linting powered by Rust backend
- Configurable rule severities (error/warning/off)
- Hierarchical configuration discovery
- Performance-optimized single-pass analysis
- Support for workspace folders and multi-root workspaces
- Fallback to custom server binary if preferred