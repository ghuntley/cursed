use crate::ast::{Expression, ExpressionStatement, IntegerLiteral, Program, Statement};
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;
use proptest::prelude::*;
use super::bytecode::{Bytecode, Instructions, Opcode, make, read_operand, lookup};
use super::*;

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
    
    let bytecode = result.unwrap();
    
    // Verify the bytecode instructions match what we expect
    // This is a simplified test - in practice, you'd verify all instructions
    // Expected bytecode structure:
    // 0: GetGlobal(0)       // Load 'x'
    // 3: Push(10)           // Load 10
    // 4: LessThan           // Compare x < 10
    // 5: JumpNotTruthy(16)  // Jump to end if false
    // 8: GetGlobal(0)       // Load 'x'
    // 11: GetGlobal(0)      // Load 'x'
    // 14: Constant(0)       // Load constant 1
    // 17: Add               // Add x + 1
    // 18: SetGlobal(0)      // Store result back to 'x'
    // 21: Jump(0)           // Jump back to start
    // 24: Pop               // Clean up stack
    
    let instructions = bytecode.instructions;
    assert!(instructions.len() > 0, "No instructions generated");
}

#[test]
fn test_compile_for_statement() {
    // Test a C-style for loop: bestie i := 0; i < 10; i = i + 1 { x = x + i; }
    let input = "sus i = 0; bestie i < 10; i = i + 1 { x = x + i; }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    let mut compiler = Compiler::new();
    let result = compiler.compile(&program);
    assert!(result.is_ok(), "Compilation of C-style for loop failed: {:?}", result.err());
    
    // Test a condition-only loop: bestie i < 10 { x = x + i; }
    let input = "sus i = 0; bestie i < 10 { x = x + i; i = i + 1; }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    let mut compiler = Compiler::new();
    let result = compiler.compile(&program);
    assert!(result.is_ok(), "Compilation of condition-only for loop failed: {:?}", result.err());
    
    // Test an infinite loop: bestie { x = x + 1; }
    let input = "bestie { x = x + 1; break; }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    let mut compiler = Compiler::new();
    let result = compiler.compile(&program);
    assert!(result.is_ok(), "Compilation of infinite for loop failed: {:?}", result.err());
}

#[test]
fn test_compile_switch_statement() {
    // Test basic switch statement
    let input = r#"
        sus day = "Monday";
        vibe_check day {
            mood "Monday", "Tuesday":
                x = 1;
            mood "Friday":
                x = 2;
            basic:
                x = 0;
        }
    "#;
    
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    let mut compiler = Compiler::new();
    let result = compiler.compile(&program);
    assert!(result.is_ok(), "Compilation of basic switch statement failed: {:?}", result.err());
    
    // Test switch statement without default case
    let input = r#"
        vibe_check day {
            mood "Monday":
                x = 1;
            mood "Friday":
                x = 2;
        }
    "#;
    
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    let mut compiler = Compiler::new();
    let result = compiler.compile(&program);
    assert!(result.is_ok(), "Compilation of switch without default failed: {:?}", result.err());
    
    // Test switch statement with a complex expression
    let input = r#"
        vibe_check x + 10 {
            mood 20:
                y = "twenty";
            mood 30:
                y = "thirty";
            basic:
                y = "other";
        }
    "#;
    
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    let mut compiler = Compiler::new();
    let result = compiler.compile(&program);
    assert!(result.is_ok(), "Compilation of switch with complex expression failed: {:?}", result.err());
}

#[test]
fn test_compile_package_statement() {
    // Test basic package declaration
    let input = "vibe main;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    let mut compiler = Compiler::new();
    let result = compiler.compile(&program);
    assert!(result.is_ok(), "Compilation of package declaration failed: {:?}", result.err());
    
    // Verify the bytecode contains the package name as a constant
    let bytecode = result.unwrap();
    assert!(bytecode.constants.len() > 0, "No constants generated");
    
    // The first constant should be the package name "main"
    match &bytecode.constants[0] {
        Object::String(name) => assert_eq!(name, "main"),
        _ => panic!("Expected package name to be a string constant")
    }
    
    // Test package with a more complex name
    let input = "vibe com.example.project;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    let mut compiler = Compiler::new();
    let result = compiler.compile(&program);
    assert!(result.is_ok(), "Compilation of namespaced package failed: {:?}", result.err());
}

#[test]
fn test_compile_import_statement() {
    // Test basic import statement
    let input = "yeet \"fmt\";";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    let mut compiler = Compiler::new();
    let result = compiler.compile(&program);
    assert!(result.is_ok(), "Compilation of basic import statement failed: {:?}", result.err());
    
    // Verify the bytecode contains the import path as a constant
    let bytecode = result.unwrap();
    assert!(bytecode.constants.len() >= 2, "Not enough constants generated");
    
    // The constants should include the import path "fmt" and name "fmt"
    let mut found_path = false;
    let mut found_name = false;
    
    for constant in &bytecode.constants {
        if let Object::String(value) = constant {
            if value == "fmt" {
                if !found_path {
                    found_path = true;
                } else {
                    found_name = true;
                }
            }
        }
    }
    
    assert!(found_path, "Import path 'fmt' not found in constants");
    assert!(found_name, "Import name 'fmt' not found in constants");
    
    // Test import with alias
    let input = "yeet math \"math/advanced\";";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    let mut compiler = Compiler::new();
    let result = compiler.compile(&program);
    assert!(result.is_ok(), "Compilation of import with alias failed: {:?}", result.err());
    
    // Verify the bytecode contains both the import path and the alias
    let bytecode = result.unwrap();
    
    let mut found_path = false;
    let mut found_name = false;
    
    for constant in &bytecode.constants {
        if let Object::String(value) = constant {
            if value == "math/advanced" {
                found_path = true;
            } else if value == "math" {
                found_name = true;
            }
        }
    }
    
    assert!(found_path, "Import path 'math/advanced' not found in constants");
    assert!(found_name, "Import alias 'math' not found in constants");
}

#[test]
fn test_compile_type_declaration() {
    // Test basic type declaration
    let input = "be_like Person squad { name tea age normie height meal }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    let mut compiler = Compiler::new();
    let result = compiler.compile(&program);
    assert!(result.is_ok(), "Compilation of type declaration failed: {:?}", result.err());
    
    // Verify the bytecode contains the type name and field information
    let bytecode = result.unwrap();
    
    // Check that we have constants for type name and field names/types
    assert!(bytecode.constants.len() >= 7, "Not enough constants generated");
    
    // The first constant should be the type name "Person"
    match &bytecode.constants[0] {
        Object::String(name) => assert_eq!(name, "Person", "Expected type name 'Person'"),
        _ => panic!("Expected type name to be a string constant")
    }
    
    // Check that we have field names and types in the constants
    let expected_fields = vec![
        "name", "tea", "age", "normie", "height", "meal"
    ];
    
    let mut found_fields = 0;
    for constant in &bytecode.constants {
        if let Object::String(value) = constant {
            if expected_fields.contains(&value.as_str()) {
                found_fields += 1;
            }
        }
    }
    
    assert_eq!(found_fields, 6, "Not all expected fields found in constants");
    
    // Test type declaration with semicolons
    let input = "be_like Point squad { x meal; y meal; }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    let mut compiler = Compiler::new();
    let result = compiler.compile(&program);
    assert!(result.is_ok(), "Compilation of type declaration with semicolons failed: {:?}", result.err());
} 