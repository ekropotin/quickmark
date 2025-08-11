# Valid MD032 Test Cases

This file contains examples that should NOT trigger MD032 violations.

## Properly spaced lists

Text before list.

* List item 1
* List item 2

Text after list.

## Ordered lists with proper spacing

Some text.

1. First item
2. Second item

More text.

## Lists at document boundaries

* List at start of document
* Second item

Text in middle.

* List at end of document
* Last item

## Nested lists (should not trigger MD032)

Text before outer list.

* Outer item 1
  * Nested item 1
  * Nested item 2
* Outer item 2
  * Another nested item

Text after outer list.

## Lists in blockquotes with proper spacing

> Some quoted text.
>
> * Quoted list item 1
> * Quoted list item 2
>
> More quoted text.

## Lists with lazy continuation (valid per CommonMark)

Text before list.

1. List item one
   Continued text for item one.
2. List item two
More lazy continuation for item two.

Text after list.

## Mixed content with proper spacing

Some text.

* Unordered item
* Another item

## Code block

```javascript
console.log("code");
```

## Ordered list

1. Numbered item
2. Another numbered item

Final text.