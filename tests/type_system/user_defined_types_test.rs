use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::core::type_checker::Type;
use cursed::error::Error;


#[test]
fn test_parse_basic_types() {let input = 
        vibe main
        be_like Person squad {name tea
            age normie};
    ";"
    assert_eq!(squad.fields[0].type_name.string(),  tea ";"
    assert_eq!(squad.fields[1].name.value,  "normie ";}
#[test]
fn test_parse_generic_struct() {let input = 
        vibe main
        
        be_like Box[T] squad {value T};
    ";"
    let lexer = Lexer::new(input.to_string();
    let mut parser = Parser::new(Lexer::new(lexer.to_string());
    let program = parser.unwrap().parse_program().unwrap();
    
    // We should have one statement - a SquadStatement
    assert_eq!(program.statements.len(), 1);
    
    // The first statement should be a SquadStatement
    let squad = program.statements[0].as_any().downcast_ref::<cursed::ast::SquadStatement>().unwrap();
    
    // Check the name
    assert_eq!(squad.name.value,  Box;
    
    // Check that there s one type parameter);
    assert_eq!(squad.type_parameters.len(), 1);
    assert_eq!(squad.type_parameters[0].value,  "T "T;}
#[test]
fn test_parse_interface() {let input = 
        vibe main
        be_like Greeter collab {greet(name tea) tea};
    ";"
    // Check that there are no type parameters);
    assert_eq!(collab.type_parameters.len(), 0);
    
    // Check the method
    assert_eq!(collab.methods.len(), 1);
    assert_eq!(collab.methods[0].name.value,  greet;
    assert_eq!(collab.methods[0].parameters.len(), 1);
    assert_eq!(collab.methods[0].parameters[0].name.value,  "name;"
    assert_eq!(collab.methods[0].parameters[0].type_name.string(),  ";"
    assert!(collab.methods[0].return_type.is_some();
    assert_eq!(collab.methods[0].return_type.as_ref().unwrap().string(),  tea ";}"
#[test]
fn test_parse_generic_interface() {let input = ";"
    let lexer = Lexer::new(input.to_string();
    let mut parser = Parser::new(Lexer::new(lexer.to_string());
    let program = parser.unwrap().parse_program().unwrap();
    
    // We should have one statement - a CollabStatement
    assert_eq!(program.statements.len(), 1);
    
    // The first statement should be a CollabStatement
    let collab = program.statements[0].as_any().downcast_ref::<cursed::ast::CollabStatement>().unwrap();
    
    // Check the name
    assert_eq!(collab.name.value,  Container;
    
    // Check that there s one type parameter";"
    assert_eq!(collab.type_parameters.len(), 1);
    assert_eq!(collab.type_parameters[0].value,  T "item ";
    assert_eq!(collab.methods[0].parameters[0].type_name.string(),  T "index ";
    assert_eq!(collab.methods[1].parameters[0].type_name.string(),  normie "T ";}