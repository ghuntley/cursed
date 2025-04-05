use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::ast::{Statement, Expression};

#[test]
fn test_parse_generic_type_definition() {
    let input = "be_like Box[T] squad { value T }";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();
    
    assert_eq!(program.statements.len(), 1, "Program should have 1 statement");
    
    // Look for a SquadStatement
    let squad_stmt = program.statements[0].as_any().downcast_ref::<cursed::ast::SquadStatement>().unwrap();
    
    // Check the type parameters
    assert_eq!(squad_stmt.type_parameters.len(), 1, "Should have 1 type parameter");
    assert_eq!(squad_stmt.type_parameters[0].value, "T", "Type parameter should be 'T'");
    
    // Check the field uses the type parameter
    assert_eq!(squad_stmt.fields.len(), 1, "Should have 1 field");
    assert_eq!(squad_stmt.fields[0].name.value, "value", "Field name should be 'value'");
    assert_eq!(squad_stmt.fields[0].type_name.value, "T", "Field type should be 'T'");
}

#[test]
fn test_parse_multiple_generic_type_parameters() {
    let input = "be_like Pair[K, V] squad { key K value V }";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();
    
    assert_eq!(program.statements.len(), 1, "Program should have 1 statement");
    
    // Look for a SquadStatement
    let squad_stmt = program.statements[0].as_any().downcast_ref::<cursed::ast::SquadStatement>().unwrap();
    
    // Check the type parameters
    assert_eq!(squad_stmt.type_parameters.len(), 2, "Should have 2 type parameters");
    assert_eq!(squad_stmt.type_parameters[0].value, "K", "First type parameter should be 'K'");
    assert_eq!(squad_stmt.type_parameters[1].value, "V", "Second type parameter should be 'V'");
    
    // Check the fields use the type parameters
    assert_eq!(squad_stmt.fields.len(), 2, "Should have 2 fields");
    assert_eq!(squad_stmt.fields[0].name.value, "key", "First field name should be 'key'");
    assert_eq!(squad_stmt.fields[0].type_name.value, "K", "First field type should be 'K'");
    assert_eq!(squad_stmt.fields[1].name.value, "value", "Second field name should be 'value'");
    assert_eq!(squad_stmt.fields[1].type_name.value, "V", "Second field type should be 'V'");
}