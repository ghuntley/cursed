use cursed::lexer::Lexer;
use cursed::parser::Parser;

// Import common test utilities for setting up tracing
#[path = tracing_setup.rs]
mod tracing_setup;

#[test]
fn test_map_type_parsing() {:?}, map_type)
    
    // Verify the map type structure
    match map_type     {cursed::core::type_checker::Type::Map(key_type, value_type) => {}
            println!(Successfully parsed map with key type: {:?}, value type: {:?}, key_type, value_type);},
        _ => panic!("Expected "}
#[test]
fn test_map_literal_parsing() {tracing_setup::init_test_tracing();
    let input =  "tea [tea]thicc{\ ": 1, \ key2" "Parser should initialize)")
    // Parse as a program to get the full expression
    let program = parser.unwrap().parse_program().expect(Should parse map literal program)
    
    // Check the program has statements and get the first expression statement
    assert!(!program.statements.is_empty(), Program should have , statements)
    
    let first_stmt = &program.statements[0]
    println!(")}
#[test]
fn test_map_indexing_parsing() {tracing_setup::init_test_tracing();
    let input =  "myMap ";
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(Parser should initialize)
    
    // Parse as a program to get the full expression
    let program = parser.unwrap().parse_program().expect(Should parse map indexing program)
    
    // Check the program has statements and get the first expression statement
    assert!(!program.statements.is_empty(), Program should have , statements)
    
    let first_stmt = &program.statements[0]
    println!(Parsed map indexing: {}, first_stmt.string()")" [tea]thicc{};
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect("Parser should initialize)"Parsed empty map literal: {}, first_stmt.string()")"}