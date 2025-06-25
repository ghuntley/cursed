/// Basic Preprocessor Tests
/// 
/// This module tests the core functionality of the generic syntax preprocessor
/// to ensure it correctly identifies and processes generic patterns.

use cursed::lexer::Lexer;
use cursed::preprocessor::{Preprocessor, TokenMetadata, process_source};

#[test]
fn test_generic_type_declaration_processing() {
    let source = "be_like Box[T] squad { value T }";
    let lexer = Lexer::new(source.to_string());
    let mut preprocessor = Preprocessor::new(lexer);
    
    let result = preprocessor.process();
    assert!(result.is_ok(), "Should successfully process generic type declaration");
    
    let stream = result.unwrap();
    assert!(stream.contains_generic_type_declaration(), "Should detect generic type declaration");
    
    // Check that tokens have appropriate metadata
    let generic_tokens = stream.tokens_with_metadata(&TokenMetadata::GenericType);
    assert!(!generic_tokens.is_empty(), "Should have tokens with GenericType metadata");
    
    // Check statistics
    let stats = stream.statistics();
    assert!(stats.generic_type_tokens > 0, "Should have generic type tokens");
    assert!(stats.brackets_balanced(), "Brackets should be balanced");
}

#[test]
fn test_generic_function_declaration_processing() {
    let source = "slay foo[T](x T) T { x }";
    let lexer = Lexer::new(source.to_string());
    let mut preprocessor = Preprocessor::new(lexer);
    
    let result = preprocessor.process();
    assert!(result.is_ok(), "Should successfully process generic function declaration");
    
    let stream = result.unwrap();
    assert!(stream.contains_generic_function_declaration(), "Should detect generic function declaration");
    
    // Check that tokens have appropriate metadata
    let generic_tokens = stream.tokens_with_metadata(&TokenMetadata::GenericFunction);
    assert!(!generic_tokens.is_empty(), "Should have tokens with GenericFunction metadata");
    
    // Check statistics
    let stats = stream.statistics();
    assert!(stats.generic_function_tokens > 0, "Should have generic function tokens");
}

#[test]
fn test_generic_function_call_processing() {
    let source = "facts result = foo[normie](42)";
    let lexer = Lexer::new(source.to_string());
    let mut preprocessor = Preprocessor::new(lexer);
    
    let result = preprocessor.process();
    assert!(result.is_ok(), "Should successfully process generic function call");
    
    let stream = result.unwrap();
    assert!(stream.contains_generic_function_call(), "Should detect generic function call");
    
    // Check that tokens have appropriate metadata
    let generic_tokens = stream.tokens_with_metadata(&TokenMetadata::GenericFunctionCall);
    assert!(!generic_tokens.is_empty(), "Should have tokens with GenericFunctionCall metadata");
    
    // Check statistics
    let stats = stream.statistics();
    assert!(stats.generic_function_call_tokens > 0, "Should have generic function call tokens");
}

#[test]
fn test_nested_generic_type_processing() {
    let source = "be_like Pair[K, V[T]] squad { key K, value V[T] }";
    let lexer = Lexer::new(source.to_string());
    let mut preprocessor = Preprocessor::new(lexer);
    
    let result = preprocessor.process();
    assert!(result.is_ok(), "Should successfully process nested generic type");
    
    let stream = result.unwrap();
    assert!(stream.contains_nested_generic_type(), "Should detect nested generic type");
    
    // Check that tokens have appropriate metadata
    let nested_tokens = stream.tokens_with_metadata(&TokenMetadata::NestedGenericType);
    assert!(!nested_tokens.is_empty(), "Should have tokens with NestedGenericType metadata");
    
    // Check statistics
    let stats = stream.statistics();
    assert!(stats.nested_generic_tokens > 0, "Should have nested generic tokens");
}

#[test]
fn test_process_source_convenience_function() {
    let source = "slay map[T, U](items Array[T], func Function[T, U]) Array[U] { ... }";
    
    let result = process_source(source);
    assert!(result.is_ok(), "Should successfully process source with convenience function");
    
    let stream = result.unwrap();
    assert!(stream.contains_generic_function_declaration(), "Should detect generic function declaration");
    assert!(stream.contains_nested_generic_type(), "Should detect nested generic types in parameters");
    
    // Verify comprehensive metadata coverage
    let stats = stream.statistics();
    assert!(stats.total_tokens > 0, "Should have processed tokens");
    assert!(stats.metadata_percentage() > 0.0, "Should have some tokens with metadata");
}

#[test]
fn test_multiple_generic_patterns() {
    let source = r#"
        be_like Container[T] squad {
            items Array[T]
        }
        
        slay process[T](container Container[T]) T {
            container.items.first()
        }
        
        facts result = process[normie](my_container)
    "#;
    
    let result = process_source(source);
    assert!(result.is_ok(), "Should successfully process multiple generic patterns");
    
    let stream = result.unwrap();
    assert!(stream.contains_generic_type_declaration(), "Should detect generic type declaration");
    assert!(stream.contains_generic_function_declaration(), "Should detect generic function declaration");
    assert!(stream.contains_generic_function_call(), "Should detect generic function call");
    
    // Check that we have tokens for all metadata types
    let type_tokens = stream.tokens_with_metadata(&TokenMetadata::GenericType);
    let function_tokens = stream.tokens_with_metadata(&TokenMetadata::GenericFunction);
    let call_tokens = stream.tokens_with_metadata(&TokenMetadata::GenericFunctionCall);
    
    assert!(!type_tokens.is_empty(), "Should have generic type tokens");
    assert!(!function_tokens.is_empty(), "Should have generic function tokens");
    assert!(!call_tokens.is_empty(), "Should have generic function call tokens");
}

#[test]
fn test_preprocessor_statistics() {
    let source = "slay foo[T, U](x T, y U) Pair[T, U] { Pair[T, U] { x, y } }";
    let lexer = Lexer::new(source.to_string());
    let mut preprocessor = Preprocessor::new(lexer);
    
    // Check initial state
    assert!(preprocessor.is_initialized());
    assert_eq!(preprocessor.position(), 0);
    
    let result = preprocessor.process();
    assert!(result.is_ok());
    
    // Check final statistics
    let stats = preprocessor.statistics();
    assert!(stats.tokens_processed > 0, "Should have processed tokens");
    assert!(stats.stream_stats.total_tokens > 0, "Should have total tokens");
    assert!(stats.stream_stats.generic_function_tokens > 0, "Should have generic function tokens");
    assert!(stats.stream_stats.brackets_balanced(), "Brackets should be balanced");
    
    println!("Preprocessor Statistics:");
    println!("  Tokens processed: {}", stats.tokens_processed);
    println!("  Generic function tokens: {}", stats.stream_stats.generic_function_tokens);
    println!("  Metadata percentage: {:.1}%", stats.stream_stats.metadata_percentage());
}

#[test]
fn test_error_handling_unclosed_brackets() {
    let source = "be_like Box[T squad { value T }"; // Missing closing bracket
    let lexer = Lexer::new(source.to_string());
    let mut preprocessor = Preprocessor::new(lexer);
    
    let result = preprocessor.process();
    assert!(result.is_err(), "Should fail with unclosed brackets");
    
    let error = result.unwrap_err();
    assert!(error.to_string().contains("bracket"), "Error should mention brackets");
}

#[test]
fn test_plain_tokens_without_generics() {
    let source = "facts x = 42; facts y = \"hello\"; slay add(a normie, b normie) normie { a + b }";
    let result = process_source(source);
    assert!(result.is_ok(), "Should successfully process non-generic code");
    
    let stream = result.unwrap();
    assert!(!stream.contains_generic_type_declaration(), "Should not detect generic type declaration");
    assert!(!stream.contains_generic_function_declaration(), "Should not detect generic function declaration");
    assert!(!stream.contains_generic_function_call(), "Should not detect generic function call");
    
    // All tokens should be plain (no metadata)
    let stats = stream.statistics();
    assert!(stats.total_tokens > 0, "Should have processed tokens");
    assert_eq!(stats.metadata_percentage(), 0.0, "Should have no tokens with metadata");
}
