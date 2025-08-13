# MD033 Violations Test Cases

This file contains HTML content that should trigger MD033 violations.

## Basic HTML Tags

<p>This paragraph uses HTML instead of Markdown</p>

<h1>HTML heading instead of Markdown</h1>

<div>A div element</div>

## Self-Closing Tags

<br>

<hr>

<br/>

<hr/>

<img src="image.jpg" alt="HTML image">

<input type="text" value="example">

## Mixed HTML and Markdown

Regular text with <span>inline HTML span</span> element.

This paragraph has <strong>HTML strong</strong> instead of **Markdown bold**.

Using <em>HTML emphasis</em> instead of *Markdown italic*.

## Complex HTML

<div class="container">
    <section id="main">
        <article>
            <header>
                <h2>Article Title</h2>
            </header>
            <p>Article content with HTML structure.</p>
        </article>
    </section>
</div>

## HTML with Attributes

<p class="special" id="intro">Paragraph with CSS class and ID</p>

<a href="https://example.com" target="_blank">Link with attributes</a>

<img src="image.jpg" alt="Description" width="100" height="50">

## Closing Tags

Both opening and closing tags should be detected, but only opening tags should be reported:

<blockquote>This uses HTML blockquote</blockquote>

<code>HTML code element</code>

## Valid Markdown That Should Not Trigger

Normal **bold** and *italic* text.

Regular [link](https://example.com) and ![image](image.jpg).

Code blocks are ignored:

```html
<p>This HTML is in a code block</p>
```

Code spans are ignored: `<span>inline HTML</span>` in backticks.