# All Rules Violations Test

This file contains violations of all available quickmark rules to test comprehensive linting.

# Heading Level 1

### Heading Level 3 - MD001 VIOLATION: Skips level 2

## Heading Level 2 - OK now  

##### Heading Level 5 - MD001 VIOLATION: Skips levels 3 and 4

Some paragraph content between headings.

## ATX Closed Heading Level 2 ##

This introduces MD003 VIOLATION: Mixed styles (first was open ATX, this is closed ATX).

#### Heading Level 4 - MD001 VIOLATION: Skips level 3

More content here.

Setext Heading Level 1
======================

This is MD003 VIOLATION: Mixed styles (now using setext after ATX styles above).

### Heading Level 3 - OK increment from setext level 1

###### Heading Level 6 - MD001 VIOLATION: Skips levels 4 and 5  

Another Setext Heading
----------------------

More MD003 VIOLATION: Mixed setext and ATX styles in same document.

## Another ATX Closed ## 

Final content with MD003 VIOLATION: Mixed open/closed ATX styles.

# Final Heading ### 

MD003 VIOLATION: This is inconsistent (closed on a previously open ATX document).

Multiple violations per line and comprehensive rule coverage for integration testing.