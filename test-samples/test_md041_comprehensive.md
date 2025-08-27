# Valid Test Cases

## Case 1: ATX heading at the beginning
# First Heading

This is valid content.

## Case 2: Setext heading at the beginning
First Heading
=============

This is also valid.

## Case 3: Comments before heading (should be ignored)
<!-- This is a comment -->

# First Heading

Content after heading.

## Case 4: Whitespace before heading (should be ignored)


# First Heading

Content.

## Case 5: Front matter with title (should allow content)
---
title: "Document Title"
layout: post
---

This content is allowed because front matter has title.

# Invalid Test Cases

## Case 6: Text before heading (violation)
Some text before heading.

# Heading

Content.

## Case 7: Wrong heading level (violation)
## Wrong Level Heading

Should be H1 but this is H2.

## Case 8: List before heading (violation)
- List item
- Another item

# Heading

Content.

## Case 9: Code block before heading (violation)
```
code block
```

# Heading

Content.

## Case 10: Front matter without title
---
layout: post
author: "John Doe"
---

This content requires a heading since no title in front matter.

# Heading

Content.

## Case 11: Empty document (valid)
(This case would be in a separate empty file)

## Case 12: Blockquote before heading (violation)
> This is a blockquote

# Heading

Content.

## Case 13: Table before heading (violation)
| Column 1 | Column 2 |
|----------|----------|
| Data     | Data     |

# Heading

Content.