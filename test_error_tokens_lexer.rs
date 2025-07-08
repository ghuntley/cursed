use cursed::lexer::{Lexer, TokenKind};

fn main() {
    let input = "yikes shook fam";
    let mut lexer = Lexer::new(input.to_string());
    
    let tokens = lexer.tokenize().unwrap();
    
    println!("Testing error handling tokens:");
    for token in &tokens {
        match token.kind {
            TokenKind::Yikes => println!("✅ Found Yikes token: {}", token.lexeme),
            TokenKind::Shook => println!("✅ Found Shook token: {}", token.lexeme),
            TokenKind::Fam => println!("✅ Found Fam token: {}", token.lexeme),
            TokenKind::Eof => println!("✅ Found EOF token"),
            _ => println!("ℹ️  Other token: {} -> {:?}", token.lexeme, token.kind),
        }
    }
    
    // Verify we have the expected tokens
    assert_eq!(tokens.len(), 4); // 3 error tokens + EOF
    assert_eq!(tokens[0].kind, TokenKind::Yikes);
    assert_eq!(tokens[1].kind, TokenKind::Shook);
    assert_eq!(tokens[2].kind, TokenKind::Fam);
    assert_eq!(tokens[3].kind, TokenKind::Eof);
    
    println!("🎉 All error handling tokens are working correctly!");
}
