# Contributing to QuickMark

First off, thank you for considering contributing to QuickMark! Your contributions are what make this project better. Whether you're reporting bugs, adding new features, or improving documentation, we appreciate your help.

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue and include as much detail as possible:

- Steps to reproduce the issue
- The expected behavior
- The actual behavior
- Screenshots or logs, if applicable
- Your environment (OS, Rust version, etc.)

### Suggesting Enhancements

We welcome suggestions for improvements. Please open an issue and provide:

- A clear and descriptive title
- A detailed description of the proposed enhancement
- Any relevant context or examples

### Submitting Pull Requests

If you'd like to contribute code to QuickMark, please follow these steps:

1. **Fork the Repository**
   - Click the "Fork" button at the top of this repository.

2. **Clone Your Fork**

   ```sh
   git clone https://github.com/your-username/quickmark.git
   cd quickmark
   ```

3. **Create a Branch**

Use descriptive branch names that include the issue number:

**Format:** `type/issue-number-short-description`

- `feature/123-add-md025-rule` - New features
- `fix/456-line-ending-detection` - Bug fixes
- `docs/789-update-contributing-guide` - Documentation changes
- `chore/101-update-dependencies` - Maintenance tasks
- `refactor/202-rule-system-cleanup` - Code improvements

```sh
git checkout -b feature/123-your-feature-name
```

If working on something without an existing issue, please create one first to get an issue number.

4. **Make Changes**

- implement your feature or bugfix.
- Ensure your code follows the project’s coding standards.
- Add or update tests as necessary.
- Run tests to ensure everything works correctly.

5. **Commit Changes**

Follow the [Conventional Commits](https://www.conventionalcommits.org/) standard for commit messages:

**Format:** `type(scope): description`

Examples:
- `feat(md025): add single H1 rule implementation`
- `fix(parser): handle edge case in heading detection`
- `docs(contributing): update branch naming guidelines`
- `test(md013): add comprehensive line length tests`

If your commit relates to a specific issue, include `Fixes #123` in the commit body:

```sh
git add .
git commit -m "feat(md025): add single H1 rule implementation

Implements the MD025 rule that ensures documents have only one H1 heading.
Includes comprehensive tests and documentation.

Fixes #123"
```

6. **Push to Your Fork**

```sh
git push origin feature-or-bugfix-description
```

7. **Open a Pull Request**

- Go to the original repository on GitHub.
- Click on the "Pull Requests" tab and then the "New Pull Request" button.
- Provide a clear title and description of your changes.

## Code Style and Standards

- Follow Rust’s standard formatting with rustfmt.
- Write meaningful commit messages.
- Ensure your code is well-documented.

## Linting Rules

QuickMark is in the early stages of porting rules from [markdownlint](https://github.com/markdownlint/markdownlint). If you're interested in helping with this effort:

- Check the list of rules that need to be ported in the issues section.
- Follow the guidelines in existing rule implementations.

## Code of Conduct

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project, you agree to abide by its terms.

## Getting Help

If you have any questions or need assistance, feel free to open an issue or contact the maintainers.

## Thank You

Thank you for your interest in contributing to QuickMark! We look forward to your contributions and appreciate your support in improving the project.
