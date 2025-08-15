# MD036 Test - Violations

This file contains examples that should trigger MD036 violations (emphasis used instead of heading).

**Section 1**

This is content under what appears to be a heading but is actually bold text.

*Section 2*

This is content under what appears to be a heading but is actually italic text.

**Important Note**

More content here.

_Another Section_

Content under an italic "heading".

**Yet Another Section**

And more content.

***Section with both emphasis***

This should also be flagged as it looks like a heading.

Some regular paragraph with **normal emphasis** that should not be flagged.

**Multi-word section heading**

This should be detected as a violation.

*Single word*

This should also be flagged.

**CamelCaseSection**

This should be flagged too.

**Section-with-dashes**

This should be flagged as well.

**123 Numbered Section**

Numbers in the "heading" should still be flagged.

**Section with, commas and other stuff**

This should be flagged.

**Section with UPPERCASE**

This should be flagged.