use crate::error::{Error, ErrorReporter};
use crate::lexer::{Lexer, Token};

/// Debug function to print all tokens in a file
pub fn debug_tokens(input: &str) -> Result<(), Error> {
    let mut lexer = Lexer::new(input);
    println!("TOKEN STREAM DUMP:");
    
    loop {
        let token = lexer.next_token()?;
        println!("Token: {:?}", token);
        
        if token == Token::Eof {
            break;
        }
    }
    
    Ok(())
}