use cursed::{Lexer, Parser, ast::*};

#[test]
fn test_debug_parser_issue() {
    let code = r#"
slay calculateArea(radius snack) snack {
    yolo 3.14159 * radius * radius
}

slay demonstrateBasics() {
    sus x = 5
}
"#;

    println!("Testing parser with code:");
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
    let program = parser.parse().expect("Parsing failed");

    println!("\nParsed {} statements:", program.statements.len());
    for (i, stmt) in program.statements.iter().enumerate() {
        println!("  Statement {}: {:?}", i, std::mem::discriminant(stmt));
        match stmt {
            Statement::Function(func) => {
                println!("    Function: {}", func.name);
            },
            Statement::Expression(expr) => {
                println!("    Expression: {:?}", expr);
            },
            _ => {}
        }
    }
}
