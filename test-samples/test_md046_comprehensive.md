# Test MD046 Comprehensive

This file contains comprehensive test cases for MD046 code block style rule.

## Valid: All Fenced

```text
First fenced code block.
```

Some text.

```python
def example():
    return "fenced"
```

More text.

```bash
echo "All fenced blocks"
```

## Valid: All Indented

    First indented code block.

Some text.

    def example():
        return "indented"

More text.

    echo "All indented blocks"

## Valid: Single Block Only

```single
The only code block in this section.
```

## Violation: Mixed Styles (Indented First)

    First code block is indented.

Some text.

```text
This fenced block should violate.
```

More text.

    This indented block should be OK.

Another paragraph.

```python
# This fenced block should also violate
def another_function():
    pass
```

## Violation: Mixed Styles (Fenced First)

```bash
echo "First code block is fenced"
```

Some text.

    This indented block should violate.

More text.

```python
# This fenced block should be OK
print("consistent with first")
```

Another paragraph.

    This indented block should also violate.
    echo "inconsistent"

## Edge Cases

### Empty Code Blocks

```
```

Some text.

```text
```

### Code Blocks in Lists

1. First item with code:

   ```text
   Fenced code in list
   ```

2. Second item with code:

       Indented code in list (should violate if fenced was first)

### Nested Code Blocks

> Quote with code:
> 
> ```text
> Fenced code in quote
> ```

And regular code:

    Indented code outside quote (should violate if fenced was first)

## Language-Specific Blocks

```javascript
function example() {
    return "JavaScript";
}
```

```python
def example():
    return "Python"
```

```bash
echo "Bash script"
ls -la
```

## Complex Mixed Example

First we have an indented block:

    echo "This sets the expected style to indented"
    ls -la

Then some fenced blocks that should violate:

```bash
echo "This should violate"
```

```python
print("This should also violate")
```

Another indented block (should be OK):

    echo "This matches the expected style"
    pwd

Final fenced block (should violate):

```text
Final violation
```