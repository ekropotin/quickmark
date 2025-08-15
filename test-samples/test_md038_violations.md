# MD038 Violations Test Cases

This file contains code spans that should trigger MD038 violations.

## Multiple Leading Spaces

Code spans with multiple leading spaces: `  code`, `   another`.

## Multiple Trailing Spaces

Code spans with multiple trailing spaces: `code  `, `another   `.

## Both Leading and Trailing Spaces

Code spans with both: `  code  `, `   another   `.

## Tabs Instead of Spaces

Code spans with tabs: `	code	`, `		another		`.

## Mixed Whitespace

Code spans with mixed whitespace: ` 	code	 `, `  	another 	 `.

## Double Backtick Violations

Double backticks with spaces: ``  code  ``, ``   another   ``.

## Multiple Code Spans on Same Line

Valid and invalid: `valid` and `  invalid  `.

## Violations in Different Contexts

- List item with `  invalid  ` code span
- Another item with `	tab	` code span

> Blockquote with `  invalid  ` code span

**Bold with `  invalid  ` code**

*Italic with `  invalid  ` code*

[Link with `  invalid  ` code](http://example.com)

## Complex Violation Cases

Multiple violations in one span: `   code with lots of spaces   `.

Mixed valid and invalid: `valid`, `  invalid  `, `also valid`.