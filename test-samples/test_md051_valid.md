# Test MD051 Valid Cases

This file contains valid link fragments that should NOT trigger MD051 violations.

## Valid Heading Links

### Basic Heading

[Link to basic heading](#basic-heading)

### Heading with Punctuation

[Link to punctuation heading](#heading-with-punctuation)

### Multiple Words in Heading

[Link to multiple words](#multiple-words-in-heading)

## Custom Anchors

### Custom Heading {#my-custom-anchor}

[Link to custom anchor](#my-custom-anchor)

## Setext Headings

Setext Level 1
==============

[Link to setext h1](#setext-level-1)

Setext Level 2
--------------

[Link to setext h2](#setext-level-2)

## Duplicate Headings

### Duplicate

### Duplicate

[Link to first duplicate](#duplicate)
[Link to second duplicate](#duplicate-1)

## HTML Anchors

<div id="html-anchor">Content with HTML id</div>

[Link to HTML anchor](#html-anchor)

<a name="named-anchor">Named anchor</a>

[Link to named anchor](#named-anchor)

## GitHub Special Fragments

[Link to top](#top)
[Link to line](#L42)
[Link to range](#L10C5-L15C20)

## Mixed Valid Cases

# Main Section

Valid content here.

## Subsection

[Back to main](#main-section)
[Link to subsection](#subsection)

## `header:with:colons_in_it`

[should be ok](#headerwithcolons_in_it)
