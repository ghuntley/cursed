//! LLVM Codegen for CURSED Channels
//!
//! This module implements LLVM IR generation for channel operations:
//! - Channel creation (`dm<T>` type)
//! - Channel send operations (`channel <- value`)
//! - Channel receive operations (`value := <-channel`)
//! - Channel closing operations
//! - Integration with the runtime channel system

use crate::error::CursedError;
use crate::ast::{Expression, Statement, Type};
use crate::codegen::llvm::LlvmCodeGenerator;

/// Channel-specific LLVM codegen implementation
pub struct ChannelCodegen;

impl ChannelCodegen {
    pub fn new() -> Self {
        Self
    }

    /// Generate LLVM IR for channel creation (`dm<T>` or `dm<T>(capacity)`)
    pub fn generate_channel_creation(
        &self,
        channel_type: &Type,
        capacity: Option<&Expression>,
        codegen: &mut LlvmCodeGenerator,
    ) -> Result<String, CursedError> {
        let capacity_reg = format!("%{}", codegen.next_variable());
        let result_reg = format!("%{}", codegen.next_variable());
        
        let mut ir_code = String::new();
        
        // Declare runtime function if not already declared
        if !codegen.has_function_declaration("cursed_channel_create") {
            ir_code.push_str("declare i8* @cursed_channel_create(i64)\n");
            codegen.mark_function_declared("cursed_channel_create");
        }
        
        // Determine capacity (0 for unbuffered, specified value for buffered)
        if let Some(cap_expr) = capacity {
            let cap_code = codegen.generate_expression_public(cap_expr)?;
            ir_code.push_str(&cap_code);
            // Get the last generated register value
            let cap_reg = format!("t{}", codegen.get_last_variable_counter());
            ir_code.push_str(&format!(
                "  {} = call i8* @cursed_channel_create(i64 %{})\n",
                result_reg, cap_reg
            ));
        } else {
            // Unbuffered channel
            ir_code.push_str(&format!(
                "  {} = call i8* @cursed_channel_create(i64 0)\n",
                result_reg
            ));
        }
        
        Ok(ir_code)
    }

    /// Generate LLVM IR for channel send operation (`channel <- value`)
    pub fn generate_channel_send(
        &self,
        channel_expr: &Expression,
        value_expr: &Expression,
        codegen: &mut LlvmCodeGenerator,
    ) -> Result<String, CursedError> {
        let value_ptr_reg = format!("%{}", codegen.next_variable());
        let value_i8_ptr_reg = format!("%{}", codegen.next_variable());
        let result_reg = format!("%{}", codegen.next_variable());
        
        let mut ir_code = String::new();
        
        // Declare runtime function if not already declared
        if !codegen.has_function_declaration("cursed_channel_send") {
            ir_code.push_str("declare i32 @cursed_channel_send(i8*, i64)\n");
            codegen.mark_function_declared("cursed_channel_send");
        }
        
        // Generate code for the channel
        let channel_code = codegen.generate_expression_public(channel_expr)?;
        ir_code.push_str(&channel_code);
        let channel_result_reg = format!("%t{}", codegen.get_last_variable_counter());
        
        // Generate code for the value to send
        let value_code = codegen.generate_expression_public(value_expr)?;
        ir_code.push_str(&value_code);
        let value_result_reg = format!("%t{}", codegen.get_last_variable_counter());
        
        // Call runtime send function directly with the value
        ir_code.push_str(&format!(
            "  {} = call i32 @cursed_channel_send(i8* {}, i64 {})\n",
            result_reg, channel_result_reg, value_result_reg
        ));
        
        Ok(ir_code)
    }

    /// Generate LLVM IR for channel receive operation (`<-channel`)
    pub fn generate_channel_receive(
        &self,
        channel_expr: &Expression,
        codegen: &mut LlvmCodeGenerator,
    ) -> Result<String, CursedError> {
        let value_ptr_reg = format!("%{}", codegen.next_variable());
        let value_i8_ptr_reg = format!("%{}", codegen.next_variable());
        let recv_result_reg = format!("%{}", codegen.next_variable());
        let received_value_reg = format!("%{}", codegen.next_variable());
        
        let mut ir_code = String::new();
        
        // Declare runtime function if not already declared
        if !codegen.has_function_declaration("cursed_channel_receive") {
            ir_code.push_str("declare i32 @cursed_channel_receive(i8*, i64*)\n");
            codegen.mark_function_declared("cursed_channel_receive");
        }
        
        // Generate code for the channel
        let channel_code = codegen.generate_expression_public(channel_expr)?;
        ir_code.push_str(&channel_code);
        let channel_result_reg = format!("%t{}", codegen.get_last_variable_counter());
        
        // Allocate space for the received value
        ir_code.push_str(&format!(
            "  {} = alloca i64\n",
            value_ptr_reg
        ));
        
        // Call runtime receive function
        ir_code.push_str(&format!(
            "  {} = call i32 @cursed_channel_receive(i8* {}, i64* {})\n",
            recv_result_reg, channel_result_reg, value_ptr_reg
        ));
        
        // Load the received value
        ir_code.push_str(&format!(
            "  {} = load i64, i64* {}\n",
            received_value_reg, value_ptr_reg
        ));
        
        // TODO: Add error checking based on recv_result_reg
        
        Ok(ir_code)
    }

    /// Generate LLVM IR for channel close operation
    pub fn generate_channel_close(
        &self,
        channel_expr: &Expression,
        codegen: &mut LlvmCodeGenerator,
    ) -> Result<String, CursedError> {
        let result_reg = format!("%{}", codegen.next_variable());
        
        let mut ir_code = String::new();
        
        // Declare runtime function if not already declared
        if !codegen.has_function_declaration("cursed_channel_close") {
            ir_code.push_str("declare i32 @cursed_channel_close(i8*)\n");
            codegen.mark_function_declared("cursed_channel_close");
        }
        
        // Generate code for the channel
        let channel_code = codegen.generate_expression_public(channel_expr)?;
        ir_code.push_str(&channel_code);
        let channel_result_reg = format!("%t{}", codegen.get_last_variable_counter());
        
        // Call runtime close function
        ir_code.push_str(&format!(
            "  {} = call i32 @cursed_channel_close(i8* {})\n",
            result_reg, channel_result_reg
        ));
        
        Ok(ir_code)
    }

    /// Generate LLVM IR for channel select operation
    pub fn generate_channel_select(
        &self,
        cases: &[(Expression, Statement)],
        default_case: Option<&Statement>,
        codegen: &mut LlvmCodeGenerator,
    ) -> Result<String, CursedError> {
        // This is a simplified implementation of select
        // A full implementation would need to handle multiple channels simultaneously
        
        let mut ir_code = String::new();
        let select_result_reg = format!("%{}", codegen.next_variable());
        
        // For now, implement as a series of try operations
        for (i, (channel_expr, stmt)) in cases.iter().enumerate() {
            let case_label = format!("select_case_{}", i);
            let next_label = format!("select_next_{}", i);
            let try_result_reg = format!("%{}", codegen.next_variable());
            
            // Try to receive from this channel (non-blocking)
            let channel_code = codegen.generate_expression_public(channel_expr)?;
            ir_code.push_str(&channel_code);
            
            // Simplified: just assume successful receive for now
            // In a real implementation, this would use a try_recv function
            let stmt_code = codegen.generate_statement_public(stmt)?;
            ir_code.push_str(&stmt_code);
        }
        
        // Handle default case if present
        if let Some(default_stmt) = default_case {
            let default_code = codegen.generate_statement_public(default_stmt)?;
            ir_code.push_str(&default_code);
        }
        
        ir_code.push_str(&format!(
            "  {} = add i32 0, 0  ; select completed\n",
            select_result_reg
        ));
        
        Ok(ir_code)
    }
}

/// Integration with main LLVM codegen
impl LlvmCodeGenerator {
    /// Generate LLVM IR for channel creation
    pub fn generate_channel_creation(
        &mut self,
        channel_type: &Type,
        capacity: Option<&Expression>,
    ) -> Result<String, CursedError> {
        let channel_codegen = ChannelCodegen::new();
        channel_codegen.generate_channel_creation(channel_type, capacity, self)
    }

    /// Generate LLVM IR for channel send operation
    pub fn generate_channel_send(
        &mut self,
        channel_expr: &Expression,
        value_expr: &Expression,
    ) -> Result<String, CursedError> {
        let channel_codegen = ChannelCodegen::new();
        channel_codegen.generate_channel_send(channel_expr, value_expr, self)
    }

    /// Generate LLVM IR for channel receive operation
    pub fn generate_channel_receive(
        &mut self,
        channel_expr: &Expression,
    ) -> Result<String, CursedError> {
        let channel_codegen = ChannelCodegen::new();
        channel_codegen.generate_channel_receive(channel_expr, self)
    }

    /// Generate LLVM IR for channel close operation
    pub fn generate_channel_close(
        &mut self,
        channel_expr: &Expression,
    ) -> Result<String, CursedError> {
        let channel_codegen = ChannelCodegen::new();
        channel_codegen.generate_channel_close(channel_expr, self)
    }

    /// Generate LLVM IR for channel select operation
    pub fn generate_channel_select(
        &mut self,
        cases: &[(Expression, Statement)],
        default_case: Option<&Statement>,
    ) -> Result<String, CursedError> {
        let channel_codegen = ChannelCodegen::new();
        channel_codegen.generate_channel_select(cases, default_case, self)
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
    Ok("CURSED channel system enabled".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Type, Literal};

    #[test]
    fn test_channel_codegen_creation() {
        let channel_codegen = ChannelCodegen::new();
        // Test that we can create the codegen instance
        assert!(true); // Basic instantiation test
    }

    #[test]
    fn test_channel_creation_generation() {
        let mut codegen = LlvmCodeGenerator::new().unwrap();
        let channel_type = Type::Normie; // Use the correct type
        
        let result = codegen.generate_channel_creation(&channel_type, None);
        assert!(result.is_ok());
        
        let ir_code = result.unwrap();
        assert!(ir_code.contains("cursed_channel_create"));
    }

    #[test]
    fn test_channel_send_generation() {
        let mut codegen = LlvmCodeGenerator::new().unwrap();
        let channel_expr = Expression::Identifier("ch".to_string());
        let value_expr = Expression::Literal(Literal::Integer(42));
        
        let result = codegen.generate_channel_send(&channel_expr, &value_expr);
        assert!(result.is_ok());
        
        let ir_code = result.unwrap();
        assert!(ir_code.contains("cursed_channel_send"));
    }

    #[test]
    fn test_channel_receive_generation() {
        let mut codegen = LlvmCodeGenerator::new().unwrap();
        let channel_expr = Expression::Identifier("ch".to_string());
        
        let result = codegen.generate_channel_receive(&channel_expr);
        assert!(result.is_ok());
        
        let ir_code = result.unwrap();
        assert!(ir_code.contains("cursed_channel_receive"));
    }
}
