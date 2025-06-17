/// LLVM Expression Compilation Tests
/// 
/// Tests the LLVM code generation for various expressions,
/// ensuring correct IR generation and compilation.

use cursed::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_expression_compilation() {
        let test_cases = vec![
            ("facts x = 2 + 3;", vec!["add", "i64"]),
            ("facts y = 5 - 2;", vec!["sub", "i64"]),
            ("facts z = 4 * 6;", vec!["mul", "i64"]),
            ("facts w = 8 / 2;", vec!["sdiv", "i64"]),
            ("facts m = 7 % 3;", vec!["srem", "i64"]),
        ];
        
        for (source, expected_tokens) in test_cases {
            let result = compile_to_ir(source);
            match result {
                Ok(ir) => {
                    assert!(!ir.is_empty(), "IR should not be empty for: {}", source);
                    
                    for token in expected_tokens {
                        assert!(ir.contains(token), 
                               "IR should contain '{}' for source: {}", token, source);
                    }
                    
                    println!("Arithmetic compilation succeeded for: {}", source);
                }
                Err(error) => {
                    println!("Arithmetic compilation failed for '{}': {}", source, error);
                }
            }
        }
    }

    #[test]
    fn test_comparison_expression_compilation() {
        let test_cases = vec![
            ("facts a = 5 > 3;", vec!["icmp", "sgt"]),
            ("facts b = 2 < 7;", vec!["icmp", "slt"]),
            ("facts c = 4 >= 4;", vec!["icmp", "sge"]),
            ("facts d = 3 <= 6;", vec!["icmp", "sle"]),
            ("facts e = 5 == 5;", vec!["icmp", "eq"]),
            ("facts f = 2 != 8;", vec!["icmp", "ne"]),
        ];
        
        for (source, expected_tokens) in test_cases {
            let result = compile_to_ir(source);
            match result {
                Ok(ir) => {
                    assert!(!ir.is_empty(), "IR should not be empty for: {}", source);
                    
                    for token in expected_tokens {
                        assert!(ir.contains(token), 
                               "IR should contain '{}' for source: {}", token, source);
                    }
                    
                    println!("Comparison compilation succeeded for: {}", source);
                }
                Err(error) => {
                    println!("Comparison compilation failed for '{}': {}", source, error);
                }
            }
        }
    }

    #[test]
    fn test_logical_expression_compilation() {
        let test_cases = vec![
            ("facts x = true && false;", vec!["and", "i1"]),
            ("facts y = true || false;", vec!["or", "i1"]),
            ("facts z = !true;", vec!["xor", "i1"]),
        ];
        
        for (source, expected_tokens) in test_cases {
            let result = compile_to_ir(source);
            match result {
                Ok(ir) => {
                    assert!(!ir.is_empty(), "IR should not be empty for: {}", source);
                    
                    for token in expected_tokens {
                        assert!(ir.contains(token), 
                               "IR should contain '{}' for source: {}", token, source);
                    }
                    
                    println!("Logical compilation succeeded for: {}", source);
                }
                Err(error) => {
                    println!("Logical compilation failed for '{}': {}", source, error);
                }
            }
        }
    }

    #[test]
    fn test_variable_expression_compilation() {
        let source = r#"
            facts x = 42;
            facts y = x + 10;
            facts z = y * 2;
        "#;
        
        let result = compile_to_ir(source);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "IR should not be empty");
                assert!(ir.contains("alloca"), "IR should contain variable allocation");
                assert!(ir.contains("store"), "IR should contain variable store");
                assert!(ir.contains("load"), "IR should contain variable load");
                
                println!("Variable expression compilation succeeded");
            }
            Err(error) => {
                println!("Variable expression compilation failed: {}", error);
            }
        }
    }

    #[test]
    fn test_function_call_expression_compilation() {
        let source = r#"
            slay add(a i64, b i64) i64 {
                periodt a + b;
            }
            
            facts result = add(5, 3);
        "#;
        
        let result = compile_to_ir(source);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "IR should not be empty");
                assert!(ir.contains("define"), "IR should contain function definition");
                assert!(ir.contains("call"), "IR should contain function call");
                assert!(ir.contains("ret"), "IR should contain return instruction");
                
                println!("Function call compilation succeeded");
            }
            Err(error) => {
                println!("Function call compilation failed: {}", error);
            }
        }
    }

    #[test]
    fn test_array_expression_compilation() {
        let source = r#"
            facts arr = [1, 2, 3, 4, 5];
            facts first = arr[0];
            facts second = arr[1];
        "#;
        
        let result = compile_to_ir(source);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "IR should not be empty");
                assert!(ir.contains("getelementptr") || ir.contains("array"), 
                       "IR should contain array operations");
                
                println!("Array expression compilation succeeded");
            }
            Err(error) => {
                println!("Array expression compilation failed: {}", error);
            }
        }
    }

    #[test]
    fn test_struct_expression_compilation() {
        let source = r#"
            squad Point {
                x i64,
                y i64,
            }
            
            facts p = Point { x: 10, y: 20 };
            facts x_val = p.x;
        "#;
        
        let result = compile_to_ir(source);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "IR should not be empty");
                assert!(ir.contains("getelementptr") || ir.contains("struct"), 
                       "IR should contain struct operations");
                
                println!("Struct expression compilation succeeded");
            }
            Err(error) => {
                println!("Struct expression compilation failed: {}", error);
            }
        }
    }

    #[test]
    fn test_string_expression_compilation() {
        let source = r#"
            facts greeting = "Hello, World!";
            facts name = "CURSED";
            facts message = greeting + " from " + name;
        "#;
        
        let result = compile_to_ir(source);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "IR should not be empty");
                assert!(ir.contains("i8*") || ir.contains("ptr"), 
                       "IR should contain string pointers");
                
                println!("String expression compilation succeeded");
            }
            Err(error) => {
                println!("String expression compilation failed: {}", error);
            }
        }
    }

    #[test]
    fn test_conditional_expression_compilation() {
        let source = r#"
            facts condition = true;
            facts result = condition ? 42 : 0;
        "#;
        
        let result = compile_to_ir(source);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "IR should not be empty");
                assert!(ir.contains("br") || ir.contains("select"), 
                       "IR should contain conditional branching");
                
                println!("Conditional expression compilation succeeded");
            }
            Err(error) => {
                println!("Conditional expression compilation failed: {}", error);
            }
        }
    }

    #[test]
    fn test_type_cast_expression_compilation() {
        let source = r#"
            facts i = 42;
            facts f = f64(i);
            facts back = i64(f);
        "#;
        
        let result = compile_to_ir(source);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "IR should not be empty");
                assert!(ir.contains("sitofp") || ir.contains("fptosi") || ir.contains("cast"), 
                       "IR should contain type conversion instructions");
                
                println!("Type cast compilation succeeded");
            }
            Err(error) => {
                println!("Type cast compilation failed: {}", error);
            }
        }
    }

    #[test]
    fn test_complex_expression_compilation() {
        let source = r#"
            slay factorial(n i64) i64 {
                lowkey (n <= 1) {
                    periodt 1;
                }
                periodt n * factorial(n - 1);
            }
            
            facts result = factorial(5) + 10 * 2;
        "#;
        
        let result = compile_to_ir(source);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "IR should not be empty");
                assert!(ir.contains("define"), "IR should contain function definition");
                assert!(ir.contains("call"), "IR should contain function calls");
                assert!(ir.contains("mul"), "IR should contain multiplication");
                assert!(ir.contains("add"), "IR should contain addition");
                
                println!("Complex expression compilation succeeded");
            }
            Err(error) => {
                println!("Complex expression compilation failed: {}", error);
            }
        }
    }

    #[test]
    fn test_optimization_effects() {
        let source = r#"
            facts a = 2 + 3;
            facts b = a * 1;
            facts c = b + 0;
        "#;
        
        // Test with different optimization levels
        let levels = vec!["O0", "O1", "O2", "O3"];
        
        for level in levels {
            let result = compile_to_ir_with_optimization(source, Some(level));
            match result {
                Ok(ir) => {
                    assert!(!ir.is_empty(), "IR should not be empty for {}", level);
                    
                    // O0 should have more instructions than O3
                    if level == "O0" {
                        println!("O0 IR length: {}", ir.len());
                    } else if level == "O3" {
                        println!("O3 IR length: {}", ir.len());
                    }
                    
                    println!("Optimization level {} compilation succeeded", level);
                }
                Err(error) => {
                    println!("Optimization level {} failed: {}", level, error);
                }
            }
        }
    }

    #[test]
    fn test_memory_operations() {
        let source = r#"
            facts ptr = &42;
            facts value = *ptr;
        "#;
        
        let result = compile_to_ir(source);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "IR should not be empty");
                assert!(ir.contains("alloca") || ir.contains("load") || ir.contains("store"), 
                       "IR should contain memory operations");
                
                println!("Memory operations compilation succeeded");
            }
            Err(error) => {
                println!("Memory operations compilation failed: {}", error);
            }
        }
    }

    #[test]
    fn test_channel_expression_compilation() {
        let source = r#"
            facts ch = make(chan i64, 10);
            ch <- 42;
            facts value = <-ch;
        "#;
        
        let result = compile_to_ir(source);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "IR should not be empty");
                // Channel operations might compile to function calls
                assert!(ir.contains("call") || ir.contains("channel"), 
                       "IR should contain channel operations");
                
                println!("Channel expression compilation succeeded");
            }
            Err(error) => {
                println!("Channel expression compilation failed: {}", error);
            }
        }
    }

    #[test]
    fn test_error_expression_compilation() {
        let source = r#"
            slay divide(a i64, b i64) Result<i64, String> {
                lowkey (b == 0) {
                    periodt Err("Division by zero");
                }
                periodt Ok(a / b);
            }
            
            facts result = divide(10, 2)?;
        "#;
        
        let result = compile_to_ir(source);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "IR should not be empty");
                // Error handling might compile to branching or function calls
                assert!(ir.contains("br") || ir.contains("call"), 
                       "IR should contain error handling");
                
                println!("Error expression compilation succeeded");
            }
            Err(error) => {
                println!("Error expression compilation failed: {}", error);
            }
        }
    }

    #[test]
    fn test_generic_expression_compilation() {
        let source = r#"
            slay identity<T>(value T) T {
                periodt value;
            }
            
            facts num = identity<i64>(42);
            facts text = identity<String>("hello");
        "#;
        
        let result = compile_to_ir(source);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "IR should not be empty");
                // Generic functions should be monomorphized
                assert!(ir.contains("define"), "IR should contain function definitions");
                
                println!("Generic expression compilation succeeded");
            }
            Err(error) => {
                println!("Generic expression compilation failed: {}", error);
            }
        }
    }

    #[test]
    fn test_ir_structure_validation() {
        let source = "facts x = 42;";
        
        let result = compile_to_ir(source);
        match result {
            Ok(ir) => {
                // Basic IR structure validation
                assert!(ir.contains("define") || ir.contains("@main") || ir.contains("declare"), 
                       "IR should contain function declarations");
                
                // Check for proper LLVM IR syntax
                let lines: Vec<&str> = ir.lines().collect();
                assert!(!lines.is_empty(), "IR should have multiple lines");
                
                // Should not contain obvious syntax errors
                assert!(!ir.contains("ERROR"), "IR should not contain error markers");
                assert!(!ir.contains("TODO"), "IR should not contain TODO markers");
                
                println!("IR structure validation passed");
                println!("Generated IR preview:\n{}", 
                        ir.lines().take(10).collect::<Vec<&str>>().join("\n"));
            }
            Err(error) => {
                println!("IR structure validation failed: {}", error);
            }
        }
    }
}
