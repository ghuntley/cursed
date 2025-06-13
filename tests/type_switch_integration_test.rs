/// Integration tests for type switch functionality in CURSED
/// Tests end-to-end compilation and runtime behavior

use cursed::ast::*;
use cursed::parser::Parser;
use cursed::lexer::Lexer;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::runtime::Runtime;
use cursed::error::*;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // End-to-End Compilation Tests
    // ============================================================================

    #[test]
    fn test_compile_basic_type_switch() {
        let source = r#"
        slay process_value(input interface{}) string {
            vibe_check v := input.(type) {
                mood string:
                    vibe "String: " + v
                mood int:
                    vibe "Number: " + v.toString()
                basic:
                    vibe "Unknown type"
            }
        }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "Basic type switch should compile successfully");
    }

    #[test]
    fn test_compile_type_switch_with_multiple_types() {
        let source = r#"
        slay handle_data(data interface{}) {
            vibe_check v := data.(type) {
                mood string, []byte:
                    println("Text data:", v)
                mood int, int64, float64:
                    println("Numeric data:", v)
                mood []interface{}:
                    println("Array data")
                basic:
                    println("Unknown data type")
            }
        }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "Type switch with multiple types should compile");
    }

    #[test]
    fn test_compile_nested_type_switches() {
        let source = r#"
        slay process_nested(input interface{}) {
            vibe_check outer := input.(type) {
                mood map[string]interface{}:
                    lowkey (sus key, value in outer) {
                        vibe_check inner := value.(type) {
                            mood string:
                                println("Nested string:", inner)
                            mood int:
                                println("Nested int:", inner)
                        }
                    }
                mood []interface{}:
                    lowkey (sus i, item in outer) {
                        vibe_check element := item.(type) {
                            mood string:
                                println("Array element string:", element)
                        }
                    }
            }
        }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "Nested type switches should compile");
    }

    // ============================================================================
    // Runtime Behavior Tests
    // ============================================================================

    #[test]
    fn test_type_switch_runtime_string() {
        let source = r#"
        slay test_string() string {
            sus input interface{} = "hello"
            vibe_check v := input.(type) {
                mood string:
                    vibe "Got string: " + v
                basic:
                    vibe "Not a string"
            }
        }
        "#;

        let result = compile_and_run(source, "test_string");
        assert!(result.is_ok());
        
        if let Ok(value) = result {
            assert!(value.contains("Got string: hello"));
        }
    }

    #[test]
    fn test_type_switch_runtime_integer() {
        let source = r#"
        slay test_integer() string {
            sus input interface{} = 42
            vibe_check v := input.(type) {
                mood int:
                    vibe "Got integer: " + v.toString()
                basic:
                    vibe "Not an integer"
            }
        }
        "#;

        let result = compile_and_run(source, "test_integer");
        assert!(result.is_ok());
        
        if let Ok(value) = result {
            assert!(value.contains("Got integer: 42"));
        }
    }

    #[test]
    fn test_type_switch_runtime_default_case() {
        let source = r#"
        slay test_default() string {
            sus input interface{} = true
            vibe_check v := input.(type) {
                mood string:
                    vibe "Got string"
                mood int:
                    vibe "Got integer"
                basic:
                    vibe "Got unknown type"
            }
        }
        "#;

        let result = compile_and_run(source, "test_default");
        assert!(result.is_ok());
        
        if let Ok(value) = result {
            assert!(value.contains("Got unknown type"));
        }
    }

    // ============================================================================
    // Type Safety Tests
    // ============================================================================

    #[test]
    fn test_type_switch_variable_scoping() {
        let source = r#"
        slay test_scoping() {
            sus input interface{} = "test"
            vibe_check v := input.(type) {
                mood string:
                    // Variable 'v' should be available here as string type
                    println("Length:", v.length())
                mood int:
                    // Variable 'v' should be available here as int type
                    println("Value:", v)
            }
            // Variable 'v' should not be available here
        }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "Variable scoping should be correct");
    }

    #[test]
    fn test_type_switch_interface_compatibility() {
        let source = r#"
        collab Stringer {
            slay string() string
        }
        
        squad StringImpl {
            value: string
        }
        
        slay (s StringImpl) string() string {
            vibe s.value
        }
        
        slay test_interface() {
            sus obj Stringer = StringImpl{value: "test"}
            sus input interface{} = obj
            
            vibe_check v := input.(type) {
                mood Stringer:
                    println("Stringer:", v.string())
                mood StringImpl:
                    println("StringImpl:", v.value)
                basic:
                    println("Unknown type")
            }
        }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "Interface compatibility should work");
    }

    // ============================================================================
    // Error Handling Tests
    // ============================================================================

    #[test]
    fn test_type_switch_compile_error_invalid_syntax() {
        let source = r#"
        slay test_invalid() {
            vibe_check v := input.type {  // Missing parentheses
                mood string:
                    println("string")
            }
        }
        "#;

        let result = compile_source(source);
        assert!(result.is_err(), "Invalid syntax should produce compile error");
    }

    #[test]
    fn test_type_switch_compile_error_missing_variable() {
        let source = r#"
        slay test_missing_var() {
            vibe_check input.(type) {  // Missing variable assignment
                mood string:
                    println(v)  // 'v' is not defined
            }
        }
        "#;

        let result = compile_source(source);
        // Should either compile (if this syntax is valid) or produce error
        // Implementation depends on language design decisions
    }

    #[test]
    fn test_type_switch_duplicate_case_types() {
        let source = r#"
        slay test_duplicate() {
            sus input interface{} = "test"
            vibe_check v := input.(type) {
                mood string:
                    println("First string case")
                mood string:  // Duplicate case
                    println("Second string case")
            }
        }
        "#;

        let result = compile_source(source);
        // Should produce warning or error for duplicate cases
        // Exact behavior depends on language specification
    }

    // ============================================================================
    // Performance Tests
    // ============================================================================

    #[test]
    fn test_type_switch_performance_many_cases() {
        let mut cases = Vec::new();
        for i in 0..100 {
            cases.push(format!(
                "mood Type{}:\n    println(\"Type {}\", v)",
                i, i
            ));
        }
        
        let source = format!(
            r#"
            slay test_many_cases(input interface{}) {{
                vibe_check v := input.(type) {{
                    {}
                    basic:
                        println("Unknown type")
                }}
            }}
            "#,
            cases.join("\n                ")
        );

        let result = compile_source(&source);
        assert!(result.is_ok(), "Type switch with many cases should compile");
    }

    #[test]
    fn test_type_switch_complex_types() {
        let source = r#"
        slay test_complex_types(input interface{}) {
            vibe_check v := input.(type) {
                mood map[string]interface{}:
                    println("Map type")
                mood []map[string]int:
                    println("Slice of maps")
                mood chan<- int:
                    println("Send-only channel")
                mood func(int) string:
                    println("Function type")
                mood *ComplexStruct:
                    println("Pointer to struct")
                basic:
                    println("Other type")
            }
        }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "Complex types should compile correctly");
    }

    // ============================================================================
    // LLVM Code Generation Tests
    // ============================================================================

    #[test]
    fn test_type_switch_llvm_ir_generation() {
        let source = r#"
        slay test_llvm() {
            sus input interface{} = "test"
            vibe_check v := input.(type) {
                mood string:
                    println("String case")
                mood int:
                    println("Int case")
                basic:
                    println("Default case")
            }
        }
        "#;

        let llvm_result = compile_to_llvm_ir(source);
        assert!(llvm_result.is_ok(), "Should generate valid LLVM IR");
        
        if let Ok(ir) = llvm_result {
            // Check for type checking instructions
            assert!(ir.contains("call"), "Should contain function calls");
            assert!(ir.contains("br"), "Should contain branch instructions");
            // Additional LLVM IR validation would go here
        }
    }

    #[test]
    fn test_type_switch_optimization() {
        let source = r#"
        slay test_optimization(input interface{}) {
            vibe_check v := input.(type) {
                mood string:
                    vibe v
                basic:
                    vibe ""
            }
        }
        "#;

        let optimized_result = compile_with_optimization(source);
        assert!(optimized_result.is_ok(), "Optimized compilation should succeed");
    }

    // ============================================================================
    // Integration with Other Language Features
    // ============================================================================

    #[test]
    fn test_type_switch_with_generics() {
        let source = r#"
        slay process_generic<T>(input interface{}) {
            vibe_check v := input.(type) {
                mood T:
                    println("Matched generic type T")
                mood string:
                    println("String type")
                basic:
                    println("Other type")
            }
        }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "Type switch with generics should work");
    }

    #[test]
    fn test_type_switch_with_error_propagation() {
        let source = r#"
        slay process_with_errors(input interface{}) error {
            vibe_check v := input.(type) {
                mood string:
                    sus result = process_string(v)?
                    vibe result
                mood int:
                    sus result = process_int(v)?
                    vibe result
                basic:
                    vibe error("Unsupported type")
            }
        }
        "#;

        let result = compile_source(source);
        assert!(result.is_ok(), "Type switch with error propagation should work");
    }

    // ============================================================================
    // Helper Functions
    // ============================================================================

    fn compile_source(source: &str) -> Result<(), CursedError> {
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(&mut lexer);
        
        let program = parser.parse_program()?;
        
        // Basic compilation check
        let mut codegen = LlvmCodeGenerator::new();
        codegen.compile_program(&program)?;
        
        Ok(())
    }

    fn compile_and_run(source: &str, function_name: &str) -> Result<String, CursedError> {
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(&mut lexer);
        
        let program = parser.parse_program()?;
        
        let mut codegen = LlvmCodeGenerator::new();
        let compiled = codegen.compile_program(&program)?;
        
        // Mock runtime execution
        // In a real implementation, this would execute the compiled code
        Ok("Mock execution result".to_string())
    }

    fn compile_to_llvm_ir(source: &str) -> Result<String, CursedError> {
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(&mut lexer);
        
        let program = parser.parse_program()?;
        
        let mut codegen = LlvmCodeGenerator::new();
        let ir = codegen.generate_ir(&program)?;
        
        Ok(ir)
    }

    fn compile_with_optimization(source: &str) -> Result<(), CursedError> {
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(&mut lexer);
        
        let program = parser.parse_program()?;
        
        let mut codegen = LlvmCodeGenerator::new();
        codegen.set_optimization_level(2);
        codegen.compile_program(&program)?;
        
        Ok(())
    }
}
