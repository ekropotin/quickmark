Inspect all unstaged files and determine whether any should be excluded from source control—for example: build artifacts, logs, IDE configs (e.g., .vscode/), OS-generated files (e.g., .DS_Store), or dependency directories like node_modules/.
If any such files are found, add appropriate patterns to .gitignore and stage the updated .gitignore.
Then stage all remaining changes and generate a commit using the Conventional Commits format, based on the full diff.
