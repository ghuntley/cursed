use cursed::error::Error;
use cursed::ast::BoxedStatement;
use crate::common;

// Enhanced range clause implementation tests
//
// This module contains tests for the enhanced range clause implementation
// that includes proper error handling and improved LLVM builder operations.
//
// These tests focus on verifying the correctness of the implementation without
// introducing conflicts with the original implementation.


#[path = "common.rs"]
mod common;

#[path = "range_clause_test_helper.rs"]
mod helper;

#[test]
fn test_numeric_range_iteration() {
    // Initialize test tracing
    helper::setup_tracing();
    
    // Test basic range statement (for i := range 5)
    let input = r#"
        slay main() lit {
            sus sum lit = 0
            
            periodt i := range 5 {
                sum = sum + i
            }
            
            return sum // Should be 0+1+2+3+4 = 10
        }
    "#;
    
    // For now, we can use the original implementation via common helper
    // Later we'll switch to the enhanced implementation
    match common::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(10);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }

    // Once the enhanced implementation is fully integrated,
    // we can use this instead:
    // match helper::run_enhanced_impl(input) {
    //     Ok(result) => {
    //         assert_eq!(result.as_i64(), Some(10);
    //     },
    //     Err(e) => panic!("Failed to run test: {}", e),
    // }
}

#[test]
fn test_range_with_start_and_end() {
    // Initialize test tracing
    common::tracing::setup();
    
    // Test range with start and end (for i := range 2, 8)
    let input = r#"
        slay main() lit {
            sus sum lit = 0
            
            periodt i := range 2, 8 {
                sum = sum + i
            }
            
            return sum // Should be 2+3+4+5+6+7 = 27
        }
    "#;
    
    match common::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(27);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_range_with_step() {
    // Initialize test tracing
    common::tracing::setup();
    
    // Test range with start, end, and step (for i := range 1, 10, 2)
    let input = r#"
        slay main() lit {
            sus sum lit = 0
            
            periodt i := range 1, 10, 2 {
                sum = sum + i
            }
            
            return sum // Should be 1+3+5+7+9 = 25
        }
    "#;
    
    match common::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(25);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_negative_step_range() {
    // Initialize test tracing
    common::tracing::setup();
    
    // Test range with negative step (for i := range 10, 1, -2)
    let input = r#"
        slay main() lit {
            sus sum lit = 0
            
            periodt i := range 10, 1, -2 {
                sum = sum + i
            }
            
            return sum // Should be 10+8+6+4+2 = 30
        }
    "#;
    
    match common::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(30);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_break_in_range_loop() {
    // Initialize test tracing
    common::tracing::setup();
    
    // Test breaking out of a range loop
    let input = r#"
        slay main() lit {
            sus sum lit = 0
            
            periodt i := range 10 {
                sum = sum + i
                lowkey i == 5 {
                    break
                }
            }
            
            return sum // Should be 0+1+2+3+4+5 = 15
        }
    "#;
    
    match common::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(15);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_continue_in_range_loop() {
    // Initialize test tracing
    common::tracing::setup();
    
    // Test continuing in a range loop
    let input = r#"
        slay main() lit {
            sus sum lit = 0
            
            periodt i := range 10 {
                lowkey i % 2 == 0 {
                    continue
                }
                sum = sum + i
            }
            
            return sum // Should be 1+3+5+7+9 = 25
        }
    "#;
    
    match common::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(25);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_array_iteration() {
    // Initialize test tracing
    common::tracing::setup();
    
    // Test iterating over an array
    let input = r#"
        slay main() lit {
            sus numbers = [10, 20, 30, 40, 50]
            sus sum lit = 0
            
            periodt num := range numbers {
                sum = sum + num
            }
            
            return sum // Should be 10+20+30+40+50 = 150
        }
    "#;
    
    match common::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(150);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_map_key_value_iteration() {
    // Initialize test tracing
    common::tracing::setup();
    
    // Test iterating over a map's key-value pairs
    let input = r#"
        slay main() lit {
            sus scores = {"Alice": 95, "Bob": 87, "Charlie": 92}
            sus sum lit = 0
            
            // Key-value iteration
            periodt name, score := range scores {
                sum = sum + score
            }
            
            return sum // Should be 95+87+92 = 274
        }
    "#;
    
    match common::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(274);
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}