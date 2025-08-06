# Test MD053 Violations

This file contains reference definitions that should trigger MD053 violations.

## Unused Definitions

[Valid reference][used1]

Some text content.

## Multiple Unused Definitions

Some more content without any references.

## Duplicate Definitions (unused)

Some text that doesn't reference anything.

## Duplicate Definitions (used)

[Valid reference to duplicated][duplicate_used]

## Mixed Scenarios

[Valid reference][mixed_valid]

Some text content.

## Reference Definitions (Some Unused, Some Duplicated)

[used1]: https://example.com/used1
[unused1]: https://example.com/unused1
[unused2]: https://example.com/unused2
[duplicate_unused]: https://example.com/duplicate1
[duplicate_unused]: https://example.com/duplicate2
[duplicate_used]: https://example.com/first_used
[duplicate_used]: https://example.com/second_used
[mixed_valid]: https://example.com/mixed_valid
[unused3]: https://example.com/unused3