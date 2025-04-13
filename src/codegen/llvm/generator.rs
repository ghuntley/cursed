//! Adapter module for LLVM code generator
//! 
//! This module provides backward compatibility with code that imports from generator.rs
//! by re-exporting the LlvmCodeGenerator from context.rs.

// Re-export LlvmCodeGenerator for backward compatibility
pub use super::context::LlvmCodeGenerator;

// Re-export loop context
pub use super::LoopContext;
