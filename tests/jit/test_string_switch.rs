//! Standalone test for string switch functionality

use cursed::ast::control_flow::{CaseStatement, SwitchStatement};
use cursed::ast::expressions::StringLiteral;
use cursed::ast::statements::block::BlockStatement;
use cursed::ast::statements::declarations::ReturnStatement;
use cursed::ast::{Expression, Node, Statement};
use cursed::codegen::llvm::LlvmCodeGenerator;
use inkwell::context::Context;
use std::path::PathBuf;

// Helper function to create a string literal expression
fn create_string_literal(value: &str) -> Box<dyn Expression> {
    Box::new(StringLiteral {
        token: "\"string\"".to_string(),
        value: value.to_string(),
    })
}

// Helper function to create a return statement
fn create_return_statement(value: Option<Box<dyn Expression>>) -> Box<dyn Statement> {
    Box::new(ReturnStatement {
        token: "yolo".to_string(),
        return_value: value,
    })
}

// Helper function to create a block with a return statement
fn create_block_with_return(value: &str) -> BlockStatement {
    BlockStatement {
        token: "{".to_string(),
        statements: vec![create_return_statement(Some(create_string_literal(value)))],
    }
}

fn main() {
    println!("Testing string switch functionality...");
    
    // Create a switch statement that tests different string values
    let switch_value = create_string_literal("Monday");
    
    // Create case statements
    let monday_case = CaseStatement {
        token: "mood".to_string(),
        expressions: vec![create_string_literal("Monday")],
        body: create_block_with_return("Start of week"),
    };
    
    let friday_case = CaseStatement {
        token: "mood".to_string(),
        expressions: vec![create_string_literal("Friday")],
        body: create_block_with_return("End of week"),
    };
    
    // Create default case
    let default_case = create_block_with_return("Mid-week");
    
    // Create the switch statement
    let switch_stmt = SwitchStatement {
        token: "vibe_check".to_string(),
        value: switch_value,
        cases: vec![monday_case, friday_case],
        default: Some(default_case),
    };
    
    // Initialize LLVM code generator
    let context = Context::create();
    let module_name = "string_switch_test";
    let file_path = PathBuf::from("test_module.csd");
    let mut code_generator = LlvmCodeGenerator::new(&context, module_name, file_path);
    
    // Create a function to test the switch statement
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let fn_type = i8_ptr_type.fn_type(&[], false);
    let function = code_generator.module.add_function("test_switch", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    
    // Position at the entry block
    unsafe {
        let builder = &code_generator.builder;
        builder.position_at_end(entry_block);
    }
    
    // Create a string value to switch on
    let switch_value = code_generator.create_string_constant("Monday").unwrap();
    
    // Compile the switch statement for the string value
    let result = code_generator.compile_string_switch_statement(&switch_stmt, switch_value);
    if result.is_ok() {
        println!("✅ Compiled string switch successfully");
    } else {
        println!("❌ Failed to compile string switch: {:?}", result.err());
        return;
    }
    
    // Add a return value to satisfy the function type
    let return_str = code_generator.create_string_constant("test return").unwrap();
    unsafe {
        let builder = &code_generator.builder;
        builder.build_return(Some(&return_str)).unwrap();
    }
    
    // Verify the module
    let verification = code_generator.module.verify();
    if verification.is_ok() {
        println!("✅ Module verification passed");
    } else {
        println!("❌ Module verification failed: {:?}", verification.err());
        return;
    }
    
    // Get the IR code and print it
    let ir_code = code_generator.module.print_to_string().to_string();
    println!("\n--- Generated LLVM IR ---");
    println!("{}", ir_code);
    println!("-------------------------");
    
    // Check for key components in the IR
    let has_strcmp = ir_code.contains("@strcmp");
    let has_string_constants = ir_code.contains("@string_");
    let has_cases = ir_code.contains("switch.case");
    let has_default = ir_code.contains("switch.default");
    
    println!("\n--- Verification Results ---");
    println!("Contains strcmp function: {}", has_strcmp);
    println!("Contains string constants: {}", has_string_constants);
    println!("Contains case blocks: {}", has_cases);
    println!("Contains default block: {}", has_default);
    println!("-----------------------------");
    
    if has_strcmp && has_string_constants && has_cases && has_default {
        println!("\n✅ String switch implementation works correctly");
    } else {
        println!("\n❌ String switch implementation is missing components");
    }
}