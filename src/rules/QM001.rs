use crate::{
    linter::{RuleViolation, RuleViolationSeverity},
    rules::{Context, Rule, RuleLinter},
};
use comrak::nodes::{Ast, NodeHeading, NodeValue};

pub(crate) struct QM001Linter {
    context: Context,
    current_heading_level: u8,
}

impl QM001Linter {
    pub fn new(context: Context) -> Self {
        Self {
            context,
            current_heading_level: 0,
        }
    }
}

impl RuleLinter for QM001Linter {
    fn feed(&mut self, node: &Ast) -> Option<RuleViolation> {
        if let NodeValue::Heading(NodeHeading { level, setext: _ }) = node.value {
            if self.current_heading_level > 0 && level as i8 - self.current_heading_level as i8 > 1
            {
                return Option::Some(RuleViolation::new(
                    &QM001,
                    RuleViolationSeverity::Error,
                    self.context.file_path.clone(),
                    &(node.sourcepos),
                ));
            }
            self.current_heading_level = level;
        }
        None
    }
}

pub const QM001: Rule = Rule {
    id: "QM001",
    alias: "heading-increment",
    tags: &["headings"],
    description: "Heading levels should only increment by one level at a time",
    new_linter: |context| Box::new(QM001Linter::new(context)),
};

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use comrak::{parse_document, Arena, Options};

    use crate::linter::{RuleViolation, Settings};
    use crate::rules::Context;

    use super::super::RuleLinter;
    use super::QM001;

    fn lint_content(input: &str, linter: &mut Box<dyn RuleLinter>) -> Vec<RuleViolation> {
        parse_document(&Arena::new(), input, &Options::default())
            .descendants()
            .filter_map(|node| linter.feed(&node.data.borrow()))
            .collect()
    }

    fn test_context() -> Context {
        Context {
            file_path: PathBuf::from("test.md"),
            settings: Settings {},
        }
    }

    #[test]
    fn test_positive() {
        let input = "# Heading level 1
some text
`some code`
## Heading level 2
some other text
###### Heading level 6
foobar
#### Heading level 4
### Heading level 3
";

        let violations = lint_content(input, &mut (QM001.new_linter)(test_context()));
        assert_eq!(2, violations.len());
        let mut iter = violations.iter();
        let range1 = &iter.next().unwrap().location.range;
        assert_eq!(6, range1.start.line);
        assert_eq!(1, range1.start.character);
        assert_eq!(6, range1.end.line);
        assert_eq!(22, range1.end.character);

        let range2 = &iter.next().unwrap().location.range;
        assert_eq!(8, range2.start.line);
        assert_eq!(1, range2.start.character);
        assert_eq!(8, range2.end.line);
        assert_eq!(20, range2.end.character);
    }

    #[test]
    fn test_negative() {
        let input = "# Heading level 1
some text
`some code`
## Heading level 2
some other text
### Heading level 3
foobar
#### Heading level 4
##### Heading level 5
###### Heading level 6
";

        let violations = lint_content(input, &mut (QM001.new_linter)(test_context()));
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_negative_starts_not_with_level_1() {
        let input = "## Heading level 2
some text
`some code`
### Heading level 3
some other text
#### Heading level 4
foobar
##### Heading level 5
###### Heading level 6
# level 1
";

        let violations = lint_content(input, &mut (QM001.new_linter)(test_context()));
        assert_eq!(0, violations.len());
    }
}