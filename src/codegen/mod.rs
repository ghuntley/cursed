// Code generation module for CURSED
use crate::error_types::CursedError;

pub mod llvm;

// Re-export main types
pub use llvm::{LlvmCodeGenerator, LlvmType};

/// Main code generator interface
pub trait CodeGenerator {
    fn compile(&mut self, source: &str) -> crate::error_types::Result<String>;
    fn set_optimization_level(&mut self, level: u8);
    fn set_target_triple(&mut self, triple: String);
}

impl CodeGenerator for LlvmCodeGenerator {
    fn compile(&mut self, source: &str) -> crate::error_types::Result<String> {
        self.compile(source)
    }

    fn set_optimization_level(&mut self, level: u8) {
        self.optimization_level = level;
    }

    fn set_target_triple(&mut self, triple: String) {
        self.target_triple = triple;
    }
}
