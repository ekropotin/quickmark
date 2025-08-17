# MD059 Comprehensive Test

This document contains both valid and invalid link text examples.

## Valid descriptive links

[Download the user manual](manual.pdf)
[View API documentation](api-docs.html)
[Contact our support team](mailto:support@example.com)
[Check the installation guide](install.html)
[Browse source code](https://github.com/example/repo)

## Invalid generic links (violations)

[click here](https://example.com)
[here](page.html)
[link](document.pdf)
[more](info.html)

## Mixed valid and invalid

This paragraph has a [detailed explanation](explanation.html) which is good,
but also has a [click here](bad.html) which is not descriptive.

## Images (should be ignored)

![click here](image1.jpg)
![here](image2.png)
![link](icon.svg)
![more](photo.gif)

## Links with code (should be allowed)

[`click here` function](api.html)
[Configuration `here`](config.html)
[The `link` method](methods.html)

## Reference links

### Valid reference links

[Complete user guide][guide]
[Technical specifications][specs]
[Contributing guidelines][contrib]

[guide]: user-guide.html
[specs]: technical-specs.html
[contrib]: contributing.md

### Invalid reference links (violations)

[click here][bad1]
[here][bad2]
[link][bad3]
[more][bad4]

[bad1]: https://example.com
[bad2]: page.html
[bad3]: doc.pdf
[bad4]: extra.html

## Collapsed reference links

### Valid collapsed reference links

[User documentation][]
[Developer resources][]

[User documentation]: user-docs.html
[Developer resources]: dev-resources.html

### Invalid collapsed reference links (violations)

[click here][]
[here][]
[link][]
[more][]

[click here]: https://example.com
[here]: page.html
[link]: doc.pdf
[more]: extra.html

## Edge cases

### Punctuation and spacing variations (all violations)

[click-here](page1.html)
[click_here](page2.html)
[click.here!](page3.html)
[  click here  ](page4.html)
[CLICK HERE](page5.html)

### Complex sentences with multiple links

You can [download the complete documentation](docs.html) or just [click here](summary.html) for a summary.
For more details, [see our comprehensive guide](guide.html) or [here](quick.html) for quick reference.

## Autolinks (should be ignored by this rule)

<https://example.com>
<mailto:test@example.com>

## HTML links (should be ignored by this rule)

<a href="https://example.com">click here</a>
<a href="page.html">here</a>