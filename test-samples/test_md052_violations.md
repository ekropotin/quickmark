# Test MD052 Violations

This file contains reference links and images that should trigger MD052 violations.

## Invalid Full References

[Invalid full reference][missing1]
[Another invalid][missing2]

## Invalid Collapsed References

[missing3][]
[missing4][]

## Invalid Images

![Invalid image][missing_image1]
![Another invalid image][missing_image2]

## Invalid Shortcut References (when enabled)

These would be invalid when shortcut_syntax is enabled:
[undefined1]
[undefined2]

## Multiple Violations in Same Paragraph

This paragraph has [invalid1][missing5] and [invalid2][missing6] references.

## Mixed Valid and Invalid

[Valid reference][valid1]
[Invalid reference][missing7]

## Valid Reference Definitions

[valid1]: https://example.com/valid