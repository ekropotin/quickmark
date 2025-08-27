# MD004 Violation Examples

## Mixed markers in same list (violations)

* Item 1
+ Item 2
- Item 3

## Inconsistent nested list markers

* Level 1 item 1
  + Level 2 item 1  
    - Level 3 item 1
  + Level 2 item 2
* Level 1 item 2
  - Level 2 item 3  <!-- This should be inconsistent with level 2 above -->

## Mixed markers with complex nesting

* Top level asterisk
  * Nested asterisk (consistent)
+ Top level plus (inconsistent with asterisk above)
  - Nested dash under plus (inconsistent style)

## Multiple violations in sequence

- First item dash
* Second item asterisk (violation)
+ Third item plus (violation)
- Fourth item dash (consistent with first)
* Fifth item asterisk (violation)