# QuickMark

[![image](https://img.shields.io/badge/license-MIT-blue)](https://github.com/ekropotin/quickmark/blob/main/LICENSE)

> **Notice:** This project is at super early stage of development. Expect frequent updates and breaking changes.

An lightning-fast linter for Markdown/[CommonMark](https://commonmark.org/) files, written in Rust.

QuickMark is not just another Markdown linter; it's a tool designed with the modern developer in mind. By prioritizing speed and integrating seamlessly with your development environment, QuickMark enhances your productivity and makes Markdown linting an effortless part of your workflow.

This project takes a lot of inspiration from David Anson's [markdownlint](https://github.com/DavidAnson/markdownlint). Our goal is to match its supported rules and behavior as closely as possible. When a rule is ambiguous or its behavior isn’t explicitly defined, we rely on the following specifications as the ultimate sources of truth:

- [CommonMark](https://spec.commonmark.org/current/)
- [GitHub Flavored Markdown Spec](https://github.github.com/gfm/)

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
3. **Default**: If neither is found, [default configuration](#default-configuration) is used

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

#### Default configuration

```toml
[linters.severity]
# possible values are: 'warn', 'err' and 'off'
heading-increment = 'err'
heading-style = 'err'
ul-style = 'err'
list-indent = 'err'
ul-indent = 'err'
no-trailing-spaces = 'err'
no-hard-tabs = 'err'
no-reversed-links = 'err'
no-multiple-blanks = 'err'
line-length = 'err'
commands-show-output = 'err'
no-missing-space-atx = 'err'
no-multiple-space-atx = 'err'
no-missing-space-closed-atx = 'err'
no-multiple-space-closed-atx = 'err'
blanks-around-headings = 'err'
heading-start-left = 'err'
no-duplicate-heading = 'err'
single-h1 = 'err'
no-trailing-punctuation = 'err'
no-multiple-space-blockquote = 'err'
no-blanks-blockquote = 'err'
ol-prefix = 'err'
list-marker-space = 'err'
blanks-around-fences = 'err'
blanks-around-lists = 'err'
no-inline-html = 'err'
no-bare-urls = 'err'
hr-style = 'err'
no-emphasis-as-heading = 'err'
no-space-in-emphasis = 'err'
no-space-in-code = 'err'
no-space-in-links = 'err'
fenced-code-language = 'err'
first-line-heading = 'err'
no-empty-links = 'err'
proper-names = 'err'
required-headings = 'err'
no-alt-text = 'err'
code-block-style = 'err'
single-trailing-newline = 'err'
code-fence-style = 'err'
emphasis-style = 'err'
strong-style = 'err'
link-fragments = 'warn'
reference-links-images = 'err'
link-image-reference-definitions = 'err'
link-image-style = 'err'
table-pipe-style = 'err'
table-column-count = 'err'
blanks-around-tables = 'err'
descriptive-link-text = 'err'

# see a specific rule's doc for details of configuration
[linters.settings.heading-style]
style = 'consistent'

[linters.settings.ul-style]
style = 'consistent'

[linters.settings.ol-prefix]
style = 'one_or_ordered'

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

[linters.settings.first-line-heading]
allow_preamble = false
front_matter_title = '^\s*title\s*[:=]'
level = 1

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

[linters.settings.proper-names]
names = []
code_blocks = true
html_elements = true

[linters.settings.fenced-code-language]
allowed_languages = []
language_only = false

[linters.settings.code-block-style]
style = 'consistent'

[linters.settings.code-fence-style]
style = 'consistent'

[linters.settings.table-pipe-style]
style = 'consistent'

[linters.settings.no-trailing-spaces]
br_spaces = 2
list_item_empty_lines = false
strict = false

[linters.settings.no-hard-tabs]
code_blocks = true
ignore_code_languages = []
spaces_per_tab = 1

[linters.settings.no-multiple-blanks]
maximum = 1

[linters.settings.list-marker-space]
ul_single = 1
ol_single = 1
ul_multi = 1
ol_multi = 1

[linters.settings.hr-style]
style = 'consistent'

[linters.settings.no-emphasis-as-heading]
punctuation = '.,;:!?。，；：！？'

[linters.settings.emphasis-style]
style = 'consistent'

[linters.settings.strong-style]
style = 'consistent'

[linters.settings.link-image-style]
autolink = true
inline = true
full = true
collapsed = true
shortcut = true
url_inline = true

[linters.settings.descriptive-link-text]
prohibited_texts = ["click here", "here", "link", "more"]
```

## Rules

- **[MD001](docs/rules/md001.md)** *heading-increment* - Heading levels should only increment by one level at a time
- **[MD003](docs/rules/md003.md)** *heading-style* - Consistent heading styles
- **[MD004](docs/rules/md004.md)** *ul-style* - Unordered list style consistency
- **[MD005](docs/rules/md005.md)** *list-indent* - Inconsistent indentation for list items at the same level
- **[MD007](docs/rules/md007.md)** *ul-indent* - Unordered list indentation consistency
- **[MD009](docs/rules/md009.md)** *no-trailing-spaces* - Trailing spaces at end of lines
- **[MD010](docs/rules/md010.md)** *no-hard-tabs* - Hard tabs should not be used
- **[MD011](docs/rules/md011.md)** *no-reversed-links* - Reversed link syntax
- **[MD012](docs/rules/md012.md)** *no-multiple-blanks* - Multiple consecutive blank lines
- **[MD013](docs/rules/md013.md)** *line-length* - Line length limits with configurable exceptions
- **[MD014](docs/rules/md014.md)** *commands-show-output* - Dollar signs before shell commands
- **[MD018](docs/rules/md018.md)** *no-missing-space-atx* - Space after hash in ATX headings
- **[MD019](docs/rules/md019.md)** *no-multiple-space-atx* - Multiple spaces after hash in ATX headings
- **[MD020](docs/rules/md020.md)** *no-missing-space-closed-atx* - Space inside closed ATX headings
- **[MD021](docs/rules/md021.md)** *no-multiple-space-closed-atx* - Multiple spaces in closed ATX headings
- **[MD022](docs/rules/md022.md)** *blanks-around-headings* - Headings surrounded by blank lines
- **[MD023](docs/rules/md023.md)** *heading-start-left* - Headings must start at the beginning of the line
- **[MD024](docs/rules/md024.md)** *no-duplicate-heading* - Multiple headings with same content
- **[MD025](docs/rules/md025.md)** *single-h1* - Multiple top-level headings
- **[MD026](docs/rules/md026.md)** *no-trailing-punctuation* - Trailing punctuation in headings
- **[MD027](docs/rules/md027.md)** *no-multiple-space-blockquote* - Multiple spaces after blockquote symbol
- **[MD028](docs/rules/md028.md)** *no-blanks-blockquote* - Blank lines inside blockquotes
- **[MD029](docs/rules/md029.md)** *ol-prefix* - Ordered list item prefix consistency
- **[MD030](docs/rules/md030.md)** *list-marker-space* - Spaces after list markers
- **[MD031](docs/rules/md031.md)** *blanks-around-fences* - Fenced code blocks surrounded by blank lines
- **[MD032](docs/rules/md032.md)** *blanks-around-lists* - Lists surrounded by blank lines
- **[MD033](docs/rules/md033.md)** *no-inline-html* - Inline HTML usage
- **[MD034](docs/rules/md034.md)** *no-bare-urls* - Bare URLs without proper formatting
- **[MD035](docs/rules/md035.md)** *hr-style* - Horizontal rule style consistency
- **[MD036](docs/rules/md036.md)** *no-emphasis-as-heading* - Emphasis used instead of heading
- **[MD037](docs/rules/md037.md)** *no-space-in-emphasis* - Spaces inside emphasis markers
- **[MD038](docs/rules/md038.md)** *no-space-in-code* - Spaces inside code span elements
- **[MD039](docs/rules/md039.md)** *no-space-in-links* - Spaces inside link text
- **[MD040](docs/rules/md040.md)** *fenced-code-language* - Language specified for fenced code blocks
- **[MD041](docs/rules/md041.md)** *first-line-heading* - First line should be top-level heading
- **[MD042](docs/rules/md042.md)** *no-empty-links* - Empty links
- **[MD043](docs/rules/md043.md)** *required-headings* - Required heading structure
- **[MD044](docs/rules/md044.md)** *proper-names* - Proper names with correct capitalization
- **[MD045](docs/rules/md045.md)** *no-alt-text* - Images should have alternate text
- **[MD046](docs/rules/md046.md)** *code-block-style* - Code block style consistency
- **[MD047](docs/rules/md047.md)** *single-trailing-newline* - Files should end with a single newline
- **[MD048](docs/rules/md048.md)** *code-fence-style* - Code fence style consistency
- **[MD049](docs/rules/md049.md)** *emphasis-style* - Emphasis style consistency
- **[MD050](docs/rules/md050.md)** *strong-style* - Strong style consistency
- **[MD051](docs/rules/md051.md)** *link-fragments* - Link fragments should be valid
- **[MD052](docs/rules/md052.md)** *reference-links-images* - Reference links should be defined
- **[MD053](docs/rules/md053.md)** *link-image-reference-definitions* - Reference definitions should be needed
- **[MD054](docs/rules/MD054.md)** *link-image-style* - Link and image style
- **[MD055](docs/rules/md055.md)** *table-pipe-style* - Table pipe style
- **[MD056](docs/rules/md056.md)** *table-column-count* - Table column count
- **[MD058](docs/rules/md058.md)** *blanks-around-tables* - Tables should be surrounded by blank lines
- **[MD059](docs/rules/md059.md)** *descriptive-link-text* - Link text should be descriptive