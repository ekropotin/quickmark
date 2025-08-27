# MD005 Comprehensive Test Cases

## Valid Cases

### Basic unordered lists

* Item 1
* Item 2
* Item 3

### Basic ordered lists (left-aligned)

1. Item 1
2. Item 2
3. Item 3

### Basic ordered lists (right-aligned)

 1. Item 1
 2. Item 2
10. Item 10
11. Item 11

### Nested lists with consistent indentation

* Top level
  * Nested level 1
    * Nested level 2
  * Back to nested level 1
* Back to top level

### Complex ordered nesting

1. First item
   1. Nested ordered
   2. Nested ordered
2. Second item

### Mixed list types

1. Ordered item
2. Another ordered item

* Unordered item
* Another unordered item

## Violation Cases

### Inconsistent unordered indentation

* Item 1
 * Item 2 (wrong indentation)
* Item 3

### Inconsistent ordered indentation

1. Item 1
 2. Item 2 (wrong indentation)
3. Item 3

### Nested inconsistencies

* Top level
  * Properly nested
   * Improperly nested (wrong indentation)
  * Back to proper nesting

### Ordered list right-alignment violations

 1. Item 1
 2. Item 2
10. Item 10
 11. Item 11 (should align with 10, not 1-2)

### Multiple violations in same list

* Item 1
 * Wrong 1
  * Wrong 2
   * Wrong 3
* Item 2

## Edge Cases

### Single item lists (always valid)

* Single item

1. Single ordered item

### Empty content after markers

*
*

1.
2.

### Lists with different markers

* Asterisk
+ Plus (different marker but should align)
- Dash (different marker but should align)

### Very deeply nested

* Level 1
  * Level 2
    * Level 3
      * Level 4
        * Level 5
      * Back to Level 4
    * Back to Level 3
  * Back to Level 2
* Back to Level 1

### Long ordered list numbers

  1. Item 1
  2. Item 2
  9. Item 9
 10. Item 10
100. Item 100
101. Item 101