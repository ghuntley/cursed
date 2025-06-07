use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::code::jit_compile_and_run;
use cursed::core::JitOptions;
use cursed::object::ObjectRef;
use super::*;


#[cfg(test)]
mod property_access_tests {

    // Helper function to run code and get the result
    fn run_code(code: &str) -> Result<ObjectRef, Error> {
        let lexer = Lexer::new(code);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program()?;
        
        let options = JitOptions::default().with_main_args(vec![]);
        jit_compile_and_run(&program, options)
    }

    // Helper to run code and extract the integer result
    fn run_code_int(code: &str) -> Result<i64, Error> {
        let result = run_code(code)?;
        match result.as_i64() {
            Some(i) => Ok(i),
            None => Err(Error::from_str(&format!("Expected integer, got {:?}", result)))
        }
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
            // Define Point struct
            squad Point {
                x: normie,
                y: normie
            }

            slay main() normie {
                // Create a Point instance
                sus p = be_like Point {
                    x: 42,
                    y: 24
                };

                // Access fields
                yolo p.x; // Return the x value
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
            // Define inner struct
            squad Point {
                x: normie,
                y: normie
            }

            // Define outer struct that contains a Point
            squad Rectangle {
                topLeft: Point,
                bottomRight: Point
            }

            slay main() normie {
                // Create a nested structure
                sus p1 = be_like Point {
                    x: 10,
                    y: 20
                };

                sus p2 = be_like Point {
                    x: 30,
                    y: 40
                };

                sus rect = be_like Rectangle {
                    topLeft: p1,
                    bottomRight: p2
                };

                // Access nested field
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
            squad Point {
                x: normie,
                y: normie
            }

            slay main() normie {
                sus p = be_like Point {
                    x: 10,
                    y: 20
                };

                // Try to access non-existent field
                yolo p.z;
            }
        "#;

        // This should fail with a field not found error
        let result = run_code_int(code);
        assert!(result.is_err(), "Expected an error when accessing non-existent field");
        let error = result.unwrap_err().to_string());
        assert!(error.contains("not found"), "Error should mention field not found: {}", error);
    }

    #[test]
    fn test_field_modification() -> Result<(), Error> {
        // Test field modification
        let code = r#"
            squad Counter {
                value: normie
            }

            slay main() normie {
                sus counter = be_like Counter {
                    value: 5
                };

                // Modify the field
                counter.value = 10;

                // Return the modified value
                yolo counter.value;
            }
        "#;

        let result = run_code_int(code)?;
        assert_eq!(result, 10, "Expected counter.value to be 10 after modification");

        Ok(())
    }
}