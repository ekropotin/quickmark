# MD020 Valid Examples

This file contains only valid closed ATX headings with proper spacing.

# Properly spaced heading 1 #

## Properly spaced heading 2 ##

### Properly spaced heading 3 ###

#### Multiple spaces work fine    ####

##### Tab characters also work	#####

###### Mixed whitespace is ok 	######

# Heading with # hash inside content #

## Heading with ## multiple ### hashes inside ##

### Content with **bold** and *italic* ###

#### Content with `code` inside ####

##### Content with [link](url) inside #####

###### Content with ![image](url) inside ######

# Open ATX headings are not affected by MD020

## These don't need closing hashes

### MD020 only applies to closed ATX headings

Setext Headings Are Fine Too
=============================

Another Setext Heading
----------------------

```markdown
# Code blocks are ignored
#EvenIfTheyLookWrong#
##NoSpacesNeeded##
```

    # Indented code blocks are also ignored
    #NoViolation#
    ##AlsoFine##

<div>
# HTML blocks are ignored
#ThisIsFine#
##NoViolation##
</div>

# Headings with escaped content \# are fine if properly spaced #

## Backslash at end with space \# ##

### Complex content with \# escaped hashes in middle ###

#### Content ending with actual backslash \ ####