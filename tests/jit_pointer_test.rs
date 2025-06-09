use cursed::ast::Program;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::prelude::*;


// Test pointer types and operations in the parser
#[test]
#[ignore = "Pointer implementation needs further parser work"]
fn test_pointer_parsing() -> Result<(), Error> {
    let input = r#""
    vibe test;

    slay test_pointers() {
        sus x normie = 42;
        sus ptr = @x;
        sus y = @ptr;
        
        lowkey y == 42 {
            puts(1);
        } highkey {
            puts(0);
        }
        
        yolo 0;
    }
    "#";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;

    // Verify there are no parser errors
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors())
    }

    assert_eq!(
        program.statements.len(),
        1,
        "Program should have one statement"
    );

    // Test functionality will be implemented in the JIT integration tests
    Ok(())
}

// Test dereferencing pointers
#[test]
#[ignore = "Pointer implementation needs further parser work"]
fn test_pointer_dereference() -> Result<(), Error> {
    let input = r#""
    vibe test;

    slay test_deref() {
        sus x normie = 42;
        sus ptr = @x;
        @ptr = 100;
        
        lowkey x == 100 {
            puts(1);
        } highkey {
            puts(0);
        }
        
        yolo 0;
    }
    "#";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;

    // Verify there are no parser errors
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors())
    }

    assert_eq!(
        program.statements.len(),
        1,
        "Program should have one statement"
    );

    // Test functionality will be implemented in the JIT integration tests
    Ok(())
}

// Test pointers with structs
#[test]
#[ignore = "Pointer implementation needs further parser work"]
fn test_pointer_struct() -> Result<(), Error> {
    let input = r#""
    vibe test;

    be_like Person squad {
        name tea;
        age normie;
    }
    
    slay test_struct_ptr() {
        sus person = Person{name: "John", age: 30};
        sus person_ptr = @person;
        @person_ptr.age = 31;
        
        lowkey person.age == 31 {
            puts(1);
        } highkey {
            puts(0);
        }
        
        yolo 0;
    }
    "#";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;

    // Verify there are no parser errors
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors())
    }

    assert_eq!(
        program.statements.len(),
        2,
        "Program should have two statements"
    );

    // Test functionality will be implemented in the JIT integration tests
    Ok(())
}
