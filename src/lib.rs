#![recursion_limit = "512"]

/// The CURSED programming language implementation
/// 
/// This crate provides the main API for the CURSED language,
/// including lexer, parser, compiler, and virtual machine.

pub mod ast;
pub mod code;
// pub mod compiler;  // Using stub version for now
pub mod error;
pub mod lexer;
pub mod parser;
pub mod symbol;
pub mod vm;
pub mod prelude;
// pub mod prelude_ext;  // Merged into prelude.rs
// pub mod memory;  // Using stub version for now
pub mod evaluator;
pub mod object;
pub mod repl;
pub mod helpers;

// Basic stub implementations for compiler and memory
pub mod compiler {
    pub mod symbol_table {
        pub use crate::symbol::{Symbol, SymbolScope, SymbolTable};
    }
    
    // Re-export from bytecode module
    pub type Instructions = Vec<u8>;
    
    /// Bytecode operation codes
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Opcode {
        Invalid = 0,
        Nop = 1,
    }
    
    // Basic bytecode structure
    #[derive(Debug, Clone)]
    pub struct Bytecode {
        pub instructions: Instructions,
        pub constants: Vec<crate::object::Object>,
    }
    
    // Minimal compiler implementation
    pub struct Compiler {
        instructions: Instructions,
        constants: Vec<crate::object::Object>,
    }
    
    impl Compiler {
        pub fn new() -> Self {
            Self {
                instructions: Vec::new(),
                constants: Vec::new(),
            }
        }
        
        // Compile a program
        pub fn compile_program(&mut self, program: &crate::ast::Program) -> Result<Bytecode, crate::error::Error> {
            for stmt in &program.statements {
                self.compile_statement(&**stmt)?;
            }
            Ok(self.bytecode())
        }
        
        // Compile a statement
        fn compile_statement(&mut self, stmt: &dyn crate::ast::Statement) -> Result<(), crate::error::Error> {
            // Stub implementation
            Ok(())
        }
        
        pub fn bytecode(&self) -> Bytecode {
            Bytecode {
                instructions: self.instructions.clone(),
                constants: self.constants.clone(),
            }
        }
    }
    
    // Minimal compiled function
    #[derive(Debug, Clone, PartialEq)]
    pub struct CompiledFunction {
        pub instructions: Vec<u8>,
        pub num_locals: u8,
        pub num_parameters: u8,
        pub name: Option<String>,
    }
}

pub mod memory {
    pub mod gc {
        use std::collections::HashSet;
        
        // Basic traceable trait
        pub trait Traceable {
            fn trace(&self, visitor: &mut dyn Visitor);
            fn size(&self) -> usize;
        }
        
        // Simple trace trait as alias
        pub trait Trace: Traceable {}
        
        // Simple visitor trait
        pub trait Visitor {
            fn visit(&mut self, obj: &dyn Traceable);
            fn visit_ptr(&mut self, ptr: usize, tag: crate::memory::tagged::Tag);
        }
        
        // Simple GC
        pub struct GarbageCollector {
            marked: HashSet<usize>,
        }
        
        impl GarbageCollector {
            pub fn new() -> Self {
                Self {
                    marked: HashSet::new(),
                }
            }
        }
        
        // GC reference
        pub struct Gc<T: Traceable + 'static> {
            inner: T,
        }
        
        impl<T: Traceable + 'static> Gc<T> {
            pub fn new(value: T) -> Self {
                Self {
                    inner: value,
                }
            }
        }
    }
    
    pub mod tagged {
        use std::ptr::NonNull;
        
        // Simple tag enum
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Tag {
            Int,
            Float,
            String,
            Boolean,
            Array,
            Map,
            Function,
            Null,
            Object,
        }
        
        // Tagged pointer
        pub struct TaggedPtr<T> {
            ptr: NonNull<T>,
            tag: Tag,
        }
        
        // Type-erased tagged pointer
        pub struct TaggedDynPtr {
            ptr: NonNull<u8>,
            tag: Tag,
        }
        
        // NonNull extension
        pub trait NonNullExt<T> {
            fn with_tag(self, tag: Tag) -> TaggedPtr<T>;
        }
        
        impl<T> NonNullExt<T> for NonNull<T> {
            fn with_tag(self, tag: Tag) -> TaggedPtr<T> {
                TaggedPtr { ptr: self, tag }
            }
        }
    }
    
    // Allocator modules
    pub mod allocator {}
    pub mod block {}
    pub mod bump {}
    
    // Re-exports
    pub use gc::{Traceable, Visitor, GarbageCollector, Gc};
    pub use tagged::{TaggedPtr, Tag, NonNullExt};
    
    // Minimal memory manager
    pub struct MemoryManager {}
    
    impl MemoryManager {
        pub fn new() -> Self {
            Self {}
        }
    }
    
    // Allocated object
    pub struct Allocated<T> {
        pub inner: T,
    }
}

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

// Re-export VM and related items
pub use vm::VM;
pub use vm::Frame;
pub use vm::Closure;
pub use vm::constants::*;

// Re-export parsers
pub use parser::Parser;

// Re-export compiler
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

// Memory management
pub use memory::MemoryManager;
pub use memory::Allocated;

// Memory internals
pub use memory::tagged::{TaggedPtr, Tag, NonNullExt};
pub use memory::gc::{Traceable, Visitor, GarbageCollector, Gc};

/// Main entry point for the REPL
pub fn run_repl() -> Result<(), Error> {
    repl::start_repl()
}


