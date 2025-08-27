# MD021 Test Cases - Valid

This file contains test cases that should NOT trigger MD021 violations.

## Regular ATX headings (not closed)

These should not trigger MD021 because they are not closed ATX headings:

# Regular heading
## Regular heading
### Regular heading with multiple spaces
#### Another regular heading

## Correctly formatted closed ATX headings

These should not trigger MD021 because they have single spaces:

# Correctly formatted closed heading #
## Correctly formatted closed heading ##
### Correctly formatted closed heading ###
#### Correctly formatted closed heading ####
##### Correctly formatted closed heading #####
###### Correctly formatted closed heading ######

## No spaces around hashes (also valid)

These should not trigger MD021 (MD021 only cares about multiple spaces, not missing spaces):

#No spaces around hashes#
##No spaces around hashes##
###No spaces around hashes###

## Setext headings (not ATX)

These should not be affected by MD021:

Setext Heading Level 1
=====================

Setext Heading Level 2
----------------------

## Content that looks like headings but isn't

Regular text with # symbols in it should not be affected.

Code blocks with headings:

```
# This is code, not a heading #
## This is also code ##
```

    # This is an indented code block #
    ## Not a heading either ##