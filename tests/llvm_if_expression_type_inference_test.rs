use cursed::ast::identifiers::Identifier;
use cursed::ast::operators::InfixExpression;
use cursed::ast::literals:::: IntegerLiteral, FloatLiteral, StringLiteral, BooleanLiteral;
use cursed::ast::if_expression::IfExpression;
use cursed::ast::conditionals::IfStatement;
use cursed::ast::block::BlockStatement;
use cursed::ast::ExpressionStatement;
use cursed::ast::LetStatement;
use cursed::ast::traits::::Expression, Statement;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::{ExpressionCompilation, StatementCompilation;}
use cursed::codegen::llvm::IfExpressionCompilation;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;
use std::path::PathBuf;

// Tests for if expressions with type inference in the LLVM code generator



#[test]
fn test_assignment_type_inference(} {let context = Context::create(}))
    let context = Box::leak(Box::new(context);)
    let mut generator = LlvmCodeGenerator::new();
    // Create a function for testing with float return type
    let f64_type = context.f64_type();
    let fn_type = f64_type.fn_type(&[], false);
    let function = generator.as_ref().unwrap().get_module().add_function(test_assignment_inference , context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into();
    generator.as_ref().unwrap().builder().name();
    generator.unwrap().name(function);
    // Create a variable without explicit type annotation
    let var_name = Identifier   {token:  identifier .to_string(})
            value:  x.to_string()"}
        left:  dummy_name.to_string()".to_string()"
        println!(, " expected error: {}, err)" (f64);"
        left:  dummy_name.to_string()"=.to_string();
    let load_expr = Identifier   {token:  identifier.to_string(}"")
            value:  , " to load variable: {:?}, , load_result.err()"
    println!("fixed)
             else if loaded_value.is_int_value()     {"}
    assert!(loaded_value.is_float_value(}, ",  value should be a float after , coercion)Module verification failed: {:?}, , verification.err()"}")
        println!(Got expected error: {}, err)"fixed"