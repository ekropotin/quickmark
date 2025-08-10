# MD034 Comprehensive Test - Bare URL Detection

This file tests various edge cases and combinations for the MD034 rule.

## Basic Violations

Visit https://example.com for more info.
Email me at user@example.com.

## Valid Cases (Should Not Trigger)

Visit <https://example.com> for more info.
Email me at <user@example.com>.

## Code Spans (Should Not Trigger)

Use `https://example.com` in your code.
Email format: `user@example.com`.

## Markdown Links (Should Not Trigger)

Visit [our site](https://example.com) today.
Email [support](mailto:user@example.com).

## HTML Attributes (Should Not Trigger)

<a href="https://example.com">Link</a>
<a href='http://test.org'>Another link</a>

## Mixed Scenarios

Bare URL https://bare.com and proper <https://proper.com> link.
Bare email admin@bare.com and proper <admin@proper.com> address.

## Edge Cases

### URLs with Various Endings

Visit https://example.com. (period)
Visit https://example.com, (comma)
Visit https://example.com) (paren)
Visit https://example.com> (bracket - should not be included in URL)

### Emails with Various Endings

Contact user@example.com. (period)
Contact user@example.com, (comma)
Contact user@example.com) (paren)

### URLs in Different Punctuation Contexts

The site (https://example.com) is down.
Check https://example.com, then proceed.
Visit: https://example.com

### Complex URLs

https://example.com/path?param=value&other=123#section
http://user:pass@example.com:8080/path

### International Domains

Visit https://müller.example or email ünser@müller.example.

## Reference Links (Should Not Trigger)

[example]: https://example.com
[email]: mailto:user@example.com

Check the [example] site or [email] us.

## Nested Scenarios

### Valid nested (Should Not Trigger)
[link text with https://example.com in it](https://proper-target.com)

### Invalid nested (Should Trigger)
Links bind to the innermost [link that https://example.com link](https://target.com)

## Multiple Lines with Mixed Cases

Line 1: https://violation.com
Line 2: <https://valid.com>
Line 3: `https://code-span.com`
Line 4: another-violation@example.com
Line 5: <proper@example.com>

## Blockquotes

> Visit https://example.com for more info.
> Email support@example.com for help.
>
> Check <https://proper.com> for valid formatting.

## Lists

* Bare URL: https://example.com
* Proper URL: <https://example.com>
* Bare email: user@example.com
* Proper email: <user@example.com>

1. https://numbered-list.com
2. <https://proper-numbered.com>

## Tables

| Site | URL |
|------|-----|
| Bad | https://example.com |
| Good | <https://example.com> |

## Emphasis

**Bold text with https://example.com URL**
*Italic with user@example.com email*

**Bold with <https://proper.com> URL**
*Italic with <proper@example.com> email*