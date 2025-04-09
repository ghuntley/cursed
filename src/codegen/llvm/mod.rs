//! LLVM IR generation module for the Cursed compiler
//!
//! This module is responsible for generating LLVM IR from the AST
//! and providing JIT compilation capabilities.

// Re-export the LlvmCodeGenerator
pub mod generator;
pub use generator::LlvmCodeGenerator;

// Type conversion utilities
pub mod types;

// Expression and statement code generation
pub mod expressions;
pub mod statements;

// Concurrency support
pub mod goroutines;
pub mod channels;

// LLVM optimization and intrinsics
pub mod optimization;
pub mod intrinsics;