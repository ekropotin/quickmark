# MD029 Violations Examples

## Mixed numbering patterns (violates one_or_ordered default)

1. First item
1. Second item  
3. Third item violates - expected 1 or 2

## Inconsistent ordered pattern

1. First item
2. Second item
4. Third item violates - skipped 3

## Bad restart pattern

1. First item
2. Second item
1. Third item violates - should be 3

## Zero mixed with ones (in one_or_ordered mode)

1. First item
0. Second item violates - inconsistent with first

## Large number jumps

1. First item
2. Second item
10. Third item violates - too big a jump

## Pattern that doesn't follow any consistent style

1. First item
3. Second item violates - not following any pattern
1. Third item violates - inconsistent
4. Fourth item violates - no clear pattern

## Nested list violations

1. Outer item
2. Outer item
   1. Inner item
   3. Inner item violates - skipped 2
3. Outer item