# Valid Link Examples (MD042)

These examples should NOT trigger MD042 violations.

## Normal links with URLs

[Valid link](https://example.com)

[Another link](http://test.org/path)

[Secure link](https://secure.example.com/path?query=value)

[Link with fragment](https://example.com#section)

## Links with meaningful fragments

[Go to section](#introduction)

[Navigate to conclusion](#conclusion-section)

[Link to subsection](#sub-section-2-1)

## Reference links with definitions

[Reference link][ref1]

[Another reference][ref2]

[Shortcut reference link][]

[ref1]: https://example.com
[ref2]: https://another-site.org
[Shortcut reference link]: https://shortcut.example.com

## Links with various schemes

[FTP link](ftp://files.example.com)

[Mailto link](mailto:user@example.com)

[File link](file:///path/to/file.txt)

[Custom scheme](custom://protocol/path)

## Links with titles

[Link with title](https://example.com "This is a title")

[Another titled link](https://example.com 'Single quoted title')

## Images (should not be affected by this rule)

![Empty image]()

![Image with fragment](#)

![Normal image](image.jpg)

## Links in different contexts

Here is a [valid inline link](https://example.com) in text.

- [Link in list item](https://example.com)
- Another [link here](https://test.org)

> [Link in blockquote](https://example.com)

| Link | URL |
|------|-----|
| [Table link](https://example.com) | https://example.com |

`[Not a real link](https://example.com)` - this is in code

## Complex valid cases

[Link with query](https://example.com?param=value&other=123)

[Link with port](https://example.com:8080/path)

[Link with username](https://user@example.com/path)

[Unicode link](https://example.com/ünïcödé)

[Very long URL](https://example.com/very/long/path/with/many/segments/and/parameters?param1=value1&param2=value2&param3=value3#section)