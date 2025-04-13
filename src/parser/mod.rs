//! Parser for the CURSED programming language
//!
//! This module implements a recursive descent parser that transforms
//! a token stream from the lexer into an Abstract Syntax Tree (AST).
//! The parser is responsible for syntactic analysis and ensures that
//! the code follows the CURSED language grammar rules.
//!
//! ## Components
//!
//! * `parser`: Core parser implementation
//! * `expressions`: Parsing logic for expression constructs
//! * `statements`: Parsing logic for statement constructs
//! * `types`: Type-related parsing functionality
//! * `precedence`: Operator precedence handling
//! * `channel`: Channel-specific parsing
//! * `reference`: Reference and pointer parsing
//! * `expression_list`: Parsing of expression lists
//! * `switch`: Parsing logic for switch/vibe_check statements

mod channel;
mod channel_helpers;
mod context;
mod expression_list;
mod expressions;
mod parser;
mod precedence;
pub mod preprocessor;
mod reference;
mod statements;
mod switch;
mod types;

#[cfg(test)]
mod tests;

pub use parser::Parser;
pub use preprocessor::{Preprocessor, TokenStream};
