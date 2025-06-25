/// Integration test for LLVM function compilation pipeline
/// 
/// Tests the complete compilation flow from AST to LLVM IR generation
/// including function compilation, expression compilation, and GC integration.

use cursed::ast::expressions::{Literal, LiteralValue};
use cursed::ast::identifiers::Identifier;
use cursed::ast::declarations::FunctionStatement;
use cursed::ast::statements::LetStatement;
use cursed::ast::block::BlockStatement;
use cursed::codegen::llvm::{LlvmCodeGenerator, FunctionCompilation, LlvmGcIntegration};
use cursed::error::Error;
use cursed::memory::gc::GcConfig;
use std::sync::Arc;

#[test]
fn test_basic_function_compilation() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Create a simple function: slay add(x: int, y: int) -> int { x + y }
    let func_name = Identifier::new("add".to_string(), "add".to_string());
    let params = vec![
        cursed::ast::expressions::Parameter::new("x".to_string(), "int".to_string()),
        cursed::ast::expressions::Parameter::new("y".to_string(), "int".to_string()),
    ];
    
    let statements = vec![
        // Return statement would go here in a real implementation
    ];
    let body = BlockStatement::new("add_body".to_string(), statements);
    
    let function = FunctionStatement::new(
        "slay".to_string(),
        func_name,
        params,
        Some(Box::new(Identifier::new("int".to_string(), "int".to_string()))),
        body,
    );
    
    let result = generator.compile_function_declaration(&function);
    assert!(result.is_ok(), "Function compilation should succeed");
    
    let ir = result.unwrap();
    assert!(ir.contains("define i32 @add"), "Should generate function declaration");
    assert!(ir.contains("ret i32"), "Should have return statement");
}

#[test]
fn test_expression_compilation() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Test integer literal
    let int_literal = Literal::new(LiteralValue::Integer(42));
    let result = generator.compile_basic_expression(&int_literal);
    assert!(result.is_ok(), "Integer literal compilation should succeed");
    
    let value = result.unwrap();
    assert_eq!(value.llvm_name, "42");
    assert!(value.is_constant);
    
    // Test string literal  
    let string_literal = Literal::new(LiteralValue::String("hello".to_string()));
    let string_result = generator.compile_string_literal(&string_literal);
    assert!(string_result.is_ok(), "String literal compilation should succeed");
    
    let string_value = string_result.unwrap();
    assert!(string_value.llvm_name.starts_with("@str_"));
    assert!(string_value.is_constant);
}

#[test]
fn test_variable_compilation() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Test identifier (variable reference)
    let identifier = Identifier::new("test_var".to_string(), "test_var".to_string());
    let result = generator.compile_basic_expression(&identifier);
    assert!(result.is_ok(), "Identifier compilation should succeed");
    
    let value = result.unwrap();
    assert_eq!(value.llvm_name, "%test_var");
    assert!(!value.is_constant);
}

#[test]
fn test_gc_integration() {
    let gc_config = GcConfig::default();
    let mut integration = LlvmGcIntegration::new(gc_config).unwrap();
    
    // Test type registration
    integration.register_type("TestType".to_string(), 64);
    
    // Test allocation IR generation
    let alloc_ir = integration.generate_allocation_ir("TestType", "%obj");
    assert!(alloc_ir.is_ok(), "Allocation IR generation should succeed");
    
    let ir = alloc_ir.unwrap();
    assert!(ir.contains("cursed_allocate_object"), "Should call allocation function");
    assert!(ir.contains("TestType"), "Should reference type name");
    assert!(ir.contains("%obj"), "Should use provided variable name");
    
    // Test safe point generation
    let safe_point_ir = integration.generate_safe_point_ir("test_context");
    assert!(safe_point_ir.contains("cursed_safe_point"), "Should generate safe point call");
    assert!(safe_point_ir.contains("test_context"), "Should include context in comment");
    
    // Test write barrier generation
    let write_barrier_ir = integration.generate_write_barrier_ir("%obj", "%field", "%value");
    assert!(write_barrier_ir.contains("cursed_write_barrier"), "Should generate write barrier call");
    assert!(write_barrier_ir.contains("%obj"), "Should include object pointer");
    assert!(write_barrier_ir.contains("%field"), "Should include field pointer");
    assert!(write_barrier_ir.contains("%value"), "Should include value pointer");
}

#[test]
fn test_complete_ir_generation() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Initialize GC integration
    let gc_config = GcConfig::default();
    generator.initialize_gc_integration(gc_config).unwrap();
    
    // Test complete IR generation
    let source = "slay main() { facts x = 42; }";
    let ir = generator.generate_ir_with_gc(source);
    assert!(ir.is_ok(), "Complete IR generation should succeed");
    
    let generated_ir = ir.unwrap();
    
    // Check for required components
    assert!(generated_ir.contains("target datalayout"), "Should have target layout");
    assert!(generated_ir.contains("target triple"), "Should have target triple");
    assert!(generated_ir.contains("cursed_allocate_object"), "Should declare GC functions");
    assert!(generated_ir.contains("cursed_safe_point"), "Should declare safe point function");
    assert!(generated_ir.contains("define i32 @main"), "Should have main function");
}

#[test]
fn test_error_handling() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Test allocation with unregistered type
    let gc_config = GcConfig::default();
    let integration = LlvmGcIntegration::new(gc_config).unwrap();
    
    let result = integration.generate_allocation_ir("UnknownType", "%obj");
    assert!(result.is_err(), "Should fail for unregistered type");
}

#[test]
fn test_function_context_management() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Test function context creation and management
    let context = cursed::codegen::llvm::function_compilation::FunctionContext::new(
        "test_func".to_string(), 
        "i32".to_string()
    );
    
    assert_eq!(context.name, "test_func");
    assert_eq!(context.return_type, "i32");
    assert_eq!(context.current_block, "test_func_entry");
    assert_eq!(context.entry_block, "test_func_entry");
    assert_eq!(context.temp_counter, 0);
    
    // Test temp variable generation
    let mut ctx = context;
    assert_eq!(ctx.next_temp(), "%temp0");
    assert_eq!(ctx.next_temp(), "%temp1");
    assert_eq!(ctx.next_temp(), "%temp2");
}

#[test]
fn test_type_mapping() {
    let generator = LlvmCodeGenerator::new().unwrap();
    
    // Test CURSED to LLVM type mapping
    assert_eq!(generator.map_cursed_type_to_llvm("normie"), "i64");
    assert_eq!(generator.map_cursed_type_to_llvm("sus"), "i64");
    assert_eq!(generator.map_cursed_type_to_llvm("facts"), "i1");
    assert_eq!(generator.map_cursed_type_to_llvm("tea"), "i8*");
    assert_eq!(generator.map_cursed_type_to_llvm("vibes"), "double");
    assert_eq!(generator.map_cursed_type_to_llvm("void"), "void");
    assert_eq!(generator.map_cursed_type_to_llvm("unknown"), "i8*");
}

#[test]
fn test_llvm_value_operations() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Test temporary ID generation
    let id1 = generator.next_temp_id();
    let id2 = generator.next_temp_id();
    assert_ne!(id1, id2, "Should generate unique IDs");
    
    // Test temporary name generation
    generator.reset_counters();
    let name1 = generator.next_temp_name();
    let name2 = generator.next_temp_name();
    assert_eq!(name1, "%temp_0");
    assert_eq!(name2, "%temp_1");
    
    // Test block name generation
    generator.reset_counters();
    let block1 = generator.next_block_name("test");
    let block2 = generator.next_block_name("test");
    assert_eq!(block1, "test_block_0");
    assert_eq!(block2, "test_block_1");
}

#[test]
fn test_result_option_type_checking() {
    let generator = LlvmCodeGenerator::new().unwrap();
    
    // Test Result type checking
    assert!(generator.is_result_type("Result<i32, String>"));
    assert!(generator.is_result_type("Result<(), Error>"));
    assert!(!generator.is_result_type("Option<i32>"));
    assert!(!generator.is_result_type("i32"));
    
    // Test Option type checking
    assert!(generator.is_option_type("Option<i32>"));
    assert!(generator.is_option_type("Option<String>"));
    assert!(!generator.is_option_type("Result<i32, Error>"));
    assert!(!generator.is_option_type("Vec<i32>"));
    
    // Test type generation
    assert_eq!(generator.get_result_type("i32", "String"), "Result<i32, String>");
    assert_eq!(generator.get_option_type("i32"), "Option<i32>");
    assert_eq!(generator.get_error_type(), "CursedError");
}

#[test]
fn test_runtime_function_declarations() {
    let gc_config = GcConfig::default();
    let integration = LlvmGcIntegration::new(gc_config).unwrap();
    
    let declarations = integration.generate_runtime_function_declarations();
    
    // Check for all required runtime functions
    assert!(declarations.contains("declare i8* @cursed_allocate_object"), "Should declare allocation function");
    assert!(declarations.contains("declare void @cursed_safe_point"), "Should declare safe point function");
    assert!(declarations.contains("declare void @cursed_write_barrier"), "Should declare write barrier function");
    assert!(declarations.contains("declare void @cursed_collect_garbage"), "Should declare collection function");
    assert!(declarations.contains("declare i8* @cursed_spawn_goroutine"), "Should declare goroutine spawn function");
    assert!(declarations.contains("declare void @cursed_yield_goroutine"), "Should declare goroutine yield function");
    assert!(declarations.contains("declare i64 @cursed_object_type_id"), "Should declare type introspection function");
    assert!(declarations.contains("declare i64 @cursed_object_size"), "Should declare size introspection function");
}

/// Test the complete compilation pipeline with a simple program
#[test]
fn test_end_to_end_compilation() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Initialize all systems
    let gc_config = GcConfig::default();
    generator.initialize_gc_integration(gc_config).unwrap();
    
    // Simulate compiling a simple program
    // In a real implementation, this would parse actual CURSED code
    let source = r#"
        slay main() -> int {
            sus x = 42;
            facts result = x + 10;
            yolo result;
        }
    "#;
    
    // Generate IR for the program
    let ir_result = generator.generate_ir_with_gc(source);
    assert!(ir_result.is_ok(), "End-to-end compilation should succeed");
    
    let ir = ir_result.unwrap();
    
    // Verify the generated IR has all necessary components
    assert!(ir.contains("target datalayout"), "Should have data layout");
    assert!(ir.contains("target triple"), "Should have target triple");
    assert!(ir.contains("define i32 @main"), "Should have main function");
    assert!(ir.contains("ret i32 0"), "Should have return statement");
    assert!(ir.contains("cursed_allocate_object"), "Should have GC runtime functions");
    assert!(ir.contains("cursed_safe_point"), "Should have safe point calls");
}
