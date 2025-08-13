# Valid Markdown - No Hard Tabs

This markdown file contains no hard tabs and should pass MD010 validation.

## Code Blocks with Spaces

```javascript
function example() {
    console.log("All indentation uses spaces");
    if (true) {
        return "nested with spaces";
    }
}
```

## Lists with Proper Indentation

- First level item
  - Second level item (indented with spaces)
    - Third level item (also spaces)

## Indented Code Block with Spaces

    def python_example():
        """This code block uses spaces for indentation"""
        return "spaces only"

## Table with Spaces

| Column 1    | Column 2    | Column 3    |
|-------------|-------------|-------------|
| Value 1     | Value 2     | Value 3     |

## Regular Text

All regular text content uses proper spacing and no hard tabs.
Even when we need to align things, we use spaces instead of tabs.

The end.