# MD030 Violation Cases

## Single-line lists with wrong spacing

Unordered list with double spaces (should be 1):

*  Item 1
*  Item 2

Unordered list with triple spaces (should be 1):

*   Item 1
*   Item 2

Ordered list with double spaces (should be 1):

1.  Item 1
2.  Item 2

Ordered list with triple spaces (should be 1):

1.   Item 1
2.   Item 2

## Multi-line lists with wrong spacing

Multi-line list with insufficient spaces (default config expects 1):

*  Multi-line item

   With second paragraph.

*  Another multi-line item

   With more content.

## Mixed marker violations

Plus markers with wrong spacing:

+  Item 1
+  Item 2

Dash markers with wrong spacing:

-  Item 1
-  Item 2

## Nested list violations

Incorrectly spaced nested items:

* Parent item
  *  Child item (wrong spacing)
  *  Another child (wrong spacing)

## Ordered list violations with different number lengths

*  Item 1
*  Item 2
*  Item 10

Ordered lists:

1.  Item 1 (wrong spacing)
2.  Item 2 (wrong spacing)
10.  Item 10 (wrong spacing)

## Multiple violations in same list

*  First item (2 spaces)
*   Second item (3 spaces) 
*    Third item (4 spaces)