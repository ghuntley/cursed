use cursed::lexer::{Lexer, TokenKind};

fn main() {
    let sources = vec![
        "slay",
        "max",
        "struct",
        "collab",
        "where",
        "Display",
        "T",
        "<",
        ">",
        ":",
        "+",
        "=",
        "slay max<T>(a T, b T) -> T { damn a }",
    ];
    
    for source in sources {
        println!("=== Testing: '{}' ===", source);
        let mut lexer = Lexer::new(source.to_string());
        match lexer.tokenize() {
            Ok(tokens) => {
                for token in tokens {
                    if token.kind != TokenKind::Eof {
                        println!("{:?} = '{}'", token.kind, token.lexeme);
                    }
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
        println!();
    }
}
