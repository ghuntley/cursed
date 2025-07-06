//! LLVM Codegen for CURSED Goroutines
//!
//! This module implements LLVM IR generation for goroutine operations:
//! - Goroutine spawning (`stan function()`)
//! - Goroutine yielding (`yolo`)
//! - Integration with the runtime scheduler
//! - FFI function calls to runtime system

use crate::error::CursedError;
use crate::ast::{Expression, Statement};
use crate::codegen::llvm::LlvmCodeGenerator;

/// Goroutine-specific LLVM codegen implementation
pub struct GoroutineCodegen;

impl GoroutineCodegen {
    pub fn new() -> Self {
        Self
    }

    /// Generate LLVM IR for goroutine spawn (`stan function()`)
    pub fn generate_goroutine_spawn(
        &self,
        function_expr: &Expression,
        args: &[Expression],
        codegen: &mut LlvmCodeGenerator,
    ) -> Result<String, CursedError> {
        // Generate function pointer and arguments
        let function_name = match function_expr {
            Expression::Identifier(name) => name.clone(),
            _ => return Err(CursedError::compiler_error("Invalid function expression for goroutine spawn")),
        };
        
        // Generate LLVM IR for goroutine spawn
        let function_ptr_reg = format!("%{}", codegen.next_variable());
        let args_ptr_reg = format!("%{}", codegen.next_variable());
        let result_reg = format!("%{}", codegen.next_variable());
        
        let mut ir_code = String::new();
        
        // Declare runtime function if not already declared
        if !codegen.has_function_declaration("cursed_stan_goroutine") {
            ir_code.push_str("declare i64 @cursed_stan_goroutine(i8*, i8*)\n");
            codegen.mark_function_declared("cursed_stan_goroutine");
        }
        
        // Cast function to function pointer
        ir_code.push_str(&format!(
            "  {} = bitcast i8* ({}) to i8*\n",
            function_ptr_reg, function_name
        ));
        
        // Create null pointer for arguments (simplified)
        ir_code.push_str(&format!(
            "  {} = inttoptr i64 0 to i8*\n",
            args_ptr_reg
        ));
        
        // Call runtime goroutine spawn function
        ir_code.push_str(&format!(
            "  {} = call i64 @cursed_stan_goroutine(i8* {}, i8* {})\n",
            result_reg, function_ptr_reg, args_ptr_reg
        ));
        
        Ok(ir_code)
    }

    /// Generate LLVM IR for goroutine yield (`yolo`)
    pub fn generate_goroutine_yield(
        &self,
        codegen: &mut LlvmCodeGenerator,
    ) -> Result<String, CursedError> {
        let result_reg = format!("%{}", codegen.next_variable());
        
        let mut ir_code = String::new();
        
        // Declare runtime function if not already declared
        if !codegen.has_function_declaration("cursed_yolo_goroutine") {
            ir_code.push_str("declare i1 @cursed_yolo_goroutine()\n");
            codegen.mark_function_declared("cursed_yolo_goroutine");
        }
        
        // Call runtime yield function
        ir_code.push_str(&format!(
            "  {} = call i1 @cursed_yolo_goroutine()\n",
            result_reg
        ));
        
        Ok(ir_code)
    }

    /// Generate LLVM IR for goroutine join (wait for completion)
    pub fn generate_goroutine_join(
        &self,
        goroutine_id_expr: &Expression,
        codegen: &mut LlvmCodeGenerator,
    ) -> Result<String, CursedError> {
        let goroutine_id_reg = format!("%{}", codegen.next_variable());
        let result_reg = format!("%{}", codegen.next_variable());
        
        let mut ir_code = String::new();
        
        // Generate code for goroutine ID
        let id_code = codegen.generate_expression_public(goroutine_id_expr)?;
        ir_code.push_str(&id_code);
        
        // Declare runtime function if not already declared
        if !codegen.has_function_declaration("cursed_goroutine_join") {
            ir_code.push_str("declare i32 @cursed_goroutine_join(i64)\n");
            codegen.mark_function_declared("cursed_goroutine_join");
        }
        
        // Call runtime join function - use the last generated variable
        let last_var = format!("t{}", codegen.get_last_variable_counter());
        ir_code.push_str(&format!(
            "  {} = call i32 @cursed_goroutine_join(i64 %{})\n",
            result_reg, last_var
        ));
        
        Ok(ir_code)
    }
}

/// Integration with main LLVM codegen
impl LlvmCodeGenerator {
    /// Generate LLVM IR for goroutine spawn statement
    pub fn generate_goroutine_spawn_statement(
        &mut self,
        function_expr: &Expression,
        args: &[Expression],
    ) -> Result<String, CursedError> {
        let goroutine_codegen = GoroutineCodegen::new();
        goroutine_codegen.generate_goroutine_spawn(function_expr, args, self)
    }

    /// Generate LLVM IR for goroutine yield statement
    pub fn generate_goroutine_yield_statement(&mut self) -> Result<String, CursedError> {
        let goroutine_codegen = GoroutineCodegen::new();
        goroutine_codegen.generate_goroutine_yield(self)
    }

    /// Generate LLVM IR for goroutine join statement
    pub fn generate_goroutine_join_statement(
        &mut self,
        goroutine_id_expr: &Expression,
    ) -> Result<String, CursedError> {
        let goroutine_codegen = GoroutineCodegen::new();
        goroutine_codegen.generate_goroutine_join(goroutine_id_expr, self)
    }
}

/// Keep existing minimal implementation for compatibility
pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED goroutine scheduler enabled".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goroutine_codegen_creation() {
        let goroutine_codegen = GoroutineCodegen::new();
        // Test that we can create the codegen instance
        assert!(true); // Basic instantiation test
    }

    #[test]
    fn test_goroutine_spawn_generation() {
        let mut codegen = LlvmCodeGenerator::new().unwrap();
        let function_expr = Expression::Identifier("test_func".to_string());
        let args = vec![];
        
        let result = codegen.generate_goroutine_spawn_statement(&function_expr, &args);
        assert!(result.is_ok());
        
        let ir_code = result.unwrap();
        assert!(ir_code.contains("cursed_stan_goroutine"));
    }

    #[test]
    fn test_goroutine_yield_generation() {
        let mut codegen = LlvmCodeGenerator::new().unwrap();
        
        let result = codegen.generate_goroutine_yield_statement();
        assert!(result.is_ok());
        
        let ir_code = result.unwrap();
        assert!(ir_code.contains("cursed_yolo_goroutine"));
    }
}
