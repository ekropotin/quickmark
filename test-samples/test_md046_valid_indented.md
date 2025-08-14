# Test MD046 Valid Cases (Indented)

This file contains valid cases that should not trigger MD046 violations.
All code blocks in this document use the same style (indented).

## Multiple Indented Code Blocks

Some text before the first code block.

    This is an indented code block.
    It can span multiple lines.

More text between code blocks.

    # Another indented code block
    def function():
        return True

Another paragraph.

    echo "Yet another indented code block"
    ls -la

## Code with Different Languages

    function example() {
        return "JavaScript";
    }

    {
        "key": "value",
        "number": 42
    }

## Simple Commands

    echo "Simple command"

    ls -la
    pwd

## Single Code Block

Just one code block should always be valid.

    This is the only code block.

End of document.