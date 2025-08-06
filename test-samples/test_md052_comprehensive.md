# Test MD052 Comprehensive

This file tests various MD052 features and configuration options.

## Basic Valid References

[Valid full][label1]
[Valid collapsed][]
[Valid image][img1]

## Basic Invalid References

[Invalid full][missing1]
[Invalid collapsed missing][]
[Invalid image][missing_img]

## Shortcut Syntax Tests

These are potential shortcut references:
[potential1]
[potential2]
[valid_shortcut]

## Ignored Labels

GitHub task list items should be ignored by default:
- [x] Completed task
- [ ] Incomplete task
- [X] Capital X task

Custom ignored patterns would ignore these:
[custom1]
[custom2]

## Case Sensitivity Tests

[Mixed Case Reference][LABEL1]
[lowercase reference][label1]

## Whitespace Normalization Tests

[Reference with spaces][  label with spaces  ]
[Reference with tabs][	label	with	tabs	]

## Complex Reference Definitions

[label1]: https://example.com/1 "Title 1"
[valid collapsed]: https://example.com/collapsed
[img1]: https://example.com/image.png
[valid_shortcut]: https://example.com/shortcut
[label with spaces]: https://example.com/spaces
[label	with	tabs]: https://example.com/tabs

## Inline Links (Not Reference Links)

These should not be processed by MD052:
[Inline link](https://example.com)
![Inline image](https://example.com/image.png)

## Mixed Content

This paragraph contains [valid reference][label1], [invalid reference][missing], 
and ![valid image][img1] along with ![invalid image][missing_img2].

## Edge Cases

Empty brackets: []
Just text in brackets without reference: [this is just text]
Escaped brackets: \[not a reference\]

## Multiple Lines

[Multi-line
reference][label1]

[Another
multi-line][missing_multiline]

## Reference Definitions at Different Positions

Some definitions at the end:

[end_label]: https://example.com/end

[Reference to end][end_label]
[Invalid end reference][missing_end]