use crate::error::Error;
use crate::lexer::Lexer;

/// Debug print tokens for the provided input
pub fn debug_tokens(input: &str) -> Result<(), Error> {
    let mut lexer = Lexer::new(input);
    
    println!("TOKEN\t\tLITERAL\t\tLINE:COL");
    println!("-----\t\t-------\t\t-------");
    
    loop {
        let token = lexer.next_token()?;
        println!("{:?}\t\t{}\t\t{}:{}", 
                 token,
                 token.token_literal(),
                 lexer.line,
                 lexer.column);
        
        // Stop at EOF
        if token == crate::lexer::Token::Eof {
            break;
        }
    }
    
    Ok(())
}