# MD044 Violations Test

This document contains improper capitalization that should trigger violations.

## Incorrect Capitalization

We use javascript for frontend development.
The github platform hosts repositories.
Our code is written in typescript.
We style pages with css.
The document uses html markup.
Server applications run on node.js.
This project uses quickmark for linting.

## All Uppercase

JAVASCRIPT is a programming language.
GITHUB hosts code repositories.
TYPESCRIPT extends JavaScript.
CSS styles web pages.
HTML structures content.
NODE.JS runs server code.
QUICKMARK lints Markdown files.

## Mixed Case Issues

Javascript should be JavaScript.
Github should be GitHub.
Typescript should be TypeScript.
Html should be HTML.
Css should be CSS.
Nodejs should be Node.js.
Quickmark should be QuickMark.

## In Code Blocks

```javascript
// This contains incorrect names if code_blocks is true
console.log("Using javascript instead of JavaScript");
const github = "Should be GitHub";
let typescript = "Should be TypeScript";
```

Inline `javascript` and `github` violations.

## In HTML

<p>We use javascript and github for development.</p>
<div class="typescript">typescript styling</div>
<span>css and html elements</span>

## Repeated Violations

The javascript language and javascript frameworks.
Both github and GITHUB are incorrect.
Using typescript, TYPESCRIPT, and Typescript.