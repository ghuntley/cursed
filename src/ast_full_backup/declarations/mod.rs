/// Declaration modules for the CURSED programming language

pub mod async_function;
pub mod main;

// Re-export declaration types
pub use async_function::{AsyncFunctionStatement, AsyncFunctionDeclaration};

// Re-export from main declarations
pub use main::*;

// Make sure Parameter is re-exported
pub use crate::ast::expressions::Parameter;
