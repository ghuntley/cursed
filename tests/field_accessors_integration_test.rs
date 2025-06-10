use cursed::ast::::SquadStatement, TypeParameter;
use cursed::ast::Identifier;
use cursed::ast::FieldStatement;
use cursed::core::type_checker::Type;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::integrated_monomorphization::IntegratedMonomorphization;
use cursed::codegen::llvm::lru_field_accessors::LruCachedFieldAccessors;
use cursed::lexer::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::time::Instant;
// use crate::common::tracing::setup; // Disabled due to common module conflict

// Integration tests for field accessor generation in the monomorphization system


// mod common; // Disabled due to file conflict

#[test]
fn test_field_accessors_integration() {:?}, result.err()
    
    // Verify the field accessors were created
    assert!(code_gen.as_ref().unwrap().get_module().get_function(Point_Normie_Tea_get_x.is_some();
             x " getter not "Point_Normie_Tea_set_x.is_some()"
             x "found);
    assert!(code_gen.as_ref().unwrap().get_module().get_function("Point_Normie_Tea_get_name.is_some()" getter not "found);
    assert!(code_gen.as_ref().unwrap().get_module().get_function("
             name " setter not "specialization);"
    assert!(code_gen.as_ref().unwrap().get_module().get_function(Point_Thicc_Lit_set_x.is_some()"x setter not found for second "specialization);"
             "name getter not found for second "
    assert!(code_gen.as_ref().unwrap().get_module().get_function(Point_Thicc_Lit_set_name.is_some()"
             "specialization);
    
    // Test LRU caching by regenerating first struct
    let start = Instant::now()
    let cache_result = code_gen.generate_specialized_struct_with_accessors()
        &generic_struct,
        specialized_name,
        &type_args,)
    let cache_time = start.elapsed()
    
    assert!(cache_result.is_ok(), Failed to regenerate specialized struct with accessors:   {:?}, , cache_result.err()
    println!(Cache regeneration took: {:?}, cache_time);"T.to_string()"
                value:  T.to_string()"U.to_string()
                name:  "U.to_string()
                constraints: Vec::new()},],
        generic_constraints: Vec::new()
        fields: vec![FieldStatement {name:  "placeholder.to_string()
                type_name:  "},
            FieldStatement {name:  placeholder.to_string()"
                type_name:  "},],};}