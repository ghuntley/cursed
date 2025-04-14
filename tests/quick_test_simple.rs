use cursed::stdlib::quick_test::*;
use cursed::object::Object;
use std::rc::Rc;
use std::time::Instant;

/// Test the quick_test implementation

#[test]
fn test_random_integer_range() {
    for _ in 0..5 {
        // Generate a random integer between -10 and 10
        let value = int_range(-10, 10);
        if let Object::Integer(n) = value {
            assert!(n >= -10 && n <= 10);
        } else {
            panic!("int_range did not return an integer");
        }
    }
}

#[test]
fn test_random_float_range() {
    for _ in 0..5 {
        // Generate a random float between 0.0 and 1.0
        let value = float_range(0.0, 1.0);
        if let Object::Float(f) = value {
            assert!(f >= 0.0 && f <= 1.0);
        } else {
            panic!("float_range did not return a float");
        }
    }
}

#[test]
fn test_string_generation() {
    for _ in 0..5 {
        // Generate a random string with length between 5 and 10
        let value = string_with_length(5, 10);
        if let Object::String(s) = value {
            assert!(s.len() >= 5 && s.len() <= 10);
        } else {
            panic!("string_with_length did not return a string");
        }
    }
}

#[test]
fn test_generator_trait() {
    // Creating a simple constant generator
    let const_gen = value(Object::Integer(42));
    let mut rand = Rand::new(1234);
    
    // The generator should return the constant value
    let result = const_gen.generate(&mut rand, 100);
    if let Object::Integer(n) = result {
        assert_eq!(n, 42);
    } else {
        panic!("constant generator did not return expected value");
    }
    
    // Create a one_of generator
    let values = vec![Object::Integer(1), Object::Integer(2), Object::Integer(3)];
    let one_of_gen = one_of(values.clone());
    
    for _ in 0..10 {
        let result = one_of_gen.generate(&mut rand, 100);
        if let Object::Integer(n) = result {
            assert!(n >= 1 && n <= 3);
        } else {
            panic!("one_of generator did not return expected value type");
        }
    }
}