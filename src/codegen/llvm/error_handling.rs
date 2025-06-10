/// LLVM Error Handling Integration for CURSED
///
/// This module provides LLVM code generation support for the comprehensive
/// error handling system, including:
/// - Compilation of panic/recovery constructs
/// - Generation of `?` operator code
/// - Integration of error handling with function calls
/// - Debug info emission for error contexts

use crate::error::{Error as CursedError, SourceLocation};
use crate::runtime::error_handling::{ErrorRuntime, ErrorContext};
use crate::runtime::panic::{PanicRuntime, CursedPanicInfo, PanicSeverity, PanicCategory};
use crate::runtime::stack_trace::{StackTraceManager, CallFrame};
use crate::runtime::runtime_error::{RuntimeError, ErrorSeverity, ErrorCategory};
use crate::codegen::llvm::{LlvmCodeGenerator, LlvmValue, LlvmType};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, error, info, instrument, warn};

/// LLVM integration for error handling
pub trait ErrorHandlingCompiler<'ctx> {
    /// Compile a panic statement
    fn compile_panic_statement(
        &mut self,
        message: &str,
        severity: PanicSeverity,
        category: PanicCategory,
        location: Option<SourceLocation>,
    ) -> Result<(), CursedError>;

    /// Compile a recovery block
    fn compile_recovery_block<F>(
        &mut self,
        protected_operation: F,
        recovery_handler: Option<F>,
        location: Option<SourceLocation>,
    ) -> Result<LlvmValue, CursedError>
    where
        F: FnOnce(&mut Self) -> Result<LlvmValue, CursedError>;

    /// Compile error propagation for the `?` operator
    fn compile_error_propagation(
        &mut self,
        result_value: LlvmValue,
        result_type: LlvmType,
        location: Option<SourceLocation>,
        function_name: Option<String>,
    ) -> Result<LlvmValue, CursedError>;

    /// Generate error checking code
    fn generate_error_check(
        &mut self,
        value: LlvmValue,
        value_type: LlvmType,
    ) -> Result<LlvmValue, CursedError>;

    /// Generate stack trace capture
    fn generate_stack_trace_capture(
        &mut self,
        max_depth: Option<usize>,
    ) -> Result<LlvmValue, CursedError>;

    /// Generate error context creation
    fn generate_error_context(
        &mut self,
        error_message: &str,
        location: Option<SourceLocation>,
        function_name: Option<String>,
    ) -> Result<LlvmValue, CursedError>;
}

/// Error handling function registry for LLVM
#[derive(Debug)]
pub struct ErrorHandlingFunctions {
    /// Function declarations for error handling
    pub functions: HashMap<String, ErrorHandlingFunction>,
}

/// Individual error handling function descriptor
#[derive(Debug, Clone)]
pub struct ErrorHandlingFunction {
    /// Function name in LLVM IR
    pub llvm_name: String,
    /// Function description
    pub description: String,
    /// Parameter types
    pub parameters: Vec<LlvmType>,
    /// Return type
    pub return_type: LlvmType,
    /// Whether function can raise errors
    pub can_error: bool,
    /// LLVM IR template for the function
    pub ir_template: String,
}

impl ErrorHandlingFunctions {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        
        // Register core error handling functions
        functions.insert("cursed_panic".to_string(), ErrorHandlingFunction {
            llvm_name: "cursed_panic".to_string(),
            description: "Trigger a CURSED panic with detailed information".to_string(),
            parameters: vec![
                LlvmType::String, // message
                LlvmType::Integer(8), // severity
                LlvmType::Integer(8), // category
                LlvmType::Integer(32), // line
                LlvmType::Integer(32), // column
                LlvmType::String, // file
            ],
            return_type: LlvmType::Void,
            can_error: false, // Panics instead of returning errors
            ir_template: "declare void @cursed_panic(i8* %message, i64 %message_len, i8 %severity, i8 %category, i32 %line, i32 %column, i8* %file, i64 %file_len)".to_string(),
        });

        functions.insert("cursed_propagate_error".to_string(), ErrorHandlingFunction {
            llvm_name: "cursed_propagate_error".to_string(),
            description: "Propagate an error using ? operator semantics".to_string(),
            parameters: vec![
                LlvmType::String, // error_message
                LlvmType::Integer(32), // error_code
                LlvmType::Integer(32), // line
                LlvmType::Integer(32), // column
                LlvmType::String, // file
                LlvmType::String, // function
            ],
            return_type: LlvmType::Integer(8), // 0 = success, 1 = error
            can_error: true,
            ir_template: "declare i8 @cursed_propagate_error(i8* %message, i64 %message_len, i32 %error_code, i32 %line, i32 %column, i8* %file, i64 %file_len, i8* %function, i64 %function_len)".to_string(),
        });

        functions.insert("cursed_stack_capture".to_string(), ErrorHandlingFunction {
            llvm_name: "cursed_stack_capture".to_string(),
            description: "Capture current stack trace".to_string(),
            parameters: vec![
                LlvmType::Integer(32), // max_depth
            ],
            return_type: LlvmType::Pointer, // Pointer to stack trace
            can_error: true,
            ir_template: "declare i8* @cursed_stack_capture(i32 %max_depth)".to_string(),
        });

        functions.insert("cursed_create_error_context".to_string(), ErrorHandlingFunction {
            llvm_name: "cursed_create_error_context".to_string(),
            description: "Create error context for propagation".to_string(),
            parameters: vec![
                LlvmType::String, // error_message
                LlvmType::Integer(32), // line
                LlvmType::Integer(32), // column
                LlvmType::String, // file
                LlvmType::String, // function
            ],
            return_type: LlvmType::Pointer, // Pointer to error context
            can_error: true,
            ir_template: "declare i8* @cursed_create_error_context(i8* %message, i64 %message_len, i32 %line, i32 %column, i8* %file, i64 %file_len, i8* %function, i64 %function_len)".to_string(),
        });

        functions.insert("cursed_is_in_error_handling".to_string(), ErrorHandlingFunction {
            llvm_name: "cursed_is_in_error_handling".to_string(),
            description: "Check if current thread is in error handling mode".to_string(),
            parameters: vec![],
            return_type: LlvmType::Integer(8), // 0 = false, 1 = true
            can_error: false,
            ir_template: "declare i8 @cursed_is_in_error_handling()".to_string(),
        });

        functions.insert("cursed_clear_error_context".to_string(), ErrorHandlingFunction {
            llvm_name: "cursed_clear_error_context".to_string(),
            description: "Clear error context for current thread".to_string(),
            parameters: vec![],
            return_type: LlvmType::Void,
            can_error: false,
            ir_template: "declare void @cursed_clear_error_context()".to_string(),
        });

        ErrorHandlingFunctions { functions }
    }

    pub fn get_function(&self, name: &str) -> Option<&ErrorHandlingFunction> {
        self.functions.get(name)
    }

    pub fn generate_declarations(&self) -> String {
        let mut declarations = vec![
            "; CURSED Error Handling Runtime Functions".to_string(),
            "".to_string(),
        ];

        for function in self.functions.values() {
            declarations.push(format!("; {}", function.description));
            declarations.push(function.ir_template.clone());
            declarations.push("".to_string());
        }

        declarations.join("\n")
    }
}

impl Default for ErrorHandlingFunctions {
    fn default() -> Self {
        Self::new()
    }
}

/// Error handling code patterns for LLVM
pub struct ErrorHandlingPatterns;

impl ErrorHandlingPatterns {
    /// Generate LLVM IR for panic statement
    pub fn generate_panic_ir(
        message: &str,
        severity: PanicSeverity,
        category: PanicCategory,
        location: Option<SourceLocation>,
        temp_counter: &mut usize,
    ) -> String {
        let mut ir = Vec::new();
        
        // Convert message to LLVM string
        let message_var = format!("%panic_msg_{}", temp_counter);
        *temp_counter += 1;
        let message_len = message.len();
        
        ir.push(format!("{} = alloca [{}x i8], align 1", message_var, message_len + 1));
        ir.push(format!("store [{}x i8] c\"{}\\00\", [{}x i8]* {}, align 1", 
                       message_len + 1, message, message_len + 1, message_var));
        
        // Convert location to parameters
        let (line, column, file_var) = if let Some(loc) = location {
            let file_var = format!("%panic_file_{}", temp_counter);
            *temp_counter += 1;
            let file_name = loc.file.as_deref().unwrap_or("unknown");
            let file_len = file_name.len();
            
            ir.push(format!("{} = alloca [{}x i8], align 1", file_var, file_len + 1));
            ir.push(format!("store [{}x i8] c\"{}\\00\", [{}x i8]* {}, align 1", 
                           file_len + 1, file_name, file_len + 1, file_var));
            
            (loc.line as u32, loc.column as u32, file_var)
        } else {
            let file_var = format!("%panic_file_{}", temp_counter);
            *temp_counter += 1;
            ir.push(format!("{} = alloca [8 x i8], align 1", file_var));
            ir.push(format!("store [8 x i8] c\"unknown\\00\", [8 x i8]* {}, align 1", file_var));
            (0, 0, file_var)
        };
        
        // Convert severity and category to integers
        let severity_val = match severity {
            PanicSeverity::Recoverable => 0,
            PanicSeverity::Critical => 1,
            PanicSeverity::Fatal => 2,
        };
        
        let category_val = match category {
            PanicCategory::Memory => 0,
            PanicCategory::TypeAssertion => 1,
            PanicCategory::BoundsCheck => 2,
            PanicCategory::Arithmetic => 3,
            PanicCategory::Channel => 4,
            PanicCategory::Goroutine => 5,
            PanicCategory::User => 6,
            PanicCategory::System => 7,
            PanicCategory::Generic => 8,
        };
        
        // Generate function call
        let msg_ptr = format!("%panic_msg_ptr_{}", temp_counter);
        let file_ptr = format!("%panic_file_ptr_{}", temp_counter);
        *temp_counter += 2;
        
        ir.push(format!("{} = getelementptr inbounds [{}x i8], [{}x i8]* {}, i32 0, i32 0", 
                       msg_ptr, message_len + 1, message_len + 1, message_var));
        ir.push(format!("{} = getelementptr inbounds [{}x i8], [{}x i8]* {}, i32 0, i32 0", 
                       file_ptr, if location.is_some() { location.as_ref().unwrap().file.as_deref().unwrap_or("unknown").len() + 1 } else { 8 }, 
                       if location.is_some() { location.as_ref().unwrap().file.as_deref().unwrap_or("unknown").len() + 1 } else { 8 }, file_var));
        
        ir.push(format!("call void @cursed_panic(i8* {}, i64 {}, i8 {}, i8 {}, i32 {}, i32 {}, i8* {}, i64 {})",
                       msg_ptr, message_len, severity_val, category_val, line, column, file_ptr, 
                       if location.is_some() { location.as_ref().unwrap().file.as_deref().unwrap_or("unknown").len() } else { 7 }));
        
        ir.push("unreachable".to_string());
        
        ir.join("\n  ")
    }

    /// Generate LLVM IR for error propagation (? operator)
    pub fn generate_error_propagation_ir(
        error_message: &str,
        error_code: u32,
        location: Option<SourceLocation>,
        function_name: Option<String>,
        temp_counter: &mut usize,
    ) -> String {
        let mut ir = Vec::new();
        
        // Convert error message to LLVM string
        let msg_var = format!("%error_msg_{}", temp_counter);
        *temp_counter += 1;
        let msg_len = error_message.len();
        
        ir.push(format!("{} = alloca [{}x i8], align 1", msg_var, msg_len + 1));
        ir.push(format!("store [{}x i8] c\"{}\\00\", [{}x i8]* {}, align 1", 
                       msg_len + 1, error_message, msg_len + 1, msg_var));
        
        // Handle location
        let (line, column, file_var, file_len) = if let Some(loc) = location {
            let file_var = format!("%error_file_{}", temp_counter);
            *temp_counter += 1;
            let file_name = loc.file.as_deref().unwrap_or("unknown");
            let file_len = file_name.len();
            
            ir.push(format!("{} = alloca [{}x i8], align 1", file_var, file_len + 1));
            ir.push(format!("store [{}x i8] c\"{}\\00\", [{}x i8]* {}, align 1", 
                           file_len + 1, file_name, file_len + 1, file_var));
            
            (loc.line as u32, loc.column as u32, file_var, file_len)
        } else {
            let file_var = format!("%error_file_{}", temp_counter);
            *temp_counter += 1;
            ir.push(format!("{} = alloca [8 x i8], align 1", file_var));
            ir.push(format!("store [8 x i8] c\"unknown\\00\", [8 x i8]* {}, align 1", file_var));
            (0, 0, file_var, 7)
        };
        
        // Handle function name
        let (func_var, func_len) = if let Some(func_name) = function_name {
            let func_var = format!("%error_func_{}", temp_counter);
            *temp_counter += 1;
            let func_len = func_name.len();
            
            ir.push(format!("{} = alloca [{}x i8], align 1", func_var, func_len + 1));
            ir.push(format!("store [{}x i8] c\"{}\\00\", [{}x i8]* {}, align 1", 
                           func_len + 1, func_name, func_len + 1, func_var));
            
            (func_var, func_len)
        } else {
            let func_var = format!("%error_func_{}", temp_counter);
            *temp_counter += 1;
            ir.push(format!("{} = alloca [8 x i8], align 1", func_var));
            ir.push(format!("store [8 x i8] c\"unknown\\00\", [8 x i8]* {}, align 1", func_var));
            (func_var, 7)
        };
        
        // Generate pointer variables
        let msg_ptr = format!("%error_msg_ptr_{}", temp_counter);
        let file_ptr = format!("%error_file_ptr_{}", temp_counter);
        let func_ptr = format!("%error_func_ptr_{}", temp_counter);
        let result_var = format!("%error_result_{}", temp_counter);
        *temp_counter += 4;
        
        ir.push(format!("{} = getelementptr inbounds [{}x i8], [{}x i8]* {}, i32 0, i32 0", 
                       msg_ptr, msg_len + 1, msg_len + 1, msg_var));
        ir.push(format!("{} = getelementptr inbounds [{}x i8], [{}x i8]* {}, i32 0, i32 0", 
                       file_ptr, file_len + 1, file_len + 1, file_var));
        ir.push(format!("{} = getelementptr inbounds [{}x i8], [{}x i8]* {}, i32 0, i32 0", 
                       func_ptr, func_len + 1, func_len + 1, func_var));
        
        // Make the function call
        ir.push(format!("{} = call i8 @cursed_propagate_error(i8* {}, i64 {}, i32 {}, i32 {}, i32 {}, i8* {}, i64 {}, i8* {}, i64 {})",
                       result_var, msg_ptr, msg_len, error_code, line, column, file_ptr, file_len, func_ptr, func_len));
        
        ir.join("\n  ")
    }

    /// Generate LLVM IR for stack trace capture
    pub fn generate_stack_trace_capture_ir(
        max_depth: Option<usize>,
        temp_counter: &mut usize,
    ) -> String {
        let depth = max_depth.unwrap_or(100);
        let result_var = format!("%stack_trace_{}", temp_counter);
        *temp_counter += 1;
        
        format!("{} = call i8* @cursed_stack_capture(i32 {})", result_var, depth)
    }

    /// Generate LLVM IR for error context creation
    pub fn generate_error_context_ir(
        error_message: &str,
        location: Option<SourceLocation>,
        function_name: Option<String>,
        temp_counter: &mut usize,
    ) -> String {
        let mut ir = Vec::new();
        
        // Similar to error propagation but for context creation
        let msg_var = format!("%ctx_msg_{}", temp_counter);
        *temp_counter += 1;
        let msg_len = error_message.len();
        
        ir.push(format!("{} = alloca [{}x i8], align 1", msg_var, msg_len + 1));
        ir.push(format!("store [{}x i8] c\"{}\\00\", [{}x i8]* {}, align 1", 
                       msg_len + 1, error_message, msg_len + 1, msg_var));
        
        // Handle location and function name (similar to propagation)
        let (line, column, file_var, file_len) = if let Some(loc) = location {
            let file_var = format!("%ctx_file_{}", temp_counter);
            *temp_counter += 1;
            let file_name = loc.file.as_deref().unwrap_or("unknown");
            let file_len = file_name.len();
            
            ir.push(format!("{} = alloca [{}x i8], align 1", file_var, file_len + 1));
            ir.push(format!("store [{}x i8] c\"{}\\00\", [{}x i8]* {}, align 1", 
                           file_len + 1, file_name, file_len + 1, file_var));
            
            (loc.line as u32, loc.column as u32, file_var, file_len)
        } else {
            let file_var = format!("%ctx_file_{}", temp_counter);
            *temp_counter += 1;
            ir.push(format!("{} = alloca [8 x i8], align 1", file_var));
            ir.push(format!("store [8 x i8] c\"unknown\\00\", [8 x i8]* {}, align 1", file_var));
            (0, 0, file_var, 7)
        };
        
        let (func_var, func_len) = if let Some(func_name) = function_name {
            let func_var = format!("%ctx_func_{}", temp_counter);
            *temp_counter += 1;
            let func_len = func_name.len();
            
            ir.push(format!("{} = alloca [{}x i8], align 1", func_var, func_len + 1));
            ir.push(format!("store [{}x i8] c\"{}\\00\", [{}x i8]* {}, align 1", 
                           func_len + 1, func_name, func_len + 1, func_var));
            
            (func_var, func_len)
        } else {
            let func_var = format!("%ctx_func_{}", temp_counter);
            *temp_counter += 1;
            ir.push(format!("{} = alloca [8 x i8], align 1", func_var));
            ir.push(format!("store [8 x i8] c\"unknown\\00\", [8 x i8]* {}, align 1", func_var));
            (func_var, 7)
        };
        
        // Generate pointers and call
        let msg_ptr = format!("%ctx_msg_ptr_{}", temp_counter);
        let file_ptr = format!("%ctx_file_ptr_{}", temp_counter);
        let func_ptr = format!("%ctx_func_ptr_{}", temp_counter);
        let result_var = format!("%ctx_result_{}", temp_counter);
        *temp_counter += 4;
        
        ir.push(format!("{} = getelementptr inbounds [{}x i8], [{}x i8]* {}, i32 0, i32 0", 
                       msg_ptr, msg_len + 1, msg_len + 1, msg_var));
        ir.push(format!("{} = getelementptr inbounds [{}x i8], [{}x i8]* {}, i32 0, i32 0", 
                       file_ptr, file_len + 1, file_len + 1, file_var));
        ir.push(format!("{} = getelementptr inbounds [{}x i8], [{}x i8]* {}, i32 0, i32 0", 
                       func_ptr, func_len + 1, func_len + 1, func_var));
        
        ir.push(format!("{} = call i8* @cursed_create_error_context(i8* {}, i64 {}, i32 {}, i32 {}, i8* {}, i64 {}, i8* {}, i64 {})",
                       result_var, msg_ptr, msg_len, line, column, file_ptr, file_len, func_ptr, func_len));
        
        ir.join("\n  ")
    }

    /// Generate LLVM IR for conditional error checking
    pub fn generate_error_check_ir(
        value_name: &str,
        error_label: &str,
        success_label: &str,
        temp_counter: &mut usize,
    ) -> String {
        let check_var = format!("%error_check_{}", temp_counter);
        *temp_counter += 1;
        
        format!(
            "{} = icmp eq i8 {}, 0\n  br i1 {}, label %{}, label %{}",
            check_var, value_name, check_var, success_label, error_label
        )
    }
}

/// Integration helper for LLVM code generator
pub struct ErrorHandlingIntegration {
    /// Function registry
    pub functions: ErrorHandlingFunctions,
    /// Pattern generator
    pub patterns: ErrorHandlingPatterns,
    /// Temporary variable counter
    pub temp_counter: usize,
}

impl ErrorHandlingIntegration {
    pub fn new() -> Self {
        ErrorHandlingIntegration {
            functions: ErrorHandlingFunctions::new(),
            patterns: ErrorHandlingPatterns,
            temp_counter: 0,
        }
    }

    /// Generate all function declarations
    pub fn generate_function_declarations(&self) -> String {
        self.functions.generate_declarations()
    }

    /// Generate panic statement LLVM IR
    pub fn generate_panic(
        &mut self,
        message: &str,
        severity: PanicSeverity,
        category: PanicCategory,
        location: Option<SourceLocation>,
    ) -> String {
        ErrorHandlingPatterns::generate_panic_ir(
            message,
            severity,
            category,
            location,
            &mut self.temp_counter,
        )
    }

    /// Generate error propagation LLVM IR
    pub fn generate_error_propagation(
        &mut self,
        error_message: &str,
        error_code: u32,
        location: Option<SourceLocation>,
        function_name: Option<String>,
    ) -> String {
        ErrorHandlingPatterns::generate_error_propagation_ir(
            error_message,
            error_code,
            location,
            function_name,
            &mut self.temp_counter,
        )
    }

    /// Generate stack trace capture LLVM IR
    pub fn generate_stack_trace_capture(&mut self, max_depth: Option<usize>) -> String {
        ErrorHandlingPatterns::generate_stack_trace_capture_ir(
            max_depth,
            &mut self.temp_counter,
        )
    }

    /// Generate error context creation LLVM IR
    pub fn generate_error_context(
        &mut self,
        error_message: &str,
        location: Option<SourceLocation>,
        function_name: Option<String>,
    ) -> String {
        ErrorHandlingPatterns::generate_error_context_ir(
            error_message,
            location,
            function_name,
            &mut self.temp_counter,
        )
    }

    /// Generate error checking LLVM IR
    pub fn generate_error_check(
        &mut self,
        value_name: &str,
        error_label: &str,
        success_label: &str,
    ) -> String {
        ErrorHandlingPatterns::generate_error_check_ir(
            value_name,
            error_label,
            success_label,
            &mut self.temp_counter,
        )
    }
}

impl Default for ErrorHandlingIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_handling_functions_registry() {
        let functions = ErrorHandlingFunctions::new();
        
        assert!(functions.get_function("cursed_panic").is_some());
        assert!(functions.get_function("cursed_propagate_error").is_some());
        assert!(functions.get_function("cursed_stack_capture").is_some());
        assert!(functions.get_function("nonexistent").is_none());
        
        let declarations = functions.generate_declarations();
        assert!(declarations.contains("declare void @cursed_panic"));
        assert!(declarations.contains("declare i8 @cursed_propagate_error"));
    }

    #[test]
    fn test_panic_ir_generation() {
        let mut temp_counter = 0;
        let location = Some(SourceLocation::new(10, 5).with_file("test.csd"));
        
        let ir = ErrorHandlingPatterns::generate_panic_ir(
            "Test panic",
            PanicSeverity::Critical,
            PanicCategory::User,
            location,
            &mut temp_counter,
        );
        
        assert!(ir.contains("call void @cursed_panic"));
        assert!(ir.contains("Test panic"));
        assert!(ir.contains("unreachable"));
        assert!(temp_counter > 0);
    }

    #[test]
    fn test_error_propagation_ir_generation() {
        let mut temp_counter = 0;
        let location = Some(SourceLocation::new(15, 10).with_file("test.csd"));
        let function_name = Some("test_function".to_string());
        
        let ir = ErrorHandlingPatterns::generate_error_propagation_ir(
            "Test error",
            42,
            location,
            function_name,
            &mut temp_counter,
        );
        
        assert!(ir.contains("call i8 @cursed_propagate_error"));
        assert!(ir.contains("Test error"));
        assert!(ir.contains("test_function"));
        assert!(temp_counter > 0);
    }

    #[test]
    fn test_stack_trace_capture_ir() {
        let mut temp_counter = 0;
        
        let ir = ErrorHandlingPatterns::generate_stack_trace_capture_ir(
            Some(50),
            &mut temp_counter,
        );
        
        assert!(ir.contains("call i8* @cursed_stack_capture"));
        assert!(ir.contains("i32 50"));
        assert!(temp_counter > 0);
    }

    #[test]
    fn test_error_context_ir_generation() {
        let mut temp_counter = 0;
        let location = Some(SourceLocation::new(20, 15).with_file("context_test.csd"));
        let function_name = Some("context_function".to_string());
        
        let ir = ErrorHandlingPatterns::generate_error_context_ir(
            "Context error",
            location,
            function_name,
            &mut temp_counter,
        );
        
        assert!(ir.contains("call i8* @cursed_create_error_context"));
        assert!(ir.contains("Context error"));
        assert!(ir.contains("context_function"));
        assert!(temp_counter > 0);
    }

    #[test]
    fn test_error_check_ir_generation() {
        let mut temp_counter = 0;
        
        let ir = ErrorHandlingPatterns::generate_error_check_ir(
            "%result",
            "error_block",
            "success_block",
            &mut temp_counter,
        );
        
        assert!(ir.contains("icmp eq i8 %result, 0"));
        assert!(ir.contains("br i1"));
        assert!(ir.contains("error_block"));
        assert!(ir.contains("success_block"));
        assert!(temp_counter > 0);
    }

    #[test]
    fn test_error_handling_integration() {
        let mut integration = ErrorHandlingIntegration::new();
        
        let declarations = integration.generate_function_declarations();
        assert!(declarations.contains("CURSED Error Handling Runtime Functions"));
        
        let panic_ir = integration.generate_panic(
            "Integration test panic",
            PanicSeverity::Fatal,
            PanicCategory::System,
            Some(SourceLocation::new(25, 20)),
        );
        assert!(ir.contains("Integration test panic"));
        
        let propagation_ir = integration.generate_error_propagation(
            "Integration test error",
            100,
            Some(SourceLocation::new(30, 25)),
            Some("integration_test".to_string()),
        );
        assert!(propagation_ir.contains("Integration test error"));
        
        // Check that temp counter is advancing
        assert!(integration.temp_counter > 0);
    }

    #[test]
    fn test_function_descriptors() {
        let functions = ErrorHandlingFunctions::new();
        
        let panic_func = functions.get_function("cursed_panic").unwrap();
        assert_eq!(panic_func.return_type, LlvmType::Void);
        assert!(!panic_func.can_error); // Panics instead
        
        let propagate_func = functions.get_function("cursed_propagate_error").unwrap();
        assert_eq!(propagate_func.return_type, LlvmType::Integer(8));
        assert!(propagate_func.can_error);
        
        let capture_func = functions.get_function("cursed_stack_capture").unwrap();
        assert_eq!(capture_func.return_type, LlvmType::Pointer);
        assert!(capture_func.can_error);
    }
}
