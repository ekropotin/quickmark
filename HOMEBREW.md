# Homebrew Installation

QuickMark CLI can be installed via Homebrew on macOS.

## Installation

```bash
# Add the tap (this repository)
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

## How it works

The Homebrew formula is located at `pkg/homebrew/Formula/quickmark-cli.rb` with a `HomebrewFormula` symlink in the root for easy access. The formula downloads pre-compiled binaries for both Intel and Apple Silicon Macs.

## Repository Structure

```
quickmark/
├── HomebrewFormula -> pkg/homebrew/  # Symlink for Homebrew tap
└── pkg/
    └── homebrew/
        ├── Formula/
        │   └── quickmark-cli.rb
        └── README.md
```