# MD035 Comprehensive Test Cases

This file contains comprehensive test cases for MD035 (hr-style).

## Consistent Dash Style - Valid

---

Content between horizontal rules.

---

More content.

---

Final content.

## Mixed Styles - Should Trigger Violations

---

Content after dash.

***

Content after asterisk (should be violation).

___

Content after underscore (should be violation).

- - -

Content after spaced dashes (should be violation).

* * *

Content after spaced asterisks (should be violation).

## Different Variations

<!-- All asterisk variations should be violations since first was dash -->

****

*****

******

<!-- All underscore variations should be violations -->

___

____

______

<!-- More spaced variations should be violations -->

- - - -

* * * * *

## Edge Cases

<!-- Single character horizontal rules -->

---

***

___
