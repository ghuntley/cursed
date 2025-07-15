use cursed::lexer::{Lexer, TokenKind};

fn main() {
    let code = "sus i normie = 42\nvibez.spill(i)";
    println!("Code: {}", code);
    
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token().unwrap();
        println!("Token: {:?}", token);
        if token.kind == TokenKind::Eof {
            break;
        }
        tokens.push(token);
    }
}
