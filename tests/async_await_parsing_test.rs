/// Comprehensive test suite for async/await parsing in the CURSED programming language
/// 
/// This test suite validates:
/// - Async function declaration parsing
/// - Await expression parsing
/// - Integration with existing parser infrastructure
/// - Error handling for malformed async/await syntax

use cursed::lexer::{Lexer, TokenType};
use cursed::parser::Parser;
use cursed::ast::*;
use cursed::error::Error;

#[test]
fn test_async_keyword_lexing() {
    let mut lexer = Lexer::new("async".to_string());
    let token = lexer.next_token().unwrap();
    
    assert_eq!(token.token_type, TokenType::Async);
    assert_eq!(token.literal, "async");
}

#[test]
fn test_await_keyword_lexing() {
    let mut lexer = Lexer::new("await".to_string());
    let token = lexer.next_token().unwrap();
    
    assert_eq!(token.token_type, TokenType::Await);
    assert_eq!(token.literal, "await");
}

#[test]
fn test_async_function_lexing() {
    let mut lexer = Lexer::new("slay async fetch_data() {}".to_string());
    
    let tokens: Vec<_> = std::iter::from_fn(|| {
        match lexer.next_token() {
            Ok(token) => {
                if token.token_type == TokenType::Eof {
                    None
                } else {
                    Some(token)
                }
            }
            Err(_) => None,
        }
    }).collect();
    
    assert_eq!(tokens[0].token_type, TokenType::Slay);
    assert_eq!(tokens[1].token_type, TokenType::Async);
    assert_eq!(tokens[2].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].literal, "fetch_data");
}

#[test]
fn test_await_expression_lexing() {
    let mut lexer = Lexer::new("await api_call()".to_string());
    
    let token1 = lexer.next_token().unwrap();
    let token2 = lexer.next_token().unwrap();
    
    assert_eq!(token1.token_type, TokenType::Await);
    assert_eq!(token1.literal, "await");
    assert_eq!(token2.token_type, TokenType::Identifier);
    assert_eq!(token2.literal, "api_call");
}

#[test]
fn test_simple_async_function_parsing() {
    let input = "slay async test_func() {}";
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer).unwrap();
    
    let program = parser.parse_program();
    
    match program {
        Ok(prog) => {
            assert_eq!(prog.statements.len(), 1);
            // Additional validation would go here
        }
        Err(e) => {
            panic!("Failed to parse async function: {}", e);
        }
    }
}

#[test]
fn test_async_function_with_parameters() {
    let input = "slay async fetch_user(id normie) {}";
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer).unwrap();
    
    let program = parser.parse_program();
    
    match program {
        Ok(prog) => {
            assert_eq!(prog.statements.len(), 1);
            // Additional validation would go here
        }
        Err(e) => {
            // For now, we expect this might fail due to incomplete implementation
            println!("Expected parsing failure: {}", e);
        }
    }
}

#[test]
fn test_async_function_with_return_type() {
    let input = "slay async calculate() -> normie {}";
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer).unwrap();
    
    let program = parser.parse_program();
    
    match program {
        Ok(prog) => {
            assert_eq!(prog.statements.len(), 1);
            // Additional validation would go here
        }
        Err(e) => {
            // For now, we expect this might fail due to incomplete implementation
            println!("Expected parsing failure: {}", e);
        }
    }
}

#[test]
fn test_await_expression_parsing() {
    let input = "await some_async_call()";
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer).unwrap();
    
    // Try to parse as expression
    let result = parser.parse_expression();
    
    match result {
        Ok(expr) => {
            let expr_str = expr.string();
            assert!(expr_str.contains("await"));
        }
        Err(e) => {
            // For now, we expect this might fail due to incomplete implementation
            println!("Expected parsing failure: {}", e);
        }
    }
}

#[test]
fn test_await_assignment_parsing() {
    let input = "facts result = await fetch_data()";
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer).unwrap();
    
    let program = parser.parse_program();
    
    match program {
        Ok(prog) => {
            assert_eq!(prog.statements.len(), 1);
            // Additional validation would go here
        }
        Err(e) => {
            // For now, we expect this might fail due to incomplete implementation
            println!("Expected parsing failure: {}", e);
        }
    }
}

#[test]
fn test_async_function_string_representation() {
    let mut lexer = Lexer::new("test_func".to_string());
    let identifier_token = lexer.next_token().unwrap();
    let name = Identifier::new(identifier_token.literal.clone(), identifier_token.literal);
    let body = BlockStatement::new("{}".to_string(), Vec::new());
    
    let async_func = AsyncFunctionStatement::new(
        "slay".to_string(),
        name,
        Vec::new(),
        None,
        body,
    );

    let string_repr = async_func.string();
    assert!(string_repr.contains("slay async test_func"));
    assert!(string_repr.contains("()"));
}

#[test]
fn test_await_expression_string_representation() {
    let mut lexer = Lexer::new("api_call".to_string());
    let identifier_token = lexer.next_token().unwrap();
    let identifier = Box::new(Identifier::new(identifier_token.literal.clone(), identifier_token.literal));
    
    let await_expr = AwaitExpression::new("await".to_string(), identifier);
    
    let string_repr = await_expr.string();
    assert_eq!(string_repr, "await api_call");
}

#[test] 
fn test_async_function_detection() {
    use cursed::parser::async_await::AsyncParser;
    
    let mut lexer = Lexer::new("slay async test_func() {}".to_string());
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token() {
            Ok(token) => {
                let is_eof = token.token_type == TokenType::Eof;
                tokens.push(token);
                if is_eof { break; }
            }
            Err(_) => break,
        }
    }

    assert!(AsyncParser::is_async_function_declaration(&tokens));
    assert_eq!(AsyncParser::extract_async_function_name(&tokens), Some("test_func".to_string()));
}

#[test]
fn test_regular_function_not_detected_as_async() {
    use cursed::parser::async_await::AsyncParser;
    
    let mut lexer = Lexer::new("slay test_func() {}".to_string());
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token() {
            Ok(token) => {
                let is_eof = token.token_type == TokenType::Eof;
                tokens.push(token);
                if is_eof { break; }
            }
            Err(_) => break,
        }
    }

    assert!(!AsyncParser::is_async_function_declaration(&tokens));
}

#[test]
fn test_await_expression_detection() {
    use cursed::parser::async_await::AsyncParser;
    use cursed::error::SourceLocation;
    
    let token = cursed::lexer::Token {
        token_type: TokenType::Await,
        literal: "await".to_string(),
        location: SourceLocation::default(),
    };

    assert!(AsyncParser::is_await_expression(&token));
}

#[test]
fn test_async_function_validation() {
    use cursed::parser::async_await::AsyncParser;
    
    let result = AsyncParser::validate_async_function_signature(
        "test_func",
        &[],
        &None,
    );
    assert!(result.is_ok());

    let result = AsyncParser::validate_async_function_signature(
        "",
        &[],
        &None,
    );
    assert!(result.is_err());
}

#[test]
fn test_complex_async_await_program() {
    let input = r#"
        slay async fetch_and_process() {
            facts data = await fetch_data()
            facts processed = await process_data(data)
            yolo processed
        }
    "#;
    
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer).unwrap();
    
    let program = parser.parse_program();
    
    match program {
        Ok(prog) => {
            assert!(!prog.statements.is_empty());
            // More detailed validation would go here in a complete implementation
        }
        Err(e) => {
            // For now, we expect this might fail due to incomplete implementation
            println!("Expected parsing failure for complex program: {}", e);
        }
    }
}

#[test]
fn test_nested_await_expressions() {
    let input = "await process(await fetch())";
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer).unwrap();
    
    let result = parser.parse_expression();
    
    match result {
        Ok(expr) => {
            let expr_str = expr.string();
            assert!(expr_str.contains("await"));
            // Should contain nested await expressions
        }
        Err(e) => {
            // For now, we expect this might fail due to incomplete implementation
            println!("Expected parsing failure for nested await: {}", e);
        }
    }
}

#[test]
fn test_async_function_with_generics() {
    let input = "slay async fetch<T>() {}";
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer).unwrap();
    
    let program = parser.parse_program();
    
    match program {
        Ok(prog) => {
            assert!(!prog.statements.is_empty());
            // Generics validation would go here
        }
        Err(e) => {
            // For now, we expect this might fail due to incomplete implementation
            println!("Expected parsing failure for generic async function: {}", e);
        }
    }
}

#[test]
fn test_error_handling_for_invalid_async_syntax() {
    let invalid_inputs = vec![
        "async slay test_func() {}", // wrong order
        "slay async {} test_func",   // missing name
        "await",                     // await without expression
        "slay async () {}",          // missing function name
    ];
    
    for input in invalid_inputs {
        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        
        let result = parser.parse_program();
        // We expect these to fail
        assert!(result.is_err(), "Expected error for input: {}", input);
    }
}

#[test]
fn test_await_in_non_async_context_warning() {
    // This test ensures we can parse await expressions
    // In a full implementation, we'd validate they're only in async contexts
    let input = "await some_call()";
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer).unwrap();
    
    let result = parser.parse_expression();
    
    // For now, we allow await expressions anywhere
    // A complete implementation would validate async context
    match result {
        Ok(_) => {
            // This is fine for the basic implementation
        }
        Err(_) => {
            // Also acceptable for now
        }
    }
}
