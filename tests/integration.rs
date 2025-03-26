// CURSED language integration tests
mod integration {
    // Add a simple test file
    #[test]
    fn test_basic_functionality() {
        // Import just what we need to avoid dependency issues
        use cursed::error::Error;
        use cursed::lexer::Lexer;
        use cursed::parser_impl::Parser;
        
        // Create a simple lexer
        let input = "vibe foo;";
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
            line: 1,
            column: 1,
        };
        
        // Initialize lexer
        lexer.read_char();
        
        // Create a parser
        let mut parser = Parser::new(&mut lexer);
        
        // Parse program
        let program = parser.parse_program().expect("Failed to parse simple program");
        
        // Verify we parsed one statement
        assert_eq!(program.statements.len(), 1, "Expected one statement");
        
        // Success!
        assert!(true);
    }

    // Helper function to compile and run code
    fn compile_and_run(input: &str) -> Result<Object, Error> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;
        
        let mut compiler = Compiler::new()?;
        let bytecode = compiler.compile(program)?;
        
        let mut vm = VM::new(bytecode);
        vm.run()
    }

    #[test]
    fn test_basic_integration() {
        let input = r#"
            vibe main;
            yeet "std";
            
            let x = 42;
            let y = x + 1;
        "#;
        
        let result = compile_and_run(input).unwrap();
        assert_eq!(result, Object::Integer(43));
    }
}

// Re-export the cursed crate for tests
use cursed::*; 