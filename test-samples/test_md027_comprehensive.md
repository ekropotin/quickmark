# MD027 Comprehensive Test

This file contains a comprehensive set of blockquote examples to test MD027 thoroughly.

## Basic cases

> This is correct
>  This should violate - multiple spaces
> This is also correct
>   This should violate - three spaces

## No space after >

>No space is valid
>Another line without space

## Nested blockquotes

> Level 1 correct
>> Level 2 correct
>>  Level 2 violation - multiple spaces
> > Alternative level 2 correct
> >  Alternative level 2 violation
>>> Level 3 correct
>>>  Level 3 violation

## Leading whitespace combinations

 > Leading space + correct blockquote
 >  Leading space + violation
  > Two leading spaces + correct
  >   Two leading spaces + violation
   > Three leading + correct
   >    Three leading + violation

## Empty blockquotes

>
> 
>  
>   
>    

## List items in blockquotes

> - Correct list item
>  - List item violation
> * Correct asterisk
>   * Asterisk violation
> 1. Correct ordered
>  2. Ordered violation
>   3. Another ordered violation

## Mixed content scenarios

> Normal blockquote text
>  Violation in the middle
> Back to normal
>
> New blockquote paragraph
>   Another violation

## Code blocks (should be ignored)

    > This is in an indented code block
    >  Even with multiple spaces
    >   Should not trigger violations

```
> This is in a fenced code block
>  Multiple spaces here
>   Should also be ignored
```

## Complex nesting patterns

> > > All correct
>>  > Middle violation
> >>  Last violation
> > >  All positions violation

## Edge cases

>
>>
>>>
> >
>> >
>>> >

## Content that looks like violations but isn't

> Some text with  multiple spaces in content
> More text with   spaces inside

## Real-world examples

> **Important:** This is a note
>  This would be a violation
> 
> *Note:* Another important point
>   This is also a violation