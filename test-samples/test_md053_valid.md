# Test MD053 Valid Cases

This file contains reference definitions that should not trigger MD053 violations.

## All References Used

[Valid full reference][label1]
[Another full reference][label2]

[label1][]
[label2][]

![Valid image][image1]
![Another image][image2]

## Shortcut References

[label3]
[label4]

## Case Insensitive Matching

[Valid case insensitive][LABEL5]
[Mixed Case][Label6]

## Whitespace Normalization

[Valid with spaces][  label7  ]
[Valid with tabs][	label8	]

## Duplicate Definitions (first one is used)

[Valid duplicate reference][duplicate]

## Ignored Definitions

[//]: # (This is a comment)
[//]: <> (Another comment)

## Reference Definitions

[label1]: https://example.com/1
[label2]: https://example.com/2
[label3]: https://example.com/3
[label4]: https://example.com/4
[label5]: https://example.com/5
[label6]: https://example.com/6
[label7]: https://example.com/7
[label8]: https://example.com/8
[image1]: https://example.com/image1.png
[image2]: https://example.com/image2.png

[duplicate]: https://example.com/first
[duplicate]: https://example.com/second