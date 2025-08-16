# MD039 Comprehensive Test - Spaces Inside Link Text

This file contains a comprehensive set of test cases for MD039, including both valid and invalid examples.

## Valid Examples (should not trigger violations)

### Basic valid links
[good link](https://example.com)
[another good link](url)
[empty]()

### Valid reference links
[good reference][ref1]
[another good reference][ref2]

### Valid shortcut reference links
[ref1]
[ref2]

### Valid collapsed reference links
[ref1][]
[ref2][]

### Links with markup (valid)
[**bold**](url)
[*italic*](url)
[`code`](url)
[~~strikethrough~~](url)

## Invalid Examples (should trigger violations)

### Leading spaces
[ leading](url)
[  double leading](url)
[	tab leading](url)

### Trailing spaces
[trailing ](url)
[double trailing  ](url)
[tab trailing	](url)

### Both leading and trailing
[ both ](url)
[  double both  ](url)

### Reference links with spaces
[ bad reference ][ref1]
[bad trailing ][ref1]
[ bad both ][ref1]

### Shortcut references with spaces
[ bad shortcut ]
[bad trailing ]
[ bad both ]

### Collapsed references with spaces
[ bad collapsed ][]
[bad trailing ][]
[ bad both ][]

### Empty with spaces
[ ](url)
[  ](url)
[	](url)

### Mixed content with spaces
[ **bold with spaces** ](url)
[ *italic with spaces* ](url)
[ `code with spaces` ](url)

## Edge Cases

### Images (should NOT trigger violations)
![valid image](image.jpg)
![ image with spaces ](image.jpg)
![  lots of spaces  ](image.jpg)

### Code blocks (should NOT trigger violations)
```
[ link with spaces ](url)
[ another spaced link ](url)
```

    [ indented code link ](url)

### Inline code (should NOT trigger violations)
This is `[ not a link ]` in code.
Use `[ spaced brackets ]` for arrays.

### Text in brackets (should NOT trigger violations)
This is [ not a link ] because no URL.
Some [ text ] for emphasis.

### Complex mixed scenarios
Good [link](url1) and [ bad link ](url2) in same paragraph.

Valid [reference][ref1] and [ bad reference ][ref2] links.

[Good start][] but [ bad end ][].

## Link Definitions
[ref1]: https://example.com
[ref2]: https://another-example.com

## Multi-line links
[link 
text](url)

[ spaced
link ](url)

[good
reference][ref1]

[ bad
reference ][ref1]