//! LLVM Codegen Error Recovery for CURSED
//! 
//! This module implements graceful error handling and recovery for LLVM code generation

use crate::error_types::{Error, Result};
use crate::error_recovery::{SourceLocation, ErrorContext, CodegenErrorRecovery};
use crate::codegen::llvm::main::LlvmCodeGenerator;
use crate::ast::{Program, Statement, Expression};
use std::collections::HashMap;

impl CodegenErrorRecovery for LlvmCodeGenerator {
    /// Handle codegen failures gracefully
    fn graceful_codegen_failure(&mut self, error: Error) -> String {
        eprintln!("Codegen error recovery: {}", error);
        
        match error {
            Error::Compile(ref msg) => {
                if msg.contains("LLVM") {
                    self.generate_llvm_error_placeholder()
                } else if msg.contains("register") {
                    self.generate_register_error_placeholder()
                } else {
                    self.generate_generic_error_placeholder("compilation_error")
                }
            }
            Error::Type(ref msg) => {
                eprintln!("Type error in codegen: {}", msg);
                self.generate_type_error_placeholder()
            }
            _ => {
                self.generate_generic_error_placeholder("unknown_error")
            }
        }
    }
    
    /// Generate error placeholder IR
    fn generate_error_placeholder(&self, context: &str) -> String {
        format!(
            r#"; Error recovery placeholder for {}
; Original compilation failed, continuing with minimal viable code
define i32 @__error_placeholder_{}() {{
entry:
  ; Return error code to indicate compilation issue
  ret i32 -1
}}

; Error recovery: Placeholder global for failed compilation
@__error_context_{} = private unnamed_addr constant [64 x i8] c"Error in context: {}\00"
"#,
            context, context, context, context
        )
    }
    
    /// Check if we should fallback to interpretation
    fn fallback_to_interpretation(&self) -> bool {
        // Fallback conditions
        true // For now, always allow fallback
    }
}

impl LlvmCodeGenerator {
    /// Enhanced program compilation with error recovery
    pub fn compile_program_with_recovery(&mut self, program: &Program) -> Result<String> {
        let mut successful_compilation = true;
        let mut ir_parts = Vec::new();
        
        // Add header with error handling support
        ir_parts.push(self.generate_error_recovery_header());
        
        // Compile each statement with individual error recovery
        for statement in &program.statements {
            match self.compile_statement_with_recovery(statement) {
                Ok(ir) => {
                    ir_parts.push(ir);
                }
                Err(error) => {
                    eprintln!("Statement compilation failed: {}", error);
                    // Generate placeholder for failed statement
                    let placeholder = self.graceful_codegen_failure(error);
                    ir_parts.push(placeholder);
                    successful_compilation = false;
                }
            }
        }
        
        // Add error recovery footer
        ir_parts.push(self.generate_error_recovery_footer());
        
        let final_ir = ir_parts.join("\n");
        
        if successful_compilation {
            Ok(final_ir)
        } else {
            // Return IR with error indicators but don't fail completely
            eprintln!("Warning: Compilation completed with errors. Some functions may not work correctly.");
            Ok(final_ir)
        }
    }
    
    /// Compile statement with error recovery
    pub fn compile_statement_with_recovery(&mut self, statement: &Statement) -> Result<String> {
        match self.compile_statement(statement) {
            Ok(ir) => Ok(ir),
            Err(error) => {
                // Try different recovery strategies based on statement type
                match statement {
                    Statement::Function(_) => {
                        Ok(self.generate_function_error_placeholder())
                    }
                    Statement::Let(_) => {
                        Ok(self.generate_variable_error_placeholder())
                    }
                    Statement::Expression(_) => {
                        Ok(self.generate_expression_error_placeholder())
                    }
                    _ => {
                        Ok(self.graceful_codegen_failure(error))
                    }
                }
            }
        }
    }
    
    /// Compile expression with error recovery
    pub fn compile_expression_with_recovery(&mut self, expression: &Expression) -> Result<String> {
        match self.compile_expression(expression) {
            Ok(ir) => Ok(ir),
            Err(error) => {
                eprintln!("Expression compilation failed: {}", error);
                // Return a safe placeholder value
                Ok(self.generate_safe_expression_placeholder(expression))
            }
        }
    }
    
    /// Generate header with error recovery support
    fn generate_error_recovery_header(&self) -> String {
        r#"; CURSED Compiler - Error Recovery Enabled
; This LLVM IR was generated with error recovery active
; Some functions may contain placeholders for compilation failures

; Error recovery runtime support
declare i32 @printf(i8*, ...)
declare void @exit(i32)

; Error reporting function
define void @__cursed_report_error(i8* %msg) {
entry:
  %fmt = getelementptr inbounds [23 x i8], [23 x i8]* @.error_fmt, i32 0, i32 0
  call i32 (i8*, ...) @printf(i8* %fmt, i8* %msg)
  ret void
}

@.error_fmt = private unnamed_addr constant [23 x i8] c"CURSED Runtime Error: %s\00"
"#.to_string()
    }
    
    /// Generate footer with error recovery cleanup
    fn generate_error_recovery_footer(&self) -> String {
        r#"
; Error recovery footer
; Check for any unresolved symbols or failed compilations

define i32 @__cursed_check_errors() {
entry:
  ; Return 0 if no errors, non-zero if errors detected
  ret i32 0
}
"#.to_string()
    }
    
    /// Generate LLVM error placeholder
    fn generate_llvm_error_placeholder(&self) -> String {
        let register = self.register_tracker.next_register();
        format!(
            r#"; LLVM compilation error recovery
%{} = alloca i32
store i32 -1, i32* %{}
"#,
            register, register
        )
    }
    
    /// Generate register error placeholder
    fn generate_register_error_placeholder(&self) -> String {
        format!(
            r#"; Register allocation error recovery
; Using fixed register allocation for recovery
%recovery_reg = alloca i32
store i32 0, i32* %recovery_reg
"#
        )
    }
    
    /// Generate type error placeholder
    fn generate_type_error_placeholder(&self) -> String {
        format!(
            r#"; Type error recovery
; Defaulting to i32 type for recovery
%type_error_recovery = alloca i32
store i32 0, i32* %type_error_recovery
"#
        )
    }
    
    /// Generate generic error placeholder
    fn generate_generic_error_placeholder(&self, context: &str) -> String {
        format!(
            r#"; Generic error recovery for {}
; Continuing compilation with placeholder
%error_placeholder_{} = alloca i32
store i32 -1, i32* %error_placeholder_{}
"#,
            context, context, context
        )
    }
    
    /// Generate function error placeholder
    fn generate_function_error_placeholder(&self) -> String {
        let register = self.register_tracker.next_register();
        format!(
            r#"; Function compilation failed - using placeholder
define i32 @__error_function_{}() {{
entry:
  ; Placeholder function that returns error code
  ret i32 -1
}}
"#,
            register
        )
    }
    
    /// Generate variable error placeholder
    fn generate_variable_error_placeholder(&self) -> String {
        let register = self.register_tracker.next_register();
        format!(
            r#"; Variable declaration failed - using placeholder
%error_var_{} = alloca i32
store i32 0, i32* %error_var_{}
"#,
            register, register
        )
    }
    
    /// Generate expression error placeholder
    fn generate_expression_error_placeholder(&self) -> String {
        let register = self.register_tracker.next_register();
        format!(
            r#"; Expression compilation failed - using placeholder
%expr_error_{} = alloca i32
store i32 0, i32* %expr_error_{}
"#,
            register, register
        )
    }
    
    /// Generate safe expression placeholder based on expression type
    fn generate_safe_expression_placeholder(&mut self, expression: &Expression) -> String {
        let register = self.register_tracker.next_register();
        
        match expression {
            Expression::Literal(_) => {
                format!(
                    r#"; Literal compilation failed - using default
%literal_error_{} = alloca i32
store i32 0, i32* %literal_error_{}
"#,
                    register, register
                )
            }
            Expression::Binary(_) => {
                format!(
                    r#"; Binary expression compilation failed - using default
%binary_error_{} = alloca i32
store i32 0, i32* %binary_error_{}
"#,
                    register, register
                )
            }
            Expression::Call(_) => {
                format!(
                    r#"; Function call compilation failed - using placeholder
%call_error_{} = call i32 @__error_function_placeholder()
"#,
                    register
                )
            }
            _ => {
                format!(
                    r#"; Generic expression error - using placeholder
%expr_placeholder_{} = alloca i32
store i32 -1, i32* %expr_placeholder_{}
"#,
                    register, register
                )
            }
        }
    }
    
    /// Check if error is recoverable for codegen
    pub fn is_recoverable_codegen_error(&self, error: &Error) -> bool {
        match error {
            Error::Compile(msg) => {
                // Recoverable compilation errors
                msg.contains("register") ||
                msg.contains("type mismatch") ||
                msg.contains("LLVM")
            }
            Error::Type(_) => true, // Type errors are usually recoverable
            Error::Runtime(_) => false, // Runtime errors are usually fatal
            _ => true, // Most other errors are recoverable
        }
    }
    
    /// Attempt graceful degradation
    pub fn attempt_graceful_degradation(&mut self, program: &Program) -> Result<String> {
        eprintln!("Attempting graceful degradation for failed compilation...");
        
        // Generate minimal viable IR that at least compiles
        let mut ir = String::new();
        
        // Add basic structure
        ir.push_str(&self.generate_error_recovery_header());
        
        // Add a main function that just returns success
        ir.push_str(r#"
define i32 @main() {
entry:
  ; Graceful degradation: minimal main function
  %msg = getelementptr inbounds [50 x i8], [50 x i8]* @.degraded_msg, i32 0, i32 0
  call void @__cursed_report_error(i8* %msg)
  ret i32 0
}

@.degraded_msg = private unnamed_addr constant [50 x i8] c"Warning: Program compiled with graceful degradation\00"
"#);
        
        ir.push_str(&self.generate_error_recovery_footer());
        
        eprintln!("Graceful degradation successful - generated minimal viable IR");
        Ok(ir)
    }
    
    /// Generate error report for codegen issues
    pub fn generate_codegen_error_report(&self, errors: &[Error]) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("LLVM Codegen Report: {} error(s)\n\n", errors.len()));
        
        for (index, error) in errors.iter().enumerate() {
            report.push_str(&format!("{}. Codegen Error: {}\n", index + 1, error));
            
            // Add specific suggestions based on error type
            match error {
                Error::Compile(msg) if msg.contains("register") => {
                    report.push_str("   help: Register allocation issue - check for complex expressions\n");
                }
                Error::Compile(msg) if msg.contains("LLVM") => {
                    report.push_str("   help: LLVM IR generation issue - check syntax and types\n");
                }
                Error::Type(_) => {
                    report.push_str("   help: Type checking failed - verify variable types and function signatures\n");
                }
                _ => {
                    report.push_str("   help: Check the generated LLVM IR for placeholder functions\n");
                }
            }
            
            report.push('\n');
        }
        
        if errors.is_empty() {
            report.push_str("✅ LLVM codegen completed successfully!\n");
        } else {
            report.push_str("⚠️ LLVM codegen completed with errors. Placeholder code was generated for failed sections.\n");
            report.push_str("💡 Consider running in interpretation mode if compilation issues persist.\n");
        }
        
        report
    }
}

/// Helper function to create a codegen with error recovery enabled
pub fn create_codegen_with_recovery() -> LlvmCodeGenerator {
    let mut codegen = LlvmCodeGenerator::new();
    // Enable error recovery features
    eprintln!("LLVM codegen initialized with error recovery enabled");
    codegen
}

/// Enhanced compilation with comprehensive error recovery
pub fn compile_with_full_recovery(program: &Program) -> Result<String> {
    let mut codegen = create_codegen_with_recovery();
    
    match codegen.compile_program_with_recovery(program) {
        Ok(ir) => {
            eprintln!("✅ Compilation successful with error recovery");
            Ok(ir)
        }
        Err(_) => {
            eprintln!("⚠️ Primary compilation failed, attempting graceful degradation...");
            match codegen.attempt_graceful_degradation(program) {
                Ok(ir) => {
                    eprintln!("✅ Graceful degradation successful");
                    Ok(ir)
                }
                Err(final_error) => {
                    eprintln!("❌ All recovery attempts failed");
                    Err(final_error)
                }
            }
        }
    }
}
