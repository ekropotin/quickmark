use core::fmt;
use std::rc::Rc;

use comrak::nodes::{Ast, NodeHeading, NodeValue};

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

fn heading_style(heading: &NodeHeading) -> Style {
    match heading.setext {
        true => Style::Setext,
        false => Style::Atx,
    }
}

impl RuleLinter for MD003Linter {
    fn feed(&mut self, node: &Ast) -> Option<RuleViolation> {
        if let NodeValue::Heading(heading) = node.value {
            let style = heading_style(&heading);
            if let Some(enforced_style) = &self.enforced_style {
                if style != *enforced_style {
                    return Option::Some(RuleViolation::new(
                        &MD003,
                        format!(
                            "{} [Expected: {}; Actual: {}]",
                            MD003.description, enforced_style, style
                        ),
                        self.context.file_path.clone(),
                        &node.sourcepos,
                    ));
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
    use crate::linter::lint_content;
    use crate::rules::Context;

    use super::MD003;

    fn test_context(style: HeadingStyle) -> Rc<Context> {
        let severity: HashMap<_, _> = vec![("heading-style".to_string(), RuleSeverity::Error)]
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
        let mut linter = (MD003.new_linter)(context);

        let input = "
Setext level 1
--------------
Setext level 2
==============
### ATX header level 3
#### ATX header level 4
";
        let violations = lint_content(input, &mut linter);
        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_heading_style_consistent_negative_setext() {
        let context = test_context(HeadingStyle::Consistent);
        let mut linter = (MD003.new_linter)(context);

        let input = "
Setext level 1
--------------
Setext level 2
==============
";
        let violations = lint_content(input, &mut linter);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_heading_style_consistent_negative_atx() {
        let context = test_context(HeadingStyle::Consistent);
        let mut linter = (MD003.new_linter)(context);

        let input = "
# Atx heading 1
## Atx heading 2
### Atx heading 3
";
        let violations = lint_content(input, &mut linter);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_heading_style_atx_positive() {
        let context = test_context(HeadingStyle::ATX);
        let mut linter = (MD003.new_linter)(context);

        let input = "
Setext heading 1
----------------
Setext heading 2
================
### Atx heading 3
";
        let violations = lint_content(input, &mut linter);
        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_heading_style_atx_negative() {
        let context = test_context(HeadingStyle::ATX);
        let mut linter = (MD003.new_linter)(context);

        let input = "
# Atx heading 1
## Atx heading 2
### Atx heading 3
";
        let violations = lint_content(input, &mut linter);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_heading_style_setext_positive() {
        let context = test_context(HeadingStyle::Setext);
        let mut linter = (MD003.new_linter)(context);

        let input = "
# Atx heading 1
Setext heading 1
----------------
Setext heading 2
================
### Atx heading 3
";
        let violations = lint_content(input, &mut linter);
        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_heading_style_setext_negative() {
        let context = test_context(HeadingStyle::Setext);
        let mut linter = (MD003.new_linter)(context);

        let input = "
Setext heading 1
----------------
Setext heading 2
================
Setext heading 2
================
";
        let violations = lint_content(input, &mut linter);
        assert_eq!(violations.len(), 0);
    }
}
