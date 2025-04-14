//! Standalone test for the quick_test module

use cursed::stdlib::quick_test;
use cursed::object::Object;
use std::rc::Rc;

#[test]
fn test_quick_test_random_generation() {
    // Test random integer generation in range
    let int_val = quick_test::int_range(-10, 10);
    if let Object::Integer(n) = int_val {
        assert!(n >= -10 && n <= 10);
    } else {
        panic!("Expected integer value");
    }
    
    // Test random boolean generation
    let bool_val = quick_test::boolean();
    assert!(matches!(bool_val, Object::Boolean(_)));
    
    // Test random string generation
    let string_val = quick_test::string();
    assert!(matches!(string_val, Object::String(_)));
    
    // Test random array generation
    let array_val = quick_test::int_array(3, 7, 0, 100);
    if let Object::Array(arr) = array_val {
        assert!(arr.len() >= 3 && arr.len() <= 7);
        for elem in arr {
            if let Object::Integer(n) = elem {
                assert!(n >= 0 && n <= 100);
            } else {
                panic!("Expected array of integers");
            }
        }
    } else {
        panic!("Expected array value");
    }
    
    // Test random float generation
    let float_val = quick_test::float_range(-1.0, 1.0);
    if let Object::Float(f) = float_val {
        assert!(f >= -1.0 && f <= 1.0);
    } else {
        panic!("Expected float value");
    }
    
    // Test hash map generation
    let hash_val = quick_test::hash_map(2, 5);
    if let Object::HashTable(map) = hash_val {
        assert!(map.len() >= 2 && map.len() <= 5);
    } else {
        panic!("Expected hash table value");
    }
}

#[test]
fn test_property_testing() {
    // Mock property and generator functions
    // The property will check if a number is even
    let mock_property = Object::Builtin {
        name: "is_even".to_string(),
        function: |args: &[Rc<Object>]| {
            if let Some(arg) = args.get(0) {
                if let Object::Integer(n) = **arg {
                    return Ok(Rc::new(Object::Boolean(n % 2 == 0)));
                }
            }
            Ok(Rc::new(Object::Boolean(false)))
        },
    };
    
    // The generator will create even numbers (which should satisfy the property)
    let mock_generator = Object::Builtin {
        name: "even_number_generator".to_string(),
        function: |_args: &[Rc<Object>]| {
            Ok(Rc::new(Object::Integer(2)))
        },
    };
    
    // Create config with limited iterations for test
    let config = quick_test::Config {
        max_count: 5,
        quiet: true,
        ..quick_test::Config::default()
    };
    
    // Run property test
    let result = quick_test::for_all(mock_generator, mock_property, &config);
    
    // Check the result - all even numbers should satisfy the property
    assert!(result.passed);
    assert_eq!(result.count, 5);
}