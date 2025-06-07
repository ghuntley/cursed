use cursed::ast::declarations::{SquadStatement, TypeParameter};
use cursed::ast::expressions::Identifier;
use cursed::ast::FieldStatement;
use cursed::core::type_checker::Type;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::integrated_monomorphization::IntegratedMonomorphization;
use cursed::codegen::llvm::lru_field_accessors::LruCachedFieldAccessors;
use cursed::lexer::Token;
use inkwell::context::Context;
use std::time::Instant;
use crate::common::tracing::setup;

// Integration tests for field accessor generation in the monomorphization system


mod common;

#[test]
fn test_field_accessors_integration() {
    setup();
    let context = Context::create();
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", std::path::PathBuf::from("test.csd");
    
    // Create a simple generic struct for testing
    let generic_struct = create_test_generic_struct();
    
    // Specialize it with concrete types
    let type_args = vec![Type::Normie, Type::Tea];
    let specialized_name = "Point_Normie_Tea";
    
    // Generate struct with field accessors
    let result = code_gen.generate_specialized_struct_with_accessors(
        &generic_struct,
        specialized_name,
        &type_args,
    );
    
    assert!(result.is_ok(), "Failed to generate specialized struct with accessors: {:?}", result.err());
    
    // Verify the field accessors were created
    assert!(code_gen.module().get_function("Point_Normie_Tea_get_x").is_some(), 
            "x getter not found");
    assert!(code_gen.module().get_function("Point_Normie_Tea_set_x").is_some(), 
            "x setter not found");
    assert!(code_gen.module().get_function("Point_Normie_Tea_get_name").is_some(), 
            "name getter not found");
    assert!(code_gen.module().get_function("Point_Normie_Tea_set_name").is_some(), 
            "name setter not found");
    
    // Test with a different type specialization
    let type_args2 = vec![Type::Thicc, Type::Lit];
    let specialized_name2 = "Point_Thicc_Lit";
    
    // Generate struct with field accessors for the second specialization
    let result2 = code_gen.generate_specialized_struct_with_accessors(
        &generic_struct,
        specialized_name2,
        &type_args2,
    );
    
    assert!(result2.is_ok(), "Failed to generate second specialized struct with accessors: {:?}", result2.err();
    
    // Verify the field accessors were created for the second specialization
    assert!(code_gen.module().get_function("Point_Thicc_Lit_get_x").is_some(), 
            "x getter not found for second specialization");
    assert!(code_gen.module().get_function("Point_Thicc_Lit_set_x").is_some(), 
            "x setter not found for second specialization");
    assert!(code_gen.module().get_function("Point_Thicc_Lit_get_name").is_some(), 
            "name getter not found for second specialization");
    assert!(code_gen.module().get_function("Point_Thicc_Lit_set_name").is_some(), 
            "name setter not found for second specialization");
    
    // Test LRU caching by regenerating first struct
    let start = Instant::now();
    let cache_result = code_gen.generate_specialized_struct_with_accessors(
        &generic_struct,
        specialized_name,
        &type_args,
    );
    let cache_time = start.elapsed();
    
    assert!(cache_result.is_ok(), "Failed to regenerate specialized struct with accessors: {:?}", cache_result.err());
    println!("Cache regeneration took: {:?}", cache_time);
    
    // Verify module integrity
    assert!(code_gen.module().verify().is_ok(), "Module verification failed after field accessor generation");
}

/// Helper function to create a test generic struct
fn create_test_generic_struct() -> SquadStatement {
    SquadStatement {
        token: "squad".to_string(),
        name: Identifier {
            token: "Point".to_string(),
            value: "Point".to_string(),
        },
        type_parameters: vec![
            TypeParameter {
                token: Token::Identifier("T".to_string(),
                name: "T".to_string(),
                value: "T".to_string(),
                constraints: Vec::new(),
            },
            TypeParameter {
                token: Token::Identifier("U".to_string(),
                name: "U".to_string(),
                value: "U".to_string(),
                constraints: Vec::new(),
            },
        ],
        generic_constraints: Vec::new(),
        fields: vec![
            FieldStatement {
                token: "x".to_string(),
                name: Identifier {
                    token: "x".to_string(),
                    value: "x".to_string(),
                },
                type_name: Identifier {
                    token: "T".to_string(),
                    value: "T".to_string(),
                },
            },
            FieldStatement {
                token: "name".to_string(),
                name: Identifier {
                    token: "name".to_string(),
                    value: "name".to_string(),
                },
                type_name: Identifier {
                    token: "U".to_string(),
                    value: "U".to_string(),
                },
            },
        ],
    }
}