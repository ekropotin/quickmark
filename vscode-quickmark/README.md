# QuickMark VSCode Extension

A lightning-fast Markdown linter for Visual Studio Code, powered by the QuickMark LSP server written in Rust.

## Features

- **Ultra-fast linting**: Powered by Rust and tree-sitter for exceptional performance
- **CommonMark compliance**: Follows CommonMark specification
- **Real-time diagnostics**: Get instant feedback as you edit
- **Configurable rules**: Customize linting rules via `quickmark.toml`
- **LSP integration**: Full Language Server Protocol support for robust editor integration

## Installation

### Simple Installation

1. Install the extension from the VSCode marketplace
2. Start using it immediately - the extension includes pre-built QuickMark server binaries for all major platforms

The extension automatically detects your platform and uses the appropriate bundled binary.

### Advanced Installation

If you prefer to use your own QuickMark server binary:

1. Build from source:
   ```bash
   git clone https://github.com/quickmark/quickmark.git
   cd quickmark
   cargo build --release --bin quickmark_server
   ```

2. Configure the extension to use your binary:
   ```json
   {
     "quickmark.serverPath": "/path/to/your/quickmark_server"
   }
   ```

## Configuration

The extension can be configured via VSCode settings:

### Extension Settings

- `quickmark.serverPath`: Path to custom QuickMark LSP server executable (default: `""` - uses bundled binary)
- `quickmark.configPath`: Path to quickmark.toml configuration file (optional)
- `quickmark.trace.server`: LSP communication tracing level (`"off"`, `"messages"`, `"verbose"`)
- `quickmark.lintOnSave`: Whether to lint files on save (default: `true`)
- `quickmark.lintOnType`: Whether to lint as you type (default: `false`)

### QuickMark Configuration

Create a `quickmark.toml` file in your workspace root to configure linting rules:

```toml
[linters.severity]
# Configure rule severities: "error", "warning", or "off"
line-length = "warning"
heading-increment = "error"
no-duplicate-heading = "warning"

[linters.line-length]
# MD013: Line length configuration
limit = 120
heading_limit = 120
code_block_limit = 120
```

## Commands

- **QuickMark: Restart Server**: Restart the LSP server
- **QuickMark: Show Output**: Show the extension's output channel

## Usage

1. Open a Markdown file (`.md`, `.markdown`, etc.)
2. The extension will automatically start linting
3. Diagnostics will appear as squiggly underlines
4. Hover over issues to see detailed messages
5. Rule codes (e.g., `line-length`, `heading-increment`) are shown for reference

## Supported File Types

- `.md`
- `.markdown`
- `.mdown`
- `.mkdn`
- `.mkd`
- `.mdwn`
- `.mdtxt`
- `.mdtext`

## Performance

QuickMark is designed for speed:

- **Rust backend**: Native performance with minimal resource usage
- **Tree-sitter parsing**: Efficient AST-based analysis
- **Single-pass architecture**: Optimized for large documents
- **Configurable linting**: Enable only the rules you need

## Troubleshooting

### Server Not Starting

The extension includes pre-built binaries for all major platforms. If you encounter issues:

1. Check the output channel for detailed error messages: `View > Output > QuickMark`

2. Verify your platform is supported:
   - Windows (x64, x86)
   - macOS (Intel, Apple Silicon)
   - Linux (x64, ARM64)

3. If using a custom server path, ensure it's accessible:
   ```bash
   /your/path/to/quickmark_server --help
   ```

4. Try clearing the server path setting to use the bundled binary:
   ```json
   {
     "quickmark.serverPath": ""
   }
   ```

### Configuration Issues

1. Ensure `quickmark.toml` is in your workspace root
2. Check the configuration syntax matches the expected TOML format
3. Use the restart command after configuration changes

### Performance Issues

1. Disable `quickmark.lintOnType` if typing feels slow
2. Use `quickmark.lintOnSave` for better performance
3. Configure rule severities to disable unused rules

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed development setup, testing instructions, and contribution guidelines.

### Quick Start for Contributors

```bash
# Clone and setup
git clone https://github.com/quickmark/quickmark.git
cd quickmark/vscode-quickmark
npm install

# Build server and copy binary
cd .. && cargo build --release --bin quickmark_server
cd vscode-quickmark && npm run copy-local-binary

# Start developing
npm run compile
# Press F5 in VSCode to run extension
```

## License

This extension is part of the QuickMark project. See the main repository for license information.

## Changelog

### 0.0.1

- Initial release
- Basic LSP integration with quickmark_server
- Configuration support for rules and severity levels
- Commands for server management