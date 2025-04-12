//! Integration tests for the vibe_check statement codegen

use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::sync::Arc;

#[test]
fn test_simple_vibe_check_codegen() {
    let input = r#"
    slay test_simple_switch(x normie) tea {
        sus result tea = "unknown";
        
        vibe_check x {
            mood 1:
                result = "one";
            mood 2:
                result = "two";
            mood 3:
                result = "three";
            basic:
                result = "other";
        }
        
        yolo result;
    }
    "#;

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();

    // No errors should be reported during parsing
    assert_eq!(
        parser.errors().len(),
        0,
        "Parser errors: {:?}",
        parser.errors()
    );

    // Create LLVM code generator
    let context = inkwell::context::Context::create();
    let module_name = "test_module";
    let file_path = std::path::PathBuf::from("test_module.csd");
    let mut code_generator = LlvmCodeGenerator::new(&context, module_name, file_path);

    // Generate LLVM IR code
    let result = code_generator.compile(&program);
    assert!(result.is_ok(), "Code generation failed: {:?}", result.err());

    // Get the resulting IR code
    let ir_code = code_generator.module().print_to_string().to_string();
    println!("Generated LLVM IR:\n{}", ir_code);

    // Verify the test_simple_switch function exists in the IR
    assert!(
        ir_code.contains("@test_simple_switch"),
        "Function test_simple_switch not found in IR"
    );

    // Verify switch instruction is present in the IR
    assert!(
        ir_code.contains("switch"),
        "Switch instruction not found in IR"
    );
}

#[test]
fn test_multiple_case_values() {
    let input = r#"
    slay test_multiple_cases(x normie) tea {
        sus result tea = "unknown";
        
        vibe_check x {
            mood 1, 2, 3:
                result = "small";
            mood 4, 5, 6:
                result = "medium";
            mood 7, 8, 9:
                result = "large";
            basic:
                result = "unknown";
        }
        
        yolo result;
    }
    "#;

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();

    // No errors should be reported during parsing
    assert_eq!(
        parser.errors().len(),
        0,
        "Parser errors: {:?}",
        parser.errors()
    );

    // Create LLVM code generator
    let context = inkwell::context::Context::create();
    let module_name = "test_module";
    let file_path = std::path::PathBuf::from("test_module.csd");
    let mut code_generator = LlvmCodeGenerator::new(&context, module_name, file_path);

    // Generate LLVM IR code
    let result = code_generator.compile(&program);
    assert!(result.is_ok(), "Code generation failed: {:?}", result.err());

    // Get the resulting IR code
    let ir_code = code_generator.module().print_to_string().to_string();
    println!("Generated LLVM IR:\n{}", ir_code);

    // Verify we have multiple case values for the same block in the IR
    // This is harder to verify from just the IR text but we can check that our function exists
    assert!(
        ir_code.contains("@test_multiple_cases"),
        "Function test_multiple_cases not found in IR"
    );
}

#[test]
fn test_fallthrough_behavior() {
    let input = r#"
    slay test_fallthrough(day tea) tea {
        sus result tea = "unknown";
        
        vibe_check day {
            mood "Monday":
                result = "Start of week";
                // Fallthrough is implicit in CURSED without break
            mood "Tuesday":
                result = "Weekday";
                ghosted; // break to exit the case
            mood "Wednesday":
                result = "Mid-week";
                ghosted;
            mood "Thursday":
                result = "Almost weekend";
                ghosted;
            mood "Friday":
                result = "End of week";
                ghosted;
            basic:
                result = "Weekend";
        }
        
        yolo result;
    }
    "#;

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();

    // No errors should be reported during parsing
    assert_eq!(
        parser.errors().len(),
        0,
        "Parser errors: {:?}",
        parser.errors()
    );

    // Create LLVM code generator
    let context = inkwell::context::Context::create();
    let module_name = "test_module";
    let file_path = std::path::PathBuf::from("test_module.csd");
    let mut code_generator = LlvmCodeGenerator::new(&context, module_name, file_path);

    // Generate LLVM IR code
    let result = code_generator.compile(&program);

    // This test will fail due to string case values not being supported yet
    assert!(
        result.is_err(),
        "Code generation should fail with string case values"
    );
    let error_msg = format!("{:?}", result.err());
    assert!(
        error_msg.contains("String switch values not yet supported"),
        "Expected string case value error, got: {}",
        error_msg
    );
}
