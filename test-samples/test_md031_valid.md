# MD031 Valid Cases

These examples should not trigger MD031 violations.

## Properly spaced fenced code blocks

Some text before.

```javascript
const x = 1;
console.log(x);
```

More text after.

## Tilde fences with proper spacing

Another example.

~~~python
def hello():
    print("world")
~~~

End of example.

## Code block at document start

```bash
echo "This is at the start"
```

Regular text follows.

## Code block at document end

Some introductory text.

```json
{
    "name": "example",
    "version": "1.0.0"
}
```

## Multiple language examples

Text before first block.

```html
<div>HTML content</div>
```

Text between blocks.

```css
.example {
    color: blue;
}
```

Text after last block.

## Empty code blocks

Some text.

```
```

More text.

## Code blocks with info strings

Description here.

```javascript filename="example.js"
// This has an info string
const example = "test";
```

Final text.