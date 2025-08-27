# MD007 Comprehensive Test Cases

This file tests various configuration options and edge cases for MD007.

## Default settings (indent=2, start_indent=2, start_indented=false)

### Valid cases
* Proper indentation
  * Level 2
    * Level 3

### Invalid cases  
* Item 1
 * Wrong indentation (1 space)

## With start_indented=true configuration

This section would be valid with start_indented=true, start_indent=2:

  * Top level should be indented by start_indent
    * Second level should be start_indent + indent
      * Third level should be start_indent + (2 * indent)

Without start_indented=true, the above would be violations.

## With custom indent=4 configuration

This section would be valid with indent=4:

* Top level
    * Second level (4 spaces)
        * Third level (8 spaces)

With default indent=2, the above would be violations.

## With start_indented=true and start_indent=3 configuration

This would be valid with start_indented=true, start_indent=3, indent=2:

   * Top level (3 spaces for start_indent)
     * Second level (start_indent + indent = 5 spaces)
       * Third level (start_indent + 2*indent = 7 spaces)

## Edge cases

### Single items
* Single item list

### Empty lists (no items)

### Lists with content between items

* Item 1
  * Subitem 1

  Some paragraph text

  * Subitem 2
* Item 2

### Mixed ordered and unordered (unordered in ordered should be ignored)

1. Ordered item 1
   * This unordered list should be ignored by MD007
     * Even if indentation is wrong
2. Ordered item 2

### Deeply nested

* Level 1
  * Level 2
    * Level 3
      * Level 4
        * Level 5
          * Level 6

### Lists with various bullet styles (should all be checked)

* Asterisk list
  * Nested asterisk

+ Plus list  
  + Nested plus

- Dash list
  - Nested dash

### Complex markdown within lists

* Item with **bold** text
  * Item with [link](http://example.com)
    * Item with `code`
      * Item with > quote

### Lists immediately after headings

## Heading 1
* List item 1
  * Nested item

### Heading 2  
* Another list
  * Another nested item