use std::{fmt::Display, path::PathBuf, rc::Rc};
use tree_sitter::{Node, Parser};
use tree_sitter_md::LANGUAGE;

use crate::{
    config::{QuickmarkConfig, RuleSeverity},
    rules::{Rule, ALL_RULES},
    tree_sitter_walker::TreeSitterWalker,
};

#[derive(Debug)]
pub struct CharPosition {
    pub line: usize,
    pub character: usize,
}

#[derive(Debug)]
pub struct Range {
    pub start: CharPosition,
    pub end: CharPosition,
}
#[derive(Debug)]
pub struct Location {
    pub file_path: PathBuf,
    pub range: Range,
}

#[derive(Debug)]
pub struct RuleViolation {
    pub location: Location,
    pub message: String,
    pub rule: &'static Rule,
}

impl RuleViolation {
    pub fn new(
        rule: &'static Rule,
        message: String,
        file_path: PathBuf,
        pos: &tree_sitter::Range,
    ) -> Self {
        Self {
            rule,
            message,
            location: Location {
                file_path,
                range: Range {
                    start: CharPosition {
                        line: pos.start_point.row,
                        character: pos.start_point.column,
                    },
                    end: CharPosition {
                        line: pos.end_point.row,
                        character: pos.end_point.column,
                    },
                },
            },
        }
    }
}

impl Display for RuleViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{} {}/{} {}",
            self.location.file_path.to_string_lossy(),
            self.location.range.start.line,
            self.location.range.start.character,
            self.rule.id,
            self.rule.alias,
            self.message
        )
    }
}

#[derive(Debug)]
pub struct Context {
    pub file_path: PathBuf,
    pub config: QuickmarkConfig,
}

pub trait RuleLinter {
    fn feed(&mut self, node: &Node) -> Option<RuleViolation>;
}
pub struct MultiRuleLinter {
    linters: Vec<Box<dyn RuleLinter>>,
}

impl MultiRuleLinter {
    pub fn new(context: Rc<Context>) -> Self {
        Self {
            linters: ALL_RULES
                .iter()
                .filter(|r| {
                    *context.config.linters.severity.get(r.alias).unwrap() != RuleSeverity::Off
                })
                .map(|r| ((r.new_linter)(context.clone())))
                .collect(),
        }
    }

    pub fn lint(&mut self, document: &str) -> Vec<RuleViolation> {
        let mut parser = Parser::new();
        parser
            .set_language(&LANGUAGE.into())
            .expect("Error loading Markdown grammar");
        let tree = parser.parse(document, None).expect("Parse failed");

        let mut violations = Vec::new();
        let walker = TreeSitterWalker::new(&tree);
        walker.walk(|_node| {
            let node_violations = self
                .linters
                .iter_mut()
                .filter_map(|linter| linter.feed(&_node))
                .collect::<Vec<_>>();
            violations.extend(node_violations);
        });
        violations
    }
}

pub fn print_linting_errors(results: &[RuleViolation], config: &QuickmarkConfig) -> (i32, i32) {
    let severities = &config.linters.severity;

    let res = results.iter().fold((0, 0), |(errs, warns), v| {
        let severity = severities.get(v.rule.alias).unwrap();
        let prefix;
        let mut new_err = errs;
        let mut new_warns = warns;
        match severity {
            RuleSeverity::Error => {
                prefix = "ERR";
                new_err += 1;
            }
            _ => {
                prefix = "WARN";
                new_warns += 1;
            }
        };
        eprintln!("{}: {}", prefix, v);
        (new_err, new_warns)
    });

    println!("\nErrors: {}", res.0);
    println!("Warnings: {}", res.1);
    res
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use tree_sitter::Range;

    use crate::{
        config::{self, QuickmarkConfig, RuleSeverity},
        rules::{md001::MD001, md003::MD003},
    };

    use super::{print_linting_errors, Context, MultiRuleLinter, RuleViolation};

    #[test]
    fn test_print_linting_errors() {
        let severity: HashMap<_, _> = vec![
            (MD001.alias.to_string(), RuleSeverity::Error),
            (MD003.alias.to_string(), RuleSeverity::Warning),
        ]
        .into_iter()
        .collect();
        let config = QuickmarkConfig {
            linters: config::LintersTable {
                severity,
                settings: config::LintersSettingsTable {
                    heading_style: config::MD003HeadingStyleTable {
                        style: config::HeadingStyle::Consistent,
                    },
                },
            },
        };
        let pos = Range {
            start_byte: 0,
            end_byte: 4,
            start_point: tree_sitter::Point { row: 1, column: 1 },
            end_point: tree_sitter::Point { row: 1, column: 5 },
        };
        let file = PathBuf::default();
        let results = vec![
            RuleViolation::new(&MD001, "all is bad".to_string(), file.clone(), &pos),
            RuleViolation::new(&MD003, "all is even worse".to_string(), file.clone(), &pos),
            RuleViolation::new(&MD003, "all is even worse2".to_string(), file.clone(), &pos),
        ];

        let (errs, warns) = print_linting_errors(&results, &config);
        assert_eq!(1, errs);
        assert_eq!(2, warns);
    }

    #[test]
    fn test_multiple_violations() {
        use std::rc::Rc;

        let severity: HashMap<_, _> = vec![
            (MD001.alias.to_string(), RuleSeverity::Error),
            (MD003.alias.to_string(), RuleSeverity::Error),
        ]
        .into_iter()
        .collect();

        let context = Rc::new(Context {
            file_path: PathBuf::from("test.md"),
            config: QuickmarkConfig {
                linters: config::LintersTable {
                    severity,
                    settings: config::LintersSettingsTable {
                        heading_style: config::MD003HeadingStyleTable {
                            style: config::HeadingStyle::ATX,
                        },
                    },
                },
            },
        });

        let mut linter = MultiRuleLinter::new(context);

        // This creates a setext h1 after an ATX h1, which should violate:
        // MD003: mixes ATX and setext styles when ATX is enforced
        // It's also at the wrong level for MD001 testing, so let's use a different approach
        let input = "
# First heading
Second heading
==============
#### Fourth level
";

        let violations = linter.lint(input);
        assert_eq!(
            2,
            violations.len(),
            "Should find both MD001 and MD003 violations"
        );
        assert_eq!(MD003.id, violations[0].rule.id);
        assert_eq!(2, violations[0].location.range.start.line);
        assert_eq!(MD001.id, violations[1].rule.id);
        assert_eq!(4, violations[1].location.range.start.line);
    }
}
