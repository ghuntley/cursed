use cursed::object::Object;
use cursed::stdlib::quick_test;

#[test]
fn test_basic_generators() {
    // Test int_range
    let int = quick_test::int_range(-10, 10);
    if let Object::Integer(i) = int {
        assert!(i >= -10 && i <= 10, "int_range generated value out of range");
    } else {
        panic!("int_range didn't return an Integer");
    }
    
    // Test boolean
    let bool_val = quick_test::boolean();
    match bool_val {
        Object::Boolean(_) => {}, // Success
        _ => panic!("boolean didn't return a Boolean")
    }
    
    // Test string
    let string_val = quick_test::string();
    match string_val {
        Object::String(s) => {
            assert!(s.len() >= 1 && s.len() <= 10, "string length out of expected range");
        },
        _ => panic!("string didn't return a String")
    }
    
    // Test int_array
    let array_val = quick_test::int_array(3, 7, 0, 100);
    match array_val {
        Object::Array(arr) => {
            assert!(arr.len() >= 3 && arr.len() <= 7, "array length out of expected range");
            for item in arr {
                if let Object::Integer(i) = item {
                    assert!(i >= 0 && i <= 100, "array item out of expected range");
                } else {
                    panic!("array contains non-integer values");
                }
            }
        },
        _ => panic!("int_array didn't return an Array")
    }
}

#[test]
fn test_config_and_result() {
    // Test Config default creation
    let config = quick_test::Config::default();
    assert_eq!(config.max_count, 100, "Default max_count should be 100");
    assert_eq!(config.max_size, 100, "Default max_size should be 100");
    assert_eq!(config.shrink_strategy, quick_test::DEFAULT_SHRINK, 
               "Default shrink strategy should be DEFAULT_SHRINK");
    
    // Test TestResult default creation
    let result = quick_test::TestResult::default();
    assert!(result.passed, "Default result should be passed");
    assert_eq!(result.count, 0, "Default count should be 0");
    assert_eq!(result.failed_after, 0, "Default failed_after should be 0");
}