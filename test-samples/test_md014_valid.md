# MD014 Test Cases - Valid (No Violations)

## Command with output

```bash
$ git status
On branch main
nothing to commit
working tree clean
```

## Mixed commands and output

```bash
$ git status
On branch main
$ ls -la
total 8
drwxr-xr-x 2 user user 4096 Jan 1 00:00 .
```

## No dollar signs

```bash
git status
ls -la
pwd
```

## Mixed dollar signs (some lines without)

```bash
$ git status
ls -la
$ pwd
```

## Empty code block

```bash
```

## Blank lines only

```bash



```