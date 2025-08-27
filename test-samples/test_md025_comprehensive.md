# Basic Multiple H1 Test

This tests the basic case of multiple H1 headings.

## Section 1

Some content.

# Second H1 - Should Violate

This should trigger MD025.

---

# ATX and Setext Mix Test

Testing with mixed heading styles.

Second H1 with Setext
=====================

This setext H1 should also trigger MD025.

## Regular H2

Content.

Third H1
========

Another setext H1 violation.

---

# Comments and Whitespace Test

<!-- This is a comment before the heading -->

Some intro text that makes this H1 not the first content.

# Not First Content H1

This H1 comes after content, so MD025 should not apply to this document section.

# Another H1 After Content

This should also not trigger since the first H1 wasn't "top-level".

---

<!-- Test case: First heading is top-level -->
# Top Level H1

This H1 is the first content (comments don't count).

## Section

Content.

# Second Top Level H1 - Should Violate

This should trigger MD025.

---

# Custom Level Test (H2 as top-level)

When configured with level=2, this H1 should be ignored.

## First H2 - Top Level

When level=2, this becomes the "title" heading.

### H3 Content

Regular content.

## Second H2 - Should Violate with level=2

This would violate if level=2 is configured.

# Another H1 - Still Ignored

H1s are ignored when level=2.

---

# Front Matter Test Cases

Note: Front matter examples are conceptual since this is a regular markdown file.
In actual usage, these would have YAML front matter at the top.

When front matter contains a title field, any H1 in the document should violate.

---

# Edge Cases

## Only Lower Level Headings Valid

### H3 Content
#### H4 Content
##### H5 Content
###### H6 Content

When there are no headings at the target level, no violations should occur.

---

# Empty and Whitespace Headings

##   

##

##   

Multiple empty H2 headings (when level=2) should be treated as duplicates.

---

# ATX Closed Headings Test #

Content here.

## Section ##

More content.

# Second ATX Closed H1 # 

This should trigger MD025.