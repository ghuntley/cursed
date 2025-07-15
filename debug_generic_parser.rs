use cursed::lexer::Lexer;
use cursed::parser::generic_parser::GenericParser;

fn main() {
    let source = "slay max<T>(a T, b T) -> T { damn a }";
    println!("Source: {}", source);
    
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.tokenize().unwrap();
    
    println!("Tokens:");
    for token in &tokens {
        println!("  {:?}", token);
    }
    
    let mut parser = GenericParser::new(&tokens);
    let result = parser.parse_generic_function();
    match result {
        Ok(func) => println!("Success: {:?}", func),
        Err(e) => println!("Error: {:?}", e),
    }
}
