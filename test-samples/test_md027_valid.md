# MD027 Valid Test

This file contains blockquotes that should NOT violate MD027.

## Correct blockquotes

> This is a standard blockquote
> Another line in the blockquote

## Blockquotes with no space after >

>This is also valid
>Another line without space

## Blockquotes with exactly one space

> Single space is correct
> Multiple lines with single space

## Nested blockquotes (correct)

> Level 1
>> Level 2
> > Another way to do level 2
>>> Level 3
>> > Mixed nesting style

## With leading whitespace (correct)

 > Blockquote with leading space
  > Two leading spaces
   > Three leading spaces

## Empty blockquotes (valid)

>
> 
>>

## Normal content after blockquotes

> Quote followed by normal text

This is normal text, not a blockquote.

## Lists in blockquotes (correct spacing)

> - List item with single space
> * Another list item
> 1. Ordered list item

## Code blocks that look like blockquotes

    > This is in a code block
    > So it should not be treated as blockquote

```
> This is also in a code block
> Not a real blockquote
```

## Complex valid nesting

> > > Level 3 correctly formatted
>> > Level 3 another way
> >> Level 3 mixed style