# MD033 Comprehensive Test Cases

This file tests various edge cases and scenarios for the MD033 rule.

## Normal Markdown (Should Not Trigger)

### Headers
# H1 Heading
## H2 Heading
### H3 Heading

### Text Formatting
**Bold text** and *italic text* and ~~strikethrough~~.

### Lists
- Unordered list
- Another item
  - Nested item

1. Ordered list
2. Another item

### Links and Images
[Link text](https://example.com)
![Alt text](image.jpg)

## HTML in Code Contexts (Should Not Trigger)

### Fenced Code Blocks
```html
<div class="example">
    <p>HTML in fenced code block</p>
    <span>Should not trigger MD033</span>
    <br>
    <hr/>
</div>
```

```xml
<root>
    <element attribute="value">Content</element>
</root>
```

### Indented Code Blocks
    <div>
        <p>HTML in indented code block</p>
        <span>Should not trigger</span>
    </div>

### Inline Code Spans
Text with `<code>` in backticks should not trigger.

Multiple inline code: `<div>`, `<span>`, and `<p>` elements.

Complex inline code: `<img src="test.jpg" alt="test"/>` should not trigger.

## HTML Content (Should Trigger Violations)

### Basic Block Elements
<p>HTML paragraph</p>

<div>HTML div element</div>

<section>HTML section</section>

<article>HTML article</article>

### Inline Elements
Text with <span>HTML span</span> element.

Using <strong>HTML strong</strong> instead of Markdown.

Text with <em>HTML emphasis</em> element.

### Self-Closing Tags
<br>

<hr>

<br/>

<hr/>

<img src="test.jpg" alt="test">

<input type="text" value="example">

### HTML with Attributes
<p class="intro">Paragraph with class attribute</p>

<div id="main" class="container">Div with multiple attributes</div>

<a href="https://example.com" target="_blank" rel="noopener">Link with attributes</a>

### Mixed Content
Normal text <span>with HTML span</span> in the middle.

**Markdown bold** and <strong>HTML strong</strong> mixed together.

### Complex Nested HTML
<div class="wrapper">
    <header>
        <h1>HTML heading</h1>
        <nav>
            <ul>
                <li><a href="#section1">Link 1</a></li>
                <li><a href="#section2">Link 2</a></li>
            </ul>
        </nav>
    </header>
    <main>
        <section id="section1">
            <h2>Section Title</h2>
            <p>Section content</p>
        </section>
    </main>
</div>

### Form Elements
<form action="/submit" method="post">
    <label for="name">Name:</label>
    <input type="text" id="name" name="name" required>
    <textarea name="message" rows="4" cols="50"></textarea>
    <button type="submit">Submit</button>
</form>

### Table Elements
<table>
    <thead>
        <tr>
            <th>Header 1</th>
            <th>Header 2</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>Cell 1</td>
            <td>Cell 2</td>
        </tr>
    </tbody>
</table>

## Edge Cases

### HTML-like Text (Should Not Trigger)
This text has angle brackets < and > but is not HTML.

Mathematical expressions: a < b and x > y.

### Invalid HTML (Should Still Trigger Opening Tags)
<div>Unclosed div

<p>Paragraph without proper closing</span>

### HTML Comments (Should Not Trigger - Not Handled by This Rule)
<!-- This is an HTML comment -->

### Case Sensitivity
<P>Uppercase P tag</P>

<DIV>Uppercase DIV tag</DIV>

<Span>Mixed case span</Span>

## Markdown Tables (Should Not Trigger)
| Column 1 | Column 2 |
|----------|----------|
| Data 1   | Data 2   |
| Data 3   | Data 4   |

## Blockquotes (Should Not Trigger)
> This is a Markdown blockquote
> With multiple lines
>
> And multiple paragraphs