# Invalid Link Examples (MD042)

These examples SHOULD trigger MD042 violations.

## Empty URLs

[Empty link]()

[Another empty link]()

## Only fragment identifier

[Fragment only](#)

[Another fragment](#)

## Whitespace-only URLs

[Space only]( )

[Tab only](	)

[Multiple spaces](   )

## Note: URLs with only titles are NOT violations

According to the original markdownlint behavior, links with title attributes
(even without URLs) are considered valid and should NOT trigger MD042.

Examples that are valid (not violations):
- [Title only]( "Just a title")
- [Single quoted title]( 'Title only')

## Mixed empty and valid links (testing bug prevention)

[Valid link](https://example.com)
[Empty link]()
[Another valid](https://test.org)
[Another empty]()

## Empty links in different contexts

Here is an [empty inline link]() in text.

- [Empty link in list]()
- [Another empty](  )

> [Empty link in blockquote](#)

| Link | URL |
|------|-----|
| [Empty table link]() | Empty |

## Multiple empty patterns on same line

[First empty]() and [second empty](#) and [third empty]( ).

## Reference links without definitions

[Undefined reference][missing]

[Another undefined][also-missing]

Note: These may not be detected in our current implementation since we skip reference link validation for now.