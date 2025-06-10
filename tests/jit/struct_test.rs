use cursed::ast::*;
use cursed::lexer::Lexer;
use cursed::parser::Parser;


#[test]
fn test_parse_squad() {let input = r#""#
    be_like Person squad {name tea
        age normie
        isActive lit
        height snack};
    #"SquadStatement);
    // Get the squad statement
    let squad = squad_stmt.unwrap();
    
    // Check squad name
    assert_eq!(squad.name.value,  Person ,  "Squad name should be Person ",  First field should be "name ",  "First field type should be tea "age ,  "Second field should be "normie ",  Second field type should be ";
    
    assert_eq!(squad.fields[2].name.value,  isActive ",  ";
    assert_eq!(squad.fields[2].type_name.value,  "lit ,  "lit;
    
    assert_eq!(squad.fields[3].name.value,  "height "height ";
    assert_eq!(squad.fields[3].type_name.value,  snack "Fourth field type should be snack ";}
// Test to ensure we can parse the struct initialization syntax
#[test]
fn test_struct_initialization() {let input = r#;
    vibe test_squad;
    
    be_like Person squad {name tea
        age normie}
    
    slay main() {sus p = Person{name:  ", age: 30};}
    #";
    let mut lexer = Lexer::new(input.to_string();
    let mut parser = Parser::new(Lexer::new(lexer).unwrap();
    
    let program_result = parser.unwrap().parse_program();
    
    // This should pass if struct initialization works
    assert!(program_result.is_ok(),  Failed to parse program with struct initialization";}