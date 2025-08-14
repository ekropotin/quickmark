# MD040 Violations Test Cases

This file contains examples of fenced code blocks that should trigger MD040 violations.

## Missing language specification

```
def hello():
    print("Hello, World!")
```

```
function greet() {
    console.log("Hello!");
}
```

## Mixed valid and invalid

```python
# This one is valid
print("Hello")
```

```
# This one is invalid - no language
echo "missing language"
```

## Different fence types without language

```
Some code here
```

~~~
More code without language
~~~

## Empty fenced blocks

```

```

~~~

~~~

## Complex cases

Regular text content.

```rust
// This is valid
fn main() {}
```

```
// This is invalid - no language specified
let x = 5;
```

```javascript
// This is valid
const y = 10;
```

```
// Another invalid one
SELECT * FROM table;
```

## Nested in lists

- Item 1
  ```
  echo "no language in list"
  ```

- Item 2
  ```bash
  echo "has language in list"
  ```

## In blockquotes

> This is a quote
> 
> ```
> code without language in quote
> ```
> 
> ```python
> print("code with language in quote")
> ```