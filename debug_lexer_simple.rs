use cursed::lexer::{Lexer, TokenKind};

fn main() {
    let content = "vibe main

slay main() {
    sus x = 42;
    yolo x;
}";

    println!("=== ANALYZING CONTENT ===");
    for (i, line) in content.lines().enumerate() {
        println!("{:2}: {}", i + 1, line);
    }
    println!();

    println!("=== TOKEN ANALYSIS ===");
    let mut lexer = Lexer::new(content.to_string());
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
