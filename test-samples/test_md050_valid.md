# MD050 Valid Test Cases (Asterisk Style)

This document contains examples that should NOT trigger MD050 violations.
All strong text uses consistent asterisk style.

## Consistent asterisk style

This paragraph has **strong text** and **another strong text**.

Here's **more strong text** in the same style.

## Mixed with emphasis (should not affect strong consistency)

This paragraph has *emphasis* and **strong** text together.

Here's _emphasis_ and **strong** with consistent strong style.

## Strong emphasis (triple markers)

This has ***strong emphasis*** and ***more strong emphasis***.

## Code contexts (should be ignored)

This has `**strong in code**` which should not affect consistency.

```
**strong in code block**
__also in code block__
```

The **actual strong** text outside code should remain consistent.

## Multiple strong in same paragraph

This has **multiple** strong **words** in the **same paragraph**.

## Strong at beginning and end

**Strong at start** and content with **strong at end**.