use cursed::*;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser_impl::Parser;
use cursed::compiler::Compiler;
use cursed::vm::VM;
use cursed::object::Object;
use std::rc::Rc;

#[test]
fn test_basic_arithmetic() {
    let input = r#"
        let x = 42;
        let y = x + 1;
        y
    "#;
    
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;
    
    let mut compiler = Compiler::new()?;
    let bytecode = compiler.compile(program)?;
    
    let mut vm = VM::new(bytecode);
    let result = vm.run()?;
    
    assert_eq!(result, Object::Integer(43));
}

#[test]
fn test_basic_string_operations() {
    let input = r#"
        let s1 = "Hello";
        let s2 = " World";
        let s3 = s1 + s2;
        s3
    "#;
    
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;
    
    let mut compiler = Compiler::new()?;
    let bytecode = compiler.compile(program)?;
    
    let mut vm = VM::new(bytecode);
    let result = vm.run()?;
    
    assert_eq!(result, Object::String("Hello World".to_string()));
}

#[test]
fn test_boolean_operations() -> Result<(), Error> {
    let input = "true && false || true";
    
    // Create lexer and parser
    let mut lexer = Lexer {
        input,
        position: 0,
        read_position: 0,
        ch: None,
        line: 1,
        column: 1,
    };
    let mut parser = Parser::new(&mut lexer);
    
    // Parse the input
    let program = parser.parse_program()?;
    
    // Compile the program
    let mut compiler = Compiler::new()?;
    let bytecode = compiler.compile_program(&program)?;
    
    // Create VM and run the program
    let mut vm = VM::new(bytecode)?;
    let result = vm.run()?;
    
    // Check result
    match result {
        Object::Boolean(val) => {
            assert_eq!(val, true, "Expected true && false || true = true, got {}", val);
            Ok(())
        },
        other => {
            panic!("Expected boolean result, got {:?}", other);
        }
    }
} 