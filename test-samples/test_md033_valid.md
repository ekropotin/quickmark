# MD033 Valid Test Cases

This file contains valid Markdown content that should not trigger MD033 violations.

## Regular Markdown Content

This is regular markdown with no HTML.

- List item 1
- List item 2

**Bold text** and *italic text*.

### Code Blocks

HTML in code blocks should be ignored:

```html
<div class="example">
    <p>This is HTML inside a code block</p>
    <span>It should not trigger MD033</span>
</div>
```

Indented code blocks should also be ignored:

    <p>This is in an indented code block</p>
    <h1>Should not trigger</h1>

### Code Spans

HTML in `<code>` spans should be ignored.

Text with `<p>inline HTML</p>` in code spans.

Multiple code spans: `<div>` and `<span>` should not trigger.

### Links and Images

[Regular link](https://example.com)

![Regular image](image.jpg)

## Mixed Content

Regular text with normal markdown features.

> This is a blockquote
> With multiple lines

1. Numbered list
2. Another item
   - Nested item
   - Another nested item

| Table | Header |
|-------|--------|
| Cell  | Data   |