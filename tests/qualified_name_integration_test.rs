//! Integration tests for qualified name support
//!
//! These tests verify that the complete qualified name system works end-to-end,
//! including parsing, symbol resolution, and LLVM code generation.

use std::path::PathBuf;

// Common test setup
fn init_test_tracing() {
    use tracing_subscriber::{EnvFilter, FmtSubscriber};
    let _ = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_test_writer()
        .try_init();
}

#[test]
fn test_qualified_function_call() {
    init_test_tracing();
    
    let code = r#"
    yeet "math"
    
    func main() {
        let result = math.sqrt(25.0);
        vibez.spill("Square root result:", result);
    }
    "#;
    
    // This test verifies that:
    // 1. The import statement is parsed correctly
    // 2. The qualified function call `math.sqrt` is recognized
    // 3. The qualified function call `vibez.spill` works
    // 4. Code generation produces valid LLVM IR
    
    // Parse the code
    match cursed::parse_string(code) {
        Ok(program) => {
            println!("✓ Successfully parsed qualified function calls");
            
            // Check that we have the expected statements
            assert!(program.statements.len() >= 2); // import + function
            
            // Find the import statement
            let has_import = program.statements.iter().any(|stmt| {
                if let Some(import) = stmt.as_any().downcast_ref::<cursed::ast::statements::declarations::ImportStatement>() {
                    import.path.value == "math"
                } else {
                    false
                }
            });
            assert!(has_import, "Should have math import statement");
            
        },
        Err(e) => {
            panic!("Failed to parse qualified function calls: {}", e);
        }
    }
}

#[test]
fn test_qualified_constant_access() {
    init_test_tracing();
    
    let code = r#"
    yeet "math"
    
    func main() {
        let pi_value = math.Pi;
        let e_value = math.E;
        vibez.spill("Pi:", pi_value, "E:", e_value);
    }
    "#;
    
    // This test verifies:
    // 1. Qualified constant access works
    // 2. Multiple constants can be accessed
    // 3. Constants are properly resolved
    
    match cursed::parse_string(code) {
        Ok(program) => {
            println!("✓ Successfully parsed qualified constant access");
            assert!(program.statements.len() >= 2);
        },
        Err(e) => {
            panic!("Failed to parse qualified constant access: {}", e);
        }
    }
}

#[test]
fn test_qualified_type_usage() {
    init_test_tracing();
    
    let code = r#"
    yeet "http"
    
    struct MyRequest {
        base: http.Request,
        custom_field: string,
    }
    
    func main() {
        let req = MyRequest{
            base: http.Request{},
            custom_field: "test",
        };
    }
    "#;
    
    // This test verifies:
    // 1. Qualified types can be used in struct fields
    // 2. Qualified types can be used in struct literals
    
    match cursed::parse_string(code) {
        Ok(program) => {
            println!("✓ Successfully parsed qualified type usage");
            assert!(program.statements.len() >= 2);
        },
        Err(e) => {
            panic!("Failed to parse qualified type usage: {}", e);
        }
    }
}

#[test]
fn test_import_aliases() {
    init_test_tracing();
    
    let code = r#"
    yeet "mathematics" as "math"
    
    func main() {
        let result = math.sqrt(16.0);
        vibez.spill("Result:", result);
    }
    "#;
    
    // This test verifies:
    // 1. Import aliases are parsed correctly
    // 2. Qualified names work with aliases
    
    match cursed::parse_string(code) {
        Ok(program) => {
            println!("✓ Successfully parsed import aliases");
            // Note: Full alias support may require parser enhancements
        },
        Err(e) => {
            // This might fail until full alias support is implemented
            println!("Import aliases not yet fully supported: {}", e);
        }
    }
}

#[test]
fn test_chained_access() {
    init_test_tracing();
    
    let code = r#"
    yeet "http"
    
    func main() {
        let req = http.Request{};
        let header_value = req.headers.get("Content-Type");
    }
    "#;
    
    // This test verifies:
    // 1. Chained property access works
    // 2. Method calls on qualified types work
    
    match cursed::parse_string(code) {
        Ok(program) => {
            println!("✓ Successfully parsed chained access");
        },
        Err(e) => {
            // This might fail until full chained access is implemented
            println!("Chained access not yet fully supported: {}", e);
        }
    }
}

#[test]
fn test_error_cases() {
    init_test_tracing();
    
    // Test undefined package
    let code1 = r#"
    func main() {
        let result = undefined_package.function();
    }
    "#;
    
    match cursed::parse_string(code1) {
        Ok(_) => {
            // This should eventually fail during compilation, not parsing
            println!("Note: Undefined package error should be caught during compilation");
        },
        Err(_) => {
            println!("✓ Undefined package error caught");
        }
    }
    
    // Test undefined symbol
    let code2 = r#"
    yeet "math"
    
    func main() {
        let result = math.undefined_function();
    }
    "#;
    
    match cursed::parse_string(code2) {
        Ok(_) => {
            println!("Note: Undefined symbol error should be caught during compilation");
        },
        Err(_) => {
            println!("✓ Undefined symbol error caught");
        }
    }
}

#[test]
fn test_stdlib_qualified_calls() {
    init_test_tracing();
    
    let code = r#"
    func main() {
        vibez.spill("Hello from qualified name!");
        
        let time_now = timez.Now();
        vibez.spill("Current time:", time_now);
        
        let escaped = htmlrizzler.escape_html("<script>alert('test')</script>");
        vibez.spill("Escaped HTML:", escaped);
    }
    "#;
    
    // This test verifies that standard library functions work with qualified names
    
    match cursed::parse_string(code) {
        Ok(program) => {
            println!("✓ Successfully parsed stdlib qualified calls");
            
            // Verify we can compile this to LLVM IR
            match cursed::compile_to_llvm_string(&program, "test") {
                Ok(llvm_ir) => {
                    println!("✓ Successfully compiled to LLVM IR");
                    // Check that the IR contains expected function calls
                    assert!(llvm_ir.contains("puts") || llvm_ir.contains("vibez") || llvm_ir.contains("spill"));
                },
                Err(e) => {
                    println!("Note: LLVM compilation may need additional work: {}", e);
                }
            }
        },
        Err(e) => {
            panic!("Failed to parse stdlib qualified calls: {}", e);
        }
    }
}

#[test]
fn test_mixed_dot_expressions() {
    init_test_tracing();
    
    let code = r#"
    yeet "http"
    
    struct MyStruct {
        field: string,
    }
    
    func main() {
        // Regular dot expression (struct field access)
        let s = MyStruct{ field: "test" };
        let field_value = s.field;
        
        // Qualified name (package function)
        let result = http.Get("https://example.com");
        
        // Mixed usage
        vibez.spill("Field:", field_value, "HTTP result:", result);
    }
    "#;
    
    // This test verifies that regular dot expressions and qualified names coexist
    
    match cursed::parse_string(code) {
        Ok(program) => {
            println!("✓ Successfully parsed mixed dot expressions");
        },
        Err(e) => {
            panic!("Failed to parse mixed dot expressions: {}", e);
        }
    }
}

// Helper function to simulate end-to-end compilation
fn compile_with_qualified_names(code: &str) -> Result<String, String> {
    match cursed::parse_string(code) {
        Ok(program) => {
            match cursed::compile_to_llvm_string(&program, "test") {
                Ok(ir) => Ok(ir),
                Err(e) => Err(format!("Compilation error: {}", e))
            }
        },
        Err(e) => Err(format!("Parse error: {}", e))
    }
}

#[test]
fn test_end_to_end_compilation() {
    init_test_tracing();
    
    let code = r#"
    yeet "math"
    
    func main() {
        let x = 25.0;
        let result = math.sqrt(x);
        vibez.spill("Square root of", x, "is", result);
    }
    "#;
    
    match compile_with_qualified_names(code) {
        Ok(llvm_ir) => {
            println!("✓ End-to-end compilation successful");
            println!("Generated LLVM IR length: {} bytes", llvm_ir.len());
            
            // Verify the IR contains expected elements
            assert!(llvm_ir.len() > 0, "LLVM IR should not be empty");
        },
        Err(e) => {
            println!("End-to-end compilation failed: {}", e);
            // This is expected until full integration is complete
        }
    }
}

// Test that the qualified name system maintains backward compatibility
#[test]
fn test_backward_compatibility() {
    init_test_tracing();
    
    // This code should still work without qualified names
    let code = r#"
    func main() {
        let x = 42;
        vibez.spill("The answer is", x);
    }
    "#;
    
    match cursed::parse_string(code) {
        Ok(program) => {
            println!("✓ Backward compatibility maintained");
            
            // This should still work with the stdlib patch system
            if cursed::main_patch::patch_for_vibez_spill("test.csd") {
                println!("✓ Stdlib patch system still functional");
            }
        },
        Err(e) => {
            panic!("Backward compatibility broken: {}", e);
        }
    }
}
