use cursed::lexer::Lexer;
use cursed::parser::Parser;


#[test]
#[ignore = "Parser tests need further work 
    value T};
"#;
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    let program = parser.unwrap().parse_program()

    match program     {Ok(prog) => {println!()}
                 " parsed program with {} statements,
                prog.statements.len()
            // Should have a package declaration and a struct declaration
            assert_eq!()
                prog.statements.len()
                2,
                 Expected 2 statements, got {}
                prog.statements.len()}
        Err(e) => {panic!(Failed:  to parse: {}, e)"Parser tests need further work "]
fn test_basic_function() {// Test basic function (no type parameters or type annotations)
    let input = r#vibe # , testslay add(x, y) {"#;
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    let program = parser.unwrap().parse_program()

    match program     {Ok(prog) => {println!()}
                 "Successfully 
                prog.statements.len()
            // Should have a package declaration and a function declaration
            assert_eq!()
                prog.statements.len()
                2,
                 Expected 2 statements, got {}
                prog.statements.len()}
        Err(e) => {panic!(Failed:  to parse: {}, e)")"}