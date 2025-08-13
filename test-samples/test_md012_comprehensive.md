# MD012 Comprehensive Test

This document tests various edge cases for MD012 (no-multiple-blanks).

## Valid cases - should not trigger

Single blank line:

Valid content

No blank lines:
Also valid

## Invalid cases - should trigger violations

Two blank lines (violation):


Content after violation

Three blank lines (violation):




Content after violation

## Code blocks - blank lines inside should be ignored

```javascript
function test() {


    return true;
}
```

But blank lines around code blocks count:


```bash
echo "test"
```


Above and below have violations.

## Indented code blocks

    code line 1
    
    
    code line 2

Blank lines after indented code:


Should be violation.

## Complex mixing

Valid single blank:

Valid content

Invalid double blank:


Violation content

Valid single blank:

End content

## Edge case: spaces on blank lines

Lines with only spaces count as blank.

  

Above line has 2 spaces - should count as blank lines.

## Multiple violations in sequence

First violation:


Second violation immediately after:


Third content

## End of document violations

Final content.

