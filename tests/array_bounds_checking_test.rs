//! Test cases for array bounds checking in the CURSED compiler.
//!
//! These tests verify that the compiler correctly implements runtime bounds
//! checking for array access operations, ensuring memory safety.  

use cursed::object::Object;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::code::compile_ast_to_bytecode;
use cursed::codegen::jit::JitCompiler;

#[test]
fn test_array_access_in_bounds() {
    // This test verifies that normal in-bounds array access works correctly
    let input = r#"
    function test_bounds() thicc {
        let arr = [1, 2, 3, 4, 5];
        return arr[2]; // Should return 3 (0-indexed)
    }
    "#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program().unwrap();
    
    let mut jit = JitCompiler::new();
    let result = jit.compile_and_run(&program, "test_bounds").unwrap();
    
    // Check that accessing a valid index returns the correct value
    assert_eq!(result.as_thicc(), 3);
}

#[test]
#[should_panic(expected = "Array index out of bounds")]
fn test_array_access_out_of_bounds() {
    // This test verifies that out-of-bounds array access is detected and causes a panic
    let input = r#"
    function test_out_of_bounds() thicc {
        let arr = [1, 2, 3];
        return arr[5]; // This index is out of bounds
    }
    "#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program().unwrap();
    
    let mut jit = JitCompiler::new();
    // This should panic with an "Array index out of bounds" message
    let _ = jit.compile_and_run(&program, "test_out_of_bounds").unwrap();
}

#[test]
#[should_panic(expected = "Array index out of bounds")]
fn test_negative_array_index() {
    // This test verifies that negative array indices are rejected
    let input = r#"
    function test_negative_index() thicc {
        let arr = [10, 20, 30];
        return arr[-1]; // Negative indices should be rejected
    }
    "#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program().unwrap();
    
    let mut jit = JitCompiler::new();
    // This should panic with an "Array index out of bounds" message
    let _ = jit.compile_and_run(&program, "test_negative_index").unwrap();
}

#[test]
fn test_dynamic_index_calculation() {
    // This test verifies bounds checking with dynamically calculated indices
    let input = r#"
    function test_dynamic_index(thicc idx) thicc {
        let arr = [100, 200, 300, 400, 500];
        if idx >= 0 && idx < 5 {
            return arr[idx];
        } else {
            return -1; // Indicate out of bounds without panicking
        }
    }
    "#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program().unwrap();
    
    let mut jit = JitCompiler::new();
    
    // Test valid indices
    for idx in 0..5 {
        let expected = (idx + 1) * 100;
        let result = jit.compile_and_run_with_args(&program, "test_dynamic_index", &[Object::Thicc(idx as i64)]).unwrap();
        assert_eq!(result.as_thicc(), expected as i64);
    }
    
    // Test invalid indices
    let result = jit.compile_and_run_with_args(&program, "test_dynamic_index", &[Object::Thicc(-1)]).unwrap();
    assert_eq!(result.as_thicc(), -1);
    
    let result = jit.compile_and_run_with_args(&program, "test_dynamic_index", &[Object::Thicc(5)]).unwrap();
    assert_eq!(result.as_thicc(), -1);
}