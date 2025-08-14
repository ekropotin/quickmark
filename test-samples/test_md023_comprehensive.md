# MD023 Comprehensive Test

This document tests the MD023 rule (heading-start-left) comprehensively.

## Valid Examples

These should NOT trigger MD023:

# ATX Heading Level 1

## ATX Heading Level 2

### ATX Heading Level 3

#### ATX Heading Level 4

##### ATX Heading Level 5

###### ATX Heading Level 6

# ATX Closed Heading #

## ATX Closed Heading with spaces ###

Setext Heading Level 1
======================

Setext Heading Level 2
----------------------

Another Setext H1
=================

Another Setext H2
-----------------

> # Heading in blockquote (valid)

> ## Another heading in blockquote
> Text in blockquote continues.

> Setext in blockquote
> ====================

```ruby
# This is a code comment, not heading
   # Indented comment in code
```

```yaml
# YAML comment
title: "Document"
   # Another comment
```

`# Hash in inline code`

Text with `# hash symbol` inline.

## Violations

These SHOULD trigger MD023:

 # ATX H1 with 1 space indent

  ## ATX H2 with 2 space indent

   ### ATX H3 with 3 space indent

    #### ATX H4 with 4 space indent

     ##### ATX H5 with 5 space indent

      ###### ATX H6 with 6 space indent

 # ATX Closed with indent #

  ## ATX Closed with 2 spaces ###

 Setext H1 text indented
========================

  Setext H1 text with 2 spaces  
===============================

Setext H1 with indented underline
 =================================

 Setext H1 both text and underline indented
 ==========================================

   Setext H2 text with 3 spaces
   -----------------------------

Setext H2 underline only indented
 ---------------------------------

## Edge Cases

* List item with valid heading:

# This is valid per CommonMark section 5.2

* List item with invalid heading:
  ## This should trigger MD023

- Dash list with invalid heading:
   ### This should trigger MD023

1. Numbered list item:
    #### This should trigger MD023

2. Another numbered item with valid heading:

##### Valid heading after list

## Mixed Content

Normal paragraph.

 # This heading should trigger

Normal heading after violation.

## Final Section

This concludes the MD023 comprehensive test.