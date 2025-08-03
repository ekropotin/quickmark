use std::{fmt::Display, path::PathBuf, rc::Rc};
use tree_sitter::{Node, Parser};
use tree_sitter_md::LANGUAGE;

use crate::{
    config::{QuickmarkConfig, RuleSeverity},
    rules::{Rule, ALL_RULES},
    tree_sitter_walker::TreeSitterWalker,
};

#[derive(Debug, Clone)]
pub struct CharPosition {
    pub line: usize,
    pub character: usize,
}

#[derive(Debug, Clone)]
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
    location: Location,
    message: String,
    rule: &'static Rule,
}

impl RuleViolation {
    pub fn new(
        rule: &'static Rule,
        message: String,
        file_path: PathBuf,
        range: Range,
    ) -> Self {
        Self {
            rule,
            message,
            location: Location {
                file_path,
                range,
            },
        }
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn rule(&self) -> &'static Rule {
        self.rule
    }
}

/// Convert from tree-sitter range to library range
pub fn range_from_tree_sitter(ts_range: &tree_sitter::Range) -> Range {
    Range {
        start: CharPosition {
            line: ts_range.start_point.row,
            character: ts_range.start_point.column,
        },
        end: CharPosition {
            line: ts_range.end_point.row,
            character: ts_range.end_point.column,
        },
    }
}

impl Display for RuleViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{} {}/{} {}",
            self.location().file_path.to_string_lossy(),
            self.location().range.start.line,
            self.location().range.start.character,
            self.rule().id,
            self.rule().alias,
            self.message()
        )
    }
}

#[derive(Debug)]
pub struct Context {
    pub file_path: PathBuf,
    pub config: QuickmarkConfig,
}

pub trait RuleLinter {
    fn feed(&mut self, node: &Node, source: &str) -> Option<RuleViolation>;
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
                .filter_map(|linter| linter.feed(&_node, document))
                .collect::<Vec<_>>();
            violations.extend(node_violations);
        });
        violations
    }
}


#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use crate::{
        config::{self, QuickmarkConfig, RuleSeverity},
        rules::{md001::MD001, md003::MD003},
    };

    use super::{Context, MultiRuleLinter};

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
        assert_eq!(MD003.id, violations[0].rule().id);
        assert_eq!(2, violations[0].location().range.start.line);
        assert_eq!(MD001.id, violations[1].rule().id);
        assert_eq!(4, violations[1].location().range.start.line);
    }
}
