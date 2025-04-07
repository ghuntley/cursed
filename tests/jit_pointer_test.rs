use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::ast::Program;
use cursed::prelude::*;

// Test pointer types and operations in the parser
#[test]
fn test_pointer_parsing() -> Result<(), Error> {
    let input = r#"
    slay test_pointers() normie {
        sus x normie = 42;
        sus ptr @normie = @x;
        sus y normie = @ptr;
        
        lowkey y == 42 {
            yolo 1;
        }
        
        yolo 0;
    }
    "#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;
    
    // Verify there are no parser errors
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors());
    }
    
    assert_eq!(program.statements.len(), 1, "Program should have one statement");
    
    // Test functionality will be implemented in the JIT integration tests
    Ok(())
}

// Test dereferencing pointers
#[test]
fn test_pointer_dereference() -> Result<(), Error> {
    let input = r#"
    slay test_deref() normie {
        sus x normie = 42;
        sus ptr @normie = @x;
        @ptr = 100;
        
        lowkey x == 100 {
            yolo 1;
        }
        
        yolo 0;
    }
    "#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;
    
    // Verify there are no parser errors
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors());
    }
    
    assert_eq!(program.statements.len(), 1, "Program should have one statement");
    
    // Test functionality will be implemented in the JIT integration tests
    Ok(())
}

// Test pointers with structs
#[test]
fn test_pointer_struct() -> Result<(), Error> {
    let input = r#"
    be_like Person squad {
        name tea
        age normie
    }
    
    slay test_struct_ptr() normie {
        sus person Person = Person{name: "John", age: 30};
        sus person_ptr @Person = @person;
        @person_ptr.age = 31;
        
        lowkey person.age == 31 {
            yolo 1;
        }
        
        yolo 0;
    }
    "#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;
    
    // Verify there are no parser errors
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors());
    }
    
    assert_eq!(program.statements.len(), 2, "Program should have two statements");
    
    // Test functionality will be implemented in the JIT integration tests
    Ok(())
}