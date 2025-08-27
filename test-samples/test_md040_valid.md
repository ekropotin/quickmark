# Valid MD040 Test Cases

This file contains examples of valid fenced code blocks that should not trigger MD040 violations.

## Basic language specifications

```rust
fn main() {
    println!("Hello, World!");
}
```

```javascript
console.log('Hello, World!');
```

```python
def hello():
    print("Hello, World!")
```

## Different fence markers

```bash
echo "Using backticks"
```

~~~shell
echo "Using tildes"
~~~

## Various languages

```html
<p>HTML example</p>
```

```css
body {
    margin: 0;
}
```

```sql
SELECT * FROM users WHERE active = 1;
```

```json
{
  "name": "example",
  "version": "1.0.0"
}
```

## Text content

```text
This is plain text content without syntax highlighting.
```

```text
Some content
```

## Mixed content with indented code blocks

Regular markdown content.

    This is an indented code block
    It should not trigger MD040 violations

More content with fenced blocks:

```yaml
name: test
value: 123
```

## Edge cases

```c
#include <stdio.h>
int main() { return 0; }
```

```xml
<?xml version="1.0" encoding="UTF-8"?>
<root>
  <item>content</item>
</root>
```