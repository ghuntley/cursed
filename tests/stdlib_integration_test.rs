//! Integration test for CURSED standard library core functionality

use cursed::runtime::value::Value;

#[test]
fn test_value_creation_and_formatting() {
    // Test creating different types of values
    let null_val = Value::null();
    let bool_val = Value::bool(true);
    let int_val = Value::integer(42);
    let num_val = Value::number(3.14);
    let str_val = Value::string("Hello CURSED!");
    
    // Test that values can be formatted
    assert!(format!("{:?}", null_val).contains("Null"));
    assert!(format!("{:?}", bool_val).contains("Bool"));
    assert!(format!("{:?}", int_val).contains("Integer"));
    assert!(format!("{:?}", num_val).contains("Number"));
    assert!(format!("{:?}", str_val).contains("String"));
}

#[test]
fn test_value_operations() {
    let val1 = Value::integer(42);
    let val2 = Value::integer(0);
    let val3 = Value::bool(false);
    let val4 = Value::string("");
    
    // Test truthiness
    assert!(val1.is_truthy());
    assert!(!val2.is_truthy());
    assert!(!val3.is_truthy());
    assert!(!val4.is_truthy());
    
    // Test null check
    assert!(!val1.is_null());
    assert!(!val2.is_null());
    assert!(Value::null().is_null());
}

#[test]
fn test_array_and_object_values() {
    let array = Value::array(vec![
        Value::integer(1),
        Value::integer(2),
        Value::integer(3),
    ]);
    
    let mut map = std::collections::HashMap::new();
    map.insert("name".to_string(), Value::string("test"));
    map.insert("value".to_string(), Value::integer(123));
    let object = Value::object(map);
    
    // Test that they format correctly
    let array_str = format!("{:?}", array);
    let object_str = format!("{:?}", object);
    
    assert!(array_str.contains("Array"));
    assert!(object_str.contains("Object"));
}

#[test]
fn test_error_types() {
    use cursed::error_types::CursedError;
    
    let parse_error = CursedError::Parse("test parse error".to_string());
    let runtime_error = CursedError::Runtime("test runtime error".to_string());
    let io_error = CursedError::Io("test io error".to_string());
    
    // Test that errors format correctly
    assert!(format!("{}", parse_error).contains("Parse error"));
    assert!(format!("{}", runtime_error).contains("Runtime error"));
    assert!(format!("{}", io_error).contains("I/O error"));
}

#[test]
fn test_value_conversions() {
    // Test that values can be converted and compared
    let int_val = Value::integer(42);
    let num_val = Value::number(42.0);
    let str_val = Value::string("42");
    
    // They should all be truthy
    assert!(int_val.is_truthy());
    assert!(num_val.is_truthy());
    assert!(str_val.is_truthy());
    
    // Test cloning
    let cloned_int = int_val.clone();
    assert_eq!(int_val, cloned_int);
}
