# MD059 Valid Link Text Examples

These link texts are descriptive and should not trigger violations:

[Download the budget document](https://example.com/budget.pdf)

[CommonMark Specification](https://commonmark.org)

[View the full report](report.html)

[Contact us via email](mailto:contact@example.com)

[Installation instructions](./docs/install.md)

[API documentation](api-docs.html)

[Submit a bug report](https://github.com/example/repo/issues/new)

[Source code on GitHub](https://github.com/example/repo)

## Images should be ignored

![click here](image.jpg)
![here](another-image.png)
![link](icon.svg)

## Links with code or HTML should be allowed

[`click here`](https://example.com)
[<code>here</code>](https://example.com)
[Configuration for `click here`](config.html)

## Reference links with descriptive text

[Download the user manual][manual]
[View technical documentation][docs]
[See installation guide][install]

[manual]: manual.pdf
[docs]: https://docs.example.com
[install]: ./install.md

## Collapsed reference links

[User documentation][]
[Development guide][]

[User documentation]: user-docs.html
[Development guide]: dev-guide.html

## Complex descriptive text

[Learn about advanced configuration options](config-advanced.html)
[Troubleshoot common installation issues](troubleshooting.html)
[Submit feature requests on our forum](https://forum.example.com)