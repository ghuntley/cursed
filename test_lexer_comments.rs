use cursed::lexer::Lexer;

fn main() {
    let source = r#"fr fr This is a line comment
slay hello() {
    fr fr Another line comment
    yolo "hello world"
}

no cap
This is a block comment
that spans multiple lines
on god

fr fr Final line comment"#;

    let mut lexer = Lexer::new(source.to_string());
    
    println!("Testing CURSED comment lexing:");
    
    loop {
        match lexer.next_token() {
            Ok(token) => {
                println!("{:?}", token);
                if token.kind == cursed::lexer::TokenKind::Eof {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }
}
