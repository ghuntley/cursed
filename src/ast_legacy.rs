/// Legacy AST compatibility layer for CURSED language
/// 
/// This module provides backward compatibility with the old AST structure
/// while the new comprehensive AST system is in the `mod.rs` file.

// Re-export the new AST system
pub use super::ast::*;
