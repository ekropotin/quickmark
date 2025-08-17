# Test MD055 Comprehensive

This file contains comprehensive test cases for MD055 table pipe style rule, covering all configuration styles and edge cases.

## Leading and Trailing Pipes Style

### Valid Cases

| Simple | Table |
| ------ | ----- |
| Cell 1 | Cell 2 |

| Complex | Table | With | Multiple | Columns |
| ------- | ----- | ---- | -------- | ------- |
| Row 1   | Data  | More | Values   | Here    |
| Row 2   | Info  | Some | Content  | Extra   |

### Invalid Cases for Leading and Trailing

Missing leading pipe:
Simple | Table |
------ | ----- |
Cell 1 | Cell 2 |

Missing trailing pipe:
| Simple | Table
| ------ | -----
| Cell 1 | Cell 2

## Leading Only Style

### Valid Cases

| Simple | Table
| ------ | -----
| Cell 1 | Cell 2

| Multiple | Columns | Here
| -------- | ------- | ----
| Value A  | Value B | Value C

### Invalid Cases for Leading Only

Unexpected trailing pipe:
| Simple | Table |
| ------ | ----- |
| Cell 1 | Cell 2 |

## Trailing Only Style

### Valid Cases

Simple | Table |
------ | ----- |
Cell 1 | Cell 2 |

Multiple | Columns | Here |
-------- | ------- | ---- |
Value A  | Value B | Value C |

### Invalid Cases for Trailing Only

Unexpected leading pipe:
| Simple | Table |
| ------ | ----- |
| Cell 1 | Cell 2 |

## No Leading or Trailing Style

### Valid Cases

Simple | Table
------ | -----
Cell 1 | Cell 2

Multiple | Columns | Here
-------- | ------- | ----
Value A  | Value B | Value C

### Invalid Cases for No Pipes

Unexpected leading pipe:
| Simple | Table
| ------ | -----
| Cell 1 | Cell 2

Unexpected trailing pipe:
Simple | Table |
------ | ----- |
Cell 1 | Cell 2 |

Both unexpected:
| Simple | Table |
| ------ | ----- |
| Cell 1 | Cell 2 |

## Consistent Style Testing

### First Table Sets Style (Leading and Trailing)

| Header A | Header B |
| -------- | -------- |
| Data 1   | Data 2   |

### Second Table Should Match

| Header C | Header D |
| -------- | -------- |
| Data 3   | Data 4   |

### Violation: Does Not Match First Table

Header E | Header F |
-------- | -------- |
Data 5   | Data 6   |

## Edge Cases

### Single Column Table

| Single |
| ------ |
| Value  |

### Two Column Table

| Left | Right |
| ---- | ----- |
| A    | B     |

### Empty Cells

| Header | Empty |
| ------ | ----- |
| Value  |       |
|        | Value |

### Special Characters in Content

| Symbol | Description |
| ------ | ----------- |
| &amp;  | Ampersand   |
| \|     | Pipe        |
| *bold* | Emphasis    |