# MD024 Comprehensive Test Cases

This file demonstrates various scenarios for the MD024 rule configuration options.

## Default Configuration Tests

With default settings (siblings_only=false, allow_different_nesting=false), 
any duplicate heading content is a violation.

# Chapter 1

## Introduction

Content for chapter 1 introduction.

## Setup

Setup instructions for chapter 1.

# Chapter 2

## Introduction - VIOLATION (default): Same content as Chapter 1 Introduction

Content for chapter 2 introduction.

## Setup - VIOLATION (default): Same content as Chapter 1 Setup

Setup instructions for chapter 2.

### Advanced Setup

More detailed setup.

### Advanced Setup - VIOLATION (default): Duplicate at same level

Another advanced setup section.

## Siblings Only Configuration Tests

The following would be ALLOWED with siblings_only=true but VIOLATIONS with default:

# Part A

## Section 1

Content here.

# Part B  

## Section 1 - ALLOWED (siblings_only): Different parent

Content here.

But these would still be violations even with siblings_only=true:

# Part C

## Section Alpha

Content here.

## Section Alpha - VIOLATION (even with siblings_only): Same parent

Content here.

## Allow Different Nesting Tests

The following would be ALLOWED with allow_different_nesting=true but VIOLATIONS with default:

# Main Topic

## Main Topic - ALLOWED (allow_different_nesting): Different level

More detailed content about the main topic.

But these would still be violations even with allow_different_nesting=true:

## Subtopic

### Details

## Subtopic - VIOLATION (even with allow_different_nesting): Same level

More content.

## Both Options Enabled Tests

With both siblings_only=true AND allow_different_nesting=true:

# Document A

## Overview

Content A.

### Overview - ALLOWED (both options): Different level, same parent

Detailed overview.

# Document B

## Overview - ALLOWED (both options): Different parent

Content B.

## Analysis

Analysis content.

## Analysis - VIOLATION (both options): Same parent, same level

More analysis.

## Edge Cases

### Whitespace   Normalization

Content here.

###    Whitespace Normalization    - VIOLATION: Should normalize to same content

More content.

## Case Sensitivity

### Case Test

Content.

### case test - Should NOT be duplicate (case sensitive)

Content.

### CASE TEST - Should NOT be duplicate (case sensitive)

Content.

## Empty and Near-Empty Headings

### 

### - VIOLATION: Both are empty

####

####   - VIOLATION: Both are effectively empty after trimming

This file tests various MD024 configuration scenarios comprehensively.