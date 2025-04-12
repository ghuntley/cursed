//! # Lexical Analysis for CURSED Language
//!
//! This module implements the lexical analyzer (lexer) for the CURSED programming language.
//! The lexer converts source code text into a stream of tokens that can be processed by the parser.
//!
//! ## Key Components
//!
//! * `Lexer`: The main lexical analyzer that converts source to tokens
//! * `Token`: Represents individual tokens in the language
//! * `TokenType`: Enumeration of all token types in the language
//! * `utils`: Helper functions for character classification and manipulation
//! * `debug`: Tools for debugging token streams

pub mod token;
pub mod utils;
pub mod lexer;
pub mod lexer_methods;
pub mod debug;
pub mod token_type;

pub use token::Token;
pub use token_type::TokenType;
pub use lexer::Lexer;
pub use debug::debug_tokens;