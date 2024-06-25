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
pub enum RuleViolationSeverity {
    Error,
    Warning,
    Information,
    Hint,
}
#[derive(Debug)]
pub struct Location {
    pub file_path: PathBuf,
    pub range: Range,
}

#[derive(Debug)]
pub struct RuleViolation {
    pub location: Location,
    pub severity: RuleViolationSeverity,
    pub message: String,
    pub rule: &'static Rule,
}

impl RuleViolation {
    pub fn new(
        rule: &'static Rule,
        message: String,
        severity: RuleViolationSeverity,
        file_path: PathBuf,
        pos: &Sourcepos,
    ) -> Self {
        Self {
            rule,
            message,
            severity,
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
    pub linters: Vec<Box<dyn RuleLinter>>,
}

impl<'a> MultiRuleLinter {
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
