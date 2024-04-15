use std::{fmt::Display, path::PathBuf};

use comrak::{
    nodes::{Ast, Sourcepos},
    parse_document, Arena, Options,
};

use crate::rules::Rule;

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
        severity: RuleViolationSeverity,
        file_path: PathBuf,
        pos: &Sourcepos,
    ) -> Self {
        Self {
            rule,
            message: rule.description.to_string(),
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

#[derive(Clone)]
pub struct Settings {
    //TBD
}
#[derive(Clone)]
pub struct Context {
    pub file_path: PathBuf,
    pub settings: Settings,
}

pub trait RuleLinter {
    fn feed(&mut self, node: &Ast) -> Option<RuleViolation>;
}
pub struct MultiRuleLinter {
    pub linters: Vec<Box<dyn RuleLinter>>,
}

impl<'a> MultiRuleLinter {
    pub fn new(rules: &'a [Rule], context: Context) -> Self {
        Self {
            linters: rules
                .iter()
                .map(|r| (r.new_linter)(context.clone()))
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
