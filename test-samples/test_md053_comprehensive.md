# Test MD053 Comprehensive

This file contains a comprehensive set of test cases for MD053 rule validation.

## Valid Scenarios

### All Reference Types Used

[Full reference][valid_full]
[valid_collapsed][]
[valid_shortcut]

![Image reference][valid_image]

### Case Insensitive and Whitespace Normalization

[Mixed case reference][VALID_CASE]
[Whitespace reference][  valid_spaces  ]

### Ignored Definitions

[//]: # (This is a comment - should be ignored)
[//]: <> (Another comment - should be ignored)

## Violation Scenarios

### Unused Definitions

Some text without any references to unused definitions.

### Duplicate Definitions

[Reference to first duplicate][duplicate_referenced]

### Multiple Issues

Some text with [valid ref][valid_multi] but no reference to unused_multi.

## Reference Definitions

### Valid Definitions (Used)

[valid_full]: https://example.com/valid_full
[valid_collapsed]: https://example.com/valid_collapsed
[valid_shortcut]: https://example.com/valid_shortcut
[valid_image]: https://example.com/valid_image.png
[valid_case]: https://example.com/valid_case
[valid_spaces]: https://example.com/valid_spaces
[duplicate_referenced]: https://example.com/first_duplicate
[duplicate_referenced]: https://example.com/second_duplicate
[valid_multi]: https://example.com/valid_multi

### Invalid Definitions (Unused)

[unused_simple]: https://example.com/unused_simple
[unused_complex]: https://example.com/unused_complex "Title"

### Invalid Definitions (Duplicate)

[duplicate_unused]: https://example.com/dup1
[duplicate_unused]: https://example.com/dup2

### Mixed Invalid

[unused_multi]: https://example.com/unused_multi