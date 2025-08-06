use std::rc::Rc;

use crate::linter::{Context, RuleLinter};

pub mod md001;
pub mod md003;
pub mod md013;
pub mod md051;
pub mod md052;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleType {
    /// Rules that primarily analyze raw text lines (e.g., line length, whitespace)
    Line,
    /// Rules that analyze specific AST node types (e.g., headings, lists, code blocks)
    Token,
    /// Rules that require full document analysis (e.g., duplicate headings, cross-references)
    Document,
}

#[derive(Debug)]
pub struct Rule {
    pub id: &'static str,
    pub alias: &'static str,
    pub tags: &'static [&'static str],
    pub description: &'static str,
    pub rule_type: RuleType,
    pub required_nodes: &'static [&'static str], // For caching optimization
    pub new_linter: fn(Rc<Context>) -> Box<dyn RuleLinter>,
}

pub const ALL_RULES: &[Rule] = &[md001::MD001, md003::MD003, md013::MD013, md051::MD051, md052::MD052];
