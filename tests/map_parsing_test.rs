use cursed::lexer::Lexer;
use cursed::parser::Parser;

// Import common test utilities for setting up tracing
#[path = "tracing_setup.rs"]
mod tracing_setup;

#[test]
fn test_map_type_parsing() {
    tracing_setup::init_test_tracing();
    let input = "tea[tea]thicc";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).expect("Parser should initialize");
    
    let map_type = parser.parse_type().expect("Should parse map type");
    println!("Parsed map type: {:?}", map_type);
    
    // Verify the map type structure
    match map_type {
        cursed::core::type_checker::Type::Map(key_type, value_type) => {
            println!("Successfully parsed map with key type: {:?}, value type: {:?}", key_type, value_type);
        },
        _ => panic!("Expected map type, got: {:?}", map_type),
    }
}

#[test]
fn test_map_literal_parsing() {
    tracing_setup::init_test_tracing();
    let input = "tea[tea]thicc{\"key1\": 1, \"key2\": 2}";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).expect("Parser should initialize");
    
    // Parse as a program to get the full expression
    let program = parser.parse_program().expect("Should parse map literal program");
    
    // Check the program has statements and get the first expression statement
    assert!(!program.statements.is_empty(), "Program should have statements");
    
    let first_stmt = &program.statements[0];
    println!("Parsed first statement: {}", first_stmt.string());
}

#[test]
fn test_map_indexing_parsing() {
    tracing_setup::init_test_tracing();
    let input = "myMap[\"key\"]";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).expect("Parser should initialize");
    
    // Parse as a program to get the full expression
    let program = parser.parse_program().expect("Should parse map indexing program");
    
    // Check the program has statements and get the first expression statement
    assert!(!program.statements.is_empty(), "Program should have statements");
    
    let first_stmt = &program.statements[0];
    println!("Parsed map indexing: {}", first_stmt.string());
}

#[test]
fn test_empty_map_literal() {
    tracing_setup::init_test_tracing();
    let input = "tea[tea]thicc{}";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).expect("Parser should initialize");
    
    // Parse as a program to get the full expression
    let program = parser.parse_program().expect("Should parse empty map literal program");
    
    // Check the program has statements and get the first expression statement
    assert!(!program.statements.is_empty(), "Program should have statements");
    
    let first_stmt = &program.statements[0];
    println!("Parsed empty map literal: {}", first_stmt.string());
}
