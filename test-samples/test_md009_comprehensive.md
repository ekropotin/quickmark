# MD009 Trailing Spaces Comprehensive Test

## Basic trailing spaces

Line with no trailing spaces
Line with one space (should violate) 
Line with two spaces (default allowed)  
Line with three spaces (should violate)   
Line with four spaces (should violate)    

## Empty lines

Normal line

Empty line above (clean)
  
Empty line above with spaces (should violate)

## Code blocks (should be excluded)

```javascript
function example() {
    // Code can have trailing spaces  
    let x = "test";    
    return x;  
}
```

Indented code block:

    def python_func():
        # Trailing spaces allowed in code  
        return "hello world"    

## Lists (complex behavior)

- List item 1
- List item 2

  - Nested item
  
  - Another nested item

1. Ordered list
2. Second item

   Text in list
   
   More text

## Mixed content

Regular paragraph without trailing spaces.

Paragraph with line break  
Continuing on next line.

Single space violation 
Two spaces allowed  
Three space violation   

Final paragraph.