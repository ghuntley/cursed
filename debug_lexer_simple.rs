use cursed::lexer::{Lexer, TokenKind};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = if args.len() > 1 {
        // Read from file
        let filename = &args[1];
        fs::read_to_string(filename).unwrap_or_else(|_| {
            eprintln!("Error: Could not read file {}", filename);
            std::process::exit(1);
        })
    } else {
        // Default content
        "vibe main

slay main() {
    sus x = 42;
    yolo x;
}".to_string()
    };

    println!("=== ANALYZING CONTENT ===");
    for (i, line) in content.lines().enumerate() {
        println!("{:2}: {}", i + 1, line);
    }
    println!();

    println!("=== TOKEN ANALYSIS ===");
    let mut lexer = Lexer::new(content);
    let mut token_count = 0;
    
    loop {
        let token = match lexer.next_token() {
            Ok(token) => token,
            Err(e) => {
                println!("❌ Lexer error: {:?}", e);
                break;
            }
        };
        
        token_count += 1;
        println!("#{:2} {:?} '{}' at line {}, col {}", 
                 token_count, token.kind, token.lexeme, token.line, token.column);
        
        if matches!(token.kind, TokenKind::Eof) {
            break;
        }
    }
}
