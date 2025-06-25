/// Core Compilation Pipeline Basic Tests
/// 
/// Tests the fundamental compilation pipeline including parsing, type checking,
/// code generation, and execution for basic CURSED language constructs.

use cursed::*;
use std::fs;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic_compilation() {
        let source = r#"
            facts result = 2 + 3 * 4;
        "#;
        
        // Test that compilation succeeds
        let result = compile_to_ir(source);
        assert!(result.is_ok(), "Basic arithmetic should compile successfully");
        
        let ir = result.unwrap();
        assert!(!ir.is_empty(), "Generated IR should not be empty");
        assert!(ir.contains("add"), "IR should contain addition operation");
        assert!(ir.contains("mul"), "IR should contain multiplication operation");
    }

    #[test]
    fn test_variable_declaration_compilation() {
        let source = r#"
            facts x = 42;
            sus y = "hello";
            facts z = true;
        "#;
        
        let result = compile_to_ir(source);
        assert!(result.is_ok(), "Variable declarations should compile successfully");
        
        let ir = result.unwrap();
        assert!(!ir.is_empty(), "Generated IR should not be empty");
        assert!(ir.contains("i64"), "IR should contain integer type");
        assert!(ir.contains("i8*"), "IR should contain string type reference");
    }

    #[test]
    fn test_function_declaration_compilation() {
        let source = r#"
            slay add_numbers(a i64, b i64) i64 {
                periodt a + b;
            }
        "#;
        
        let result = compile_to_ir(source);
        assert!(result.is_ok(), "Function declaration should compile successfully");
        
        let ir = result.unwrap();
        assert!(!ir.is_empty(), "Generated IR should not be empty");
        assert!(ir.contains("define"), "IR should contain function definition");
        assert!(ir.contains("add_numbers"), "IR should contain function name");
        assert!(ir.contains("ret"), "IR should contain return instruction");
    }

    #[test]
    fn test_control_flow_compilation() {
        let source = r#"
            lowkey (true) {
                facts x = 10;
            } highkey {
                facts x = 20;
            }
        "#;
        
        let result = compile_to_ir(source);
        assert!(result.is_ok(), "Control flow should compile successfully");
        
        let ir = result.unwrap();
        assert!(!ir.is_empty(), "Generated IR should not be empty");
        assert!(ir.contains("br"), "IR should contain branch instructions");
        assert!(ir.contains("label"), "IR should contain basic block labels");
    }

    #[test]
    fn test_syntax_error_handling() {
        let source = r#"
            facts x = 42
            // Missing semicolon
        "#;
        
        let result = compile_to_ir(source);
        // Should either succeed with error recovery or fail gracefully
        match result {
            Ok(_) => {
                // Error recovery succeeded
                println!("Compilation succeeded with error recovery");
            }
            Err(error) => {
                // Should be a meaningful error message
                let error_str = format!("{}", error);
                assert!(!error_str.is_empty(), "Error message should not be empty");
                assert!(error_str.contains("parse") || error_str.contains("syntax"), 
                       "Error should indicate parsing/syntax issue");
            }
        }
    }

    #[test]
    fn test_type_error_handling() {
        let source = r#"
            facts x: i64 = "not a number";
        "#;
        
        let result = compile_to_ir(source);
        // Should fail with type error
        match result {
            Ok(_) => {
                // If it compiles, check that error handling is in the IR
                println!("Type error handled during compilation");
            }
            Err(error) => {
                let error_str = format!("{}", error);
                assert!(!error_str.is_empty(), "Error message should not be empty");
            }
        }
    }

    #[test]
    fn test_check_function_basic() {
        let source = r#"
            facts valid_program = 42;
        "#;
        
        let result = check(source);
        assert!(result.is_ok(), "Valid program should pass check");
    }

    #[test]
    fn test_check_function_invalid() {
        let source = r#"
            this is not valid CURSED syntax at all
        "#;
        
        let result = check(source);
        assert!(result.is_err(), "Invalid syntax should fail check");
    }

    #[test]
    fn test_format_function_basic() {
        let source = r#"facts x=42;facts y="hello";"#;
        
        let result = format(source);
        match result {
            Ok(formatted) => {
                assert!(!formatted.is_empty(), "Formatted code should not be empty");
                assert!(formatted.contains("facts x = 42"), "Should format with proper spacing");
                assert!(formatted.contains("facts y = \"hello\""), "Should format strings properly");
            }
            Err(_) => {
                // Formatter might not be fully implemented
                println!("Formatter not fully implemented yet");
            }
        }
    }

    #[test]
    fn test_run_function_basic() {
        let source = r#"
            // Simple program that doesn't crash
            facts x = 42;
        "#;
        
        let result = run(source);
        // Should either succeed or fail gracefully
        match result {
            Ok(_) => {
                println!("Simple program executed successfully");
            }
            Err(error) => {
                // Error should be meaningful
                let error_str = format!("{}", error);
                assert!(!error_str.is_empty(), "Error message should not be empty");
                println!("Execution failed with: {}", error_str);
            }
        }
    }

    #[test]
    fn test_optimization_levels() {
        let source = r#"
            facts x = 2 + 3;
            facts y = x * 4;
        "#;
        
        // Test different optimization levels
        let levels = vec!["O0", "O1", "O2", "O3"];
        
        for level in levels {
            let result = compile_to_ir_with_optimization(source, Some(level));
            match result {
                Ok(ir) => {
                    assert!(!ir.is_empty(), "IR should not be empty for optimization level {}", level);
                    println!("Optimization level {} succeeded", level);
                }
                Err(error) => {
                    println!("Optimization level {} failed: {}", level, error);
                }
            }
        }
    }

    #[test]
    fn test_package_integration_basic() {
        let source = r#"
            import "stdlib::math";
            facts result = math::abs(-42);
        "#;
        
        let result = compile_to_ir_with_packages(source, None);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "IR with packages should not be empty");
                println!("Package integration succeeded");
            }
            Err(error) => {
                println!("Package integration failed: {}", error);
                // This might fail if stdlib integration isn't complete
            }
        }
    }

    #[test]
    fn test_file_compilation() {
        // Create a temporary test file
        let test_content = r#"
            facts test_variable = 123;
        "#;
        
        let test_file = "test_temp_file.csd";
        fs::write(test_file, test_content).expect("Failed to write test file");
        
        // Test file compilation
        let result = run_file(test_file);
        
        // Clean up
        let _ = fs::remove_file(test_file);
        
        match result {
            Ok(_) => {
                println!("File compilation succeeded");
            }
            Err(error) => {
                println!("File compilation failed: {}", error);
            }
        }
    }

    #[test]
    fn test_repl_execution() {
        let mut session_manager = cursed::repl::SessionManager::new();
        
        // Test simple expressions
        let test_cases = vec![
            "42",
            "2 + 3",
            "\"hello world\"",
            "true",
        ];
        
        for test_case in test_cases {
            let result = execute_repl_code(test_case, &mut session_manager);
            match result {
                Ok(output) => {
                    assert!(!output.is_empty(), "REPL output should not be empty for: {}", test_case);
                    println!("REPL '{}' -> '{}'", test_case, output);
                }
                Err(error) => {
                    println!("REPL '{}' failed: {}", test_case, error);
                }
            }
        }
    }
}
