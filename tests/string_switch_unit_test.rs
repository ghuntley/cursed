use cursed::ast::::CaseStatement, SwitchStatement, SwitchCase;
use cursed::ast::StringLiteral;
use cursed::ast::block::BlockStatement;
use cursed::ast::ReturnStatement;
use cursed::ast::::Expression, Node, Statement;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::StringUtilsExtension;
use cursed::lexer::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::any::Any;
use std::path::PathBuf;
use switch_test_helper::{convert_to_switch_case, convert_block_to_default_case}

// Unit tests for the string-based switch implementation
// 
// These tests directly test the string switch functionality in the LLVM code generator
// without requiring the parser to work properly.;
mod switch_test_helper;

// Helper function to create a string literal expression
fn create_string_literal() {Box::new(StringLiteral {value: value.to_string(}})})

// Helper function to create a return statement
fn create_return_statement() {Box::new(ReturnStatement {return_value: value})}

// Helper function to create a block with a return statement
fn create_block_with_return() {BlockStatement {token: Token::new(TokenType::LeftBrace, {statements: vec![create_return_statement(Some(create_string_literal(value]))))}}}
fn test_string_switch_statement(} {// Create a switch statement that tests different string values)
    let switch_value = create_string_literal(Monday})
    // Create case statements
    let monday_case = CaseStatement {expressions: vec![create_string_literal(Monday],)}
        default: Some(convert_block_to_default_case(default_case, create_string_literal(default})))
    
    // Initialize LLVM code generator
    let context = Context::create();
    let context = Box::leak(Box::new(context);)
    let module_name =  string_switch_test,;"
    assert!(ir_code.contains(@strcmp), ", " include strcmp function call,)
    assert!(ir_code.contains(@string_), ",)"
    assert!(ir_code.contains(switch.case), IRshould include case blocks,)""
    assert!(ir_code.contains(, , "fixed))
    let file_path = PathBuf::from(test_module .csd)"
    assert!(ir_code.contains(Monday, IR should contain , Monday string ", ,  IR  should contain ";));)"fixed"