# MD020 No Space Inside Hashes on Closed ATX Style Heading Violations

This file demonstrates violations of the MD020 rule (no-missing-space-closed-atx).

#Heading 1#

## Heading 2##

##Heading 3##

### Heading 4###

#####Heading 5#####

# Heading with \###

###Content with hashes # inside###

```
#IgnoredInCodeBlock#
##AlsoIgnored##  
```

    #IndentedCodeBlockIgnored#
    ##AlsoIgnored##

<div>
#HTMLBlockIgnored#
##AlsoIgnored##
</div>

# Unbalanced closing hashes ### - OK: Open ATX heading (not closed, MD020 doesn't apply)

## Extra closing hashes ########## - OK: Open ATX heading

### Just content, no closing - OK: Open ATX heading

#### Content with backslash \# at end #### - OK: Backslash before hash with space