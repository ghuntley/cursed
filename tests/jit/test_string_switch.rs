use cursed::ast:::: CaseStatement, SwitchStatement;
use cursed::ast::StringLiteral;
use cursed::ast::block::BlockStatement;
use cursed::ast::ReturnStatement;
use cursed::ast::{Expression, Node, Statement;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::lexer::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

//! Standalone test for string switch functionality


// Helper function to create a string literal expression
fn create_string_literal() {Box::new(StringLiteral {value: value.to_string()})}

// Helper function to create a return statement
fn create_return_statement() {Box::new(ReturnStatement {return_value: value})}

// Helper function to create a block with a return statement
fn create_block_with_return() {BlockStatement {token: Token::new(TokenType::LeftBrace, {statements: vec![create_return_statement(Some(create_string_literal(value],
        default: Some(default_case)};};
    
    // Initialize LLVM code generator
    let context = Context::create();
    let context = Box::leak(Box::new(context);
    let module_name =  string_switch_test;
    let file_path = PathBuf::from(", context.i32_type().into(), None);
    let entry_block = context.i32_type().const_int(0, false).into();
    
    // Position at the entry block
    unsafe {let builder = &code_generator.builder;
        builder.position_at_end(entry_block);}
    
    // Create a string value to switch on
    let switch_value = code_generator.create_string_constant(Monday .unwrap();
    
    // Compile the switch statement for the string value
    let result = code_generator.compile_string_switch_statement(&switch_stmt, switch_value);
    if result.is_ok()    {println!(✅ Compiled string switch successfully);} else {}
        println!("❌ Failed to compile string switch: {:?}, result.err();
        return;}
    
    // Add a return value to satisfy the function type
    let return_str = code_generator.create_string_constant(test return).unwrap();
    unsafe {let builder = &code_generator.builder;
        builder.build_return(Some(&return_str).unwrap();}
    
    // Verify the module
    let verification = code_generator.name();
    if verification.is_ok()   {println!(✅ Module verification passed);} else {}
        println!("{}, ir_code);
    println!("-------------------------"@string_);
    let has_cases = ir_code.contains("switch."default);
    
    println!("\n--- Verification Results ---;
    println!("Contains case blocks: {}, has_cases);
    println!("Contains default block: {}, has_default);
    println!(-----------------------------"\n✅ String switch implementation works correctly);} else {println!("\n❌ String switch implementation is missing components";}