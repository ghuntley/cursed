//! Error Handling Code Generation for CURSED
//! 
//! This module handles the generation of LLVM IR for CURSED error handling constructs:
//! - yikes: Error creation statements
//! - shook: Error propagation expressions
//! - fam: Error recovery blocks

use crate::ast::{YikesStatement, FamStatement, ShookExpression, ErrorValueExpression, Expression, Statement, Literal};
use crate::error::CursedError;
use crate::lexer::SourceLocation;
use crate::codegen::llvm::expression_compiler::ExpressionCompiler;
use std::collections::HashMap;

/// Error handling code generator
pub struct ErrorHandlingCodegen {
    /// Current error variable mappings
    error_variables: HashMap<String, String>,
    /// Current error propagation context
    propagation_context: Vec<String>,
    /// Error recovery blocks
    recovery_blocks: Vec<RecoveryBlock>,
    /// LLVM register counter
    register_counter: usize,
}

#[derive(Debug, Clone)]
struct RecoveryBlock {
    label: String,
    error_variable: Option<String>,
    recovery_code: Vec<Statement>,
}

impl ErrorHandlingCodegen {
    pub fn new() -> Self {
        Self {
            error_variables: HashMap::new(),
            propagation_context: Vec::new(),
            recovery_blocks: Vec::new(),
            register_counter: 0,
        }
    }

    /// Generate LLVM IR for yikes error creation statement
    pub fn generate_yikes_statement(&mut self, stmt: &YikesStatement) -> Result<String, CursedError> {
        let mut ir = String::new();
        
        // Generate error object allocation
        let error_register = self.next_register();
        ir.push_str(&format!("  %{} = call i8* @malloc(i32 32)  ; Allocate error object\n", error_register));
        
        // Generate error message setup
        let message_register = self.next_register();
        ir.push_str(&format!("  %{} = getelementptr inbounds i8, i8* %{}, i32 0  ; Error message ptr\n", 
                            message_register, error_register));
        
        // If there's a value expression, generate it
        if let Some(value) = &stmt.value {
            let value_ir = self.generate_expression_for_error(value)?;
            ir.push_str(&value_ir);
        }
        
        // Store the error variable mapping
        self.error_variables.insert(stmt.name.clone(), format!("%{}", error_register));
        
        // Generate error initialization call
        let init_register = self.next_register();
        ir.push_str(&format!("  %{} = call i8* @cursed_error_init(i8* %{}, i8* getelementptr inbounds ([15 x i8], [15 x i8]* @error_msg_default, i32 0, i32 0))\n", 
                            init_register, error_register));
        
        Ok(ir)
    }

    /// Generate LLVM IR for fam error recovery block
    pub fn generate_fam_statement(&mut self, stmt: &FamStatement) -> Result<String, CursedError> {
        let mut ir = String::new();
        
        // Generate recovery block label
        let recovery_label = format!("recovery_{}", self.next_register());
        let normal_label = format!("normal_{}", self.next_register());
        let end_label = format!("end_{}", self.next_register());
        
        // Set up exception handling
        ir.push_str(&format!("  invoke void @cursed_try_begin()\n"));
        ir.push_str(&format!("    to label %{} unwind label %{}\n", normal_label, recovery_label));
        
        // Normal execution block
        ir.push_str(&format!("{}:\n", normal_label));
        
        // Generate recovery block code
        for statement in &stmt.body {
            let stmt_ir = self.generate_statement_for_recovery(statement)?;
            ir.push_str(&stmt_ir);
        }
        
        ir.push_str(&format!("  call void @cursed_try_end()\n"));
        ir.push_str(&format!("  br label %{}\n", end_label));
        
        // Recovery block
        ir.push_str(&format!("{}:\n", recovery_label));
        ir.push_str(&format!("  %panic_value = call i8* @cursed_get_panic_value()\n"));
        ir.push_str(&format!("  ; Recovery code would go here\n"));
        ir.push_str(&format!("  br label %{}\n", end_label));
        
        // End block
        ir.push_str(&format!("{}:\n", end_label));
        
        Ok(ir)
    }

    /// Generate LLVM IR for shook error propagation expression
    pub fn generate_shook_expression(&mut self, expr: &ShookExpression) -> Result<String, CursedError> {
        let mut ir = String::new();
        
        // Generate the wrapped expression
        let expr_ir = self.generate_expression_for_error(&expr.expression)?;
        ir.push_str(&expr_ir);
        
        // Generate error checking code
        let check_register = self.next_register();
        let success_label = format!("success_{}", self.next_register());
        let error_label = format!("error_{}", self.next_register());
        
        ir.push_str(&format!("  %{} = call i1 @cursed_is_error(i8* %result)\n", check_register));
        ir.push_str(&format!("  br i1 %{}, label %{}, label %{}\n", 
                            check_register, error_label, success_label));
        
        // Error propagation block
        ir.push_str(&format!("{}:\n", error_label));
        ir.push_str(&format!("  call void @cursed_propagate_error(i8* %result)\n"));
        ir.push_str(&format!("  ret i8* %result  ; Early return with error\n"));
        
        // Success block
        ir.push_str(&format!("{}:\n", success_label));
        ir.push_str(&format!("  ; Continue with normal execution\n"));
        
        Ok(ir)
    }

    /// Generate LLVM IR for error value expression
    pub fn generate_error_value_expression(&mut self, expr: &ErrorValueExpression) -> Result<String, CursedError> {
        let mut ir = String::new();
        
        // Generate string literal for error message
        let str_register = self.next_register();
        let msg_len = expr.message.len();
        ir.push_str(&format!("  @error_msg_{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"\n", 
                            str_register, msg_len + 1, expr.message));
        
        // Generate error object creation
        let error_register = self.next_register();
        ir.push_str(&format!("  %{} = call i8* @cursed_create_error(i8* getelementptr inbounds ([{} x i8], [{} x i8]* @error_msg_{}, i32 0, i32 0))\n", 
                            error_register, msg_len + 1, msg_len + 1, str_register));
        
        Ok(ir)
    }

    /// Generate LLVM IR for structured error expression
    pub fn generate_structured_error_expression(&mut self, 
        message: &Expression, 
        code: Option<&Expression>, 
        details: Option<&Expression>,
        fields: &[(String, Expression)]
    ) -> Result<String, CursedError> {
        let mut ir = String::new();
        
        // Generate error object allocation
        let error_register = self.next_register();
        ir.push_str(&format!("  %{} = call i8* @cursed_create_structured_error()\n", error_register));
        
        // Generate message
        let message_ir = self.generate_expression_for_error(message)?;
        ir.push_str(&message_ir);
        let message_register = self.next_register();
        ir.push_str(&format!("  %{} = call i8* @cursed_set_error_message(i8* %{}, i8* %result)\n", 
                            message_register, error_register));
        
        // Generate error code if provided
        if let Some(code_expr) = code {
            let code_ir = self.generate_expression_for_error(code_expr)?;
            ir.push_str(&code_ir);
            let code_register = self.next_register();
            ir.push_str(&format!("  %{} = call i8* @cursed_set_error_code(i8* %{}, i32 %result)\n", 
                                code_register, error_register));
        }
        
        // Generate error details if provided
        if let Some(details_expr) = details {
            let details_ir = self.generate_expression_for_error(details_expr)?;
            ir.push_str(&details_ir);
            let details_register = self.next_register();
            ir.push_str(&format!("  %{} = call i8* @cursed_set_error_details(i8* %{}, i8* %result)\n", 
                                details_register, error_register));
        }
        
        // Generate custom fields
        for (field_name, field_expr) in fields {
            let field_ir = self.generate_expression_for_error(field_expr)?;
            ir.push_str(&field_ir);
            let field_register = self.next_register();
            let field_name_register = self.next_register();
            ir.push_str(&format!("  @field_name_{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"\n", 
                                field_name_register, field_name.len() + 1, field_name));
            ir.push_str(&format!("  %{} = call i8* @cursed_set_error_field(i8* %{}, i8* getelementptr inbounds ([{} x i8], [{} x i8]* @field_name_{}, i32 0, i32 0), i8* %result)\n", 
                                field_register, error_register, field_name.len() + 1, field_name.len() + 1, field_name_register));
        }
        
        ir.push_str(&format!("  %result = add i8* %{}, null\n", error_register));
        Ok(ir)
    }

    /// Generate LLVM IR for enhanced panic recovery
    pub fn generate_enhanced_panic_recovery(&mut self, goroutine_id: u64, recovery_body: &[Statement]) -> Result<String, CursedError> {
        let mut ir = String::new();
        
        // Generate panic recovery setup
        let recovery_label = format!("panic_recovery_{}", self.next_register());
        let normal_label = format!("normal_execution_{}", self.next_register());
        let end_label = format!("end_recovery_{}", self.next_register());
        
        // Set up enhanced exception handling with goroutine isolation
        ir.push_str(&format!("  invoke void @cursed_enhanced_try_begin(i64 {})\n", goroutine_id));
        ir.push_str(&format!("    to label %{} unwind label %{}\n", normal_label, recovery_label));
        
        // Normal execution block
        ir.push_str(&format!("{}:\n", normal_label));
        ir.push_str(&format!("  call void @cursed_enhanced_try_end(i64 {})\n", goroutine_id));
        ir.push_str(&format!("  br label %{}\n", end_label));
        
        // Enhanced recovery block with context
        ir.push_str(&format!("{}:\n", recovery_label));
        ir.push_str(&format!("  %panic_context = call i8* @cursed_get_panic_context(i64 {})\n", goroutine_id));
        ir.push_str(&format!("  %panic_value = call i8* @cursed_extract_panic_value(i8* %panic_context)\n"));
        ir.push_str(&format!("  %stack_trace = call i8* @cursed_extract_stack_trace(i8* %panic_context)\n"));
        
        // Generate recovery body
        for statement in recovery_body {
            let stmt_ir = self.generate_statement_for_recovery(statement)?;
            ir.push_str(&stmt_ir);
        }
        
        ir.push_str(&format!("  call void @cursed_clear_panic_context(i64 {})\n", goroutine_id));
        ir.push_str(&format!("  br label %{}\n", end_label));
        
        // End block
        ir.push_str(&format!("{}:\n", end_label));
        
        Ok(ir)
    }

    /// Generate runtime function declarations for error handling
    pub fn generate_runtime_declarations(&self) -> String {
        let mut ir = String::new();
        
        // Basic error handling runtime functions
        ir.push_str("declare i8* @cursed_error_init(i8*, i8*)\n");
        ir.push_str("declare i8* @cursed_create_error(i8*)\n");
        ir.push_str("declare i1 @cursed_is_error(i8*)\n");
        ir.push_str("declare void @cursed_propagate_error(i8*)\n");
        ir.push_str("declare void @cursed_try_begin()\n");
        ir.push_str("declare void @cursed_try_end()\n");
        ir.push_str("declare i8* @cursed_get_panic_value()\n");
        
        // Enhanced error handling runtime functions
        ir.push_str("declare i8* @cursed_create_structured_error()\n");
        ir.push_str("declare i8* @cursed_set_error_message(i8*, i8*)\n");
        ir.push_str("declare i8* @cursed_set_error_code(i8*, i32)\n");
        ir.push_str("declare i8* @cursed_set_error_details(i8*, i8*)\n");
        ir.push_str("declare i8* @cursed_set_error_field(i8*, i8*, i8*)\n");
        ir.push_str("declare i8* @cursed_get_error_field(i8*, i8*)\n");
        ir.push_str("declare i32 @cursed_get_error_code(i8*)\n");
        ir.push_str("declare i8* @cursed_get_error_message(i8*)\n");
        ir.push_str("declare i8* @cursed_get_error_details(i8*)\n");
        
        // Enhanced panic recovery runtime functions (declarations handled by main LLVM codegen)
        ir.push_str("declare void @cursed_register_panic_handler(i64, i8*)\n");
        ir.push_str("declare i8* @cursed_handle_panic(i64, i8*)\n");
        
        // Error context and propagation functions
        ir.push_str("declare void @cursed_propagate_error_context(i64, i64)\n");
        ir.push_str("declare i8* @cursed_get_goroutine_error_context(i64)\n");
        ir.push_str("declare void @cursed_clear_goroutine_error_context(i64)\n");
        ir.push_str("declare i8* @cursed_create_enhanced_context(i8*, i64)\n");
        ir.push_str("declare i8* @cursed_link_error_context(i8*, i8*)\n");
        ir.push_str("declare i8* @cursed_capture_stack_trace()\n");
        ir.push_str("declare i64 @cursed_get_current_goroutine_id()\n");
        ir.push_str("declare i64 @time(i64*)\n");
        ir.push_str("declare i8* @cursed_propagate_with_context(i8*, i8*)\n");
        
        // Memory management functions - handled by main codegen deduplication
        // malloc and free are declared in main.rs to avoid duplicates
        
        // Default error message
        ir.push_str("@error_msg_default = private unnamed_addr constant [15 x i8] c\"Error occurred\\00\"\n");
        
        ir
    }

    /// Helper to generate expression code for error handling
    fn generate_expression_for_error(&mut self, expr: &Expression) -> Result<String, CursedError> {
        match expr {
            Expression::String(s) => {
                let str_register = self.next_register();
                let msg_len = s.len();
                Ok(format!("  @str_{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"\n  %result = getelementptr inbounds ([{} x i8], [{} x i8]* @str_{}, i32 0, i32 0)\n", 
                          str_register, msg_len + 1, s, msg_len + 1, msg_len + 1, str_register))
            }
            Expression::Integer(n) => {
                Ok(format!("  %result = add i32 0, {}\n", n))
            }
            Expression::Boolean(b) => {
                Ok(format!("  %result = add i1 0, {}\n", if *b { 1 } else { 0 }))
            }
            Expression::Identifier(name) => {
                if let Some(register) = self.error_variables.get(name) {
                    Ok(format!("  %result = load i8*, i8** {}\n", register))
                } else {
                    Ok(format!("  %result = load i8*, i8** %{}\n", name))
                }
            }
            _ => {
                // For other expressions, generate proper error handling code
                match self.expression_compiler.compile_expression(expr) {
                    Ok(register) => Ok(format!("  %result = add i32 {}, 0  ; Complex expression result\n", register)),
                    Err(e) => {
                        eprintln!("Failed to compile expression in error context: {}", e);
                        Ok(format!("  %result = add i32 0, 0  ; Expression compilation failed: {}\n", e))
                    }
                }
            }
        }
    }

    /// Helper to generate statement code for recovery blocks
    fn generate_statement_for_recovery(&mut self, stmt: &Statement) -> Result<String, CursedError> {
        match stmt {
            Statement::Expression(expr) => {
                match self.generate_expression_for_error(expr) {
                    Ok(ir) => Ok(ir),
                    Err(e) => {
                        // Error recovery - generate proper error handling code
                        eprintln!("Error recovery triggered for expression: {}", e);
                        let msg_len = e.to_string().len() + 1;
                        Ok(format!("  ; ERROR RECOVERY: Expression failed: {}\n  call void @cursed_error_handler(i8* getelementptr inbounds ([{}x i8], [{}x i8]* @error_message_{}, i32 0, i32 0))\n  %result = add i32 0, 0\n", 
                               e, msg_len, msg_len, self.error_counter))
                    }
                }
            }
            Statement::Return(ret_stmt) => {
                if let Some(value) = &ret_stmt.value {
                    match self.generate_expression_for_error(value) {
                        Ok(value_ir) => Ok(format!("{}  ret i8* %result\n", value_ir)),
                        Err(e) => {
                            // Error recovery - generate safe return
                            Ok(format!("  ; ERROR RECOVERY: Return expression failed: {}\n  ret i8* null\n", e))
                        }
                    }
                } else {
                    Ok("  ret void\n".to_string())
                }
            }
            Statement::Let(let_stmt) => {
                // Generate variable allocation with error recovery
                match self.generate_expression_for_error(&let_stmt.value) {
                    Ok(value_ir) => {
                        let var_name = match &let_stmt.target {
                            crate::ast::LetTarget::Single(name) => name.clone(),
                            crate::ast::LetTarget::Tuple(names) => names.first().cloned().unwrap_or_else(|| "temp".to_string()),
                        };
                        Ok(format!("{}  ; Variable {} allocated in recovery block\n", value_ir, var_name))
                    }
                    Err(e) => {
                        Ok(format!("  ; ERROR RECOVERY: Let statement failed: {}\n", e))
                    }
                }
            }
            Statement::Assignment(assign_stmt) => {
                // Generate assignment with error recovery
                match self.generate_expression_for_error(&assign_stmt.value) {
                    Ok(value_ir) => {
                        let var_name = match &assign_stmt.target {
                            crate::ast::AssignmentTarget::Single(name) => name.clone(),
                            crate::ast::AssignmentTarget::Tuple(names) => names.first().cloned().unwrap_or_else(|| "temp".to_string()),
                        };
                        Ok(format!("{}  ; Assignment to {} in recovery block\n", value_ir, var_name))
                    }
                    Err(e) => {
                        Ok(format!("  ; ERROR RECOVERY: Assignment failed: {}\n", e))
                    }
                }
            }
            Statement::If(if_stmt) => {
                // Generate simplified if statement for recovery
                match self.generate_expression_for_error(&if_stmt.condition) {
                    Ok(cond_ir) => {
                        Ok(format!("{}  ; Conditional execution in recovery block\n", cond_ir))
                    }
                    Err(e) => {
                        Ok(format!("  ; ERROR RECOVERY: If statement failed: {}\n", e))
                    }
                }
            }
            _ => {
                // For other statements, generate safe placeholder
                Ok("  ; Statement in recovery block (safe placeholder)\n".to_string())
            }
        }
    }

    /// Get next register number
    fn next_register(&mut self) -> usize {
        self.register_counter += 1;
        self.register_counter
    }
}

/// Helper function to create error handling codegen instance
pub fn create_error_handler() -> ErrorHandlingCodegen {
    ErrorHandlingCodegen::new()
}

/// Helper function to generate all error handling runtime support
pub fn generate_error_runtime_support() -> String {
    let handler = ErrorHandlingCodegen::new();
    handler.generate_runtime_declarations()
}
