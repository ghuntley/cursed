// Integration test for the CURSED language
//
// This test file verifies that all components of the language work together correctly,
// from parsing through compilation to execution.

use cursed::ast::Program;
use cursed::compiler::{Bytecode, Compiler};
use cursed::lexer::Lexer;
use cursed::object::Object;
use cursed::parser_impl::Parser;
use cursed::vm::VM;
use std::rc::Rc;

// Test utility to run source code and return the result
fn execute_code(input: &str) -> Result<Object, String> {
    // Create lexer from input
    let lexer = Lexer::new(input);
    
    // Create parser and parse program
    let mut parser = Parser::new(lexer);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => return Err(format!("Parser error: {}", e)),
    };
    
    // Create compiler and compile program
    let mut compiler = match Compiler::new() {
        Ok(c) => c,
        Err(e) => return Err(format!("Compiler creation error: {}", e)),
    };
    let bytecode = match compiler.compile(program) {
        Ok(b) => b,
        Err(e) => return Err(format!("Compilation error: {}", e)),
    };
    
    // Create VM and execute bytecode
    let mut vm = match VM::new(bytecode) {
        Ok(vm) => vm,
        Err(e) => return Err(format!("VM initialization error: {}", e)),
    };
    
    // Run the VM
    match vm.run() {
        Ok(Some(result)) => Ok(result),
        Ok(None) => Ok(Object::Null),
        Err(e) => Err(format!("Runtime error: {}", e)),
    }
}

// Test different language features
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_arithmetic_operations() {
        let tests = vec![
            ("5", Object::Integer(5)),
            ("5 + 5", Object::Integer(10)),
            ("5 - 5", Object::Integer(0)),
            ("5 * 5", Object::Integer(25)),
            ("5 / 5", Object::Integer(1)),
            ("5 + 5 * 5", Object::Integer(30)),
            ("(5 + 5) * 5", Object::Integer(50)),
        ];
        
        for (input, expected) in tests {
            match execute_code(input) {
                Ok(result) => assert_eq!(result, expected, "Failed for input: {}", input),
                Err(e) => panic!("Error executing '{}': {}", input, e),
            }
        }
    }
    
    #[test]
    fn test_boolean_operations() {
        let tests = vec![
            ("based", Object::Boolean(true)),
            ("cap", Object::Boolean(false)),
            ("!based", Object::Boolean(false)),
            ("!cap", Object::Boolean(true)),
            ("based == based", Object::Boolean(true)),
            ("cap == cap", Object::Boolean(true)),
            ("based == cap", Object::Boolean(false)),
            ("based != cap", Object::Boolean(true)),
        ];
        
        for (input, expected) in tests {
            match execute_code(input) {
                Ok(result) => assert_eq!(result, expected, "Failed for input: {}", input),
                Err(e) => panic!("Error executing '{}': {}", input, e),
            }
        }
    }
    
    #[test]
    fn test_string_operations() {
        let tests = vec![
            (r#""Hello, world!""#, Object::String("Hello, world!".to_string())),
            (r#""Hello, " + "world!""#, Object::String("Hello, world!".to_string())),
        ];
        
        for (input, expected) in tests {
            match execute_code(input) {
                Ok(result) => assert_eq!(result, expected, "Failed for input: {}", input),
                Err(e) => panic!("Error executing '{}': {}", input, e),
            }
        }
    }
    
    #[test]
    fn test_variables() {
        let tests = vec![
            ("sus x = 5; x;", Object::Integer(5)),
            ("sus x = 5; sus y = x + 5; y;", Object::Integer(10)),
            ("sus x = 5; sus y = x; sus z = x + y; z;", Object::Integer(10)),
        ];
        
        for (input, expected) in tests {
            match execute_code(input) {
                Ok(result) => assert_eq!(result, expected, "Failed for input: {}", input),
                Err(e) => panic!("Error executing '{}': {}", input, e),
            }
        }
    }
    
    #[test]
    fn test_functions() {
        let tests = vec![
            (
                "slay add(x, y) { yolo x + y; }; add(5, 5);", 
                Object::Integer(10)
            ),
            (
                "slay multiply(x, y) { yolo x * y; }; multiply(5, 5);", 
                Object::Integer(25)
            ),
            (
                "slay factorial(n) { lowkey (n == 0) { yolo 1; } highkey { yolo n * factorial(n - 1); } }; factorial(5);", 
                Object::Integer(120)
            ),
        ];
        
        for (input, expected) in tests {
            match execute_code(input) {
                Ok(result) => assert_eq!(result, expected, "Failed for input: {}", input),
                Err(e) => panic!("Error executing '{}': {}", input, e),
            }
        }
    }
    
    #[test]
    fn test_closures() {
        let input = r#"
            slay makeAdder(x) {
                yolo slay(y) { yolo x + y; };
            };
            
            sus add5 = makeAdder(5);
            add5(10);
        "#;
        
        match execute_code(input) {
            Ok(result) => assert_eq!(result, Object::Integer(15)),
            Err(e) => panic!("Error executing closure test: {}", e),
        }
    }
    
    #[test]
    fn test_arrays() {
        let tests = vec![
            ("[1, 2, 3]", Object::Array(vec![
                Object::Integer(1),
                Object::Integer(2),
                Object::Integer(3),
            ])),
            ("[1, 2, 3][1]", Object::Integer(2)),
            ("sus arr = [1, 2, 3]; arr[2];", Object::Integer(3)),
        ];
        
        for (input, expected) in tests {
            match execute_code(input) {
                Ok(result) => assert_eq!(result, expected, "Failed for input: {}", input),
                Err(e) => panic!("Error executing '{}': {}", input, e),
            }
        }
    }
    
    #[test]
    fn test_hash_tables() {
        let input = r#"
            sus person = {
                "name": "Bob",
                "age": 30,
                "isEmployed": based
            };
            
            person["age"];
        "#;
        
        match execute_code(input) {
            Ok(result) => assert_eq!(result, Object::Integer(30)),
            Err(e) => panic!("Error executing hash table test: {}", e),
        }
    }
    
    #[test]
    fn test_if_statements() {
        let tests = vec![
            ("lowkey (based) { 10 } highkey { 20 }", Object::Integer(10)),
            ("lowkey (cap) { 10 } highkey { 20 }", Object::Integer(20)),
            ("lowkey (1 < 2) { 10 } highkey { 20 }", Object::Integer(10)),
            ("lowkey (1 > 2) { 10 } highkey { 20 }", Object::Integer(20)),
        ];
        
        for (input, expected) in tests {
            match execute_code(input) {
                Ok(result) => assert_eq!(result, expected, "Failed for input: {}", input),
                Err(e) => panic!("Error executing '{}': {}", input, e),
            }
        }
    }
    
    #[test]
    fn test_error_handling() {
        let input = r#"
            sus result = try {
                5 / 0
            } catch e {
                "caught: " + e
            }
        "#;
        
        match execute_code(input) {
            Ok(result) => {
                if let Object::String(s) = result {
                    assert!(s.contains("caught:") && s.contains("Division by zero"), 
                        "Error message doesn't contain expected text: {}", s);
                } else {
                    panic!("Expected String object, got: {:?}", result);
                }
            },
            Err(e) => panic!("Error executing error handling test: {}", e),
        }
    }
    
    #[test]
    fn test_structs() {
        let input = r#"
            squad Person {
                name String
                age Integer
            }
            
            sus bob = Person be_like {
                name: "Bob",
                age: 30
            };
            
            bob.age
        "#;
        
        match execute_code(input) {
            Ok(result) => assert_eq!(result, Object::Integer(30)),
            Err(e) => panic!("Error executing struct test: {}", e),
        }
    }

    #[test]
    fn test_end_to_end() -> Result<(), Error> {
        let input = r#"
            let x = 42;
            let y = x + 1;
            let z = y * 2;
            z
        "#;
        
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;
        
        let mut compiler = Compiler::new()?;
        let bytecode = compiler.compile(program)?;
        
        let mut vm = VM::new(bytecode);
        let result = vm.run()?;
        
        assert_eq!(result, Object::Integer(86));
        Ok(())
    }
} 