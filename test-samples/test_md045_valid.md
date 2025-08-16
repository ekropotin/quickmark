# MD045 Valid Cases

## Markdown Images with Alt Text

![Valid alt text](image.jpg)

![Another valid image](image.jpg "Title")

![Reference image with alt][ref]

Reference image with alt text ![Alt text reference][ref2]

## HTML Images with Alt Attributes

<img src="image.jpg" alt="Valid alt text" />

<img src="image.jpg" alt="Another valid" >

<IMG SRC="image.jpg" ALT="Case insensitive" />

<img 
  src="image.jpg" 
  alt="Multi-line" 
  />

## HTML Images with Empty Alt (Valid for Decorative Images)

<img src="image.jpg" alt="" />

<img src="image.jpg" alt='' />

## HTML Images with aria-hidden (Valid)

<img src="image.jpg" aria-hidden="true" />

<img src="image.jpg" ARIA-HIDDEN="TRUE" />

<img 
  src="image.jpg" 
  aria-hidden="true"
  />

<img src="image.jpg" aria-hidden='true' />

## Images in Code (Should Be Ignored)

```html
![](image.jpg)
<img src="image.jpg" />
```

    ![](indented-code.jpg)
    <img src="indented.jpg" />

`![](inline-code.jpg)` and `<img src="inline.jpg" />`

[ref]: image.jpg
[ref2]: image.jpg "Title"