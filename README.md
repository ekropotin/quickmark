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

QuickMark looks for configuration in the following order:

1. **Environment Variable**: If `QUICKMARK_CONFIG` environment variable is set, it uses the config file at the specified path
2. **Local Config**: If not found, it looks for `quickmark.toml` in the current working directory  
3. **Default**: If neither is found, default configuration is used

#### Using QUICKMARK_CONFIG Environment Variable

You can specify a custom configuration file location using the `QUICKMARK_CONFIG` environment variable:

```shell
# Set config file path
export QUICKMARK_CONFIG="/path/to/your/custom-config.toml"
qmark file.md

# Or use it inline
QUICKMARK_CONFIG="/path/to/custom-config.toml" qmark file.md
```

This is especially useful for:
- Shared configurations across multiple projects
- CI/CD pipelines with centralized configs
- Different config files for different environments

Below is a full configuration with default values:

```toml
[linters.severity]
# possible values are: 'warn', 'err' and 'off'
heading-increment = 'err'
heading-style = 'err'
ul-style = 'err'
list-indent = 'err'
ul-indent = 'err'
line-length = 'err'
no-missing-space-atx = 'err'
no-missing-space-closed-atx = 'err'
no-multiple-space-atx = 'err'
no-multiple-space-closed-atx = 'err'
blanks-around-headings = 'err'
heading-start-left = 'err'
single-h1 = 'err'
no-trailing-punctuation = 'err'
no-multiple-space-blockquote = 'err'
blanks-around-fences = 'err'
blanks-around-lists = 'err'
no-duplicate-heading = 'err'
required-headings = 'err'
code-block-style = 'err'
code-fence-style = 'err'
link-fragments = 'warn'
reference-links-images = 'err'
link-image-reference-definitions = 'err'

# see a specific rule's doc for details of configuration
[linters.settings.heading-style]
style = 'consistent'

[linters.settings.ul-style]
style = 'consistent'

[linters.settings.ul-indent]
indent = 2
start_indent = 2
start_indented = false

[linters.settings.line-length]
line_length = 80
code_blocks = true
headings = true
tables = true
strict = false
stern = false

[linters.settings.blanks-around-headings]
lines_above = [1]
lines_below = [1]

[linters.settings.blanks-around-fences]
list_items = true

[linters.settings.no-duplicate-heading]
siblings_only = false
allow_different_nesting = false

[linters.settings.single-h1]
level = 1
front_matter_title = '^\s*title\s*[:=]'

[linters.settings.no-trailing-punctuation]
punctuation = '.,;:!。，；：！'

[linters.settings.link-fragments]
ignore_case = false
ignored_pattern = ""

[linters.settings.reference-links-images]
shortcut_syntax = false
ignored_labels = ["x"]

[linters.settings.required-headings]
headings = []
match_case = false

[linters.settings.link-image-reference-definitions]
ignored_definitions = ["//"]

[linters.settings.no-inline-html]
allowed_elements = []

[linters.settings.fenced-code-language]
allowed_languages = []
language_only = false

[linters.settings.code-block-style]
style = 'consistent'

[linters.settings.code-fence-style]
style = 'consistent'
```

## Rules

**Implementation Progress: 33/52 rules completed (63.5%)**

- [x] **[MD001](docs/rules/md001.md)** *heading-increment* - Heading levels should only increment by one level at a time
- [x] **[MD003](docs/rules/md003.md)** *heading-style* - Consistent heading styles
- [x] **[MD004](docs/rules/md004.md)** *ul-style* - Unordered list style consistency
- [x] **[MD005](docs/rules/md005.md)** *list-indent* - Inconsistent indentation for list items at the same level
- [x] **[MD007](docs/rules/md007.md)** *ul-indent* - Unordered list indentation consistency
- [x] **[MD009](docs/rules/md009.md)** *no-trailing-spaces* - Trailing spaces at end of lines
- [x] **[MD010](docs/rules/md010.md)** *no-hard-tabs* - Hard tabs should not be used
- [x] **[MD011](docs/rules/md011.md)** *no-reversed-links* - Reversed link syntax
- [x] **[MD012](docs/rules/md012.md)** *no-multiple-blanks* - Multiple consecutive blank lines
- [x] **[MD013](docs/rules/md013.md)** *line-length* - Line length limits with configurable exceptions
- [x] **[MD014](docs/rules/md014.md)** *commands-show-output* - Dollar signs before shell commands
- [x] **[MD018](docs/rules/md018.md)** *no-missing-space-atx* - Space after hash in ATX headings
- [x] **[MD019](docs/rules/md019.md)** *no-multiple-space-atx* - Multiple spaces after hash in ATX headings
- [x] **[MD020](docs/rules/md020.md)** *no-missing-space-closed-atx* - Space inside closed ATX headings
- [x] **[MD021](docs/rules/md021.md)** *no-multiple-space-closed-atx* - Multiple spaces in closed ATX headings
- [x] **[MD022](docs/rules/md022.md)** *blanks-around-headings* - Headings surrounded by blank lines
- [x] **[MD023](docs/rules/md023.md)** *heading-start-left* - Headings must start at the beginning of the line
- [x] **[MD024](docs/rules/md024.md)** *no-duplicate-heading* - Multiple headings with same content
- [x] **[MD025](docs/rules/md025.md)** *single-h1* - Multiple top-level headings
- [x] **[MD026](docs/rules/md026.md)** *no-trailing-punctuation* - Trailing punctuation in headings
- [x] **[MD027](docs/rules/md027.md)** *no-multiple-space-blockquote* - Multiple spaces after blockquote symbol
- [x] **[MD028](docs/rules/md028.md)** *no-blanks-blockquote* - Blank lines inside blockquotes
- [ ] **MD029** *ol-prefix* - Ordered list item prefix consistency
- [x] **[MD030](docs/rules/md030.md)** *list-marker-space* - Spaces after list markers
- [x] **[MD031](docs/rules/md031.md)** *blanks-around-fences* - Fenced code blocks surrounded by blank lines
- [x] **[MD032](docs/rules/md032.md)** *blanks-around-lists* - Lists surrounded by blank lines
- [x] **[MD033](docs/rules/md033.md)** *no-inline-html* - Inline HTML usage
- [x] **[MD034](docs/rules/md034.md)** *no-bare-urls* - Bare URLs without proper formatting
- [x] **[MD035](docs/rules/md035.md)** *hr-style* - Horizontal rule style consistency
- [x] **[MD036](docs/rules/md036.md)** *no-emphasis-as-heading* - Emphasis used instead of heading
- [x] **[MD037](docs/rules/md037.md)** *no-space-in-emphasis* - Spaces inside emphasis markers
- [x] **[MD038](docs/rules/md038.md)** *no-space-in-code* - Spaces inside code span elements
- [ ] **MD039** *no-space-in-links* - Spaces inside link text
- [x] **MD040** *fenced-code-language* - Language specified for fenced code blocks
- [ ] **MD041** *first-line-heading* - First line should be top-level heading
- [ ] **MD042** *no-empty-links* - Empty links
- [x] **[MD043](docs/rules/md043.md)** *required-headings* - Required heading structure
- [ ] **MD044** *proper-names* - Proper names with correct capitalization
- [ ] **MD045** *no-alt-text* - Images should have alternate text
- [x] **[MD046](docs/rules/md046.md)** *code-block-style* - Code block style consistency
- [ ] **MD047** *single-trailing-newline* - Files should end with a single newline
- [x] **[MD048](docs/rules/md048.md)** *code-fence-style* - Code fence style consistency
- [ ] **MD049** *emphasis-style* - Emphasis style consistency
- [ ] **MD050** *strong-style* - Strong style consistency
- [x] **[MD051](docs/rules/md051.md)** *link-fragments* - Link fragments should be valid
- [x] **[MD052](docs/rules/md052.md)** *reference-links-images* - Reference links should be defined
- [x] **[MD053](docs/rules/md053.md)** *link-image-reference-definitions* - Reference definitions should be needed
- [ ] **MD054** *link-image-style* - Link and image style
- [ ] **MD055** *table-pipe-style* - Table pipe style
- [ ] **MD056** *table-column-count* - Table column count
- [ ] **MD058** *blanks-around-tables* - Tables should be surrounded by blank lines
- [ ] **MD059** *descriptive-link-text* - Link text should be descriptive
