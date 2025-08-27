# MD028 Test: Valid Cases

This file contains various cases that should NOT violate MD028 (no-blanks-blockquote).

## Continuous blockquotes without blank lines

> First line of blockquote
> Second line of blockquote
> Third line of blockquote

## Blockquotes separated by content

> First blockquote with content

Some separating content here.

> Second blockquote after proper separation

## Blockquotes with proper blank line markers

> First line
>
> Second line after blank with marker

## Nested blockquotes done correctly

> First level
> > Second level
> > > Third level properly nested

## Single blockquote (no separation issue)

> This is just a single blockquote
> with multiple lines
> all properly formatted

## Blockquotes separated by headings

> First quote

## Heading separator

> Second quote after heading

## Blockquotes separated by lists

> Quote before list

- List item 1
- List item 2

> Quote after list

## Blockquotes separated by code blocks

> Quote before code

```
code block here
```

> Quote after code

## Mixed content with proper separation

> Quote with content

Here's some text explaining things.

Another paragraph.

> Another quote properly separated

## Blockquotes with links and other elements

> Quote with [link](https://example.com)

Text between quotes.

> Quote with `inline code`

## Indented blockquotes (valid)

   > Properly indented blockquote
   > with multiple lines

Text in between.

   > Another indented blockquote properly separated

## Empty blockquotes with markers

>
> Content after empty line with marker
>

## Complex valid nesting

> Level 1 content
> > Level 2 content
> > 
> > Level 2 continues with proper marker
> Level 1 continues