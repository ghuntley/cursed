#!/usr/bin/env rust-script

//! Debug script to isolate the function parsing issue

use cursed::{Lexer, Parser, ast::*};

fn main() {
    println!("🔍 Testing function parsing issue...");

    // Test 1: Simple function (should work)
    let simple_func = r#"
slay simple() {
    sus x = 1
}
"#;
    test_parse("Simple function", simple_func);

    // Test 2: Function with if statement (the problematic case)
    let func_with_if = r#"
slay testFunc() {
    lowkey true {
        sus x = 1
    } highkey {
        sus y = 2
    }
}
"#;
    test_parse("Function with if", func_with_if);

    // Test 3: The exact demonstrateBasics function
    let demonstrate_basics = r#"
slay demonstrateBasics() {
    sus radius = 5.0
    sus userName = "Developer"
    sus isAwesome = based
    
    lowkey isAwesome {
        sus x = 1
    } highkey {
        sus y = 2
    }
}
"#;
    test_parse("demonstrateBasics function", demonstrate_basics);
}

fn test_parse(name: &str, code: &str) {
    println!("\n🧪 Testing: {}", name);
    println!("📝 Code:\n{}", code);
    
    // Tokenize
    let mut lexer = Lexer::new(code.to_string());
    let tokens = match lexer.tokenize() {
        Ok(tokens) => {
            println!("✅ Tokenized successfully: {} tokens", tokens.len());
            tokens
        },
        Err(e) => {
            println!("❌ Tokenization failed: {}", e);
            return;
        }
    };
    
    // Parse
    let mut parser = Parser::from_tokens(tokens);
    let program = match parser.parse() {
        Ok(program) => {
            println!("✅ Parsed successfully!");
            program
        },
        Err(e) => {
            println!("❌ Parsing failed: {}", e);
            let errors = parser.errors();
            if !errors.is_empty() {
                println!("Parser errors:");
                for error in errors {
                    println!("  - {}", error);
                }
            }
            return;
        }
    };
    
    // Analyze results
    println!("📊 Analysis:");
    println!("  - Statements: {}", program.statements.len());
    for (i, stmt) in program.statements.iter().enumerate() {
        match stmt {
            Statement::Function(func) => {
                println!("  - Statement {}: Function '{}' with {} statements in body", 
                         i, func.name, func.body.len());
            },
            Statement::Expression(_) => {
                println!("  - Statement {}: Expression", i);
            },
            _ => {
                println!("  - Statement {}: {:?}", i, std::mem::discriminant(stmt));
            }
        }
    }
}
