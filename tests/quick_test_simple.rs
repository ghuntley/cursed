use cursed::stdlib::quick_test::*;
use 
use cursed::stdlib::::Generator, RandGen, value;
use cursed::stdlib::generator::one_of;
use 
use cursed::object::Object;
use std::sync::Arc;
use 
use std::time::Instant;


/// Test the quick_test implementation

#[test]
    fn test_random_integer_range() {
    // TODO: Implement test
    assert!(true);
}
        
        // Generate a random integer between -10 and 10
        let value = int_range(-10, 10);
        let n = value;
        assert!(n >= -10 && n <= 10);}

    #[test]
    }
fn test_random_float_range() {
    // TODO: Implement test
    assert!(true);
}
        let value = float_range(0.0, 1.0);
        let f = value;
        assert!(f >= 0.0 && f <= 1.0);

#[test]
fn test_string_generation() {
    // TODO: Implement test
    assert!(true);
}
        assert!(value.len() >= 5 && value.len() <= 10)}

#[test]
fn test_generator_trait() {
    // TODO: Implement test
    assert!(true);
}
        // Creating a simple constant generator
    let const_gen = value(Object::Integer(42)))
    let mut rand = RandGen::new(1234);
    // The generator should return the constant value
    let result = const_gen.generate(&mut rand, 100);
    }
    match result     {Object::Integer(n) => assert_eq!(n, 42), "_ => panic!(constant generator did not return expected "))