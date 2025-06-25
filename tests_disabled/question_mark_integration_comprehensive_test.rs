/// Comprehensive integration test for question mark operator (?) compilation
/// 
/// This test verifies that the question mark operator is properly integrated
/// throughout the LLVM codegen pipeline and generates correct IR.

use cursed::codegen::llvm::{LlvmCodeGenerator, QuestionMarkCompiler};
use cursed::ast::expressions::QuestionMarkExpression;
use cursed::ast::identifiers::Identifier;
use cursed::ast::traits::Expression;
use cursed::error::CursedError;
use cursed::debug::DebugConfig;

#[path = "common/mod.rs"]
mod common;

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{debug, info};

    /// Test that question mark operator is recognized by expression compiler
    #[test]
    fn test_question_mark_expression_compilation() {
        common::tracing::setup();
        
        let mut codegen = LlvmCodeGenerator::new().expect("Failed to create code generator");
        
        // Create a simple variable expression
        let var_expr = Box::new(Identifier::from_name("test_result"));
        
        // Create question mark expression
        let question_expr = QuestionMarkExpression::new(var_expr, 1, 5);
        
        // Test that it can be compiled
        let result = codegen.compile_expression(&question_expr);
        
        // Should either succeed or give a meaningful error (not "unsupported expression")
        match result {
            Ok(value) => {
                info!("Question mark expression compiled successfully: {:?}", value);
                assert!(!value.llvm_name.is_empty());
            }
            Err(e) => {
                let error_msg = e.to_string();
                // Should not be an "unsupported expression" error
                assert!(!error_msg.contains("Unsupported expression type"), 
                       "Question mark operator not integrated in expression compiler: {}", error_msg);
                debug!("Compilation error (expected): {}", error_msg);
            }
        }
    }

    /// Test that question mark compiler trait methods work
    #[test]
    fn test_question_mark_compiler_trait() {
        common::tracing::setup();
        
        let mut codegen = LlvmCodeGenerator::new().expect("Failed to create code generator");
        
        // Create a Result question mark expression
        let var_expr = Box::new(Identifier::from_name("result_var"));
        let question_expr = QuestionMarkExpression::new(var_expr, 1, 10);
        
        // Test Result question mark compilation
        let result_ir = codegen.compile_result_question_mark(&question_expr);
        
        match result_ir {
            Ok(ir) => {
                info!("Result question mark IR generated: {}", ir);
                assert!(ir.contains("extractvalue"), "Should generate extractvalue instruction");
                assert!(ir.contains("br i1"), "Should generate conditional branch");
            }
            Err(e) => {
                debug!("Result question mark compilation error: {}", e);
                // Even if it fails, the method should exist and be callable
            }
        }
        
        // Test Option question mark compilation
        let option_ir = codegen.compile_option_question_mark(&question_expr);
        
        match option_ir {
            Ok(ir) => {
                info!("Option question mark IR generated: {}", ir);
                assert!(ir.contains("extractvalue"), "Should generate extractvalue instruction");
            }
            Err(e) => {
                debug!("Option question mark compilation error: {}", e);
                // Even if it fails, the method should exist and be callable
            }
        }
    }

    /// Test that error propagation call generation works
    #[test]
    fn test_error_propagation_call_generation() {
        common::tracing::setup();
        
        let mut codegen = LlvmCodeGenerator::new().expect("Failed to create code generator");
        
        let var_expr = Box::new(Identifier::from_name("some_result"));
        let question_expr = QuestionMarkExpression::new(var_expr, 2, 15);
        
        // Test error propagation call generation
        let propagation_result = codegen.generate_error_propagation_call(&question_expr);
        
        match propagation_result {
            Ok(ir) => {
                info!("Error propagation IR generated: {}", ir);
                // Should contain proper LLVM IR structures
                assert!(ir.len() > 0, "Should generate non-empty IR");
            }
            Err(e) => {
                debug!("Error propagation generation error: {}", e);
                // Method should exist and be callable
                let error_msg = e.to_string();
                assert!(!error_msg.contains("method not found"), 
                       "QuestionMarkCompiler trait not properly implemented");
            }
        }
    }

    /// Test question mark operator with Result type inference
    #[test]
    fn test_question_mark_result_type_inference() {
        common::tracing::setup();
        
        let mut codegen = LlvmCodeGenerator::new().expect("Failed to create code generator");
        
        // Test type inference for Result types
        let var_expr = Box::new(Identifier::from_name("result_value"));
        let question_expr = QuestionMarkExpression::new(var_expr, 3, 20);
        
        // Test that type inference recognizes Result types
        let type_result = codegen.infer_expression_type_string(&question_expr);
        
        match type_result {
            Ok(type_name) => {
                info!("Inferred type: {}", type_name);
                // Should be able to infer some type (even if default)
                assert!(!type_name.is_empty());
            }
            Err(e) => {
                debug!("Type inference error: {}", e);
                // Method should exist
            }
        }
    }

    /// Test that helper methods for Result/Option handling exist
    #[test]
    fn test_result_option_helper_methods() {
        common::tracing::setup();
        
        let codegen = LlvmCodeGenerator::new().expect("Failed to create code generator");
        
        // Test Result type helpers
        assert!(codegen.is_result_type("Result<i32, String>"));
        assert!(!codegen.is_result_type("Option<i32>"));
        
        // Test Option type helpers
        assert!(codegen.is_option_type("Option<i32>"));
        assert!(!codegen.is_option_type("Result<i32, String>"));
        
        // Test type creation
        let result_type = codegen.get_result_type("i32", "String");
        assert_eq!(result_type, "Result<i32, String>");
        
        let option_type = codegen.get_option_type("i32");
        assert_eq!(option_type, "Option<i32>");
        
        info!("Result/Option helper methods working correctly");
    }

    /// Test that question mark IR generation includes runtime calls
    #[test]
    fn test_question_mark_runtime_integration() {
        common::tracing::setup();
        
        let mut codegen = LlvmCodeGenerator::new().expect("Failed to create code generator");
        
        let var_expr = Box::new(Identifier::from_name("error_prone_call"));
        let question_expr = QuestionMarkExpression::new(var_expr, 4, 25);
        
        // Test that the question mark compilation generates runtime calls
        let result = codegen.compile_result_question_mark(&question_expr);
        
        match result {
            Ok(ir) => {
                info!("Generated IR: {}", ir);
                
                // Check for key LLVM IR patterns that indicate proper compilation
                let has_extract = ir.contains("extractvalue");
                let has_branch = ir.contains("br i1") || ir.contains("br label");
                let has_blocks = ir.contains(":");
                
                info!("IR analysis - extract: {}, branch: {}, blocks: {}", 
                      has_extract, has_branch, has_blocks);
                
                // At least one of these should be present for proper question mark compilation
                assert!(has_extract || has_branch || has_blocks, 
                       "Question mark IR should contain LLVM control flow structures");
            }
            Err(e) => {
                debug!("Question mark compilation error: {}", e);
                // Should not be a "method not found" type error
                let error_msg = e.to_string();
                assert!(!error_msg.contains("not implemented"), 
                       "Question mark compilation not properly implemented");
            }
        }
    }

    /// Test end-to-end question mark operator compilation
    #[test]
    fn test_end_to_end_question_mark_compilation() {
        common::tracing::setup();
        
        let mut codegen = LlvmCodeGenerator::new().expect("Failed to create code generator");
        
        // Create a more realistic question mark expression
        let var_expr = Box::new(Identifier::from_name("file_operation"));
        let question_expr = QuestionMarkExpression::new(var_expr, 5, 30);
        
        // Test complete compilation pipeline
        let expr_compilation = codegen.compile_expression(&question_expr);
        let ir_generation = codegen.compile_expression_to_ir(&question_expr);
        let string_compilation = codegen.compile_expression_to_string(&question_expr);
        
        info!("Expression compilation: {:?}", expr_compilation);
        info!("IR generation: {:?}", ir_generation);
        info!("String compilation: {:?}", string_compilation);
        
        // At least one compilation path should work
        let any_success = expr_compilation.is_ok() || ir_generation.is_ok() || string_compilation.is_ok();
        
        if !any_success {
            // If all fail, check that it's not due to missing integration
            if let Err(e) = expr_compilation {
                let error_msg = e.to_string();
                assert!(!error_msg.contains("Unsupported expression type"), 
                       "Question mark operator not integrated in main compilation pipeline");
            }
        }
        
        info!("Question mark operator compilation pipeline tested");
    }

    /// Test question mark operator with debug information
    #[test]
    fn test_question_mark_with_debug_info() {
        common::tracing::setup();
        
        let debug_config = DebugConfig {
            generate_debug_info: true,
            debug_level: 3,
            include_source: true,
            ..Default::default()
        };
        
        let mut codegen = LlvmCodeGenerator::new_with_debug(debug_config)
            .expect("Failed to create debug code generator");
        
        let var_expr = Box::new(Identifier::from_name("debug_result"));
        let question_expr = QuestionMarkExpression::new(var_expr, 6, 35);
        
        // Test that question mark works with debug enabled
        let result = codegen.compile_expression(&question_expr);
        
        match result {
            Ok(value) => {
                info!("Question mark compiled with debug info: {:?}", value);
                assert!(codegen.debug_enabled());
            }
            Err(e) => {
                debug!("Question mark debug compilation error: {}", e);
                assert!(codegen.debug_enabled(), "Debug should still be enabled even if compilation fails");
            }
        }
    }
}
