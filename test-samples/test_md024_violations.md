# MD024 Multiple Headings Violations

This file demonstrates violations of the MD024 rule (no-duplicate-heading).

# Introduction

This section introduces the document.

## Getting Started

How to begin using the software.

## Getting Started - VIOLATION: Duplicate content

This is a duplicate of the previous heading.

### Installation

Steps to install the software.

### Configuration

How to configure the software.

### Installation - VIOLATION: Duplicate content

This is a duplicate of a previous heading at the same level.

## Usage

How to use the software.

# Configuration - VIOLATION: Duplicate content

This heading has the same content as a previous heading but at a different level.

## Common Issues

List of common problems.

## FAQ

Frequently asked questions.

## Common Issues - VIOLATION: Duplicate content

Another duplicate heading.

# Mixed Styles Test

Testing mixed heading styles.

Mixed Styles Test
=================

VIOLATION: This setext heading has the same content as the ATX heading above.

## Section

Content here.

Section
-------

VIOLATION: This setext heading duplicates the ATX heading above.

### Subsection

Some content.

###   Subsection   - VIOLATION: Duplicate with whitespace

This should be detected as duplicate despite different whitespace.

## Empty

## Empty - VIOLATION: Duplicate empty-like heading

### 

###  - VIOLATION: Duplicate empty heading with spaces

All violations in this file should be detected by MD024 rule.