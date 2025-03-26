#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::Object;
    use crate::compiler::{Bytecode, Compiler, Instructions};
    use crate::compiler::bytecode::{Opcode, lookup};
    use crate::ast::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use proptest::prelude::*;
    
    struct TestCase {
        input: String,
        expected_constants: Vec<Object>,
        expected_instructions: Vec<Instructions>,
    }
    
    #[test]
    fn test_integer_arithmetic() {
        let tests = vec![
            TestCase {
                input: "1 + 2".to_string(),
                expected_constants: vec![
                    Object::Integer(1),
                    Object::Integer(2),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Add, vec![]),
                ],
            },
            TestCase {
                input: "1 - 2".to_string(),
                expected_constants: vec![
                    Object::Integer(1),
                    Object::Integer(2),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Sub, vec![]),
                ],
            },
            TestCase {
                input: "1 * 2".to_string(),
                expected_constants: vec![
                    Object::Integer(1),
                    Object::Integer(2),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Mul, vec![]),
                ],
            },
            TestCase {
                input: "2 / 1".to_string(),
                expected_constants: vec![
                    Object::Integer(2),
                    Object::Integer(1),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Div, vec![]),
                ],
            },
            TestCase {
                input: "-1".to_string(),
                expected_constants: vec![
                    Object::Integer(1),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Minus, vec![]),
                ],
            },
        ];
        
        run_compiler_tests(tests);
    }
    
    #[test]
    fn test_boolean_expressions() {
        let tests = vec![
            TestCase {
                input: "true".to_string(),
                expected_constants: vec![],
                expected_instructions: vec![
                    make(Opcode::True, vec![]),
                ],
            },
            TestCase {
                input: "false".to_string(),
                expected_constants: vec![],
                expected_instructions: vec![
                    make(Opcode::False, vec![]),
                ],
            },
            TestCase {
                input: "1 > 2".to_string(),
                expected_constants: vec![
                    Object::Integer(1),
                    Object::Integer(2),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::GreaterThan, vec![]),
                ],
            },
            TestCase {
                input: "1 < 2".to_string(),
                expected_constants: vec![
                    Object::Integer(1),
                    Object::Integer(2),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::GreaterThan, vec![]),
                ],
            },
            TestCase {
                input: "1 == 2".to_string(),
                expected_constants: vec![
                    Object::Integer(1),
                    Object::Integer(2),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Equal, vec![]),
                ],
            },
            TestCase {
                input: "1 != 2".to_string(),
                expected_constants: vec![
                    Object::Integer(1),
                    Object::Integer(2),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::NotEqual, vec![]),
                ],
            },
            TestCase {
                input: "!true".to_string(),
                expected_constants: vec![],
                expected_instructions: vec![
                    make(Opcode::True, vec![]),
                    make(Opcode::Bang, vec![]),
                ],
            },
        ];
        
        run_compiler_tests(tests);
    }
    
    #[test]
    fn test_string_expressions() {
        let tests = vec![
            TestCase {
                input: r#""hello world""#.to_string(),
                expected_constants: vec![
                    Object::String("hello world".to_string()),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                ],
            },
            TestCase {
                input: r#""hello" + " " + "world""#.to_string(),
                expected_constants: vec![
                    Object::String("hello".to_string()),
                    Object::String(" ".to_string()),
                    Object::String("world".to_string()),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Add, vec![]),
                    make(Opcode::Constant, vec![2]),
                    make(Opcode::Add, vec![]),
                ],
            },
        ];
        
        run_compiler_tests(tests);
    }
    
    #[test]
    fn test_let_statements() {
        let tests = vec![
            TestCase {
                input: "let x = 5;".to_string(),
                expected_constants: vec![
                    Object::Integer(5),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::SetGlobal, vec![0]),
                ],
            },
            TestCase {
                input: "let y = true;".to_string(),
                expected_constants: vec![],
                expected_instructions: vec![
                    make(Opcode::True, vec![]),
                    make(Opcode::SetGlobal, vec![0]),
                ],
            },
            TestCase {
                input: "let z = \"hello\";".to_string(),
                expected_constants: vec![
                    Object::String("hello".to_string()),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::SetGlobal, vec![0]),
                ],
            },
            TestCase {
                input: "let a = 1; let b = a;".to_string(),
                expected_constants: vec![
                    Object::Integer(1),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::SetGlobal, vec![0]),
                    make(Opcode::GetGlobal, vec![0]),
                    make(Opcode::SetGlobal, vec![1]),
                ],
            },
        ];
        
        run_compiler_tests(tests);
    }
    
    #[test]
    fn test_if_statements() {
        let tests = vec![
            TestCase {
                input: "if (true) { 10 } else { 20 }".to_string(),
                expected_constants: vec![
                    Object::Integer(10),
                    Object::Integer(20),
                ],
                expected_instructions: vec![
                    make(Opcode::True, vec![]),                   // 0000
                    make(Opcode::JumpNotTruthy, vec![10]),        // 0001
                    make(Opcode::Constant, vec![0]),              // 0004
                    make(Opcode::Pop, vec![]),                    // 0007
                    make(Opcode::Jump, vec![13]),                 // 0008
                    make(Opcode::Constant, vec![1]),              // 0011
                    make(Opcode::Pop, vec![]),                    // 0014
                ],
            },
            TestCase {
                input: "if (true) { 10 }".to_string(),
                expected_constants: vec![
                    Object::Integer(10),
                ],
                expected_instructions: vec![
                    make(Opcode::True, vec![]),                   // 0000
                    make(Opcode::JumpNotTruthy, vec![7]),         // 0001
                    make(Opcode::Constant, vec![0]),              // 0004
                    make(Opcode::Pop, vec![]),                    // 0007
                ],
            },
        ];
        
        run_compiler_tests(tests);
    }
    
    #[test]
    fn test_return_statements() {
        let tests = vec![
            TestCase {
                input: "return 10;".to_string(),
                expected_constants: vec![
                    Object::Integer(10),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::ReturnValue, vec![]),
                ],
            },
            TestCase {
                input: "return true;".to_string(),
                expected_constants: vec![],
                expected_instructions: vec![
                    make(Opcode::True, vec![]),
                    make(Opcode::ReturnValue, vec![]),
                ],
            },
            TestCase {
                input: "return;".to_string(),
                expected_constants: vec![],
                expected_instructions: vec![
                    make(Opcode::Return, vec![]),
                ],
            },
            TestCase {
                input: "return 1 + 2;".to_string(),
                expected_constants: vec![
                    Object::Integer(1),
                    Object::Integer(2),
                ],
                expected_instructions: vec![
                    make(Opcode::Constant, vec![0]),
                    make(Opcode::Constant, vec![1]),
                    make(Opcode::Add, vec![]),
                    make(Opcode::ReturnValue, vec![]),
                ],
            },
        ];
        
        run_compiler_tests(tests);
    }
    
    #[test]
    fn test_scopes() {
        let mut compiler = Compiler::new().expect("Failed to create compiler");
        compiler.emit(Opcode::Mul, vec![]);
        compiler.emit(Opcode::Sub, vec![]);
        
        let glob_scope = compiler.current_instructions().clone();
        
        compiler.enter_scope();
        compiler.emit(Opcode::Add, vec![]);
        
        assert_eq!(compiler.current_instructions().len(), 1);
        assert_eq!(compiler.current_instructions()[0], Opcode::Add as u8);
        
        compiler.leave_scope();
        
        assert_eq!(compiler.current_instructions(), &glob_scope);
        
        compiler.emit(Opcode::Div, vec![]);
        assert_eq!(compiler.current_instructions().len(), 3);
        assert_eq!(compiler.current_instructions()[0], Opcode::Mul as u8);
        assert_eq!(compiler.current_instructions()[1], Opcode::Sub as u8);
        assert_eq!(compiler.current_instructions()[2], Opcode::Div as u8);
    }
    
    // Property-based tests for the compiler
    proptest! {
        #[test]
        fn compiler_doesnt_crash_on_valid_expressions(
            i in 0..100i64,
            f in 0.0..100.0f64,
            s in "[a-zA-Z0-9_\\s!@#$%^&*(),.?\":{}|<>]{1,10}"
        ) {
            // Integer expressions
            let input = format!("{}", i);
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.clone());
            prop_assert!(compiler.compile(&program).is_ok());
            
            // Integer arithmetic
            let input = format!("{} + {}", i, i + 1);
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.clone());
            prop_assert!(compiler.compile(&program).is_ok());
            
            let input = format!("{} - {}", i, i - 1);
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.clone());
            prop_assert!(compiler.compile(&program).is_ok());
            
            let input = format!("{} * {}", i, i);
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.clone());
            prop_assert!(compiler.compile(&program).is_ok());
            
            let input = if i != 0 { format!("{} / {}", i*i, i) } else { format!("1 / 1") };
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.clone());
            prop_assert!(compiler.compile(&program).is_ok());
            
            // Float expressions
            let input = format!("{}", f);
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.clone());
            prop_assert!(compiler.compile(&program).is_ok());
            
            // String expressions
            let input = format!("\"{}\"", s);
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.clone());
            prop_assert!(compiler.compile(&program).is_ok());
            
            // String concatenation
            let input = format!("\"{}\" + \"{}\"", s, s);
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.clone());
            prop_assert!(compiler.compile(&program).is_ok());
            
            // Boolean expressions
            let input = "true";
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.to_string());
            prop_assert!(compiler.compile(&program).is_ok());
            
            let input = "false";
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.to_string());
            prop_assert!(compiler.compile(&program).is_ok());
            
            // Boolean operations
            let input = "!true";
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.to_string());
            prop_assert!(compiler.compile(&program).is_ok());
            
            let input = "!false";
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.to_string());
            prop_assert!(compiler.compile(&program).is_ok());
            
            // Comparison expressions
            let input = format!("{} > {}", i, i - 1);
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.clone());
            prop_assert!(compiler.compile(&program).is_ok());
            
            let input = format!("{} < {}", i, i + 1);
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.clone());
            prop_assert!(compiler.compile(&program).is_ok());
            
            let input = format!("{} == {}", i, i);
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.clone());
            prop_assert!(compiler.compile(&program).is_ok());
            
            let input = format!("{} != {}", i, i + 1);
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.clone());
            prop_assert!(compiler.compile(&program).is_ok());
            
            // Combined expressions
            let input = format!("-{} + {} * ({} / {})", 
                            i, i + 1, i + 2, if i != 0 { i } else { 1 });
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.clone());
            prop_assert!(compiler.compile(&program).is_ok());
            
            // Complex expressions
            let input = format!("(({} > {}) == ({} < {})) != !true", 
                            i, i - 1, i, i + 1);
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            let program = parse(input.clone());
            prop_assert!(compiler.compile(&program).is_ok());
        }
        
        #[test]
        fn compiler_integer_arithmetic_property(a in 0..100i64, b in 0..100i64) {
            // Test addition
            let input = format!("{} + {}", a, b);
            let program = parse(input);
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            compiler.compile(&program).unwrap();
            let bytecode = compiler.bytecode();
            
            // Verify constants
            prop_assert_eq!(bytecode.constants.len(), 2);
            prop_assert_eq!(bytecode.constants[0], Object::Integer(a));
            prop_assert_eq!(bytecode.constants[1], Object::Integer(b));
            
            // Verify instructions (Constant 0, Constant 1, Add)
            prop_assert_eq!(bytecode.instructions[0], Opcode::Constant as u8);
            prop_assert_eq!(bytecode.instructions[2], Opcode::Constant as u8);
            prop_assert_eq!(bytecode.instructions[4], Opcode::Add as u8);
        }
        
        #[test]
        fn compiler_string_compilation_property(s in "[a-zA-Z0-9_\\s!@#$%^&*(),.?\":{}|<>]{1,10}") {
            // Test string literal
            let input = format!("\"{}\"", s);
            let program = parse(input);
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            compiler.compile(&program).unwrap();
            let bytecode = compiler.bytecode();
            
            // Verify constants
            prop_assert_eq!(bytecode.constants.len(), 1);
            if let Object::String(compiled_string) = &bytecode.constants[0] {
                prop_assert_eq!(compiled_string, &s);
            } else {
                prop_assert!(false, "Expected String object, got {:?}", bytecode.constants[0]);
            }
            
            // Verify instructions (Constant 0)
            prop_assert_eq!(bytecode.instructions[0], Opcode::Constant as u8);
        }
        
        #[test]
        fn compiler_boolean_compilation_property(b in proptest::bool::ANY) {
            // Test boolean literal
            let input = format!("{}", b);
            let program = parse(input);
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            compiler.compile(&program).unwrap();
            let bytecode = compiler.bytecode();
            
            // Verify no constants (booleans don't need constants)
            prop_assert_eq!(bytecode.constants.len(), 0);
            
            // Verify instructions (True or False opcode)
            if b {
                prop_assert_eq!(bytecode.instructions[0], Opcode::True as u8);
            } else {
                prop_assert_eq!(bytecode.instructions[0], Opcode::False as u8);
            }
        }
        
        #[test]
        fn compiler_nested_expressions_property(a in 1..10i64, b in 1..10i64) {
            // Test nested expressions with precedence
            let input = format!("{} + {} * {}", a, b, a);
            let program = parse(input);
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            compiler.compile(&program).unwrap();
            let bytecode = compiler.bytecode();
            
            // Verify constants (should have a, b, a again)
            prop_assert_eq!(bytecode.constants.len(), 3);
            prop_assert_eq!(bytecode.constants[0], Object::Integer(a));
            prop_assert_eq!(bytecode.constants[1], Object::Integer(b));
            prop_assert_eq!(bytecode.constants[2], Object::Integer(a));
            
            // Verify instructions follow correct order for operator precedence
            // Constant a, Constant b, Constant a, Mul, Add
            let opcodes: Vec<u8> = bytecode.instructions.iter()
                .enumerate()
                .filter(|(i, _)| i % 2 == 0) // Only check at even positions where opcodes are
                .map(|(_, &byte)| byte)
                .collect();
            
            prop_assert!(opcodes.contains(&(Opcode::Constant as u8)));
            prop_assert!(opcodes.contains(&(Opcode::Mul as u8)));
            prop_assert!(opcodes.contains(&(Opcode::Add as u8)));
            
            // Check that Mul comes before Add (operator precedence)
            let mul_pos = opcodes.iter().position(|&op| op == Opcode::Mul as u8).unwrap();
            let add_pos = opcodes.iter().position(|&op| op == Opcode::Add as u8).unwrap();
            prop_assert!(mul_pos < add_pos);
        }
        
        #[test]
        fn compiler_combined_expressions_property() {
            // Define generators for different kinds of expressions
            let integer_expr = (0..100i64).prop_map(|i| format!("{}", i));
            let float_expr = (0.0..100.0f64).prop_map(|f| format!("{}", f));
            let string_expr = "[a-zA-Z0-9_\\s!@#$%^&*(),.?\":{}|<>]{1,10}"
                .prop_map(|s| format!("\"{}\"", s));
            let boolean_expr = proptest::bool::ANY
                .prop_map(|b| format!("{}", b));
            
            // Define a recursive expression generator
            let leaf = prop_oneof![
                3 => integer_expr.clone(),
                2 => float_expr.clone(),
                2 => string_expr.clone(),
                1 => boolean_expr.clone()
            ];
            
            fn make_expr(depth: u32, leaf: &impl Strategy<Value = String>) -> impl Strategy<Value = String> {
                let leaf_strategy = leaf.clone();
                if depth == 0 {
                    leaf_strategy.boxed()
                } else {
                    prop_oneof![
                        // Leaf expression
                        3 => leaf_strategy.clone(),
                        
                        // Unary expressions
                        1 => make_expr(depth-1, &leaf_strategy)
                            .prop_map(|e| format!("-{}", e)),
                        1 => make_expr(depth-1, &leaf_strategy)
                            .prop_map(|e| format!("!{}", e)),
                        
                        // Binary expressions
                        1 => (make_expr(depth-1, &leaf_strategy), make_expr(depth-1, &leaf_strategy))
                            .prop_map(|(l, r)| format!("{} + {}", l, r)),
                        1 => (make_expr(depth-1, &leaf_strategy), make_expr(depth-1, &leaf_strategy))
                            .prop_map(|(l, r)| format!("{} - {}", l, r)),
                        1 => (make_expr(depth-1, &leaf_strategy), make_expr(depth-1, &leaf_strategy))
                            .prop_map(|(l, r)| format!("{} * {}", l, r)),
                        1 => (make_expr(depth-1, &leaf_strategy), make_expr(depth-1, &leaf_strategy))
                            .prop_map(|(l, r)| format!("{} / {}", l, r)),
                        1 => (make_expr(depth-1, &leaf_strategy), make_expr(depth-1, &leaf_strategy))
                            .prop_map(|(l, r)| format!("{} > {}", l, r)),
                        1 => (make_expr(depth-1, &leaf_strategy), make_expr(depth-1, &leaf_strategy))
                            .prop_map(|(l, r)| format!("{} < {}", l, r)),
                        1 => (make_expr(depth-1, &leaf_strategy), make_expr(depth-1, &leaf_strategy))
                            .prop_map(|(l, r)| format!("{} == {}", l, r)),
                        1 => (make_expr(depth-1, &leaf_strategy), make_expr(depth-1, &leaf_strategy))
                            .prop_map(|(l, r)| format!("{} != {}", l, r)),
                        
                        // Grouped expressions
                        1 => make_expr(depth-1, &leaf_strategy)
                            .prop_map(|e| format!("({})", e))
                    ].boxed()
                }
            }
            
            // Generate expressions with a maximum depth of 3
            let expr_strategy = make_expr(3, &leaf);
            
            // Test that the compiler can compile these expressions without crashing
            expr_strategy.prop_flat_map(|expr| {
                // Try to parse the expression first to filter out invalid expressions
                let lexer = Lexer::new(&expr);
                let mut parser = Parser::new(lexer);
                match parser.parse_program() {
                    Ok(program) => Just((expr, program)).boxed(),
                    Err(_) => Just((format!("42"), parse("42".to_string()))).boxed() // Fallback to a simple valid expression
                }
            }).prop_flat_map(|(expr, program)| {
                // Now we have a valid expression, compile it
                Just((expr, program)).boxed()
            }).prop_map(|(expr, program)| {
                // Now compile the expression and check that it works
                let mut compiler = Compiler::new().expect("Failed to create compiler");
                let result = compiler.compile(&program);
                prop_assert!(result.is_ok(), "Failed to compile expression: {}", expr);
                
                // Get the bytecode and verify basic properties
                let bytecode = compiler.bytecode();
                prop_assert!(!bytecode.instructions.is_empty(), "Empty instructions for expression: {}", expr);
                
                // The test succeeded
                true
            }).boxed()
        }
        
        #[test]
        fn compiler_statement_compilation_property() {
            // Test let statements
            prop_oneof![
                // Let with integer
                (0..100i64).prop_map(|i| format!("let x = {};", i)),
                // Let with string
                "[a-zA-Z0-9_\\s]{1,10}".prop_map(|s| format!("let x = \"{}\";", s)),
                // Let with boolean
                proptest::bool::ANY.prop_map(|b| format!("let x = {};", b)),
                // Let with expression
                (0..10i64, 0..10i64).prop_map(|(a, b)| format!("let x = {} + {};", a, b))
            ].prop_map(|input| {
                let program = parse(input.clone());
                let mut compiler = Compiler::new().expect("Failed to create compiler");
                let result = compiler.compile(&program);
                prop_assert!(result.is_ok(), "Failed to compile let statement: {}", input);
                true
            }).boxed()
        }
        
        #[test]
        fn compiler_if_statement_property() {
            // Test if statements with different conditions and bodies
            let conditions = prop_oneof![
                // Boolean literal
                proptest::bool::ANY.prop_map(|b| format!("{}", b)),
                // Comparison
                (0..10i64, 0..10i64).prop_map(|(a, b)| format!("{} > {}", a, b)),
                // Complex condition
                (0..10i64, 0..10i64).prop_map(|(a, b)| format!("{} == {} || {} != {}", a, a, a, b))
            ];
            
            let bodies = prop_oneof![
                // Single expression
                (0..10i64).prop_map(|i| format!("{}", i)),
                // Multiple expressions
                (0..10i64, 0..10i64).prop_map(|(a, b)| format!("{} + {}", a, b))
            ];
            
            let if_stmt = (conditions, bodies.clone(), option::of(bodies.clone()))
                .prop_map(|(cond, conseq, alt)| {
                    if let Some(alt_body) = alt {
                        format!("if ({}) {{ {} }} else {{ {} }}", cond, conseq, alt_body)
                    } else {
                        format!("if ({}) {{ {} }}", cond, conseq)
                    }
                });
            
            if_stmt.prop_map(|input| {
                let program = parse(input.clone());
                let mut compiler = Compiler::new().expect("Failed to create compiler");
                let result = compiler.compile(&program);
                prop_assert!(result.is_ok(), "Failed to compile if statement: {}", input);
                
                // Get bytecode and verify it has the right instructions
                let bytecode = compiler.bytecode();
                let instructions = bytecode.instructions;
                
                // If statements should contain JumpNotTruthy instruction
                let contains_jump = instructions.iter().enumerate()
                    .any(|(i, &byte)| i % 2 == 0 && byte == Opcode::JumpNotTruthy as u8);
                
                prop_assert!(contains_jump, "If statement compilation should include JumpNotTruthy instruction");
                true
            }).boxed()
        }
        
        #[test]
        fn compiler_return_statement_property() {
            // Test return statements with different expressions
            prop_oneof![
                // Return with integer
                (0..100i64).prop_map(|i| format!("return {};", i)),
                // Return with string
                "[a-zA-Z0-9_\\s]{1,10}".prop_map(|s| format!("return \"{}\";", s)),
                // Return with boolean
                proptest::bool::ANY.prop_map(|b| format!("return {};", b)),
                // Return with expression
                (0..10i64, 0..10i64).prop_map(|(a, b)| format!("return {} + {};", a, b)),
                // Simple return
                Just("return;".to_string())
            ].prop_map(|input| {
                let program = parse(input.clone());
                let mut compiler = Compiler::new().expect("Failed to create compiler");
                let result = compiler.compile(&program);
                prop_assert!(result.is_ok(), "Failed to compile return statement: {}", input);
                
                // Get bytecode and verify it has the right instructions
                let bytecode = compiler.bytecode();
                let instructions = bytecode.instructions;
                
                // Return statements should end with ReturnValue or Return instruction
                let has_return_instr = instructions.iter().enumerate()
                    .any(|(i, &byte)| i % 2 == 0 && 
                         (byte == Opcode::Return as u8 || byte == Opcode::ReturnValue as u8));
                
                prop_assert!(has_return_instr, "Return statement compilation should include Return or ReturnValue instruction");
                true
            }).boxed()
        }
    }
    
    // Helper function to run compiler tests
    fn run_compiler_tests(tests: Vec<TestCase>) {
        for test in tests {
            let program = parse(test.input);
            
            let mut compiler = Compiler::new().expect("Failed to create compiler");
            match compiler.compile(&program) {
                Ok(_) => {
                    // Check bytecode matches expected
                    let bytecode = compiler.bytecode();
                    
                    test_instructions(&test.expected_instructions, &bytecode.instructions);
                    test_constants(&test.expected_constants, &bytecode.constants);
                }
                Err(err) => {
                    panic!("Compiler error: {}", err);
                }
            }
        }
    }
    
    // Helper function to parse input
    fn parse(input: String) -> Box<Program> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_program()
    }
    
    // Helper function to make an instruction
    fn make(op: Opcode, operands: Vec<usize>) -> Instructions {
        let mut instruction = vec![];
        let definition = lookup(op);
        
        instruction.push(op as u8);
        
        for (i, operand) in operands.iter().enumerate() {
            let width = definition.operand_widths[i];
            match width {
                1 => instruction.push(*operand as u8),
                2 => {
                    let bytes = (*operand as u16).to_be_bytes();
                    instruction.extend_from_slice(&bytes);
                }
                _ => panic!("Unsupported operand width: {}", width),
            }
        }
        
        instruction
    }
    
    // Helper function to test instructions
    fn test_instructions(expected: &[Instructions], actual: &Instructions) {
        let mut concatted = vec![];
        for instr in expected {
            concatted.extend_from_slice(instr);
        }
        
        assert_eq!(
            concatted.len(),
            actual.len(),
            "Wrong instruction length. Expected {}, got {}",
            concatted.len(),
            actual.len()
        );
        
        for (i, (expected, actual)) in concatted.iter().zip(actual.iter()).enumerate() {
            assert_eq!(
                expected, actual,
                "Wrong instruction at position {}. Expected {}, got {}",
                i, expected, actual
            );
        }
    }
    
    // Helper function to test constants
    fn test_constants(expected: &[Object], actual: &[Object]) {
        assert_eq!(
            expected.len(),
            actual.len(),
            "Wrong number of constants. Expected {}, got {}",
            expected.len(),
            actual.len()
        );
        
        for (i, (expected, actual)) in expected.iter().zip(actual.iter()).enumerate() {
            match (expected, actual) {
                (Object::Integer(e), Object::Integer(a)) => {
                    assert_eq!(e, a, "Wrong integer at position {}. Expected {}, got {}", i, e, a);
                }
                (Object::Float(e), Object::Float(a)) => {
                    assert_eq!(e, a, "Wrong float at position {}. Expected {}, got {}", i, e, a);
                }
                (Object::String(e), Object::String(a)) => {
                    assert_eq!(e, a, "Wrong string at position {}. Expected {}, got {}", i, e, a);
                }
                _ => panic!(
                    "Wrong constant type at position {}. Expected {:?}, got {:?}",
                    i, expected, actual
                ),
            }
        }
    }
} 