use cursed::{Lexer, Parser, ast::*};

fn main() {
    println!("🔍 Debugging type parsing issue...");
    
    // Test the specific problematic line
    let type_code = r#"sus isAwesome lit = based"#;
    
    println!("Testing type parsing: {}", type_code);
    let mut lexer = Lexer::new(type_code.to_string());
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
    
    // Also test with a simple type
    let simple_type_code = r#"sus x normie = 42"#;
    
    println!("\n\nTesting simple type parsing: {}", simple_type_code);
    let mut lexer = Lexer::new(simple_type_code.to_string());
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
}
