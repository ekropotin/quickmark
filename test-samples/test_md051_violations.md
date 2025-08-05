# Test MD051 Violations

This file contains invalid link fragments that SHOULD trigger MD051 violations.

## Existing Headings

### Valid Heading

## Invalid Fragment Cases

[Link to nonexistent heading](#nonexistent-heading)

[Link with wrong case](#Valid-Heading)

[Link with extra words](#valid-heading-extra)

[Link with wrong punctuation](#valid_heading)

## Multiple Violations in One Line

[First invalid](#invalid-one) and [second invalid](#invalid-two).

## Complex Invalid Cases

### Another Valid Heading

[Wrong reference](#another-invalid-heading)

[Partial match](#another-valid)

[Case mismatch](#Another-Valid-Heading)

## Mixed Valid and Invalid

[Valid link](#valid-heading)
[Invalid link](#completely-wrong)
[Another valid](#another-valid-heading)
[Another invalid](#does-not-exist)