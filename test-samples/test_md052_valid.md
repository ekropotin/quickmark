# Test MD052 Valid Cases

This file contains valid reference links and images that should not trigger MD052 violations.

## Valid Full References

[Valid full reference][label1]
[Another full reference][label2]

## Valid Collapsed References

[label1][]
[label2][]

## Valid Images

![Valid image][image1]
![Another image][image2]

## Valid Shortcut References (when enabled)

This section would be valid when shortcut_syntax is enabled:
[label1]
[label2]

## Ignored Labels (GitHub task lists)

- [x] Completed task
- [ ] Incomplete task

## Reference Definitions

[label1]: https://example.com/1
[label2]: https://example.com/2
[image1]: https://example.com/image1.png
[image2]: https://example.com/image2.png

## Case Insensitive Matching

[Valid case insensitive][LABEL1]
[Mixed Case][Label2]

## Whitespace Normalization

[Valid with spaces][  label1  ]
[Valid with tabs][	label2	]

## Duplicate Definitions (first one wins)

[duplicate]: https://example.com/first
[duplicate]: https://example.com/second

[Valid duplicate reference][duplicate]