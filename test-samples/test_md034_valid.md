# MD034 Valid Cases - No Bare URLs

This file contains examples that should NOT trigger MD034 violations.

## Proper Angle Bracket URLs

Visit <https://example.com> for more information.

Check out <http://test.org> as well.

## Proper Angle Bracket Emails

Contact us at <user@example.com> for support.

Email <admin@test.org> with questions.

## URLs in Code Spans

Not a clickable link: `https://example.com`

Code example: `http://test.org/path`

## Emails in Code Spans

Example email format: `user@example.com`

Template: `name@domain.com`

## URLs in Markdown Links

Visit [our website](https://example.com) for details.

Check out [this link](http://test.org/page).

## URLs in HTML Attributes

<a href='https://example.com'>External link</a>

<img src="https://example.com/image.jpg" alt="Example">

## Reference Links

[example]: https://example.com

This is a [reference link][example].

## Autolink Already Formatted

These are already properly formatted:
- <https://example.com/path?query=value>
- <mailto:user@example.com>
- <http://localhost:8080>

## URLs in Fenced Code Blocks

```bash
curl https://example.com/api
wget http://test.org/file.txt
```

## URLs in Indented Code Blocks

    GET https://api.example.com/users
    POST http://test.org/submit

## Mixed Valid Examples

Visit <https://example.com> or check the code: `https://github.com/user/repo`.

Email <support@example.com> for help with `user@domain.com` format.