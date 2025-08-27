# MD028 Comprehensive Test

This file tests MD028 (no-blanks-blockquote) comprehensively with both valid and invalid cases.

## Valid: Basic continuous blockquote

> This is a continuous blockquote
> with multiple lines
> all properly connected

## Invalid: Basic separated blockquotes

> First blockquote

> Second blockquote

## Valid: Proper separation with content

> First blockquote

Some content separating the blockquotes.

> Second blockquote

## Invalid: Multiple blank lines

> First blockquote


> Second blockquote

## Valid: Blank lines with blockquote markers

> First part
>
> Second part with proper marker

## Invalid: Nested blockquotes with blank separation

> Level 1
> > Level 2

> > Another level 2

## Valid: Complex nesting without violations

> Level 1
> > Level 2
> > > Level 3
> > Back to level 2
> Back to level 1

## Invalid: Mixed nesting with violations

> Level 1
> > Level 2

> > Violation here

## Valid: Blockquotes around code blocks

> Before code block

```python
def hello():
    print("Hello")
```

> After code block

## Invalid: Blockquotes with inline elements but still violations

> Quote with **bold**

> Quote with *italic*

## Valid: Lists separating blockquotes

> Before list

1. First item
2. Second item

> After list

## Invalid: Edge case with whitespace

> Quote one
    
> Quote two with whitespace line

## Valid: Headers separating blockquotes

> Quote before header

### Header Here

> Quote after header

## Invalid: Deeply nested violations

> Level 1
> > Level 2
> > > Level 3

> > > Another level 3

## Valid: Proper indentation handling

   > Indented quote 1

Regular text.

   > Indented quote 2

## Invalid: Indented violations

   > Indented quote 1

   > Indented quote 2

## Valid: Empty blockquote markers

>
> Content after empty marker
>
> More content

## Invalid: Complex document structure

Start of document.

> Introduction quote

> Another quote (violation)

Middle content here.

> Some middle quote

> Another middle quote (violation)

End content.

> Final quote

> Last quote (violation)

## Valid: Proper document structure

Start of document.

> Introduction quote

Explanation text here.

> Another quote properly separated

Middle content here.

> Some middle quote

More explanation.

> Another middle quote properly separated

End content.

> Final quote

Closing remarks.

> Last quote properly separated