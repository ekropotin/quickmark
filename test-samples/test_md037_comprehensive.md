# MD037 Comprehensive Test - Spaces inside emphasis markers

This document contains a comprehensive set of test cases for MD037, including both valid and invalid emphasis.

## Valid emphasis (should not trigger violations)

### Basic valid emphasis
This has *valid emphasis* text.
This has **valid strong** text.
This has ***valid strong emphasis*** text.
This has _valid emphasis_ text.
This has __valid strong__ text.
This has ___valid strong emphasis___ text.

### Multiple valid emphasis on same line
Text with *first* and *second* emphasis.
Text with **first** and **second** strong.
Text with ***first*** and ***second*** strong emphasis.

### Mixed asterisk and underscore (valid)
Text with *asterisk* and _underscore_ emphasis.
Text with **asterisk strong** and __underscore strong__.
Text with ***asterisk strong emphasis*** and ___underscore strong emphasis___.

### Valid emphasis in different contexts
Normal paragraph with *emphasis*.

- List item with *emphasis*
- Another item with **strong**

> Blockquote with *emphasis*
> And **strong** text

| Table | With |
|-------|------|
| *emphasis* | **strong** |

### Code contexts (should be ignored - valid)
```markdown
* This should be ignored * in code blocks
** And this should be ignored ** too
```

Inline `* code spans * should be ignored` completely.

## Invalid emphasis (should trigger violations)

### Basic invalid emphasis with spaces
This has * invalid emphasis * with spaces.
This has ** invalid strong ** with spaces.
This has *** invalid strong emphasis *** with spaces.
This has _ invalid emphasis _ with spaces.
This has __ invalid strong __ with spaces.
This has ___ invalid strong emphasis ___ with spaces.

### One-sided space violations
Text with * space after* marker.
Text with *space before * marker.
Text with ** space after** marker.
Text with **space before ** marker.
Text with *** space after*** marker.
Text with ***space before *** marker.

Text with _ space after_ marker.
Text with _space before _ marker.
Text with __ space after__ marker.
Text with __space before __ marker.
Text with ___ space after___ marker.
Text with ___space before ___ marker.

### Multiple spaces
Text with *  multiple  spaces  * inside.
Text with **  multiple  spaces  ** inside.
Text with ***  multiple  spaces  *** inside.
Text with _  multiple  spaces  _ inside.
Text with __  multiple  spaces  __ inside.
Text with ___  multiple  spaces  ___ inside.

### Tab characters (whitespace violations)
Text with *	tab	* characters.
Text with **	tab	** characters.
Text with _	tab	_ characters.
Text with __	tab	__ characters.

### Mixed violations and valid
Mix of *valid* and * invalid * on same line.
Mix of **valid** and ** invalid ** strong on same line.
Mix of _valid_ and _ invalid _ emphasis on same line.
Mix of __valid__ and __ invalid __ strong on same line.

### Violations in different contexts
Paragraph with * spaces * in emphasis.

- List item with * spaces * in emphasis
- Another item with ** spaces ** in strong

> Blockquote with * spaces * in emphasis
> And ** spaces ** in strong text

| Table | With |
|-------|------|
| * spaces * | ** spaces ** |

### Edge cases

#### Mismatched markers (should not trigger - invalid emphasis syntax)
This * asterisk and _ underscore don't match.
This ** double and * single don't match.
This *** triple and ** double don't match.

#### Empty or minimal content
This has * * just spaces (violation).
This has ** ** just spaces (violation).
This has _ _ just spaces (violation).
This has __ __ just spaces (violation).

This has *a* single character (valid).
This has **a** single character (valid).
This has _a_ single character (valid).
This has __a__ single character (valid).

#### At line boundaries
* Space after* at line start.
Line ending with *space before *.

** Space after** at line start.
Line ending with **space before **.

_ Space after_ at line start.
Line ending with _space before _.

__ Space after__ at line start.
Line ending with __space before __.

## Nested and complex cases

### Valid nested emphasis
This has *emphasis with **strong** inside* it.
This has **strong with *emphasis* inside** it.
This has _emphasis with __strong__ inside_ it.
This has __strong with _emphasis_ inside__ it.

### Invalid nested emphasis (spaces in outer)
This has * emphasis with **strong** inside * it (violation in outer).
This has ** strong with *emphasis* inside ** it (violation in outer).
This has _ emphasis with __strong__ inside _ it (violation in outer).
This has __ strong with _emphasis_ inside __ it (violation in outer).

### Complex text with multiple violations
A paragraph with * first violation * and *valid* and ** second violation ** text.
Another line with _ third violation _ and __valid__ and ___ fourth violation ___ text.

### Real-world-like content
This is a *real* document with **important** information.
However, this has * formatting errors * that need fixing.
The ** proper way ** should be like this: **proper way**.
Similarly, _ formatting errors _ should be _formatting errors_.

## Special characters and escapes

### Valid escaped markers
This has \*escaped asterisks\* (not emphasis).
This has \_escaped underscores\_ (not emphasis).

### Invalid emphasis with special characters
This has * emphasis with punctuation! * violation.
This has ** strong with numbers 123 ** violation.
This has _ emphasis with symbols @#$ _ violation.
This has __ strong with emoji 😀 __ violation.

## Links and other inline elements

### Valid emphasis with links
This has *emphasis* and [a link](https://example.com).
This has **strong** and [another link](https://example.com).

### Invalid emphasis with links
This has * emphasis * and [a link](https://example.com).
This has ** strong ** and [another link](https://example.com).

### Links containing emphasis-like text (should be ignored)
[This * link text * should be ignored](https://example.com)
[This ** link text ** should be ignored](https://example.com)

## Performance test cases
Multiple * violations * on * the * same * line * with * many * instances.
Similarly ** multiple ** strong ** violations ** on ** same ** line.
And _ multiple _ underscore _ violations _ on _ the _ same _ line.
Plus __ multiple __ underscore __ strong __ violations __ on __ same __ line.

## Unicode and international text
This has * международный текст * with violations.
This has ** 国际文本 ** with violations.
This has _ النص الدولي _ with violations.
This has __ διεθνές κείμενο __ with violations.