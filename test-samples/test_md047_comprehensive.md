# Comprehensive MD047 Test File

This file tests various scenarios for MD047 (single-trailing-newline).

## Valid Cases

These scenarios should NOT trigger violations:

### Case 1: Simple content with newline
Simple content.

### Case 2: Content with HTML comments
Content with comment.
<!-- This is a comment -->

### Case 3: Content with blockquote markers
Content with blockquote.
>>>

### Case 4: Mixed comments and blockquotes
Content with mixed.
<!-- comment -->>

### Case 5: Empty lines with whitespace
Content before empty whitespace.
   

### Case 6: Code blocks
```bash
echo "hello"
```

Final line that ends properly.
