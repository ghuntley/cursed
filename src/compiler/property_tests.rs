//! Property-based tests for the CURSED compiler
//!
//! These tests use the proptest framework to generate random inputs to the compiler
//! and verify that they are handled correctly.

use proptest::prelude::*;
use crate::compiler::{Bytecode, Compiler, Opcode};
use crate::object::Object;
use crate::ast::{Program, Node};
use crate::lexer::Lexer;
use crate::parser_impl::Parser;

// Helper to parse a string into a Program
fn parse(input: String) -> Program {
    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    parser.parse_program().unwrap_or_else(|_| Program::new())
}

// Basic compilation property tests
proptest! {
    #[test]
    fn compiler_handles_any_valid_integer(i in -1000i64..1000i64) {
        let input = format!("{}", i);
        let program = parse(input);
        
        let mut compiler = Compiler::new().expect("Failed to create compiler");
        prop_assert!(compiler.compile(program).is_ok());
        
        let bytecode = compiler.bytecode();
        prop_assert_eq!(bytecode.constants.len(), 1);
        prop_assert_eq!(bytecode.constants[0], Object::Integer(i));
    }
    
    #[test]
    fn compiler_handles_any_valid_float(f in -1000.0..1000.0f64) {
        let input = format!("{}", f);
        let program = parse(input);
        
        let mut compiler = Compiler::new().expect("Failed to create compiler");
        prop_assert!(compiler.compile(program).is_ok());
        
        let bytecode = compiler.bytecode();
        prop_assert_eq!(bytecode.constants.len(), 1);
        
        if let Object::Float(value) = bytecode.constants[0] {
            // Compare with some epsilon for floating point differences
            prop_assert!((value - f).abs() < 0.00001);
        } else {
            prop_assert!(false, "Expected Float object, got {:?}", bytecode.constants[0]);
        }
    }
    
    #[test]
    fn compiler_handles_any_valid_string(s in "[a-zA-Z0-9_\\s!@#$%^&*(),.?\":{}|<>]{1,50}") {
        let input = format!("\"{}\"", s);
        let program = parse(input);
        
        let mut compiler = Compiler::new().expect("Failed to create compiler");
        prop_assert!(compiler.compile(program).is_ok());
        
        let bytecode = compiler.bytecode();
        prop_assert_eq!(bytecode.constants.len(), 1);
        
        if let Object::String(value) = &bytecode.constants[0] {
            prop_assert_eq!(value, &s);
        } else {
            prop_assert!(false, "Expected String object, got {:?}", bytecode.constants[0]);
        }
    }
    
    #[test]
    fn compiler_handles_any_valid_boolean() {
        let input = "true";
        let program = parse(input.to_string());
        
        let mut compiler = Compiler::new().expect("Failed to create compiler");
        prop_assert!(compiler.compile(program).is_ok());
        
        let bytecode = compiler.bytecode();
        prop_assert_eq!(bytecode.constants.len(), 0); // Booleans don't need constants
        
        let input = "false";
        let program = parse(input.to_string());
        
        let mut compiler = Compiler::new().expect("Failed to create compiler");
        prop_assert!(compiler.compile(program).is_ok());
        
        let bytecode = compiler.bytecode();
        prop_assert_eq!(bytecode.constants.len(), 0); // Booleans don't need constants
    }
    
    #[test]
    fn compiler_handles_any_valid_array(
        len in 0usize..10,
        elements in prop::collection::vec((-1000i64..1000i64), 0..10)
    ) {
        let input = format!("[{}]", elements.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(", "));
        let program = parse(input);
        
        let mut compiler = Compiler::new().expect("Failed to create compiler");
        prop_assert!(compiler.compile(program).is_ok());
        
        let bytecode = compiler.bytecode();
        prop_assert_eq!(bytecode.constants.len(), elements.len());
        
        for (i, element) in elements.iter().enumerate() {
            prop_assert_eq!(bytecode.constants[i], Object::Integer(*element));
        }
    }
    
    #[test]
    fn compiler_handles_any_valid_hash(
        keys in prop::collection::vec((-1000i64..1000i64), 0..10),
        values in prop::collection::vec((-1000i64..1000i64), 0..10)
    ) {
        let pairs: Vec<_> = keys.iter().zip(values.iter())
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect();
        let input = format!("{{{}}}", pairs.join(", "));
        let program = parse(input);
        
        let mut compiler = Compiler::new().expect("Failed to create compiler");
        prop_assert!(compiler.compile(program).is_ok());
        
        let bytecode = compiler.bytecode();
        prop_assert_eq!(bytecode.constants.len(), keys.len() * 2);
        
        for i in 0..keys.len() {
            prop_assert_eq!(bytecode.constants[i*2], Object::Integer(keys[i]));
            prop_assert_eq!(bytecode.constants[i*2 + 1], Object::Integer(values[i]));
        }
    }
    
    #[test]
    fn compiler_handles_variable_assignment(
        name in "[a-zA-Z_][a-zA-Z0-9_]{0,20}",
        value in 0..100i64
    ) {
        let input = format!("sus {} tea = {};", name, value);
        let program = parse(input);
        
        let mut compiler = Compiler::new().expect("Failed to create compiler");
        prop_assert!(compiler.compile(program).is_ok());
        
        // The bytecode should contain the value as a constant
        let bytecode = compiler.bytecode();
        prop_assert_eq!(bytecode.constants.len(), 1);
        prop_assert_eq!(bytecode.constants[0], Object::Integer(value));
    }
    
    #[test]
    fn compiler_handles_arithmetic_expressions(a in 0..100i64, b in 0..100i64) {
        // Test addition
        let input = format!("{} + {}", a, b);
        let program = parse(input);
        
        let mut compiler = Compiler::new().expect("Failed to create compiler");
        prop_assert!(compiler.compile(program).is_ok());
        
        // Check for constants and Add opcode
        let bytecode = compiler.bytecode();
        prop_assert_eq!(bytecode.constants.len(), 2);
        prop_assert_eq!(bytecode.constants[0], Object::Integer(a));
        prop_assert_eq!(bytecode.constants[1], Object::Integer(b));
        
        // The instructions should end with Add
        let last_opcode = bytecode.instructions[bytecode.instructions.len() - 1];
        prop_assert_eq!(last_opcode, Opcode::Add as u8);
        
        // Test subtraction
        let input = format!("{} - {}", a, b);
        let program = parse(input);
        
        let mut compiler = Compiler::new().expect("Failed to create compiler");
        prop_assert!(compiler.compile(program).is_ok());
        
        // Check for constants and Sub opcode
        let bytecode = compiler.bytecode();
        prop_assert_eq!(bytecode.constants.len(), 2);
        prop_assert_eq!(bytecode.constants[0], Object::Integer(a));
        prop_assert_eq!(bytecode.constants[1], Object::Integer(b));
        
        // The instructions should end with Sub
        let last_opcode = bytecode.instructions[bytecode.instructions.len() - 1];
        prop_assert_eq!(last_opcode, Opcode::Sub as u8);
    }
    
    #[test]
    fn compiler_handles_if_statements(cond in proptest::bool::ANY) {
        let input = format!(
            "lowkey {} {{ yolo 1; }} highkey {{ yolo 2; }}", 
            if cond { "based" } else { "cap" }
        );
        let program = parse(input);
        
        let mut compiler = Compiler::new().expect("Failed to create compiler");
        prop_assert!(compiler.compile(program).is_ok());
        
        // If statements should generate jump instructions
        let bytecode = compiler.bytecode();
        let has_jump = bytecode.instructions.iter()
            .any(|&instr| instr == Opcode::JumpNotTruthy as u8 || instr == Opcode::Jump as u8);
            
        prop_assert!(has_jump, "If statement should generate jump instructions");
    }
    
    #[test]
    fn compiler_handles_function_definitions(
        param_count in 0..5usize,
        body_value in 0..100i64
    ) {
        // Generate a function with the specified number of parameters
        let mut params = String::new();
        for i in 0..param_count {
            if i > 0 {
                params.push_str(", ");
            }
            params.push_str(&format!("p{}", i));
        }
        
        let input = format!(
            "slay test({}) {{ yolo {}; }}", 
            params, 
            body_value
        );
        let program = parse(input);
        
        let mut compiler = Compiler::new().expect("Failed to create compiler");
        prop_assert!(compiler.compile(program).is_ok());
        
        // Function should be compiled to a CompiledFunction constant
        let bytecode = compiler.bytecode();
        prop_assert!(!bytecode.constants.is_empty());
        
        // The last constant should be the compiled function
        let last_constant = &bytecode.constants[bytecode.constants.len() - 1];
        match last_constant {
            Object::CompiledFunction(func) => {
                prop_assert_eq!(func.num_parameters, param_count);
                // Function body should have a constant instruction
                prop_assert!(!func.instructions.is_empty());
            },
            _ => prop_assert!(false, "Expected CompiledFunction, got {:?}", last_constant),
        }
    }
    
    #[test]
    fn compiler_handles_recursive_expressions(depth in 1..10u32) {
        // Create a nested expression with the specified depth
        fn create_nested_expr(depth: u32, base: i64) -> String {
            if depth == 0 {
                return format!("{}", base);
            }
            
            match depth % 4 {
                0 => format!("({} + {})", 
                    create_nested_expr(depth - 1, base), 
                    create_nested_expr(depth - 1, base + 1)
                ),
                1 => format!("({} - {})", 
                    create_nested_expr(depth - 1, base), 
                    create_nested_expr(depth - 1, base + 1)
                ),
                2 => format!("({} * {})", 
                    create_nested_expr(depth - 1, base), 
                    create_nested_expr(depth - 1, base + 1)
                ),
                _ => format!("({} / {})", 
                    create_nested_expr(depth - 1, base * 2), 
                    create_nested_expr(depth - 1, base + 1)
                ),
            }
        }
        
        let expr = create_nested_expr(depth, 1);
        let program = parse(expr);
        
        let mut compiler = Compiler::new().expect("Failed to create compiler");
        prop_assert!(compiler.compile(program).is_ok());
    }
    
    #[test]
    fn compiler_handles_closures_with_free_variables(
        free_var_name in "[a-zA-Z_][a-zA-Z0-9_]{0,10}",
        free_var_value in 0..100i64,
        param_name in "[a-zA-Z_][a-zA-Z0-9_]{0,10}",
        body_op in 0..4u8
    ) {
        // Create a closure that captures a free variable
        let body_expr = match body_op % 4 {
            0 => format!("{} + {}", free_var_name, param_name),
            1 => format!("{} - {}", free_var_name, param_name),
            2 => format!("{} * {}", free_var_name, param_name),
            _ => format!("{} / {}", free_var_name, param_name),
        };
        
        let input = format!(
            "sus {} tea = {}; sus closure tea = slay({}) {{ yolo {}; }};", 
            free_var_name, free_var_value, param_name, body_expr
        );
        
        let program = parse(input);
        let mut compiler = Compiler::new().expect("Failed to create compiler");
        prop_assert!(compiler.compile(program).is_ok(), "Failed to compile closure with free variable");
        
        // The compiled function should have one free variable
        let bytecode = compiler.bytecode();
        let has_closure = bytecode.instructions.iter().any(|&instr| instr == Opcode::Closure as u8);
        prop_assert!(has_closure, "Closure opcode should be generated");
    }
    
    #[test]
    fn compiler_handles_function_calls_with_arguments(
        func_name in "[a-zA-Z_][a-zA-Z0-9_]{0,10}",
        args_count in 0..5usize
    ) {
        // Create a function call with the specified number of arguments
        let mut args = String::new();
        for i in 0..args_count {
            if i > 0 {
                args.push_str(", ");
            }
            args.push_str(&format!("{}", i));
        }
        
        let input = format!("{}({})", func_name, args);
        let program = parse(input);
        
        let mut compiler = Compiler::new().expect("Failed to create compiler");
        prop_assert!(compiler.compile(program).is_ok(), "Failed to compile function call");
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Program, Statement, Node};
    use crate::compiler::Compiler;
    use crate::parser_impl::Parser;
    use crate::lexer::Lexer;
    use crate::error::Error;
    
    use proptest::prelude::*;
    
    // Generate a random integer
    fn gen_int() -> impl Strategy<Value = i64> {
        -1000..1000i64
    }
    
    // Generate a random boolean
    fn gen_bool() -> impl Strategy<Value = bool> {
        prop::bool::ANY
    }
    
    // Generate a random string
    fn gen_string() -> impl Strategy<Value = String> {
        "\\PC*".prop_map(|s| s.replace("\\", ""))
    }
    
    // Generate a random identifier
    fn gen_identifier() -> impl Strategy<Value = String> {
        "[a-zA-Z][a-zA-Z0-9_]*".prop_map(String::from)
    }
    
    // Generate a random simple expression
    fn gen_simple_expr() -> impl Strategy<Value = Expression> {
        prop_oneof![
            gen_int().prop_map(Expression::IntegerLiteral),
            gen_bool().prop_map(Expression::Boolean),
            gen_string().prop_map(Expression::StringLiteral),
            gen_identifier().prop_map(Expression::Identifier)
        ]
    }
    
    // Generate a random prefix expression
    fn gen_prefix_expr() -> impl Strategy<Value = Expression> {
        (prop_oneof![Just("!"), Just("-")], gen_simple_expr())
            .prop_map(|(op, expr)| Expression::PrefixExpression {
                operator: op.to_string(),
                right: Box::new(expr)
            })
    }
    
    // Generate a random infix expression
    fn gen_infix_expr() -> impl Strategy<Value = Expression> {
        (
            gen_simple_expr(),
            prop_oneof![
                Just("+"), Just("-"), Just("*"), Just("/"),
                Just("=="), Just("!="), Just("<"), Just(">"),
                Just("<="), Just(">=")
            ],
            gen_simple_expr()
        ).prop_map(|(left, op, right)| Expression::InfixExpression {
            left: Box::new(left),
            operator: op.to_string(),
            right: Box::new(right)
        })
    }
    
    // Generate a random expression
    fn gen_expr() -> impl Strategy<Value = Expression> {
        prop_oneof![
            gen_simple_expr(),
            gen_prefix_expr(),
            gen_infix_expr()
        ]
    }
    
    // Generate a random let statement
    fn gen_let_statement() -> impl Strategy<Value = Statement> {
        (gen_identifier(), gen_expr())
            .prop_map(|(name, value)| Statement::Let {
                name,
                value: Box::new(value)
            })
    }
    
    // Generate a random return statement
    fn gen_return_statement() -> impl Strategy<Value = Statement> {
        gen_expr()
            .prop_map(|value| Statement::Return {
                value: Box::new(value)
            })
    }
    
    // Generate a random expression statement
    fn gen_expr_statement() -> impl Strategy<Value = Statement> {
        gen_expr()
            .prop_map(|expr| Statement::Expression {
                expression: Box::new(expr)
            })
    }
    
    // Generate a random statement
    fn gen_statement() -> impl Strategy<Value = Statement> {
        prop_oneof![
            gen_let_statement(),
            gen_return_statement(),
            gen_expr_statement()
        ]
    }
    
    // Generate a random program
    fn gen_program() -> impl Strategy<Value = Program> {
        prop::collection::vec(gen_statement(), 0..10)
            .prop_map(|statements| {
                let mut program = Program::new();
                for stmt in statements {
                    program.statements.push(stmt);
                }
                program
            })
    }
    
    proptest! {
        // Test that any valid program can be compiled without errors
        #[test]
        fn test_compile_random_program(program in gen_program()) {
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            match compiler.compile(program) {
                Ok(_) => {}
                Err(e) => panic!("Compilation failed: {:?}", e),
            }
        }
        
        // Test that any valid expression can be compiled without errors
        #[test]
        fn test_compile_random_expression(expr in gen_expr()) {
            let mut program = Program::new();
            program.statements.push(Statement::Expression {
                expression: Box::new(expr),
            });
            
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            match compiler.compile(program) {
                Ok(_) => {}
                Err(e) => panic!("Compilation failed: {:?}", e),
            }
        }
        
        // Test that the compiler can handle nested expressions
        #[test]
        fn test_compile_nested_expressions(
            expr1 in gen_expr(),
            expr2 in gen_expr(),
            expr3 in gen_expr()
        ) {
            let nested_expr = Expression::InfixExpression {
                left: Box::new(Expression::InfixExpression {
                    left: Box::new(expr1),
                    operator: "+".to_string(),
                    right: Box::new(expr2),
                }),
                operator: "*".to_string(),
                right: Box::new(expr3),
            };
            
            let mut program = Program::new();
            program.statements.push(Statement::Expression {
                expression: Box::new(nested_expr),
            });
            
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            match compiler.compile(program) {
                Ok(_) => {}
                Err(e) => panic!("Compilation failed: {:?}", e),
            }
        }
        
        // Test that a program can be parsed from source and compiled
        #[test]
        fn test_parse_and_compile(
            int1 in gen_int(),
            int2 in gen_int()
        ) {
            let source = format!("sus x = {}; sus y = {}; x + y;", int1, int2);
            let lexer = Lexer::new(&source);
            let mut parser = Parser::new(lexer);
            
            let program = parser.parse_program().unwrap_or_else(|_| Program::new());
            
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            match compiler.compile(program) {
                Ok(_) => {}
                Err(e) => panic!("Compilation failed: {:?}", e),
            }
        }
    }
    
    #[test]
    fn test_constants_compilation() {
        // Test integer constant
        let source = "42";
        compile_and_verify_constant(source, |obj| {
            if let crate::compiler::Object::Integer(val) = obj {
                assert_eq!(*val, 42);
                true
            } else {
                false
            }
        });
        
        // Test string constant
        let source = "\"hello, world\"";
        compile_and_verify_constant(source, |obj| {
            if let crate::compiler::Object::String(val) = obj {
                assert_eq!(*val, "hello, world");
                true
            } else {
                false
            }
        });
        
        // Test boolean constant
        let source = "based";
        compile_and_verify_constant(source, |obj| {
            if let crate::compiler::Object::Boolean(val) = obj {
                assert_eq!(*val, true);
                true
            } else {
                false
            }
        });
    }
    
    fn compile_and_verify_constant<F>(source: &str, verify: F)
    where
        F: Fn(&crate::compiler::Object) -> bool,
    {
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        
        let program = parser.parse_program().unwrap_or_else(|_| Program::new());
        
        let mut compiler = Compiler::new().expect("Failed to create compiler");
        let bytecode = compiler.compile(program).unwrap();
        
        assert!(!bytecode.constants.is_empty(), "No constants were compiled");
        assert!(verify(&bytecode.constants[0]), "Constant verification failed");
    }
} 