use cursed::error::{CursedError, ErrorType, SourceLocation};
use cursed::parser::Parser;
use cursed::lexer::Lexer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() -> Result<(), CursedError> {
        let location = SourceLocation {
            line: 10,
            column: 5,
            file: "test.csd".to_string(),
        };
        
        let error = CursedError::new(
            ErrorType::SyntaxError,
            "Test error message".to_string(),
            Some(location),
        );
        
        assert_eq!(error.error_type(), &ErrorType::SyntaxError);
        assert_eq!(error.message(), "Test error message");
        assert!(error.location().is_some());
        
        Ok(())
    }

    #[test]
    fn test_error_propagation() {
        let result: Result<(), CursedError> = Err(CursedError::new(
            ErrorType::TypeError,
            "Type mismatch".to_string(),
            None,
        ));
        
        // Test error propagation with ? operator
        let propagated_result = test_error_propagation_helper(result);
        assert!(propagated_result.is_err());
        
        if let Err(error) = propagated_result {
            assert_eq!(error.error_type(), &ErrorType::TypeError);
        }
    }

    fn test_error_propagation_helper(input: Result<(), CursedError>) -> Result<(), CursedError> {
        input?;
        Ok(())
    }

    #[test]
    fn test_parser_error_handling() {
        let invalid_source = r#"
            slay invalid_function( {
                // Missing closing parenthesis
            }
        "#;
        
        let mut lexer = Lexer::new(invalid_source);
        match lexer.tokenize() {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                let result = parser.parse();
                
                // Should result in parse error
                assert!(result.is_err());
                
                if let Err(error) = result {
                    // Verify error contains useful information
                    assert!(!error.message().is_empty());
                }
            }
            Err(lexer_error) => {
                // Lexer error is also acceptable for malformed input
                assert!(!lexer_error.message().is_empty());
            }
        }
    }

    #[test]
    fn test_error_context_chain() -> Result<(), CursedError> {
        let original_error = CursedError::new(
            ErrorType::RuntimeError,
            "Original error".to_string(),
            None,
        );
        
        let wrapped_error = CursedError::new(
            ErrorType::CompilationError,
            "Wrapped error".to_string(),
            None,
        ).with_cause(Box::new(original_error));
        
        assert_eq!(wrapped_error.error_type(), &ErrorType::CompilationError);
        assert!(wrapped_error.cause().is_some());
        
        Ok(())
    }

    #[test]
    fn test_error_display() {
        let location = SourceLocation {
            line: 5,
            column: 10,
            file: "example.csd".to_string(),
        };
        
        let error = CursedError::new(
            ErrorType::SyntaxError,
            "Unexpected token".to_string(),
            Some(location),
        );
        
        let error_string = format!("{}", error);
        
        // Error display should include location information
        assert!(error_string.contains("example.csd"));
        assert!(error_string.contains("5"));
        assert!(error_string.contains("10"));
        assert!(error_string.contains("Unexpected token"));
    }

    #[test]
    fn test_multiple_error_types() {
        let error_types = vec![
            ErrorType::SyntaxError,
            ErrorType::TypeError,
            ErrorType::RuntimeError,
            ErrorType::CompilationError,
            ErrorType::IoError,
        ];
        
        for error_type in error_types {
            let error = CursedError::new(
                error_type.clone(),
                "Test message".to_string(),
                None,
            );
            
            assert_eq!(error.error_type(), &error_type);
        }
    }
}
