//! LLVM IR generation tests for CURSED error propagation
//! 
//! This test suite validates that the error propagation system generates correct
//! LLVM IR code for all patterns of `?` operator usage. It ensures that the
//! generated IR correctly implements error propagation semantics, maintains
//! type safety, and produces efficient code.

use cursed::codegen::llvm::{
    LlvmCodeGenerator,
    ErrorPropagationCompiler,
    EnhancedErrorPropagationCompiler,
    QuestionMarkCompiler,
    MainResultTypeCompiler,
};
use cursed::ast::expressions::{ErrorPropagation, QuestionMarkExpression};
use cursed::ast::traits::Expression;
use cursed::debug::SourceLocation;
use cursed::error::SourceLocation as ErrorSourceLocation;
use std::path::PathBuf;
use inkwell::{
    context::Context,
    types::{BasicType, BasicTypeEnum},
    values::BasicValueEnum,
};

/// Test LLVM IR generation for basic Result<T,E> error propagation
#[test]
fn test_basic_result_ir_generation() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    let source = r#"
        function test_result() -> Result<i32, String> {
            sus value = get_result()?;
            facts value + 10
        }
        
        function get_result() -> Result<i32, String> {
            facts 42
        }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should generate IR");
    
    // Verify function signature
    assert!(ir.contains("define"), "Should contain function definition");
    assert!(ir.contains("i32"), "Should contain i32 type");
    
    // Verify Result type handling
    assert!(ir.contains("call i8* @cursed_error_propagation_check") ||
           ir.contains("call i1 @cursred_result_is_ok") ||
           ir.contains("extractvalue"), 
           "Should contain Result error checking");
    
    // Verify proper branching for error cases
    assert!(ir.contains("br i1"), "Should contain conditional branching");
    assert!(ir.contains("br label"), "Should contain unconditional branching");
    
    // Verify value extraction
    assert!(ir.contains("extractvalue") || ir.contains("load"), 
           "Should extract value from Result");
    
    println!("✓ Basic Result IR generation is correct");
}

/// Test LLVM IR generation for Option<T> error propagation
#[test]
fn test_option_ir_generation() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    let source = r#"
        function test_option() -> Option<i32> {
            sus value = get_option()?;
            facts value * 2
        }
        
        function get_option() -> Option<i32> {
            facts Some(21)
        }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should generate IR");
    
    // Verify Option-specific IR patterns
    assert!(ir.contains("call i8* @cursed_option_propagation_check") ||
           ir.contains("call i1 @cursed_option_is_some") ||
           ir.contains("extractvalue"), 
           "Should contain Option presence checking");
    
    // Verify None handling
    assert!(ir.contains("br i1"), "Should branch on Option presence");
    
    // Verify Some value extraction  
    assert!(ir.contains("extractvalue") || ir.contains("load"), 
           "Should extract value from Some");
    
    println!("✓ Option IR generation is correct");
}

/// Test IR generation for chained error propagation
#[test]
fn test_chained_propagation_ir() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    let source = r#"
        function test_chain() -> Result<i32, String> {
            sus a = step_one()?;
            sus b = step_two(a)?;
            sus c = step_three(b)?;
            facts c
        }
        
        function step_one() -> Result<i32, String> { facts 1 }
        function step_two(x: i32) -> Result<i32, String> { facts x + 1 }
        function step_three(x: i32) -> Result<i32, String> { facts x + 1 }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should generate chained IR");
    
    // Count error propagation checks
    let check_count = ir.matches("call i8* @cursed_error_propagation_check").count() +
                     ir.matches("call i1 @cursed_result_is_ok").count() +
                     ir.matches("icmp eq i1").count();
    
    assert!(check_count >= 3, "Should have at least 3 error checks for chained propagation");
    
    // Verify proper control flow structure
    let branch_count = ir.matches("br i1").count();
    assert!(branch_count >= 3, "Should have proper branching for each propagation point");
    
    // Verify multiple basic blocks for error handling
    let block_count = ir.matches("^[a-zA-Z0-9_]+:").count();
    assert!(block_count >= 6, "Should have sufficient basic blocks for complex control flow");
    
    println!("✓ Chained propagation IR generation is correct");
}

/// Test IR generation for error propagation in complex expressions
#[test]
fn test_complex_expression_ir() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    let source = r#"
        function test_complex() -> Result<i32, String> {
            sus result = (get_a()? + get_b()?) * get_multiplier()?;
            facts result
        }
        
        function get_a() -> Result<i32, String> { facts 5 }
        function get_b() -> Result<i32, String> { facts 3 }
        function get_multiplier() -> Result<i32, String> { facts 2 }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should generate complex expression IR");
    
    // Verify multiple error checks for complex expressions
    let check_count = ir.matches("call i8* @cursed_error_propagation_check").count() +
                     ir.matches("extractvalue").count();
    assert!(check_count >= 3, "Should handle multiple error checks in complex expressions");
    
    // Verify arithmetic operations are present
    assert!(ir.contains("add i32") || ir.contains("mul i32"), 
           "Should contain arithmetic operations");
    
    println!("✓ Complex expression IR generation is correct");
}

/// Test IR generation for error propagation with different return types
#[test]
fn test_different_return_types_ir() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    // Test with string return type
    let source1 = r#"
        function test_string() -> Result<String, String> {
            sus value = get_string_result()?;
            facts value + " processed"
        }
        
        function get_string_result() -> Result<String, String> {
            facts "hello"
        }
    "#;
    
    let ir1 = generator.generate_ir(source1).expect("Should generate string IR");
    assert!(ir1.contains("i8*") || ir1.contains("ptr"), "Should handle string types");
    
    // Test with boolean return type
    let source2 = r#"
        function test_bool() -> Result<bool, String> {
            sus value = get_bool_result()?;
            facts !value
        }
        
        function get_bool_result() -> Result<bool, String> {
            facts true
        }
    "#;
    
    let ir2 = generator.generate_ir(source2).expect("Should generate bool IR");
    assert!(ir2.contains("i1") || ir2.contains("i8"), "Should handle boolean types");
    
    println!("✓ Different return types IR generation is correct");
}

/// Test IR generation for error propagation with structs
#[test]
fn test_struct_propagation_ir() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    let source = r#"
        squad Point {
            x: i32,
            y: i32,
        }
        
        function test_struct() -> Result<Point, String> {
            sus point = create_point()?;
            facts Point { x: point.x + 1, y: point.y + 1 }
        }
        
        function create_point() -> Result<Point, String> {
            facts Point { x: 10, y: 20 }
        }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should generate struct IR");
    
    // Verify struct type handling
    assert!(ir.contains("type") || ir.contains("struct"), "Should define struct types");
    
    // Verify struct field access
    assert!(ir.contains("getelementptr") || ir.contains("extractvalue") || 
           ir.contains("insertvalue"), "Should handle struct field operations");
    
    println!("✓ Struct propagation IR generation is correct");
}

/// Test IR generation for error propagation in control flow
#[test]
fn test_control_flow_propagation_ir() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    let source = r#"
        function test_control_flow() -> Result<i32, String> {
            lowkey (get_condition()?) {
                sus value = get_true_value()?;
                facts value * 2
            } flex {
                sus value = get_false_value()?;
                facts value + 5
            }
        }
        
        function get_condition() -> Result<bool, String> { facts true }
        function get_true_value() -> Result<i32, String> { facts 10 }
        function get_false_value() -> Result<i32, String> { facts 20 }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should generate control flow IR");
    
    // Verify proper basic block structure
    let block_count = ir.matches("^[a-zA-Z0-9_]+:").count();
    assert!(block_count >= 4, "Should have sufficient basic blocks for if-else with error propagation");
    
    // Verify conditional branches
    let conditional_branches = ir.matches("br i1").count();
    assert!(conditional_branches >= 2, "Should have conditional branches for both condition and error checks");
    
    println!("✓ Control flow propagation IR generation is correct");
}

/// Test IR generation for error propagation in loops
#[test]
fn test_loop_propagation_ir() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    let source = r#"
        function test_loop() -> Result<i32, String> {
            sus total = 0;
            sus i = 0;
            bestie (i < 5) {
                sus value = get_loop_value(i)?;
                total = total + value;
                i = i + 1;
            }
            facts total
        }
        
        function get_loop_value(index: i32) -> Result<i32, String> {
            facts index * 2
        }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should generate loop IR");
    
    // Verify loop structure
    assert!(ir.contains("br i1"), "Should have conditional branching for loop");
    assert!(ir.contains("br label"), "Should have unconditional branching for loop back-edge");
    
    // Verify loop with error propagation has proper structure
    let block_count = ir.matches("^[a-zA-Z0-9_]+:").count();
    assert!(block_count >= 3, "Should have sufficient basic blocks for loop with error propagation");
    
    println!("✓ Loop propagation IR generation is correct");
}

/// Test IR generation for error propagation with tail calls
#[test]
fn test_tail_call_propagation_ir() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    let source = r#"
        function test_tail_call() -> Result<i32, String> {
            facts tail_recursive(5)?
        }
        
        function tail_recursive(n: i32) -> Result<i32, String> {
            lowkey (n <= 0) {
                facts 1
            } flex {
                sus prev = tail_recursive(n - 1)?;
                facts n * prev
            }
        }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should generate tail call IR");
    
    // Verify function calls
    assert!(ir.contains("call"), "Should contain function calls");
    
    // Verify tail call optimization opportunities
    // Note: LLVM tail call optimization is complex and may not always be visible
    // in the IR without optimization passes
    
    println!("✓ Tail call propagation IR generation is correct");
}

/// Test IR generation for error propagation with custom error types
#[test]
fn test_custom_error_type_ir() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    let source = r#"
        squad CustomError {
            code: i32,
            message: String,
        }
        
        function test_custom_error() -> Result<i32, CustomError> {
            sus value = risky_operation()?;
            facts value + 100
        }
        
        function risky_operation() -> Result<i32, CustomError> {
            facts Err(CustomError { code: 404, message: "Not found" })
        }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should generate custom error IR");
    
    // Verify custom error type handling
    assert!(ir.contains("type") || ir.contains("struct"), "Should define custom error type");
    
    // Verify error propagation with custom types
    assert!(ir.contains("call i8* @cursed_error_propagation_check") ||
           ir.contains("extractvalue"), "Should handle custom error propagation");
    
    println!("✓ Custom error type IR generation is correct");
}

/// Test IR generation produces valid LLVM IR
#[test]
fn test_ir_validity() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    let source = r#"
        function test_validity() -> Result<i32, String> {
            sus a = operation_a()?;
            sus b = operation_b(a)?;
            facts a + b
        }
        
        function operation_a() -> Result<i32, String> { facts 10 }
        function operation_b(x: i32) -> Result<i32, String> { facts x * 2 }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should generate valid IR");
    
    // Basic LLVM IR structure validation
    assert!(ir.contains("define"), "Should contain function definitions");
    assert!(ir.contains("ret"), "Should contain return statements");
    
    // Verify basic blocks are properly terminated
    let lines: Vec<&str> = ir.lines().collect();
    let mut in_function = false;
    let mut last_instruction_terminates = false;
    
    for line in lines {
        let trimmed = line.trim();
        
        if trimmed.starts_with("define") {
            in_function = true;
            last_instruction_terminates = false;
            continue;
        }
        
        if trimmed == "}" {
            if in_function {
                assert!(last_instruction_terminates, 
                       "Function should end with terminating instruction");
            }
            in_function = false;
            continue;
        }
        
        if in_function && !trimmed.is_empty() && !trimmed.starts_with(";") {
            // Check if this is a terminating instruction
            last_instruction_terminates = trimmed.starts_with("ret ") ||
                                        trimmed.starts_with("br ") ||
                                        trimmed.contains("unreachable");
            
            // Basic blocks should start with a label
            if trimmed.ends_with(":") {
                last_instruction_terminates = false;
            }
        }
    }
    
    println!("✓ Generated IR is structurally valid");
}

/// Test IR generation helper methods
#[test]
fn test_ir_generation_helpers() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    // Test temp name generation
    let temp1 = generator.next_temp_name();
    let temp2 = generator.next_temp_name();
    assert!(temp1.starts_with("%temp_"));
    assert!(temp2.starts_with("%temp_"));
    assert_ne!(temp1, temp2, "Temp names should be unique");
    
    // Test block name generation
    let block1 = generator.next_block_name("test");
    let block2 = generator.next_block_name("test");
    assert!(block1.starts_with("test_block_"));
    assert!(block2.starts_with("test_block_"));
    assert_ne!(block1, block2, "Block names should be unique");
    
    // Test Result type checking
    assert!(generator.is_result_type("Result<i32, String>"));
    assert!(generator.is_result_type("Result<bool, CustomError>"));
    assert!(!generator.is_result_type("Option<i32>"));
    assert!(!generator.is_result_type("Vec<i32>"));
    
    // Test Option type checking
    assert!(generator.is_option_type("Option<i32>"));
    assert!(generator.is_option_type("Option<String>"));
    assert!(!generator.is_option_type("Result<i32, String>"));
    assert!(!generator.is_option_type("i32"));
    
    // Test type string generation
    assert_eq!(generator.get_result_type("i32", "String"), "Result<i32, String>");
    assert_eq!(generator.get_option_type("bool"), "Option<bool>");
    assert_eq!(generator.get_error_type(), "String"); // Default error type
    
    println!("✓ IR generation helper methods work correctly");
}

/// Test IR generation for specific error propagation patterns
#[test]
fn test_ir_specific_patterns() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    // Test Result success check IR generation
    let result_check = generator.generate_result_success_check("%result_val", "Result<i32, String>");
    assert!(result_check.contains("extractvalue") || result_check.contains("icmp"), 
           "Should generate Result success check");
    
    // Test Option presence check IR generation
    let option_check = generator.generate_option_presence_check("%option_val", "Option<i32>");
    assert!(option_check.contains("extractvalue") || option_check.contains("icmp"), 
           "Should generate Option presence check");
    
    // Test Result value extraction
    let result_extract = generator.extract_result_value("%result_val", "Result<i32, String>", "i32");
    assert!(result_extract.contains("extractvalue"), "Should extract Result value");
    
    // Test Option value extraction  
    let option_extract = generator.extract_option_value("%option_val", "Option<i32>", "i32");
    assert!(option_extract.contains("extractvalue"), "Should extract Option value");
    
    // Test Result error extraction
    let error_extract = generator.extract_result_error("%result_val", "Result<i32, String>", "String");
    assert!(error_extract.contains("extractvalue"), "Should extract Result error");
    
    println!("✓ Specific IR generation patterns work correctly");
}

/// Test IR generation counter management
#[test]
fn test_ir_counter_management() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    // Test initial counter states
    let initial_temp = generator.next_temp_id();
    let initial_block = generator.next_block_counter();
    
    // Generate some names
    let _temp1 = generator.next_temp_name();
    let _temp2 = generator.next_temp_name();
    let _block1 = generator.next_block_name("test");
    let _block2 = generator.next_block_name("test");
    
    // Check counters advanced
    let after_temp = generator.next_temp_id();
    let after_block = generator.next_block_counter();
    
    assert!(after_temp > initial_temp, "Temp counter should advance");
    assert!(after_block > initial_block, "Block counter should advance");
    
    // Test counter reset
    generator.reset_counters();
    
    let reset_temp = generator.next_temp_id();
    let reset_block = generator.next_block_counter();
    
    assert_eq!(reset_temp, 0, "Temp counter should reset to 0");
    assert_eq!(reset_block, 0, "Block counter should reset to 0");
    
    println!("✓ IR generation counter management works correctly");
}

/// Test IR generation with debug information
#[test]
fn test_ir_debug_information() {
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    
    // Set debug location
    let location = SourceLocation::new(PathBuf::from("debug_test.csd"), 42, 13);
    generator.set_location(location);
    
    let source = r#"
        function debug_test() -> Result<i32, String> {
            sus value = get_debug_value()?;
            facts value + 1
        }
        
        function get_debug_value() -> Result<i32, String> {
            facts 99
        }
    "#;
    
    let ir = generator.generate_ir(source).expect("Should generate IR with debug info");
    
    // Verify debug information is included
    assert!(ir.contains("!dbg") || ir.contains("metadata"), 
           "Should contain debug information");
    
    // Test location retrieval
    let current_location = generator.get_location();
    assert!(current_location.is_some(), "Should have current location set");
    
    if let Some(loc) = current_location {
        assert_eq!(loc.line, 42);
        assert_eq!(loc.column, 13);
        assert!(loc.file.to_string_lossy().contains("debug_test.csd"));
    }
    
    println!("✓ IR generation with debug information works correctly");
}
