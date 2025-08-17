# MD059 Violations - Generic Link Text

These links contain generic, non-descriptive text that violates MD059:

## Basic violations

[click here](https://example.com)

[here](https://example.com/page)

[link](document.pdf)

[more](additional-info.html)

## Case insensitive violations

[CLICK HERE](https://example.com)

[Here](another-page.html)

[Link](some-document.pdf)

[MORE](extras.html)

## Punctuation and spacing variations

[click-here](https://example.com)

[click_here](page.html)

[click.here](document.pdf)

[click   here](spaced.html)

[  click here  ](padded.html)

## Reference link violations

[click here][ref1]

[here][ref2]

[link][ref3]

[more][ref4]

[ref1]: https://example.com
[ref2]: page.html
[ref3]: document.pdf
[ref4]: extras.html

## Collapsed reference link violations

[click here][]

[here][]

[link][]

[more][]

[click here]: https://example.com
[here]: page.html
[link]: document.pdf
[more]: extras.html

## Mixed content - violations and valid links

Here is a [good descriptive link](https://example.com) but also a [click here](bad-link.html) violation.

Multiple violations: [here](page1.html) and [more](page2.html) and [link](page3.html).

## Edge cases that should still be violations

[click here !](https://example.com)

[here???](question.html)

[   more   ](extra-spaces.html)