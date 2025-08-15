# MD038 Valid Test Cases

This file contains valid code spans that should not trigger MD038 violations.

## No Spaces

Simple code spans: `code`, `another`, `third`.

## Single Space Padding

Code spans with single space padding are allowed: ` code `, ` another `, ` third `.

This is necessary for: `` ` `` (backtick display).

## Code Spans with Only Whitespace

These are allowed: `   `, `	`, ` 	 `.

## Empty Code Spans

Empty code spans: ``, ````.

## Multiple Backtick Code Spans

Double backticks: ``code``, ``another``.

Triple backticks: ```code```.

## Valid in Different Contexts

- List item with `valid` code span
- Another item with ` valid ` single space padding

> Blockquote with `valid` code span

**Bold with `valid` code**

*Italic with `valid` code*

[Link with `valid` code](http://example.com)

## Complex Valid Cases

Code span with content that starts/ends with non-whitespace: `a `, ` b`, `a b`.

Mixed content: `code and text`.