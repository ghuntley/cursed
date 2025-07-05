use cursed::{Lexer, Parser, ast::*};

fn main() {
    println!("🔍 Debugging if statement error in function...");
    
    // Test just the if statement that's causing issues
    let problematic_code = r#"
slay demonstrateBasics() {
    lowkey isAwesome {
        vibez.spill("This language is based!")
    } highkey {
        vibez.spill("Something is sus...")
    }
}
"#;
    
    println!("Testing problematic code:");
    println!("{}", problematic_code);
    
    let mut lexer = Lexer::new(problematic_code.to_string());
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
