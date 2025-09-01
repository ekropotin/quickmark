# Test MD051 Comprehensive

This file tests various MD051 features and configuration options.

## Basic Headings

### Test Heading One

### Test Heading Two

## Case Sensitivity Tests

[Valid lowercase link](#test-heading-one)
[Invalid uppercase link](#test-heading-one)
[Mixed case invalid](#test-heading-two)

## Custom Anchors

### Heading with Custom Anchor {#custom-test-anchor}

[Valid custom anchor link](#custom-test-anchor)
[Invalid custom anchor link](#wrong-custom-anchor)

## Punctuation in Headings

### Heading: With? Special! Characters

[Valid punctuation link](#heading-with-special-characters)
[Invalid punctuation link](#heading-with-special-characters!)

## HTML Elements

<div id="test-html-id">HTML content</div>
<a name="test-html-name">Named anchor</a>

[Valid HTML id link](#test-html-id)
[Valid HTML name link](#test-html-name)
[Invalid HTML link](#wrong-html-id)

## GitHub Special Cases

[Valid top link](#top)
[Valid line link](#L123)
[Valid range link](#L10C1-L20C5)
[Invalid line format](#L)
[Invalid range format](#L10-L20)

## Setext Headings

First Setext Heading
====================

Second Setext Heading
---------------------

[Valid setext h1 link](#first-setext-heading)
[Valid setext h2 link](#second-setext-heading)
[Invalid setext link](#wrong-setext-heading)

## Duplicate Headings

### Duplicate Name

### Duplicate Name

[Link to first duplicate](#duplicate-name)
[Link to second duplicate](#duplicate-name-1)
[Invalid duplicate link](#duplicate-name-2)

## Multiple Links in Same Paragraph

This paragraph has [valid link](#test-heading-one) and [invalid link](#nonexistent) and [another valid](#custom-test-anchor).

## Edge Cases

[Empty fragment link](#)
[Fragment with spaces](#test heading one)
[Fragment with underscores](#test_heading_one)
[Fragment with numbers](#test-heading-123)

### Should not trigger

[Fragment with external link](https://developer.hashicorp.com/vault/api-docs/auth/jwt#default_role)
[Fragment with relative link](../../project/issues/managing_issues.md#add-an-issue-to-an-iteration-starter)
