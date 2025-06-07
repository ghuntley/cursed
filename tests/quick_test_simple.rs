use cursed::stdlib::quick_test::*;
use cursed::stdlib::{Generator, RandGen, value, one_of};
use cursed::object::Object;
use std::sync::Arc;
use std::time::Instant;


/// Test the quick_test implementation

#[test]
fn test_random_integer_range() {
    for _ in 0..5 {
        // Generate a random integer between -10 and 10
        let value = int_range(-10, 10);
        let n = value;
        assert!(n >= -10 && n <= 10);
    }
}

#[test]
fn test_random_float_range() {
    for _ in 0..5 {
        // Generate a random float between 0.0 and 1.0
        let value = float_range(0.0, 1.0);
        let f = value;
        assert!(f >= 0.0 && f <= 1.0);
    }
}

#[test]
fn test_string_generation() {
    for _ in 0..5 {
        // Generate a random string with length between 5 and 10
        let value = string_with_length(5, 10);
        assert!(value.len() >= 5 && value.len() <= 10);
    }
}

#[test]
fn test_generator_trait() {
    // Creating a simple constant generator
    let const_gen = value(Object::Integer(42);
    let mut rand = RandGen::new(1234);
    
    // The generator should return the constant value
    let result = const_gen.generate(&mut rand, 100);
    match result {
        Object::Integer(n) => assert_eq!(n, 42),
        _ => panic!("constant generator did not return expected value")
    }
    
    // Create a one_of generator
    let values = vec![Object::Integer(1), Object::Integer(2), Object::Integer(3)];
    let one_of_gen = one_of(values.clone();
    
    for _ in 0..10 {
        let result = one_of_gen.generate(&mut rand, 100);
        match result {
            Object::Integer(n) => assert!(n >= 1 && n <= 3),
            _ => panic!("one_of generator did not return expected value type")
        }
    }
}