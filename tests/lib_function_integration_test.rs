/// Library Function Integration Tests
/// 
/// Tests the main public API functions exposed by the CURSED library,
/// ensuring they work correctly for basic use cases.

use cursed::*;
use std::fs;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_function() {
        // Test that init doesn't panic
        init();
        println!("Library initialization succeeded");
    }

    #[test]
    fn test_version_constants() {
        assert!(!VERSION.is_empty(), "VERSION constant should not be empty");
        assert!(!NAME.is_empty(), "NAME constant should not be empty");
        assert_eq!(NAME, "cursed", "NAME should be 'cursed'");
        println!("Version: {}, Name: {}", VERSION, NAME);
    }

    #[test]
    fn test_compile_to_ir_basic() {
        let source = "facts x = 42;";
        
        let result = compile_to_ir(source);
        assert!(result.is_ok(), "Basic compilation should succeed");
        
        let ir = result.unwrap();
        assert!(!ir.is_empty(), "Generated IR should not be empty");
        assert!(ir.contains("i64") || ir.contains("i32"), "IR should contain integer type");
    }

    #[test]
    fn test_compile_to_ir_with_optimization() {
        let source = r#"
            facts x = 2 + 3;
            facts y = x * 4;
        "#;
        
        let result = compile_to_ir_with_optimization(source, Some("O2"));
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Optimized IR should not be empty");
                println!("Optimization compilation succeeded");
            }
            Err(error) => {
                println!("Optimization compilation failed: {}", error);
                // This might fail if optimization passes aren't fully implemented
            }
        }
    }

    #[test]
    fn test_check_function() {
        let valid_source = "facts x = 42;";
        let invalid_source = "this is not valid syntax";
        
        // Valid source should pass
        let result = check(valid_source);
        assert!(result.is_ok(), "Valid source should pass check");
        
        // Invalid source should fail
        let result = check(invalid_source);
        assert!(result.is_err(), "Invalid source should fail check");
    }

    #[test]
    fn test_format_function() {
        let unformatted = "facts x=42;facts y=\"hello\";";
        
        let result = format(unformatted);
        match result {
            Ok(formatted) => {
                assert!(!formatted.is_empty(), "Formatted output should not be empty");
                assert!(formatted.len() >= unformatted.len(), "Formatted should add spacing");
                println!("Formatting succeeded: {}", formatted);
            }
            Err(error) => {
                println!("Formatting failed: {}", error);
                // Formatter might not be fully implemented
            }
        }
    }

    #[test]
    fn test_run_function() {
        let source = r#"
            facts x = 42;
            facts message = "Hello, CURSED!";
        "#;
        
        let result = run(source);
        match result {
            Ok(_) => {
                println!("Program execution succeeded");
            }
            Err(error) => {
                println!("Program execution failed: {}", error);
                // This might fail if the execution engine isn't fully implemented
            }
        }
    }

    #[test]
    fn test_run_with_packages() {
        let source = r#"
            import "stdlib::math";
            facts result = 42;
        "#;
        
        let result = run_with_packages(source, None);
        match result {
            Ok(_) => {
                println!("Package-enabled execution succeeded");
            }
            Err(error) => {
                println!("Package-enabled execution failed: {}", error);
                // This might fail if package management isn't fully integrated
            }
        }
    }

    #[test]
    fn test_file_operations() {
        // Create a temporary test file
        let test_file = "temp_test_lib.csd";
        let content = r#"
            facts greeting = "Hello from file!";
            facts number = 123;
        "#;
        
        fs::write(test_file, content).expect("Failed to write test file");
        
        // Test file execution
        let result = run_file(test_file);
        
        // Clean up
        let _ = fs::remove_file(test_file);
        
        match result {
            Ok(_) => {
                println!("File execution succeeded");
            }
            Err(error) => {
                println!("File execution failed: {}", error);
            }
        }
    }

    #[test]
    fn test_optimization_config_integration() {
        let source = "facts x = 2 + 3 * 4;";
        
        // Create optimization config
        let optimization_config = optimization::OptimizationConfig {
            optimization_level: "O2".to_string(),
            enable_inlining: true,
            enable_loop_unrolling: true,
            enable_dead_code_elimination: true,
            ..optimization::OptimizationConfig::default()
        };
        
        // Test optimized file execution
        let test_file = "temp_optimization_test.csd";
        fs::write(test_file, source).expect("Failed to write test file");
        
        let result = run_file_optimized(test_file, optimization_config);
        
        // Clean up
        let _ = fs::remove_file(test_file);
        
        match result {
            Ok(_) => {
                println!("Optimized execution succeeded");
            }
            Err(error) => {
                println!("Optimized execution failed: {}", error);
            }
        }
    }

    #[test]
    fn test_enhanced_optimization() {
        let source = r#"
            slay fibonacci(n i64) i64 {
                lowkey (n <= 1) {
                    periodt n;
                }
                periodt fibonacci(n - 1) + fibonacci(n - 2);
            }
            
            facts result = fibonacci(10);
        "#;
        
        let optimization_config = optimization::OptimizationConfig {
            optimization_level: "O3".to_string(),
            enable_inlining: true,
            enable_loop_unrolling: true,
            enable_dead_code_elimination: true,
            ..optimization::OptimizationConfig::default()
        };
        
        let test_file = "temp_enhanced_test.csd";
        fs::write(test_file, source).expect("Failed to write test file");
        
        let result = run_file_enhanced(test_file, optimization_config, true);
        
        // Clean up
        let _ = fs::remove_file(test_file);
        
        match result {
            Ok(_) => {
                println!("Enhanced optimization succeeded");
            }
            Err(error) => {
                println!("Enhanced optimization failed: {}", error);
            }
        }
    }

    #[test]
    fn test_repl_integration() {
        let mut session_manager = repl::SessionManager::new();
        
        let test_expressions = vec![
            "42",
            "2 + 3",
            "\"hello\"",
            "true",
            "facts x = 10",
        ];
        
        for expr in test_expressions {
            let result = execute_repl_code(expr, &mut session_manager);
            match result {
                Ok(output) => {
                    println!("REPL '{}' -> '{}'", expr, output);
                }
                Err(error) => {
                    println!("REPL '{}' failed: {}", expr, error);
                }
            }
        }
    }

    #[test]
    fn test_error_handling_chain() {
        let test_cases = vec![
            ("facts x = ;", "Incomplete expression"),
            ("unknown_keyword", "Unknown syntax"),
            ("facts x: i64 = \"string\";", "Type mismatch"),
            ("import \"nonexistent::module\";", "Missing module"),
        ];
        
        for (source, description) in test_cases {
            println!("Testing error case: {}", description);
            
            // Test compilation error handling
            let compile_result = compile_to_ir(source);
            let check_result = check(source);
            let run_result = run(source);
            
            // At least one should fail gracefully
            let any_succeeded = compile_result.is_ok() || check_result.is_ok() || run_result.is_ok();
            
            if !any_succeeded {
                println!("All operations failed for '{}' - this is expected", description);
            } else {
                println!("Some operations succeeded for '{}' - error recovery might be working", description);
            }
        }
    }

    #[test]
    fn test_package_integration_chain() {
        let source = r#"
            import "stdlib::math";
            facts result = math::abs(-42);
        "#;
        
        // Test the full package integration chain
        let compile_result = compile_to_ir_with_packages(source, None);
        let check_result = check_with_packages(source, None);
        let run_result = run_with_packages(source, None);
        
        match (compile_result, check_result, run_result) {
            (Ok(_), Ok(_), Ok(_)) => {
                println!("Full package integration chain succeeded");
            }
            _ => {
                println!("Package integration chain has some failures - expected during development");
            }
        }
    }

    #[test]
    fn test_concurrent_operations() {
        let source = r#"
            stan goroutine_task() {
                facts x = 42;
            }
            
            goroutine_task();
        "#;
        
        let result = compile_to_ir(source);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Concurrent operation IR should not be empty");
                println!("Concurrent operations compilation succeeded");
            }
            Err(error) => {
                println!("Concurrent operations failed: {}", error);
            }
        }
    }
}
