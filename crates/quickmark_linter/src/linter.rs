use std::{cell::RefCell, collections::HashMap, fmt::Display, path::PathBuf, rc::Rc};
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

/// **SINGLE-USE CONTRACT**: Context instances are designed for one-time use only.
///
/// Each Context instance should be used to analyze exactly one source document.
/// The lazy initialization of caches (lines, node_cache) happens once and the
/// context becomes immutable after that point.
///
#[derive(Debug)]
pub struct Context {
    pub file_path: PathBuf,
    pub config: QuickmarkConfig,
    /// Raw text lines for line-based rules (MD013, MD010, etc.) - initialized once per document
    pub lines: RefCell<Vec<String>>,
    /// Cached AST nodes filtered by type for efficient access - initialized once per document
    pub node_cache: RefCell<HashMap<String, Vec<NodeInfo>>>,
    /// Original document content for byte-based access - initialized once per document
    pub document_content: RefCell<String>,
}

/// Lightweight node information for caching
#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub line_start: usize,
    pub line_end: usize,
    pub kind: String,
}

impl Context {
    pub fn new_enhanced(file_path: PathBuf, config: QuickmarkConfig, source: &str, root_node: &Node) -> Self {
        let lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();
        let node_cache = Self::build_node_cache(root_node);

        Self {
            file_path,
            config,
            lines: RefCell::new(lines),
            node_cache: RefCell::new(node_cache),
            document_content: RefCell::new(source.to_string()),
        }
    }

    /// Get the full document content as a string reference
    /// Returns a reference to the original document content stored during initialization
    pub fn get_document_content(&self) -> std::cell::Ref<String> {
        self.document_content.borrow()
    }

    /// Build cache of nodes filtered by type for efficient rule access
    fn build_node_cache(root_node: &Node) -> HashMap<String, Vec<NodeInfo>> {
        let mut cache = HashMap::new();
        Self::collect_nodes_recursive(root_node, &mut cache);
        cache
    }

    fn collect_nodes_recursive(node: &Node, cache: &mut HashMap<String, Vec<NodeInfo>>) {
        let node_info = NodeInfo {
            line_start: node.start_position().row,
            line_end: node.end_position().row,
            kind: node.kind().to_string(),
        };

        // Add to cache for this node type
        cache.entry(node.kind().to_string())
            .or_default()
            .push(node_info.clone());

        // Add to cache for pattern-based lookups (e.g., all heading types)
        if node.kind().contains("heading") {
            cache.entry("*heading*".to_string())
                .or_default()
                .push(node_info.clone());
        }

        // Recursively process children
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                Self::collect_nodes_recursive(&child, cache);
            }
        }
    }

    /// Get cached nodes of specific types - optimized equivalent of filterByTypesCached
    pub fn get_nodes(&self, node_types: &[&str]) -> Vec<NodeInfo> {
        let cache = self.node_cache.borrow();
        let mut result = Vec::new();
        for node_type in node_types {
            if let Some(nodes) = cache.get(*node_type) {
                result.extend(nodes.iter().cloned());
            }
        }
        result
    }

    /// Get the most specific node type that contains a given line number
    pub fn get_node_type_for_line(&self, line_number: usize) -> String {
        let cache = self.node_cache.borrow();
        // Find the most specific (smallest range) node that contains this line
        let mut best_match: Option<&NodeInfo> = None;
        let mut smallest_range = usize::MAX;

        for nodes in cache.values() {
            for node in nodes {
                if line_number >= node.line_start && line_number <= node.line_end {
                    let range_size = node.line_end - node.line_start;
                    if range_size < smallest_range {
                        smallest_range = range_size;
                        best_match = Some(node);
                    }
                }
            }
        }

        best_match.map(|n| n.kind.clone()).unwrap_or_else(|| "text".to_string())
    }
}

/// **SINGLE-USE CONTRACT**: RuleLinter instances are designed for one-time use only.
///
/// Each RuleLinter instance should be used to analyze exactly one source document
/// and then discarded. This eliminates the complexity of state management and cleanup:
///
/// - No reset/cleanup methods needed
/// - No state contamination between different documents
/// - Simpler, more predictable behavior
///
/// After calling `analyze()` on a `MultiRuleLinter`, the entire linter and all its
/// rule instances become invalid and should not be reused.
///
/// ## Usage Pattern
/// ```rust,no_run
/// # use quickmark_linter::linter::MultiRuleLinter;
/// # use quickmark_linter::config::QuickmarkConfig;
/// # use std::path::PathBuf;
/// # let path = PathBuf::new();
/// # let config: QuickmarkConfig = unimplemented!();
/// # let source1 = "";
/// # let source2 = "";
///
/// // Correct: Create fresh linter for each document
/// let mut linter1 = MultiRuleLinter::new_for_document(path.clone(), config.clone(), source1);
/// let violations1 = linter1.analyze(); // Use once, then discard
///
/// // Create new linter for next document
/// let mut linter2 = MultiRuleLinter::new_for_document(path, config, source2);
/// let violations2 = linter2.analyze(); // Fresh linter, no contamination
/// ```
pub trait RuleLinter {
    /// Process a single AST node and potentially return a violation.
    ///
    /// **CONTRACT**: This method will be called exactly once per AST node
    /// for a single document analysis session. Rule linters have access to the
    /// document content and parsed data through their initialized Context.
    fn feed(&mut self, node: &Node) -> Option<RuleViolation>;

    /// Called after all nodes have been processed to collect any remaining violations.
    /// This is essential for rules that generate more violations than there are AST nodes.
    ///
    /// **CONTRACT**: This method will be called exactly once at the end of document analysis.
    fn finalize(&mut self) -> Vec<RuleViolation> {
        Vec::new() // Default implementation for rules that don't need finalization
    }
}
/// **SINGLE-USE CONTRACT**: MultiRuleLinter instances are designed for one-time use only.
///
/// Create a fresh MultiRuleLinter for each document you want to analyze using `new_for_document()`.
/// After calling `analyze()`, the linter and all its rule instances should be discarded.
pub struct MultiRuleLinter {
    linters: Vec<Box<dyn RuleLinter>>,
    tree: tree_sitter::Tree,
}

impl MultiRuleLinter {
    /// **SINGLE-USE API ENFORCEMENT**: Create a MultiRuleLinter bound to a specific document.
    ///
    /// This constructor enforces the single-use contract by:
    /// 1. Taking the document content immediately
    /// 2. Parsing and initializing the context cache upfront
    /// 3. Creating rule linters with pre-initialized context
    /// 4. Making the linter ready for immediate use with `analyze()`
    ///
    /// After calling `analyze()`, this linter instance should be discarded.
    pub fn new_for_document(
        file_path: PathBuf,
        config: QuickmarkConfig,
        document: &str,
    ) -> Self {
        // Parse the document immediately
        let mut parser = Parser::new();
        parser
            .set_language(&LANGUAGE.into())
            .expect("Error loading Markdown grammar");
        let tree = parser.parse(document, None).expect("Parse failed");

        // Create context with pre-initialized cache
        let context = Rc::new(Context::new_enhanced(
            file_path,
            config,
            document,
            &tree.root_node(),
        ));

        // Create rule linters with fully-initialized context
        let linters = ALL_RULES
            .iter()
            .filter(|r| {
                context.config.linters.severity.get(r.alias)
                    .map(|severity| *severity != RuleSeverity::Off)
                    .unwrap_or(false)
            })
            .map(|r| ((r.new_linter)(context.clone())))
            .collect();

        Self { linters, tree }
    }

    /// Analyze the document that was provided during construction.
    ///
    /// **SINGLE-USE CONTRACT**: This method should be called exactly once.
    /// After calling this method, the linter instance should be discarded.
    pub fn analyze(&mut self) -> Vec<RuleViolation> {
        let mut violations = Vec::new();
        let walker = TreeSitterWalker::new(&self.tree);

        walker.walk(|node| {
            let node_violations = self
                .linters
                .iter_mut()
                .filter_map(|linter| linter.feed(&node))
                .collect::<Vec<_>>();
            violations.extend(node_violations);
        });

        // Collect any remaining violations from finalize
        for linter in &mut self.linters {
            let remaining_violations = linter.finalize();
            violations.extend(remaining_violations);
        }

        violations
    }
}


#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use crate::{
        config::{self, QuickmarkConfig, RuleSeverity},
        rules::{md001::MD001, md003::MD003, md013::MD013},
    };

    use super::MultiRuleLinter;

    #[test]
    fn test_multiple_violations() {

        let severity: HashMap<_, _> = vec![
            (MD001.alias.to_string(), RuleSeverity::Error),
            (MD003.alias.to_string(), RuleSeverity::Error),
            (MD013.alias.to_string(), RuleSeverity::Error),
        ]
        .into_iter()
        .collect();

        let config = QuickmarkConfig {
            linters: config::LintersTable {
                severity,
                settings: config::LintersSettingsTable {
                    heading_style: config::MD003HeadingStyleTable {
                        style: config::HeadingStyle::ATX,
                    },
                    line_length: config::MD013LineLengthTable::default(),
                },
            },
        };

        // This creates a setext h1 after an ATX h1, which should violate:
        // MD003: mixes ATX and setext styles when ATX is enforced
        // It's also at the wrong level for MD001 testing, so let's use a different approach
        let input = "
# First heading
Second heading
==============
#### Fourth level
";

        let mut linter = MultiRuleLinter::new_for_document(PathBuf::from("test.md"), config, input);
        let violations = linter.analyze();
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
