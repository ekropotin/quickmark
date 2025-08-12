# Required Headings Violations Test Cases

## Test 1: Wrong heading content

# Title

## Wrong Section

### Details

This should trigger a violation because "## Wrong Section" doesn't match the required "## Section".

## Test 2: Missing required heading

# Introduction

### Details

This should trigger a violation because "## Overview" is missing.

## Test 3: Case sensitivity violation

# TITLE

## Section

This should trigger a violation when match_case is true because "# TITLE" doesn't match "# Title".

## Test 4: Missing heading at end

# Introduction

## Overview

This should trigger a violation because "### Details" is missing at the end.

## Test 5: Wrong order

# Introduction

### Details

## Overview

This should trigger a violation because headings are in wrong order.

## Test 6: Extra headings without wildcards

# Introduction

## Overview

### Details

## Extra Section

This should trigger no violation if wildcards are not used and all required headings are present.