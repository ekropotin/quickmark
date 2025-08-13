# Hard Tabs Violations

This file contains hard tabs and should trigger MD010 violations.

## Text with Hard Tabs

This line has a hard tab	after the word "tab".
Another line	with	multiple	hard	tabs.

## List with Hard Tab

-	This list item starts with a hard tab instead of spaces

## Code Blocks

### Fenced Code Block with Tabs

```javascript
function badExample() {
	console.log("This uses tabs for indentation");
	if (true) {
		return "nested with tabs";
	}
}
```

### Indented Code Block with Tabs

	def another_bad_example():
		"""This indented code block uses tabs"""
		return "tabs everywhere"

## Mixed Indentation

Some lines use spaces, others use	tabs - this is inconsistent.
	This line starts with a tab.
    This line starts with spaces.

## Table with Tab Separators

| Column 1	| Column 2	| Column 3	|
|----------|----------|----------|
| Value	1	| Value	2	| Value	3	|