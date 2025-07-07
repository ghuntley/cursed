use cursed::lexer::Lexer;
use cursed::parser::Parser;

fn main() {
    let code = "bestie i := 0; i < 5; i++";
    
    println!("Analyzing code: {}", code);
    
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    
    println!("Tokens:");
    for (i, token) in tokens.iter().enumerate() {
        println!("  {}: {:?}", i, token);
    }
    
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("Parse successful: {:#?}", program);
        }
        Err(e) => {
            println!("Parse error: {}", e);
            println!("Current token index: {}", parser.current);
        }
    }
}
