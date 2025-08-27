# MD027 Violations Test

This file contains blockquotes that violate MD027 by having multiple spaces after the blockquote symbol.

## Basic violations

> This is correct
>  This has multiple spaces
>   This has three spaces
>    This has four spaces

## Nested blockquotes

> Level 1
>>  Level 2 with multiple spaces
> >  Another level 2 with multiple spaces

## With leading whitespace

 >  Indented blockquote with multiple spaces
  >   Two spaces then multiple spaces
   >    Three spaces then multiple spaces

## Empty blockquotes with spaces

>  
>   
>    

## Mixed content

> Good blockquote
>  Bad blockquote with multiple spaces
> Another good one
>   Another bad one with three spaces

## List items in blockquotes (should violate by default)

>  - List item with multiple spaces
>   * Another list item
>    1. Ordered list item

## Complex nesting

> > > Level 3
>>  > Level 3 with violation
> >>  Level 3 with violation