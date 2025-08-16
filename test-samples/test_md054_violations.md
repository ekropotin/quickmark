# MD054 Violations Test Cases

This file contains examples of various link and image style violations.
Configure MD054 to disallow specific styles to see violations.

## Autolinks (disallowed when autolink=false)
<https://example.com>
<https://github.com/DavidAnson/markdownlint>

## Inline Links (disallowed when inline=false)
[Link text](https://example.com)
[GitHub](https://github.com)

## Inline Images (disallowed when inline=false)
![Alt text](https://example.com/image.jpg)
![GitHub logo](https://github.com/logo.png)

## Full Reference Links (disallowed when full=false)
[Link text][ref1]
[Another link][ref2]

## Full Reference Images (disallowed when full=false)
![Alt text][img1]
![Another image][img2]

## Collapsed Reference Links (disallowed when collapsed=false)
[example.com][]
[GitHub][]

## Collapsed Reference Images (disallowed when collapsed=false)
![example logo][]
![GitHub logo][]

## Shortcut Reference Links (disallowed when shortcut=false)
[example.com]
[GitHub]

## Shortcut Reference Images (disallowed when shortcut=false)
![example logo]
![GitHub logo]

## URL Inline Links (disallowed when url_inline=false)
[https://example.com](https://example.com)
[https://github.com](https://github.com)

## Reference Definitions
[ref1]: https://example.com
[ref2]: https://github.com
[img1]: https://example.com/image.jpg
[img2]: https://github.com/logo.png
[example.com]: https://example.com
[GitHub]: https://github.com
[example logo]: https://example.com/logo.png
[GitHub logo]: https://github.com/logo.png