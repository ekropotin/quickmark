# Test MD055 Violations

This file contains tables with inconsistent pipe styles that should trigger MD055 violations.

## Missing Leading Pipes

Header 1 | Header 2 |
-------- | -------- |
Cell 1   | Cell 2   |

## Missing Trailing Pipes

| Header 1 | Header 2
| -------- | --------
| Cell 1   | Cell 2

## Mixed Pipe Styles

| Header 1 | Header 2 |
| -------- | -------- |
Cell 1   | Cell 2   |

## Inconsistent Across Tables

| Table 1 | Header |
| ------- | ------ |
| Cell    | Value  |

Header | Column |
------ | ------ |
Data   | Info   |

## No Leading or Trailing Pipes

Header 1 | Header 2
-------- | --------
Cell 1   | Cell 2

## Only Leading Pipes

| Header 1 | Header 2
| -------- | --------
| Cell 1   | Cell 2

## Only Trailing Pipes

Header 1 | Header 2 |
-------- | -------- |
Cell 1   | Cell 2   |