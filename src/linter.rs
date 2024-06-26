use std::{fmt::Display, path::PathBuf};

use comrak::{
    nodes::{Ast, Sourcepos},
    parse_document, Arena, Options,
};

use crate::{
    config::{QuickmarkConfig, RuleSeverity},
    rules::{Rule, ALL_RULES},
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
    pub fn new(rule: &'static Rule, message: String, file_path: PathBuf, pos: &Sourcepos) -> Self {
        Self {
            rule,
            message,
            location: Location {
                file_path,
                range: Range {
                    start: CharPosition {
                        line: pos.start.line,
                        character: pos.start.column,
                    },
                    end: CharPosition {
                        line: pos.end.line,
                        character: pos.end.column,
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

#[derive(Debug, Clone)]
pub struct Context {
    pub file_path: PathBuf,
    pub config: QuickmarkConfig,
}

pub trait RuleLinter {
    fn feed(&mut self, node: &Ast) -> Option<RuleViolation>;
}
pub struct MultiRuleLinter {
    linters: Vec<Box<dyn RuleLinter>>,
}

impl MultiRuleLinter {
    pub fn new(context: Context) -> Self {
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
        let arena = Arena::new();
        parse_document(&arena, document, &Options::default())
            .descendants()
            .filter_map(|node| {
                self.linters
                    .iter_mut()
                    .filter_map(|linter| linter.feed(&node.data.borrow()))
                    .next()
            })
            .collect()
    }
}

pub fn lint_content(input: &str, linter: &mut Box<dyn RuleLinter>) -> Vec<RuleViolation> {
    parse_document(&Arena::new(), input, &Options::default())
        .descendants()
        .filter_map(|node| linter.feed(&node.data.borrow()))
        .collect()
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

    use comrak::nodes::Sourcepos;

    use crate::{
        config::{self, QuickmarkConfig, RuleSeverity},
        rules::{md001::MD001, md003::MD003},
    };

    use super::{print_linting_errors, RuleViolation};

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
        let pos = Sourcepos {
            start: comrak::nodes::LineColumn { line: 1, column: 1 },
            end: comrak::nodes::LineColumn { line: 1, column: 5 },
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
}
