# MD005 Violations

## Inconsistent unordered list indentation

* Item 1
 * Item 2 (should be indented same as item 1)
* Item 3

## Inconsistent nested unordered list indentation

* Top level item 1
  * Nested item 1
   * Nested item 2 (should be 2 spaces like item 1)
* Top level item 2

## Inconsistent ordered list indentation (left-aligned)

1. Item 1
 2. Item 2 (should start at same column as item 1)
3. Item 3

## Inconsistent ordered list with mixed alignment

1. Item 1
 2. Item 2
 3. Item 3
10. Item 10 (inconsistent with established right-alignment)

## Multiple inconsistencies in same list

* Item 1
 * Item 2 (1 space)
  * Item 3 (2 spaces)
   * Item 4 (3 spaces)

## Ordered list with inconsistent right-alignment

  1. Item 1
  2. Item 2
  3. Item 3
 10. Item 10 (should align with period at same position)
 11. Item 11

## More complex ordered list violations

 1. Item 1
 2. Item 2
10. Item 10
 11. Item 11 (should align period with item 10, not 1-2)