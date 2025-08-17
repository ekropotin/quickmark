# MD056 Violations

These tables should trigger MD056 violations.

## Too few cells in data row

| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |
| Cell 3   |

## Too many cells in data row

| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   | Cell 5 |

## Mixed violations in same table

| Header 1 | Header 2 | Header 3 |
| -------- | -------- | -------- |
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   | Cell 5   | Cell 6 |

## Too many cells in single column table

| Header |
| ------ |
| Cell 1 | Cell 2 |

## Too few cells in single column table

| Header |
| ------ |
|        |

## Multiple tables with violations

| Table 1 | Header |
| ------- | ------ |
| Cell    |

| Different | Table |
| --------- | ----- |
| More      | Data  | Extra |

## Table without pipes but inconsistent

Header 1 | Header 2
-------- | --------
Cell 1
Cell 3   | Cell 4   | Cell 5