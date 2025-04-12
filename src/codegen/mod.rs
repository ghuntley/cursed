//! Code generation for the CURSED programming language
//!
//! This module contains the code generators that translate the CURSED Abstract
//! Syntax Tree (AST) to executable code formats, primarily LLVM Intermediate
//! Representation (IR). The code generation pipeline includes:
//!
//! 1. Type specialization through monomorphization for generic code
//! 2. LLVM IR generation from the AST
//! 3. Just-In-Time (JIT) compilation for immediate execution
//!
//! The CURSED compiler uses LLVM as its backend to leverage its extensive
//! optimization pipeline and cross-platform code generation capabilities.

pub mod jit;
pub mod llvm;
pub mod monomorphization;

pub use monomorphization::MonomorphizationManager;

// Re-export the LlvmCodeGenerator
pub use llvm::LlvmCodeGenerator;
