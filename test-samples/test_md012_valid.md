# Valid MD012 Cases

This document contains valid cases that should not trigger MD012 violations.

## Single blank lines are allowed

Line one

Line two

## No blank lines also valid

Line one
Line two
Line three

## Blank lines in code blocks are ignored

```javascript
function example() {


    return true;
}
```

Also indented code blocks:

    code line 1
    
    
    code line 2

## Mixed content valid

Normal paragraph

```bash
echo "hello"


echo "world"
```

Another paragraph

## End of document with single blank line

Final line