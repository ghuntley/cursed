//! Statement nodes for the CURSED Abstract Syntax Tree
//!
//! This module contains all the statement types that represent the executable
//! units in CURSED programs. Statements are top-level constructs that don't
//! produce values but instead perform actions or define structures.
//!
//! The module is organized into submodules by statement category:
//!
//! - `declarations`: Variable, constant, function, and type declarations
//! - `block`: Blocks of grouped statements
//! - `expressions`: Expression statements (expressions used as statements)
//! - `fields`: Field declarations in struct definitions

// Submodules
pub mod block;
pub mod declarations;
pub mod expressions;
pub mod fields;
pub mod go_statement;

// Re-export all types for easier imports
pub use self::block::BlockStatement;
pub use self::declarations::{
    FactsStatement, ImportStatement, LetStatement, PackageStatement, ReturnStatement,
};
pub use self::expressions::ExpressionStatement;
pub use self::fields::FieldStatement;
pub use self::go_statement::GoStatement;
