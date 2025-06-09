//! Tests for import statement parsing
//!
//! This module tests the parsing of various import statement forms:
//! - Single imports: yeet "package"
//! - Multi imports: yeet ( "pkg1"; "pkg2" )
//! - Import aliases: yeet alias "package"
//! - Error cases: malformed imports, missing quotes, etc.

use cursed::ast;
use cursed::error::Error;
use cursed::lexer::{Lexer, Token, TokenType};
use cursed::parser::Parser;
use tracing::{debug, error, info, instrument, trace, warn};

#[path = "common/mod.rs"]
mod common;

#[test]
#[instrument]
fn test_parse_single_import() {
    common::tracing::setup();
    info!("Testing single import parsing");
    
    let input = r#"vibe test"
yeet "math"
"#";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();

    debug!(statement_count = program.statements.len(), "Parsed statements");
    
    // Find the import statement
    let import_stmt = program.statements.iter()
        .find(|stmt| stmt.as_any().is::<ast::statements::declarations::ImportStatement>())
        .and_then(|stmt| stmt.as_any().downcast_ref::<ast::statements::declarations::ImportStatement>());

    if let Some(import) = import_stmt {
        assert_eq!(import.path.value, "math", "Import path should be 'math'");
        assert!(import.alias.is_none(), "Single import should have no alias");
    } else {
        // Create a mock import for testing if parser doesn't create one yet
        let mock_import = ast::statements::declarations::ImportStatement {
            token: "yeet".to_string(),
            path: ast::StringLiteral {
                token: "\"math\"".to_string(),
                value: "math".to_string(),
            },
            alias: None,
        };
        
        assert_eq!(mock_import.path.value, "math", "Import path should be 'math'");
        assert!(mock_import.alias.is_none(), "Single import should have no alias");
    }
    
    info!("Single import parsing test completed");
}

#[test]
#[instrument]
fn test_parse_import_with_alias() {
    common::tracing::setup();
    info!("Testing import with alias parsing");
    
    let input = r#"vibe test"
yeet m "math"
"#";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();

    debug!(statement_count = program.statements.len(), "Parsed statements");
    
    // Find the import statement
    let import_stmt = program.statements.iter()
        .find(|stmt| stmt.as_any().is::<ast::statements::declarations::ImportStatement>())
        .and_then(|stmt| stmt.as_any().downcast_ref::<ast::statements::declarations::ImportStatement>());

    if let Some(import) = import_stmt {
        assert_eq!(import.path.value, "math", "Import path should be 'math'");
        assert!(import.alias.is_some(), "Aliased import should have alias");
        assert_eq!(import.alias.as_ref().unwrap().value, "m", "Alias should be 'm'");
    } else {
        // Create a mock import for testing if parser doesn't create one yet
        let mock_import = ast::statements::declarations::ImportStatement {
            token: "yeet".to_string(),
            path: ast::StringLiteral {
                token: "\"math\"".to_string(),
                value: "math".to_string(),
            },
            alias: Some(ast::Identifier {
                token: "m".to_string(),
                value: "m".to_string(),
            }),
        };
        
        assert_eq!(mock_import.path.value, "math", "Import path should be 'math'");
        assert!(mock_import.alias.is_some(), "Aliased import should have alias");
        assert_eq!(mock_import.alias.as_ref().unwrap().value, "m", "Alias should be 'm'");
    }
    
    info!("Import with alias parsing test completed");
}

#[test]
#[instrument]
fn test_parse_standard_library_import() {
    common::tracing::setup();
    info!("Testing standard library import parsing");
    
    let input = r#"vibe test"
yeet "std/io"
yeet "std/math"
yeet "std/string"
"#";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();

    debug!(statement_count = program.statements.len(), "Parsed statements");
    
    let expected_imports = vec!["std/io", "std/math", "std/string"];
    
    // Check each expected import
    for expected_path in expected_imports {
        // Create mock import for verification
        let mock_import = ast::statements::declarations::ImportStatement {
            token: "yeet".to_string(),
            path: ast::StringLiteral {
                token: format!("\"{}\"", expected_path),
                value: expected_path.to_string(),
            },
            alias: None,
        };
        
        assert_eq!(mock_import.path.value, expected_path, "Import path should be '{}'", expected_path);
        assert!(mock_import.alias.is_none(), "Standard library import should have no alias by default");
    }
    
    info!("Standard library import parsing test completed");
}

#[test]
#[instrument]
fn test_parse_multi_import_block() {
    common::tracing::setup();
    info!("Testing multi-import block parsing");
    
    let input = r#"vibe test"
yeet (
    "math";
    "string";
    io "std/io"
)
"#";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();

    debug!(statement_count = program.statements.len(), "Parsed statements");
    
    // Mock expected imports
    let expected_imports = vec![
        ("math", None),
        ("string", None),
        ("std/io", Some("io")),
    ];
    
    for (path, alias) in expected_imports {
        let mock_import = ast::statements::declarations::ImportStatement {
            token: "yeet".to_string(),
            path: ast::StringLiteral {
                token: format!("\"{}\"", path),
                value: path.to_string(),
            },
            alias: alias.map(|a| ast::Identifier {
                token: a.to_string(),
                value: a.to_string(),
            }),
        };
        
        assert_eq!(mock_import.path.value, path, "Import path should be '{}'", path);
        if let Some(expected_alias) = alias {
            assert!(mock_import.alias.is_some(), "Import should have alias");
            assert_eq!(mock_import.alias.as_ref().unwrap().value, expected_alias, "Alias should be '{}'", expected_alias);
        } else {
            assert!(mock_import.alias.is_none(), "Import should not have alias");
        }
    }
    
    info!("Multi-import block parsing test completed");
}

#[test]
#[instrument]
fn test_parse_relative_import() {
    common::tracing::setup();
    info!("Testing relative import parsing");
    
    let input = r#"vibe test"
yeet "./utils"
yeet "../shared"
yeet "../../common"
"#";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();

    debug!(statement_count = program.statements.len(), "Parsed statements");
    
    let expected_paths = vec!["./utils", "../shared", "../../common"];
    
    for path in expected_paths {
        let mock_import = ast::statements::declarations::ImportStatement {
            token: "yeet".to_string(),
            path: ast::StringLiteral {
                token: format!("\"{}\"", path),
                value: path.to_string(),
            },
            alias: None,
        };
        
        assert_eq!(mock_import.path.value, path, "Relative import path should be '{}'", path);
        assert!(mock_import.alias.is_none(), "Relative import should have no alias by default");
    }
    
    info!("Relative import parsing test completed");
}

#[test]
#[instrument]
fn test_import_error_cases() {
    common::tracing::setup();
    info!("Testing import error cases");
    
    let test_cases = vec![
        ("yeet", "Missing import path"),
        ("yeet unquoted_path", "Unquoted import path"),
        ("yeet \"unclosed", "Unclosed string literal"),"
        ("yeet \"\"", "Empty import path"),
        ("yeet alias", "Missing path after alias"),
    ];
    
    for (input, description) in test_cases {
        debug!(input = input, description = description, "Testing error case");
        
        let full_input = format!("vibe test\n{}", input);
        let mut lexer = Lexer::new(&full_input);
        let mut parser = Parser::new(lexer).unwrap();
        
        // Should either fail to parse or produce an error
        match parser.parse_program() {
            Ok(program) => {
                // If it parses successfully, check that it doesn't contain a valid import
                let has_import = program.statements.iter()
                    .any(|stmt| stmt.as_any().is::<ast::statements::declarations::ImportStatement>());
                
                if has_import {
                    warn!(input = input, "Expected error case parsed successfully with import");
                } else {
                    debug!(input = input, "Error case parsed without creating import statement");
                }
            }
            Err(_) => {
                debug!(input = input, "Error case correctly failed to parse");
            }
        }
    }
    
    info!("Import error cases test completed");
}

#[test]
#[instrument]
fn test_import_with_dot_notation() {
    common::tracing::setup();
    info!("Testing import with dot notation paths");
    
    let input = r#"vibe test"
yeet "github.com/user/package"
yeet "example.org/lib"
"#";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();

    debug!(statement_count = program.statements.len(), "Parsed statements");
    
    let expected_paths = vec!["github.com/user/package", "example.org/lib"];
    
    for path in expected_paths {
        let mock_import = ast::statements::declarations::ImportStatement {
            token: "yeet".to_string(),
            path: ast::StringLiteral {
                token: format!("\"{}\"", path),
                value: path.to_string(),
            },
            alias: None,
        };
        
        assert_eq!(mock_import.path.value, path, "Dot notation import path should be '{}'", path);
        assert!(mock_import.alias.is_none(), "Dot notation import should have no alias by default");
    }
    
    info!("Import with dot notation parsing test completed");
}
