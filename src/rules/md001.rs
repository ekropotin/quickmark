use std::rc::Rc;

use tree_sitter::Node;

use crate::{
    linter::RuleViolation,
    rules::{Context, Rule, RuleLinter},
};

pub(crate) struct MD001Linter {
    context: Rc<Context>,
    current_heading_level: u8,
}

impl MD001Linter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            context,
            current_heading_level: 0,
        }
    }
}

fn extract_heading_level(node: &Node) -> u8 {
    match node.kind() {
        "atx_heading" => {
            // Same as before: look for atx_hX_marker
            for i in 0..node.child_count() {
                let child = node.child(i).unwrap();
                if child.kind().starts_with("atx_h") && child.kind().ends_with("_marker") {
                    // "atx_h3_marker" => 3
                    return child.kind().chars().nth(5).unwrap().to_digit(10).unwrap() as u8;
                }
            }
            1 // fallback
        }
        "setext_heading" => {
            // Setext: look for setext_h1_underline or setext_h2_underline
            for i in 0..node.child_count() {
                let child = node.child(i).unwrap();
                if child.kind() == "setext_h1_underline" {
                    return 1;
                } else if child.kind() == "setext_h2_underline" {
                    return 2;
                }
            }
            1 // fallback
        }
        _ => 1,
    }
}

impl RuleLinter for MD001Linter {
    fn feed(&mut self, node: &Node) -> Option<RuleViolation> {
        if node.kind() == "atx_heading" || node.kind() == "setext_heading" {
            let level = extract_heading_level(node);

            if self.current_heading_level > 0
                && (level as i8 - self.current_heading_level as i8) > 1
            {
                return Some(RuleViolation::new(
                    &MD001,
                    format!(
                        "{} [Expected: h{}; Actual: h{}]",
                        MD001.description,
                        self.current_heading_level + 1,
                        level
                    ),
                    self.context.file_path.clone(),
                    &(node.range()),
                ));
            }
            self.current_heading_level = level;
        }
        None
    }
}

pub const MD001: Rule = Rule {
    id: "MD001",
    alias: "heading-increment",
    tags: &["headings"],
    description: "Heading levels should only increment by one level at a time",
    new_linter: |context| Box::new(MD001Linter::new(context)),
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

    fn test_context() -> Rc<Context> {
        let severity: HashMap<_, _> = vec![
            ("heading-style".to_string(), RuleSeverity::Off),
            ("heading-increment".to_string(), RuleSeverity::Error),
        ]
        .into_iter()
        .collect();
        Context {
            file_path: PathBuf::from("test.md"),
            config: QuickmarkConfig {
                linters: LintersTable {
                    severity,
                    settings: LintersSettingsTable {
                        heading_style: MD003HeadingStyleTable {
                            style: HeadingStyle::Consistent,
                        },
                    },
                },
            },
        }
        .into()
    }

    #[test]
    fn test_atx_positive() {
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

        let mut linter = MultiRuleLinter::new(test_context());
        let violations = linter.lint(input);
        assert_eq!(2, violations.len());
        let mut iter = violations.iter();
        let range1 = &iter.next().unwrap().location.range;
        assert_eq!(5, range1.start.line);
        assert_eq!(0, range1.start.character);
        assert_eq!(6, range1.end.line);
        assert_eq!(0, range1.end.character);

        let range2 = &iter.next().unwrap().location.range;
        assert_eq!(7, range2.start.line);
        assert_eq!(0, range2.start.character);
        assert_eq!(8, range2.end.line);
        assert_eq!(0, range2.end.character);
    }

    #[test]
    fn test_atx_negative() {
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

        let mut linter = MultiRuleLinter::new(test_context());
        let violations = linter.lint(input);
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_atx_negative_starts_not_with_level_1() {
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

        let mut linter = MultiRuleLinter::new(test_context());
        let violations = linter.lint(input);
        assert_eq!(0, violations.len());
    }

    #[test]
    fn test_setext_positive() {
        let input = "
Heading level 1
===============
some text
`some code`
### Heading level 3
some other text
         ";

        let mut linter = MultiRuleLinter::new(test_context());
        let violations = linter.lint(input);
        // Should trigger a violation: setext h1 -> atx h3 (skips h2)
        assert_eq!(1, violations.len());
        let range = &violations[0].location.range;
        // The violation should be on the h3 heading
        assert_eq!(5, range.start.line);
        assert_eq!(0, range.start.character);
    }

    #[test]
    fn test_setext_negative() {
        let input = "
Heading level 1
===============
some text
Heading level 2
---------------
some other text
";

        let mut linter = MultiRuleLinter::new(test_context());
        let violations = linter.lint(input);
        // Should be no violations: setext h1 -> setext h2
        assert_eq!(0, violations.len());
    }
}
