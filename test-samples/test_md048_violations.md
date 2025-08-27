# MD048 Violation Examples

## Mixed style (inconsistent)

First block with backticks (establishes the consistent style):

```python
def hello_world():
    print("Hello, World!")
```

Second block with tildes (violates consistency):

~~~javascript
function greet() {
    console.log("Hello!");
}
~~~

Third block with backticks (matches first, so it's ok):

```rust
fn main() {
    println!("Hello!");
}
```

Fourth block with tildes (violates consistency again):

~~~go
package main

import "fmt"

func main() {
    fmt.Println("Hello!")
}
~~~

## Multiple violations

When multiple blocks violate the established style:

```text
First block - establishes backtick style
```

~~~text
First violation - tildes
~~~

```text
This is ok - matches established style
```

~~~text
Second violation - tildes again
~~~

~~~text
Third violation - more tildes
~~~

## Different fence lengths still count

```python
# Three backticks
```

~~~~python
# Four tildes - still a violation
~~~~

`````python
# Five backticks - ok, matches style
`````

~~~~~python
# Five tildes - violation
~~~~~