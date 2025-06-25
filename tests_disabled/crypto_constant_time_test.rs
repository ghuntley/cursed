//! Comprehensive tests for constant-time cryptographic operations
//! 
//! Tests both correctness and timing properties of constant-time functions

use cursed::stdlib::packages::crypto_asymmetric::constant_time::*;
use cursed::stdlib::value::Value;
use std::time::{Duration, Instant};

#[test]
fn test_constant_time_compare_correctness() {
    // Test equal strings
    let args = vec![
        Value::String("hello".to_string()),
        Value::String("hello".to_string()),
    ];
    assert_eq!(constant_time_compare(args).unwrap(), Value::Bool(true));

    // Test unequal strings
    let args = vec![
        Value::String("hello".to_string()),
        Value::String("world".to_string()),
    ];
    assert_eq!(constant_time_compare(args).unwrap(), Value::Bool(false));

    // Test different length strings
    let args = vec![
        Value::String("hello".to_string()),
        Value::String("hell".to_string()),
    ];
    assert_eq!(constant_time_compare(args).unwrap(), Value::Bool(false));

    // Test empty strings
    let args = vec![
        Value::String("".to_string()),
        Value::String("".to_string()),
    ];
    assert_eq!(constant_time_compare(args).unwrap(), Value::Bool(true));

    // Test byte arrays
    let args = vec![
        Value::Array(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)]),
        Value::Array(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)]),
    ];
    assert_eq!(constant_time_compare(args).unwrap(), Value::Bool(true));

    let args = vec![
        Value::Array(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)]),
        Value::Array(vec![Value::Integer(1), Value::Integer(2), Value::Integer(4)]),
    ];
    assert_eq!(constant_time_compare(args).unwrap(), Value::Bool(false));
}

#[test]
fn test_constant_time_select_correctness() {
    // Test boolean condition with integers
    let args = vec![
        Value::Bool(true),
        Value::Integer(42),
        Value::Integer(24),
    ];
    assert_eq!(constant_time_select(args).unwrap(), Value::Integer(42));

    let args = vec![
        Value::Bool(false),
        Value::Integer(42),
        Value::Integer(24),
    ];
    assert_eq!(constant_time_select(args).unwrap(), Value::Integer(24));

    // Test integer condition (non-zero is true)
    let args = vec![
        Value::Integer(1),
        Value::Integer(42),
        Value::Integer(24),
    ];
    assert_eq!(constant_time_select(args).unwrap(), Value::Integer(42));

    let args = vec![
        Value::Integer(0),
        Value::Integer(42),
        Value::Integer(24),
    ];
    assert_eq!(constant_time_select(args).unwrap(), Value::Integer(24));

    // Test with boolean values
    let args = vec![
        Value::Bool(true),
        Value::Bool(true),
        Value::Bool(false),
    ];
    assert_eq!(constant_time_select(args).unwrap(), Value::Bool(true));
}

#[test]
fn test_constant_time_copy_correctness() {
    // Test conditional copy when condition is true
    let args = vec![
        Value::Bool(true),
        Value::Array(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)]),
        Value::Array(vec![Value::Integer(4), Value::Integer(5), Value::Integer(6)]),
    ];
    let result = constant_time_copy(args).unwrap();
    let expected = Value::Array(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)]);
    assert_eq!(result, expected);

    // Test conditional copy when condition is false
    let args = vec![
        Value::Bool(false),
        Value::Array(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)]),
        Value::Array(vec![Value::Integer(4), Value::Integer(5), Value::Integer(6)]),
    ];
    let result = constant_time_copy(args).unwrap();
    let expected = Value::Array(vec![Value::Integer(4), Value::Integer(5), Value::Integer(6)]);
    assert_eq!(result, expected);
}

#[test]
fn test_constant_time_clear_correctness() {
    let args = vec![
        Value::Array(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)]),
    ];
    let result = constant_time_clear(args).unwrap();
    let expected = Value::Array(vec![Value::Integer(0), Value::Integer(0), Value::Integer(0)]);
    assert_eq!(result, expected);

    // Test with string
    let args = vec![Value::String("secret".to_string())];
    let result = constant_time_clear(args).unwrap();
    if let Value::Array(arr) = result {
        for val in arr {
            assert_eq!(val, Value::Integer(0));
        }
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_constant_time_int_equal_correctness() {
    // Test equal integers
    let args = vec![Value::Integer(42), Value::Integer(42)];
    assert_eq!(constant_time_int_equal(args).unwrap(), Value::Integer(1));

    // Test unequal integers
    let args = vec![Value::Integer(42), Value::Integer(43)];
    assert_eq!(constant_time_int_equal(args).unwrap(), Value::Integer(0));

    // Test zero
    let args = vec![Value::Integer(0), Value::Integer(0)];
    assert_eq!(constant_time_int_equal(args).unwrap(), Value::Integer(1));

    // Test negative numbers
    let args = vec![Value::Integer(-42), Value::Integer(-42)];
    assert_eq!(constant_time_int_equal(args).unwrap(), Value::Integer(1));

    let args = vec![Value::Integer(-42), Value::Integer(42)];
    assert_eq!(constant_time_int_equal(args).unwrap(), Value::Integer(0));
}

#[test]
fn test_constant_time_less_than_correctness() {
    // Test a < b
    let args = vec![Value::Integer(5), Value::Integer(10)];
    assert_eq!(constant_time_less_than(args).unwrap(), Value::Integer(1));

    // Test a > b
    let args = vec![Value::Integer(10), Value::Integer(5)];
    assert_eq!(constant_time_less_than(args).unwrap(), Value::Integer(0));

    // Test a == b
    let args = vec![Value::Integer(5), Value::Integer(5)];
    assert_eq!(constant_time_less_than(args).unwrap(), Value::Integer(0));

    // Test with zero
    let args = vec![Value::Integer(0), Value::Integer(1)];
    assert_eq!(constant_time_less_than(args).unwrap(), Value::Integer(1));

    let args = vec![Value::Integer(1), Value::Integer(0)];
    assert_eq!(constant_time_less_than(args).unwrap(), Value::Integer(0));
}

#[test]
fn test_constant_time_key_derive_correctness() {
    let args = vec![
        Value::String("secret_input".to_string()),
        Value::Integer(32),
    ];
    let result = constant_time_key_derive(args).unwrap();
    
    if let Value::Array(key) = result {
        assert_eq!(key.len(), 32);
        
        // All elements should be valid bytes
        for val in &key {
            if let Value::Integer(i) = val {
                assert!(*i >= 0 && *i <= 255);
            } else {
                panic!("Expected integer values in key");
            }
        }
        
        // Same input should produce same key
        let args2 = vec![
            Value::String("secret_input".to_string()),
            Value::Integer(32),
        ];
        let result2 = constant_time_key_derive(args2).unwrap();
        assert_eq!(result, result2);
        
        // Different input should produce different key
        let args3 = vec![
            Value::String("different_input".to_string()),
            Value::Integer(32),
        ];
        let result3 = constant_time_key_derive(args3).unwrap();
        assert_ne!(result, result3);
    } else {
        panic!("Expected array result for key derivation");
    }
}

#[test]
fn test_error_handling() {
    // Test wrong number of arguments
    let result = constant_time_compare(vec![Value::Integer(1)]);
    assert!(result.is_err());

    let result = constant_time_select(vec![Value::Bool(true), Value::Integer(1)]);
    assert!(result.is_err());

    // Test invalid argument types
    let args = vec![
        Value::String("not_a_condition".to_string()),
        Value::Integer(1),
        Value::Integer(2),
    ];
    let result = constant_time_select(args);
    assert!(result.is_err());

    // Test invalid byte values in array
    let args = vec![
        Value::String("test".to_string()),
        Value::Array(vec![Value::Integer(256)]), // Invalid byte value
    ];
    let result = constant_time_compare(args);
    assert!(result.is_err());

    // Test key derivation with excessive length
    let args = vec![
        Value::String("input".to_string()),
        Value::Integer(2000), // Exceeds maximum
    ];
    let result = constant_time_key_derive(args);
    assert!(result.is_err());
}

#[test]
fn test_timing_analysis_constant_time_compare() {
    // This test attempts to verify constant-time behavior by measuring timing
    // Note: This is not a perfect test as it depends on system load and other factors
    
    let equal_data = "a".repeat(1000);
    let different_data = "b".repeat(1000);
    
    let equal_args = vec![
        Value::String(equal_data.clone()),
        Value::String(equal_data.clone()),
    ];
    
    let different_args = vec![
        Value::String(equal_data),
        Value::String(different_data),
    ];
    
    // Measure timing for equal comparison
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = constant_time_compare(equal_args.clone()).unwrap();
    }
    let equal_duration = start.elapsed();
    
    // Measure timing for different comparison
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = constant_time_compare(different_args.clone()).unwrap();
    }
    let different_duration = start.elapsed();
    
    // The timing difference should be minimal for constant-time operation
    let timing_ratio = equal_duration.as_nanos() as f64 / different_duration.as_nanos() as f64;
    
    // Allow for some variance due to system noise, but should be close to 1.0
    assert!(timing_ratio > 0.8 && timing_ratio < 1.2, 
           "Timing ratio {} suggests non-constant-time behavior", timing_ratio);
    
    println!("Equal comparison time: {:?}", equal_duration);
    println!("Different comparison time: {:?}", different_duration);
    println!("Timing ratio: {:.3}", timing_ratio);
}

#[test]
fn test_timing_analysis_constant_time_select() {
    let large_value_a = Value::Array((0..1000).map(|i| Value::Integer(i)).collect());
    let large_value_b = Value::Array((1000..2000).map(|i| Value::Integer(i)).collect());
    
    let true_args = vec![
        Value::Bool(true),
        large_value_a.clone(),
        large_value_b.clone(),
    ];
    
    let false_args = vec![
        Value::Bool(false),
        large_value_a,
        large_value_b,
    ];
    
    // Measure timing for true condition
    let start = Instant::now();
    for _ in 0..100 {
        let _ = constant_time_select(true_args.clone()).unwrap();
    }
    let true_duration = start.elapsed();
    
    // Measure timing for false condition
    let start = Instant::now();
    for _ in 0..100 {
        let _ = constant_time_select(false_args.clone()).unwrap();
    }
    let false_duration = start.elapsed();
    
    let timing_ratio = true_duration.as_nanos() as f64 / false_duration.as_nanos() as f64;
    
    // Should have similar timing regardless of condition
    assert!(timing_ratio > 0.8 && timing_ratio < 1.2,
           "Timing ratio {} suggests non-constant-time behavior", timing_ratio);
    
    println!("True condition time: {:?}", true_duration);
    println!("False condition time: {:?}", false_duration);
    println!("Timing ratio: {:.3}", timing_ratio);
}

#[test]
fn test_side_channel_resistance() {
    // Test that operations don't leak information through side channels
    
    // Create two similar but different keys
    let key1 = "password123456789012345678901234";
    let key2 = "password123456789012345678901235"; // Last digit different
    
    let args1 = vec![
        Value::String(key1.to_string()),
        Value::String(key1.to_string()),
    ];
    
    let args2 = vec![
        Value::String(key1.to_string()),
        Value::String(key2.to_string()),
    ];
    
    // Multiple iterations to average out noise
    let iterations = 10000;
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = constant_time_compare(args1.clone()).unwrap();
    }
    let same_duration = start.elapsed();
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = constant_time_compare(args2.clone()).unwrap();
    }
    let different_duration = start.elapsed();
    
    // Calculate timing statistics
    let same_avg = same_duration.as_nanos() / iterations;
    let different_avg = different_duration.as_nanos() / iterations;
    let timing_difference = if same_avg > different_avg {
        same_avg - different_avg
    } else {
        different_avg - same_avg
    };
    
    // The timing difference should be minimal (within 5% for constant-time ops)
    let relative_difference = timing_difference as f64 / same_avg as f64;
    
    println!("Same key avg time: {} ns", same_avg);
    println!("Different key avg time: {} ns", different_avg);
    println!("Relative timing difference: {:.1}%", relative_difference * 100.0);
    
    // This is a heuristic test - constant time operations should have minimal timing variation
    assert!(relative_difference < 0.05, 
           "Timing difference {:.1}% exceeds threshold, possible side-channel leak", 
           relative_difference * 100.0);
}

#[test]
fn test_comprehensive_key_derivation() {
    // Test key derivation with various input sizes
    let inputs = vec![
        "short",
        "medium_length_input",
        "very_long_input_that_spans_multiple_blocks_and_should_still_work_correctly",
    ];
    
    let key_lengths = vec![16, 32, 64, 128];
    
    for input in &inputs {
        for &length in &key_lengths {
            let args = vec![
                Value::String(input.to_string()),
                Value::Integer(length as i64),
            ];
            
            let result = constant_time_key_derive(args).unwrap();
            
            if let Value::Array(key) = result {
                assert_eq!(key.len(), length);
                
                // Verify all bytes are valid
                for val in &key {
                    if let Value::Integer(i) = val {
                        assert!(*i >= 0 && *i <= 255);
                    }
                }
                
                // Verify deterministic behavior
                let args2 = vec![
                    Value::String(input.to_string()),
                    Value::Integer(length as i64),
                ];
                let result2 = constant_time_key_derive(args2).unwrap();
                assert_eq!(result, result2);
            }
        }
    }
}

#[test]
fn test_memory_safety() {
    // Test operations with various memory patterns to ensure safety
    
    // Test with empty data
    let args = vec![
        Value::String("".to_string()),
        Value::String("".to_string()),
    ];
    assert_eq!(constant_time_compare(args).unwrap(), Value::Bool(true));
    
    // Test with single byte
    let args = vec![
        Value::Array(vec![Value::Integer(42)]),
        Value::Array(vec![Value::Integer(42)]),
    ];
    assert_eq!(constant_time_compare(args).unwrap(), Value::Bool(true));
    
    // Test memory clearing with various sizes
    for size in vec![0, 1, 16, 32, 64, 128, 256] {
        let data = Value::Array((0..size).map(|i| Value::Integer(i % 256)).collect());
        let result = constant_time_clear(vec![data]).unwrap();
        
        if let Value::Array(cleared) = result {
            assert_eq!(cleared.len(), size);
            for val in cleared {
                assert_eq!(val, Value::Integer(0));
            }
        }
    }
}

#[test]
fn test_arithmetic_operations_timing() {
    // Test that arithmetic operations have consistent timing
    
    let test_cases = vec![
        (0, 0),
        (1, 1),
        (42, 42),
        (42, 43),
        (1000000, 1000000),
        (1000000, 1000001),
        (i64::MAX, i64::MAX),
        (i64::MAX, i64::MAX - 1),
    ];
    
    for (a, b) in test_cases {
        let args = vec![Value::Integer(a), Value::Integer(b)];
        
        // Time the equality check
        let start = Instant::now();
        for _ in 0..1000 {
            let _ = constant_time_int_equal(args.clone()).unwrap();
        }
        let duration = start.elapsed();
        
        // Time the less-than check
        let start = Instant::now();
        for _ in 0..1000 {
            let _ = constant_time_less_than(args.clone()).unwrap();
        }
        let lt_duration = start.elapsed();
        
        println!("Equal check for ({}, {}): {:?}", a, b, duration);
        println!("Less-than check for ({}, {}): {:?}", a, b, lt_duration);
    }
}
