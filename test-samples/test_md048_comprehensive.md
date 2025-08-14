# MD048 Comprehensive Test Cases

## Section 1: Valid consistent usage

### All backticks

Text before first block.

```python
def example():
    return "backticks"
```

Text between blocks.

```javascript
// Another backtick block
console.log("consistent");
```

Text between blocks.

```text
Plain text block
with multiple lines
```

### All tildes

Text before first block.

~~~python
def example():
    return "tildes"
~~~

Text between blocks.

~~~javascript
// Another tilde block
console.log("consistent");
~~~

Text between blocks.

~~~text
Plain text block
with multiple lines
~~~

## Section 2: Single blocks (always valid)

### Single backtick block

```single
This is the only fenced block
```

### Single tilde block

~~~single
This is the only fenced block
~~~

## Section 3: Mixed with indented (indented blocks ignored)

```fenced
This is fenced
```

    This is indented
    And ignored by MD048

```fenced
Another fenced block
```

## Section 4: Violation cases

### Mixed styles (violations)

First establishes backtick style:

```established
Backtick style established
```

This violates the established style:

~~~violation
Tilde block violates consistency
~~~

Back to established style (ok):

```ok
This matches the established style
```

Another violation:

~~~violation2
Another tilde violation
~~~

### Complex violation patterns

```start
Starting with backticks
```

Text between.

~~~v1
First violation (tilde)
~~~

More text.

```ok1
This is ok (backtick)
```

Even more text.

~~~v2
Second violation (tilde)
~~~

Final text.

```ok2
Final ok block (backtick)
```

## Section 5: Edge cases

### Different fence lengths

```three
Three backticks
```

~~~~four-tildes
Four tildes - violation
~~~~

`````five-backticks
Five backticks - ok
`````

~~~~~~six-tildes
Six tildes - violation
~~~~~~

### With language specifiers

```python
# Python with backticks
print("hello")
```

~~~javascript
// JavaScript with tildes - violation
console.log("hello");
~~~

```rust
// Rust with backticks - ok
println!("hello");
```

### Empty code blocks

```
Empty backtick block
```

~~~
Empty tilde block - violation
~~~

### Nested in lists

1. First item

   ```code
   Backtick in list
   ```

2. Second item

   ~~~code
   Tilde in list - violation
   ~~~

3. Third item

   ```code
   Back to backticks - ok
   ```

## Section 6: No violations expected

### No fenced blocks

Just regular text with no code blocks.

Only headings and paragraphs here.

### Only indented blocks

    function indented() {
        return "not affected";
    }

More text.

    another_indented_block = True

### Mixed indented and single fenced

    def indented():
        pass

```single
Only one fenced block
```

    more_indented = "code"