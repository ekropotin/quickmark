# MD056 Valid Cases

These tables should not trigger MD056 violations.

## Basic table with consistent column count

| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   |

## Single column table

| Header |
| ------ |
| Cell 1 |
| Cell 2 |

## Three column table

| Header 1 | Header 2 | Header 3 |
| -------- | -------- | -------- |
| Cell 1   | Cell 2   | Cell 3   |
| Cell 4   | Cell 5   | Cell 6   |

## Table with empty cells (but consistent count)

| Header 1 | Header 2 |
| -------- | -------- |
|          | Cell 2   |
| Cell 3   |          |

## Table with header only

| Header 1 | Header 2 |

## Table with header and delimiter only

| Header 1 | Header 2 |
| -------- | -------- |

## Multiple independent tables

| Table 1 | Header |
| ------- | ------ |
| Cell    | Value  |

| Different | Table | Headers |
| --------- | ----- | ------- |
| More      | Data  | Here    |

## Table without leading/trailing pipes (but consistent)

Header 1 | Header 2
-------- | --------
Cell 1   | Cell 2
Cell 3   | Cell 4