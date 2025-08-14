# MD048 Valid Examples

## Consistent style (all backticks)

Some text with a code block:

```python
def hello_world():
    print("Hello, World!")
```

Another code block:

```javascript
function greet() {
    console.log("Hello!");
}
```

## Consistent style (all tildes)

Some text with a code block:

~~~python
def hello_world():
    print("Hello, World!")
~~~

Another code block:

~~~javascript
function greet() {
    console.log("Hello!");
}
~~~

## Single code block (any style is valid)

Just one block with backticks:

```text
This is fine
```

Just one block with tildes:

~~~text
This is also fine
~~~

## No code blocks

Just regular text without any fenced code blocks.

Regular paragraphs and headings.

### Indented code blocks are not affected

    This is indented code
    It doesn't count for MD048

More indented code:

    console.log("This is ignored");
    var x = 42;