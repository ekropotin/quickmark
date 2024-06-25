use crate::linter::{Context, RuleLinter};

pub mod md001;
pub mod md003;

#[derive(Debug)]
pub struct Rule {
    pub id: &'static str,
    pub alias: &'static str,
    pub tags: &'static [&'static str],
    pub description: &'static str,
    pub new_linter: fn(Context) -> Box<dyn RuleLinter>,
}

pub const ALL_RULES: &[Rule] = &[md001::MD001, md003::MD003];
