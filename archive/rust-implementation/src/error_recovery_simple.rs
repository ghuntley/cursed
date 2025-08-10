//! Simplified Error Recovery System for CURSED Compiler
//! 
//! This module provides a simplified but robust error recovery system

use crate::error_types::{Error, Result};
use std::fmt;

/// Simplified error recovery manager
pub struct SimpleErrorRecovery {
    pub errors: Vec<RecoveryError>,
    pub warnings: Vec<RecoveryError>,
    pub max_errors: usize,
}

/// Simplified recovery error
#[derive(Debug, Clone)]
pub struct RecoveryError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub error_type: String,
    pub suggestion: Option<String>,
}

impl SimpleErrorRecovery {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            max_errors: 50,
        }
    }
    
    /// Add an error with location and suggestion
    pub fn add_error(&mut self, message: String, line: usize, column: usize, suggestion: Option<String>) {
        let error = RecoveryError {
            message,
            line,
            column,
            error_type: "error".to_string(),
            suggestion,
        };
        self.errors.push(error);
    }
    
    /// Add a warning
    pub fn add_warning(&mut self, message: String, line: usize, column: usize, suggestion: Option<String>) {
        let warning = RecoveryError {
            message,
            line,
            column,
            error_type: "warning".to_string(),
            suggestion,
        };
        self.warnings.push(warning);
    }
    
    /// Check if we should continue compilation
    pub fn should_continue(&self) -> bool {
        self.errors.len() < self.max_errors
    }
    
    /// Generate a comprehensive report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        let error_count = self.errors.len();
        let warning_count = self.warnings.len();
        
        report.push_str(&format!("Compilation Summary: {} error(s), {} warning(s)\n\n", error_count, warning_count));
        
        // Report errors
        for (i, error) in self.errors.iter().enumerate() {
            report.push_str(&format!("{}. Error at line {}, column {}: {}\n", 
                i + 1, error.line, error.column, error.message));
            
            if let Some(ref suggestion) = error.suggestion {
                report.push_str(&format!("   suggestion: {}\n", suggestion));
            }
            report.push('\n');
        }
        
        // Report warnings
        for (i, warning) in self.warnings.iter().enumerate() {
            report.push_str(&format!("{}. Warning at line {}, column {}: {}\n", 
                i + 1, warning.line, warning.column, warning.message));
            
            if let Some(ref suggestion) = warning.suggestion {
                report.push_str(&format!("   suggestion: {}\n", suggestion));
            }
            report.push('\n');
        }
        
        // Summary
        if error_count == 0 && warning_count == 0 {
            report.push_str("✅ Compilation successful!\n");
        } else if error_count == 0 {
            report.push_str("✅ Compilation successful with warnings.\n");
        } else {
            report.push_str("❌ Compilation failed. Please fix the errors above.\n");
            report.push_str("💡 You can try running in interpretation mode for partial execution.\n");
        }
        
        report
    }
}

impl fmt::Display for RecoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at line {}, column {}: {}", 
               self.error_type, self.line, self.column, self.message)
    }
}

impl Default for SimpleErrorRecovery {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to suggest fixes for common errors
pub fn suggest_fix_for_error(error_msg: &str) -> Option<String> {
    if error_msg.contains("Expected") {
        if error_msg.contains("')'") {
            Some("Add missing closing parenthesis ')'".to_string())
        } else if error_msg.contains("';'") {
            Some("Add semicolon ';' at the end of the statement".to_string())
        } else if error_msg.contains("identifier") {
            Some("Add a valid variable or function name".to_string())
        } else {
            Some("Check syntax and add missing tokens".to_string())
        }
    } else if error_msg.contains("undefined variable") {
        Some("Declare the variable with 'sus variable_name type_name = value'".to_string())
    } else if error_msg.contains("undefined function") {
        Some("Define the function or check if you need to import a module".to_string())
    } else if error_msg.contains("type mismatch") {
        Some("Check that the types are compatible or use type conversion".to_string())
    } else if error_msg.contains("vibez.spill") && !error_msg.contains("(") {
        Some("Function calls require parentheses: vibez.spill(\"message\")".to_string())
    } else {
        None
    }
}

/// Enhanced parser that can continue after errors
pub trait ErrorRecoveringParser {
    fn parse_with_recovery(&mut self) -> (Result<crate::ast::Program>, SimpleErrorRecovery);
    fn synchronize_after_error(&mut self);
}

/// Enhanced type checker that accumulates errors
pub trait ErrorRecoveringTypeChecker {
    fn check_with_recovery(&mut self, program: &crate::ast::Program) -> (Result<()>, SimpleErrorRecovery);
    fn can_continue_after_error(&self, error: &Error) -> bool;
}

/// Enhanced codegen that provides fallbacks
pub trait ErrorRecoveringCodegen {
    fn compile_with_recovery(&mut self, program: &crate::ast::Program) -> (Result<String>, SimpleErrorRecovery);
    fn generate_safe_fallback(&self) -> String;
}

/// Main error recovery orchestrator
pub struct ErrorRecoveryOrchestrator {
    pub recovery: SimpleErrorRecovery,
}

impl ErrorRecoveryOrchestrator {
    pub fn new() -> Self {
        Self {
            recovery: SimpleErrorRecovery::new(),
        }
    }
    
    /// Orchestrate compilation with full error recovery
    pub fn compile_with_full_recovery(&mut self, source: &str) -> CompilationResult {
        let mut result = CompilationResult::new();
        
        // Step 1: Parsing with recovery
        result.parsing_success = self.parse_with_recovery_step(source);
        
        // Step 2: Semantic analysis with recovery (if parsing succeeded)
        if result.parsing_success {
            result.semantic_success = self.semantic_analysis_with_recovery_step();
        }
        
        // Step 3: Code generation with recovery (if semantic analysis succeeded)
        if result.semantic_success {
            result.codegen_success = self.codegen_with_recovery_step();
        }
        
        // Generate final report
        result.final_report = self.recovery.generate_report();
        result.should_continue = self.recovery.should_continue();
        
        result
    }
    
    fn parse_with_recovery_step(&mut self, _source: &str) -> bool {
        // This would integrate with the actual parser
        // For now, just demonstrate the concept
        self.recovery.add_error(
            "Example parse error: Expected ';' after statement".to_string(),
            10, 5,
            Some("Add semicolon at the end of the statement".to_string())
        );
        
        // Continue parsing even after errors
        true
    }
    
    fn semantic_analysis_with_recovery_step(&mut self) -> bool {
        // This would integrate with the type checker
        self.recovery.add_warning(
            "Type mismatch: expected 'normie', found 'tea'".to_string(),
            15, 8,
            Some("Use type conversion or check variable types".to_string())
        );
        
        // Continue semantic analysis
        true
    }
    
    fn codegen_with_recovery_step(&mut self) -> bool {
        // This would integrate with LLVM codegen
        self.recovery.add_warning(
            "Complex expression simplified for compilation".to_string(),
            20, 12,
            Some("Consider breaking complex expressions into simpler parts".to_string())
        );
        
        // Continue with codegen, possibly with simplified IR
        true
    }
}

/// Result of compilation with error recovery
#[derive(Debug)]
pub struct CompilationResult {
    pub parsing_success: bool,
    pub semantic_success: bool,
    pub codegen_success: bool,
    pub should_continue: bool,
    pub final_report: String,
}

impl CompilationResult {
    pub fn new() -> Self {
        Self {
            parsing_success: false,
            semantic_success: false,
            codegen_success: false,
            should_continue: true,
            final_report: String::new(),
        }
    }
    
    pub fn overall_success(&self) -> bool {
        self.parsing_success && self.semantic_success && self.codegen_success
    }
    
    pub fn can_execute(&self) -> bool {
        self.parsing_success && self.semantic_success
    }
}

impl Default for CompilationResult {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ErrorRecoveryOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility function to test error recovery with sample programs
pub fn test_error_recovery_system() -> String {
    let mut orchestrator = ErrorRecoveryOrchestrator::new();
    
    // Test with sample erroneous code
    let sample_code = r#"
        sus x normie = 42  # Missing semicolon
        vibez.spill("hello world"  # Missing closing paren
        vibez.spill(undefined_var)  # Undefined variable
        sus y tea = 100  # Type mismatch
        vibez.spill("This should still work")
    "#;
    
    let result = orchestrator.compile_with_full_recovery(sample_code);
    
    format!(
        "Error Recovery Test Results:\n\
         Parsing: {}\n\
         Semantic Analysis: {}\n\
         Code Generation: {}\n\
         Overall Success: {}\n\
         Can Execute: {}\n\n\
         Detailed Report:\n\
         {}",
        result.parsing_success,
        result.semantic_success,
        result.codegen_success,
        result.overall_success(),
        result.can_execute(),
        result.final_report
    )
}
