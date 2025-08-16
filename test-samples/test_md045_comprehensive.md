# MD045 Comprehensive Test

This file contains both valid and invalid cases for MD045 (no-alt-text) rule.

## Valid Cases

### Markdown Images with Alt Text

![Valid alt text](image.jpg)

![Another valid image](image.jpg "Title")

![Reference image with alt][ref-valid]

Reference image with alt text ![Alt text reference][ref2]

### HTML Images with Alt Attributes

<img src="image.jpg" alt="Valid alt text" />

<img src="image.jpg" alt="Another valid" >

<IMG SRC="image.jpg" ALT="Case insensitive" />

### Multiline HTML Images

<img 
  src="image.jpg" 
  alt="Multi-line valid" 
  />

### Empty Alt Text (Valid for Decorative)

<img src="decorative.jpg" alt="" />

<img src="decoration.jpg" alt='' />

### Images with aria-hidden

<img src="hidden.jpg" aria-hidden="true" />

<img src="hidden2.jpg" ARIA-HIDDEN="TRUE" />

### Images in Valid Links

[![Valid alt](image.jpg)](link.html)

[<img src="valid.jpg" alt="Valid" />](link.html)

## Invalid Cases

### Markdown Images without Alt Text

![](no-alt.jpg)

![](no-alt2.jpg "Title")

![Empty alt](image.jpg) and ![](inline-no-alt.jpg) in text

Reference image without alt ![][ref-invalid]

### HTML Images without Alt Attribute

<img src="no-alt.jpg" />

<img src="no-alt2.jpg" alt>

<IMG SRC="no-alt3.jpg" />

### Multiline HTML without Alt

<img 
  src="no-alt4.jpg" 
  title="Title only" />

### Nested HTML without Alt

<p><img src="nested-no-alt.jpg"></p>

### Images with aria-hidden != "true"

<img src="image.jpg" aria-hidden="false" />

<img src="image.jpg" aria-hidden="" />

<img src="image.jpg" aria-hidden="other" />

### Images in Links without Alt

[![](no-alt-link.jpg)](link.html)

[<img src="no-alt-link.jpg" />](link.html)

## Code Examples (Should Be Ignored)

```html
![](image.jpg)
<img src="image.jpg" />
```

    ![](indented-code.jpg)
    <img src="indented.jpg" />

Inline code: `![](inline-code.jpg)` and `<img src="inline.jpg" />`

Regular text with ![](actual-violation.jpg) should trigger.

[ref-valid]: image.jpg
[ref2]: image.jpg "Title"
[ref-invalid]: image.jpg