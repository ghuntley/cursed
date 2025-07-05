use cursed::{Lexer, Parser, ast::*};

fn main() {
    env_logger::init();
    
    println!("🔍 Testing minimal colon error case...");
    
    // Test just the tokens that seem to be causing the issue
    let test_cases = vec![
        ("Expression", "isAwesome"),
        ("If condition", "lowkey isAwesome"),
        ("If with braces", "lowkey isAwesome { }"),
        ("If with member access", "lowkey isAwesome { vibez.spill() }"),
        ("Member access alone", "vibez.spill()"),
    ];
    
    for (name, code) in test_cases {
        println!("\n🧪 Testing: {} = '{}'", name, code);
        
        let mut lexer = Lexer::new(code.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        println!("Tokens:");
        for (i, token) in tokens.iter().enumerate() {
            println!("  {}: {:?} = '{}'", i, token.kind, token.lexeme);
        }
        
        let mut parser = Parser::from_tokens(tokens);
        match parser.parse() {
            Ok(program) => {
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
