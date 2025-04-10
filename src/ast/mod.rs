// Re-exports
pub mod base;
pub mod expressions;
pub mod statements;
pub mod control_flow;
pub mod declarations;
pub mod pointer;
pub mod reference;
pub mod reference_type;
pub mod dereference;
pub mod statement_utils;
pub mod traits;

// Public re-exports
pub use base::Program;
pub use pointer::{PointerType, PointerDereference};
pub use reference::ReferenceExpression;
pub use reference_type::ReferenceType;
pub use statement_utils::StatementExtensions;
pub use dereference::DerefExpression;
pub use traits::{Node, Statement, Expression};

// Re-export all AST types for easier imports
pub use expressions::*;
pub use statements::*;
pub use control_flow::*;
pub use declarations::*;