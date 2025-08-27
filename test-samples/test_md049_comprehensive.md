# MD049 Comprehensive Test Cases

## Test all emphasis style configurations

### Default consistent mode (asterisk first)

This *sets* the style to asterisk for the rest of the document.

These _violations_ should be caught in consistent mode.

### Nested emphasis cases

This *has _nested_ emphasis* which should report violations.

This _has *nested* emphasis_ which should also report violations.

### Complex nesting scenarios

Text with *emphasis containing _mixed_ styles and more* text.

Text with _emphasis containing *mixed* styles and more_ text.

### Mixed with other formatting

This **strong** text with *emphasis* and _violations_.

This __strong__ text with _emphasis_ and *violations*.

### Code spans mixed with emphasis

This `*code*` should not interfere with _emphasis_ detection.

This `_code_` should not interfere with *emphasis* detection.

### Multiple paragraphs with mixed styles

First paragraph with *asterisk* emphasis.

Second paragraph with _underscore_ emphasis (violation).

Third paragraph with *asterisk* again.

Fourth paragraph with _underscore_ again (violation).

### Intraword emphasis edge cases

Regular apple*banana*cherry intraword (valid) with *regular* emphasis.

Mixed intraword test*word*test with _underscore_ emphasis (violation).

Start*word and word*end cases with _underscore_ violations.

### Complex document structure

This is a paragraph with *emphasis*.

> This is a blockquote with _emphasis_ (violation).

- List item with *emphasis*
- List item with _emphasis_ (violation)

1. Numbered list with *emphasis*
2. Numbered list with _emphasis_ (violation)

### Emphasis in various contexts

*Emphasis* at the start of a line.

Text ending with *emphasis*.

Middle *emphasis* in text.

_Violation_ at the start of a line.

Text ending with _violation_.

Middle _violation_ in text.