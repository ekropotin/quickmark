# MD054 Comprehensive Test Cases

This file contains comprehensive examples of all link and image styles for testing MD054.

## Introduction

MD054 controls which styles of links and images are allowed in a document.
It supports configuration for autolinks, inline, full reference, collapsed reference, 
shortcut reference, and url_inline styles.

## All Link Styles

### Autolinks
Direct URLs wrapped in angle brackets:
- <https://example.com>
- <https://www.github.com>
- <https://docs.github.com/en/get-started>

### Inline Links
Text in brackets followed by URL in parentheses:
- [Example](https://example.com)
- [GitHub](https://github.com)
- [Documentation](https://docs.github.com/en/get-started)
- [Empty]()

### Full Reference Links  
Text in brackets followed by reference label in brackets:
- [Example Website][example]
- [GitHub Homepage][github]
- [GitHub Docs][docs]

### Collapsed Reference Links
Text in brackets followed by empty brackets:
- [Example Website][]
- [GitHub Homepage][]
- [GitHub Docs][]

### Shortcut Reference Links
Just text in brackets (relies on matching reference definition):
- [Example Website]
- [GitHub Homepage]
- [GitHub Docs]

### URL Inline Links
URL text that matches the URL destination:
- [https://example.com](https://example.com)
- [https://github.com](https://github.com)
- [https://docs.github.com](https://docs.github.com)

## All Image Styles

### Inline Images
Alt text in brackets with exclamation mark, followed by URL in parentheses:
- ![Example Logo](https://example.com/logo.png)
- ![GitHub Logo](https://github.com/logo.svg)
- ![Documentation Image](https://docs.github.com/image.jpg)
- ![Empty Image]()

### Full Reference Images
Alt text in brackets with exclamation mark, followed by reference label:
- ![Example Logo][example-logo]
- ![GitHub Logo][github-logo]  
- ![Documentation Image][docs-image]

### Collapsed Reference Images
Alt text in brackets with exclamation mark, followed by empty brackets:
- ![Example Logo][]
- ![GitHub Logo][]
- ![Documentation Image][]

### Shortcut Reference Images
Just alt text in brackets with exclamation mark:
- ![Example Logo]
- ![GitHub Logo]
- ![Documentation Image]

## Mixed Content

Here's a paragraph with multiple [inline links](https://example.com) and 
<https://autolinks.com> as well as ![inline images](https://example.com/img.jpg).

You can also have [reference links][ref] and ![reference images][img-ref] 
in the same paragraph.

## Complex Cases

### Links in Lists
1. [First link](https://first.com)
2. [Second link][second]
3. <https://third.com>
4. [Fourth link][]
5. [Fifth link]

### Images in Lists
1. ![First image](https://first.com/img.jpg)
2. ![Second image][second-img]
3. ![Third image][]
4. ![Fourth image]

### Links in Tables
| Site | Link |
|------|------|
| Example | [example.com](https://example.com) |
| GitHub | [GitHub][github] |
| Docs | <https://docs.github.com> |

### Nested Cases
Text with [a link to ![an image](https://example.com/nested.jpg) inside](https://example.com).

## Edge Cases

### URLs with Special Characters
- [Special URL](https://example.com/path?param=value&other=123#section)
- <https://example.com/path?param=value&other=123#section>
- [Special URL][special]

### Empty and Minimal Cases
- []()
- ![]()
- [text]
- ![alt]

### Similar Patterns (should not match)
- Code with `[brackets](in code)`
- Escaped \[brackets\]\(parentheses\)
- Text [with brackets] but no parentheses
- Text ![with image syntax] but no parentheses

## Reference Definitions

[example]: https://example.com "Example Website"
[github]: https://github.com "GitHub Homepage"
[docs]: https://docs.github.com/en/get-started "GitHub Documentation"
[Example Website]: https://example.com
[GitHub Homepage]: https://github.com
[GitHub Docs]: https://docs.github.com

[example-logo]: https://example.com/logo.png "Example Logo"
[github-logo]: https://github.com/logo.svg "GitHub Logo"  
[docs-image]: https://docs.github.com/image.jpg "Documentation Image"
[Example Logo]: https://example.com/logo.png
[GitHub Logo]: https://github.com/logo.svg
[Documentation Image]: https://docs.github.com/image.jpg

[ref]: https://example.com/reference
[img-ref]: https://example.com/image-reference.jpg
[second]: https://second.com
[second-img]: https://second.com/img.jpg
[special]: https://example.com/path?param=value&other=123#section