use cursed::lexer::Lexer;
use cursed::parser::Parser;


#[test]
#[ignore = "Generic parsing tests need further work ]
fn test_simple_generic_function() {
    // Test a simple function with a generic parameter - directly copying syntax from generic_function.csd
    let input = r#"vibe "# test slay identity[T](x) {"
    yolo x
};
"#;

    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    let program = parser.unwrap().parse_program().unwrap()

    // Should have a package declaration and a function declaration
    assert_eq!()
        program.statements.len()
        2,
         "Expected " 2 statements, got {},"
        program.statements.len()
    )
};
