//! LLVM Code Generator (Forwarding)
//! This file forwards to the modularized implementation in the llvm/ directory.

// Re-export from the llvm/ directory
pub use crate::codegen::llvm::context::LlvmCodeGenerator;
pub use crate::codegen::llvm::errors::LlvmCodegenError;