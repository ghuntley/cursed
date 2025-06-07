use cursed::core::type_checker::{Type, TypeChecker};
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::prelude::*;


#[path = "tracing_setup.rs"]
pub mod tracing_setup;

#[macro_export]
macro_rules! init_tracing {
    () => {
        let _ = tracing_setup::init_test_tracing();
    };
}

// Helper function to run a test case for array type inference
fn test_array_type_inference(input: &str) -> Result<Type, Error> {
    // Set up tracing
    init_tracing!();
    
    // Parse the code
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;
    
    // The test input should have a single expression statement with an array literal
    let stmt = program.statements.get(0).expect("Expected a statement");
    
    // Extract the expression
    if let Some(expr_stmt) = stmt.as_any().downcast_ref::<cursed::ast::statements::ExpressionStatement>() {
        if let Some(expr) = &expr_stmt.expression {
            // Create a type checker
            let mut type_checker = TypeChecker::new();
            
            // Infer the type of the expression
            // Use the publicly available method to infer types
            type_checker.infer_type(expr.as_ref())
                .map(|_| ())
        } else {
            Err(Error::from_str("No expression in statement"))
        }
    } else {
        Err(Error::from_str("Not an expression statement"))
    }
}

#[test]
fn test_empty_array_literal() {
    let result = test_array_type_inference("[];");
    assert!(result.is_ok());
    
    if let Ok(type_) = result {
        if let Type::Array(elem_type, size) = type_ {
            assert_eq!(*elem_type, Type::Unknown);
            assert_eq!(size, 0);
        } else {
            panic!("Expected array type, got {:?}", type_);
        }
    }
}

#[test]
fn test_int_array_literal() {
    let result = test_array_type_inference("[1, 2, 3, 4, 5];");
    assert!(result.is_ok());
    
    if let Ok(type_) = result {
        if let Type::Array(elem_type, size) = type_ {
            assert_eq!(*elem_type, Type::Normie); // Should be normie (32-bit int)
            assert_eq!(size, 5);
        } else {
            panic!("Expected array type, got {:?}", type_);
        }
    }
}

#[test]
fn test_mixed_numeric_array_literal() {
    let result = test_array_type_inference("[1, 2, 3, 4.5, 5.5];");
    
    // This should fail because normie and snack are not compatible
    assert!(result.is_err());
    if let Err(err) = result {
        assert!(err.to_string().contains("must have the same type"), 
               "Error message '{}' should mention incompatible types", err.to_string());
    }
}

#[test]
fn test_string_array_literal() {
    let result = test_array_type_inference("[\"one\", \"two\", \"three\"];");
    assert!(result.is_ok());
    
    if let Ok(type_) = result {
        if let Type::Array(elem_type, size) = type_ {
            assert_eq!(*elem_type, Type::Tea); // Should be tea (string)
            assert_eq!(size, 3);
        } else {
            panic!("Expected array type, got {:?}", type_);
        }
    }
}

#[test]
fn test_mixed_types_array_literal() {
    let result = test_array_type_inference("[1, \"two\", 3];");
    
    // This should fail because int and string are not compatible
    assert!(result.is_err());
    if let Err(err) = result {
        assert!(err.to_string().contains("must have the same type"), 
               "Error message '{}' should mention incompatible types", err.to_string());
    }
}