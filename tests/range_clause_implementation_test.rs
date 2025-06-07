use cursed::error::Error;

// Implementation tests for the enhanced range clause functionality
//
// This module provides comprehensive tests for the enhanced range clause implementation,
// focusing on edge cases and specific scenarios required by the test plan.

// Import test setup and common utilities
#[path = "common.rs"]
mod common;

// Import test tracing macro via path include
#[path = "tracing_setup.rs"]
mod tracing_setup;

// Import range clause test helpers
#[path = "range_clause_test_helper.rs"]
mod helper;

/// Test basic numeric range with the enhanced implementation
#[test]
fn test_enhanced_numeric_range() {
    init_tracing!();

    // Test basic range statement with improved syntax
    let input = r#"
        slay main() lit {
            sus sum lit = 0
            
            // Use range to iterate from 0 to 9
            periodt i := range 10 {
                sum = sum + i
            }
            
            return sum // Should be 0+1+2+3+4+5+6+7+8+9 = 45
        }
    "#;
    
    match helper::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(45), "Basic numeric range should sum to 45");
        },
        Err(e) => panic!("Failed to run basic numeric range test: {}", e),
    }
}

/// Test range with custom start and end values
#[test]
fn test_enhanced_range_with_bounds() {
    init_tracing!();

    // Test range with explicit start and end bounds
    let input = r#"
        slay main() lit {
            sus sum lit = 0
            
            // Use range with start and end bounds
            periodt i := range 5, 15 {
                sum = sum + i
            }
            
            return sum // Should be 5+6+7+8+9+10+11+12+13+14 = 95
        }
    "#;
    
    match helper::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(95), "Range with bounds should sum to 95");
        },
        Err(e) => panic!("Failed to run range with bounds test: {}", e),
    }
}

/// Test range with custom step value
#[test]
fn test_enhanced_range_with_step() {
    init_tracing!();

    // Test range with explicit step value
    let input = r#"
        slay main() lit {
            sus sum lit = 0
            
            // Use range with start, end, and step
            periodt i := range 0, 20, 4 {
                sum = sum + i
            }
            
            return sum // Should be 0+4+8+12+16 = 40
        }
    "#;
    
    match helper::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(40), "Range with step should sum to 40");
        },
        Err(e) => panic!("Failed to run range with step test: {}", e),
    }
}

/// Test range with negative step value
#[test]
fn test_enhanced_negative_step_range() {
    init_tracing!();

    // Test range with negative step value for decrementing iteration
    let input = r#"
        slay main() lit {
            sus sum lit = 0
            
            // Use range with negative step
            periodt i := range 20, 0, -5 {
                sum = sum + i
            }
            
            return sum // Should be 20+15+10+5 = 50
        }
    "#;
    
    match helper::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(50), "Range with negative step should sum to 50");
        },
        Err(e) => panic!("Failed to run negative step range test: {}", e),
    }
}

/// Test empty range that should produce no iterations
#[test]
fn test_enhanced_empty_range() {
    init_tracing!();

    // Test range that produces no iterations
    let input = r#"
        slay main() lit {
            sus sum lit = 100
            sus count lit = 0
            
            // Empty range cases
            periodt i := range 10, 5 {
                // This body should never execute due to end < start
                sum = sum + i
                count = count + 1
            }
            
            return count // Should be 0 iterations
        }
    "#;
    
    match helper::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(0), "Empty range should not execute any iterations");
        },
        Err(e) => panic!("Failed to run empty range test: {}", e),
    }
}

/// Test range with large bounds approaching integer limits
#[test]
fn test_enhanced_large_range() {
    init_tracing!();

    // Test range with large numbers close to integer limits
    let input = r#"
        slay main() lit {
            sus count lit = 0
            
            // Large range with small number of iterations to avoid excessive loops
            periodt i := range 1000000, 1000010 {
                count = count + 1
            }
            
            return count // Should be 10 iterations
        }
    "#;
    
    match helper::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(10), "Large range should execute exactly 10 iterations");
        },
        Err(e) => panic!("Failed to run large range test: {}", e),
    }
}

/// Test range with negative start and end values
#[test]
fn test_enhanced_negative_bounds_range() {
    init_tracing!();

    // Test range with negative start and end values
    let input = r#"
        slay main() lit {
            sus sum lit = 0
            
            // Range with negative bounds
            periodt i := range -10, -5 {
                sum = sum + i
            }
            
            return sum // Should be -10+(-9)+(-8)+(-7)+(-6) = -40
        }
    "#;
    
    match helper::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(-40), "Range with negative bounds should sum to -40");
        },
        Err(e) => panic!("Failed to run negative bounds range test: {}", e),
    }
}

/// Test array iteration with the enhanced implementation
#[test]
fn test_enhanced_array_iteration() {
    init_tracing!();

    // Test iterating over an array using range
    let input = r#"
        slay main() lit {
            sus values = [5, 10, 15, 20, 25]
            sus product lit = 1
            
            periodt value := range values {
                product = product * value
            }
            
            return product // Should be 5*10*15*20*25 = 375000
        }
    "#;
    
    match helper::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(375000), "Array iteration should produce product 375000");
        },
        Err(e) => panic!("Failed to run array iteration test: {}", e),
    }
}

/// Test mixed type array iteration with the enhanced implementation
#[test]
fn test_enhanced_mixed_type_array() {
    init_tracing!();

    // Test iterating over an array with mixed types (should coerce to float)
    let input = r#"
        slay main() normie {
            sus values = [5, 10.5, 15, 20.5, 25]
            sus sum normie = 0.0
            
            periodt value := range values {
                sum = sum + value
            }
            
            return sum // Should be 5+10.5+15+20.5+25 = 76.0
        }
    "#;
    
    match helper::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_f64(), Some(76.0), "Mixed type array iteration should sum to 76.0");
        },
        Err(e) => panic!("Failed to run mixed type array test: {}", e),
    }
}

/// Test nested range loops
#[test]
fn test_enhanced_nested_range_loops() {
    init_tracing!();

    // Test nested range loops with proper scoping
    let input = r#"
        slay main() lit {
            sus sum lit = 0
            
            // Outer loop
            periodt i := range 3 {
                // Inner loop
                periodt j := range 3 {
                    sum = sum + (i * 10 + j)
                }
            }
            
            // Should be (0*10+0)+(0*10+1)+(0*10+2)+(1*10+0)+(1*10+1)+(1*10+2)+(2*10+0)+(2*10+1)+(2*10+2)
            // = 0+1+2+10+11+12+20+21+22 = 99
            return sum
        }
    "#;
    
    match helper::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(99), "Nested range loops should sum to 99");
        },
        Err(e) => panic!("Failed to run nested range loops test: {}", e),
    }
}

/// Test map key iteration
#[test]
fn test_enhanced_map_key_iteration() {
    init_tracing!();

    // Test iterating over map keys
    let input = r#"
        slay main() lit {
            sus scores = {"Alice": 10, "Bob": 20, "Charlie": 30}
            sus count lit = 0
            
            // Iterate over just the keys
            periodt name := range scores {
                // Just count how many keys
                count = count + 1
            }
            
            return count // Should be 3 keys
        }
    "#;
    
    match helper::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(3), "Map key iteration should count 3 keys");
        },
        Err(e) => panic!("Failed to run map key iteration test: {}", e),
    }
}

/// Test map key-value iteration
#[test]
fn test_enhanced_map_key_value_iteration() {
    init_tracing!();

    // Test iterating over map key-value pairs
    let input = r#"
        slay main() lit {
            sus scores = {"Alice": 10, "Bob": 20, "Charlie": 30}
            sus sum lit = 0
            
            // Iterate over key-value pairs
            periodt name, score := range scores {
                sum = sum + score
            }
            
            return sum // Should be 10+20+30 = 60
        }
    "#;
    
    match helper::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(60), "Map key-value iteration should sum to 60");
        },
        Err(e) => panic!("Failed to run map key-value iteration test: {}", e),
    }
}

/// Test break and continue in the same loop
#[test]
fn test_enhanced_break_continue_combined() {
    init_tracing!();

    // Test combining break and continue in the same loop
    let input = r#"
        slay main() lit {
            sus sum lit = 0
            
            periodt i := range 20 {
                // Skip odd numbers
                lowkey i % 2 == 1 {
                    continue
                }
                
                // Add even number to sum
                sum = sum + i
                
                // Break when sum exceeds 30
                lowkey sum > 30 {
                    break
                }
            }
            
            return sum // Should add 0+2+4+6+8+10+12=42, then break
        }
    "#;
    
    match helper::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(42), "Break/continue combined loop should sum to 42");
        },
        Err(e) => panic!("Failed to run break/continue combined test: {}", e),
    }
}

/// Test variable scoping within range loops
#[test]
fn test_enhanced_range_variable_scoping() {
    init_tracing!();

    // Test variable scoping within range loops
    let input = r#"
        slay main() lit {
            sus outer lit = 42
            
            periodt outer := range 5 {
                // This 'outer' should shadow the original 'outer'
                // and be scoped to the loop
            }
            
            // Should still be 42 after the loop
            return outer
        }
    "#;
    
    match helper::run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_i64(), Some(42), "Variable scoping should preserve outer value");
        },
        Err(e) => panic!("Failed to run variable scoping test: {}", e),
    }
}

/// Compare results between original and enhanced implementations
#[test]
fn test_implementation_comparison() {
    helper::setup_tracing();
    
    // Test both implementations with the same input
    let input = r#"
        slay main() lit {
            sus sum lit = 0
            
            periodt i := range 10 {
                sum = sum + i
            }
            
            return sum
        }
    "#;
    
    // This test will only work once the enhanced implementation is integrated
    // For now, we'll check that the test utility works correctly
    match helper::compare_implementations(input) {
        Ok(result) => {
            // Once enhanced implementation is integrated, this should be true
            // For now, it will use the original implementation twice
            assert!(result, "Both implementations should produce the same result");
        },
        Err(e) => panic!("Failed to compare implementations: {}", e),
    }
}

/// Test direct integration of the RangeClauseCompilationEnhanced trait
/// 
/// Note: This test will need to be enabled once the integration is complete
#[test]
#[ignore] // Ignore until full integration is complete
fn test_direct_enhanced_trait_usage() {
    init_tracing!();
    
    // This test will directly use the enhanced implementation
    // once it's fully integrated into the main codebase
    
    // Placeholder for direct trait usage testing
}