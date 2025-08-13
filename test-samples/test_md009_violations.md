# MD009 Trailing Spaces Violations

This line has one trailing space 
This line has three trailing spaces   
This line has four trailing spaces    

## Code blocks should be excluded

```rust
fn main() {
    println!("Code with trailing spaces");  
}
```

    // Indented code block with trailing spaces  

## Empty lines with spaces

Line before empty line
  
Line after empty line

## Mixed violations

Normal line without trailing spaces
Line with single space 
Line with two spaces (should be allowed)  
Line with five spaces     