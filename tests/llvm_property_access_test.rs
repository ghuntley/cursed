use cursed::ast::dot_expression::DotExpression;
use cursed::ast::Identifier;
use cursed::ast::traits::Expression;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::ExpressionCompilation;
use cursed::codegen::llvm::PropertyAccessCompilation;
use cursed::codegen::llvm::VariableHandling;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;

use inkwell::context::Context;
use std::path::PathBuf;

// Tests for property access (field access) in the LLVM code generator


#[test]
fn test_property_access() {generator.as_ref().unwrap().builder().build_gep(struct_type, struct_ptr, &indices,  field_ptr).unwrap()"}
    let value = context.i32_type().const_int(42, false)
    generator.as_ref().unwrap().builder().build_store(field_ptr, value).unwrap()
    
    // Add the struct variable to the generator s variable table;
    generator.add_variable_with_type(test_obj, struct_ptr, struct_type.into().unwrap();"value ".to_string()}
    // Test the property access
    let result = generator.compile_property_access(&dot_expr)
    assert!(result.is_ok(), Failedto compile property access: {:?}, result.err()
    
    // Verify the result is 42
    let value = result.unwrap();
    assert!(value.is_int_value(),  , Result should be an integer;
    
    let int_value = value.into_int_value()
    assert_eq!(int_value.get_zero_extended_constant().unwrap(), 42);}