# Comprehensive MD032 Test Cases

This file contains a comprehensive set of test cases for the MD032 rule (blanks-around-lists).

## Basic Valid Cases

Text with proper spacing.

* List item 1
* List item 2

More text with proper spacing.

## Basic Violation Cases

Text without spacing.
* Violation: missing blank before
* Another item

Text.

* List item
* Another item
Text without spacing - violation: missing blank after.

## Document Boundaries

* List at very start of document
* Second item

Text in middle.

* List at very end
* Final item

## Nested Lists (Should Not Trigger MD032)

Outer text.

* Outer item 1
  * Nested item 1
    * Deeply nested item
  * Nested item 2
* Outer item 2
  1. Nested ordered item
  2. Another nested ordered item

Final outer text.

## Mixed List Markers

Text before mixed lists.
+ Plus item 1
+ Plus item 2

- Dash item 1  
- Dash item 2
Text after mixed lists.

## Ordered Lists

### Valid ordered lists

Some text.

1. First ordered item
2. Second ordered item

More text.

### Invalid ordered lists

Text before.
1. Missing blank before
2. Second item
Following text - missing blank after.

## Blockquotes

### Valid blockquotes

> Some quoted text.
>
> * Properly spaced list in blockquote
> * Second item
>
> More quoted text.

### Invalid blockquotes

> Text before list.
> * Missing blank before in blockquote
> * Second item
> Text after list - missing blank after in blockquote.

## Complex Block Element Interactions

### With code blocks

Valid spacing:

* List item
* Another item

```javascript
console.log("code block");
```

Invalid spacing:
* List item
* Another item
```javascript
console.log("no blank line before code");
```

### With headings

Valid spacing:

* List before heading
* Second item

## Heading After List

Invalid spacing:
* List before heading  
* Second item
## Missing Blank Before Heading

### With thematic breaks

Valid spacing:

* List item
* Another item

---

Invalid spacing:
* List item
* Another item
---

## Lazy Continuation Lines

These should NOT trigger violations per CommonMark spec:

Text before list.

1. First item with continuation
   Properly indented continuation.
2. Second item with lazy continuation
Lazy continuation at column 0.
3. Third item

Text after list.

## Edge Cases

### Empty lists (if supported)

Text before.

*

Text after.

### Lists with only one item

Text before.

* Single item list

Text after.

### Multiple consecutive lists

First list:

* Item 1
* Item 2

Second list:

1. Item A
2. Item B

Final text.

## Lists in Complex Nesting

> Blockquote text.
>
> * Blockquote list item 1
>   * Nested in blockquote
> * Blockquote list item 2
>
> More blockquote text.

Final document text.