If there are unstaged changes:
Inspect all unstaged files and determine whether any should be excluded from source control—for example: build artifacts, logs, IDE configs (e.g., .vscode/), OS-generated files (e.g., .DS_Store), or dependency directories like node_modules/.
If any such files are found, add appropriate patterns to .gitignore and stage the updated .gitignore. And then commit these changes.

Inspect all changes on the current branch vs main, and if needed update CLAUDE.md file based on the diff. Then commit this update.

And finally, squash all changes on the branch with commit message, based on the diff, using the Conventional Commit format.
