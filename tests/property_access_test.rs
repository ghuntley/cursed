use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::jit::JitCompiler;


#[cfg(test)]
mod property_access_tests {
    use super::*;

    // Helper function to run code and get the result
    fn run_code_int(code: &str) -> Result<i64, Error> {
        use std::path::PathBuf;
        
        let mut lexer = Lexer::new(code);
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;
        
        if !parser.errors().is_empty() {
            let errors_str = parser
                .errors()
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join("\n");
            return Err(Error::from_str(&format!("Parser errors:\n{}", errors_str)));
        }
        
        // Create LLVM context and code generator
        let context = inkwell::context::Context::create();
        let mut code_gen = LlvmCodeGenerator::new());
        
        // Compile the program
        code_gen.compile(&program)?;
        
        // Create JIT execution engine
        let execution_engine = code_gen
            .module()
            .create_jit_execution_engine(inkwell::OptimizationLevel::Default)
            .map_err(|e| Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))?;
        
        // Create JIT compiler
        let mut jit_compiler = JitCompiler::new(&context, execution_engine, "_main_main", PathBuf::from("test.csd"));
        *jit_compiler.code_generator_mut() = Some(code_gen);
        
        // Execute and return result as integer
        let result = jit_compiler.execute()?;
        Ok(result as i64)
    }

    #[test]
    fn test_simple_struct_field_access() -> Result<(), Error> {
        // Initialize test tracing
        let _ = tracing_subscriber::fmt()
            .with_env_filter("debug,cursed::codegen::llvm::property_access=trace")
            .with_test_writer()
            .try_init();

        // Define a simple struct with fields and access them
        let code = r#"
            vibe main

            be_like Point squad {
                x normie
                y normie
            }

            slay main() normie {
                sus p = Point{x: 42, y: 24};
                yolo p.x;
            }
        "#;

        let result = run_code_int(code)?;
        assert_eq!(result, 42, "Expected p.x to return 42");

        Ok(())
    }

    #[test]
    fn test_nested_struct_field_access() -> Result<(), Error> {
        // Create nested structs and access fields through dot chain
        let code = r#"
            vibe main

            be_like Point squad {
                x normie
                y normie
            }

            be_like Rectangle squad {
                topLeft Point
                bottomRight Point
            }

            slay main() normie {
                sus p1 = Point{x: 10, y: 20};
                sus p2 = Point{x: 30, y: 40};
                sus rect = Rectangle{topLeft: p1, bottomRight: p2};
                yolo rect.bottomRight.x;
            }
        "#;

        let result = run_code_int(code)?;
        assert_eq!(result, 30, "Expected rect.bottomRight.x to return 30");

        Ok(())
    }

    #[test]
    fn test_field_not_found() {
        // Try to access a non-existent field
        let code = r#"
            vibe main

            be_like Point squad {
                x normie
                y normie
            }

            slay main() normie {
                sus p = Point{x: 10, y: 20};
                yolo p.z;
            }
        "#;

        // This should fail with a field not found error
        let result = run_code_int(code);
        assert!(result.is_err(), "Expected an error when accessing non-existent field");
        let error = result.unwrap_err().to_string();
        assert!(error.contains("not found") || error.contains("field") || error.contains("z"), 
                "Error should mention field not found: {}", error);
    }

    #[test]
    fn test_field_modification() -> Result<(), Error> {
        // Test field modification
        let code = r#"
            vibe main

            be_like Counter squad {
                value normie
            }

            slay main() normie {
                sus counter = Counter{value: 5};
                counter.value = 10;
                yolo counter.value;
            }
        "#;

        let result = run_code_int(code)?;
        assert_eq!(result, 10, "Expected counter.value to be 10 after modification");

        Ok(())
    }
}