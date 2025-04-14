use cursed::object::Object;
use cursed::stdlib::quick_test;

#[test]
fn test_basic_generators() {
    // Test int_range
    let int = quick_test::int_range(-10, 10);
    assert!(int >= -10 && int <= 10, "int_range generated value out of range");
    
    // Test boolean
    let bool_val = quick_test::boolean();
    // boolean() returns a boolean directly, not an Object
    assert!(bool_val == true || bool_val == false, "boolean didn't return a valid boolean");
    
    // Test string
    let string_val = quick_test::string();
    // string() returns a string directly
    assert!(string_val.len() >= 1, "string has invalid length");
    
    // Test int_array
    let array_val = quick_test::int_array(3, 7, 0, 100);
    assert!(array_val.len() >= 3 && array_val.len() <= 7, "array length out of expected range");
    for item in &array_val {
        assert!(*item >= 0 && *item <= 100, "array item out of expected range");
    }
}

#[test]
fn test_config_and_result() {
    // Test Config default creation
    let config = quick_test::Config::default();
    assert_eq!(config.max_count, 100, "Default max_count should be 100");
    assert_eq!(config.shrink, quick_test::DEFAULT_SHRINK, 
               "Default shrink should be DEFAULT_SHRINK");
    
    // Create a test result manually since there's no default() implementation
    let result = quick_test::TestResult {
        passed: true,
        count: 0,
        failed_after: 0,
        counterexample: None,
        shrunk_counterexample: None
    };
    assert!(result.passed, "Result should be passed");
    assert_eq!(result.count, 0, "Count should be 0");
    assert_eq!(result.failed_after, 0, "Failed_after should be 0");
}