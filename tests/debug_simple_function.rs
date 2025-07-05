use cursed::{Lexer, Parser, ast::*};

#[test]
fn test_debug_simple_function() {
    // Test a very simple function
    let code = r#"
slay test() {
    sus x = 5
}
"#;

    println!("Testing simple function:");
    println!("{}", code);

    // Step 1: Tokenize
    let mut lexer = Lexer::new(code.to_string());
    let tokens = lexer.tokenize().expect("Tokenization failed");
    
    println!("\nTokens:");
    for (i, token) in tokens.iter().enumerate() {
        println!("  {}: {:?} = '{}'", i, token.kind, token.lexeme);
    }

    // Step 2: Parse
    let mut parser = Parser::from_tokens(tokens);
    let program = match parser.parse() {
        Ok(prog) => prog,
        Err(e) => {
            println!("Parse error: {}", e);
            let errors = parser.errors();
            for error in errors {
                println!("  Error: {}", error);
            }
            panic!("Parsing failed");
        }
    };

    println!("\nParsed {} statements:", program.statements.len());
    for (i, stmt) in program.statements.iter().enumerate() {
        println!("  Statement {}: {:?}", i, std::mem::discriminant(stmt));
        match stmt {
            Statement::Function(func) => {
                println!("    Function: {} with {} body statements", func.name, func.body.len());
            },
            Statement::Expression(expr) => {
                println!("    Expression: {:?}", expr);
            },
            _ => {}
        }
    }
}
