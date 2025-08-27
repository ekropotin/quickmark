# MD031 Comprehensive Test Cases

This file includes various scenarios for MD031 testing.

## Basic valid cases

Text with proper spacing.

```javascript
const example = "valid";
```

More text with proper spacing.

## Basic violation cases

Text without spacing.
```python
print("violation")
```
No spacing after.

## Code blocks in lists (default behavior - should have violations)

1. First item
   ```javascript
   const x = 1;
   ```
2. Second item

- List item one
  ```bash
  echo "test"
  ```
- List item two

## Code blocks in blockquotes

> Some quoted text
> ```css
> .example { color: blue; }
> ```
> More quoted text

## Nested structures

> 1. Quoted list item
>    ```html
>    <div>content</div>
>    ```
> 2. Another item

## Multiple consecutive code blocks

Text before.

```javascript
const a = 1;
```

```python
b = 2
```

```bash
echo "c"
```

Text after.

## Code blocks with different info strings

Regular text.

```javascript title="example.js" highlight="1,3"
const x = 1;
const y = 2;
const z = 3;
```

More text.

## Empty and minimal code blocks

Text before empty block.

```
```

Text between.

```text
single line
```

Text after.

## Document boundaries

```
Start of document
```

Middle content.

```
End of document
```