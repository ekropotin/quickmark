# MD030 Comprehensive Test

This file tests all aspects of the MD030 rule (list-marker-space).

## Default Configuration Test (ul_single=1, ol_single=1, ul_multi=1, ol_multi=1)

### Valid Cases

Single-line unordered list (1 space expected, 1 provided):
* Item 1
* Item 2

Single-line ordered list (1 space expected, 1 provided):
1. Item 1
2. Item 2

Multi-line unordered list (1 space expected, 1 provided):
* Item with content

  Second paragraph.

* Another item with content

  More content here.

Multi-line ordered list (1 space expected, 1 provided):
1. First item

   Additional content.

2. Second item

   More additional content.

### Violation Cases

Single-line unordered list with 2 spaces (expects 1):
*  Item 1
*  Item 2

Single-line ordered list with 2 spaces (expects 1):
1.  Item 1
2.  Item 2

Single-line unordered list with 3 spaces (expects 1):
*   Item 1
*   Item 2

Multi-line unordered list with 2 spaces (expects 1):
*  Multi-line item

   With second paragraph.

*  Another item

   With more content.

## Edge Cases

### Different Markers

Plus markers with correct spacing:
+ Item 1
+ Item 2

Plus markers with incorrect spacing:
+  Item 1
+  Item 2

Dash markers with correct spacing:
- Item 1
- Item 2

Dash markers with incorrect spacing:
-  Item 1
-  Item 2

### Nested Lists

Correctly spaced nested items:
* Parent 1
  * Child 1
  * Child 2
* Parent 2

Incorrectly spaced nested items:
* Parent 1
  *  Child 1 (wrong spacing)
  *  Child 2 (wrong spacing)

### Mixed Scenarios

Valid single-line followed by invalid:
* Valid item
*  Invalid item (2 spaces)

Mixed valid and invalid in ordered list:
1. Valid item
2.  Invalid item (2 spaces)

### List Separation

Two separate lists (should be treated independently):

* First list item
* Second list item

Some text separating the lists.

* Third list item (separate list)
* Fourth list item

### Number Variations in Ordered Lists

Different number lengths:
1. Item 1
2. Item 2
10. Item 10
11. Item 11

With violations:
1.  Item 1 (2 spaces)
2.  Item 2 (2 spaces)
10.  Item 10 (2 spaces)

### Complex Multi-line Content

List with code blocks:
* Item with code:

  ```
  some code here
  ```

* Another item

Lists with blockquotes:
* Item with quote:

  > This is a blockquote
  > inside a list item

* Another item

### Empty Content

Lists with minimal content:
* A
* B
* C

With violations:
*  A
*  B
*  C