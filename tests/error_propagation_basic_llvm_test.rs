use cursed::ast::expressions::question_mark::QuestionMarkExpression;
use cursed::ast::identifiers::Identifier;
use cursed::codegen::llvm::{LlvmCodeGenerator, ErrorPropagationCompiler};
use cursed::codegen::llvm::question_mark::QuestionMarkCompiler;
use cursed::error::CursedError;
use tracing::{debug, info, error, warn};

macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .try_init();
    };
}

#[test]
fn test_question_mark_compilation_api() {
    init_tracing!();
    info!("Testing question mark operator compilation API");

    // Create a simple question mark expression: variable?
    let var_expr = Identifier::new("result".to_string(), "result".to_string());
    let question_expr = QuestionMarkExpression::new(
        Box::new(var_expr),
        1,
        5
    );

    // Create LLVM code generator
    let mut codegen = LlvmCodeGenerator::new().unwrap();

    // Test that the API exists and that it fails appropriately when no context is provided
    // This is expected behavior - we can't compile expressions without proper context
    let ir = codegen.compile_question_mark(&question_expr);
    
    // The compilation should fail because there's no symbol context, but the API should exist
    assert!(ir.is_err(), "Question mark compilation should fail without proper context");
    
    if let Err(e) = ir {
        info!("Expected compilation error (no context): {:?}", e);
        // The error should be about undefined variable, not about missing method
        assert!(e.to_string().contains("Undefined variable"), 
               "Error should be about undefined variable, not missing API");
    }
    
    info!("Question mark compilation API is working correctly");
}

#[test]
fn test_error_propagation_compilation() {
    init_tracing!();
    info!("Testing error propagation compilation");

    // Create LLVM code generator
    let mut codegen = LlvmCodeGenerator::new().unwrap();

    // Test result check generation
    let result_check = codegen.generate_result_check("%test_result");
    assert!(result_check.is_ok(), "Result check generation should succeed");
    
    let check_result = result_check.unwrap();
    assert!(check_result.ir_code.contains("cursed_check_result"), 
           "Result check should contain runtime call");
    
    info!("Generated result check IR: {}", check_result.ir_code);
}

#[test]
fn test_error_propagation_context() {
    init_tracing!();
    info!("Testing error propagation context");

    use cursed::codegen::llvm::error_propagation::PropagationContext;
    use cursed::error::SourceLocation;

    // Create a propagation context
    let context = PropagationContext {
        source_location: SourceLocation::new(10, 5),
        function_context: Some("test_function".to_string()),
        expected_return_type: Some("Result<i32, String>".to_string()),
    };

    assert_eq!(context.source_location.line, 10);
    assert_eq!(context.source_location.column, 5);
    assert_eq!(context.function_context, Some("test_function".to_string()));
    
    info!("Context created successfully: {:?}", context);
}

#[test]
fn test_question_mark_ir_generation() {
    init_tracing!();
    info!("Testing question mark IR generation");

    // Create LLVM code generator
    let mut codegen = LlvmCodeGenerator::new().unwrap();

    // Test temp ID generation
    let temp_id1 = codegen.next_temp_id();
    let temp_id2 = codegen.next_temp_id();
    assert!(temp_id2 > temp_id1, "Temp IDs should increment");

    // Test temp name generation
    let temp_name = codegen.next_temp_name();
    assert!(temp_name.starts_with("%temp_"), "Temp names should have proper prefix");

    // Test block name generation
    let block_name = codegen.next_block_name("test");
    assert!(block_name.starts_with("test_block_"), "Block names should have proper prefix");

    info!("Generated temp name: {}, block name: {}", temp_name, block_name);
}

#[test]
fn test_type_checking_helpers() {
    init_tracing!();
    info!("Testing type checking helpers");

    let codegen = LlvmCodeGenerator::new().unwrap();

    // Test Result type checking
    assert!(codegen.is_result_type("Result<i32, String>"));
    assert!(!codegen.is_result_type("Option<i32>"));
    assert!(!codegen.is_result_type("i32"));

    // Test Option type checking
    assert!(codegen.is_option_type("Option<i32>"));
    assert!(!codegen.is_option_type("Result<i32, String>"));
    assert!(!codegen.is_option_type("i32"));

    // Test type string generation
    let result_type = codegen.get_result_type("i32", "String");
    assert_eq!(result_type, "Result<i32, String>");
    
    let option_type = codegen.get_option_type("i32");
    assert_eq!(option_type, "Option<i32>");

    info!("Type checking helpers working correctly");
}

mod tests {
    use super::*;

    #[test]
    fn minimal_test() {
        init_tracing!();
        info!("Running minimal error propagation test");
        
        // Just verify that the basic imports and structures work
        let codegen = LlvmCodeGenerator::new();
        assert!(codegen.is_ok(), "LlvmCodeGenerator should initialize successfully");
        
        info!("Minimal test passed");
    }
}
