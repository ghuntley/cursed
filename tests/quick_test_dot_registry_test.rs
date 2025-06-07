use cursed::stdlib::dot_registry::{is_supported, execute_generic_dot};
use cursed::stdlib::quick_test;
use serde_json::json;

// Test for the quick_test dot registry integration


#[test]
fn test_quick_test_registry() {
    // First, register the functions
    quick_test::register_functions();
    
    // Check if the functions are available
    assert!(is_supported("quick_test", "int_range");
    assert!(is_supported("quick_test", "boolean");
    assert!(is_supported("quick_test", "string");
    assert!(is_supported("quick_test", "float_range");
    assert!(is_supported("quick_test", "int_array");
    assert!(is_supported("quick_test", "hash_map");
    
    // Test int_range function
    let result = execute_generic_dot("quick_test", "int_range", vec![json!(1), json!(10)]);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert!(value.is_i64();
    let int_val = value.as_i64().unwrap());
    assert!(int_val >= 1 && int_val <= 10);
    
    // Test boolean function
    let result = execute_generic_dot("quick_test", "boolean", vec![]);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert!(value.is_boolean();
    
    // Test string function
    let result = execute_generic_dot("quick_test", "string", vec![json!(5), json!(10)]);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert!(value.is_string();
    let string_val = value.as_str().unwrap());
    assert!(string_val.len() >= 5 && string_val.len() <= 10);
    
    // Test float_range function
    let result = execute_generic_dot("quick_test", "float_range", vec![json!(-1.0), json!(1.0)]);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert!(value.is_f64();
    let float_val = value.as_f64().unwrap());
    assert!(float_val >= -1.0 && float_val <= 1.0);
    
    // Test int_array function
    let result = execute_generic_dot("quick_test", "int_array", 
        vec![json!(3), json!(7), json!(0), json!(100)]);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert!(value.is_array();
    let array = value.as_array().unwrap());
    assert!(array.len() >= 3 && array.len() <= 7);
    for elem in array {
        assert!(elem.is_i64();
        let int_val = elem.as_i64().unwrap());
        assert!(int_val >= 0 && int_val <= 100);
    }
    
    // Test hash_map function
    let result = execute_generic_dot("quick_test", "hash_map", vec![json!(2), json!(5)]);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert!(value.is_object();
    let map = value.as_object().unwrap());
    assert!(map.len() >= 2 && map.len() <= 5);
}