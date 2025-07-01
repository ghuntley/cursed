//! Standalone demo parser test - no external dependencies

use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::fs;

fn main() {
    println!("🚀 CURSED Demo Parser Test");
    println!("==========================\n");
    
    // Read the demo file
    let demo_content = match fs::read_to_string("demo_cursed_hello.csd") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Failed to read demo file: {}", e);
            std::process::exit(1);
        }
    };
    
    println!("📄 Demo file content:");
    println!("{}", demo_content);
    println!("\n{}\n", "=".repeat(50));
    
    // Step 1: Tokenize
    println!("🔍 Step 1: Tokenizing...");
    let mut lexer = Lexer::new(demo_content);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => {
            println!("✅ Tokenization successful! Found {} tokens", tokens.len());
            
            // Show first 20 tokens for brevity
            for (i, token) in tokens.iter().take(20).enumerate() {
                println!("  Token {}: {:?} = '{}'", i, token.kind, token.lexeme);
            }
            if tokens.len() > 20 {
                println!("  ... and {} more tokens", tokens.len() - 20);
            }
            tokens
        },
        Err(e) => {
            eprintln!("❌ Tokenization failed: {}", e);
            std::process::exit(1);
        }
    };
    
    println!("\n{}\n", "=".repeat(50));
    
    // Step 2: Parse
    println!("🔍 Step 2: Parsing...");
    let mut parser = Parser::from_tokens(tokens);
    let program = match parser.parse() {
        Ok(program) => {
            println!("✅ Parsing successful!");
            program
        },
        Err(e) => {
            eprintln!("❌ Parsing failed: {}", e);
            let errors = parser.errors();
            if !errors.is_empty() {
                eprintln!("Parser errors:");
                for error in errors {
                    eprintln!("  - {}", error);
                }
            }
            std::process::exit(1);
        }
    };
    
    println!("\n{}\n", "=".repeat(50));
    
    // Step 3: Analyze results
    println!("🔍 Step 3: Analyzing parsed program...");
    
    // Check package
    if let Some(package) = &program.package {
        println!("✅ Package declaration: {}", package.name);
    } else {
        println!("⚠️ No package declaration found");
    }
    
    // Check imports
    println!("📤 Found {} imports:", program.imports.len());
    for import in &program.imports {
        println!("  - Import: {}", import.path);
    }
    
    // Check statements
    println!("📋 Found {} statements:", program.statements.len());
    
    let mut function_count = 0;
    let mut let_count = 0;
    let mut other_count = 0;
    
    for (i, statement) in program.statements.iter().enumerate() {
        match statement {
            cursed::ast::Statement::Function(func) => {
                function_count += 1;
                println!("  Statement {}: Function '{}' with {} parameters", 
                        i, func.name, func.parameters.len());
                
                // Show function body summary
                println!("    Body has {} statements", func.body.len());
                let mut func_let_count = 0;
                let mut func_call_count = 0;
                let mut func_if_count = 0;
                let mut func_return_count = 0;
                
                for body_stmt in &func.body {
                    match body_stmt {
                        cursed::ast::Statement::Let(_) => func_let_count += 1,
                        cursed::ast::Statement::Expression(cursed::ast::Expression::Call(_)) => func_call_count += 1,
                        cursed::ast::Statement::If(_) => func_if_count += 1,
                        cursed::ast::Statement::Return(_) => func_return_count += 1,
                        _ => {},
                    }
                }
                
                println!("      Let statements: {}, Calls: {}, If statements: {}, Returns: {}", 
                        func_let_count, func_call_count, func_if_count, func_return_count);
            },
            cursed::ast::Statement::Let(let_stmt) => {
                let_count += 1;
                println!("  Statement {}: Let variable '{}'", i, let_stmt.name);
            },
            _ => {
                other_count += 1;
                println!("  Statement {}: Other ({:?})", i, std::mem::discriminant(statement));
            }
        }
    }
    
    println!("\n📊 Summary:");
    println!("  Functions: {}", function_count);
    println!("  Let statements: {}", let_count);
    println!("  Other statements: {}", other_count);
    println!("  Total statements: {}", program.statements.len());
    
    // Expected counts for validation
    let expected_functions = 4; // main, calculateArea, greetUser, demonstrateBasics
    let success = function_count == expected_functions;
    
    println!("\n🎯 Validation:");
    if success {
        println!("✅ SUCCESS: Found expected {} functions", expected_functions);
        println!("🎉 Demo program parsed successfully!");
    } else {
        println!("⚠️ PARTIAL SUCCESS: Found {} functions, expected {}", function_count, expected_functions);
        println!("🔧 Some parsing issues may need fixing");
    }
    
    println!("\n{}", "=".repeat(50));
    println!("Demo parser test completed");
}
