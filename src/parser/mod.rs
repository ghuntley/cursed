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
mod select;
mod channel_range;
mod context;
mod expression_list;
mod expressions;
mod generic_constraints_simple;
mod parser;
mod precedence;
pub mod preprocessor;
mod range;
mod range_expressions;
mod range_expression_error_recovery;
mod range_expression_error_recovery_simple;
mod range_clause_error_recovery;
mod range_clause_error_recovery_simple;
mod reference;
mod slice_literal;
mod statements;
mod struct_field_inference;
mod switch;
mod type_assertion;
mod type_switch;
mod types;

#[cfg(test)]
mod tests;

pub use parser::Parser;
pub use preprocessor::{Preprocessor, TokenStream};
pub use range_clause_error_recovery::RangeClauseErrorRecovery;
pub use range_clause_error_recovery_simple::RangeClauseErrorRecoverySimple;
pub use range_expression_error_recovery::{RangeExpressionErrorRecovery, EnhancedRangeExpressionParsing};
pub use range_expression_error_recovery_simple::RangeExpressionErrorRecoverySimple;
pub use struct_field_inference::StructFieldTypeInference;
