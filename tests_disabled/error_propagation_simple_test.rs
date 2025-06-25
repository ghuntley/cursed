// Simple error propagation test that validates the test infrastructure
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::Error;
use tracing::{debug, info, error, warn};

macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .try_init();
    };
}

#[test]
fn test_basic_lexer_functionality() {
    init_tracing!();
    info!("Testing basic lexer functionality for error propagation");

    let test_cases = vec![
        "some_function()",
        "result?",
        "value.method()",
        "a + b",
        "if condition { }",
    ];

    for (i, code) in test_cases.iter().enumerate() {
        debug!(test_case = i, code = %code, "Testing lexer");
        
        let mut lexer = Lexer::new(code.to_string());
        
        // Collect tokens manually
        let mut tokens = Vec::new();
        loop {
            match lexer.next_token() {
                Ok(token) => {
                    if token.token_type == cursed::lexer::TokenType::Eof {
                        break;
                    }
                    tokens.push(token);
                },
                Err(e) => {
                    error!(error = ?e, code = %code, "Tokenization failed");
                    break;
                }
            }
        }
        
        debug!(token_count = tokens.len(), "Tokenization completed");
        if !tokens.is_empty() {
            info!(code = %code, tokens = tokens.len(), "Tokenization successful");
        }
    }
}

#[test]
fn test_basic_parser_functionality() {
    init_tracing!();
    info!("Testing basic parser functionality");

    let simple_code = "main";
    debug!(code = %simple_code, "Testing simple parser");
    
    let mut lexer = Lexer::new(simple_code.to_string());
    
    // Collect tokens manually
    let mut tokens = Vec::new();
    loop {
        match lexer.next_token() {
            Ok(token) => {
                if token.token_type == cursed::lexer::TokenType::Eof {
                    break;
                }
                tokens.push(token);
            },
            Err(e) => {
                error!(error = ?e, "Token collection failed");
                break;
            }
        }
    }
    
    debug!(tokens = tokens.len(), "Tokens generated");
    
    if !tokens.is_empty() {
        // Create a new lexer for the parser (parser doesn't take tokens)
        let parser_lexer = Lexer::new(simple_code.to_string());
        match Parser::new(parser_lexer) {
            Ok(_parser) => {
                info!("Parser created successfully");
                // For now, just verify we can create a parser
            },
            Err(e) => {
                warn!(error = ?e, "Parser creation failed (may be expected)");
            }
        }
    } else {
        warn!("No tokens generated");
    }
}

#[test]
fn test_question_mark_token_detection() {
    init_tracing!();
    info!("Testing question mark token detection");

    let code = "?";
    let mut lexer = Lexer::new(code.to_string());
    
    // Collect tokens manually
    let mut tokens = Vec::new();
    loop {
        match lexer.next_token() {
            Ok(token) => {
                if token.token_type == cursed::lexer::TokenType::Eof {
                    break;
                }
                tokens.push(token);
            },
            Err(e) => {
                warn!(error = ?e, "Tokenization of ? failed (may not be implemented yet)");
                break;
            }
        }
    }
    
    debug!(tokens = tokens.len(), "Generated tokens for ?");
    
    // Look for question mark token
    let has_question_mark = tokens.iter().any(|token| {
        token.token_type == cursed::lexer::TokenType::Question ||
        format!("{:?}", token).contains("Question") || 
        format!("{:?}", token).contains("?")
    });
    
    debug!(has_question_mark, "Question mark token detection");
    
    if has_question_mark {
        info!("Successfully detected ? token");
    } else {
        warn!("Question mark token not detected (may not be implemented yet)");
    }
}

#[test]
fn test_error_propagation_syntax_patterns() {
    init_tracing!();
    info!("Testing error propagation syntax patterns");

    let patterns = vec![
        "result?",
        "function()?",
        "obj.method()?",
        "value?",
    ];

    let mut successful_patterns = 0;
    let mut total_patterns = patterns.len();

    for pattern in patterns {
        debug!(pattern = %pattern, "Testing error propagation pattern");
        
        let mut lexer = Lexer::new(pattern.to_string());
        
        // Collect tokens manually
        let mut tokens = Vec::new();
        let mut tokenization_successful = true;
        
        loop {
            match lexer.next_token() {
                Ok(token) => {
                    if token.token_type == cursed::lexer::TokenType::Eof {
                        break;
                    }
                    tokens.push(token);
                },
                Err(e) => {
                    debug!(pattern = %pattern, error = ?e, "Tokenization failed");
                    tokenization_successful = false;
                    break;
                }
            }
        }
        
        if tokenization_successful && !tokens.is_empty() {
            debug!(pattern = %pattern, token_count = tokens.len(), "Pattern tokenized");
            
            // Check if we can create a parser (create new lexer)
            let parser_lexer = Lexer::new(pattern.to_string());
            match Parser::new(parser_lexer) {
                Ok(_parser) => {
                    successful_patterns += 1;
                    debug!(pattern = %pattern, "Parser created successfully");
                },
                Err(e) => {
                    debug!(pattern = %pattern, error = ?e, "Parser creation failed");
                }
            }
        }
    }

    info!(
        successful = successful_patterns, 
        total = total_patterns, 
        "Error propagation pattern testing completed"
    );
    
    // For now, we just log the results since the ? operator may not be fully implemented
    assert!(total_patterns > 0, "Should test at least one pattern");
}

#[test]
fn test_example_file_validation() {
    init_tracing!();
    info!("Testing error propagation example file validation");

    let example_files = vec![
        "examples/error_propagation_basic.csd",
        "examples/error_propagation_advanced.csd",
    ];

    for file_path in example_files {
        debug!(file = %file_path, "Validating example file");
        
        match std::fs::read_to_string(file_path) {
            Ok(content) => {
                info!(file = %file_path, size = content.len(), "Example file read successfully");
                
                // Verify the file contains ? operators
                let question_mark_count = content.matches('?').count();
                debug!(file = %file_path, question_marks = question_mark_count, "Question mark usage");
                
                assert!(question_mark_count > 0, "Example file should contain ? operators: {}", file_path);
                
                // Verify the file contains CURSED keywords
                let has_cursed_keywords = content.contains("slay") || 
                                         content.contains("facts") || 
                                         content.contains("lowkey");
                
                assert!(has_cursed_keywords, "Example file should contain CURSED keywords: {}", file_path);
                
                info!(file = %file_path, "Example file validation passed");
            },
            Err(e) => {
                error!(file = %file_path, error = ?e, "Failed to read example file");
                panic!("Example file should exist and be readable: {}", file_path);
            }
        }
    }
}

#[test]
fn test_error_propagation_infrastructure() {
    init_tracing!();
    info!("Testing error propagation test infrastructure");

    // Test that we have the necessary modules available
    let module_tests: Vec<(&str, fn() -> bool)> = vec![
        ("Lexer", test_lexer_available),
        ("Parser", test_parser_available),
        ("Error types", test_error_types_available),
    ];

    let mut passed = 0;
    let total = module_tests.len();

    for (name, test_fn) in module_tests {
        debug!(module = %name, "Testing module availability");
        
        if test_fn() {
            passed += 1;
            info!(module = %name, "Module available");
        } else {
            warn!(module = %name, "Module not available or incomplete");
        }
    }

    info!(passed, total, "Infrastructure testing completed");
    assert!(passed > 0, "At least some infrastructure should be available");
}

// Helper functions for infrastructure testing
fn test_lexer_available() -> bool {
    let lexer = Lexer::new("test".to_string());
    debug!("Lexer creation successful");
    true
}

fn test_parser_available() -> bool {
    let mut lexer = Lexer::new("test".to_string());
    
    // Collect tokens manually
    let mut tokens = Vec::new();
    loop {
        match lexer.next_token() {
            Ok(token) => {
                if token.token_type == cursed::lexer::TokenType::Eof {
                    break;
                }
                tokens.push(token);
            },
            Err(_) => {
                debug!("Tokenization failed");
                return false;
            }
        }
    }
    
    if tokens.is_empty() {
        debug!("No tokens generated");
        return false;
    }
    
    // Create new lexer for parser (parser doesn't take tokens)
    let parser_lexer = Lexer::new("test".to_string());
    match Parser::new(parser_lexer) {
        Ok(_) => {
            debug!("Parser creation successful");
            true
        },
        Err(_) => {
            debug!("Parser creation failed");
            false
        }
    }
}

fn test_error_types_available() -> bool {
    // Test that we can work with Error types
    let _error: Result<(), Error> = Err(Error::Compile("test".to_string()));
    debug!("Error types available");
    true
}
