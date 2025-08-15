# MD037 Violations - Spaces inside emphasis markers

This document contains emphasis with spaces that SHOULD trigger MD037 violations.

## Single asterisk emphasis violations
This has * invalid emphasis * with spaces inside.
Multiple * invalid * in * one * line have violations.
Text with * spaces * mixed with *valid* emphasis.

## Double asterisk (strong) emphasis violations
This has ** invalid strong ** with spaces inside.
Multiple ** invalid ** in ** one ** line have violations.
Text with ** spaces ** mixed with **valid** strong.

## Triple asterisk (strong + emphasis) violations
This has *** invalid strong emphasis *** with spaces inside.
Multiple *** invalid *** in *** one *** line have violations.
Text with *** spaces *** mixed with ***valid*** strong emphasis.

## Single underscore emphasis violations
This has _ invalid emphasis _ with spaces inside.
Multiple _ invalid _ in _ one _ line have violations.
Text with _ spaces _ mixed with _valid_ emphasis.

## Double underscore (strong) emphasis violations
This has __ invalid strong __ with spaces inside.
Multiple __ invalid __ in __ one __ line have violations.
Text with __ spaces __ mixed with __valid__ strong.

## Triple underscore (strong + emphasis) violations
This has ___ invalid strong emphasis ___ with spaces inside.
Multiple ___ invalid ___ in ___ one ___ line have violations.
Text with ___ spaces ___ mixed with ___valid___ strong emphasis.

## One-sided space violations
Text with * space after asterisk* marker.
Text with *space before asterisk * marker.
Text with ** space after double* marker (mismatched).
Text with *space before double ** marker (mismatched).

Text with _ space after underscore_ marker.
Text with _space before underscore _ marker.
Text with __ space after double_ marker (mismatched).
Text with _space before double __ marker (mismatched).

## Mixed violations and valid
Mix of *valid* and * invalid * emphasis.
Also **valid** and ** invalid ** strong.
And _valid_ with _ invalid _ emphasis.
Plus __valid__ with __ invalid __ strong.

## Multiple spaces
This has *  multiple  spaces  * inside emphasis.
This has **  multiple  spaces  ** inside strong.
This has _  multiple  spaces  _ inside emphasis.
This has __  multiple  spaces  __ inside strong.

## Leading and trailing spaces only
Text with * leading space* violation.
Text with *trailing space * violation.
Text with ** leading space** violation.
Text with **trailing space ** violation.
Text with _ leading space_ violation.
Text with _trailing space _ violation.
Text with __ leading space__ violation.
Text with __trailing space __ violation.

## Violations at start/end of lines
* Leading space* at start of line.
Line ends with *trailing space *.
** Strong leading space** at start.
Line ends with **strong trailing space **.

## Mixed markers (should not trigger - different types)
This * asterisk and _ underscore should not match.
This ** double asterisk and __ double underscore should not match.
This * single and ** double should not match.

## Tab characters (should also be violations)
This has *	tab character	* inside emphasis.
This has **	tab character	** inside strong emphasis.
This has _	tab character	_ inside emphasis.
This has __	tab character	__ inside strong emphasis.