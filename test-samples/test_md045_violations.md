# MD045 Violation Cases

## Markdown Images without Alt Text

![](image.jpg)

![](image.jpg "Title")

![Empty alt](image.jpg) and ![](inline-image.jpg) in text

Reference image without alt ![][ref]

## HTML Images without Alt Attribute

<img src="image.jpg" />

<img src="image.jpg" alt>

<IMG SRC="image.jpg" />

<img 
  src="image.jpg" 
  title="Title only" />

<p><img src="nested.jpg"></p>

## HTML Images with aria-hidden != "true"

<img src="image.jpg" aria-hidden="false" />

<img src="image.jpg" aria-hidden="" />

<img src="image.jpg" aria-hidden="other" />

## Images in Links

[![](no-alt.jpg)](link.html)

[<img src="no-alt.jpg" />](link.html)

[ref]: image.jpg