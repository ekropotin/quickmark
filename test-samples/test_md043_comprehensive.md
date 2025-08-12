# Comprehensive Required Headings Test Cases

This file tests various complex scenarios for the MD043 rule.

## Scenario 1: Complex wildcard pattern

# Project Title

## Introduction

### Background

## Features

### Feature A

### Feature B

## Documentation

### API Reference

### Examples

## Conclusion

This tests a complex pattern with "*" wildcards allowing flexible content between required sections.

## Scenario 2: Question mark exactness

# Any Project Name

## Description

This project does something.

## Examples

Here are examples.

This tests the "?" wildcard which allows exactly one unspecified heading.

## Scenario 3: Mixed heading styles with requirements

Main Title
==========

Section One
-----------

### ATX Subsection

Section Two
-----------

### Another ATX Subsection

This tests required headings with mixed setext and ATX styles.

## Scenario 4: Case sensitivity edge cases

# Title

## Section

### subsection

## SECTION TWO

This tests various case combinations when case sensitivity is enabled.

## Scenario 5: Plus wildcard (one or more)

# Documentation

## Getting Started

### Installation

### Configuration

## Advanced Topics

### Performance

### Security

### Troubleshooting

## Conclusion

This tests the "+" wildcard requiring one or more unspecified headings.

## Scenario 6: Deeply nested structure

# Project

## Part I

### Chapter 1

#### Section A

##### Subsection 1

##### Subsection 2

#### Section B

### Chapter 2

## Part II

### Chapter 3

This tests deeply nested heading structures with requirements.

## Scenario 7: Empty and whitespace headings

# 

## Main Section

###   

This tests edge cases with empty or whitespace-only headings.

## Scenario 8: Special characters in headings

# Project: Advanced Features

## Section 1 (Important)

### Sub-section A.1

## Section 2 [Optional]

This tests headings containing special characters, punctuation, and formatting.

## Scenario 9: Long headings

# This is a very long heading that might be used in some documentation to describe a complex concept or feature in detail

## Another moderately long heading that explains something important

This tests how the rule handles longer heading texts.

## Scenario 10: Unicode and international characters

# プロジェクト

## Descripción

### Раздел

This tests headings with Unicode and international characters.