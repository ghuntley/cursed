//! Integration tests for slice literal compilation in the CURSED language.
//!
//! These tests verify that slice literals can be properly compiled to LLVM IR
//! and that the resulting code produces correct behavior.

use cursed::ast::slice_literal::SliceLiteral;
use cursed::ast::literals::IntegerLiteral;
use cursed::ast::identifiers::Identifier;
use cursed::codegen::llvm::{SliceLiteralCompiler, create_slice_literal_compiler};
use cursed::core::type_checker::Type;
use cursed::lexer::Token;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::OptimizationLevel;
use std::error::Error;
use tracing_test::traced_test;

mod common;

/// Test compilation of an empty slice literal
#[traced_test]
#[test]
fn test_empty_slice_literal_compilation() -> Result<(), Box<dyn Error>> {
    // init_tracing!();
    common::tracing::setup();
    
    let context = Context::create();
    let module = context.create_module("test_empty_slice");
    let builder = context.create_builder();
    
    // Create function to contain our slice literal
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Create empty slice literal []normie{}
    let token = Token::LeftBracket;
    let element_type = Box::new(Identifier::new(Token::Ident("normie".to_string()), "normie".to_string()));
    let elements = vec![];
    
    let slice_literal = SliceLiteral::new(token, element_type, elements);
    
    // Compile the slice literal
    let compiler = create_slice_literal_compiler();
    let result = compiler.compile_slice_literal(
        &context,
        &module,
        &builder,
        &slice_literal,
        &Type::Normie,
    );
    
    assert!(result.is_ok(), "Empty slice literal compilation should succeed");
    
    let slice_value = result.unwrap();
    assert!(slice_value.is_struct_value(), "Slice literal should produce a struct value");
    
    println!("Empty slice literal compiled successfully");
    Ok(())
}

/// Test compilation of a slice literal with integer elements
#[traced_test]
#[test]
fn test_integer_slice_literal_compilation() -> Result<(), Box<dyn Error>> {
    // init_tracing!();
    common::tracing::setup();
    
    let context = Context::create();
    let module = context.create_module("test_integer_slice");
    let builder = context.create_builder();
    
    // Create function to contain our slice literal
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Create slice literal []normie{1, 2, 3}
    let token = Token::LeftBracket;
    let element_type = Box::new(Identifier::new(Token::Ident("normie".to_string()), "normie".to_string()));
    let elements = vec![
        Box::new(IntegerLiteral::new(Token::Int(1), 1)) as Box<dyn cursed::ast::Expression>,
        Box::new(IntegerLiteral::new(Token::Int(2), 2)) as Box<dyn cursed::ast::Expression>,
        Box::new(IntegerLiteral::new(Token::Int(3), 3)) as Box<dyn cursed::ast::Expression>,
    ];
    
    let slice_literal = SliceLiteral::new(token, element_type, elements);
    
    // Compile the slice literal
    let compiler = create_slice_literal_compiler();
    let result = compiler.compile_slice_literal(
        &context,
        &module,
        &builder,
        &slice_literal,
        &Type::Normie,
    );
    
    assert!(result.is_ok(), "Integer slice literal compilation should succeed");
    
    let slice_value = result.unwrap();
    assert!(slice_value.is_struct_value(), "Slice literal should produce a struct value");
    
    println!("Integer slice literal compiled successfully");
    Ok(())
}

/// Test slice struct type creation
#[traced_test]
#[test]
fn test_slice_struct_type_creation() -> Result<(), Box<dyn Error>> {
    // init_tracing!();
    common::tracing::setup();
    
    let context = Context::create();
    let compiler = create_slice_literal_compiler();
    
    // Test creating slice struct type for different element types
    let result_normie = compiler.create_slice_struct_type(&context, &Type::Normie);
    assert!(result_normie.is_ok(), "Slice struct type for normie should be created");
    
    let slice_type = result_normie.unwrap();
    assert_eq!(slice_type.count_fields(), 3, "Slice struct should have 3 fields: ptr, len, cap");
    
    // Test with other types
    let result_tea = compiler.create_slice_struct_type(&context, &Type::Tea);
    assert!(result_tea.is_ok(), "Slice struct type for tea should be created");
    
    let result_lit = compiler.create_slice_struct_type(&context, &Type::Lit);
    assert!(result_lit.is_ok(), "Slice struct type for lit should be created");
    
    println!("Slice struct types created successfully");
    Ok(())
}

/// Test slice memory allocation
#[traced_test]
#[test]
fn test_slice_memory_allocation() -> Result<(), Box<dyn Error>> {
    // init_tracing!();
    common::tracing::setup();
    
    let context = Context::create();
    let module = context.create_module("test_allocation");
    let builder = context.create_builder();
    
    // Create function to contain our allocation
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    let compiler = create_slice_literal_compiler();
    
    // Test allocating memory for 5 normie elements
    let result = compiler.allocate_slice_memory(
        &context,
        &module,
        &builder,
        &Type::Normie,
        5,
    );
    
    assert!(result.is_ok(), "Slice memory allocation should succeed");
    
    let ptr = result.unwrap();
    assert!(ptr.get_type().is_pointer_type(), "Allocated result should be a pointer");
    
    println!("Slice memory allocation successful");
    Ok(())
}

/// Test creating an empty slice
#[traced_test]
#[test]
fn test_empty_slice_creation() -> Result<(), Box<dyn Error>> {
    // init_tracing!();
    common::tracing::setup();
    
    let context = Context::create();
    let module = context.create_module("test_empty_slice");
    let builder = context.create_builder();
    
    // Create function to contain our empty slice
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    let compiler = create_slice_literal_compiler();
    
    let result = compiler.create_empty_slice(&context, &builder, &Type::Normie);
    assert!(result.is_ok(), "Empty slice creation should succeed");
    
    let empty_slice = result.unwrap();
    assert!(empty_slice.get_type().is_struct_type(), "Empty slice should be a struct");
    
    println!("Empty slice creation successful");
    Ok(())
}

/// Test slice literal string representation
#[traced_test]
#[test]
fn test_slice_literal_string_representation() -> Result<(), Box<dyn Error>> {
    // init_tracing!();
    common::tracing::setup();
    
    // Test empty slice string representation
    let token = Token::LeftBracket;
    let element_type = Box::new(Identifier::new(Token::Ident("normie".to_string()), "normie".to_string()));
    let elements = vec![];
    
    let empty_slice = SliceLiteral::new(token.clone(), element_type, elements);
    let empty_string = empty_slice.string();
    assert_eq!(empty_string, "[]normie{}", "Empty slice should have correct string representation");
    
    // Test slice with elements string representation
    let element_type2 = Box::new(Identifier::new(Token::Ident("normie".to_string()), "normie".to_string()));
    let elements2 = vec![
        Box::new(IntegerLiteral::new(Token::Int(1), 1)) as Box<dyn cursed::ast::Expression>,
        Box::new(IntegerLiteral::new(Token::Int(2), 2)) as Box<dyn cursed::ast::Expression>,
    ];
    
    let populated_slice = SliceLiteral::new(token, element_type2, elements2);
    let populated_string = populated_slice.string();
    assert!(populated_string.contains("[]normie{"), "Populated slice should contain slice prefix");
    assert!(populated_string.contains("1"), "Populated slice should contain first element");
    assert!(populated_string.contains("2"), "Populated slice should contain second element");
    
    println!("Slice literal string representations are correct");
    Ok(())
}

/// Test slice literal properties
#[traced_test]
#[test]
fn test_slice_literal_properties() -> Result<(), Box<dyn Error>> {
    // init_tracing!();
    common::tracing::setup();
    
    let token = Token::LeftBracket;
    let element_type = Box::new(Identifier::new(Token::Ident("normie".to_string()), "normie".to_string()));
    
    // Test empty slice properties
    let empty_slice = SliceLiteral::new(token.clone(), element_type.clone_box(), vec![]);
    assert!(empty_slice.is_empty(), "Empty slice should be empty");
    assert_eq!(empty_slice.len(), 0, "Empty slice should have length 0");
    
    // Test populated slice properties
    let elements = vec![
        Box::new(IntegerLiteral::new(Token::Int(1), 1)) as Box<dyn cursed::ast::Expression>,
        Box::new(IntegerLiteral::new(Token::Int(2), 2)) as Box<dyn cursed::ast::Expression>,
        Box::new(IntegerLiteral::new(Token::Int(3), 3)) as Box<dyn cursed::ast::Expression>,
    ];
    
    let populated_slice = SliceLiteral::new(token, element_type, elements);
    assert!(!populated_slice.is_empty(), "Populated slice should not be empty");
    assert_eq!(populated_slice.len(), 3, "Populated slice should have length 3");
    
    println!("Slice literal properties are correct");
    Ok(())
}

/// Test slice literal cloning
#[traced_test]
#[test]
fn test_slice_literal_cloning() -> Result<(), Box<dyn Error>> {
    // init_tracing!();
    common::tracing::setup();
    
    let token = Token::LeftBracket;
    let element_type = Box::new(Identifier::new(Token::Ident("normie".to_string()), "normie".to_string()));
    let elements = vec![
        Box::new(IntegerLiteral::new(Token::Int(42), 42)) as Box<dyn cursed::ast::Expression>,
    ];
    
    let original_slice = SliceLiteral::new(token, element_type, elements);
    let cloned_slice = original_slice.clone_box();
    
    // Cast back to SliceLiteral to check properties
    let cloned_as_slice = cloned_slice.as_any().downcast_ref::<SliceLiteral>()
        .ok_or("Failed to downcast cloned slice")?;
    
    assert_eq!(original_slice.len(), cloned_as_slice.len(), "Cloned slice should have same length");
    assert_eq!(original_slice.string(), cloned_as_slice.string(), "Cloned slice should have same string representation");
    
    println!("Slice literal cloning works correctly");
    Ok(())
}
