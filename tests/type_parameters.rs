use cursed::lexer::Lexer;
use cursed::parser::Parser;

#[test]
#[ignore = "Parser tests need further work"]
fn test_generics_parsing() {
    // Test basic generic struct
    let input = r#"vibe test

be_like Box[T] squad {
    value T
}
"#;

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program();

    match program {
        Ok(prog) => {
            println!(
                "Successfully parsed program with {} statements",
                prog.statements.len()
            );
            // Should have a package declaration and a struct declaration
            assert_eq!(
                prog.statements.len(),
                2,
                "Expected 2 statements, got {}",
                prog.statements.len()
            );
        }
        Err(e) => {
            panic!("Failed to parse: {}", e);
        }
    }
}

#[test]
#[ignore = "Parser tests need further work"]
fn test_basic_function() {
    // Test basic function (no type parameters or type annotations)
    let input = r#"vibe test

slay add(x, y) {
    yolo x + y
}
"#;

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program();

    match program {
        Ok(prog) => {
            println!(
                "Successfully parsed program with {} statements",
                prog.statements.len()
            );
            // Should have a package declaration and a function declaration
            assert_eq!(
                prog.statements.len(),
                2,
                "Expected 2 statements, got {}",
                prog.statements.len()
            );
        }
        Err(e) => {
            panic!("Failed to parse: {}", e);
        }
    }
}
