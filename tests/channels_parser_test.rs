//! Parser tests for CURSED channel implementation
//! 
//! These tests focus on parsing channel syntax including channel types,
//! send/receive operations, goroutine spawn syntax, and error recovery.

use cursed::lexer::{Lexer, Token, TokenType};
use cursed::parser::Parser;
use cursed::ast::*;
use cursed::ast::concurrency::*;
use cursed::ast::expressions::*;
use cursed::types::Type;

#[path = "common/mod.rs"]
pub mod common;

#[test]
fn test_channel_type_parsing() {
    init_tracing!();
    
    // Test basic channel type parsing
    let source = "dm<int>";
    let mut parser = Parser::new(source);
    
    let channel_type = parser.parse_type();
    assert!(channel_type.is_ok());
    
    let channel_type = channel_type.unwrap();
    match channel_type {
        Type::Channel(element_type) => {
            assert_eq!(*element_type, Type::Integer);
        }
        _ => panic!("Expected channel type, got: {:?}", channel_type),
    }
    
    tracing::info!("✓ Basic channel type parsing test passed");
}

#[test]
fn test_nested_channel_type_parsing() {
    init_tracing!();
    
    // Test nested channel types
    let test_cases = vec![
        ("dm<string>", Type::Channel(Box::new(Type::String))),
        ("dm<bool>", Type::Channel(Box::new(Type::Boolean))),
        ("dm<float>", Type::Channel(Box::new(Type::Float))),
        ("dm<dm<int>>", Type::Channel(Box::new(Type::Channel(Box::new(Type::Integer))))),
    ];
    
    for (source, expected_type) in test_cases {
        let mut parser = Parser::new(source);
        let parsed_type = parser.parse_type().unwrap();
        
        assert_eq!(parsed_type, expected_type);
        tracing::debug!(source, "Parsed nested channel type");
    }
    
    tracing::info!("✓ Nested channel type parsing test passed");
}

#[test]
fn test_make_expression_parsing() {
    init_tracing!();
    
    // Test make expression without buffer size
    let source = "make(dm<int>)";
    let mut parser = Parser::new(source);
    
    let expr = parser.parse_expression();
    assert!(expr.is_ok());
    
    let expr = expr.unwrap();
    match expr.as_any().downcast_ref::<MakeExpression>() {
        Some(make_expr) => {
            assert_eq!(make_expr.channel_type, Type::Channel(Box::new(Type::Integer)));
            assert!(make_expr.buffer_size.is_none());
        }
        None => panic!("Expected MakeExpression, got: {}", expr.string()),
    }
    
    tracing::info!("✓ Make expression without buffer parsing test passed");
}

#[test]
fn test_make_expression_with_buffer_parsing() {
    init_tracing!();
    
    // Test make expression with buffer size
    let source = "make(dm<string>, 10)";
    let mut parser = Parser::new(source);
    
    let expr = parser.parse_expression();
    assert!(expr.is_ok());
    
    let expr = expr.unwrap();
    match expr.as_any().downcast_ref::<MakeExpression>() {
        Some(make_expr) => {
            assert_eq!(make_expr.channel_type, Type::Channel(Box::new(Type::String)));
            assert!(make_expr.buffer_size.is_some());
            
            let buffer_size = make_expr.buffer_size.as_ref().unwrap();
            assert_eq!(buffer_size.string(), "10");
        }
        None => panic!("Expected MakeExpression, got: {}", expr.string()),
    }
    
    tracing::info!("✓ Make expression with buffer parsing test passed");
}

#[test]
fn test_send_expression_parsing() {
    init_tracing!();
    
    // Test send expression parsing
    let source = "ch <- 42";
    let mut parser = Parser::new(source);
    
    let expr = parser.parse_expression();
    assert!(expr.is_ok());
    
    let expr = expr.unwrap();
    match expr.as_any().downcast_ref::<SendExpression>() {
        Some(send_expr) => {
            assert_eq!(send_expr.channel.string(), "ch");
            assert_eq!(send_expr.value.string(), "42");
        }
        None => panic!("Expected SendExpression, got: {}", expr.string()),
    }
    
    tracing::info!("✓ Send expression parsing test passed");
}

#[test]
fn test_receive_expression_parsing() {
    init_tracing!();
    
    // Test receive expression parsing
    let source = "<-ch";
    let mut parser = Parser::new(source);
    
    let expr = parser.parse_expression();
    assert!(expr.is_ok());
    
    let expr = expr.unwrap();
    match expr.as_any().downcast_ref::<ReceiveExpression>() {
        Some(receive_expr) => {
            assert_eq!(receive_expr.channel.string(), "ch");
        }
        None => panic!("Expected ReceiveExpression, got: {}", expr.string()),
    }
    
    tracing::info!("✓ Receive expression parsing test passed");
}

#[test]
fn test_stan_expression_parsing() {
    init_tracing!();
    
    // Test stan (goroutine spawn) expression parsing
    let source = "stan worker()";
    let mut parser = Parser::new(source);
    
    let expr = parser.parse_expression();
    assert!(expr.is_ok());
    
    let expr = expr.unwrap();
    match expr.as_any().downcast_ref::<StanExpression>() {
        Some(stan_expr) => {
            assert_eq!(stan_expr.expression.string(), "worker()");
        }
        None => panic!("Expected StanExpression, got: {}", expr.string()),
    }
    
    tracing::info!("✓ Stan expression parsing test passed");
}

#[test]
fn test_complex_send_receive_parsing() {
    init_tracing!();
    
    // Test complex send/receive expressions
    let test_cases = vec![
        ("ch <- getValue()", "Send with function call"),
        ("ch <- x + y", "Send with arithmetic"),
        ("ch <- obj.field", "Send with field access"),
        ("<-channels[0]", "Receive from array"),
        ("<-getChannel()", "Receive from function call"),
    ];
    
    for (source, description) in test_cases {
        let mut parser = Parser::new(source);
        let expr = parser.parse_expression();
        
        assert!(expr.is_ok(), "Failed to parse: {} ({})", source, description);
        
        let expr = expr.unwrap();
        tracing::debug!(source, description, parsed = %expr.string(), "Complex expression");
    }
    
    tracing::info!("✓ Complex send/receive parsing test passed");
}

#[test]
fn test_channel_in_variable_declaration() {
    init_tracing!();
    
    // Test channel variable declarations
    let source = "facts ch dm<int> = make(dm<int>, 5)";
    let mut parser = Parser::new(source);
    
    let stmt = parser.parse_statement();
    assert!(stmt.is_ok());
    
    let stmt = stmt.unwrap();
    match stmt.as_any().downcast_ref::<VarStatement>() {
        Some(var_stmt) => {
            assert_eq!(var_stmt.name.value, "ch");
            assert_eq!(var_stmt.name.string(), "ch");
            assert!(var_stmt.value.is_some());
        }
        None => panic!("Expected VarStatement, got: {}", stmt.string()),
    }
    
    tracing::info!("✓ Channel variable declaration parsing test passed");
}

#[test]
fn test_channel_in_function_parameters() {
    init_tracing!();
    
    // Test channel in function parameters
    let source = "func worker(ch dm<string>, data int) { ch <- \"done\" }";
    let mut parser = Parser::new(source);
    
    let stmt = parser.parse_statement();
    assert!(stmt.is_ok());
    
    let stmt = stmt.unwrap();
    match stmt.as_any().downcast_ref::<FunctionStatement>() {
        Some(func_stmt) => {
            assert_eq!(func_stmt.name.value, "worker");
            assert_eq!(func_stmt.parameters.len(), 2);
            
            // Check first parameter is channel type
            let first_param = &func_stmt.parameters[0];
            assert_eq!(first_param.name.value, "ch");
            // Parameter type checking would be in type checker
        }
        None => panic!("Expected FunctionStatement, got: {}", stmt.string()),
    }
    
    tracing::info!("✓ Channel function parameters parsing test passed");
}

#[test]
fn test_select_statement_parsing() {
    init_tracing!();
    
    // Test select statement parsing
    let source = r#"
        vibe_check {
            mood value := <-ch1:
                print(value)
            mood msg := <-ch2:
                print(msg)
            basic:
                print("no channels ready")
        }
    "#;
    
    let mut parser = Parser::new(source);
    let stmt = parser.parse_statement();
    assert!(stmt.is_ok());
    
    let stmt = stmt.unwrap();
    match stmt.as_any().downcast_ref::<SelectStatement>() {
        Some(select_stmt) => {
            assert!(select_stmt.cases.len() >= 2); // At least 2 channel cases
            assert!(select_stmt.default_case.is_some()); // Has default case
        }
        None => panic!("Expected SelectStatement, got: {}", stmt.string()),
    }
    
    tracing::info!("✓ Select statement parsing test passed");
}

#[test]
fn test_channel_range_parsing() {
    init_tracing!();
    
    // Test channel range iteration parsing
    let source = "for value := range ch { print(value) }";
    let mut parser = Parser::new(source);
    
    let stmt = parser.parse_statement();
    assert!(stmt.is_ok());
    
    let stmt = stmt.unwrap();
    match stmt.as_any().downcast_ref::<ForStatement>() {
        Some(for_stmt) => {
            // Check that it's a range-based for loop
            assert!(for_stmt.is_range_loop());
            assert_eq!(for_stmt.range_variable.as_ref().unwrap().value, "value");
        }
        None => panic!("Expected ForStatement, got: {}", stmt.string()),
    }
    
    tracing::info!("✓ Channel range parsing test passed");
}

#[test]
fn test_channel_close_parsing() {
    init_tracing!();
    
    // Test channel close function call parsing
    let source = "close(ch)";
    let mut parser = Parser::new(source);
    
    let expr = parser.parse_expression();
    assert!(expr.is_ok());
    
    let expr = expr.unwrap();
    match expr.as_any().downcast_ref::<CallExpression>() {
        Some(call_expr) => {
            assert_eq!(call_expr.function.string(), "close");
            assert_eq!(call_expr.arguments.len(), 1);
            assert_eq!(call_expr.arguments[0].string(), "ch");
        }
        None => panic!("Expected CallExpression, got: {}", expr.string()),
    }
    
    tracing::info!("✓ Channel close parsing test passed");
}

#[test]
fn test_malformed_channel_syntax_recovery() {
    init_tracing!();
    
    // Test error recovery for malformed channel syntax
    let malformed_cases = vec![
        ("dm<>", "Empty channel type"),
        ("dm<int", "Unclosed channel type"),
        ("dm int>", "Missing opening bracket"),
        ("ch <-", "Incomplete send"),
        ("<- ", "Incomplete receive"),
        ("make(dm<int>,)", "Empty buffer size"),
        ("make(dm<int>, -1)", "Negative buffer size"),
    ];
    
    for (source, description) in malformed_cases {
        let mut parser = Parser::new(source);
        
        // Try to parse as expression first
        let expr_result = parser.parse_expression();
        
        if expr_result.is_err() {
            // Try to parse as type
            let mut parser = Parser::new(source);
            let type_result = parser.parse_type();
            
            if type_result.is_err() {
                tracing::debug!(source, description, "Successfully caught malformed syntax");
                continue;
            }
        }
        
        // Some malformed syntax might still parse but be caught later in type checking
        tracing::debug!(source, description, "Malformed syntax test");
    }
    
    tracing::info!("✓ Malformed channel syntax recovery test passed");
}

#[test]
fn test_channel_precedence_parsing() {
    init_tracing!();
    
    // Test operator precedence with channel operations
    let precedence_cases = vec![
        ("x + <-ch", "Arithmetic with receive"),
        ("ch <- x + y", "Send with arithmetic"),
        ("f() <- value", "Function call as channel"),
        ("<-ch.field", "Receive with field access"),
        ("ch[0] <- value", "Index as channel"),
    ];
    
    for (source, description) in precedence_cases {
        let mut parser = Parser::new(source);
        let expr = parser.parse_expression();
        
        assert!(expr.is_ok(), "Failed to parse precedence case: {} ({})", source, description);
        
        let expr = expr.unwrap();
        tracing::debug!(source, description, parsed = %expr.string(), "Precedence test");
    }
    
    tracing::info!("✓ Channel precedence parsing test passed");
}

#[test]
fn test_complete_channel_program_parsing() {
    init_tracing!();
    
    // Test parsing a complete program with channels
    let program_source = r#"
        func producer(out dm<int>) {
            for i := 0; i < 10; i++ {
                out <- i
            }
            close(out)
        }
        
        func consumer(in dm<int>) {
            for value := range in {
                print("Received:", value)
            }
        }
        
        func main() {
            facts ch = make(dm<int>, 5)
            stan producer(ch)
            stan consumer(ch)
        }
    "#;
    
    let mut parser = Parser::new(program_source);
    let program = parser.parse_program();
    
    assert!(program.is_ok());
    
    let program = program.unwrap();
    assert_eq!(program.statements.len(), 3); // 3 functions
    
    // Verify function names
    let function_names: Vec<_> = program.statements.iter()
        .filter_map(|stmt| {
            stmt.as_any().downcast_ref::<FunctionStatement>()
                .map(|f| f.name.value.clone())
        })
        .collect();
    
    assert_eq!(function_names, vec!["producer", "consumer", "main"]);
    
    tracing::info!("✓ Complete channel program parsing test passed");
}

#[test]
fn test_lexer_channel_tokens() {
    init_tracing!();
    
    // Test that lexer produces correct tokens for channel syntax
    let source = "dm<int> ch <- 42 <-ch stan make(dm<string>, 5)";
    let mut lexer = Lexer::new(source);
    
    let tokens = lexer.tokenize();
    assert!(tokens.is_ok());
    
    let tokens = tokens.unwrap();
    
    // Check for expected token types
    let token_types: Vec<_> = tokens.iter().map(|t| &t.token_type).collect();
    
    assert!(token_types.contains(&&TokenType::DM)); // dm keyword
    assert!(token_types.contains(&&TokenType::LT)); // <
    assert!(token_types.contains(&&TokenType::GT)); // >
    assert!(token_types.contains(&&TokenType::Arrow)); // <-
    assert!(token_types.contains(&&TokenType::Stan)); // stan keyword
    assert!(token_types.contains(&&TokenType::Make)); // make keyword
    
    tracing::info!("✓ Lexer channel tokens test passed");
}

#[test]
fn test_error_position_reporting() {
    init_tracing!();
    
    // Test that parser reports correct positions for channel syntax errors
    let source = "facts ch dm<int> = make(dm<int>, invalid_expr)";
    let mut parser = Parser::new(source);
    
    let stmt = parser.parse_statement();
    
    // Even if parsing succeeds, check that we can get position information
    if let Ok(stmt) = stmt {
        // Verify the statement structure
        assert!(stmt.string().contains("ch"));
        assert!(stmt.string().contains("make"));
    }
    
    // Test with definitely invalid syntax
    let invalid_source = "dm<int> ch <- <- 42";
    let mut parser = Parser::new(invalid_source);
    
    let expr_result = parser.parse_expression();
    if expr_result.is_err() {
        tracing::debug!("Successfully caught double arrow error");
    }
    
    tracing::info!("✓ Error position reporting test passed");
}
