#![recursion_limit = "512"]

/// The CURSED programming language implementation
/// This crate provides the main API for the CURSED language.

// Keep only what's needed for main.rs
/*
pub mod ast;
pub mod code;
pub mod compiler;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod parser_impl;
pub mod symbol;
pub mod vm;
pub mod prelude;
pub mod memory;
pub mod evaluator;
pub mod object;
pub mod repl;

#[cfg(test)]
mod test_traceable;
*/

/// Version of the CURSED language
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Authors of the CURSED language
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
/// Description of the CURSED language
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Simplified Error type
#[derive(Debug)]
pub enum Error {
    Generic(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Generic(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for Error {}

/// Main entry point for the REPL
pub fn run_repl() -> Result<(), Error> {
    println!("CURSED Programming Language REPL");
    println!("Not fully implemented yet");
    Ok(())
}

