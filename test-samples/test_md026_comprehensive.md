# MD026 Comprehensive Test Cases

This file contains both valid and invalid examples for comprehensive testing.

## Valid Cases (Should NOT trigger violations)

# Good heading without punctuation

## Another good heading

### FAQ: What is markdown?

#### How do I use setext headings?

##### When should I use ATX closed style?

###### Deep nesting is fine

Good setext heading
===================

Another good setext
-------------------

## ATX Closed Style Valid

# Closed style without punctuation #

## Another closed style ##

### Third level closed ###

## HTML Entities (Should be ignored)

# Copyright &copy; 2023

## Trademark &reg; mark

### Numeric entity &#169; test

#### Hex entity &#x000A9; test

##### Mixed &copy; &#174; &#x2122; entities

## Question Marks Allowed by Default

# What is this document?

## How does this work?

### When should I use this?

#### Why is this important?

Why is this a setext heading?
=============================

How does setext work?
---------------------

## Violation Cases (SHOULD trigger violations)

# This heading has a period.

## This heading has an exclamation!

### This heading has a comma,

#### This heading has a semicolon;

##### This heading has a colon:

###### Multiple punctuation...

## ATX Closed Style Violations

# Closed with period. #

## Closed with exclamation! ##

### Closed with comma, ###

## Setext Violations

This setext has a period.
=========================

This setext has exclamation!
-----------------------------

This setext has comma,
======================

This setext has semicolon;
---------------------------

This setext has colon:
======================

## Full-Width Punctuation Violations

# Full-width period。

## Full-width comma，

### Full-width semicolon；

#### Full-width colon：

##### Full-width exclamation！

## Edge Cases and Complex Punctuation

# Multiple periods...

## Multiple exclamations!!!

### Mixed punctuation.,;:!

#### Punctuation with space .

##### Complex sentence with ending.

###### Technical notation (v1.0).

## More Valid Cases

# Perfectly fine heading

## Another perfectly fine heading

### Questions are allowed?

#### More questions work too?

##### FAQ entries work?

Valid setext example
====================

Another valid example
---------------------

## More Invalid Cases

# Simple violation.

## Another violation!

### Yet another violation,

#### Semicolon violation;

##### Colon violation:

Simple setext violation.
========================

Another setext violation!
-------------------------

Final setext violation,
=======================

Last setext violation;
----------------------

Ultimate setext violation:
==========================