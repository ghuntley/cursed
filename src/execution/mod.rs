/// CURSED JIT Execution Engine
/// 
/// This module provides the complete execution infrastructure for CURSED programs,
/// including JIT compilation, runtime execution, and value management.

pub mod jit_executor;
pub mod value_manager;
pub mod execution_context;
pub mod runtime_functions;

pub use jit_executor::{CursedExecutor, ExecutionResult, ExecutionConfig, ExecutionError};
pub use value_manager::{CursedValue, ValueType, ValueManager};
pub use execution_context::{ExecutionContext, SymbolTable, FunctionRegistry};
pub use runtime_functions::{RuntimeFunctionRegistry, register_runtime_functions};

use crate::error::Error;
use std::collections::HashMap;

/// Main execution engine that coordinates JIT compilation and execution
pub struct CursedExecutionEngine {
    executor: jit_executor::CursedExecutor,
    value_manager: value_manager::ValueManager,
    context: execution_context::ExecutionContext,
    runtime_functions: runtime_functions::RuntimeFunctionRegistry,
}

impl CursedExecutionEngine {
    /// Create a new execution engine
    pub fn new() -> Result<Self, Error> {
        let executor = jit_executor::CursedExecutor::new()?;
        let value_manager = value_manager::ValueManager::new();
        let context = execution_context::ExecutionContext::new();
        let runtime_functions = runtime_functions::RuntimeFunctionRegistry::new();

        Ok(Self {
            executor,
            value_manager,
            context,
            runtime_functions,
        })
    }

    /// Execute CURSED source code and return the result
    pub fn execute(&mut self, source: &str) -> Result<CursedValue, Error> {
        tracing::info!("Executing CURSED source code");

        // Parse and compile the source code
        let lexer = crate::lexer::Lexer::new(source.to_string());
        let mut parser = crate::parser::Parser::new(lexer)?;
        let program = parser.parse_program()?;

        // Check for parse errors
        let errors = parser.errors();
        if !errors.is_empty() {
            return Err(Error::Parse(format!("Parse errors: {}", errors.join(", "))));
        }

        // Compile to LLVM IR
        let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
        let ir = codegen.compile_program(&program, source)?;

        // Execute using JIT
        self.executor.execute_ir(&ir, &mut self.context)
    }

    /// Get the value manager for external access
    pub fn get_value_manager(&mut self) -> &mut ValueManager {
        &mut self.value_manager
    }

    /// Execute a CURSED file
    pub fn execute_file(&mut self, path: &str) -> Result<CursedValue, Error> {
        let source = std::fs::read_to_string(path)
            .map_err(|e| Error::Io(e.into()))?;
        self.execute(&source)
    }

    /// Execute REPL code and return formatted result
    pub fn execute_repl(&mut self, code: &str) -> Result<String, Error> {
        let result = self.execute(code)?;
        Ok(self.value_manager.format_value(&result))
    }

    /// Get the current execution context
    pub fn get_context(&self) -> &execution_context::ExecutionContext {
        &self.context
    }

    /// Get statistics about execution
    pub fn get_stats(&self) -> ExecutionStats {
        ExecutionStats {
            functions_compiled: self.executor.get_stats().functions_compiled,
            functions_executed: self.executor.get_stats().functions_executed,
            values_created: self.value_manager.get_stats().values_created,
            variables_defined: self.context.get_variable_count(),
        }
    }
}

/// Statistics about execution engine performance
#[derive(Debug, Clone)]
pub struct ExecutionStats {
    pub functions_compiled: u64,
    pub functions_executed: u64,
    pub values_created: u64,
    pub variables_defined: usize,
}

impl Default for CursedExecutionEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create default execution engine")
    }
}
