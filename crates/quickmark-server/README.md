# quickmark-server

Lightning-fast Markdown/CommonMark linter LSP server for editor integration.

## Overview

`quickmark-server` provides a Language Server Protocol (LSP) implementation for QuickMark, enabling real-time Markdown linting in editors and IDEs that support LSP.

## Features

- **LSP Protocol**: Full Language Server Protocol support
- **Real-time Analysis**: Live document linting as you type
- **Async Processing**: Built with tokio for high performance
- **Editor Integration**: Works with VS Code, Neovim, Emacs, and other LSP-compatible editors
- **Configuration Support**: Respects `quickmark.toml` configuration files

## Installation

```bash
cargo install quickmark-server
```

## Usage

The server is typically started by your editor's LSP client. For manual testing:

```bash
quickmark-server
```

## Editor Integration

### VS Code
Install the QuickMark VS Code extension (coming soon) or configure manually:

```json
{
  "quickmark.serverPath": "/path/to/quickmark-server"
}
```

### Neovim
Add to your LSP configuration:

```lua
require'lspconfig'.quickmark.setup{}
```

### Other Editors
Configure your LSP client to use `quickmark-server` as the language server for Markdown files.

## Configuration

The server uses the same `quickmark.toml` configuration format as the CLI tool, automatically detecting configuration files in your project.

## License

MIT