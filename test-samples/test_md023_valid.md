# Valid headings - should not trigger MD023

All these headings start at the beginning of the line.

# ATX Heading Level 1

## ATX Heading Level 2

### ATX Heading Level 3

#### ATX Heading Level 4

##### ATX Heading Level 5

###### ATX Heading Level 6

# ATX Closed Heading #

## ATX Closed Heading with Extra Spaces ###

Setext Heading Level 1
======================

Setext Heading Level 2
----------------------

Another Setext H1
=================

Another Setext H2
-----------------

> # Heading in blockquote is valid

> ## Another heading in blockquote
> Text in blockquote.

> Setext heading in blockquote
> =============================

* List item with proper heading:

# This is valid according to CommonMark spec

Some code examples that should NOT trigger MD023:

```ruby
# This is a comment, not a heading
   # Indented comment should not trigger
```

```yaml
# Configuration
title: "My Document"
   # Another comment
```

`# Inline code with hash`

Text with `# hash symbol` in inline code.

## Normal heading after code examples

Text