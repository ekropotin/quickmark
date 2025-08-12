# MD014 Test Cases - Violations

## Fenced code block with all dollar signs

```bash
$ git status
$ ls -la
$ pwd
```

## Indented code block with all dollar signs

Some text:

    $ git status
    $ ls -la
    $ pwd

More text.

## Fenced with whitespace before dollar

```sh
  $ git status
  $ ls -la
  $ pwd
```

## With blank lines between commands

```bash
$ git status

$ ls -la

$ pwd
```