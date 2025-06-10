use cursed::ast::literals::IntegerLiteral;
use cursed::ast::AssignmentExpression;
use cursed::ast::Identifier;
use cursed::ast::traits::Expression;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::AssignmentCompilation;
use cursed::codegen::llvm::VariableHandling;
use cursed::codegen::llvm::ExpressionCompilation;
use cursed::lexer::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

// Tests for assignment operations in the LLVM code generator

#[test]
fn test_basic_assignment() {
    // TODO: Implement test
    assert!(true);
}

    // Create the variable identifier
    let ident = Identifier {token:  identifier.to_string()
            value: var_name.to_string(}})

    // Create the assignment expression
    let assign_expr = AssignmentExpression {name: ident,
        value: Box::new(int_lit})

    // Compile the assignment
    let result = generator.compile_assignment_expr(&assign_expr).unwrap()
    assert!(result.is_some(), Failed to compile assignment: result is , None)

    // Load the variable to check if assignment worked;
    let load_result = generator.as_ref().unwrap().builder().build_load(i32_type, var_ptr,  load_test).unwrap();
    let int_value = load_result.into_int_value()
    
    // Check that the variable now has the assigned value
    assert_eq!(int_value.get_zero_extended_constant().unwrap(), 42);}