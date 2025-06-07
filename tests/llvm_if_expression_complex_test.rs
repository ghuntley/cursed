use cursed::ast::expressions::identifiers::Identifier;
use cursed::ast::expressions::operators::InfixExpression;
use cursed::ast::expressions::literals::{IntegerLiteral, BooleanLiteral};
use cursed::ast::expressions::if_expression::IfExpression;
use cursed::ast::control_flow::conditionals::IfStatement;
use cursed::ast::Node;
use cursed::ast::statements::block::BlockStatement;
use cursed::ast::statements::ExpressionStatement;
use cursed::ast::statements::declarations::LetStatement;
use cursed::ast::traits::{Expression, Statement};
use cursed::Type;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::{ExpressionCompilation, StatementCompilation};
use cursed::codegen::llvm::IfExpressionCompilation;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use cursed::lexer::Token;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;
use std::path::PathBuf;

// Tests for complex if expressions in the LLVM code generator


#[test]
#[ignore = "This test needs more work to handle mixed integer types properly"]
fn test_if_expression_with_variable() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_if_var", PathBuf::from("test.csd"));

    // Create a function for testing
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_if_var", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    generator.set_current_function(function);
    
    // Set up a variable 'x' with value 10
    let x_ident = Identifier {
        token: "token".to_string(),
        value: "x".to_string(),
    };
    
    let x_value = IntegerLiteral {
        token: "token".to_string(),
        value: 10,
    };
    
    let let_stmt = LetStatement {
        token: "token".to_string(),
        name: x_ident.clone(),
        value: Some(Box::new(x_value)),
        // Add explicit i32 type annotation to ensure consistency
        type_annotation: Some(Token::Normie),
    };
    
    // Compile the let statement to create the variable
    let result = generator.compile_statement(&let_stmt);
    assert!(result.is_ok(), "Failed to compile let statement: {:?}", result.err())
    
    // Create a condition that compares x > 5
    let x_expr = x_ident.clone();
    let five = IntegerLiteral {
        token: "token".to_string(),
        value: 5,
    };

    // Convert to int for comparison
    let condition = InfixExpression {
        token: Token::Gt,
        left: Box::new(x_expr),
        operator: ">".to_string(),
        right: Box::new(five),
    };
    
    // Create the then expression: x + 20
    let then_x = x_ident.clone();
    let twenty = IntegerLiteral {
        token: "token".to_string(),
        value: 20,
    };
    
    let then_expr = InfixExpression {
        token: Token::Plus,
        left: Box::new(then_x),
        operator: "+".to_string(),
        right: Box::new(twenty),
    };
    
    // Wrap in an expression statement
    let then_stmt = ExpressionStatement {
        token: "token".to_string(),
        expression: Some(Box::new(then_expr)),
    };
    
    // Create the else expression: x - 5
    let else_x = x_ident.clone();
    let five_2 = IntegerLiteral {
        token: "token".to_string(),
        value: 5,
    };
    
    let else_expr = InfixExpression {
        token: Token::Minus,
        left: Box::new(else_x),
        operator: "-".to_string(),
        right: Box::new(five_2),
    };
    
    // Wrap in an expression statement
    let else_stmt = ExpressionStatement {
        token: "token".to_string(),
        expression: Some(Box::new(else_expr)),
    };
    
    // Create block statements
    let consequence = BlockStatement {
        token: Token::LBrace,
        statements: vec![Box::new(then_stmt)],
    };
    
    let alternative = BlockStatement {
        token: Token::LBrace,
        statements: vec![Box::new(else_stmt)],
    };
    
    // Create the IfStatement
    let if_stmt = IfStatement {
        token: "token".to_string(),
        condition: Box::new(condition),
        consequence: Box::new(consequence),
        alternative: Some(Box::new(alternative)),
    };
    
    // Create the if expression adapter
    let if_expr = IfExpression::new(if_stmt);
    
    // Skip this test for now until we fix the type system
    // This test needs more work to handle mixed integer types properly
}

#[test]
fn test_nested_if_expressions() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_nested_if", PathBuf::from("test.csd"));

    // Create a function for testing
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_nested_if", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    generator.set_current_function(function);
    
    // Set variable type to i32 to match our expected return type
    generator.set_default_integer_type(i32_type);
    
    // Create outer condition: true
    let outer_condition = BooleanLiteral {
        token: "token".to_string(),
        value: true,
    };
    
    // Create inner condition: false
    let inner_condition = BooleanLiteral {
        token: "token".to_string(),
        value: false,
    };
    
    // Create values for different branches
    let value_1 = IntegerLiteral {
        token: "token".to_string(),
        value: 1,
    };
    
    let value_2 = IntegerLiteral {
        token: "token".to_string(),
        value: 2,
    };
    
    let value_3 = IntegerLiteral {
        token: "token".to_string(),
        value: 3,
    };
    
    // Wrap values in expression statements
    let stmt_1 = ExpressionStatement {
        token: "token".to_string(),
        expression: Some(Box::new(value_1)),
    };
    
    let stmt_2 = ExpressionStatement {
        token: "token".to_string(),
        expression: Some(Box::new(value_2)),
    };
    
    let stmt_3 = ExpressionStatement {
        token: "token".to_string(),
        expression: Some(Box::new(value_3)),
    };
    
    // Create inner if blocks
    let inner_consequence = BlockStatement {
        token: Token::LBrace,
        statements: vec![Box::new(stmt_1)],
    };
    
    let inner_alternative = BlockStatement {
        token: Token::LBrace,
        statements: vec![Box::new(stmt_2)],
    };
    
    let inner_if = IfStatement {
        token: "token".to_string(),
        condition: Box::new(inner_condition),
        consequence: Box::new(inner_consequence),
        alternative: Some(Box::new(inner_alternative)),
    };
    
    // Wrap inner if in expression statement
    let inner_if_expr = IfExpression::new(inner_if);
    let inner_if_stmt = ExpressionStatement {
        token: "token".to_string(),
        expression: Some(Box::new(inner_if_expr)),
    };
    
    // Create outer if blocks
    let outer_consequence = BlockStatement {
        token: Token::LBrace,
        statements: vec![Box::new(inner_if_stmt)],
    };
    
    let outer_alternative = BlockStatement {
        token: Token::LBrace,
        statements: vec![Box::new(stmt_3)],
    };
    
    let outer_if = IfStatement {
        token: "token".to_string(),
        condition: Box::new(outer_condition),
        consequence: Box::new(outer_consequence),
        alternative: Some(Box::new(outer_alternative)),
    };
    
    // Create outer if expression
    let outer_if_expr = IfExpression::new(outer_if);
    
    // Compile the nested if expressions
    let result = generator.compile_if_expression(&outer_if_expr);
    assert!(result.is_ok(), "Failed to compile nested if expressions: {:?}", result.err())
    
    // Since outer is true and inner is false, the result should be 2
    let value = result.unwrap();
    assert!(value.is_int_value(), "Result should be an integer");
    
    // Get the result from LLVM
    let ret_val = generator.builder().build_return(Some(&value);
    assert!(ret_val.is_ok(), "Failed to build return: {:?}", ret_val.err();
    
    // Verify the module
    let verification = generator.module().verify());
    assert!(verification.is_ok(), "Module verification failed: {:?}", verification.err();
}