# MD040 Comprehensive Test Cases

This file contains comprehensive test cases for the MD040 rule covering various scenarios including configuration options.

## Basic cases

### Valid with languages

```rust
fn main() {
    println!("Hello, World!");
}
```

```python
def hello():
    print("Hello")
```

### Invalid without languages

```
const x = 5;
```

```
print("no language")
```

## Configuration testing

### Language restrictions (for testing allowed_languages config)

```rust
// Should be allowed when rust is in allowed_languages
fn test() {}
```

```python
# Should be allowed when python is in allowed_languages
def test(): pass
```

```javascript
// Should be disallowed when not in allowed_languages
function test() {}
```

```go
// Should be disallowed when not in allowed_languages
func test() {}
```

### Language-only mode (for testing language_only config)

```rust
// Valid - language only
fn main() {}
```

```python {.line-numbers}
# Invalid - has extra info when language_only is true
def main(): pass
```

```javascript copy
// Invalid - has extra info when language_only is true
function main() {}
```

```html class="highlight"
<!-- Invalid - has extra info when language_only is true -->
<p>test</p>
```

## Advanced syntax

### Language with attributes (should extract language correctly)

```rust{.line-numbers}
fn main() {
    println!("Hello");
}
```

```python {copy}
print("Hello")
```

### Different fence styles

~~~rust
fn tilde_fence() {}
~~~

~~~
// Tilde fence without language
let x = 5;
~~~

```bash
echo "backtick fence"
```

```
echo "backtick fence without language"
```

## Edge cases

### Empty blocks

```rust

```

```

```

### Minimal content

```c
int main() { return 0; }
```

```
return 0;
```

### Complex scenarios

Here's a code example:

```python
# Valid Python code
def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

# Test the function
print(factorial(5))
```

And here's invalid code:

```
// Missing language specification
function factorial(n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}
```

### Mixed with other markdown

- List item with valid code:
  ```yaml
  name: test
  version: 1.0
  ```

- List item with invalid code:
  ```
  name: test
  version: 1.0
  ```

> Blockquote with valid code:
> 
> ```json
> {"status": "ok"}
> ```
> 
> And invalid code:
> 
> ```
> {"status": "error"}
> ```

## Language case sensitivity

```Rust
// Capital R in Rust
fn test() {}
```

```PYTHON
# All caps Python
def test(): pass
```

```html
<!-- lowercase html -->
<p>test</p>
```

```HTML
<!-- uppercase HTML -->
<p>test</p>
```

## Special characters in language

```c++
// C++ with plus signs
int main() { return 0; }
```

```objective-c
// Objective-C with hyphen
int main() { return 0; }
```

```f#
// F# with hash
let x = 5
```

## No language specified - all violations

```
def python_code():
    return "no language"
```

~~~
function js_code() {
    return "no language";
}
~~~

```
SELECT * FROM users;
```

```
body {
    margin: 0;
}
```