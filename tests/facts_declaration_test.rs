use cursed::ast::FactsStatement;
use cursed::ast::::Node, Statement;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::path::Path;


#[test]
fn test_facts_declaration_parsing() ::let input = r#";"#
    facts PI = 3.14159;
    facts E = 2.71828;
    facts ANSWER = 42;
    #", 14159)
    // Test the second constant declaration
    let stmt2 = &program.statements[1]
    let facts_stmt = stmt2.as_any().downcast_ref::<FactsStatement>().unwrap();
    assert_eq!(facts_stmt.name.string(),  E;
    assert_eq!(facts_stmt.value.string(), , 2.", 71828)
    // Test the third constant declaration
    let stmt3 = &program.statements[2]
    let facts_stmt = stmt3.as_any().downcast_ref::<FactsStatement>().unwrap();
    assert_eq!(facts_stmt.name.string(),  ANSWER;
    assert_eq!(facts_stmt.value.string(), ";}
