use cursed::lexer::Lexer;
use cursed::parser::Parser;

fn main() {
    // Test parsing map type
    let map_type_input = "tea[tea]thicc";
    println!("Testing map type parsing: {}", map_type_input);
    
    let mut lexer = Lexer::new(map_type_input);
    let mut parser = Parser::new(lexer);
    
    match parser.parse_type() {
        Ok(typ) => println!("Successfully parsed map type: {:?}", typ),
        Err(e) => println!("Error parsing map type: {:?}", e),
    }
    
    // Test parsing map literal
    let map_literal_input = "tea[tea]thicc{\"key1\": 1, \"key2\": 2}";
    println!("\nTesting map literal parsing: {}", map_literal_input);
    
    let mut lexer = Lexer::new(map_literal_input);
    let mut parser = Parser::new(lexer);
    
    match parser.parse_expression(cursed::parser::precedence::Precedence::Lowest) {
        Ok(expr) => println!("Successfully parsed map literal: {}", expr.string()),
        Err(e) => println!("Error parsing map literal: {:?}", e),
    }
    
    // Test parsing map indexing
    let map_index_input = "myMap[\"key\"]";
    println!("\nTesting map indexing parsing: {}", map_index_input);
    
    let mut lexer = Lexer::new(map_index_input);
    let mut parser = Parser::new(lexer);
    
    match parser.parse_expression(cursed::parser::precedence::Precedence::Lowest) {
        Ok(expr) => println!("Successfully parsed map indexing: {}", expr.string()),
        Err(e) => println!("Error parsing map indexing: {:?}", e),
    }
}
