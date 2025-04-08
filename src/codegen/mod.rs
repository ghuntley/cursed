//! Code generation module for the Cursed compiler

// Export the LLVM codegen module
pub mod llvm;
pub mod jit;

// Re-export the main LLVM code generator
pub use llvm::LlvmCodeGenerator;