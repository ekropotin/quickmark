---
title: "Document Title from Front Matter"
author: "Test Author"
date: "2024-01-01"
---

# H1 After Front Matter Title

This H1 should trigger MD025 because the front matter contains a title.

## Section 1

Content here.

# Another H1 

This should also trigger MD025.

---

---
layout: post
author: "Test Author"
date: "2024-01-01"
---

# H1 Without Front Matter Title

This H1 should NOT trigger MD025 because there's no title in the front matter.

## Section

Content.

# Second H1 Without Front Matter Title

This should trigger MD025 because the first H1 established the top-level.

---

---
custom_title: "Custom Title Field"
layout: page
---

# H1 With Custom Title Field

When configured with a custom front_matter_title regex,
this should trigger MD025 if the regex matches "custom_title".

---

---
heading: "Using Different Field Name"
description: "Test description"
---

# H1 With Different Field

This tests custom regex patterns for front matter title detection.