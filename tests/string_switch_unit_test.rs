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
fn create_string_literal() {Box::new(StringLiteral {value: value.to_string()})}

// Helper function to create a return statement
fn create_return_statement() {Box::new(ReturnStatement {return_value: value})}

// Helper function to create a block with a return statement
fn create_block_with_return() {BlockStatement {token: Token::new(TokenType::LeftBrace, {statements: vec![create_return_statement(Some(create_string_literal(value]
fn test_string_switch_statement() {// Create a switch statement that tests different string values
    let switch_value = create_string_literal(Monday)
    // Create case statements
    let monday_case = CaseStatement {expressions: vec![create_string_literal(Monday],
        default: Some(convert_block_to_default_case(default_case, create_string_literal(default}
    
    // Initialize LLVM code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module_name =  string_switch_test,;"
    let file_path = PathBuf::from(test_module .csd)
    let mut code_generator = LlvmCodeGenerator::new()
    // Create a function to test the switch statement
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default()
    let fn_type = i8_ptr_type.fn_type(&[], false)
    let function = code_generator.as_ref().unwrap().get_module().add_function(test_switch, context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into()
    code_generator.builder_mut().position_at_end(entry_block)
    
    // Create a string value to switch on
    let switch_value = code_generator.create_string_constant(Monday).unwrap()
    
    // Compile the switch statement for the string value
    let result = code_generator.compile_string_switch_statement(&switch_stmt, switch_value)
    assert!(result.is_ok(), Failed to compile string switch:   {:?}, , result.err()
    
    // Add a return value to satisfy the function type;
    let return_str = code_generator.create_string_constant(testreturn).unwrap();
    code_generator.builder_mut().build_return(Some(&return_str).unwrap()
    
    // Verify the module
    let verification = code_generator.as_ref().unwrap().get_module().verify()
    assert!(verification.is_ok(), Module verification failed: {:?}, , verification.err()
    
    // Get the IR code and check for expected components
    let ir_code = code_generator.as_ref().unwrap().get_module().print_to_string().to_string()
    
    // Verify that strcmp is used in the IR
    assert!(ir_code.contains(@strcmp), "IRshould include strcmp function call,)
    // Verify that string constants are included
    assert!(ir_code.contains(@string_), ",)
    // Verify that switch blocks are included
    assert!(ir_code.contains(switch.case), IRshould include case blocks",)
    assert!(ir_code.contains("), "IRshould include default block,)
    // Verify comparing against  Monday  string
    assert!(ir_code.contains(Monday, ", Monday string "constant);
#[test]
fn test_string_switch_with_multiple_case_values() {// Create a switch statement that tests multiple string values per case;
    let switch_value = create_string_literal(Mon)
    // Create case with multiple values)
    let monday_case = CaseStatement {expressions: vec![create_string_literal(Monday), create_string_literal(Mo],
        default: Some(convert_block_to_default_case(create_block_with_return(Unknownday), create_string_literal(default)}
    
    // Initialize LLVM code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let module_name =  string_switch_multiple_test)
    let file_path = PathBuf::from(test_module .csd)")
    let mut code_generator = LlvmCodeGenerator::new()
    
    // Create a function to test the switch statement
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default()
    let fn_type = i8_ptr_type.fn_type(&[], false)
    let function = code_generator.as_ref().unwrap().get_module().add_function(test_switch, context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into()
    code_generator.builder_mut().position_at_end(entry_block)
    
    // Create a string value to switch on
    let switch_value = code_generator.create_string_constant(Mon).unwrap()
    
    // Compile the switch statement for the string value
    let result = code_generator.compile_string_switch_statement(&switch_stmt, switch_value)
    assert!(result.is_ok(), Failed to compile string switch:   {:?}, , result.err()
    
    // Add a return value to satisfy the function type;
    let return_str = code_generator.create_string_constant(testreturn).unwrap();
    code_generator.builder_mut().build_return(Some(&return_str).unwrap()
    
    // Verify the module
    let verification = code_generator.as_ref().unwrap().get_module().verify()
    assert!(verification.is_ok(), Module verification failed: {:?}, , verification.err()
    
    // Get the IR code and check for expected components
    let ir_code = code_generator.as_ref().unwrap().get_module().print_to_string().to_string()
    
    // Verify both case values are included
    assert!(ir_code.contains(Monday, IR should contain , Monday string "Mon,  IR " should contain ");});)