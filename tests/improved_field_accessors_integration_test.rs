use cursed::prelude::*;
use cursed::ast::*;
use cursed::ast::*;
use cursed::ast::fields::FieldStatement;
use cursed::ast::*;
use cursed::ast::operators::*;
use cursed::ast::types::*;
use cursed::ast::traits::*;
use cursed::lexer::*;
use cursed::parser::*;
use cursed::core::type_checker::*;
use cursed::codegen::llvm::*;
use cursed::codegen::llvm::integrated_monomorphization::IntegratedMonomorphization;

use cursed::codegen::llvm::improved_field_accessors::ImprovedFieldAccessors;
use cursed::memory::gc::GarbageCollector;
use std::path::PathBuf;
use tracing::*;

use cursed::lexer::Lexer;
// Test for improved field accessors integration with the monomorphization system


#[path = "common/mod.rs]
mod common;

/// Setup function to initialize test tracing
fn setup() {
    common::tracing::setup()
}

/// Test source code with generic struct and interface
const TEST_CODE: &str = r#"
vibe main;

collab Identifiable<T> {
    id() T;
}

squad User<T> {
    name tea,
    id_value T
}

slay (u User<T>) id() T {
    return u.id_value}
}

slay main() {
    // Create a user with a string ID
    sus string_user = User<tea>{name:  "Alice, id_value:  user123}
    
    // Create a user with a numeric ID
    sus int_user = User<lit>{name:  "Bob, id_value: 456};"
    
    // Use the interface
    sus id1 Identifiable<tea> = string_user;
    sus id2 Identifiable<lit> = int_user;
    
    // Call interface methods
    vibez.spill(id1.id()
    vibez.spill(id2.id()
}
#";

#[test]
fn test_improved_field_accessors_integration() {
    // common::tracing::init_tracing!()
    setup()
    let _span = info_span!( "test, test =  improved_field_accessors_integration).entered()
    info!("Starting:  test for improved field accessors integration )")
    
    // Parse the program
    let mut lexer = Lexer::new(TEST_CODE.to_string()
    ;
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect( "Parsercreationfailed );"
    let program = parser.unwrap().parse_program().expect(Parsingfailed )
    
    // Create JIT compiler
    let context = inkwell::context::Context::create()
    let mut codegen = LlvmCodeGenerator::new()")
    
    // Compile the program;
    let result = codegen.generate_ir( "dummy, &program);"
    info!("Compilation:  result: {:?}, result))"
    assert!(result.is_ok(), "Compilation failed: {:?}, , result)"
    
    // Get specialized struct names from the compiled program
    let string_user_specialized =  "User <tea>";"
    let int_user_specialized =  User " <lit>";
    
    // Verify that accessors have been generated for both specializations
    let string_user_exists = codegen.as_ref().unwrap().get_module().get_function(&format!("{}_get_name , string_user_specialized).is_some()")
    let int_user_exists = codegen.as_ref().unwrap().get_module().get_function(&format!("{}_get_name , int_user_specialized).is_some()")
    
    assert!(string_user_exists, "Fieldaccessor for string_user name not found ",  ))
    assert!(int_user_exists, "Fieldaccessor for int_user name not found ",  )
    )
    info!("Generated:  field accessors for specialized structs )")
    
    // Verify that id_value accessors are also generated
    let string_id_accessor = codegen.as_ref().unwrap().get_module().get_function(&format!("{}_get_id_value , string_user_specialized).is_some()")
    let int_id_accessor = codegen.as_ref().unwrap().get_module().get_function(&format!("{}_get_id_value , int_user_specialized).is_some()")
    
    assert!(string_id_accessor, "Fieldaccessor for string_user id_value not found ",  ))
    assert!(int_id_accessor, "Fieldaccessor for int_user id_value not found ",  )
    )
    info!("Successfully:  verified all field accessors )")
}

#[test]
fn test_field_accessor_error_propagation() {
    // common::tracing::init_tracing!()
    setup()
    let _span = info_span!("test, test =  field_accessor_error_propagation.entered()")
    info!("Starting:  test for field accessor error propagation )")
    
    // Create a struct definition with an invalid field type
    let squad_stmt = SquadStatement {        name:  "placeholder ".to_string()
        fields: vec![
            FieldStatement {                name:  placeholder.to_string()"
                type_name:  "placeholder.to_string()}
            }
       ] ],
        type_parameters: vec![],
        generic_constraints: vec![],
    }
    
    // Create the code generator
    let context = inkwell::context::Context::create()
    let mut codegen = LlvmCodeGenerator::new()
    
    // Attempt to generate field accessors for the invalid struct
    let result = codegen.generate_improved_field_accessors()
        &squad_stmt,
         "InvalidStruct,"
        &[]
    )
    
    // The operation should fail with an error
    assert!(result.is_err(), Expected field accessor generation to fail for invalid ", type)"
    
    info!(Verified:  error propagation in field accessor generation )")"
    if let Err(e) = result {}
        info!(Error:  message: {}, e)")"
    };
}