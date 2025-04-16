//! Tests for assignment operations in the LLVM code generator

use cursed::ast::expressions::literals::IntegerLiteral;
use cursed::ast::expressions::AssignmentExpression;
use cursed::ast::expressions::Identifier;
use cursed::ast::traits::Expression;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::AssignmentCompilation;
use cursed::codegen::llvm::VariableHandling;
use cursed::codegen::llvm::ExpressionCompilation;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

#[test]
fn test_basic_assignment() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));

    // Create a function context for testing
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_assign", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);

    // Create a variable to assign to
    let var_name = "test_var";
    let var_ptr = generator.builder().build_alloca(i32_type, var_name).unwrap();
    generator.add_variable_with_type(var_name, var_ptr, i32_type.into()).unwrap();

    // Create the integer value to assign
    let int_lit = IntegerLiteral {
        token: Token::new(TokenType::Int, "42").token_literal(),
        value: 42,
    };

    // Create the variable identifier
    let ident = Identifier {
        token: Token::new(TokenType::Identifier, var_name).token_literal(),
        value: var_name.to_string(),
    };

    // Create the assignment expression
    let assign_expr = AssignmentExpression {
        token: Token::new(TokenType::Assign, "=").token_literal(),
        name: ident,
        value: Box::new(int_lit),
    };

    // Compile the assignment
    let result = generator.compile_assignment_expr(&assign_expr).unwrap();
    assert!(result.is_some(), "Failed to compile assignment: result is None");

    // Load the variable to check if assignment worked
    let load_result = generator.builder().build_load(i32_type, var_ptr, "load_test").unwrap();
    let int_value = load_result.into_int_value();
    
    // Check that the variable now has the assigned value
    assert_eq!(int_value.get_zero_extended_constant().unwrap(), 42);
}