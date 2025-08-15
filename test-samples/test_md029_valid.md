# MD029 Valid Examples

## One style (all items use "1.")

1. First item
1. Second item
1. Third item

## Ordered style (incrementing numbers)

1. First item
2. Second item
3. Third item

## Zero-based ordered style

0. First item
1. Second item
2. Third item

## Zero style (all items use "0.")

0. First item
0. Second item
0. Third item

## Single item lists (always valid)

1. Only item

0. Only item

## Zero-padded numbers are supported

08. Item eight
09. Item nine
10. Item ten

## Separate lists are independent

1. First list item
2. Second list item

Some text

1. New list starts fresh
1. Can use different style
1. Each list is independent

## Mixed content doesn't affect list detection

1. Text item
2. Item with **bold**
3. Item with [link](example.com)

## Nested lists are handled independently

1. Outer item
   1. Inner item
   2. Inner item
2. Outer item
   1. Inner item
   2. Inner item