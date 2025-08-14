# Test MD026 Valid Cases

These examples should NOT trigger MD026 violations.

## ATX Headings Without Trailing Punctuation

# This is a good heading

## This is another good heading

### Heading without punctuation

#### Sub-heading without punctuation

##### Another level heading

###### Deep level heading

## ATX Closed Style Without Trailing Punctuation

# This is a good closed heading #

## Another closed heading ##

### Third level closed heading ###

## Question Marks Are Allowed by Default

# FAQ: What is this document about?

## How do I configure this?

### When should I use this?

## Setext Headings Without Trailing Punctuation

This is a setext h1
===================

This is a setext h2
-------------------

Another setext heading level 1
===============================

Another setext heading level 2
-------------------------------

## HTML Entities Should Be Ignored

# Copyright &copy; 2023

## Registered Trademark &reg;

### Copyright &#169; 2023

#### Copyright &#x000A9; 2023

##### Registered &#174; 2023

###### Registered &#xAE; 2023

## Headings in Lists and Blockquotes

- List item
  # Heading in list

> # Heading in blockquote
> 
> ## Another heading in blockquote

* Another list
  ## Another heading in list

## Headings in Code Blocks Should Be Ignored

```markdown
# This heading has a period.
## This heading has an exclamation!
### This heading has a comma,
```

`# Inline code with heading and period.`

## Empty or Whitespace-Only Headings

# 

##   

###

## Gemoji and Emoji at End

# Happy face :smile:

## Star emoji ⭐

### Heart emoji ❤️