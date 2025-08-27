# MD039 Violations - Spaces Inside Link Text

This file contains examples that should trigger MD039 violations.

## Leading spaces in inline links

[ leading space](https://example.com)

[  multiple leading spaces](https://example.com)

[	tab leading](https://example.com)

## Trailing spaces in inline links

[trailing space ](https://example.com)

[multiple trailing spaces  ](https://example.com)

[tab trailing	](https://example.com)

## Both leading and trailing spaces

[ both spaces ](https://example.com)

[  multiple both  ](https://example.com)

## Spaces in reference links

[ leading space ][ref1]

[trailing space ][ref1]

[ both spaces ][ref1]

## Spaces in shortcut reference links

[ leading space ]

[trailing space ]

[ both spaces ]

## Spaces in collapsed reference links

[ leading space ][]

[trailing space ][]

[ both spaces ][]

## Empty link text with spaces

[ ](https://example.com)

[  ](https://example.com)

[	](https://example.com)

## Mixed content with spaces

[ **bold with spaces** ](https://example.com)

[ *italic with spaces* ](https://example.com)

[ `code with spaces` ](https://example.com)

## Multiple links with some violations

Good [link](url) and [ bad link ](url) and [another good](url).

Valid [reference][ref] and [ bad reference ][ref] links.

## Link definitions
[ref1]: https://example.com
[ref]: https://example.com