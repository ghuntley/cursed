use cursed::{Lexer, Parser, ast::*};

#[test]
fn test_debug_demo_exact() {
    // Read the exact same content as the demo test
    let demo_content = include_str!("../demo_cursed_hello.csd");
    
    println!("Testing actual demo file content...");
    
    // Step 1: Tokenize
    let mut lexer = Lexer::new(demo_content.to_string());
    let tokens = lexer.tokenize().expect("Tokenization failed");
    
    println!("Found {} tokens", tokens.len());
    
    // Print tokens around the demonstrateBasics function
    println!("\nTokens around demonstrateBasics function:");
    let mut found_greet_user = false;
    let mut skip_counter = 0;
    
    for (i, token) in tokens.iter().enumerate() {
        if token.lexeme == "greetUser" {
            found_greet_user = true;
        }
        
        if found_greet_user && skip_counter < 30 {
            println!("  Token {}: {:?} = '{}'", i, token.kind, token.lexeme);
            skip_counter += 1;
        }
    }

    // Step 2: Parse using the correct method
    let mut parser = Parser::from_tokens(tokens);
    let program = parser.parse_program().expect("Parsing failed");

    println!("\nParsed {} statements:", program.statements.len());
    for (i, stmt) in program.statements.iter().enumerate() {
        println!("  Statement {}: {:?}", i, std::mem::discriminant(stmt));
        match stmt {
            Statement::Function(func) => {
                println!("    Function: {} with {} parameters", func.name, func.parameters.len());
            },
            Statement::Expression(expr) => {
                println!("    Expression: {:?}", expr);
            },
            _ => {
                println!("    Other statement type");
            }
        }
    }
}
