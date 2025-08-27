# MD027 List Items Configuration Test

This file is specifically for testing the list_items configuration option.

## Cases that should be affected by list_items setting

>  - Unordered list with multiple spaces
>   * Another unordered list
>    + Plus sign list

>  1. Ordered list with multiple spaces
>   2. Another ordered item
>    3. Third item

## Mixed content - some list items, some not

>  This is regular text with multiple spaces (should always violate)
>  - This is a list item (behavior depends on list_items setting)
>   Regular text again (should always violate)
>   * Another list item (behavior depends on list_items setting)

## Nested lists in blockquotes

> >  - Nested list with multiple spaces
> >   1. Nested ordered list

## Non-list content (should always violate regardless of list_items)

>  Just regular text with multiple spaces
>   More regular text
>    Even more text