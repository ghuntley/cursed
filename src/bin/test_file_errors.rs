//! Test file-based error reporting

use cursed::error::{ErrorReporter, StructuredError, ErrorCode};
use cursed::error::structured::ErrorSourceLocation;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "test_error_demo.csd"
    };
    
    println!("🔥 CURSED File Error Testing: {}", filename);
    println!("==========================================");
    
    match fs::read_to_string(filename) {
        Ok(content) => {
            test_lexer_errors(filename, &content);
            test_parser_errors(filename, &content);
        }
        Err(e) => {
            println!("❌ Could not read file {}: {}", filename, e);
        }
    }
}

fn test_lexer_errors(filename: &str, content: &str) {
    println!("\n📝 Lexer Error Analysis:");
    println!("{}","-".repeat(30));
    
    let mut reporter = ErrorReporter::new();
    let mut lexer = Lexer::new(content.to_string());
    
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("✅ Lexer completed successfully with {} tokens", tokens.len());
        }
        Err(e) => {
            println!("❌ Lexer error: {}", e);
            
            // Convert basic error to structured error for demonstration
            let structured_error = StructuredError::from(e);
            reporter.add_error(structured_error);
        }
    }
    
    if reporter.has_errors() {
        println!("\n📊 Structured Error Report:");
        reporter.print_all();
    }
}

fn test_parser_errors(filename: &str, content: &str) {
    println!("\n🔍 Parser Error Analysis:");
    println!("{}","-".repeat(30));
    
    let mut reporter = ErrorReporter::new();
    let lexer = Lexer::new(content.to_string());
    
    match Parser::new(lexer) {
        Ok(mut parser) => {
            match parser.parse() {
                Ok(ast) => {
                    println!("✅ Parser completed successfully");
                    if let cursed::ast::Ast::Program(program) = ast {
                        println!("   Program has {} statements", program.statements.len());
                        if !program.imports.is_empty() {
                            println!("   Program has {} imports", program.imports.len());
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Parser error: {}", e);
                    
                    // Convert to structured error
                    let structured_error = match e {
                        cursed::error_types::Error::Parse(msg) => StructuredError::new(ErrorCode::E0001, msg),
                        cursed::error_types::Error::Type(msg) => StructuredError::new(ErrorCode::E0100, msg),
                        cursed::error_types::Error::Lexer(msg) => StructuredError::new(ErrorCode::E0001, msg),
                        _ => StructuredError::new(ErrorCode::E0001, e.to_string()),
                    };
                    reporter.add_error(structured_error);
                }
            }
            
            // Check for any accumulated parser errors
            if !parser.errors().is_empty() {
                println!("⚠️  Parser collected {} additional errors:", parser.errors().len());
                for error in parser.errors() {
                    println!("   • {}", error);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to create parser: {}", e);
        }
    }
    
    if reporter.has_errors() {
        println!("\n📊 Parser Error Report:");
        reporter.print_all();
    }
}
