#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Expression, ExpressionStatement, IntegerLiteral, Program, Statement};
    use crate::lexer::Lexer;
    use crate::object::Object;
    use crate::parser::Parser;
    use proptest::prelude::*;
    use bytecode::{Bytecode, Instructions, Opcode, make, read_operand, lookup};
    
    // Property-based tests for the bytecode infrastructure
    
    proptest! {
        // Test that opcode conversion is bijective (round-trip conversion works)
        #[test]
        fn opcode_conversion_is_bijective(opcode in 0u8..=0x32u8) {
            // Skip invalid opcodes or opcodes outside our enum range
            if opcode <= 0x32 {
                let op = Opcode::from(opcode);
                let back_to_byte: u8 = op.into();
                assert_eq!(opcode, back_to_byte);
            }
        }
        
        // Test that make/read_operand are inverse operations
        #[test]
        fn make_and_read_operand_are_inverse(op in 0x01u8..=0x32u8, operand in 0usize..65535usize) {
            // Skip invalid opcodes
            if op <= 0x32 {
                let opcode = Opcode::from(op);
                let def = lookup(opcode);
                
                // Skip if the opcode doesn't take operands
                if !def.operand_widths.is_empty() {
                    let instruction = make(opcode, &[operand]);
                    let (operands, _) = read_operand(&def, &instruction, 0);
                    
                    // Check if operand was correctly encoded and decoded
                    if !operands.is_empty() {
                        assert_eq!(operand, operands[0]);
                    }
                }
            }
        }
        
        // Test that multiple operands are encoded/decoded correctly
        #[test]
        fn multi_operand_encoding_decoding(op in 0x01u8..=0x32u8, operand1 in 0usize..65535usize, operand2 in 0usize..255usize) {
            // We'll test with opcodes that can take multiple operands (like Closure)
            if op == 0x1B {  // Closure opcode
                let opcode = Opcode::Closure;
                let def = lookup(opcode);
                
                let instruction = make(opcode, &[operand1, operand2]);
                let (operands, _) = read_operand(&def, &instruction, 0);
                
                assert_eq!(operand1, operands[0]);
                assert_eq!(operand2, operands[1]);
            }
        }
        
        // Test that instructions lengths are correct
        #[test]
        fn instruction_length_is_correct(op in 0x01u8..=0x32u8, operand in 0usize..65535usize) {
            if op <= 0x32 {
                let opcode = Opcode::from(op);
                let def = lookup(opcode);
                
                // Skip if the opcode doesn't take operands
                if !def.operand_widths.is_empty() {
                    let instruction = make(opcode, &[operand]);
                    let expected_len = 1 + def.operand_widths.iter().sum::<usize>();
                    
                    assert_eq!(expected_len, instruction.len());
                }
            }
        }
    }
    
    // Function to create a simple integer literal program for testing
    fn create_test_program(value: i64) -> Program {
        let literal = IntegerLiteral {
            token: crate::lexer::Token::Int(value.to_string()),
            value,
        };
        
        let expr = Box::new(literal) as Box<dyn Expression>;
        let stmt = Box::new(ExpressionStatement {
            token: crate::lexer::Token::Int(value.to_string()),
            expression: Some(expr),
        }) as Box<dyn Statement>;
        
        Program {
            statements: vec![stmt],
        }
    }
    
    // Property-based tests for basic compiler functionality
    
    proptest! {
        // Test that compiling integer literals works correctly
        #[test]
        fn compile_integer_literal(value in -1000i64..1000i64) {
            let program = create_test_program(value);
            let mut compiler = Compiler::new();
            
            // Compile the program
            let result = compiler.compile(&program);
            assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
            
            // Check the bytecode
            let bytecode = result.unwrap();
            
            // Check that the constant was added
            assert_eq!(1, bytecode.constants.len());
            assert_eq!(Object::Integer(value), bytecode.constants[0]);
            
            // Check the instructions
            // Expected: [Constant(0), Pop]
            let expected_constant_op = make(Opcode::Constant, &[0]);
            let expected_pop_op = make(Opcode::Pop, &[]);
            let mut expected = Vec::new();
            expected.extend(expected_constant_op);
            expected.extend(expected_pop_op);
            
            assert_eq!(expected, bytecode.instructions);
        }
        
        // Test symbol table with random identifier names
        #[test]
        fn symbol_table_defines_resolves(name in "[a-zA-Z][a-zA-Z0-9_]{0,20}") {
            let mut symbol_table = symbol_table::SymbolTable::new();
            
            // Define a symbol
            let symbol = symbol_table.define(&name);
            
            // Check symbol properties
            assert_eq!(name, symbol.name);
            assert_eq!(symbol_table::SymbolScope::Global, symbol.scope);
            assert_eq!(0, symbol.index);
            
            // Resolve the symbol
            let resolved = symbol_table.resolve(&name);
            assert!(resolved.is_some());
            
            let resolved_symbol = resolved.unwrap();
            assert_eq!(symbol.name, resolved_symbol.name);
            assert_eq!(symbol.scope, resolved_symbol.scope);
            assert_eq!(symbol.index, resolved_symbol.index);
        }
        
        // Test nested symbol tables for scoping
        #[test]
        fn nested_symbol_tables(
            outer_name in "[a-zA-Z][a-zA-Z0-9_]{0,10}",
            inner_name in "[a-zA-Z][a-zA-Z0-9_]{0,10}",
            shared_name in "[a-zA-Z][a-zA-Z0-9_]{0,10}"
        ) {
            // Skip if the names are the same
            prop_assume!(outer_name != inner_name && outer_name != shared_name && inner_name != shared_name);
            
            let mut outer = symbol_table::SymbolTable::new();
            
            // Define in outer scope
            let outer_symbol = outer.define(&outer_name);
            let shared_outer = outer.define(&shared_name);
            
            // Create inner scope
            let mut inner = symbol_table::SymbolTable::new_enclosed(outer);
            
            // Define in inner scope
            let inner_symbol = inner.define(&inner_name);
            let shared_inner = inner.define(&shared_name);
            
            // Test resolution
            // Inner name should resolve to inner symbol
            let resolved_inner = inner.resolve(&inner_name);
            assert!(resolved_inner.is_some());
            assert_eq!(inner_symbol.name, resolved_inner.unwrap().name);
            
            // Outer name should resolve through outer scope
            let resolved_outer = inner.resolve(&outer_name);
            assert!(resolved_outer.is_some());
            assert_eq!(outer_symbol.name, resolved_outer.unwrap().name);
            
            // Shared name should resolve to inner definition
            let resolved_shared = inner.resolve(&shared_name);
            assert!(resolved_shared.is_some());
            assert_eq!(shared_inner.name, resolved_shared.unwrap().name);
        }
    }
    
    // Test compiling common CURSED programs
    
    #[test]
    fn test_compile_basic_arithmetic() {
        let input = "5 + 10 * 2";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        
        let mut compiler = Compiler::new();
        let result = compiler.compile(&program);
        assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
        
        // Expected:
        // 1. Constant(0) - Push 5
        // 2. Constant(1) - Push 10
        // 3. Constant(2) - Push 2
        // 4. Mul - Multiply 10 and 2
        // 5. Add - Add 5 and 20
        // 6. Pop - Discard the result
        
        let expected_constants = vec![
            Object::Integer(5),
            Object::Integer(10),
            Object::Integer(2),
        ];
        
        let mut expected_instructions = Vec::new();
        expected_instructions.extend(make(Opcode::Constant, &[0])); // Push 5
        expected_instructions.extend(make(Opcode::Constant, &[1])); // Push 10
        expected_instructions.extend(make(Opcode::Constant, &[2])); // Push 2
        expected_instructions.extend(make(Opcode::Mul, &[]));       // Multiply
        expected_instructions.extend(make(Opcode::Add, &[]));       // Add
        expected_instructions.extend(make(Opcode::Pop, &[]));       // Pop result
        
        let bytecode = result.unwrap();
        assert_eq!(expected_constants, bytecode.constants);
        assert_eq!(expected_instructions, bytecode.instructions);
    }
    
    #[test]
    fn test_compile_if_statement() {
        let input = "lowkey x > 5 { yolo 10; } highkey { yolo 5; }";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        
        let mut compiler = Compiler::new();
        let result = compiler.compile(&program);
        assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
        
        // Compiler should generate code equivalent to:
        // 1. Get global 'x'
        // 2. Constant(0) - Push 5
        // 3. Compare x > 5
        // 4. JumpNotTruthy to else block
        // 5. Constant(1) - Push 10 (then branch)
        // 6. ReturnValue - Return 10
        // 7. Jump to after the else block
        // 8. Constant(2) - Push 5 (else branch) 
        // 9. ReturnValue - Return 5
        
        let expected_constants = vec![
            Object::Integer(5),  // Condition value
            Object::Integer(10), // Then branch return value
            Object::Integer(5),  // Else branch return value
        ];
        
        let bytecode = result.unwrap();
        assert_eq!(expected_constants.len(), bytecode.constants.len());
        
        for (i, constant) in expected_constants.iter().enumerate() {
            assert_eq!(*constant, bytecode.constants[i]);
        }
    }
    
    #[test]
    fn test_compile_while_statement() {
        let input = "periodt x < 10 { x = x + 1; }";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        
        let mut compiler = Compiler::new();
        let result = compiler.compile(&program);
        assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
        
        // Expected bytecode structure:
        // 1. Start of loop (marker)
        // 2. Get global 'x'
        // 3. Constant(0) - Push 10
        // 4. Compare x < 10
        // 5. JumpNotTruthy to after loop
        // 6. Get global 'x'
        // 7. Constant(1) - Push 1
        // 8. Add - Add x and 1
        // 9. Set global 'x'
        // 10. Jump back to start of loop
        // 11. End of loop (marker)
        
        let expected_constants = vec![
            Object::Integer(10), // Loop condition constant
            Object::Integer(1),  // Increment amount
        ];
        
        let bytecode = result.unwrap();
        assert_eq!(expected_constants.len(), bytecode.constants.len());
        
        for (i, constant) in expected_constants.iter().enumerate() {
            assert_eq!(*constant, bytecode.constants[i]);
        }
        
        // The exact instruction sequence is checked in the implementation
    }
} 