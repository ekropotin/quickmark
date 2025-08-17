# MD056 Comprehensive Test Cases

This file contains comprehensive test cases for MD056 (table-column-count) to verify all scenarios.

## Valid Cases

### Basic consistent tables

| Two | Columns |
| --- | ------- |
| 1   | 2       |
| 3   | 4       |

| Single |
| ------ |
| Cell   |

| Three | Column | Table |
| ----- | ------ | ----- |
| A     | B      | C     |
| D     | E      | F     |

### Tables with empty cells (valid)

| Header | Empty |
| ------ | ----- |
|        | Data  |
| Data   |       |

### Header-only table (valid)

| Just | Header |

### Header with delimiter only (valid)

| Header | With | Delimiter |
| ------ | ---- | --------- |

## Violation Cases

### Too few cells

| Expected | Two | Columns |
| -------- | --- | ------- |
| Missing  | Cell|
| Also     |     |

### Too many cells

| Two | Columns |
| --- | ------- |
| Too | Many    | Cells   | Here |
| And | Another | Extra   |

### Mixed violations

| Three | Column | Headers |
| ----- | ------ | ------- |
| Too   | Few    |
| Too   | Many   | Cells   | Extra | More |
| Just  | Right  | Amount  |

### Complex scenarios

#### Multiple tables with different column counts (valid separately)

| First | Table |
| ----- | ----- |
| Has   | Two   |

| Second | Has | Three |
| ------ | --- | ----- |
| And    | Is  | Fine  |

#### Multiple tables with violations

| This | Has | Violations |
| ---- | --- | ---------- |
| Too  | Few |
| Cells| In  | Second     | Row |

| Another | Table |
| ------- | ----- |
| Also    | Has   | Too | Many |

### Edge cases

#### Single cell table

| One |
| --- |
| 1   |

#### Single cell table with violation

| One |
| --- |
| 1   | 2 |

#### Very wide table

| A | B | C | D | E | F | G | H |
| - | - | - | - | - | - | - | - |
| 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
| a | b | c | d | e | f | g |
| x | y | z | 1 | 2 | 3 | 4 | 5 | 6 |