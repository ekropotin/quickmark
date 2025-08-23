# Homebrew Tap for QuickMark

This is the official Homebrew tap for QuickMark, a lightning-fast Markdown/CommonMark linter.

## Installation

```bash
# Add the tap
brew tap ekropotin/quickmark

# Install quickmark-cli
brew install quickmark-cli
```

## Usage

After installation, the CLI tool is available as `qmark`:

```bash
# Lint a single file
qmark README.md

# Lint multiple files
qmark *.md

# Lint with custom config
qmark --config quickmark.toml *.md
```

## Updating

```bash
brew update
brew upgrade quickmark-cli
```

## Uninstall

```bash
brew uninstall quickmark-cli
brew untap ekropotin/quickmark
```
