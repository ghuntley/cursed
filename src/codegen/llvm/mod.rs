//! LLVM Code Generator Module
//! This module contains the LLVM code generator components for the Cursed compiler

// Import shared types and utilities
mod types;
mod core;
mod errors;
pub mod context;
mod builder;
mod expression;
mod statement;
mod function;
mod channel;
mod struct_type;
mod pointer;
mod string;
mod stan;
mod util;
mod array;
mod hash;

// Re-export the main components
pub use self::context::LlvmCodeGenerator;
pub use self::errors::LlvmCodegenError;
pub use self::types::{ImportedFunctionInfo, ImportedPackageInfo};

// Private utilities and shared functionality