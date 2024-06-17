use comrak::nodes::{Ast, NodeHeading, NodeValue};

use crate::linter::{Context, HeadingStyle, RuleLinter, RuleViolation, RuleViolationSeverity};

use super::Rule;

#[derive(PartialEq, Debug)]
enum Style {
    Setext,
    Atx,
}

pub(crate) struct MD003Linter {
    context: Context,
    enforced_style: Option<Style>,
}

impl MD003Linter {
    pub fn new(context: Context) -> Self {
        let enforced_style = match context.settings.heading_style {
            HeadingStyle::Atx => Some(Style::Atx),
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
                        RuleViolationSeverity::Error,
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
    use std::path::PathBuf;

    use crate::linter::{lint_content, HeadingStyle, Settings};
    use crate::rules::Context;

    use super::MD003;

    #[test]
    fn test_heading_style_consistent_positive() {
        let context = Context {
            file_path: PathBuf::from("test.md"),
            settings: Settings {
                heading_style: HeadingStyle::Consistent,
            },
        };
        let mut linter = (MD003.new_linter)(context);

        let input = "Setext level 1
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
        let context = Context {
            file_path: PathBuf::from("test.md"),
            settings: Settings {
                heading_style: HeadingStyle::Consistent,
            },
        };
        let mut linter = (MD003.new_linter)(context);

        let input = "Setext level 1
--------------
Setext level 2
==============
";
        let violations = lint_content(input, &mut linter);
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_heading_style_consistent_negative_atx() {
        let context = Context {
            file_path: PathBuf::from("test.md"),
            settings: Settings {
                heading_style: HeadingStyle::Consistent,
            },
        };
        let mut linter = (MD003.new_linter)(context);

        let input = "# Atx heading 1
## Atx heading 2
### Atx heading 3
";
        let violations = lint_content(input, &mut linter);
        assert_eq!(violations.len(), 0);
    }
}
