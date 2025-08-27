# MD004 Comprehensive Test

This file tests various configurations and edge cases for MD004 ul-style rule.

## Consistent Style (default)

### Valid consistent asterisk

* Item 1
* Item 2 
* Item 3

### Valid consistent dash

- Item 1
- Item 2
- Item 3

### Invalid mixed in single logical list

* Item 1
+ Item 2  <!-- violation -->
- Item 3  <!-- violation -->

## Specific Style Enforcement

The following should be used with appropriate config settings:

### Asterisk only (with config style = "asterisk")

* Valid asterisk item
+ Invalid plus item   <!-- should violate when style=asterisk -->
- Invalid dash item   <!-- should violate when style=asterisk -->

### Dash only (with config style = "dash")  

- Valid dash item
* Invalid asterisk item  <!-- should violate when style=dash -->
+ Invalid plus item     <!-- should violate when style=dash -->

### Plus only (with config style = "plus")

+ Valid plus item  
* Invalid asterisk item  <!-- should violate when style=plus -->
- Invalid dash item     <!-- should violate when style=plus -->

## Sublist Style Testing

With sublist style, each nesting level should differ from parent:

### Valid sublist pattern

* Level 1 (asterisk)
  + Level 2 (plus, different from asterisk)
    - Level 3 (dash, different from plus)
  + Level 2 continues (plus, consistent with level 2)
* Level 1 continues (asterisk)
  + Level 2 again (plus)

### Invalid sublist - same marker as parent

* Level 1 (asterisk)
  * Level 2 (asterisk - should violate in sublist mode)
* Level 1 continues

## Complex nesting scenarios

* Top level
  + Second level
    - Third level
      * Fourth level  
    - Third level continues
  + Second level continues  
    * Third level different marker (may violate depending on sublist logic)

## Edge cases

### Single items at different levels

* Single top level

  + Single second level

### Empty list items

* 
+ Empty item above (violation if mixed markers)

### Lists with inline code and formatting

* Item with `inline code`
+ Item with **bold** (violation)
- Item with *italic* (violation)

## Lists separated by other content

First list:

* Item A1  
* Item A2

Some text, heading, or other content.

## Another heading

Second list (should not violate even with different marker):

- Item B1
- Item B2

More content.

### Third list with different marker again

+ Item C1
+ Item C2