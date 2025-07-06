use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <cursed_file>", args[0]);
        return;
    }

    let filename = &args[1];
    let content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            return;
        }
    };

    println!("=== FILE CONTENT ===");
    println!("{}", content);
    println!();

    // Create lexer and tokenize
    let mut lexer = cursed::lexer::Lexer::new(&content);
    
    println!("=== TOKENS ===");
    let mut token_count = 0;
    loop {
        let token = lexer.next_token();
        token_count += 1;
        println!("Token {}: {:?}", token_count, token);
        
        if matches!(token, cursed::lexer::Token::EOF) {
            break;
        }
    }
    
    println!();
    println!("=== PARSER ANALYSIS ===");
    
    // Create fresh lexer for parser
    let mut fresh_lexer = cursed::lexer::Lexer::new(&content);
    let mut parser = cursed::parser::Parser::new(&mut fresh_lexer);
    
    match parser.parse() {
        Ok(ast) => {
            println!("✓ Parsing successful!");
            println!("AST: {:?}", ast);
        },
        Err(e) => {
            println!("✗ Parsing failed: {:?}", e);
            
            // Try to identify where parsing failed
            println!("\n=== DETAILED ERROR ANALYSIS ===");
            
            // Get current token position
            let current_token = parser.current_token();
            println!("Current token when error occurred: {:?}", current_token);
            
            // Try to parse step by step
            let mut step_lexer = cursed::lexer::Lexer::new(&content);
            let mut step_parser = cursed::parser::Parser::new(&mut step_lexer);
            
            println!("\n=== STEP BY STEP PARSING ===");
            
            // Try parsing each statement individually
            let mut statement_count = 0;
            while !matches!(step_parser.current_token(), cursed::lexer::Token::EOF) {
                statement_count += 1;
                println!("Attempting to parse statement {}", statement_count);
                println!("Current token: {:?}", step_parser.current_token());
                
                match step_parser.parse_statement() {
                    Ok(stmt) => {
                        println!("✓ Statement {} parsed successfully: {:?}", statement_count, stmt);
                    },
                    Err(e) => {
                        println!("✗ Statement {} failed: {:?}", statement_count, e);
                        break;
                    }
                }
            }
        }
    }
}
