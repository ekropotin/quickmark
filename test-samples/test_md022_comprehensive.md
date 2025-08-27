# MD022 Comprehensive Test

This file tests various MD022 scenarios for blank lines around headings.

## Valid cases

Text with blank line above

# Proper ATX heading with blank lines

Text below with proper spacing


## More proper headings

Text above

Setext heading level 1
======================

Text below

Setext heading level 2  
----------------------

Final text after setext

## Document boundaries

# Heading at start is valid

Content in middle

# Heading at end is valid

## Violation cases

Text without blank line
# ATX heading violation above

# ATX heading violation below
Text without blank line

Text without blank line
## Both violations
Text without blank line

Text above setext
Setext Violation Above
======================

Setext Violation Below
----------------------
Text below setext

Mixed violations
================
More text here