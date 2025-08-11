# Comprehensive MD018 Test

This file tests various scenarios for the MD018 rule.

## Valid Cases

# Valid heading 1
## Valid heading 2
### Valid heading 3
#### Valid heading 4
##### Valid heading 5
###### Valid heading 6

# Valid with multiple spaces
##  Valid with multiple spaces
###   Valid with multiple spaces

#  Valid with tab after hash

## Invalid Cases (should trigger MD018)

#Invalid heading 1
##Invalid heading 2
###Invalid heading 3
####Invalid heading 4
#####Invalid heading 5
######Invalid heading 6

#NoSpace
##NoSpaceHere
###StillNoSpace

## Edge Cases

Lines that should NOT trigger MD018:

```
#CodeBlockShouldNotTrigger
##AlsoShouldNotTrigger
```

    #IndentedCodeBlockShouldNotTrigger
    ##AlsoIndentedShouldNotTrigger

<div>
#HTMLBlockShouldNotTrigger  
##AlsoInHTMLShouldNotTrigger
</div>

Hash-only lines (should not trigger):

#

##

###

####

#####

######

# 

##  

###   

####    

#️⃣ Emoji hashtag should not trigger

#️⃣NotEvenThisOne should not trigger because it starts with emoji

Lines with # not at start should not trigger:
This line has a #hashtag but not at start
And another #example in middle

## Mixed valid and invalid

# Valid heading

#Invalid immediately after

## Another valid

###Another invalid

# Final valid heading