# MD007 Violation Test Cases

These examples should trigger MD007 violations with default settings (indent=2, start_indent=2, start_indented=false).

## Improper indentation - too little

* Item 1
 * Item 2 (1 space, should be 2)
* Item 3

## Improper indentation - too much

* Item 1
   * Item 2 (3 spaces, should be 2)
* Item 3

## Mixed improper indentation

* Item 1
 * Item 2 (1 space)
   * Item 3 (3 spaces, should be 4 for level 2)
    * Item 4 (4 spaces)
  * Item 5 (2 spaces, correct for level 1)
* Item 6

## Inconsistent indentation at same level

* Item 1
  * Item 2 (correct)
   * Item 3 (wrong, should be 2 spaces)
  * Item 4 (correct)

## Very wrong indentation

* Item 1
       * Item 2 (7 spaces, should be 2)
* Item 3

## Tab vs spaces issues (treating tabs as single characters)

* Item 1
	* Item 2 (1 tab, should be 2 spaces)
* Item 3