//! Control flow statements in the CURSED programming language.
//!
//! This module defines AST nodes for control flow structures such as
//! if/else statements, loops, and switch statements.

// Submodules
pub mod conditionals;
pub mod deferred;
pub mod loops;
pub mod switch;

// Re-export types for easier imports
pub use self::conditionals::{CaseStatement, IfStatement};
pub use self::deferred::LaterStatement;
pub use self::loops::{BreakStatement, ContinueStatement, ForStatement, WhileStatement};
pub use self::switch::{SwitchStatement, SwitchCase};

// Don't need these since they're already exported in the wildcard import above
