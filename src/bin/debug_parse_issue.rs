use cursed::{Lexer, Parser, ast::*};

fn main() {
    println!("🔍 Debugging parse issue...");
    
    // Test the simple if statement
    let if_code = r#"
lowkey true {
    sus x = 1
} highkey {
    sus y = 2
}
"#;
    
    println!("Testing if statement parsing...");
    let mut lexer = Lexer::new(if_code.to_string());
    let tokens = lexer.tokenize().unwrap();
    
    println!("Tokens:");
    for (i, token) in tokens.iter().enumerate() {
        println!("  {}: {:?} = '{}'", i, token.kind, token.lexeme);
    }
    
    let mut parser = Parser::from_tokens(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("✅ Parse successful!");
            println!("Statements: {}", program.statements.len());
            for (i, stmt) in program.statements.iter().enumerate() {
                println!("  Statement {}: {:?}", i, std::mem::discriminant(stmt));
            }
        },
        Err(e) => {
            println!("❌ Parse failed: {}", e);
            let errors = parser.errors();
            for error in errors {
                println!("  Error: {}", error);
            }
        }
    }
    
    // Test function with if statement
    let func_code = r#"
slay testFunc() {
    lowkey true {
        sus x = 1
    } highkey {
        sus y = 2
    }
}
"#;
    
    println!("\n\nTesting function with if statement...");
    let mut lexer = Lexer::new(func_code.to_string());
    let tokens = lexer.tokenize().unwrap();
    
    println!("Tokens:");
    for (i, token) in tokens.iter().enumerate() {
        println!("  {}: {:?} = '{}'", i, token.kind, token.lexeme);
    }
    
    let mut parser = Parser::from_tokens(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("✅ Parse successful!");
            println!("Statements: {}", program.statements.len());
            for (i, stmt) in program.statements.iter().enumerate() {
                match stmt {
                    Statement::Function(func) => {
                        println!("  Statement {}: Function '{}' with {} body statements", 
                                 i, func.name, func.body.len());
                        for (j, body_stmt) in func.body.iter().enumerate() {
                            println!("    Body Statement {}: {:?}", j, std::mem::discriminant(body_stmt));
                        }
                    },
                    _ => {
                        println!("  Statement {}: {:?}", i, std::mem::discriminant(stmt));
                    }
                }
            }
        },
        Err(e) => {
            println!("❌ Parse failed: {}", e);
            let errors = parser.errors();
            for error in errors {
                println!("  Error: {}", error);
            }
        }
    }
}
