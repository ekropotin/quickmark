# Homebrew Tap for QuickMark

This directory contains the Homebrew formula for QuickMark CLI.

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

## Formula Details

- **Location**: `Formula/quickmark-cli.rb`
- **Binary name**: `qmark`
- **Architecture support**: Intel and Apple Silicon Macs
- **Installation method**: Pre-compiled binaries from GitHub releases

## Maintenance

When releasing a new version:

1. Update the version and URLs in `Formula/quickmark-cli.rb`
2. Update the SHA256 hashes for both architectures
3. Test the formula: `brew install --build-from-source ./Formula/quickmark-cli.rb`
4. Commit and push the changes

### Testing changes locally

```bash
brew tap-new local/quickmark-test
cp pkg/homebrew/Formula/quickmark-cli.rb $(brew --repo local/quickmark-test)/Formula/
brew install --build-from-source local/quickmark-test/quickmark-cli
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
