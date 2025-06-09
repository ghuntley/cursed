use cursed::core::type_checker::{Type, TypeChecker};
use cursed::core::type_infer::TypeInference;
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

// Helper function to run a test case for map type inference
fn test_map_type_inference(input: &str) -> Result<Type, Error> {
    // Set up tracing
    init_tracing!();
    
    // Parse the code
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;
    
    // The test input should have a single expression statement with a map literal
    let stmt = program.statements.get(0).expect("Expected a statement");
    
    // Extract the expression
    if let Some(expr_stmt) = stmt.as_any().downcast_ref::<cursed::ast::statements::ExpressionStatement>() {
        if let Some(expr) = &expr_stmt.expression {
            // Create a type checker
            let mut type_checker = TypeChecker::new();
            
            // Infer the type of the expression
            // Use the publicly available method to infer types
            type_checker.infer_type(expr.as_ref())
        } else {
            Err(Error::from_str("No expression in statement"))
        }
    } else {
        Err(Error::from_str("Not an expression statement"))
    }
}

#[test]
fn test_empty_map_literal() {
    let result = test_map_type_inference("{};");
    assert!(result.is_ok());
    
    if let Ok(type_) = result {
        if let Type::Map(key_type, value_type) = type_ {
            assert_eq!(*key_type, Type::Unknown);
            assert_eq!(*value_type, Type::Unknown);
        } else {
            panic!("Expected map type, got {:?}", type_);
        }
    }
}

#[test]
fn test_string_to_int_map_literal() {
    let result = test_map_type_inference("{\"Alice\": 30, \"Bob\": 25, \"Charlie\": 35};");
    assert!(result.is_ok());
    
    if let Ok(type_) = result {
        if let Type::Map(key_type, value_type) = type_ {
            assert_eq!(*key_type, Type::Tea);
            assert_eq!(*value_type, Type::Normie);
        } else {
            panic!("Expected map type, got {:?}", type_);
        }
    }
}

#[test]
fn test_int_to_float_map_literal() {
    let result = test_map_type_inference("{1: 1.5, 2: 2.5, 3: 3.5};");
    assert!(result.is_ok());
    
    if let Ok(type_) = result {
        if let Type::Map(key_type, value_type) = type_ {
            assert_eq!(*key_type, Type::Normie);
            assert_eq!(*value_type, Type::Snack);
        } else {
            panic!("Expected map type, got {:?}", type_);
        }
    }
}

#[test]
fn test_mixed_key_types_map_literal() {
    let result = test_map_type_inference("{\"name\": \"Alice\", 1: \"Bob\"};");
    
    // This should fail because key types are inconsistent
    assert!(result.is_err());
    if let Err(err) = result {
        assert!(err.to_string().contains("Inconsistent key types"))
    }
}

#[test]
fn test_mixed_value_types_map_literal() {
    let result = test_map_type_inference("{\"Alice\": 30, \"Bob\": \"twenty-five\"};");
    
    // This should fail because value types are inconsistent
    assert!(result.is_err());
    if let Err(err) = result {
        assert!(err.to_string().contains("Inconsistent value types"))
    }
}