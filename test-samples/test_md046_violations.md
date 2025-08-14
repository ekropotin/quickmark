# Test MD046 Violations

This file contains cases that should trigger MD046 violations with default (consistent) style.

## Mixed Styles - Indented First

Some text before the first code block.

    This is an indented code block (first one sets the style).

More text between code blocks.

```text
This is a fenced code block (should violate).
```

Another paragraph.

    This indented block is OK (matches first style).

More text.

```python
def hello():
    print("Another violation")
```

End of document.

## Mixed Styles - Fenced First

Some text before the first code block.

```bash
echo "This is a fenced code block (first one sets the style)"
```

More text between code blocks.

    This is an indented code block (should violate).
    It spans multiple lines.

Another paragraph.

```python
# This fenced block is OK (matches first style)
def function():
    return True
```

More text.

    Another indented block violation.
    echo "This should also violate"

End of document.