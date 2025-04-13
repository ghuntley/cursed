use std::cell::RefCell;
use std::rc::Rc;

use cursed::error::Error;
use cursed::object::{self, Object};
use cursed::stdlib::quick_test::{Config, TestResult, check, int_range, boolean, string, int_array, float_range, hash_map, one_of_type};

#[test]
fn test_random_generators() {
    // Test basic random generators
    let int_val = int_range(-10, 10);
    match int_val {
        Object::Integer(n) => assert!(n >= -10 && n <= 10),
        _ => panic!("Expected integer"),
    }
    
    let bool_val = boolean();
    match bool_val {
        Object::Boolean(_) => {}, // Any boolean value is valid
        _ => panic!("Expected boolean"),
    }
    
    let string_val = string();
    match string_val {
        Object::String(s) => assert!(!s.is_empty()),
        _ => panic!("Expected string"),
    }
    
    let array_val = int_array(3, 7, 0, 100);
    match array_val {
        Object::Array(arr) => {
            assert!(arr.len() >= 3 && arr.len() <= 7);
            for elem in arr {
                match elem {
                    Object::Integer(n) => assert!(n >= 0 && n <= 100),
                    _ => panic!("Expected integer in array"),
                }
            }
        },
        _ => panic!("Expected array"),
    }
    
    let float_val = float_range(-1.0, 1.0);
    match float_val {
        Object::Float(f) => assert!(f >= -1.0 && f <= 1.0),
        _ => panic!("Expected float"),
    }
    
    let hash_map_val = hash_map(2, 5);
    match hash_map_val {
        Object::HashTable(map) => {
            assert!(map.len() >= 2 && map.len() <= 5);
        },
        _ => panic!("Expected hash map"),
    }
}

#[test]
fn test_property_based_test() {
    // Create a test configuration with small count for quick testing
    let config = Config {
        max_count: 10,
        min_size: -10,
        max_size: 10,
        expect_failure: false,
        quiet: true,
        ..Config::default()
    };
    
    // Test function: absolute value of input is less than or equal to 10
    let test_fn = |args: &[Rc<Object>], _env: Rc<RefCell<object::Environment>>| -> Result<Object, Error> {
        if let Object::Integer(n) = &*args[0] {
            let abs_n = n.abs();
            return Ok(Object::Boolean(abs_n <= 10));
        }
        Ok(Object::Boolean(false))
    };
    
    // Create a test function object
    let test_fn_obj = Object::Function(Rc::new(RefCell::new(object::Function {
        parameters: vec!["x".to_string()],
        body: vec![],
        env: Rc::new(RefCell::new(object::Environment::new())),
        call: Box::new(test_fn),
    })));
    
    // Run the property-based test
    let result = check(test_fn_obj, &config);
    
    // All tests should pass since we're generating numbers in [-10, 10]
    assert!(result.passed);
    assert_eq!(result.count, config.max_count);
    assert_eq!(result.failed_after, 0);
}