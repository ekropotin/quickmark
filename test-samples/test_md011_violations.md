# MD011 Violations Examples

This file contains examples that SHOULD trigger the MD011 rule.

## Basic Reversed Links

This is a (reversed)[link] example that should be flagged.

Here are multiple (bad)[example] and (another)[violation] cases.

## At Start of Line

(reversed)[link] at the start of a line.

## Mixed with Correct Links

This has both [correct](link) and (incorrect)[example] patterns.

Visit [GitHub](https://github.com) but avoid (bad)[links].

## Complex URLs

Visit (GitHub)[https://github.com/user/repo#section] for more info.

Check (this page)[https://example.com/path?param=value&other=123] for details.

## Multiple on Same Line

Here is (one)[link] and (another)[example] on the same line.

## Different Contexts

In a paragraph with (reversed)[syntax] that needs fixing.

*Emphasis* with (bad)[link] inside.

**Bold** text and (another)[violation] example.

## List Items

- Item with (reversed)[link] syntax
- Another item with (bad)[example]
  - Nested with (wrong)[pattern]

1. Numbered item with (incorrect)[syntax]
2. Another numbered with (bad)[link]

## Tables

| Column 1 | Column 2 |
|----------|----------|
| Text with (bad)[link] | Normal text |
| Normal | Another (violation)[here] |

## Blockquotes

> This quote has (reversed)[link] syntax.
> 
> And (another)[bad] example in quotes.

## Headers with Links

### Header with (bad)[link] syntax

#### Another header with (wrong)[pattern]