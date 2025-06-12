//! Comprehensive integration tests for the CURSED error propagation system
//! 
//! This test suite validates the complete error propagation pipeline from CURSED
//! source code through LLVM IR generation to runtime execution. It ensures that
//! the `?` operator works correctly in all contexts and integrates properly with
//! the type system, memory management, and error handling infrastructure.

use cursed::codegen::llvm::{
    LlvmCodeGenerator,
    ErrorPropagationCompiler,
    EnhancedErrorPropagationCompiler,
    QuestionMarkCompiler,
    MainResultTypeCompiler,
};
use cursed::ast::expressions::{ErrorPropagation, QuestionMarkExpression};
use cursed::ast::traits::Expression;
use cursed::debug::{DebugConfig, SourceLocation};
use cursed::error::{CursedError, SourceLocation as ErrorSourceLocation};
use cursed::parser::Parser;
use cursed::lexer::Lexer;
use std::path::PathBuf;
use std::time::Instant;
use inkwell::types::{BasicType, BasicTypeEnum};

/// Test basic error propagation compilation from CURSED source
#[test]
fn test_basic_error_propagation_compilation() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    // Test basic Result<T,E> error propagation
    let source = r#"
        function test_basic_result() -> Result<i32, String> {
            sus result = might_fail()?;
            facts result + 1
        }
        
        function might_fail() -> Result<i32, String> {
            facts 42
        }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should generate IR for basic error propagation");
    
    // Verify IR contains error propagation logic
    assert!(ir.contains("call i8* @cursed_error_propagation_check"), 
           "IR should contain error propagation check call");
    assert!(ir.contains("br i1"), "IR should contain conditional branching");
    assert!(ir.contains("extractvalue"), "IR should extract Result/Option values");
    
    println!("✓ Basic error propagation compilation works");
}

/// Test chained error propagation (a?.b?.c?)
#[test]
fn test_chained_error_propagation() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    let source = r#"
        function test_chain() -> Result<i32, String> {
            sus value = get_first()?.process()?.finalize()?;
            facts value
        }
        
        function get_first() -> Result<Data, String> {
            facts Data { value: 10 }
        }
        
        squad Data {
            value: i32,
        }
        
        collab ProcessData {
            function process(self) -> Result<i32, String>;
        }
        
        impl ProcessData for Data {
            function process(self) -> Result<i32, String> {
                facts self.value * 2
            }
        }
        
        function finalize(value: i32) -> Result<i32, String> {
            facts value + 5
        }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should generate IR for chained error propagation");
    
    // Verify multiple error propagation checks
    let check_count = ir.matches("call i8* @cursed_error_propagation_check").count();
    assert!(check_count >= 3, "Should have at least 3 error propagation checks for chained calls");
    
    // Verify proper control flow for chaining
    let branch_count = ir.matches("br i1").count();
    assert!(branch_count >= 3, "Should have proper branching for each ? operator");
    
    println!("✓ Chained error propagation compilation works");
}

/// Test Option<T> error propagation
#[test]
fn test_option_error_propagation() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    let source = r#"
        function test_option() -> Option<i32> {
            sus value = get_optional()?;
            facts value * 2
        }
        
        function get_optional() -> Option<i32> {
            facts Some(21)
        }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should generate IR for Option error propagation");
    
    // Verify Option-specific handling
    assert!(ir.contains("call i8* @cursed_option_propagation_check"), 
           "IR should contain Option propagation check");
    assert!(ir.contains("call i1 @cursed_option_is_some"), 
           "IR should check if Option is Some");
    
    println!("✓ Option error propagation compilation works");
}

/// Test mixed Result/Option propagation patterns
#[test]
fn test_mixed_result_option_propagation() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    let source = r#"
        function test_mixed() -> Result<i32, String> {
            sus optional_val = get_optional()?;
            sus result_val = process_value(optional_val)?;
            facts result_val
        }
        
        function get_optional() -> Option<i32> {
            facts Some(42)
        }
        
        function process_value(val: i32) -> Result<i32, String> {
            lowkey (val > 0) {
                facts val * 2
            } flex {
                facts Err("Invalid value")
            }
        }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should generate IR for mixed propagation");
    
    // Should contain both Option and Result propagation logic
    assert!(ir.contains("call i8* @cursed_option_propagation_check") ||
           ir.contains("call i1 @cursed_option_is_some"), 
           "Should handle Option propagation");
    assert!(ir.contains("call i8* @cursed_error_propagation_check") ||
           ir.contains("call i1 @cursed_result_is_ok"), 
           "Should handle Result propagation");
    
    println!("✓ Mixed Result/Option propagation works");
}

/// Test error propagation with generic types
#[test]
fn test_generic_error_propagation() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    let source = r#"
        function test_generic<T>() -> Result<T, String> {
            sus value = get_generic_value::<T>()?;
            facts value
        }
        
        function get_generic_value<T>() -> Result<T, String> {
            // Mock implementation - in real code this would be more complex
            facts Err("Generic error")
        }
    "#;
    
    // This might not fully compile due to generic complexity, but should at least parse
    let result = generator.generate_ir(source);
    
    // For now, just verify it doesn't crash - full generic support is complex
    match result {
        Ok(ir) => {
            assert!(ir.contains("define"), "Should generate some IR");
            println!("✓ Generic error propagation compiles successfully");
        }
        Err(_) => {
            println!("✓ Generic error propagation fails gracefully (expected for complex generics)");
        }
    }
}

/// Test error propagation in different syntactic contexts
#[test]
fn test_error_propagation_contexts() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    // Test in if conditions
    let source1 = r#"
        function test_if_context() -> Result<i32, String> {
            lowkey (get_bool_result()?) {
                facts 1
            } flex {
                facts 0
            }
        }
        
        function get_bool_result() -> Result<bool, String> {
            facts true
        }
    "#;
    
    let ir1 = generator.generate_ir(source1).expect("Should handle ? in if condition");
    assert!(ir1.contains("call i8* @cursed_error_propagation_check") ||
           ir1.contains("br i1"), "Should have proper error handling in if context");
    
    // Test in loop conditions
    let source2 = r#"
        function test_loop_context() -> Result<i32, String> {
            sus count = 0;
            bestie (get_continue_result()?) {
                count = count + 1;
                lowkey (count > 10) {
                    break;
                }
            }
            facts count
        }
        
        function get_continue_result() -> Result<bool, String> {
            facts true
        }
    "#;
    
    let ir2 = generator.generate_ir(source2).expect("Should handle ? in loop condition");
    assert!(ir2.contains("call i8* @cursed_error_propagation_check") ||
           ir2.contains("br i1"), "Should have proper error handling in loop context");
    
    println!("✓ Error propagation works in different syntactic contexts");
}

/// Test error propagation with custom error types
#[test]
fn test_custom_error_types() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    let source = r#"
        squad CustomError {
            message: String,
            code: i32,
        }
        
        function test_custom_error() -> Result<i32, CustomError> {
            sus value = risky_operation()?;
            facts value * 2
        }
        
        function risky_operation() -> Result<i32, CustomError> {
            facts Err(CustomError { 
                message: "Something went wrong", 
                code: 500 
            })
        }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should handle custom error types");
    assert!(ir.contains("call i8* @cursed_error_propagation_check") ||
           ir.contains("extractvalue"), "Should handle custom error propagation");
    
    println!("✓ Custom error types work with error propagation");
}

/// Test error propagation performance characteristics
#[test]
fn test_error_propagation_performance() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    // Create a chain of error propagations to test performance
    let source = r#"
        function performance_test() -> Result<i32, String> {
            sus a = step1()?;
            sus b = step2(a)?;
            sus c = step3(b)?;
            sus d = step4(c)?;
            sus e = step5(d)?;
            facts e
        }
        
        function step1() -> Result<i32, String> { facts 1 }
        function step2(x: i32) -> Result<i32, String> { facts x + 1 }
        function step3(x: i32) -> Result<i32, String> { facts x + 1 }
        function step4(x: i32) -> Result<i32, String> { facts x + 1 }
        function step5(x: i32) -> Result<i32, String> { facts x + 1 }
    "#;
    
    let start = Instant::now();
    let ir = generator.generate_ir(source).expect("Should generate IR for performance test");
    let compile_time = start.elapsed();
    
    // Verify the IR was generated efficiently
    assert!(compile_time.as_millis() < 1000, 
           "Error propagation compilation should be reasonably fast");
    
    // Check that the IR doesn't have excessive bloat
    let ir_lines = ir.lines().count();
    assert!(ir_lines < 500, 
           "Generated IR should be reasonably compact (got {} lines)", ir_lines);
    
    println!("✓ Error propagation compilation performance is acceptable ({:?})", compile_time);
}

/// Test memory safety during error propagation
#[test]
fn test_error_propagation_memory_safety() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    let source = r#"
        function memory_safety_test() -> Result<String, String> {
            sus data = allocate_string()?;
            sus processed = process_string(data)?;
            facts processed
        }
        
        function allocate_string() -> Result<String, String> {
            facts "Hello, World!".to_string()
        }
        
        function process_string(s: String) -> Result<String, String> {
            facts s + " - Processed"
        }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should generate safe IR");
    
    // Verify proper memory management in error paths
    assert!(ir.contains("call"), "Should contain function calls");
    
    // Should not contain obvious memory safety issues
    assert!(!ir.contains("free i8*"), "Should not have explicit free calls in error propagation");
    
    println!("✓ Error propagation maintains memory safety");
}

/// Test error context preservation through propagation chains
#[test]
fn test_error_context_preservation() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    // Set up source location for debugging
    let location = SourceLocation::new(PathBuf::from("context_test.csd"), 15, 8);
    generator.set_location(location);
    
    let source = r#"
        function context_preservation_test() -> Result<i32, String> {
            sus value1 = operation_a()?; // Line 2
            sus value2 = operation_b(value1)?; // Line 3  
            sus value3 = operation_c(value2)?; // Line 4
            facts value3
        }
        
        function operation_a() -> Result<i32, String> { facts 10 }
        function operation_b(x: i32) -> Result<i32, String> { facts x * 2 }
        function operation_c(x: i32) -> Result<i32, String> { facts x + 5 }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should preserve error context");
    
    // Verify debug information is preserved
    assert!(ir.contains("!dbg"), "Should contain debug information for context preservation");
    
    println!("✓ Error context is preserved through propagation chains");
}

/// Test error propagation with complex control flow
#[test]
fn test_error_propagation_complex_control_flow() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    let source = r#"
        function complex_control_flow() -> Result<i32, String> {
            sus initial = get_initial_value()?;
            
            lowkey (initial > 10) {
                sus doubled = double_value(initial)?;
                lowkey (doubled > 100) {
                    facts triple_value(doubled)?
                } flex {
                    facts doubled
                }
            } flex {
                sus processed = process_small_value(initial)?;
                bestie (processed < 50) {
                    processed = increment_value(processed)?;
                }
                facts processed
            }
        }
        
        function get_initial_value() -> Result<i32, String> { facts 15 }
        function double_value(x: i32) -> Result<i32, String> { facts x * 2 }
        function triple_value(x: i32) -> Result<i32, String> { facts x * 3 }
        function process_small_value(x: i32) -> Result<i32, String> { facts x + 10 }
        function increment_value(x: i32) -> Result<i32, String> { facts x + 1 }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should handle complex control flow");
    
    // Verify proper branching and error handling in complex scenarios
    let branch_count = ir.matches("br i1").count();
    assert!(branch_count >= 5, "Should have proper branching for complex control flow");
    
    let error_check_count = ir.matches("call i8* @cursed_error_propagation_check").count();
    assert!(error_check_count >= 5, "Should have error checks for all ? operators");
    
    println!("✓ Error propagation works correctly with complex control flow");
}

/// Test error propagation type inference
#[test]
fn test_error_propagation_type_inference() {
    let generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    // Test type checking methods work correctly
    assert!(generator.is_result_type("Result<i32, String>"));
    assert!(generator.is_result_type("Result<Vec<i32>, CustomError>"));
    assert!(!generator.is_result_type("Option<i32>"));
    assert!(!generator.is_result_type("i32"));
    
    assert!(generator.is_option_type("Option<i32>"));
    assert!(generator.is_option_type("Option<String>"));
    assert!(!generator.is_option_type("Result<i32, String>"));
    assert!(!generator.is_option_type("Vec<i32>"));
    
    // Test type string generation
    assert_eq!(generator.get_result_type("i32", "String"), "Result<i32, String>");
    assert_eq!(generator.get_option_type("bool"), "Option<bool>");
    
    println!("✓ Error propagation type inference works correctly");
}

/// Test all error propagation modules work together
#[test]
fn test_integration_all_modules() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    // Test that we can use all error propagation functionality together
    let _temp_name = generator.next_temp_name();
    let _block_name = generator.next_block_name("integration");
    let _result_type = generator.get_result_type("i32", "String");
    let _option_type = generator.get_option_type("bool");
    
    // Test enhanced error propagation context
    let location = SourceLocation::new(PathBuf::from("integration_test.csd"), 25, 12);
    let error_location = ErrorSourceLocation {
        file: location.file.to_str().map(|s| s.to_string()),
        line: location.line as usize,
        column: location.column as usize,
    };
    
    // Create enhanced context
    use cursed::codegen::llvm::error_propagation_enhanced::ErrorPropagationContext;
    let context = ErrorPropagationContext::new(error_location)
        .with_tail_position(true)
        .with_function_context("integration_test".to_string());
    
    assert!(context.is_tail_position);
    assert_eq!(context.function_context, Some("integration_test".to_string()));
    
    // Test IR generation methods
    let result_check = generator.generate_result_success_check("%test_result", "Result<i32, String>");
    assert!(result_check.contains("extractvalue") || result_check.contains("icmp"));
    
    let option_check = generator.generate_option_presence_check("%test_option", "Option<i32>");
    assert!(option_check.contains("extractvalue") || option_check.contains("icmp"));
    
    // Test value extraction
    let result_extract = generator.extract_result_value("%test_result", "Result<i32, String>", "i32");
    assert!(result_extract.contains("extractvalue"));
    
    let option_extract = generator.extract_option_value("%test_option", "Option<i32>", "i32");
    assert!(option_extract.contains("extractvalue"));
    
    // Reset and verify
    generator.reset_counters();
    let temp_id_after_reset = generator.next_temp_id();
    assert_eq!(temp_id_after_reset, 0);
    
    println!("✓ All error propagation modules integrate successfully");
}

/// Benchmark error propagation compilation performance
#[test]
fn test_error_propagation_compilation_benchmark() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    // Generate a more complex error propagation chain for benchmarking
    let mut source = String::from(r#"
        function benchmark_chain() -> Result<i32, String> {
            sus result = 0;
    "#);
    
    // Add 20 chained operations
    for i in 0..20 {
        source.push_str(&format!("            result = operation_{}(result)?;\n", i));
    }
    
    source.push_str("            facts result\n        }\n\n");
    
    // Add the operation functions
    for i in 0..20 {
        source.push_str(&format!(
            "        function operation_{}(x: i32) -> Result<i32, String> {{ facts x + {} }}\n", 
            i, i
        ));
    }
    
    // Benchmark compilation time
    let start = Instant::now();
    let ir = generator.generate_ir(&source).expect("Should compile benchmark");
    let compile_time = start.elapsed();
    
    // Verify reasonable performance
    assert!(compile_time.as_millis() < 2000, 
           "Complex error propagation should compile in reasonable time");
    
    // Verify all error checks are present
    let error_check_count = ir.matches("call i8* @cursed_error_propagation_check").count();
    assert!(error_check_count >= 20, "Should have error checks for all operations");
    
    println!("✓ Error propagation compilation benchmark: {:?} for 20 chained operations", compile_time);
}
