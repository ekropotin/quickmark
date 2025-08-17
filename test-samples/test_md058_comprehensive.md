# MD058 Comprehensive Test Cases

This file contains a comprehensive set of test cases for the MD058 rule (blanks-around-tables).

## 1. Valid Cases

### 1.1 Proper spacing around table

Some text before.

| Header 1 | Header 2 | Header 3 |
| -------- | -------- | -------- |
| Cell 1   | Cell 2   | Cell 3   |
| Row 2    | Data     | More     |

Some text after.

### 1.2 Table at document start

| Start | Document | Table |
| ----- | -------- | ----- |
| Data  | Goes     | Here  |

Content after start table.

### 1.3 Table at document end

Content before end table.

| End | Document | Table |
| --- | -------- | ----- |
| End | Data     | Here  |

### 1.4 Single table in document

| Alone | Table |
| ----- | ----- |
| Data  | Here  |

### 1.5 Multiple properly spaced tables

First section.

| First | Table |
| ----- | ----- |
| Data  | One   |

Middle section content here.

| Second | Table |
| ------ | ----- |
| Data   | Two   |

Final section.

## 2. Violation Cases

### 2.1 Missing blank line above
Text directly above.
| No | Above | Blank |
| -- | ----- | ----- |
| Missing | Space | Above |

Proper space below.

### 2.2 Missing blank line below

Proper space above.

| No | Below | Blank |
| -- | ----- | ----- |
| Missing | Space | Below |
Text directly below.

### 2.3 Missing both blank lines
Text above.
| No | Blanks | Around |
| -- | ------ | ------ |
| Missing | Both | Sides |
Text below.

### 2.4 Multiple violations

First text.
| First | Violation |
| ----- | --------- |
| Missing | Above |

Middle text.

| Second | Violation |
| ------ | --------- |
| Missing | Below |
Following text.

### 2.5 Complex table structures

#### Missing above spacing
Previous content.
| Complex | Table | With | Many | Columns |
| ------- | ----- | ---- | ---- | ------- |
| Row 1   | Data  | More | Info | Here    |
| Row 2   | Even  | More | Data | Points  |

Proper spacing below.

#### Missing below spacing

Proper spacing above.

| Another | Complex | Table |
| ------- | ------- | ----- |
| Data    | Values  | Items |
| More    | Rows    | Data  |
Immediate text following.

## 3. Edge Cases

### 3.1 Table with different column counts (should still check spacing)

Proper spacing above.

| Header |
| ------ |
| Single |

Proper spacing below.

### 3.2 Tables in lists

1. First item

   | Table | In | List |
   | ----- | -- | ---- |
   | Data  | In | Item |

2. Second item

### 3.3 Tables in blockquotes

> Quote with table:
>
> | Quote | Table |
> | ----- | ----- |
> | Data  | Here  |
>
> More quote text.

### 3.4 Adjacent violation cases
Text before first table.
| First | Table |
| ----- | ----- |
| Data  | One   |
| Second | Table |
| ------ | ----- |
| Data   | Two   |
Text after second table.

## 4. Mixed valid and invalid

### Valid table 1

Content before.

| Valid | Table | One |
| ----- | ----- | --- |
| Good  | Space | All |

Content after.

### Invalid table 1
Direct text above.
| Invalid | Table | One |
| ------- | ----- | --- |
| Bad | Spacing | Above |

Good spacing below.

### Valid table 2

Good spacing above.

| Valid | Table | Two |
| ----- | ----- | --- |
| Good  | Space | All |

Good spacing below.

### Invalid table 2

Good spacing above.

| Invalid | Table | Two |
| ------- | ----- | --- |
| Bad | Spacing | Below |
Direct text below.