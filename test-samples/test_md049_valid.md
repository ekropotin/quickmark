# MD049 Valid Test Cases

## Consistent asterisk emphasis

This paragraph uses *consistent* asterisk emphasis throughout the *entire* document.

Multiple *emphasis* in the *same* paragraph should all use asterisk.

## Intraword emphasis preservation

This should work: apple*banana*cherry and apple*banana*
Also works: *banana*cherry and some*text*here

Mixed intraword with regular: apple*banana*cherry and *regular* emphasis.

## Code spans should be ignored

This `*asterisk*` in code and `_underscore_` in code should not trigger violations.

```markdown
*This* is in a code block with _mixed_ emphasis.
```

## Empty document with no emphasis

This paragraph has no emphasis at all.