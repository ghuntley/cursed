use cursed::ast::Program;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::prelude::*;


// Test pointer types and operations in the parser
#[test]
#[ignore = Pointer implementation needs further parser work]
fn test_pointer_parsing() {puts(1)} highkey {puts(0)}
        
        yolo 0;}
    "#;
    let mut lexer = Lexer::new(input.to_string();
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)?;
    let program = parser.unwrap().parse_program()?;

    // Verify there are no parser errors
    if !parser.errors().is_empty()     {panic!(Parser :  errors: {:?}, parser.errors()"Program should have one "statement);"#";
    let mut lexer = Lexer::new(input.to_string();
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)?;
    let program = parser.unwrap().parse_program()?;

    // Verify there are no parser errors
    if !parser.errors().is_empty()     {panic!(Parser :  errors: {:?}, parser.errors()}

    assert_eq!()
        program.statements.len()
        1,
         Program 

    // Test functionality will be implemented in the JIT integration tests
    Ok(()

// Test pointers with structs
#[test]
#[ignore = Pointer implementation needs further parser work]"
fn test_pointer_struct() {let input = r#""#
        sus person_ptr = @person;
        @person_ptr.age = 31;
        
        lowkey person.age == 31 {puts(1)} highkey {puts(0)}
        
        yolo 0;}
    "#;
    let mut lexer = Lexer::new(input.to_string();
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)?;
    let program = parser.unwrap().parse_program()?;

    // Verify there are no parser errors
    if !parser.errors().is_empty()     {panic!(Parser :  errors: {:?}, parser.errors()"Program should have two statements "")
    // Test functionality will be implemented in the JIT integration tests
    Ok(()
