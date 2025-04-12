//! Test cases for array bounds checking in the CURSED compiler.
//!
//! These tests verify that the compiler correctly implements runtime bounds
//! checking for array access operations, ensuring memory safety.  

// TODO: These tests need to be completely rewritten to use the new JIT API
// For now, they will remain ignored until they can be properly updated
use cursed::lexer::Lexer;
use cursed::parser::Parser;

#[test]
#[ignore = "JIT API has changed - this test needs a complete rewrite with the new API"]
fn test_array_access_in_bounds() {
    // This test verifies that normal in-bounds array access works correctly
    let input = r#"
    function test_bounds() thicc {
        let arr = [1, 2, 3, 4, 5];
        return arr[2]; // Should return 3 (0-indexed)
    }
    "#;

    // FIXME: Implement with updated JIT API
    // Pseudo code for reference:
    //
    // let mut lexer = Lexer::new(input);
    // let mut parser = Parser::new(&mut lexer).unwrap();
    // let program = parser.parse_program().unwrap();
    //
    // let context = Context::create();
    // let jit = create_jit_with_modern_api(context, program, "test_bounds");
    // let result = jit.run();
    //
    // assert_eq!(result, 3);
}

#[test]
#[ignore = "JIT API has changed - this test needs a complete rewrite with the new API"]
#[should_panic(expected = "Array index out of bounds")]
fn test_array_access_out_of_bounds() {
    // This test verifies that out-of-bounds array access is detected and causes a panic
    let input = r#"
    function test_out_of_bounds() thicc {
        let arr = [1, 2, 3];
        return arr[5]; // This should trigger a bounds check failure
    }
    "#;

    // FIXME: Implement with updated JIT API
    // Pseudo code for reference:
    //
    // let mut lexer = Lexer::new(input);
    // let mut parser = Parser::new(&mut lexer).unwrap();
    // let program = parser.parse_program().unwrap();
    //
    // let context = Context::create();
    // let jit = create_jit_with_modern_api(context, program, "test_out_of_bounds");
    // // This should panic with an "Array index out of bounds" message
    // let _ = jit.run();
}

#[test]
#[ignore = "JIT API has changed - this test needs a complete rewrite with the new API"]
#[should_panic(expected = "Array index out of bounds")]
fn test_negative_array_index() {
    // This test verifies that negative array indices are rejected
    let input = r#"
    function test_negative_index() thicc {
        let arr = [10, 20, 30];
        return arr[-1]; // Negative indices should be rejected
    }
    "#;

    // FIXME: Implement with updated JIT API
    // Pseudo code for reference:
    //
    // let mut lexer = Lexer::new(input);
    // let mut parser = Parser::new(&mut lexer).unwrap();
    // let program = parser.parse_program().unwrap();
    //
    // let context = Context::create();
    // let jit = create_jit_with_modern_api(context, program, "test_negative_index");
    // // This should panic with an "Array index out of bounds" message
    // let _ = jit.run();
}

#[test]
#[ignore = "JIT API has changed - this test needs a complete rewrite with the new API"]
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

    // FIXME: Implement with updated JIT API
    // Pseudo code for reference:
    //
    // let mut lexer = Lexer::new(input);
    // let mut parser = Parser::new(&mut lexer).unwrap();
    // let program = parser.parse_program().unwrap();
    //
    // let context = Context::create();
    // let jit = create_jit_with_modern_api(context, program, "test_dynamic_index");
    //
    // // Test valid indices
    // for idx in 0..5 {
    //     let expected = (idx + 1) * 100;
    //     let result = jit.run_with_args(&[idx]);
    //     assert_eq!(result, expected);
    // }
    //
    // // Test negative index (handled gracefully in this function)
    // let result = jit.run_with_args(&[-1]);
    // assert_eq!(result, -1);
    //
    // // Test out-of-bounds index (handled gracefully in this function)
    // let result = jit.run_with_args(&[5]);
    // assert_eq!(result, -1);
}