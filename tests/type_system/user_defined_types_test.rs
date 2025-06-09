use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::core::type_checker::Type;
use cursed::error::Error;


#[test]
fn test_parse_basic_types() {
    let input = ""
        vibe main
        
        be_like Person squad {
            name tea
            age normie
        }
    "";
    
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    // We should have one statement - a SquadStatement
    assert_eq!(program.statements.len(), 1);
    
    // The first statement should be a SquadStatement
    let squad = program.statements[0].as_any().downcast_ref::<cursed::ast::SquadStatement>().unwrap();
    
    // Check the name
    assert_eq!(squad.name.value, "Person");
    
    // Check that there are no type parameters
    assert_eq!(squad.type_parameters.len(), 0);
    
    // Check the fields
    assert_eq!(squad.fields.len(), 2);
    assert_eq!(squad.fields[0].name.value, "name");
    assert_eq!(squad.fields[0].type_name.string(), "tea");
    assert_eq!(squad.fields[1].name.value, "age");
    assert_eq!(squad.fields[1].type_name.string(), "normie");
}

#[test]
fn test_parse_generic_struct() {
    let input = ""
        vibe main
        
        be_like Box[T] squad {
            value T
        }
    "";
    
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    // We should have one statement - a SquadStatement
    assert_eq!(program.statements.len(), 1);
    
    // The first statement should be a SquadStatement
    let squad = program.statements[0].as_any().downcast_ref::<cursed::ast::SquadStatement>().unwrap();
    
    // Check the name
    assert_eq!(squad.name.value, "Box");
    
    // Check that there's one type parameter
    assert_eq!(squad.type_parameters.len(), 1);
    assert_eq!(squad.type_parameters[0].value, "T");
    
    // Check the field
    assert_eq!(squad.fields.len(), 1);
    assert_eq!(squad.fields[0].name.value, "value");
    assert_eq!(squad.fields[0].type_name.string(), "T");
}

#[test]
fn test_parse_interface() {
    let input = ""
        vibe main
        
        be_like Greeter collab {
            greet(name tea) tea
        }
    "";
    
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    // We should have one statement - a CollabStatement
    assert_eq!(program.statements.len(), 1);
    
    // The first statement should be a CollabStatement
    let collab = program.statements[0].as_any().downcast_ref::<cursed::ast::CollabStatement>().unwrap();
    
    // Check the name
    assert_eq!(collab.name.value, "Greeter");
    
    // Check that there are no type parameters
    assert_eq!(collab.type_parameters.len(), 0);
    
    // Check the method
    assert_eq!(collab.methods.len(), 1);
    assert_eq!(collab.methods[0].name.value, "greet");
    assert_eq!(collab.methods[0].parameters.len(), 1);
    assert_eq!(collab.methods[0].parameters[0].name.value, "name");
    assert_eq!(collab.methods[0].parameters[0].type_name.string(), "tea");
    assert!(collab.methods[0].return_type.is_some());
    assert_eq!(collab.methods[0].return_type.as_ref().unwrap().string(), "tea");
}

#[test]
fn test_parse_generic_interface() {
    let input = ""
        vibe main
        
        be_like Container[T] collab {
            add(item T)
            get(index normie) T
        }
    "";
    
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    // We should have one statement - a CollabStatement
    assert_eq!(program.statements.len(), 1);
    
    // The first statement should be a CollabStatement
    let collab = program.statements[0].as_any().downcast_ref::<cursed::ast::CollabStatement>().unwrap();
    
    // Check the name
    assert_eq!(collab.name.value, "Container");
    
    // Check that there's one type parameter
    assert_eq!(collab.type_parameters.len(), 1);
    assert_eq!(collab.type_parameters[0].value, "T");
    
    // Check the methods
    assert_eq!(collab.methods.len(), 2);
    
    // Check the first method
    assert_eq!(collab.methods[0].name.value, "add");
    assert_eq!(collab.methods[0].parameters.len(), 1);
    assert_eq!(collab.methods[0].parameters[0].name.value, "item");
    assert_eq!(collab.methods[0].parameters[0].type_name.string(), "T");
    assert!(collab.methods[0].return_type.is_none());
    
    // Check the second method
    assert_eq!(collab.methods[1].name.value, "get");
    assert_eq!(collab.methods[1].parameters.len(), 1);
    assert_eq!(collab.methods[1].parameters[0].name.value, "index");
    assert_eq!(collab.methods[1].parameters[0].type_name.string(), "normie");
    assert!(collab.methods[1].return_type.is_some());
    assert_eq!(collab.methods[1].return_type.as_ref().unwrap().string(), "T");
}