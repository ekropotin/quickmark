# QuickMark

[![image](https://img.shields.io/badge/license-MIT-blue)](https://github.com/ekropotin/quickmark/blob/main/LICENSE)

> **Notice:** This project is at super early stage of development. Expect frequent updates and breaking changes.

An lightning-fast linter for Markdown/[CommonMark](https://commonmark.org/) files, written in Rust.

QuickMark is not just another Markdown linter; it's a tool designed with the modern developer in mind. By prioritizing speed and integrating seamlessly with your development environment, QuickMark enhances your productivity and makes Markdown linting an effortless part of your workflow.

QuickMark takes a lot of inspiration from Mark Harrison's [markdownlint](https://github.com/markdownlint/markdownlint) for Ruby. We love how thorough and reliable markdownlint is, and we're just getting started with porting its rules over to QuickMark. While the project is still in its early stages, our goal is to eventually bring all the markdownlint rules into QuickMark.

## Key features

- **Rust-Powered Speed**: Leveraging the power of Rust, QuickMark offers exceptional performance, making linting operations swift and efficient, even for large Markdown files.
- **LSP Integration**: QuickMark integrates effortlessly with your favorite code editors through LSP, providing real-time feedback and linting suggestions directly within your editor.
- **Customizable Rules**: Tailor the linting rules to fit your project's specific needs, ensuring that your Markdown files adhere to your preferred style and standards.

## Getting Started

### Installation

At this point, the only way to get the binary is building it from the sources:

```shell
git clone git@github.com:ekropotin/quickmark.git
cd quickmark
cargo build --release
```

This command will generate the `quickmark` binary in the `./target/release` directory.

### Usage

Lint a single file:

```shell
quickmark /path/to/file.md
```

### Configuration

Quickmark looks up for `quickmark.toml` configuration file in the current working directory. If the file was not found, the default is used.

Below is a full configuration with default values:

```toml
[linters.severity]
# possible values are: 'warn', 'err' and 'off'
heading-increment = 'err'
heading-style = 'err'

# see a specific rule's doc for details of configuration
[linters.settings.heading-style]
style = 'consistent'
```

## Rules

- **[MD001](docs/rules/md001.md)** *heading-increment* - Heading levels should only increment by one level at a time
- **[MD003](docs/rules/md003.md)** *heading-style* - Heading style
