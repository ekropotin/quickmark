# MD032 Violations Test Cases

This file contains examples that SHOULD trigger MD032 violations.

## Missing blank line before list

Text immediately before list.
* List item 1
* List item 2

More text.

## Missing blank line after list

Some text.

* List item 1
* List item 2
Text immediately after list.

## Missing blank lines both before and after

Text before.
* List item 1  
* List item 2
Text after.

## Ordered list violations

Text before ordered list.
1. First item
2. Second item
Following text.

## Lists followed by other block elements

Text before.

* List item 1
* List item 2
---

## Lists preceded by other block elements

Text before.

---
* List item 1
* List item 2

More text.

## Code blocks and lists

Text before.

```
code block
```
* List after code

Text.

* List before code
```
another code block
```

More text.

## Different list marker types creating separate lists

Text before.
+ Plus list item
- Dash list item
* Star list item
Text after.

## Blockquote violations

> Quoted text before.
> * List item 1
> * List item 2
> Quoted text after.

## Lists with headings

Text before.

* List item 1
* List item 2
# Heading immediately after list

## Heading before list
* List item 1
* List item 2

Text after.