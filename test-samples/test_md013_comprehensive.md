# MD013 Line Length Comprehensive Test

## Regular Text Lines - Should Violate (Default/Stern/Strict)

This line is exactly eighty characters long and should not trigger violations.
This line is clearly longer than eighty characters and should trigger a violation in all modes (default, stern, strict) because it contains spaces beyond the limit.

## Lines Without Spaces Beyond Limit - Should Only Violate in Strict Mode

This line is exactly eighty characters and then continues with no spaces: https://example.com/very-long-url-without-any-spaces-at-all.
Another line that exceeds the limit but has no spaces beyond eighty characters: file_name_with_underscores_that_cannot_be_broken.txt

## Link Reference Definitions - Should Never Violate (All Modes)

[very-long-link-reference-definition-that-exceeds-eighty-characters]: https://example.com/very-long-url-that-should-be-exempted-from-line-length-checking
[another-long-reference]: https://github.com/user/repository/blob/main/some/very/long/path/to/file.md

## Standalone Links and Images - Should Never Violate (All Modes)

[This is a very long link text that definitely exceeds eighty characters but should be exempted](https://example.com/url)
![This is a very long image alt text that definitely exceeds eighty characters but should be exempted](https://example.com/image.jpg)

## Headings - Behavior Depends on 'headings' Setting

# This is a very long heading that definitely exceeds the eighty character limit and should trigger violations when headings are enabled
## Another very long heading that exceeds the limit - level 2 heading that should also trigger violations when enabled
### Level 3 heading that is also longer than eighty characters and should be caught when headings checking is enabled

#### Short heading

## Code Blocks - Behavior Depends on 'code_blocks' Setting

```python
# This code line is longer than eighty characters and should trigger violations when code_blocks is enabled
def very_long_function_name_that_exceeds_the_line_length_limit_and_should_be_caught():
    pass

short_line = "ok"
```

```javascript
// Another long code line that exceeds eighty characters and should be caught when code_blocks setting is enabled
const very_long_variable_name_that_exceeds_line_limit = "this should trigger violations when code_blocks is true";
```

    # Indented code block with long line that exceeds eighty characters and should be caught when code_blocks is enabled
    def another_very_long_function_name_that_definitely_exceeds_the_standard_eighty_character_line_length_limit():
        return "This string is also quite long and should trigger a violation"

## Tables - Behavior Depends on 'tables' Setting

| Column 1 | Column 2 with very long header text that exceeds eighty characters | Column 3 |
|----------|---------------------------------------------------------------------|----------|
| Short    | This table cell contains very long text that exceeds the eighty character limit | More     |
| Data     | Another very long table cell that should trigger violations when tables checking is enabled | Text     |

## Mixed Scenarios for Mode Testing

These lines are designed to test the differences between default, stern, and strict modes:

Line exactly at limit (80 chars): 12345678901234567890123456789012345678901234567890123456789012345678901234567890
Line with spaces beyond limit that should violate in all modes because it has breakable content here.
Line without spaces beyond limit: https://example.com/very-long-url-that-cannot-be-easily-broken-without-changing-semantics.html
Another-line-with-no-spaces-beyond-limit-that-uses-hyphens-instead-of-spaces-to-connect-words-and-should-only-violate-in-strict-mode.

## Edge Cases

Short line.

A line that is exactly eighty characters long and ends precisely at the limit.

[Link with long text that exceeds limit](http://example.com) but has URL
![Image with long alt text that exceeds limit](http://example.com/img.jpg) URL

<!-- This HTML comment is longer than eighty characters and should trigger violations depending on how comments are handled -->

## Setext Headings

This is a setext heading level 1 that exceeds the eighty character limit
======================================================================

This is a setext heading level 2 that also exceeds the character limit
-----------------------------------------------------------------------

## Mixed Content

Normal paragraph text that exceeds the eighty character limit and should trigger violations in all modes.

`This inline code snippet is longer than eighty characters and behavior depends on implementation details.`

**This bold text is longer than eighty characters and should trigger violations in all modes since it contains spaces.**

*This italic text is also longer than eighty characters and should trigger violations in all modes as well.*

## Test Summary

This document contains:
- Lines that should violate in all modes (default/stern/strict)
- Lines that should only violate in strict mode (no spaces beyond limit)
- Lines that should never violate (link refs, standalone links/images)
- Content that depends on boolean settings (headings, code_blocks, tables)
- Edge cases and boundary conditions
- Different heading types (ATX and Setext)
- Various markdown elements (bold, italic, inline code, comments)