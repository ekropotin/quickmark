# MD028 Test: Violations

This file contains various cases that should violate MD028 (no-blanks-blockquote).

## Basic violation - single blank line between blockquotes

> First blockquote

> Second blockquote

## Multiple blank lines between blockquotes

> First blockquote


> Second blockquote with multiple blank lines

## Nested blockquotes with blank lines

> First level
> > Second level

> > Another second level after blank line

## Mixed content but still violations

> Some blockquote

> Another blockquote after one blank line

## Complex nested structure

> Level 1
> > Level 2
> > > Level 3

> > Different level 2 after blank

## Multiple violations in sequence

> First quote

> Second quote

> Third quote

## Indented blockquotes with violations

   > Indented first blockquote

   > Indented second blockquote

## Blockquotes with various content

> Quote with **bold** text

> Quote with *italic* text after blank line

## Edge case: blockquote after text with blank line issue

Some text here

> This blockquote comes after text

> This blockquote should be flagged as violation