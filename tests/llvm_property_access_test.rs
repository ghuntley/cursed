//! Tests for property access (field access) in the LLVM code generator

use cursed::ast::expressions::dot_expression::DotExpression;
use cursed::ast::expressions::Identifier;
use cursed::ast::traits::Expression;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::ExpressionCompilation;
use cursed::codegen::llvm::PropertyAccessCompilation;
use cursed::codegen::llvm::VariableHandling;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

#[test]
fn test_property_access() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));

    // Create a function context for testing
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_prop_access", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);

    // Create a struct type with one field (simplistic test case)
    let struct_type = context.opaque_struct_type("test_struct");
    struct_type.set_body(&[i32_type.into()], false);
    
    // Create an instance of the struct and store a value
    let struct_ptr = generator.builder().build_alloca(struct_type, "struct_var").unwrap();
    
    // Store 42 in the struct's field
    let indices = [context.i32_type().const_int(0, false), context.i32_type().const_int(0, false)];
    let field_ptr = unsafe {
        generator.builder().build_gep(struct_type, struct_ptr, &indices, "field_ptr").unwrap()
    };
    
    let value = context.i32_type().const_int(42, false);
    generator.builder().build_store(field_ptr, value).unwrap();
    
    // Add the struct variable to the generator's variable table
    generator.add_variable_with_type("test_obj", struct_ptr, struct_type.into()).unwrap();
    
    // Create expression to access test_obj.value
    let object_ident = Identifier {
        token: Token::new(TokenType::Identifier, "test_obj"),
        value: "test_obj".to_string(),
    };
    
    let dot_expr = DotExpression {
        token: ".".to_string(),
        object: Box::new(object_ident),
        property: "value".to_string(),
    };
    
    // Test the property access
    let result = generator.compile_property_access(&dot_expr);
    assert!(result.is_ok(), "Failed to compile property access: {:?}", result.err());
    
    // Verify the result is 42
    let value = result.unwrap();
    assert!(value.is_int_value(), "Result should be an integer");
    
    let int_value = value.into_int_value();
    assert_eq!(int_value.get_zero_extended_constant().unwrap(), 42);
}