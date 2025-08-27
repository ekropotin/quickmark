# MD007 Valid Test Cases

These examples should NOT trigger MD007 violations with default settings (indent=2, start_indent=2, start_indented=false).

## Basic proper indentation

* Item 1
  * Item 2
    * Item 3
  * Item 4
* Item 5

## Single level list

* Item 1
* Item 2
* Item 3

## Multiple separate lists

* List 1 item 1
* List 1 item 2

Some text

* List 2 item 1
* List 2 item 2

## Mixed with ordered lists (ordered lists should be ignored)

1. Ordered item 1
2. Ordered item 2

* Unordered item 1
* Unordered item 2

## Single item lists

* Single item

## Empty document

## Complex nested structure

* Top level 1
  * Second level 1
    * Third level 1
    * Third level 2
  * Second level 2
    * Third level 3
* Top level 2
  * Second level 3

## Unordered lists nested in ordered lists (should be ignored)

1. Ordered item
   * This unordered list should be ignored
     * Even deeper unordered items
   * Another unordered item
2. Another ordered item