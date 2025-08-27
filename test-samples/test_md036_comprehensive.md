# MD036 Comprehensive Test

This file contains a mix of valid and invalid examples for MD036 testing.

## Valid Examples

### Proper Headings

This content is under a proper ATX heading.

#### Another Proper Heading

More content under proper headings.

Setext Heading
==============

Content under a setext heading.

Another Setext
--------------

More setext content.

### Emphasis in Paragraphs

This is a paragraph with **normal emphasis** that should be allowed.

This paragraph contains *italic text* in the middle and should not be flagged.

### Emphasis with Punctuation

**This bold text ends with a period.**

Content after period example.

*This italic text ends with an exclamation mark!*

More content here.

**This ends with question mark?**

Question content.

**This text uses full-width punctuation。**

Full-width period content.

**Text with comma，**

Full-width comma content.

### Links in Emphasis

**[This is a bold link](https://example.com)**

Link content.

*[This is an italic link](https://github.com)*

More link content.

### Multi-line Emphasis

This paragraph contains
**emphasis that spans
across multiple lines** and should
not be flagged as a heading.

Another example with
*italic text that
continues on multiple
lines* should also be allowed.

**This is a completely emphasized paragraph
that spans multiple lines and contains
various words and punctuation marks
but should not be flagged.**

Multi-line content.

### Code and Empty Emphasis

Inline code: `**not emphasis**` should not trigger.

```markdown
**This looks like emphasis but is in a code block**
*This too should not trigger*
```

**  **

Empty emphasis above.

## Invalid Examples (Should Trigger MD036)

**Section One**

This looks like a heading but is actually bold text.

*Section Two*

This looks like a heading but is actually italic text.

**Important Notice**

Content under fake heading.

_Underscore Heading_

More content under fake heading.

**Multi-word Section Title**

Content under multi-word fake heading.

*Single*

Single word fake heading.

**CamelCaseHeading**

CamelCase fake heading.

**Heading-with-dashes**

Dashed fake heading.

**123 Numbered Section**

Numbered fake heading.

**UPPERCASE SECTION**

Uppercase fake heading.

**Section with Various Words**

Multi-word fake heading.

***Triple Emphasis Heading***

Triple emphasis fake heading.

## Mixed Content

This is a proper paragraph with **normal emphasis** that should not be flagged.

**But This Looks Like a Heading**

And should be flagged.

Another paragraph with *inline emphasis* that is perfectly fine.

*But This Also Looks Like a Heading*

And should also be flagged.

### Proper Heading After Violations

This should not be flagged as it's a proper heading.

**Another Violation Here**

This should be flagged.

End of comprehensive test.