# MD030 Custom Configuration Test

This file tests MD030 with custom spacing requirements:
- ul_single = 2 (unordered single-line items need 2 spaces)
- ol_single = 1 (ordered single-line items need 1 space)
- ul_multi = 3 (unordered multi-line items need 3 spaces)
- ol_multi = 2 (ordered multi-line items need 2 spaces)

## Valid Cases (should have no violations)

### Single-line Lists

Unordered with 2 spaces (correct):
*  Item 1
*  Item 2
*  Item 3

Ordered with 1 space (correct):
1. Item 1
2. Item 2
3. Item 3

### Multi-line Lists

Unordered with 3 spaces (correct):
*   Multi-line item

    Second paragraph.

*   Another multi-line item

    More content.

Ordered with 2 spaces (correct):
1.  Multi-line item

    Second paragraph.

2.  Another multi-line item

    More content.

## Violation Cases (should have violations)

### Single-line Lists with Wrong Spacing

Unordered with 1 space (should be 2):
* Item 1
* Item 2

Unordered with 3 spaces (should be 2):
*   Item 1
*   Item 2

Ordered with 2 spaces (should be 1):
1.  Item 1
2.  Item 2

### Multi-line Lists with Wrong Spacing

Unordered with 1 space (should be 3):
* Multi-line item

  Second paragraph.

* Another item

Unordered with 2 spaces (should be 3):
*  Multi-line item

   Second paragraph.

*  Another item

Ordered with 1 space (should be 2):
1. Multi-line item

   Second paragraph.

2. Another item

Ordered with 3 spaces (should be 2):
1.   Multi-line item

     Second paragraph.

2.   Another item