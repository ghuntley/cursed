//! Basic tests for the simplified quick_test module

use cursed::object::Object;
use cursed::stdlib::{Config, TestResult, Rand, check, int_range, boolean, string, int_array};

#[test]
fn test_basic_config() {
    let config = Config::default();
    assert_eq!(config.max_count, 100);
    assert_eq!(config.max_size, 100);
    assert_eq!(config.min_size, 0);
}

#[test]
fn test_rand() {
    let mut rand = Rand::new(12345);
    
    // Test that random numbers are generated consistently with the same seed
    let val1 = rand.next();
    let val2 = rand.next();
    
    let mut rand2 = Rand::new(12345);
    let val1_repeat = rand2.next();
    let val2_repeat = rand2.next();
    
    assert_eq!(val1, val1_repeat);
    assert_eq!(val2, val2_repeat);
    
    // Test int range function
    let range_val = rand.int_range(10, 20);
    assert!(range_val >= 10 && range_val <= 20);
}

#[test]
fn test_basic_generators() {
    // Test integer generator
    let int_val = int_range(1, 10);
    assert!(matches!(int_val, Object::Integer(_)));
    let value = match int_val { Object::Integer(i) => i, _ => panic!("Not an integer") };
    assert!(value >= 1 && value <= 10);
    
    // Test boolean generator
    let bool_val = boolean();
    assert!(matches!(bool_val, Object::Boolean(_)));
    
    // Test string generator
    let string_val = string();
    assert!(matches!(string_val, Object::String(_)));
    
    // Test array generator
    let array_val = int_array(5, 10, 1, 100);
    assert!(matches!(array_val, Object::Array(_)));
    let elements = match array_val { Object::Array(a) => a, _ => panic!("Not an array") };
    assert!(elements.len() >= 5 && elements.len() <= 10);
    
    for elem in elements {
        assert!(matches!(elem, Object::Integer(_)));
        let val = match elem { Object::Integer(i) => i, _ => panic!("Not an integer") };
        assert!(val >= 1 && val <= 100);
    }
}

#[test]
fn test_check_function() {
    // Create a test configuration
    let config = Config {
        max_count: 10,
        ..Config::default()
    };
    
    // Create a dummy test function (not actually called in our simplified implementation)
    let test_fn = Object::Boolean(true);
    
    // Run the check function
    let result = check(test_fn, &config);
    
    // Verify the result has the expected properties
    assert!(result.passed);
    assert_eq!(result.count, 10);
    assert_eq!(result.failed_after, 0);
    assert_eq!(result.shrink_count, 0);
}