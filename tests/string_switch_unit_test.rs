//! Unit tests for the string-based switch implementation
//! 
//! These tests directly test the string switch functionality in the LLVM code generator
//! without requiring the parser to work properly.

use cursed::ast::control_flow::{CaseStatement, SwitchStatement, SwitchCase};
use cursed::ast::expressions::StringLiteral;
use cursed::ast::statements::block::BlockStatement;
use cursed::ast::statements::declarations::ReturnStatement;
use cursed::ast::{Expression, Node, Statement};
use cursed::codegen::llvm::LlvmCodeGenerator;
use inkwell::context::Context;
use std::any::Any;
use std::path::PathBuf;

mod switch_test_helper;
use switch_test_helper::{convert_to_switch_case, convert_block_to_default_case};

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

#[test]
fn test_string_switch_statement() {
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
        cases: vec![convert_to_switch_case(monday_case), convert_to_switch_case(friday_case)],
        default: Some(convert_block_to_default_case(default_case, create_string_literal("default"))),
    };
    
    // Initialize LLVM code generator
    let context = Context::create();
    let module_name = "string_switch_test";
    let file_path = PathBuf::from("test_module.csd");
    let mut code_generator = LlvmCodeGenerator::new(&context, module_name, file_path);
    
    // Create a function to test the switch statement
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let fn_type = i8_ptr_type.fn_type(&[], false);
    let function = code_generator.module().add_function("test_switch", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    code_generator.builder_mut().position_at_end(entry_block);
    
    // Create a string value to switch on
    let switch_value = code_generator.create_string_constant("Monday").unwrap();
    
    // Compile the switch statement for the string value
    let result = code_generator.compile_string_switch_statement(&switch_stmt, switch_value);
    assert!(result.is_ok(), "Failed to compile string switch: {:?}", result.err());
    
    // Add a return value to satisfy the function type
    let return_str = code_generator.create_string_constant("test return").unwrap();
    code_generator.builder_mut().build_return(Some(&return_str)).unwrap();
    
    // Verify the module
    let verification = code_generator.module().verify();
    assert!(verification.is_ok(), "Module verification failed: {:?}", verification.err());
    
    // Get the IR code and check for expected components
    let ir_code = code_generator.module().print_to_string().to_string();
    
    // Verify that strcmp is used in the IR
    assert!(ir_code.contains("@strcmp"), "IR should include strcmp function call");
    
    // Verify that string constants are included
    assert!(ir_code.contains("@string_"), "IR should include string constants");
    
    // Verify that switch blocks are included
    assert!(ir_code.contains("switch.case"), "IR should include case blocks");
    assert!(ir_code.contains("switch.default"), "IR should include default block");
    
    // Verify comparing against "Monday" string
    assert!(ir_code.contains("Monday"), "IR should contain 'Monday' string constant");
}

#[test]
fn test_string_switch_with_multiple_case_values() {
    // Create a switch statement that tests multiple string values per case
    let switch_value = create_string_literal("Mon");
    
    // Create case with multiple values
    let monday_case = CaseStatement {
        token: "mood".to_string(),
        expressions: vec![create_string_literal("Monday"), create_string_literal("Mon")],
        body: create_block_with_return("Start of week"),
    };
    
    // Create the switch statement
    let switch_stmt = SwitchStatement {
        token: "vibe_check".to_string(),
        value: switch_value,
        cases: vec![convert_to_switch_case(monday_case)],
        default: Some(convert_block_to_default_case(create_block_with_return("Unknown day"), create_string_literal("default"))),
    };
    
    // Initialize LLVM code generator
    let context = Context::create();
    let module_name = "string_switch_multiple_test";
    let file_path = PathBuf::from("test_module.csd");
    let mut code_generator = LlvmCodeGenerator::new(&context, module_name, file_path);
    
    // Create a function to test the switch statement
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let fn_type = i8_ptr_type.fn_type(&[], false);
    let function = code_generator.module().add_function("test_switch", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    code_generator.builder_mut().position_at_end(entry_block);
    
    // Create a string value to switch on
    let switch_value = code_generator.create_string_constant("Mon").unwrap();
    
    // Compile the switch statement for the string value
    let result = code_generator.compile_string_switch_statement(&switch_stmt, switch_value);
    assert!(result.is_ok(), "Failed to compile string switch: {:?}", result.err());
    
    // Add a return value to satisfy the function type
    let return_str = code_generator.create_string_constant("test return").unwrap();
    code_generator.builder_mut().build_return(Some(&return_str)).unwrap();
    
    // Verify the module
    let verification = code_generator.module().verify();
    assert!(verification.is_ok(), "Module verification failed: {:?}", verification.err());
    
    // Get the IR code and check for expected components
    let ir_code = code_generator.module().print_to_string().to_string();
    
    // Verify both case values are included
    assert!(ir_code.contains("Monday"), "IR should contain 'Monday' string constant");
    assert!(ir_code.contains("Mon"), "IR should contain 'Mon' string constant");
}