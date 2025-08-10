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

This command will generate the `qmark` binary in the `./target/release` directory.

### Usage

Lint a single file:

```shell
qmark /path/to/file.md
```

### Configuration

Quickmark looks up for `quickmark.toml` configuration file in the current working directory. If the file was not found, the default is used.

Below is a full configuration with default values:

```toml
[linters.severity]
# possible values are: 'warn', 'err' and 'off'
heading-increment = 'err'
heading-style = 'err'
line-length = 'err'
no-duplicate-heading = 'err'
link-fragments = 'warn'
reference-links-images = 'err'
link-image-reference-definitions = 'err'

# see a specific rule's doc for details of configuration
[linters.settings.heading-style]
style = 'consistent'

[linters.settings.line-length]
line_length = 80
code_blocks = true
headings = true
tables = true
strict = false
stern = false

[linters.settings.no-duplicate-heading]
siblings_only = false
allow_different_nesting = false

[linters.settings.link-fragments]
ignore_case = false
ignored_pattern = ""

[linters.settings.reference-links-images]
shortcut_syntax = false
ignored_labels = ["x"]

[linters.settings.link-image-reference-definitions]
ignored_definitions = ["//"]
```

## Rules

**Implementation Progress: 8/48 rules completed (16.7%)**

- [x] **[MD001](docs/rules/md001.md)** *heading-increment* - Heading levels should only increment by one level at a time
- [x] **[MD003](docs/rules/md003.md)** *heading-style* - Consistent heading styles
- [ ] **MD004** *ul-style* - Unordered list style consistency
- [ ] **MD005** *list-indent* - List item indentation at same level
- [ ] **MD006** *ul-start-left* - Bulleted lists start at beginning of line
- [ ] **MD007** *ul-indent* - Unordered list indentation consistency
- [ ] **MD009** *no-trailing-spaces* - Trailing spaces at end of lines
- [ ] **MD010** *no-hard-tabs* - Hard tabs should not be used
- [ ] **MD011** *no-reversed-links* - Reversed link syntax
- [ ] **MD012** *no-multiple-blanks* - Multiple consecutive blank lines
- [x] **[MD013](docs/rules/md013.md)** *line-length* - Line length limits with configurable exceptions
- [ ] **MD014** *commands-show-output* - Dollar signs before shell commands
- [ ] **MD018** *no-missing-space-atx* - Space after hash in ATX headings
- [ ] **MD019** *no-multiple-space-atx* - Multiple spaces after hash in ATX headings
- [ ] **MD020** *no-missing-space-closed-atx* - Space inside closed ATX headings
- [ ] **MD021** *no-multiple-space-closed-atx* - Multiple spaces in closed ATX headings
- [ ] **MD022** *blanks-around-headings* - Headings surrounded by blank lines
- [ ] **MD023** *heading-start-left* - Headings start at beginning of line
- [x] **[MD024](docs/rules/md024.md)** *no-duplicate-heading* - Multiple headings with same content
- [ ] **MD025** *single-title* - Multiple top-level headings
- [ ] **MD026** *no-trailing-punctuation* - Trailing punctuation in headings
- [ ] **MD027** *no-multiple-space-blockquote* - Multiple spaces after blockquote
- [ ] **MD028** *no-blanks-blockquote* - Blank lines inside blockquotes
- [ ] **MD029** *ol-prefix* - Ordered list item prefix consistency
- [ ] **MD030** *list-marker-space* - Spaces after list markers
- [ ] **MD031** *blanks-around-fences* - Fenced code blocks surrounded by blank lines
- [ ] **MD032** *blanks-around-lists* - Lists surrounded by blank lines
- [ ] **MD033** *no-inline-html* - Inline HTML usage
- [x] **[MD034](docs/rules/md034.md)** *no-bare-urls* - Bare URLs without proper formatting
- [ ] **MD035** *hr-style* - Horizontal rule style consistency
- [ ] **MD036** *no-emphasis-as-heading* - Emphasis used instead of heading
- [ ] **MD037** *no-space-in-emphasis* - Spaces inside emphasis markers
- [ ] **MD038** *no-space-in-code* - Spaces inside code span elements
- [ ] **MD039** *no-space-in-links* - Spaces inside link text
- [ ] **MD040** *fenced-code-language* - Language specified for fenced code blocks
- [ ] **MD041** *first-line-heading* - First line should be top-level heading
- [ ] **MD042** *no-empty-links* - Empty links
- [ ] **MD043** *required-headings* - Required heading structure
- [ ] **MD044** *proper-names* - Proper names with correct capitalization
- [ ] **MD045** *no-alt-text* - Images should have alternate text
- [ ] **MD046** *code-block-style* - Code block style consistency
- [ ] **MD047** *single-trailing-newline* - Files should end with a single newline
- [ ] **MD048** *code-fence-style* - Code fence style consistency
- [ ] **MD049** *emphasis-style* - Emphasis style consistency
- [ ] **MD050** *strong-style* - Strong style consistency
- [x] **[MD051](docs/rules/md051.md)** *link-fragments* - Link fragments should be valid
- [x] **[MD052](docs/rules/md052.md)** *reference-links-images* - Reference links should be defined
- [x] **[MD053](docs/rules/md053.md)** *link-image-reference-definitions* - Reference definitions should be needed
