# MD021 Comprehensive Test Cases

This file contains comprehensive test cases for MD021 rule (multiple spaces inside hashes on closed atx style heading).

## Valid Cases (No Violations Expected)

### Regular ATX headings (not closed) - should be ignored by MD021
# Regular heading
##  Regular heading with multiple spaces (should be caught by MD019, not MD021)
### Regular heading
#### Another regular heading

### Correctly formatted closed ATX headings
# Single space closed heading #
## Double hash closed heading ##
### Triple hash closed heading ###
#### Quad hash closed heading ####
##### Five hash closed heading #####
###### Six hash closed heading ######

### No spaces (valid for MD021, might be caught by other rules)
#No spaces#
##No spaces##
###No spaces###

### Setext headings (not ATX)
Setext Heading Level 1
=====================

Setext Heading Level 2
----------------------

### Content in code blocks (should be ignored)
```
# This is code, not a heading #
##  Multiple spaces in code ##
###   More spaces in code   ###
```

    # Indented code block #
    ##  Multiple spaces here too  ##

## Violation Cases (Violations Expected)

### Multiple spaces after opening hashes
##  Two spaces after opening ##
###   Three spaces after opening ###
####    Four spaces after opening ####

### Multiple spaces before closing hashes
## Two spaces before closing  ##
### Three spaces before closing   ###
#### Four spaces before closing    ####

### Multiple spaces on both sides
##  Both sides have multiple  ##
###   Both sides have multiple   ###
####    Both sides have multiple    ####

### Single hash cases
#  Multiple spaces after single hash #
# Multiple spaces before single hash  #
#  Both sides with single hash  #

### Tab characters
##	Tab after opening	##
##  	Mixed space and tab  ##
###	Tab after opening	###

### Mixed content
## Valid heading ##
###  Invalid heading with multiple spaces ###
#### Valid heading ####
#####   Another invalid heading   #####

### Edge cases
# Edge case with single hash and mixed spaces #
##    Many spaces    ##
###     Even more spaces     ###

### Escaped hashes (should still be detected)
## Multiple spaces before escaped hash  \##
### Multiple spaces with escaped hash  \###