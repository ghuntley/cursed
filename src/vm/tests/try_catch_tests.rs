use super::*;
use crate::compiler::{Compiler, Bytecode};
use crate::lexer::Lexer;
use crate::parser_impl::Parser;
use crate::object::Object;

#[test]
fn test_try_catch_basic() {
    let input = r#"
        let result = try {
            5 / 0
        } catch e {
            "caught: " + e
        }
    "#;
    
    let expected = Object::String("caught: Division by zero".to_string());
    
    let vm_result = execute_program(input);
    assert_eq!(vm_result, expected);
}

#[test]
fn test_try_catch_no_error() {
    let input = r#"
        let result = try {
            5 + 5
        } catch e {
            "caught: " + e
        }
    "#;
    
    let expected = Object::Integer(10);
    
    let vm_result = execute_program(input);
    assert_eq!(vm_result, expected);
}

#[test]
fn test_try_catch_nested() {
    let input = r#"
        let result = try {
            try {
                5 / 0
            } catch e1 {
                "inner caught: " + e1
            }
        } catch e2 {
            "outer caught: " + e2
        }
    "#;
    
    let expected = Object::String("inner caught: Division by zero".to_string());
    
    let vm_result = execute_program(input);
    assert_eq!(vm_result, expected);
}

#[test]
fn test_try_catch_propagation() {
    let input = r#"
        let result = try {
            let divide_by_zero = fn() {
                5 / 0
            };
            divide_by_zero()
        } catch e {
            "caught: " + e
        }
    "#;
    
    let expected = Object::String("caught: Division by zero".to_string());
    
    let vm_result = execute_program(input);
    assert_eq!(vm_result, expected);
}

/// Helper function to parse, compile, and execute a program
fn execute_program(input: &str) -> Object {
    // Create lexer and parser
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    
    // Parse the program
    let program = parser.parse_program().unwrap();
    
    // Create compiler and compile the program
    let mut compiler = Compiler::new();
    let bytecode = compiler.compile(program).unwrap();
    
    // Create and run the VM
    let mut vm = VM::new(bytecode).unwrap();
    let result = vm.run().unwrap();
    
    // Return the result from the VM
    result.unwrap_or(Object::Null)
} 