# MD011 Valid Examples

This file contains examples that should NOT trigger the MD011 rule.

## Correct Link Syntax

Here is a [correct link](https://example.com) that should not be flagged.

Multiple [correct](link1) and [proper](link2) links.

## Links with Complex URLs

Visit [GitHub](https://github.com/user/repo#section) for more information.

Check out [this page](https://example.com/path?param=value&other=123) for details.

## Escaped Content

This is an escaped \(not)[a-link] example that should not be flagged.

Here is another \(escaped)[pattern] that should be ignored.

## Footnote References

For (example)[^1] this should not be flagged as a footnote reference.

Here is another (footnote)[^2] reference.

And a third (one)[^footnote-name] with a longer name.

## Code Blocks and Inline Code

```markdown
This (reversed)[link] should be ignored in code block.
Another (bad)[example] in the same block.
```

Here is some `(inline)[code]` that should be ignored.

More text with `(another)[inline-code]` example.

    This (reversed)[link] should be ignored in indented code block.
    Another (bad)[example] in the same indented block.

## Complex Cases

This (pattern)[link](more) should not match because it's followed by parentheses.

This (text (with parens))[link] should not match because of nested parentheses.

## Link Destinations Starting with ^ or ]

This (text)[^footnote] should not match.

This (text)[]bracket] should not match.

## Links with Backslash Endings

This (text\\)[link] should not be flagged.

This (text)[link\\] should not be flagged.

## Normal Text

Just some normal text without any links.

Parentheses (like this) and brackets [like this] should not be flagged when separate.

## Reference Links

This is a [reference link][ref] that should not be flagged.

And this is another [reference][ref2] style link.

[ref]: https://example.com
[ref2]: https://example.com/other