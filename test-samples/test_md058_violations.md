# MD058 Test - Violations

## Missing blank line above
Text directly above table.
| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |

Text after table.

## Missing blank line below

Text before table.

| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |
Text directly below table.

## Missing both blank lines above and below
Text directly above.
| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |
Text directly below.

## Multiple tables with missing spacing

First text.

| Table 1 | Header |
| ------- | ------ |
| Cell    | Value  |
Text between tables.
| Table 2 | Header |
| ------- | ------ |
| Cell    | Value  |

Final text.

## Complex case with multiple violations

Some initial text.
| Table 1 | Missing | Above |
| ------- | ------- | ----- |
| Data    | Goes    | Here  |

Middle text content.

| Table 2 | Missing | Below |
| ------- | ------- | ----- |
| More    | Data    | Here  |
Following text without spacing.

## Table missing above spacing only

Previous paragraph content.
| Header | Value |
| ------ | ----- |
| Test   | Data  |

Proper spacing below.

## Table missing below spacing only

Proper spacing above.

| Header | Value |
| ------ | ----- |
| Test   | Data  |
Immediate text below.

## Single row table violations
Text above.
| Single Row |

Text below.