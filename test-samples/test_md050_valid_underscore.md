# MD050 Valid Test Cases (Underscore Style)

This document contains examples that should NOT trigger MD050 violations.
All strong text uses consistent underscore style.

## Consistent underscore style

This paragraph has __strong text__ and __another strong text__.

Here's __more strong text__ in the same style.

## Mixed with emphasis (should not affect strong consistency)

This paragraph has *emphasis* and __strong__ text together.

Here's _emphasis_ and __strong__ with consistent strong style.

## Strong emphasis (triple markers)

This has ___strong emphasis___ and ___more strong emphasis___.

## Code contexts (should be ignored)

This has `**strong in code**` which should not affect consistency.

```
**strong in code block**
__also in code block__
```

The __actual strong__ text outside code should remain consistent.

## Multiple strong in same paragraph

This has __multiple__ strong __words__ in the __same paragraph__.

## Strong at beginning and end

__Strong at start__ and content with __strong at end__.