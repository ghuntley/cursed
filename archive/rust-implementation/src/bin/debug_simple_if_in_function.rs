use cursed::{Lexer, Parser, ast::*};

fn main() {
    env_logger::init(); // Initialize logging
    
    println!("🔍 Debugging simple if in function...");
    
    // Test full case from demo
    let simple_if_func = r#"
slay demonstrateBasics() {
    sus isAwesome lit = based
    lowkey isAwesome {
        vibez.spill("This language is based!")
    } highkey {
        vibez.spill("Something is sus...")
    }
}
"#;
    
    println!("Testing simple if in function:");
    println!("{}", simple_if_func);
    
    let mut lexer = Lexer::new(simple_if_func.to_string());
    let tokens = lexer.tokenize().unwrap();
    
    let mut parser = Parser::from_tokens(tokens);
    match parser.parse() {
        Ok(ast) => {
            let program = match ast {
                cursed::ast::Ast::Program(program) => program,
                _ => {
                    println!("❌ Expected Program AST node");
                    return;
                }
            };
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
