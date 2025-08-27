# MD031 Violation Cases

These examples should trigger MD031 violations.

## Missing blank line before code block
Some text immediately before.
```javascript
const x = 1;
```

More text after.

## Missing blank line after code block

Some text before.

```python
print("hello")
```
Text immediately after.

## Missing both blank lines
Text without spacing.
```bash
echo "no spacing"
```
More text without spacing.

## Tilde fences with violations
Some text.
~~~css
.example { color: red; }
~~~
No spacing after.

## Multiple violations in document
First text.
```html
<div>content</div>
```
No space before next.
```json
{"key": "value"}
```
No space after either.

## Mixed fence types
Text before.
```javascript
const a = 1;
```
No space.
~~~python
print("test")
~~~
Final text.