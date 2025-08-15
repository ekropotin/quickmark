# MD036 Test - Valid Examples

This file contains examples that should NOT trigger MD036 violations.

## Proper Heading

This is content under a proper heading.

### Another Proper Heading

More content under a proper heading.

This is a paragraph with **normal emphasis** in the middle of text.

This is a paragraph with *italic text* in the middle.

**This text ends with punctuation.**

Content after punctuation example.

*This text also ends with punctuation!*

More content.

**This ends with a question mark?**

Content.

**This ends with a semicolon;**

Content.

**This ends with a colon:**

Content.

**This ends with a comma,**

Content.

**This text ends with full-width punctuation。**

Content with full-width punctuation.

**This ends with full-width comma，**

Content.

**[This is a link](https://example.com)**

Links in emphasis should be allowed.

*[Another link](https://example.org)*

More link examples.

This paragraph has
**emphasis that spans
multiple lines** and should
not be flagged.

This is another paragraph with
*multi-line italic
text* that should be allowed.

**This is an emphasized paragraph
that continues on another line
and should not trigger the rule.**

Content after multi-line.

Regular paragraph text without any emphasis.

## Code Examples

In code blocks, emphasis markers should not trigger:

```markdown
**This looks like emphasis but is in code**
*This too*
```

Inline code: `**not emphasis**` should not trigger.

**  **

Empty emphasis should not trigger violations.

*   *

Another empty emphasis example.