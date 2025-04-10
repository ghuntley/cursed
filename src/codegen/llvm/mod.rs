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
pub mod basic_expressions;
pub mod statement;
pub mod control_flow;
pub mod break_continue;
pub mod expression;
//pub mod expressions;
//pub mod statements;

// User-defined types
//pub mod user_types;

// Concurrency support
//pub mod concurrency;
pub mod goroutines;
pub mod channels;

// LLVM optimization and intrinsics
pub mod optimization;
pub mod intrinsics;