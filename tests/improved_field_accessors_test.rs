use cursed::error::Error;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::ImprovedFieldAccessors;
use cursed::ast::declarations::SquadStatement;
use cursed::ast::FieldDefinition;
use cursed::ast::expressions::Identifier;
use cursed::core::type_checker::Type;

// Tests for the improved field accessors implementation
//
// This module tests the ImprovedFieldAccessors implementation which provides
// better error handling for LLVM operations.


#[path = "common.rs"]
mod common;

#[test]
fn test_improved_field_accessors() {
    // Initialize tracing for test debugging
    common::tracing::setup();
    
    // Create a context
    let context = inkwell::context::Context::create();
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", std::path::PathBuf::from("test.csd");
    
    // Create a simple struct with a couple of fields
    let struct_name = Identifier { value: "Point".to_string() };
    let fields = vec![
        FieldDefinition {
            name: Identifier { value: "x".to_string() },
            type_name: Identifier { value: "normie".to_string() },
        },
        FieldDefinition {
            name: Identifier { value: "y".to_string() },
            type_name: Identifier { value: "normie".to_string() },
        },
    ];
    
    let squad_stmt = SquadStatement {
        name: struct_name,
        fields,
        type_parameters: vec![],
        generic_constraints: vec![],
    };
    
    // Generate LLVM struct type using standard struct monomorphization
    let specialized_name = "Point";
    cursed::codegen::llvm::StructMonomorphization::generate_specialized_struct(
        &mut code_gen,
        &squad_stmt,
        specialized_name,
        &[],
    ).expect("Failed to generate struct type");
    
    // Generate field accessors using the improved implementation
    let result = ImprovedFieldAccessors::generate_improved_field_accessors(
        &mut code_gen,
        &squad_stmt,
        specialized_name,
        &[],
    );
    
    // Check that it succeeded
    assert!(result.is_ok(), "Failed to generate field accessors: {:?}", result.err())
    
    // Verify that the accessor functions were created
    let getter_x = code_gen.module().get_function("Point_get_x");
    let getter_y = code_gen.module().get_function("Point_get_y");
    let setter_x = code_gen.module().get_function("Point_set_x");
    let setter_y = code_gen.module().get_function("Point_set_y");
    
    assert!(getter_x.is_some(), "x getter not created");
    assert!(getter_y.is_some(), "y getter not created");
    assert!(setter_x.is_some(), "x setter not created");
    assert!(setter_y.is_some(), "y setter not created");
    
    // Print the generated LLVM IR for validation
    println!("Generated LLVM IR:\n{}", code_gen.module().print_to_string().to_string());
}