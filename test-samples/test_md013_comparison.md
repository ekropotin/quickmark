# Comprehensive MD013 Test

This is a normal line within 80 characters.

This line is exactly eighty characters long and should be just at the limit!!

This line definitely exceeds eighty characters and should trigger a violation in both linters.

[Link reference that is very long]: https://example.com/very-long-url-that-should-be-exempted-from-line-length-rules

[This is a standalone link with very long text that should be exempted](https://example.com)

![This is a standalone image with very long alt text that should be exempted](https://example.com/image.jpg)

Text with URL without spaces beyond limit: https://example.com/very-long-url-without-any-spaces-should-be-exempted

Text with URL that has spaces beyond the limit: https://example.com/url and then more text.

## This heading is longer than eighty characters and should trigger a violation

```
This code block line is longer than eighty characters and should trigger violation.
```

| Column 1 | Column 2 | This table cell is longer than eighty characters and should trigger a violation |
|----------|----------|----------------------------------------------------------------------------------|
| Data     | Data     | Data                                                                             |

Another regular line that exceeds the eighty character limit and should be flagged.