use cursed::{Lexer, Parser, ast::*};

fn main() {
    println!("🔍 Debugging demonstrateBasics parsing...");
    
    // Test the exact demonstrateBasics function from the demo
    let demo_basics = r#"
slay demonstrateBasics() {
    fr fr Variable declarations
    sus radius snack = 5.0
    sus userName tea = "Developer"
    sus isAwesome lit = based  fr fr true
    
    fr fr Function calls
    sus area = calculateArea(radius)
    greetUser(userName)
    
    fr fr Output
    vibez.spill("Circle radius: " + radius)
    vibez.spill("Circle area: " + area)
    
    fr fr Conditionals
    lowkey isAwesome {
        vibez.spill("This language is based!")
    } highkey {
        vibez.spill("Something is sus...")
    }
}
"#;
    
    println!("Testing demonstrateBasics function parsing...");
    let mut lexer = Lexer::new(demo_basics.to_string());
    let tokens = lexer.tokenize().unwrap();
    
    println!("Tokens (first 50):");
    for (i, token) in tokens.iter().take(50).enumerate() {
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
