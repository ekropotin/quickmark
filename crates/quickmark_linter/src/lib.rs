//! # QuickMark Linter Core
//! 
//! ## Single-Use Architecture Contract
//! 
//! **IMPORTANT**: All linter components in this crate follow a strict single-use contract:
//! 
//! - **Context**: One context instance per document analysis
//! - **MultiRuleLinter**: One linter instance per document analysis  
//! - **RuleLinter**: Individual rule linters are used once and discarded
//! 
//! This design eliminates state management complexity.
//! 
//! ### Usage Pattern
//! ```rust,no_run
//! use quickmark_linter::linter::MultiRuleLinter;
//! use quickmark_linter::config::QuickmarkConfig;
//! use std::path::PathBuf;
//! 
//! // Example usage (variables would be provided by your application)
//! # let path = PathBuf::new();
//! # let config: QuickmarkConfig = unimplemented!();
//! # let source = "";
//! 
//! // Correct: Fresh instances for each document
//! let mut linter = MultiRuleLinter::new_for_document(path, config, source);
//! let violations = linter.analyze();
//! // linter is now invalid - create new one for next document
//! ```

pub mod config;
pub mod linter;
pub mod rules;
pub mod tree_sitter_walker;

#[cfg(any(test, feature = "testing"))]
pub mod test_utils;
