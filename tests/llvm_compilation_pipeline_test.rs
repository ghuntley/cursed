/// Integration test for LLVM compilation pipeline
/// 
/// This test verifies that the main compilation pipeline can successfully
/// compile CURSED programs to LLVM IR using the newly implemented compile() method.

#[path = "common.rs"]
mod common;

use crate::common::{init_tracing, Timer};
use cursed::ast::{
    Program, FunctionStatement, Identifier, BlockStatement, ReturnStatement,
    PackageStatement, ImportStatement, LetStatement, FactsStatement
};
use cursed::ast::expressions::{Parameter, Literal, LiteralValue};
use cursed::ast::traits::{Statement, Expression};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::Token;

#[test]
fn test_empty_program_compilation() {
    init_tracing!();
    let _timer = Timer::new("empty_program_compilation");
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    let program = Program::new();
    
    let result = generator.compile(&program);
    assert!(result.is_ok(), "Empty program compilation should succeed");
    
    // Generate IR to verify it's valid
    let ir = generator.generate_ir("").expect("Failed to generate IR");
    assert!(ir.contains("target triple"), "IR should contain target information");
    assert!(ir.contains("define i32 @main()"), "IR should contain main function");
}

#[test]
fn test_program_with_package() {
    init_tracing!();
    let _timer = Timer::new("program_with_package");
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    let mut program = Program::with_package("test_package".to_string());
    
    // Add a package statement
    let package_stmt = PackageStatement {
        token: Token::new("vibe".to_string(), "vibe".to_string(), 1, 1),
        name: "test_package".to_string(),
    };
    program.add_statement(Box::new(package_stmt));
    
    let result = generator.compile(&program);
    assert!(result.is_ok(), "Package program compilation should succeed");
    
    let ir = generator.generate_ir("").expect("Failed to generate IR");
    assert!(ir.contains("Module: test_package"), "IR should contain module name");
}

#[test]
fn test_program_with_imports() {
    init_tracing!();
    let _timer = Timer::new("program_with_imports");
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    let mut program = Program::new();
    
    // Add import statements
    let import1 = ImportStatement::new(
        Token::new("yeet".to_string(), "yeet".to_string(), 1, 1),
        "stdlib::io".to_string()
    );
    let import2 = ImportStatement::with_alias(
        Token::new("yeet".to_string(), "yeet".to_string(), 2, 1),
        "stdlib::math".to_string(),
        "math".to_string()
    );
    
    program.add_import(import1);
    program.add_import(import2);
    
    let result = generator.compile(&program);
    assert!(result.is_ok(), "Import program compilation should succeed");
}

#[test]
fn test_simple_function_compilation() {
    init_tracing!();
    let _timer = Timer::new("simple_function_compilation");
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    let mut program = Program::new();
    
    // Create a simple function: slay greet() { yolo "Hello" }
    let function = FunctionStatement::new(
        "slay".to_string(),
        Identifier::new("greet".to_string(), "greet".to_string()),
        vec![], // no parameters
        None,   // no return type
        BlockStatement::new("greet_body".to_string(), vec![
            Box::new(ReturnStatement::with_value(Box::new(Literal::new(
                LiteralValue::String("Hello".to_string()),
                1, 1
            ))))
        ])
    );
    
    program.add_statement(Box::new(function));
    
    let result = generator.compile(&program);
    assert!(result.is_ok(), "Function compilation should succeed: {:?}", result);
    
    let ir = generator.generate_ir("slay greet").expect("Failed to generate IR");
    assert!(ir.contains("target triple"), "IR should contain target information");
}

#[test]
fn test_variable_declaration_compilation() {
    init_tracing!();
    let _timer = Timer::new("variable_declaration_compilation");
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    let mut program = Program::new();
    
    // Create a variable declaration: sus x = 42
    let var_stmt = LetStatement::new(
        "sus".to_string(),
        Identifier::new("x".to_string(), "x".to_string()),
        Some(Box::new(Literal::new(LiteralValue::Integer(42), 1, 5)))
    );
    
    program.add_statement(Box::new(var_stmt));
    
    let result = generator.compile(&program);
    assert!(result.is_ok(), "Variable declaration compilation should succeed");
}

#[test]
fn test_constant_declaration_compilation() {
    init_tracing!();
    let _timer = Timer::new("constant_declaration_compilation");
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    let mut program = Program::new();
    
    // Create a constant declaration: facts PI = 3.14159
    let const_stmt = FactsStatement::new(
        "facts".to_string(),
        Identifier::new("PI".to_string(), "PI".to_string()),
        Box::new(Literal::new(LiteralValue::Float(3.14159), 1, 10))
    );
    
    program.add_statement(Box::new(const_stmt));
    
    let result = generator.compile(&program);
    assert!(result.is_ok(), "Constant declaration compilation should succeed");
}

#[test]
fn test_complete_compilation_workflow() {
    init_tracing!();
    let _timer = Timer::new("complete_compilation_workflow");
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    let mut program = Program::with_package("test_app".to_string());
    
    // Add import
    program.add_import(ImportStatement::new(
        Token::new("yeet".to_string(), "yeet".to_string(), 1, 1),
        "stdlib::io".to_string()
    ));
    
    // Add constant
    let const_stmt = FactsStatement::new(
        "facts".to_string(),
        Identifier::new("VERSION".to_string(), "VERSION".to_string()),
        Box::new(Literal::new(LiteralValue::String("1.0.0".to_string()), 2, 15))
    );
    program.add_statement(Box::new(const_stmt));
    
    // Add variable
    let var_stmt = LetStatement::new(
        "sus".to_string(),
        Identifier::new("counter".to_string(), "counter".to_string()),
        Some(Box::new(Literal::new(LiteralValue::Integer(0), 3, 15)))
    );
    program.add_statement(Box::new(var_stmt));
    
    // Add main function
    let main_func = FunctionStatement::new(
        "slay".to_string(),
        Identifier::new("main".to_string(), "main".to_string()),
        vec![],
        None,
        BlockStatement::new("main_body".to_string(), vec![
            Box::new(ReturnStatement::with_value(Box::new(Literal::new(
                LiteralValue::Integer(0),
                4, 10
            ))))
        ])
    );
    program.add_statement(Box::new(main_func));
    
    // Test complete workflow
    let source = r#"
vibe test_app

yeet "stdlib::io"

facts VERSION = "1.0.0"
sus counter = 0

slay main() {
    yolo 0
}
"#;
    
    let ir = generator.compile_program(&program, source).expect("Complete compilation should succeed");
    
    // Verify IR contains expected elements
    assert!(ir.contains("Module: test_app"), "IR should contain module name");
    assert!(ir.contains("target triple"), "IR should contain target information");
    assert!(ir.len() > 100, "IR should be substantial (got {} bytes)", ir.len());
    
    println!("Generated LLVM IR:\n{}", ir);
}

#[test]
fn test_type_inference() {
    init_tracing!();
    let _timer = Timer::new("type_inference");
    
    let generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    
    // Test integer literal
    let int_literal = Literal::new(LiteralValue::Integer(42), 1, 1);
    let int_type = generator.infer_type_from_expression(&int_literal).expect("Type inference should work");
    assert_eq!(int_type, "i64", "Integer should map to i64");
    
    // Test string literal
    let str_literal = Literal::new(LiteralValue::String("hello".to_string()), 1, 1);
    let str_type = generator.infer_type_from_expression(&str_literal).expect("Type inference should work");
    assert_eq!(str_type, "i8*", "String should map to i8*");
    
    // Test boolean literal
    let bool_literal = Literal::new(LiteralValue::Boolean(true), 1, 1);
    let bool_type = generator.infer_type_from_expression(&bool_literal).expect("Type inference should work");
    assert_eq!(bool_type, "i1", "Boolean should map to i1");
}

#[test]
fn test_cursed_type_mapping() {
    init_tracing!();
    let _timer = Timer::new("cursed_type_mapping");
    
    let generator = LlvmCodeGenerator::new().expect("Failed to create LLVM generator");
    
    // Test Gen Z slang type mappings
    assert_eq!(generator.map_cursed_type_to_llvm("normie"), "i64");
    assert_eq!(generator.map_cursed_type_to_llvm("sus"), "i64");
    assert_eq!(generator.map_cursed_type_to_llvm("facts"), "i1");
    assert_eq!(generator.map_cursed_type_to_llvm("tea"), "i8*");
    assert_eq!(generator.map_cursed_type_to_llvm("vibes"), "double");
    assert_eq!(generator.map_cursed_type_to_llvm("void"), "void");
    
    // Test unknown type defaults to generic pointer
    assert_eq!(generator.map_cursed_type_to_llvm("unknown_type"), "i8*");
}
