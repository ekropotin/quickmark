# MD044 Comprehensive Test

This document tests various scenarios for proper name capitalization.

## Basic Capitalization Issues

We use javascript instead of JavaScript for frontend development.
The github repository contains code, but GitHub is the company name.
Our project is built with typescript and not TypeScript.
We write css styles but should refer to CSS standards.
The html document uses HTML5 features.

## Mixed Case and Special Names

The github.com website is properly capitalized.
We use node.js for backend development (Node.js should be correct).
Both typescript and TYPESCRIPT are incorrect - only TypeScript is right.

## Code Blocks

Here's some JavaScript code:

```javascript
console.log("This is javascript in a code block");
const github = "This should be flagged if code_blocks is true";
```

Inline code with `javascript` and `github` issues.

## HTML Elements

<p>This paragraph mentions javascript and github in HTML.</p>
<div class="github">github styling class</div>

## Edge Cases

### Word Boundaries
The javascriptish language is not javascript.
A githubuser might use github for projects.

### Overlapping Names
We prefer GitHub over git, but github.com is the correct URL.
The github.com site uses GitHub branding consistently.

### Punctuation
Using JavaScript! TypeScript? CSS. HTML, etc.
"JavaScript" and 'TypeScript' with quotes.
JavaScript's features and TypeScript's benefits.

## Valid Examples (These should not be flagged)

JavaScript is a programming language.
GitHub is a code hosting platform.
TypeScript adds types to JavaScript.
CSS styles web pages.
HTML structures content.
Node.js runs JavaScript on servers.
The github.com website is accessible.