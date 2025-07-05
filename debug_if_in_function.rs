use cursed::{Lexer, Parser};

fn main() {
    let code = include_str!("../debug_if_in_function.csd");
    println!("Code:\n{}", code);
    
    let mut lexer = Lexer::new(code.to_string());
    let tokens = lexer.tokenize().unwrap();
    
    println!("\nTokens:");
    for (i, token) in tokens.iter().enumerate() {
        println!("  {}: {:?} = '{}'", i, token.kind, token.lexeme);
    }
    
    let mut parser = Parser::from_tokens(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("\nParsed successfully!");
            println!("Statements: {}", program.statements.len());
            
            for (i, stmt) in program.statements.iter().enumerate() {
                println!("Statement {}: {:?}", i, std::mem::discriminant(stmt));
                
                if let cursed::ast::Statement::Function(func) = stmt {
                    println!("  Function '{}' with {} body statements", func.name, func.body.len());
                    for (j, body_stmt) in func.body.iter().enumerate() {
                        println!("    Body statement {}: {:?}", j, std::mem::discriminant(body_stmt));
                    }
                }
            }
        },
        Err(e) => {
            println!("\nParse error: {}", e);
            let errors = parser.errors();
            for error in errors {
                println!("  Error: {}", error);
            }
        }
    }
}
