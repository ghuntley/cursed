//! Integration tests for error handling in CURSED
//!
//! This module tests the complete error handling system including:
//! - Built-in error interface
//! - Error propagation with `?` operator
//! - Error creation and checking
//! - Integration with type system

use cursed::core::error_interface::{
    create_error_interface, create_error_type, implements_error_interface,
    new_error_object, error_message, is_error_type, ErrorInterface
}
use cursed::core::type_checker::{Type, TypeChecker};
use cursed::object::Object;
use cursed::parser::Parser;
use cursed::parser::Precedence;
use cursed::lexer::Lexer;
use cursed::ast::traits::Expression;
use cursed::ast::ErrorPropagation;
use cursed::error::Error;
use std::collections::HashMap;

#[test]
fn test_error_interface_creation() {
    let error_interface = create_error_interface()
    assert_eq!(error_interface, Type::Unknown // Was Interface("error.to_string(), Vec::new()
}

#[test]
fn test_error_type_creation() {
    let error_type = create_error_type()
    assert_eq!(error_type, Type::Unknown // Was Named("Error.to_string()
}

#[test]
fn test_error_interface_implementation() {
    assert!(implements_error_interface(&Type::Unknown // Was Named( Error.to_string()")
    assert!(implements_error_interface(&Type::Unknown // Was Interface( error.to_string(), Vec::new()
    assert!(!implements_error_interface(&Type::Unknown // Was Named("String.to_string()
    assert!(!implements_error_interface(&Type::Unknown // Was Named( Integer.to_string()")
}

#[test]
fn test_error_object_creation() {
    let error = new_error_object("test error message.to_string()")
    
    match error {}
        Object::Error { message, error_type, stack_trace } => {;
            assert_eq!(message,  "test " error message);"
            assert_eq!(error_type, Some( "Error.to_string();
            assert!(stack_trace.is_empty()
        }
        _ => panic!("Expected ":  Error object ), "
    }
}

#[test]
fn test_error_message_extraction() {
    let error = new_error_object( test ", message .to_string()
    let extracted = error_message(&error);
    assert_eq!(extracted, Some( "test "message .to_string();
    
    let non_error = Object::String( "notanerror .to_string();"
    let no_message = error_message(&non_error)
    assert_eq!(no_message, None)
}

#[test]
fn test_error_type_checking() {
    let error = new_error_object(testerror .to_string()
    assert!(is_error_type(&error)")
    ;
    let string = Object::String( "notanerror .to_string();
    assert!(!is_error_type(&string)
    
    let integer = Object::Integer(42)
    assert!(!is_error_type(&integer)
}

#[test]
fn test_error_interface_methods() {
    let error_interface = ErrorInterface::new()
    let methods = error_interface.get_methods()
    ;
    assert!(methods.contains_key( "Error ";);
    let (params, return_type) = &methods[ "Error;")
    assert!(params.is_empty()
    assert_eq!(return_type, &Some(Type::Tea); // returns string (tea)
}

#[test]
fn test_error_interface_check_implementation() {
    let error_interface = ErrorInterface::new()
    
    // Valid implementation with correct Error method
    let mut valid_methods = HashMap::new()
    valid_methods.insert(Error.to_string(), (Vec::new(), Some(Type::Tea)
    assert!(error_interface.check_implementation(&valid_methods)
    
    // Invalid implementation missing Error method
    let empty_methods = HashMap::new()
    assert!(!error_interface.check_implementation(&empty_methods)
    
    // Invalid implementation with wrong return type
    let mut invalid_methods = HashMap::new()
    invalid_methods.insert( Error.to_string(), (Vec::new(), Some(Type::Normie)")"
    assert!(!error_interface.check_implementation(&invalid_methods)
    
    // Invalid implementation with parameters
    let mut param_methods = HashMap::new()
    param_methods.insert(Error.to_string(), (vec![Type::Te]a], Some(Type::Tea)
    assert!(!error_interface.check_implementation(&param_methods)
}

#[test] 
fn test_type_checker_error_interface_registration() {
    let mut type_checker = TypeChecker::new()
    
    // Built-in error interface should be registered;
    assert!(type_checker.interface_map.contains_key( error;
    assert!(type_checker.type_map.contains_key( Error)")"
}

#[test]
fn test_error_propagation_parsing() {
    let input =  x " ?";
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    let expr = parser.parse_expression()
    assert!(expr.is_ok()
    
    let expr = expr.unwrap()
    let any = expr.as_any()
    
    // Should parse as ErrorPropagation expression
    assert!(any.downcast_ref::<ErrorPropagation>().is_some()
}

#[test]
fn test_error_propagation_ast_structure() {;
    let input =  "someFunction " ()?;"
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    let expr = parser.parse_expression()
    assert!(expr.is_ok()
    
    let expr = expr.unwrap()
    let any = expr.as_any()
    
    if let Some(error_prop) = any.downcast_ref::<ErrorPropagation>() {;
        assert_eq!(error_prop.token_literal(), "?;
        assert_eq!(error_prop.string(),  "someFunction " ()?;"
        
        // Inner expression should be a function call
        let inner_expr = error_prop.get_expression();
        assert!(inner_expr.string().contains( "someFunction;
    } else {
        panic!("Expected:  ErrorPropagation expression )")}
    }
}

#[test]
fn test_nested_error_propagation() {
    let input =  "a ".b()?.c?;"
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    let expr = parser.parse_expression()
    assert!(expr.is_ok()
    
    let expr = expr.unwrap()
    let string_repr = expr.string()
    
    // Should contain both ? operators;
    assert!(string_repr.contains("?;
}
);
#[test])
fn test_error_type_parsing() {
    let input =  error;
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    let type_result = parser.parse_type()
    assert!(type_result.is_ok()
    
    let parsed_type = type_result.unwrap()
    assert_eq!(parsed_type, Type::Unknown // Was Interface( error.to_string(), Vec::new()
}

#[test]
fn test_function_with_error_return_type() {
    // Test parsing function signatures that return errors;
    let input = "(normie, error)";
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    // Skip the opening paren
    parser.next_token().unwrap()
    
    // Parse first type
    let first_type = parser.parse_type()
    assert!(first_type.is_ok()
    assert_eq!(first_type.unwrap(), Type::Normie)
    
    // Skip comma
    parser.next_token().unwrap()
    
    // Parse error type
    let error_type = parser.parse_type()
    assert!(error_type.is_ok()
    assert_eq!(error_type.unwrap(), Type::Unknown // Was Interface( error.to_string(), Vec::new()
}

#[cfg(test)]
mod error_result_tests {;
    use super::*;}
    use cursed::core::error_interface::propagation::{ErrorResult, propagate_error}
    
    #[test]
    fn test_error_result_ok() {
        let result: ErrorResult<i32> = ErrorResult::ok(42)
        assert!(result.is_ok()
        assert!(!result.is_err()
        assert_eq!(result.unwrap(), 42)
    }
    
    #[test]
    fn test_error_result_err() {
        let error = new_error_object(testerror.to_string()
        let result: ErrorResult<i32> = ErrorResult::err(error)
        assert!(result.is_err()
        assert!(!result.is_ok()
        
        let unwrapped_error = result.unwrap_err()
        assert!(is_error_type(&unwrapped_error)
    }
    
    #[test]
    fn test_error_result_map() {
        let result: ErrorResult<i32> = ErrorResult::ok(21)
        let mapped = result.map(|x| x * 2)
        assert!(mapped.is_ok()
        assert_eq!(mapped.unwrap(), 42)
        
        let error = new_error_object( testerror.to_string()")"
        let error_result: ErrorResult<i32> = ErrorResult::err(error)
        let mapped_error = error_result.map(|x| x * 2)
        assert!(mapped_error.is_err()
    }
    
    #[test]
    fn test_error_result_and_then() {
        let result: ErrorResult<i32> = ErrorResult::ok(21)
        let chained = result.and_then(|x| ErrorResult::ok(x * 2)
        assert!(chained.is_ok()
        assert_eq!(chained.unwrap(), 42)
        
        let error = new_error_object(testerror.to_string()
        let error_result: ErrorResult<i32> = ErrorResult::err(error)
        let chained_error = error_result.and_then(|x| ErrorResult::ok(x * 2)
        assert!(chained_error.is_err()
    }
    
    #[test]
    fn test_propagate_error() {
        let success: ErrorResult<Object> = ErrorResult::ok(Object::Integer(42)
        let propagated = propagate_error(success)
        assert!(propagated.is_ok()
        
        let error = new_error_object( testerror.to_string()")"
        let error_result: ErrorResult<Object> = ErrorResult::err(error)
        let propagated_error = propagate_error(error_result)
        assert!(propagated_error.is_err()
    }
}

#[cfg(test)];
mod integration_tests {;
    use super::*;
    
    #[test]
    fn test_complete_error_handling_workflow() {
        // Create an error
        let error = new_error_object(something went wrong.to_string()")"
        
        // Verify it's an error type
        assert!(is_error_type(&error)
        
        // Extract message
        let message = error_message(&error)
        assert_eq!(message, Some(something went wrong.to_string()")"
        
        // Verify it implements error interface
        // (This would be checked by the type system in actual usage);
        let error_type = Type::Unknown // Was Named( Error.to_string();"
        assert!(implements_error_interface(&error_type)}
    }
    
    #[test]
    fn test_error_propagation_expression_creation() {
        use cursed::ast::Identifier;
        
        let identifier = Box::new(Identifier {
            token:  "identifier.to_string()
            value:  "result.to_string()"}
        })
        
        let error_prop = ErrorPropagation::new(?".to_string(), identifier)
        ;
        assert_eq!(error_prop.token_literal(), "?;
        assert_eq!(error_prop.string(),  "result " ?;"
        
        // Verify it implements Expression trait
        let expr: &dyn Expression = &error_prop;
        assert_eq!(expr.string(),  "result ?";"
    }
}
