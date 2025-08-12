# MD014 Comprehensive Test

## Variables (should not trigger)

```bash
$foo = 'bar'
$baz = 'qux'
```

## Mixed content with output (should not trigger)

```bash
$ ls
file1.txt file2.txt
$ git status
On branch main
$ cat file1.txt
content here
```

## No space after dollar (should not trigger - not a command)

```bash
$HOME/bin/script
$PATH variable
```

## All commands without output (should trigger)

```bash
$ mkdir test
$ cd test
$ ls
```

## Commands in indented block (should trigger)

Text before:

    $ command1
    $ command2

Text after.

## Whitespace variations (should trigger)

```bash
  $ command1
   $ command2
    $ command3
```

## Tab-indented commands (should trigger)

	$ command1
	$ command2

## Mixed tabs and spaces (should trigger if all have dollar)

```bash
	$ command1
    $ command2
```