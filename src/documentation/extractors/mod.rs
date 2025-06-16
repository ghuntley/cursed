//! AST Documentation Extractors
//! 
//! This module provides comprehensive extractors for all CURSED language constructs,
//! ensuring complete AST integration for accurate documentation generation.

pub mod ast_extractor;
pub mod comment_extractor;
pub mod type_extractor;
pub mod generic_extractor;
pub mod relationship_extractor;
pub mod ast_node_support;

pub use ast_extractor::*;
pub use comment_extractor::*;
pub use type_extractor::*;
pub use generic_extractor::*;
pub use relationship_extractor::*;
pub use ast_node_support::*;
