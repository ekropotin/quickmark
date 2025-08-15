The goal is to port $ARGEMENTS rule implementation from the original markdownlinter.
Think hard to create an implementation plan.
Besides other steps you may come up with, you must incorporate the steps below.

## 1. Unit tests

Write comprehensive unit-tests covering as much as possible combinations of rule's settings as possible. Embrace TDD approach. This means, start with writing minimum set of data structures needed for a test, refrain from writing actual logic for linting at this stage. When, write unit tests. Confirm they are failing.
Update existing config's deserialization tests with parameters for the rule.

## 2. Logic implementation

Iterate on the implementation, continue until all tests are green.

## 3. Creating test samples

You'd also need to create new samples for that rule in `test-samples` directory, following existing naming conventions.

## 4. Parity validation

You must validate that the implementation is consistent with markdownlinter.
Parity means the reported violations should match in type, quantity as well as in reported lines/character positions.
This must be done via running both linters against test samples and when analyzing the output. If any inconsistencies found - you must fix them.

### Important considerations

    - Even minior deviations of behaviour from markdownlint are not acceptable. 100% parity MUST be achived. The only exception is quickmark's behaviour which is more aligned with the Commonmark specification.
    - For fixing inconsistencies, TDD approach MUST be used. Every discovered edge case should be covered by test.
    - `markdownlint` is avaliable as global tool on this machine. Just call it from pwd.

## 5. Documentation update

At the end, copy original rule documentation in `docs/rules`.
Update README.md file accordingly.
