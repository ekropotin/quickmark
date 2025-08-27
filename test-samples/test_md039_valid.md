# Valid Link Text Examples

This file contains examples of links that should NOT trigger MD039 violations.

## Valid inline links

[link text](https://example.com)

[another link](url)

[link with spaces in url](https://example.com/path with spaces)

[empty link]()

## Valid reference links

[reference link][ref1]

[another ref link][ref2]

## Valid shortcut reference links

[ref1]

[ref2]

## Valid collapsed reference links

[ref1][]

[ref2][]

## Valid links with markup

[**bold link**](https://example.com)

[*italic link*](https://example.com)

[`code link`](https://example.com)

[~~strikethrough link~~](https://example.com)

## Images (should not be affected by this rule)

![image alt text](image.jpg)

![ image with spaces ](image.jpg)

![  lots of spaces  ](image.jpg)

## Link definitions

[ref1]: https://example.com
[ref2]: https://another-example.com

## Links in code blocks (should not be affected)

```
[ link with spaces ](url)
```

    [ indented code link ](url)

## Inline code with brackets (should not be affected)

This is `[ not a link ]` in code.

## Text in brackets that's not a link

This is [ not a link ] because it has no URL.

Some [ text in brackets ] for emphasis.