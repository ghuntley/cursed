use cursed::{Lexer, Parser, ast::*};

fn main() {
    env_logger::init();
    
    println!("🔍 Testing member access in different contexts...");
    
    let test_cases = vec![
        ("Top level", "vibez.spill()"),
        ("In function", "slay test() { vibez.spill() }"),
        ("In if statement", "lowkey true { vibez.spill() }"),
        ("In function with if", "slay test() { lowkey true { vibez.spill() } }"),
    ];
    
    for (name, code) in test_cases {
        println!("\n🧪 Testing: {}", name);
        println!("Code: {}", code);
        
        let mut lexer = Lexer::new(code.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        let mut parser = Parser::from_tokens(tokens);
        match parser.parse() {
            Ok(ast) => {
                let program = match ast {
                    cursed::ast::Ast::Program(program) => program,
                    _ => {
                        println!("❌ Expected Program AST node");
                        continue;
                    }
                };
                println!("✅ Parse successful! {} statements", program.statements.len());
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
}
