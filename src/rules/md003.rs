use core::fmt;
use std::rc::Rc;
use tree_sitter::Node;

use crate::{
    config::HeadingStyle,
    linter::{Context, RuleLinter, RuleViolation},
};

use super::Rule;

#[derive(PartialEq, Debug)]
enum Style {
    Setext,
    Atx,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Style::Setext => write!(f, "setext"),
            Style::Atx => write!(f, "atx"),
        }
    }
}

pub(crate) struct MD003Linter {
    context: Rc<Context>,
    enforced_style: Option<Style>,
}

impl MD003Linter {
    pub fn new(context: Rc<Context>) -> Self {
        let enforced_style = match context.config.linters.settings.heading_style.style {
            HeadingStyle::ATX => Some(Style::Atx),
            HeadingStyle::Setext => Some(Style::Setext),
            _ => None,
        };
        Self {
            context,
            enforced_style,
        }
    }
}

impl RuleLinter for MD003Linter {
    fn feed(&mut self, node: &Node) -> Option<RuleViolation> {
        let style = match node.kind() {
            "atx_heading" => Some(Style::Atx),
            "setext_heading" => Some(Style::Setext),
            _ => None,
        };
        if let Some(style) = style {
            if let Some(enforced_style) = &self.enforced_style {
                if style != *enforced_style {
                    let start = node.start_position();
                    let end = node.end_position();
                    return Some(RuleViolation {
                        rule: &MD003,
                        message: format!(
                            "{} [Expected: {}; Actual: {}]",
                            MD003.description, enforced_style, style
                        ),
                        location: crate::linter::Location {
                            file_path: self.context.file_path.clone(),
                            range: crate::linter::Range {
                                start: crate::linter::CharPosition {
                                    line: start.row,
                                    character: start.column,
                                },
                                end: crate::linter::CharPosition {
                                    line: end.row,
                                    character: end.column,
                                },
                            },
                        },
                    });
                }
            } else {
                self.enforced_style = Some(style);
            }
        }
        None
    }
}

pub const MD003: Rule = Rule {
    id: "MD003",
    alias: "heading-style",
    tags: &["headings"],
    description: "Heading style",
    new_linter: |context| Box::new(MD003Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::rc::Rc;

    use crate::config::{
        HeadingStyle, LintersSettingsTable, LintersTable, MD003HeadingStyleTable, QuickmarkConfig,
        RuleSeverity,
    };
    use crate::linter::MultiRuleLinter;
    use crate::rules::Context;


    fn test_context(style: HeadingStyle) -> Rc<Context> {
        let severity: HashMap<_, _> = vec![
            ("heading-style".to_string(), RuleSeverity::Error),
            ("heading-increment".to_string(), RuleSeverity::Off),
        ]
        .into_iter()
        .collect();
        Context {
            file_path: PathBuf::from("test.md"),
            config: QuickmarkConfig {
                linters: LintersTable {
                    severity,
                    settings: LintersSettingsTable {
                        heading_style: MD003HeadingStyleTable { style },
                    },
                },
            },
        }
        .into()
    }

    #[test]
    fn test_heading_style_consistent_positive() {
        let context = test_context(HeadingStyle::Consistent);

        let input = "
Setext level 1
--------------
Setext level 2
==============
### ATX header level 3
#### ATX header level 4
";
        let mut linter = MultiRuleLinter::new(context);
        let violations = linter.lint(input);
        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_heading_style_consistent_negative_setext() {
        let context = test_context(HeadingStyle::Consistent);

        let input = "
Setext level 1
--------------
Setext level 2
==============
";
        let mut linter = MultiRuleLinter::new(context);
        let violations = linter.lint(input);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_heading_style_consistent_negative_atx() {
        let context = test_context(HeadingStyle::Consistent);

        let input = "
# Atx heading 1
## Atx heading 2
### Atx heading 3
";
        let mut linter = MultiRuleLinter::new(context);
        let violations = linter.lint(input);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_heading_style_atx_positive() {
        let context = test_context(HeadingStyle::ATX);

        let input = "
Setext heading 1
----------------
Setext heading 2
================
### Atx heading 3
";
        let mut linter = MultiRuleLinter::new(context);
        let violations = linter.lint(input);
        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_heading_style_atx_negative() {
        let context = test_context(HeadingStyle::ATX);

        let input = "
# Atx heading 1
## Atx heading 2
### Atx heading 3
";
        let mut linter = MultiRuleLinter::new(context);
        let violations = linter.lint(input);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_heading_style_setext_positive() {
        let context = test_context(HeadingStyle::Setext);

        let input = "
# Atx heading 1
Setext heading 1
----------------
Setext heading 2
================
### Atx heading 3
";
        let mut linter = MultiRuleLinter::new(context);
        let violations = linter.lint(input);
        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_heading_style_setext_negative() {
        let context = test_context(HeadingStyle::Setext);

        let input = "
Setext heading 1
----------------
Setext heading 2
================
Setext heading 2
================
";
        let mut linter = MultiRuleLinter::new(context);
        let violations = linter.lint(input);
        assert_eq!(violations.len(), 0);
    }
}
