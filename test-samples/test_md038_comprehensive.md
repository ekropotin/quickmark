# MD038 Comprehensive Test Cases

This file contains a comprehensive mix of valid and invalid code spans for testing MD038 rule.

## Valid Cases

### Basic Valid Code Spans
Simple code spans without spaces: `code`, `function`, `variable`.

### Single Space Padding (Valid)
These are valid per CommonMark spec: ` code `, ` function `, ` variable `.

Required for backtick display: `` ` ``, `` `` ``, ``` ` ```.

### Code Spans with Only Whitespace (Valid)
These should be allowed: `   `, `	`, ` 	 `, `    `.

### Empty Code Spans (Valid)
Empty spans: ``, ````, `````.

### Multi-Backtick Valid
Double backticks: ``code``, ``function``.
Triple backticks: ```code```, ```function```.

## Invalid Cases

### Multiple Leading Spaces (Invalid)
These should trigger violations: `  code`, `   function`, `    variable`.

### Multiple Trailing Spaces (Invalid)
These should trigger violations: `code  `, `function   `, `variable    `.

### Both Leading and Trailing (Invalid)
These should trigger violations: `  code  `, `   function   `, `    variable    `.

### Tabs (Invalid)
Tabs should always be violations: `	code`, `code	`, `	code	`.

### Mixed Whitespace (Invalid)
These combinations are invalid: ` 	code`, `code	 `, ` 	code	 `.

### Multi-Backtick Violations
Double backticks: ``  code  ``, ``   function   ``.
Triple backticks: ```  code  ```, ```   function   ```.

## Context Testing

### In Lists
- Valid: `code` and ` code `
- Invalid: `  code  ` and `	code	`

### In Blockquotes
> Valid code spans: `code` and ` code `
> 
> Invalid code spans: `  code  ` and `	code	`

### In Emphasis
**Bold with valid `code` and invalid `  code  `**

*Italic with valid `code` and invalid `  code  `*

### In Links
[Valid link `code`](http://example.com)

[Invalid link `  code  `](http://example.com)

### Multiple on Same Line
Valid `code` and invalid `  code  ` and valid ` code ` mixed together.

## Edge Cases

### Boundary Conditions
Just inside limit: ` c ` (valid)
Just over limit: `  c  ` (invalid)

### Content with Spaces
Valid: `hello world`, ` hello world ` (single space padding)
Invalid: `  hello world  ` (multiple space padding)

### Special Characters
Valid: `$var`, `@user`, `#tag`
Invalid: `  $var  `, `	@user	`, `   #tag   `

### Complex Mixed
Line with: `valid` `  invalid  ` ` valid ` `	invalid	` `also valid`.