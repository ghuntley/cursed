mod tests;

// Re-export the parser from parser_impl.rs
pub use crate::parser_impl::Parser;

// This module just organizes tests into a separate file
// The actual parser implementation is in src/parser_impl.rs 