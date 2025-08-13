# MD011 Comprehensive Test

This file contains a mix of valid and invalid examples for the MD011 rule.

## Valid Examples (Should NOT be flagged)

Here is a [correct link](https://example.com) that should not be flagged.

Multiple [correct](link1) and [proper](link2) links.

### Escaped Content

This is an escaped \(not)[a-link] example that should not be flagged.

### Footnote References

For (example)[^1] this should not be flagged as a footnote reference.

### Code Blocks

```markdown
This (reversed)[link] should be ignored in code block.
```

Inline code: `(reversed)[link]` should also be ignored.

    Indented code: (reversed)[link] should be ignored.

### Complex Cases

This (pattern)[link](more) should not match because it's followed by parentheses.

This (text (with parens))[link] should not match because of nested parentheses.

This (text)[^footnote] should not match.

This (text\\)[link] should not be flagged.

This (text)[link\\] should not be flagged.

## Invalid Examples (SHOULD be flagged)

This is a (reversed)[link] example that should be flagged.

### Multiple Violations

Here are multiple (bad)[example] and (another)[violation] cases.

(reversed)[link] at the start of a line.

### In Different Contexts

Visit (GitHub)[https://github.com/user/repo#section] for more info.

*Emphasis* with (bad)[link] inside.

**Bold** text and (another)[violation] example.

### Lists

- Item with (reversed)[link] syntax
- Another item with (bad)[example]

1. Numbered item with (incorrect)[syntax]

### Tables

| Column 1 | Column 2 |
|----------|----------|
| Text with (bad)[link] | Normal text |

### Blockquotes

> This quote has (reversed)[link] syntax.

### Headers

#### Header with (bad)[link] syntax

## Mixed Valid and Invalid

This paragraph has both [correct](link) and (incorrect)[example] patterns.

```
Code block with (ignored)[pattern]
```

But this (real)[violation] should be caught.

## Reference Links (Valid)

This is a [reference link][ref] that should not be flagged.

[ref]: https://example.com

## More Edge Cases

### Valid Cases

For (reference)[^footnote] - footnote style.

Text with (nested (parens))[link] - nested parentheses.

Pattern (followed)[by](other) - followed by parentheses.

### Invalid Cases

Simple (case)[here] should be flagged.

Another (wrong)[pattern] to catch.

End of file with (final)[violation].