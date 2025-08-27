# MD020 Comprehensive Test File

This file tests various scenarios for the MD020 rule (no-missing-space-closed-atx).

## Valid Cases

# Correct spacing single hash #

## Correct spacing double hash ##

### Correct spacing triple hash ###

#### Correct spacing quad hash ####

##### Correct spacing five hash #####

###### Correct spacing six hash ######

# Multiple   spaces   work   #

## 	Tab	characters	work	##

### Mixed 	whitespace 	types ###

#### Content with # hashes inside ####

##### Content with ## multiple ### hashes #####

###### Content with **formatting** and `code` ######

# Escaped hash at end with space \# #

## Backslash in middle \# with proper spacing ##

### Complex content with \#escaped and normal# hashes ###

## Violation Cases

#SingleViolation#

##DoubleViolation##  

###TripleViolation###

####QuadViolation####

#####FiveViolation#####

######SixViolation######

# LeftSpaceOnly#

## RightSpaceOnly##

###NoSpacesAtAll###

# Content with hash#

## Multiple#hash#problems ##

###Mixed spacing problems ###

#### Problems with \#### 

##### Content ending with escape\#####

###### Multiple issues #######

## Edge Cases

# Single character content a #

## Empty-ish   ##

### Just symbols !@# ###

#### Numbers 123 ####

##### Unicode content 你好 #####

###### Emoji content 🚀 ######

# Very long content that goes on and on and on and on and should still work properly #

## Content with "quotes" and 'apostrophes' ##

### Content with (parentheses) and [brackets] ###

#### Content with <HTML> tags ####

## Ignored Blocks

```
#IgnoreInCodeBlock#
##AlsoIgnored##
###ViolationIgnored###
```

```python
def function():
    #Comment style in code
    ##Also ignored##
    return "#NotAHeading#"
```

    #IndentedCodeBlock#
    ##AlsoIndentedIgnored##
    ###IndentedViolation###

<div>
#HTMLBlockContent#
##IgnoredHTMLViolation##
</div>

<script>
// #JavaScriptComment#
##AlsoIgnored##
</script>

## Mixed Open and Closed

# Open ATX heading (no MD020 issue)

## Another open heading

### Closed heading violation###

#### Open heading again

##### Another closed violation#####

###### Final open heading

## Special Characters and Escaping

# Content with \ backslash #

## Content with \# escaped hash ##

### Content with \\# double backslash ###

#### Content with \\\# triple backslash ####

##### Content with \\ at end #####

###### Content with real end \######

# Tab and space combinations 	#

##	Leading tab violation##

### Trailing tab violation	###

#### 	Both tabs fine 	####

##  Multiple spaces after content  ##

###	Mixed tabs and spaces	 ###

#Violation at start#

## Violation at end##

###Both violations###

#### 	Tab start violation####

#####Space end violation #####

######	Tab and space violations	######