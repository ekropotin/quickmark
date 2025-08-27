# MD037 Valid Cases - No spaces inside emphasis markers

This document contains valid emphasis that should NOT trigger MD037 violations.

## Single asterisk emphasis
This has *valid emphasis* text.
Multiple *emphasis* in *one* line are fine.
Text with *proper* emphasis and **strong** text.

## Double asterisk (strong) emphasis
This has **valid strong** text.
Multiple **strong** in **one** line are fine.
Text with **proper** strong and *emphasis* text.

## Triple asterisk (strong + emphasis)
This has ***valid strong emphasis*** text.
Multiple ***strong emphasis*** in ***one*** line are fine.

## Single underscore emphasis
This has _valid emphasis_ text.
Multiple _emphasis_ in _one_ line are fine.
Text with _proper_ emphasis and __strong__ text.

## Double underscore (strong) emphasis
This has __valid strong__ text.
Multiple __strong__ in __one__ line are fine.
Text with __proper__ strong and _emphasis_ text.

## Triple underscore (strong + emphasis)
This has ___valid strong emphasis___ text.
Multiple ___strong emphasis___ in ___one___ line are fine.

## Mixed valid emphasis
Text with *asterisk* and _underscore_ emphasis.
Text with **double asterisk** and __double underscore__ strong.
Text with ***triple asterisk*** and ___triple underscore___ strong emphasis.

## Code blocks (should be ignored)
```markdown
* This should not trigger * any violations in code blocks.
** Neither should this ** in code blocks.
_ Nor this _ in code blocks.
__ Or this __ in code blocks.
```

## Code spans (should be ignored)
Regular text with `* invalid * code spans` should not trigger violations.
Also `** invalid **` and `_ invalid _` and `__ invalid __` in code spans.
Multiple code spans: `* one *` and `** two **` should be fine.

## Inline code with emphasis outside
This `code` has *proper* emphasis outside.
This `code` has **proper** strong outside.

## Links and other inline elements
[Link text](https://example.com) with *emphasis* after.
![Alt text](image.jpg) with **strong** after.

## Edge cases that are valid
Text*with*no*spaces is not emphasis (just text with asterisks).
Text_with_under_scores is not emphasis (just text with underscores).
URLs like https://example.com/path_with_underscores are fine.
Email addresses like user_name@example.com are fine.

## Escaped emphasis markers
This has \*escaped asterisks\* that are not emphasis.
This has \_escaped underscores\_ that are not emphasis.

## Emphasis at start/end of lines
*Emphasis* at the start of a line.
Line ends with *emphasis*.

## Nested valid emphasis
This has *emphasis with **strong** inside* it.
This has **strong with *emphasis* inside** it.