//! Comprehensive tests for advanced function signature parsing

use cursed::parser::advanced_signature_parser::*;
use cursed::parser::AdvancedSignatureParser;
use cursed::lexer::{Lexer, TokenKind};
use cursed::ast::Type;

fn create_tokens(input: &str) -> Vec<cursed::lexer::Token> {
    let mut lexer = Lexer::new(input.to_string());
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token() {
            Ok(token) => {
                let is_eof = token.kind == TokenKind::Eof;
                tokens.push(token);
                if is_eof { break; }
            }
            Err(_) => break,
        }
    }
    
    tokens
}

#[test]
fn test_parse_simple_function_signature() {
    let input = "slay add(x normie, y normie) -> normie";
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    assert!(result.is_ok());
    
    let signature = result.unwrap();
    assert_eq!(signature.name, "add");
    assert_eq!(signature.parameters.len(), 2);
    assert!(signature.return_type.is_some());
    assert!(!signature.is_async);
    assert!(!signature.is_unsafe);
}

#[test]
fn test_parse_variadic_function_signature() {
    let input = "slay printf(format tea, ...args normie)";
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    assert!(result.is_ok());
    
    let signature = result.unwrap();
    assert_eq!(signature.name, "printf");
    assert_eq!(signature.parameters.len(), 2);
    
    // First parameter should not be variadic
    assert!(!signature.parameters[0].is_variadic);
    assert_eq!(signature.parameters[0].name, "format");
    
    // Second parameter should be variadic
    assert!(signature.parameters[1].is_variadic);
    assert_eq!(signature.parameters[1].name, "args");
}

#[test]
fn test_parse_tuple_return_type() {
    let input = "slay get_coordinates() -> (normie, normie)";
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    assert!(result.is_ok());
    
    let signature = result.unwrap();
    assert_eq!(signature.name, "get_coordinates");
    assert!(signature.return_type.is_some());
    
    if let Some(Type::Tuple(types)) = signature.return_type {
        assert_eq!(types.len(), 2);
        assert!(matches!(types[0], Type::Normie));
        assert!(matches!(types[1], Type::Normie));
    } else {
        panic!("Expected tuple return type");
    }
}

#[test]
fn test_parse_function_pointer_parameter() {
    let input = "slay callback(handler fn(normie) -> lit)";
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    assert!(result.is_ok());
    
    let signature = result.unwrap();
    assert_eq!(signature.parameters.len(), 1);
    
    if let Some(Type::Function(params, ret)) = &signature.parameters[0].param_type {
        assert_eq!(params.len(), 1);
        assert!(matches!(params[0], Type::Normie));
        assert!(matches!(**ret, Type::Lit));
    } else {
        panic!("Expected function pointer parameter type");
    }
}

#[test]
fn test_parse_generic_function_with_bounds() {
    let input = "slay sort<T: Clone + Debug>(items [T]) where T: Ord";
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    assert!(result.is_ok());
    
    let signature = result.unwrap();
    assert_eq!(signature.name, "sort");
    assert_eq!(signature.type_parameters.len(), 1);
    
    let type_param = &signature.type_parameters[0];
    assert_eq!(type_param.name, "T");
    assert_eq!(type_param.bounds.len(), 2);
    
    // Check that where clause is parsed
    assert!(!signature.where_clauses.is_empty());
}

#[test]
fn test_parse_async_function() {
    let input = "async slay fetch_data(url tea) -> tea";
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    assert!(result.is_ok());
    
    let signature = result.unwrap();
    assert_eq!(signature.name, "fetch_data");
    assert!(signature.is_async);
    assert!(!signature.is_unsafe);
}

#[test]
fn test_parse_unsafe_function() {
    let input = "unsafe slay raw_memory_access(ptr *normie) -> normie";
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    assert!(result.is_ok());
    
    let signature = result.unwrap();
    assert_eq!(signature.name, "raw_memory_access");
    assert!(!signature.is_async);
    assert!(signature.is_unsafe);
    
    // Check pointer type parameter
    if let Some(Type::Pointer(inner)) = &signature.parameters[0].param_type {
        assert!(matches!(**inner, Type::Normie));
    } else {
        panic!("Expected pointer parameter type");
    }
}

#[test]
fn test_parse_complex_array_types() {
    let input = "slay process_matrix(matrix [[normie; 10]; 20], buffer []byte)";
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    assert!(result.is_ok());
    
    let signature = result.unwrap();
    assert_eq!(signature.parameters.len(), 2);
    
    // First parameter should be a 2D array
    if let Some(Type::Array(inner1, _)) = &signature.parameters[0].param_type {
        if let Type::Array(inner2, _) = inner1.as_ref() {
            assert!(matches!(**inner2, Type::Normie));
        } else {
            panic!("Expected nested array type");
        }
    } else {
        panic!("Expected array parameter type");
    }
    
    // Second parameter should be a slice
    if let Some(Type::Slice(inner)) = &signature.parameters[1].param_type {
        assert!(matches!(**inner, Type::Byte));
    } else {
        panic!("Expected slice parameter type");
    }
}

#[test]
fn test_parse_mutable_parameters() {
    let input = "slay modify_data(mut buffer []byte, mut count normie)";
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    assert!(result.is_ok());
    
    let signature = result.unwrap();
    assert_eq!(signature.parameters.len(), 2);
    
    // Both parameters should be mutable
    assert!(signature.parameters[0].is_mutable);
    assert!(signature.parameters[1].is_mutable);
}

#[test]
fn test_parse_function_with_default_parameters() {
    let input = "slay connect(host tea = \"localhost\", port normie = 8080)";
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    assert!(result.is_ok());
    
    let signature = result.unwrap();
    assert_eq!(signature.parameters.len(), 2);
    
    // Both parameters should have default values
    assert!(signature.parameters[0].default_value.is_some());
    assert!(signature.parameters[1].default_value.is_some());
}

#[test]
fn test_parse_complex_where_clauses() {
    let input = r#"slay complex_function<T, U, V>(x T, y U, z V) -> T 
                  where T: Clone + Debug + Display,
                        U: PartialEq + Eq,
                        V: Send + Sync"#;
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    assert!(result.is_ok());
    
    let signature = result.unwrap();
    assert_eq!(signature.type_parameters.len(), 3);
    assert!(!signature.where_clauses.is_empty());
    
    // Check that multiple where clauses are parsed
    let constraints = &signature.where_clauses[0].constraints;
    assert!(constraints.len() >= 3);
}

#[test]
fn test_parse_visibility_modifiers() {
    let input = "pub slay public_function(x normie)";
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    assert!(result.is_ok());
    
    let signature = result.unwrap();
    assert!(matches!(signature.visibility, cursed::ast::Visibility::Public));
}

#[test]
fn test_parse_documentation_comments() {
    let input = r#"/// This function adds two numbers
                   /// and returns the result
                   slay add(x normie, y normie) -> normie"#;
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    assert!(result.is_ok());
    
    let signature = result.unwrap();
    assert!(signature.documentation.is_some());
    
    let doc = signature.documentation.unwrap();
    assert!(doc.contains("adds two numbers"));
    assert!(doc.contains("returns the result"));
}

#[test]
fn test_parse_nested_generic_types() {
    let input = "slay process<T: Clone>(data squad<T>, processor fn(T) -> T) -> squad<T>";
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    assert!(result.is_ok());
    
    let signature = result.unwrap();
    assert_eq!(signature.name, "process");
    assert_eq!(signature.type_parameters.len(), 1);
    assert_eq!(signature.parameters.len(), 2);
    
    // Check generic bounds
    let type_param = &signature.type_parameters[0];
    assert_eq!(type_param.name, "T");
    assert!(!type_param.bounds.is_empty());
}

#[test]
fn test_parse_self_hosting_compiler_signature() {
    let input = r#"/// Parse a complete CURSED program into an AST
                   pub async slay parse_program<T: TokenStream>(
                       tokens T,
                       options ParserOptions,
                       ...extensions ParserExtension
                   ) -> Result<Program, ParseError> 
                   where T: Iterator<Item = Token> + Clone"#;
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    assert!(result.is_ok());
    
    let signature = result.unwrap();
    assert_eq!(signature.name, "parse_program");
    assert!(signature.is_async);
    assert_eq!(signature.type_parameters.len(), 1);
    assert_eq!(signature.parameters.len(), 3);
    
    // Check variadic parameter
    assert!(signature.parameters[2].is_variadic);
    assert_eq!(signature.parameters[2].name, "extensions");
    
    // Check where clause
    assert!(!signature.where_clauses.is_empty());
    
    // Check documentation
    assert!(signature.documentation.is_some());
}

#[test]
fn test_error_recovery_on_invalid_syntax() {
    let input = "slay invalid_function(x) {"; // Missing type and incomplete
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    // Should return an error but not panic
    assert!(result.is_err());
}

#[test]
fn test_parse_multiple_tuple_parameters() {
    let input = "slay transform(point1 (normie, normie), point2 (normie, normie)) -> (normie, normie)";
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    assert!(result.is_ok());
    
    let signature = result.unwrap();
    assert_eq!(signature.parameters.len(), 2);
    
    // Both parameters should be tuple types
    for param in &signature.parameters {
        if let Some(Type::Tuple(types)) = &param.param_type {
            assert_eq!(types.len(), 2);
        } else {
            panic!("Expected tuple parameter type");
        }
    }
}

#[test]
fn test_parse_function_returning_function() {
    let input = "slay create_handler() -> fn(normie) -> lit";
    let tokens = create_tokens(input);
    let mut parser = AdvancedSignatureParser::new(&tokens);
    
    let result = parser.parse_advanced_function_signature();
    assert!(result.is_ok());
    
    let signature = result.unwrap();
    
    // Should return a function pointer type
    if let Some(Type::Function(params, ret)) = signature.return_type {
        assert_eq!(params.len(), 1);
        assert!(matches!(params[0], Type::Normie));
        assert!(matches!(**ret, Type::Lit));
    } else {
        panic!("Expected function pointer return type");
    }
}
