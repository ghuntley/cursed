#![recursion_limit = "512"]

/// The CURSED programming language implementation
/// 
/// This crate provides the main API for the CURSED language,
/// including lexer, parser, compiler, and virtual machine.

// Standard library imports
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ptr::NonNull;

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

/// Version of the CURSED language
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Authors of the CURSED language
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
/// Description of the CURSED language
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

// Convenience re-exports at the crate level
pub use prelude::*;
pub use error::{Error, ErrorReporter, SourceLocation};
pub use ast::{Node, Statement, Expression, Program};
pub use object::Object;
pub use lexer::{Lexer, Token};

// Re-export VM and related items with full implementation
pub use vm::VM;
pub use vm::Frame;
pub use vm::Closure;
pub use vm::constants::*;

// Re-export parsers with full implementations
pub use parser_impl::Parser;

// Re-export compiler with full implementations
pub use compiler::Compiler;
pub use compiler::Bytecode;
pub use compiler::Instructions;
pub use compiler::Opcode;
pub use compiler::CompiledFunction;
pub use compiler::symbol_table::SymbolTable;
pub use compiler::symbol_table::Symbol;
pub use compiler::symbol_table::SymbolScope;

// Re-export repl
pub use repl::start_repl;

// Memory management with full implementations
pub use memory::MemoryManager;
pub use memory::allocator::Allocator;
pub use memory::allocator::AllocatorBase;
pub use memory::block::BlockAllocator;
pub use memory::bump::BumpAllocator;
pub use memory::Allocated;

// Memory internals with full implementations
pub use memory::tagged::{TaggedPtr, Tag, NonNullExt};
pub use memory::gc::{Traceable, Visitor, Trace, GarbageCollector, Gc};

/// Main entry point for the REPL
pub fn run_repl() -> Result<(), Error> {
    repl::start_repl()
}

