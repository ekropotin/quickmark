use crate::linter::{Context, RuleLinter};

pub mod qm001;

#[derive(Debug)]
pub struct Rule {
    pub id: &'static str,
    pub alias: &'static str,
    pub tags: &'static [&'static str],
    pub description: &'static str,
    pub new_linter: fn(Context) -> Box<dyn RuleLinter>,
}
