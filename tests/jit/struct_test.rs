use cursed::ast::*;
use cursed::lexer::Lexer;
use cursed::parser::Parser;


#[test]
fn test_parse_squad() {
    let input = r#"
    be_like Person squad {
        name tea
        age normie
        isActive lit
        height snack
    }
    "#;

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    
    let program = parser.parse_program().unwrap();
    
    // Check that we have one statement
    assert_eq!(program.statements.len(), 1, "Program should have 1 statement");
    
    // Check that the statement is a squad statement
    let squad_stmt = program.statements[0].as_any().downcast_ref::<SquadStatement>();
    assert!(squad_stmt.is_some(), "Statement is not a SquadStatement");
    
    // Get the squad statement
    let squad = squad_stmt.unwrap();
    
    // Check squad name
    assert_eq!(squad.name.value, "Person", "Squad name should be 'Person'");
    
    // Check number of fields
    assert_eq!(squad.fields.len(), 4, "Squad should have 4 fields");
    
    // Check field names and types
    assert_eq!(squad.fields[0].name.value, "name", "First field should be 'name'");
    assert_eq!(squad.fields[0].type_name.value, "tea", "First field type should be 'tea'");
    
    assert_eq!(squad.fields[1].name.value, "age", "Second field should be 'age'");
    assert_eq!(squad.fields[1].type_name.value, "normie", "Second field type should be 'normie'");
    
    assert_eq!(squad.fields[2].name.value, "isActive", "Third field should be 'isActive'");
    assert_eq!(squad.fields[2].type_name.value, "lit", "Third field type should be 'lit'");
    
    assert_eq!(squad.fields[3].name.value, "height", "Fourth field should be 'height'");
    assert_eq!(squad.fields[3].type_name.value, "snack", "Fourth field type should be 'snack'");
}

// Test to ensure we can parse the struct initialization syntax
#[test]
fn test_struct_initialization() {
    let input = r#"
    vibe test_squad;
    
    be_like Person squad {
        name tea
        age normie
    }
    
    slay main() {
        sus p = Person{name: "John", age: 30};
    }
    "#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    
    let program_result = parser.parse_program();
    
    // This should pass if struct initialization works
    assert!(program_result.is_ok(), "Failed to parse program with struct initialization");
}