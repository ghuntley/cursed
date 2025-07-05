use cursed::{Lexer, Parser, ast::*};

fn main() {
    env_logger::init();
    
    println!("🔍 Testing complex function with multiple statements...");
    
    let complex_func = r#"
slay demonstrateBasics() {
    sus radius snack = 5.0
    sus userName tea = "Developer"
    sus isAwesome lit = based
    
    sus area = calculateArea(radius)
    greetUser(userName)
    
    vibez.spill("Circle radius: " + radius)
    vibez.spill("Circle area: " + area)
    
    lowkey isAwesome {
        vibez.spill("This language is based!")
    } highkey {
        vibez.spill("Something is sus...")
    }
}
"#;
    
    println!("Code:\n{}", complex_func);
    
    let mut lexer = Lexer::new(complex_func.to_string());
    let tokens = lexer.tokenize().unwrap();
    
    let mut parser = Parser::from_tokens(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("✅ Parse successful! {} statements", program.statements.len());
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
        }
    }
    
    let errors = parser.errors();
    if !errors.is_empty() {
        println!("Parser errors:");
        for error in errors {
            println!("  - {}", error);
        }
    }
}
