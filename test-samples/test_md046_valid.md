# Test MD046 Valid Cases

This file contains valid cases that should not trigger MD046 violations.
All code blocks in this document use the same style (fenced).

## Multiple Fenced Code Blocks

Some text before the first code block.

```text
This is a fenced code block.
```

More text between code blocks.

```python
def hello():
    print("Hello, world!")
```

Another paragraph.

```bash
echo "Another fenced code block"
```

## Language-Specific Fenced Blocks

```javascript
function example() {
    return "JavaScript";
}
```

```json
{
    "key": "value",
    "number": 42
}
```

## Empty Fenced Blocks

```
```

```text
```

## Single Code Block

Just one code block should always be valid.

```single
This is the only code block.
```

End of document.