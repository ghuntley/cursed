/// LLVM CursedError Handling Integration for CURSED
///
/// This module provides LLVM code generation support for the comprehensive
/// error handling system, including:
/// - Compilation of panic/recovery constructs
/// - Generation of `?` operator code
/// - Integration of error handling with function calls
/// - Debug info emission for error contexts

use crate::error::{CursedError, SourceLocation};
use crate::runtime::error_handling::{ErrorRuntime, ErrorContext};
use crate::runtime::panic::{PanicRuntime, CursedPanicInfo, PanicSeverity, PanicCategory};
use crate::runtime::stack_trace::{StackTraceManager, CallFrame};
// use crate::runtime::runtime_error::{RuntimeError, ErrorSeverity, ErrorCategory};
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression_compiler::{LlvmValue, LlvmType};

use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, error, info, instrument, warn};

/// LLVM integration for error handling
pub trait ErrorHandlingCompiler<'ctx> {
    /// Compile a panic statement
    fn compile_panic_statement(
    ) -> crate::error::Result<()>;

    /// Compile a recovery block
    fn compile_recovery_block<F>(
    ) -> crate::error::Result<()>
    where
        F: FnOnce(&mut Self) -> crate::error::Result<()>;

    /// Compile error propagation for the `?` operator
    fn compile_error_propagation(
    ) -> crate::error::Result<()>;

    /// Generate error checking code
    fn generate_error_check(
    ) -> crate::error::Result<()>;

    /// Generate stack trace capture
    fn generate_stack_trace_capture(
    ) -> crate::error::Result<()>;

    /// Generate error context creation
    fn generate_error_context(
    ) -> crate::error::Result<()>;
/// CursedError handling function registry for LLVM
#[derive(Debug)]
pub struct ErrorHandlingFunctions {
    /// Function declarations for error handling
/// Individual error handling function descriptor
#[derive(Debug, Clone)]
pub struct ErrorHandlingFunction {
    /// Function name in LLVM IR
    /// Function description
    /// Parameter types
    /// Return type
    /// Whether function can raise errors
    /// LLVM IR template for the function
impl ErrorHandlingFunctions {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        
        // Register core error handling functions
        functions.insert("cursed_panic".to_string(), ErrorHandlingFunction {
            parameters: vec![
                LlvmType::String, // message
                LlvmType::Boolean, // severity (i8 -> i1)
                LlvmType::Boolean, // category (i8 -> i1)
                LlvmType::Int32, // line
                LlvmType::Int32, // column
                LlvmType::String, // file
            can_error: false, // Panics instead of returning errors
        });

        functions.insert("cursed_propagate_error".to_string(), ErrorHandlingFunction {
            parameters: vec![
                LlvmType::String, // error_message
                LlvmType::Int32, // error_code
                LlvmType::Int32, // line
                LlvmType::Int32, // column
                LlvmType::String, // file
                LlvmType::String, // function
            return_type: LlvmType::Boolean, // 0 = success, 1 = error
        });

        functions.insert("cursed_stack_capture".to_string(), ErrorHandlingFunction {
            parameters: vec![
                LlvmType::Int32, // max_depth
            return_type: LlvmType::Pointer(Box::new(LlvmType::Boolean)), // Pointer to stack trace
        });

        functions.insert("cursed_create_error_context".to_string(), ErrorHandlingFunction {
            parameters: vec![
                LlvmType::String, // error_message
                LlvmType::Int32, // line
                LlvmType::Int32, // column
                LlvmType::String, // file
                LlvmType::String, // function
            return_type: LlvmType::Pointer(Box::new(LlvmType::Boolean)), // Pointer to error context
        });

        functions.insert("cursed_is_in_error_handling".to_string(), ErrorHandlingFunction {
            return_type: LlvmType::Boolean, // 0 = false, 1 = true
        });

        functions.insert("cursed_clear_error_context".to_string(), ErrorHandlingFunction {
        });

        ErrorHandlingFunctions { functions }
    }

    pub fn get_function(&self, name: &str) -> Option<&ErrorHandlingFunction> {
        self.functions.get(name)
    pub fn generate_declarations(&self) -> String {
        let mut declarations = vec![
        ];

        for function in self.functions.values() {
            declarations.push(format!("; {}", function.description));
            declarations.push(function.ir_template.clone());
            declarations.push("".to_string());
        declarations.join("\n")
    }
}

impl Default for ErrorHandlingFunctions {
    fn default() -> Self {
        Self::new()
    }
}

/// CursedError handling code patterns for LLVM
pub struct ErrorHandlingPatterns;

impl ErrorHandlingPatterns {
    /// Generate LLVM IR for panic statement
    pub fn generate_panic_ir(
    ) -> String {
        let mut ir = Vec::new();
        
        // Convert message to LLVM string
        let message_var = format!("%panic_msg_{}", temp_counter);
        *temp_counter += 1;
        let message_len = message.len();
        
        ir.push(format!("{} = alloca [{}x i8], align 1", message_var, message_len + 1));
                       message_len + 1, message, message_len + 1, message_var));
        
        // Convert location to parameters
        let (line, column, file_var, file_len) = if let Some(ref loc) = location {
            let file_var = format!("%panic_file_{}", temp_counter);
            *temp_counter += 1;
            let file_name = loc.file.as_deref().unwrap_or("unknown");
            let file_len = file_name.len();
            
            ir.push(format!("{} = alloca [{}x i8], align 1", file_var, file_len + 1));
                           file_len + 1, file_name, file_len + 1, file_var));
            
            (loc.line as u32, loc.column as u32, file_var, file_len)
        } else {
            let file_var = format!("%panic_file_{}", temp_counter);
            *temp_counter += 1;
            ir.push(format!("{} = alloca [8 x i8], align 1", file_var));
            ir.push(format!("store [8 x i8] c\"unknown\\00\", [8 x i8]* {}, align 1", file_var));
            (0, 0, file_var, 7)
        
        // Convert severity and category to integers
        let severity_val = match severity {
        
        let category_val = match category {
        
        // Generate function call
        let msg_ptr = format!("%panic_msg_ptr_{}", temp_counter);
        let file_ptr = format!("%panic_file_ptr_{}", temp_counter);
        *temp_counter += 2;
        
                       msg_ptr, message_len + 1, message_len + 1, message_var));
                       file_ptr, file_len + 1, file_len + 1, file_var));
        
                       msg_ptr, message_len, severity_val, category_val, line, column, file_ptr, file_len));
        
        ir.push("unreachable".to_string());
        
        ir.join("\n  ")
    /// Generate LLVM IR for error propagation (? operator)
    pub fn generate_error_propagation_ir(
    ) -> String {
        let mut ir = Vec::new();
        
        // Convert error message to LLVM string
        let msg_var = format!("%error_msg_{}", temp_counter);
        *temp_counter += 1;
        let msg_len = error_message.len();
        
        ir.push(format!("{} = alloca [{}x i8], align 1", msg_var, msg_len + 1));
                       msg_len + 1, error_message, msg_len + 1, msg_var));
        
        // Handle location
        let (line, column, file_var, file_len) = if let Some(loc) = location {
            let file_var = format!("%error_file_{}", temp_counter);
            *temp_counter += 1;
            let file_name = loc.file.as_deref().unwrap_or("unknown");
            let file_len = file_name.len();
            
            ir.push(format!("{} = alloca [{}x i8], align 1", file_var, file_len + 1));
                           file_len + 1, file_name, file_len + 1, file_var));
            
            (loc.line as u32, loc.column as u32, file_var, file_len)
        } else {
            let file_var = format!("%error_file_{}", temp_counter);
            *temp_counter += 1;
            ir.push(format!("{} = alloca [8 x i8], align 1", file_var));
            ir.push(format!("store [8 x i8] c\"unknown\\00\", [8 x i8]* {}, align 1", file_var));
            (0, 0, file_var, 7)
        
        // Handle function name
        let (func_var, func_len) = if let Some(func_name) = function_name {
            let func_var = format!("%error_func_{}", temp_counter);
            *temp_counter += 1;
            let func_len = func_name.len();
            
            ir.push(format!("{} = alloca [{}x i8], align 1", func_var, func_len + 1));
                           func_len + 1, func_name, func_len + 1, func_var));
            
            (func_var, func_len)
        } else {
            let func_var = format!("%error_func_{}", temp_counter);
            *temp_counter += 1;
            ir.push(format!("{} = alloca [8 x i8], align 1", func_var));
            ir.push(format!("store [8 x i8] c\"unknown\\00\", [8 x i8]* {}, align 1", func_var));
            (func_var, 7)
        
        // Generate pointer variables
        let msg_ptr = format!("%error_msg_ptr_{}", temp_counter);
        let file_ptr = format!("%error_file_ptr_{}", temp_counter);
        let func_ptr = format!("%error_func_ptr_{}", temp_counter);
        let result_var = format!("%error_result_{}", temp_counter);
        *temp_counter += 4;
        
                       msg_ptr, msg_len + 1, msg_len + 1, msg_var));
                       file_ptr, file_len + 1, file_len + 1, file_var));
                       func_ptr, func_len + 1, func_len + 1, func_var));
        
        // Make the function call
                       result_var, msg_ptr, msg_len, error_code, line, column, file_ptr, file_len, func_ptr, func_len));
        
        ir.join("\n  ")
    /// Generate LLVM IR for stack trace capture
    pub fn generate_stack_trace_capture_ir(
    ) -> String {
        let depth = max_depth.unwrap_or(100);
        let result_var = format!("%stack_trace_{}", temp_counter);
        *temp_counter += 1;
        
        format!("{} = call i8* @cursed_stack_capture(i32 {})", result_var, depth)
    /// Generate LLVM IR for error context creation
    pub fn generate_error_context_ir(
    ) -> String {
        let mut ir = Vec::new();
        
        // Similar to error propagation but for context creation
        let msg_var = format!("%ctx_msg_{}", temp_counter);
        *temp_counter += 1;
        let msg_len = error_message.len();
        
        ir.push(format!("{} = alloca [{}x i8], align 1", msg_var, msg_len + 1));
                       msg_len + 1, error_message, msg_len + 1, msg_var));
        
        // Handle location and function name (similar to propagation)
        let (line, column, file_var, file_len) = if let Some(loc) = location {
            let file_var = format!("%ctx_file_{}", temp_counter);
            *temp_counter += 1;
            let file_name = loc.file.as_deref().unwrap_or("unknown");
            let file_len = file_name.len();
            
            ir.push(format!("{} = alloca [{}x i8], align 1", file_var, file_len + 1));
                           file_len + 1, file_name, file_len + 1, file_var));
            
            (loc.line as u32, loc.column as u32, file_var, file_len)
        } else {
            let file_var = format!("%ctx_file_{}", temp_counter);
            *temp_counter += 1;
            ir.push(format!("{} = alloca [8 x i8], align 1", file_var));
            ir.push(format!("store [8 x i8] c\"unknown\\00\", [8 x i8]* {}, align 1", file_var));
            (0, 0, file_var, 7)
        
        let (func_var, func_len) = if let Some(func_name) = function_name {
            let func_var = format!("%ctx_func_{}", temp_counter);
            *temp_counter += 1;
            let func_len = func_name.len();
            
            ir.push(format!("{} = alloca [{}x i8], align 1", func_var, func_len + 1));
                           func_len + 1, func_name, func_len + 1, func_var));
            
            (func_var, func_len)
        } else {
            let func_var = format!("%ctx_func_{}", temp_counter);
            *temp_counter += 1;
            ir.push(format!("{} = alloca [8 x i8], align 1", func_var));
            ir.push(format!("store [8 x i8] c\"unknown\\00\", [8 x i8]* {}, align 1", func_var));
            (func_var, 7)
        
        // Generate pointers and call
        let msg_ptr = format!("%ctx_msg_ptr_{}", temp_counter);
        let file_ptr = format!("%ctx_file_ptr_{}", temp_counter);
        let func_ptr = format!("%ctx_func_ptr_{}", temp_counter);
        let result_var = format!("%ctx_result_{}", temp_counter);
        *temp_counter += 4;
        
                       msg_ptr, msg_len + 1, msg_len + 1, msg_var));
                       file_ptr, file_len + 1, file_len + 1, file_var));
                       func_ptr, func_len + 1, func_len + 1, func_var));
        
                       result_var, msg_ptr, msg_len, line, column, file_ptr, file_len, func_ptr, func_len));
        
        ir.join("\n  ")
    /// Generate LLVM IR for conditional error checking
    pub fn generate_error_check_ir(
    ) -> String {
        let check_var = format!("%error_check_{}", temp_counter);
        *temp_counter += 1;
        
        format!(
            check_var, value_name, check_var, success_label, error_label
        )
    }
}

/// Integration helper for LLVM code generator
pub struct ErrorHandlingIntegration {
    /// Function registry
    /// Pattern generator
    /// Temporary variable counter
impl ErrorHandlingIntegration {
    pub fn new() -> Self {
        ErrorHandlingIntegration {
        }
    }

    /// Generate all function declarations
    pub fn generate_function_declarations(&self) -> String {
        self.functions.generate_declarations()
    /// Generate panic statement LLVM IR
    pub fn generate_panic(
    ) -> String {
        ErrorHandlingPatterns::generate_panic_ir(
        )
    /// Generate error propagation LLVM IR
    pub fn generate_error_propagation(
    ) -> String {
        ErrorHandlingPatterns::generate_error_propagation_ir(
        )
    /// Generate stack trace capture LLVM IR
    pub fn generate_stack_trace_capture(&mut self, max_depth: Option<usize>) -> String {
        ErrorHandlingPatterns::generate_stack_trace_capture_ir(
        )
    /// Generate error context creation LLVM IR
    pub fn generate_error_context(
    ) -> String {
        ErrorHandlingPatterns::generate_error_context_ir(
        )
    /// Generate error checking LLVM IR
    pub fn generate_error_check(
    ) -> String {
        ErrorHandlingPatterns::generate_error_check_ir(
        )
    }
}

impl Default for ErrorHandlingIntegration {
    fn default() -> Self {
        Self::new()
    }
}

