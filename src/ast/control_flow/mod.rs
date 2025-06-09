//! Control flow statements in the CURSED programming language.
//!
//! This module defines AST nodes for control flow structures such as
//! if/else statements, loops, switch statements, and type switches.

// Submodules
pub mod conditionals;
pub mod deferred;
pub mod loops;
pub mod range;
pub mod channel_range;
pub mod select;
pub mod switch;
pub mod type_switch;

// Re-export types for easier imports
pub use self::conditionals::{CaseStatement, IfStatement};
pub use self::deferred::LaterStatement;
pub use self::loops::{BreakStatement, ContinueStatement, ForStatement, WhileStatement};
pub use self::switch::{SwitchStatement, SwitchCase};
pub use self::type_switch::{TypeSwitchStatement, TypeCase, DefaultTypeCase, TypePattern};
pub use self::range::{RangeClause, RangeForStatement};
pub use self::channel_range::{ChannelRangeClause, ChannelRangeForStatement, ChannelClosureDetection};
pub use self::select::{SelectStatement, SelectCase, DefaultCase, TimeoutCase};

// Don't need these since they're already exported in the wildcard import above
