# MD042 Comprehensive Test

This file contains a mix of valid and invalid links to test MD042 comprehensively.

## Valid links that should NOT trigger violations

[Normal link](https://example.com)

[Link with fragment](https://example.com#section)

[Meaningful fragment](#introduction)

[Reference with definition][good-ref]

[good-ref]: https://example.com

[Title-only link (valid)]( "This has a title so it's valid")

## Invalid links that SHOULD trigger violations

[Empty link]()

[Fragment only](#)

[Whitespace only]( )

[Space only]( )

## Sequential bug prevention test

[Link 1](https://example.com)
[Empty link]()
[Link 3](https://example.com)
[Another empty](#)
[Link 5](https://example.com)

This tests the issue #308 where subsequent valid links were incorrectly flagged after an empty link.

## Mixed contexts

### In paragraphs

This paragraph has a [valid link](https://example.com) and an [empty link]() mixed together.

### In lists

- [Valid item](https://example.com)
- [Empty item]()
- [Another valid](https://test.org)
- [Another empty](#)

### In blockquotes

> Here's a [valid quote link](https://example.com) and an [empty quote link]().

### In tables

| Description | Link |
|-------------|------|
| Valid | [Good link](https://example.com) |
| Invalid | [Bad link]() |
| Fragment only | [Fragment](#) |
| Valid fragment | [Good fragment](#section) |

## Edge cases

### Images (should not be affected)

![Empty image]()
![Image fragment](#)
![Normal image](image.jpg)

### Code (should not be affected)

`[Not a real link]()`

```markdown
[Also not real]()
```

### Complex scenarios

[Empty]() followed by [valid](https://example.com) followed by [title only]( "Title").

### Footnote-style (complex case)

[^note]: <> "This is a footnote-style reference"

Note: This footnote case from issue #370 may or may not be detected depending on implementation.

## Reference links

### With definitions (valid)

[Defined reference][defined]

[defined]: https://example.com

### Without definitions (may trigger violations in future implementation)

[Undefined reference][undefined]

Note: Currently we don't validate reference link definitions, so this won't trigger a violation yet.