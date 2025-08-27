# MD010 Comprehensive Test Cases

This file tests various scenarios for hard tab detection.

## Regular Text Cases

Valid line with spaces only.
Invalid line	with single hard tab.
Another invalid	line	with	multiple	tabs.

## Code Block Cases

### Fenced Code Block - No Language

```
function noLanguage() {
	return "tab indented";
}
```

### Fenced Code Block - JavaScript

```javascript
function jsExample() {
	console.log("tabs in JavaScript");
	var x = {
		key:	"value with tab"
	};
}
```

### Fenced Code Block - Python

```python
def python_example():
	"""Function with tab indentation"""
	if True:
		return "tabs in Python"
```

### Fenced Code Block - Bash

```bash
#!/bin/bash
if [ -f "file.txt" ]; then
	echo "File exists"
	cat	file.txt
fi
```

### Tilde Fenced Code Block

~~~rust
fn rust_example() {
	println!("Rust with tabs");
	let x =	42;
}
~~~

## Indented Code Blocks

This is a regular paragraph.

    This is an indented code block with spaces
    def spaces_example():
        return "uses spaces"

Another paragraph.

	This is an indented code block with tabs
	def tabs_example():
		return "uses tabs"

## Lists with Mixed Indentation

- First item (spaces)
  - Nested item (spaces)
-	Second item (tab)
	-	Nested item (tab)

## Blockquotes

> This is a blockquote with spaces
> > Nested blockquote with spaces

>	This is a blockquote with tab
>	>	Nested blockquote with tabs

## Tables

### Table with Spaces

| Header 1 | Header 2 | Header 3 |
|----------|----------|----------|
| Cell 1   | Cell 2   | Cell 3   |

### Table with Tabs

| Header 1	| Header 2	| Header 3	|
|----------|----------|----------|
| Cell 1	| Cell 2	| Cell 3	|

## Edge Cases

Empty line with spaces:    
Empty line with tab:	

Line ending with tab	
Line with tab at start:	followed by text

## HTML Blocks

<div>
    <p>HTML with spaces</p>
</div>

<div>
	<p>HTML with tabs</p>
</div>

The end.