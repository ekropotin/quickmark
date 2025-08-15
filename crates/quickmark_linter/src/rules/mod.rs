use std::rc::Rc;

use crate::linter::{Context, RuleLinter};

pub mod md001;
pub mod md003;
pub mod md004;
pub mod md005;
pub mod md007;
pub mod md009;
pub mod md010;
pub mod md011;
pub mod md012;
pub mod md013;
pub mod md014;
pub mod md018;
pub mod md019;
pub mod md020;
pub mod md021;
pub mod md022;
pub mod md023;
pub mod md024;
pub mod md025;
pub mod md026;
pub mod md027;
pub mod md028;
pub mod md030;
pub mod md031;
pub mod md032;
pub mod md033;
pub mod md034;
pub mod md035;
pub mod md036;
pub mod md037;
pub mod md040;
pub mod md043;
pub mod md046;
pub mod md048;
pub mod md051;
pub mod md052;
pub mod md053;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleType {
    /// Rules that primarily analyze raw text lines (e.g., line length, whitespace)
    Line,
    /// Rules that analyze specific AST node types (e.g., headings, lists, code blocks)
    Token,
    /// Rules that require full document analysis (e.g., duplicate headings, cross-references)
    Document,
    /// Rules that need both AST nodes and line context (blank line spacing around elements)
    Hybrid,
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

pub const ALL_RULES: &[Rule] = &[
    md001::MD001,
    md003::MD003,
    md004::MD004,
    md005::MD005,
    md007::MD007,
    md009::MD009,
    md010::MD010,
    md011::MD011,
    md012::MD012,
    md013::MD013,
    md014::MD014,
    md018::MD018,
    md019::MD019,
    md020::MD020,
    md021::MD021,
    md022::MD022,
    md023::MD023,
    md024::MD024,
    md025::MD025,
    md026::MD026,
    md027::MD027,
    md028::MD028,
    md030::MD030,
    md031::MD031,
    md032::MD032,
    md033::MD033,
    md034::MD034,
    md035::MD035,
    md036::MD036,
    md037::MD037,
    md040::MD040,
    md043::MD043,
    md046::MD046,
    md048::MD048,
    md051::MD051,
    md052::MD052,
    md053::MD053,
];
