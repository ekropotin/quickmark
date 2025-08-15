# MD029 Comprehensive Test

This file tests various aspects of the MD029 rule (ordered list item prefix).

## Default style (one_or_ordered) - Valid cases

### Pattern detected as "one" style

1. Item one
1. Item two  
1. Item three

### Pattern detected as "ordered" style

1. Item one
2. Item two
3. Item three

### Zero-based ordered (detected as ordered)

0. Item zero
1. Item one
2. Item two

## Default style violations

### Mixed pattern (should use consistent style)

1. Item one
1. Item two
3. Item three violates

### Invalid restart

1. Item one
2. Item two
1. Item three violates

## Edge cases

### Large numbers

100. Item hundred
101. Item hundred-one
102. Item hundred-two

### Zero-padded (should work fine)

08. Item eight
09. Item nine  
10. Item ten
11. Item eleven

### Single item (no violations)

42. Single item

### Empty lists don't cause issues

Some text with no lists.

### Very short list

1. Only
2. Two items

## Separate lists

First list:
1. Item
2. Item

Text separating lists.

Second list (independent):
0. Different style is OK
0. Because it's a new list

## Nested scenarios

1. Outer
   1. Inner
   2. Inner continues
   3. Inner continues
2. Outer continues
   1. New inner list
   1. Can use different style
3. Outer final

## Mixed with unordered

1. Ordered item
2. Another ordered

- Unordered item
- Another unordered

3. Back to ordered (this continues the first list)

## Complex document structure

### Section with ordered list

1. First
2. Second  
3. Third

#### Subsection

Some text.

4. Continues previous list
5. Still going

### New section, new list

1. Fresh start
2. New numbering