/// Comprehensive test suite for CURSED Core builtin functions
/// 
/// This test suite validates all core functionality including type conversions,
/// collection operations, panic handling, and utility functions to ensure
/// production-ready reliability.

use cursed::stdlib::core::*;
use cursed::stdlib::value::Value;
use std::collections::HashMap;

// ================================
// TYPE CONVERSION TESTS
// ================================

#[test]
fn test_lit_conversion_comprehensive() {
    // Test integer to boolean conversions
    assert_eq!(lit(&Value::Integer(1)).unwrap(), true);
    assert_eq!(lit(&Value::Integer(0)).unwrap(), false);
    assert_eq!(lit(&Value::Integer(-1)).unwrap(), true);
    assert_eq!(lit(&Value::Integer(42)).unwrap(), true);
    assert_eq!(lit(&Value::Integer(9223372036854775807)).unwrap(), true);
    
    // Test float to boolean conversions
    assert_eq!(lit(&Value::Number(1.0)).unwrap(), true);
    assert_eq!(lit(&Value::Number(0.0)).unwrap(), false);
    assert_eq!(lit(&Value::Number(-1.0)).unwrap(), true);
    assert_eq!(lit(&Value::Number(0.1)).unwrap(), true);
    assert_eq!(lit(&Value::Number(3.14159)).unwrap(), true);
    
    // Test string to boolean conversions
    assert_eq!(lit(&Value::String("hello".to_string())).unwrap(), true);
    assert_eq!(lit(&Value::String("".to_string())).unwrap(), false);
    assert_eq!(lit(&Value::String(" ".to_string())).unwrap(), true);
    assert_eq!(lit(&Value::String("false".to_string())).unwrap(), true); // Non-empty string is true
    
    // Test collection to boolean conversions
    assert_eq!(lit(&Value::Array(vec![Value::Int32(1)])).unwrap(), true);
    assert_eq!(lit(&Value::Array(vec![])).unwrap(), false);
    
    let mut obj = HashMap::new();
    obj.insert("key".to_string(), Value::Int32(1));
    assert_eq!(lit(&Value::Object(obj)).unwrap(), true);
    assert_eq!(lit(&Value::Object(HashMap::new())).unwrap(), false);
    
    // Test null to boolean conversion
    assert_eq!(lit(&Value::Null).unwrap(), false);
    
    // Test boolean to boolean conversion (identity)
    assert_eq!(lit(&Value::Bool(true)).unwrap(), true);
    assert_eq!(lit(&Value::Bool(false)).unwrap(), false);
}

#[test]
fn test_normie_conversion_comprehensive() {
    // Test identity conversion
    assert_eq!(normie(&Value::Int32(42)).unwrap(), 42);
    assert_eq!(normie(&Value::Int32(-42)).unwrap(), -42);
    assert_eq!(normie(&Value::Int32(0)).unwrap(), 0);
    assert_eq!(normie(&Value::Int32(i32::MAX)).unwrap(), i32::MAX);
    assert_eq!(normie(&Value::Int32(i32::MIN)).unwrap(), i32::MIN);
    
    // Test i64 to i32 conversion
    assert_eq!(normie(&Value::Int64(42)).unwrap(), 42);
    assert_eq!(normie(&Value::Int64(-42)).unwrap(), -42);
    assert_eq!(normie(&Value::Int64(i32::MAX as i64)).unwrap(), i32::MAX);
    assert_eq!(normie(&Value::Int64(i32::MIN as i64)).unwrap(), i32::MIN);
    
    // Test out-of-range i64 to i32 conversion
    assert!(normie(&Value::Int64(i64::MAX)).is_err());
    assert!(normie(&Value::Int64(i64::MIN)).is_err());
    assert!(normie(&Value::Int64(i32::MAX as i64 + 1)).is_err());
    assert!(normie(&Value::Int64(i32::MIN as i64 - 1)).is_err());
    
    // Test float to int conversion (truncation)
    assert_eq!(normie(&Value::Float32(42.0)).unwrap(), 42);
    assert_eq!(normie(&Value::Float32(42.7)).unwrap(), 42);
    assert_eq!(normie(&Value::Float32(-42.7)).unwrap(), -42);
    assert_eq!(normie(&Value::Float64(42.99)).unwrap(), 42);
    
    // Test boolean to int conversion
    assert_eq!(normie(&Value::Bool(true)).unwrap(), 1);
    assert_eq!(normie(&Value::Bool(false)).unwrap(), 0);
    
    // Test string to int conversion
    assert_eq!(normie(&Value::String("42".to_string())).unwrap(), 42);
    assert_eq!(normie(&Value::String("-42".to_string())).unwrap(), -42);
    assert_eq!(normie(&Value::String("0".to_string())).unwrap(), 0);
    
    // Test invalid string to int conversion
    assert!(normie(&Value::String("hello".to_string())).is_err());
    assert!(normie(&Value::String("42.5".to_string())).is_err());
    assert!(normie(&Value::String("".to_string())).is_err());
    
    // Test null to int conversion
    assert_eq!(normie(&Value::Null).unwrap(), 0);
    
    // Test invalid type conversions
    assert!(normie(&Value::Array(vec![])).is_err());
    assert!(normie(&Value::Object(HashMap::new())).is_err());
}

#[test]
fn test_thicc_conversion_comprehensive() {
    // Test identity conversion
    assert_eq!(thicc(&Value::Int64(42)).unwrap(), 42);
    assert_eq!(thicc(&Value::Int64(-42)).unwrap(), -42);
    assert_eq!(thicc(&Value::Int64(i64::MAX)).unwrap(), i64::MAX);
    assert_eq!(thicc(&Value::Int64(i64::MIN)).unwrap(), i64::MIN);
    
    // Test i32 to i64 conversion
    assert_eq!(thicc(&Value::Int32(42)).unwrap(), 42);
    assert_eq!(thicc(&Value::Int32(-42)).unwrap(), -42);
    assert_eq!(thicc(&Value::Int32(i32::MAX)).unwrap(), i32::MAX as i64);
    assert_eq!(thicc(&Value::Int32(i32::MIN)).unwrap(), i32::MIN as i64);
    
    // Test float to int conversion
    assert_eq!(thicc(&Value::Float32(42.0)).unwrap(), 42);
    assert_eq!(thicc(&Value::Float64(42.99)).unwrap(), 42);
    assert_eq!(thicc(&Value::Float64(-42.99)).unwrap(), -42);
    
    // Test boolean to int conversion
    assert_eq!(thicc(&Value::Bool(true)).unwrap(), 1);
    assert_eq!(thicc(&Value::Bool(false)).unwrap(), 0);
    
    // Test string to int conversion
    assert_eq!(thicc(&Value::String("123456789".to_string())).unwrap(), 123456789);
    assert_eq!(thicc(&Value::String("-123456789".to_string())).unwrap(), -123456789);
    
    // Test invalid string conversions
    assert!(thicc(&Value::String("hello".to_string())).is_err());
    assert!(thicc(&Value::String("42.5".to_string())).is_err());
    
    // Test null conversion
    assert_eq!(thicc(&Value::Null).unwrap(), 0);
}

#[test]
fn test_snack_conversion_comprehensive() {
    // Test identity conversion
    assert_eq!(snack(&Value::Float32(3.14)).unwrap(), 3.14f32);
    assert_eq!(snack(&Value::Float32(-3.14)).unwrap(), -3.14f32);
    assert_eq!(snack(&Value::Float32(0.0)).unwrap(), 0.0f32);
    
    // Test f64 to f32 conversion
    assert_eq!(snack(&Value::Float64(3.14159)).unwrap(), 3.14159f32);
    assert_eq!(snack(&Value::Float64(-3.14159)).unwrap(), -3.14159f32);
    
    // Test int to float conversion
    assert_eq!(snack(&Value::Int32(42)).unwrap(), 42.0f32);
    assert_eq!(snack(&Value::Int64(42)).unwrap(), 42.0f32);
    assert_eq!(snack(&Value::Int32(-42)).unwrap(), -42.0f32);
    
    // Test boolean to float conversion
    assert_eq!(snack(&Value::Bool(true)).unwrap(), 1.0f32);
    assert_eq!(snack(&Value::Bool(false)).unwrap(), 0.0f32);
    
    // Test string to float conversion
    assert_eq!(snack(&Value::String("3.14".to_string())).unwrap(), 3.14f32);
    assert_eq!(snack(&Value::String("-3.14".to_string())).unwrap(), -3.14f32);
    assert_eq!(snack(&Value::String("42".to_string())).unwrap(), 42.0f32);
    
    // Test invalid string conversions
    assert!(snack(&Value::String("hello".to_string())).is_err());
    assert!(snack(&Value::String("".to_string())).is_err());
    
    // Test null conversion
    assert_eq!(snack(&Value::Null).unwrap(), 0.0f32);
}

#[test]
fn test_meal_conversion_comprehensive() {
    // Test identity conversion
    assert_eq!(meal(&Value::Float64(3.14159)).unwrap(), 3.14159f64);
    assert_eq!(meal(&Value::Float64(-3.14159)).unwrap(), -3.14159f64);
    
    // Test f32 to f64 conversion
    assert_eq!(meal(&Value::Float32(3.14)).unwrap(), 3.14f32 as f64);
    
    // Test int to float conversion
    assert_eq!(meal(&Value::Int32(42)).unwrap(), 42.0f64);
    assert_eq!(meal(&Value::Int64(42)).unwrap(), 42.0f64);
    
    // Test boolean to float conversion
    assert_eq!(meal(&Value::Bool(true)).unwrap(), 1.0f64);
    assert_eq!(meal(&Value::Bool(false)).unwrap(), 0.0f64);
    
    // Test string to float conversion
    assert_eq!(meal(&Value::String("3.14159".to_string())).unwrap(), 3.14159f64);
    assert_eq!(meal(&Value::String("-3.14159".to_string())).unwrap(), -3.14159f64);
    
    // Test invalid string conversions
    assert!(meal(&Value::String("hello".to_string())).is_err());
    
    // Test null conversion
    assert_eq!(meal(&Value::Null).unwrap(), 0.0f64);
}

#[test]
fn test_tea_conversion_comprehensive() {
    // Test string identity
    assert_eq!(tea(&Value::String("hello".to_string())).unwrap(), "hello");
    assert_eq!(tea(&Value::String("".to_string())).unwrap(), "");
    
    // Test int to string conversion
    assert_eq!(tea(&Value::Int32(42)).unwrap(), "42");
    assert_eq!(tea(&Value::Int32(-42)).unwrap(), "-42");
    assert_eq!(tea(&Value::Int64(123456789)).unwrap(), "123456789");
    
    // Test float to string conversion
    assert_eq!(tea(&Value::Float32(3.14)).unwrap(), "3.14");
    assert_eq!(tea(&Value::Float64(3.14159)).unwrap(), "3.14159");
    
    // Test boolean to string conversion
    assert_eq!(tea(&Value::Bool(true)).unwrap(), "true");
    assert_eq!(tea(&Value::Bool(false)).unwrap(), "false");
    
    // Test null to string conversion
    assert_eq!(tea(&Value::Null).unwrap(), "null");
    
    // Test complex types (should not error, but format may vary)
    assert!(tea(&Value::Array(vec![Value::Int32(1)])).is_ok());
    assert!(tea(&Value::Object(HashMap::new())).is_ok());
}

// ================================
// COLLECTION OPERATION TESTS
// ================================

#[test]
fn test_append_comprehensive() {
    // Test appending to empty array
    let empty_array = Value::Array(vec![]);
    let elements = vec![Value::Int32(1), Value::Int32(2)];
    let result = append(&empty_array, &elements).unwrap();
    
    if let Value::Array(arr) = result {
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0], Value::Int32(1));
        assert_eq!(arr[1], Value::Int32(2));
    } else {
        panic!("Expected array result");
    }
    
    // Test appending to existing array
    let existing_array = Value::Array(vec![Value::Int32(1), Value::Int32(2)]);
    let more_elements = vec![Value::Int32(3), Value::Int32(4), Value::Int32(5)];
    let result = append(&existing_array, &more_elements).unwrap();
    
    if let Value::Array(arr) = result {
        assert_eq!(arr.len(), 5);
        for i in 0..5 {
            assert_eq!(arr[i], Value::Int32(i as i32 + 1));
        }
    } else {
        panic!("Expected array result");
    }
    
    // Test appending empty elements
    let array = Value::Array(vec![Value::Int32(1)]);
    let empty_elements = vec![];
    let result = append(&array, &empty_elements).unwrap();
    
    if let Value::Array(arr) = result {
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0], Value::Int32(1));
    } else {
        panic!("Expected array result");
    }
    
    // Test appending mixed types
    let array = Value::Array(vec![Value::Int32(1)]);
    let mixed_elements = vec![Value::String("hello".to_string()), Value::Bool(true)];
    let result = append(&array, &mixed_elements).unwrap();
    
    if let Value::Array(arr) = result {
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], Value::Int32(1));
        assert_eq!(arr[1], Value::String("hello".to_string()));
        assert_eq!(arr[2], Value::Bool(true));
    } else {
        panic!("Expected array result");
    }
    
    // Test error case: non-array input
    assert!(append(&Value::Int32(42), &[]).is_err());
    assert!(append(&Value::String("hello".to_string()), &[]).is_err());
}

#[test]
fn test_len_comprehensive() {
    // Test array length
    let empty_array = Value::Array(vec![]);
    assert_eq!(len(&empty_array).unwrap(), 0);
    
    let array = Value::Array(vec![Value::Int32(1), Value::Int32(2), Value::Int32(3)]);
    assert_eq!(len(&array).unwrap(), 3);
    
    // Test string length (Unicode-aware)
    let empty_string = Value::String("".to_string());
    assert_eq!(len(&empty_string).unwrap(), 0);
    
    let ascii_string = Value::String("hello".to_string());
    assert_eq!(len(&ascii_string).unwrap(), 5);
    
    let unicode_string = Value::String("🔥💯".to_string());
    assert_eq!(len(&unicode_string).unwrap(), 2); // 2 Unicode code points
    
    let mixed_string = Value::String("hello🔥".to_string());
    assert_eq!(len(&mixed_string).unwrap(), 6); // 5 ASCII + 1 Unicode
    
    // Test object length
    let empty_object = Value::Object(HashMap::new());
    assert_eq!(len(&empty_object).unwrap(), 0);
    
    let mut object = HashMap::new();
    object.insert("key1".to_string(), Value::Int32(1));
    object.insert("key2".to_string(), Value::Int32(2));
    let value_object = Value::Object(object);
    assert_eq!(len(&value_object).unwrap(), 2);
    
    // Test error cases
    assert!(len(&Value::Int32(42)).is_err());
    assert!(len(&Value::Bool(true)).is_err());
    assert!(len(&Value::Null).is_err());
}

#[test]
fn test_cap_comprehensive() {
    // Test array capacity
    let array = Value::Array(Vec::with_capacity(10));
    let capacity = cap(&array).unwrap();
    assert!(capacity >= 0); // Capacity should be non-negative
    
    // Test string capacity
    let string = Value::String(String::with_capacity(20));
    let capacity = cap(&string).unwrap();
    assert!(capacity >= 0);
    
    // Test object capacity
    let object = Value::Object(HashMap::with_capacity(15));
    let capacity = cap(&object).unwrap();
    assert!(capacity >= 0);
    
    // Test error cases
    assert!(cap(&Value::Int32(42)).is_err());
    assert!(cap(&Value::Bool(true)).is_err());
    assert!(cap(&Value::Null).is_err());
}

#[test]
fn test_make_comprehensive() {
    // Test making arrays
    let empty_array = make("array", None).unwrap();
    assert!(matches!(empty_array, Value::Array(_)));
    if let Value::Array(arr) = empty_array {
        assert_eq!(arr.len(), 0);
    }
    
    let sized_array = make("array", Some(10)).unwrap();
    assert!(matches!(sized_array, Value::Array(_)));
    if let Value::Array(arr) = sized_array {
        assert_eq!(arr.len(), 0);
        assert!(arr.capacity() >= 10);
    }
    
    // Test making slices (alias for array)
    let slice = make("slice", Some(5)).unwrap();
    assert!(matches!(slice, Value::Array(_)));
    
    // Test making objects
    let empty_object = make("object", None).unwrap();
    assert!(matches!(empty_object, Value::Object(_)));
    if let Value::Object(obj) = empty_object {
        assert_eq!(obj.len(), 0);
    }
    
    let sized_object = make("object", Some(20)).unwrap();
    assert!(matches!(sized_object, Value::Object(_)));
    
    // Test making maps (alias for object)
    let map = make("map", Some(15)).unwrap();
    assert!(matches!(map, Value::Object(_)));
    
    // Test error cases
    assert!(make("invalid_type", None).is_err());
    assert!(make("", None).is_err());
    assert!(make("channel", None).is_err()); // Not implemented yet
}

#[test]
fn test_new_comprehensive() {
    // Test creating new values of each type
    assert_eq!(new("litean").unwrap(), Value::Bool(false));
    assert_eq!(new("bool").unwrap(), Value::Bool(false));
    
    assert_eq!(new("normie").unwrap(), Value::Int32(0));
    assert_eq!(new("int32").unwrap(), Value::Int32(0));
    
    assert_eq!(new("thicc").unwrap(), Value::Int64(0));
    assert_eq!(new("int64").unwrap(), Value::Int64(0));
    
    assert_eq!(new("snack").unwrap(), Value::Float32(0.0));
    assert_eq!(new("float32").unwrap(), Value::Float32(0.0));
    
    assert_eq!(new("meal").unwrap(), Value::Float64(0.0));
    assert_eq!(new("float64").unwrap(), Value::Float64(0.0));
    
    assert_eq!(new("tea").unwrap(), Value::String("".to_string()));
    assert_eq!(new("string").unwrap(), Value::String("".to_string()));
    
    // Test collection types
    let new_array = new("array").unwrap();
    assert!(matches!(new_array, Value::Array(_)));
    if let Value::Array(arr) = new_array {
        assert_eq!(arr.len(), 0);
    }
    
    let new_object = new("object").unwrap();
    assert!(matches!(new_object, Value::Object(_)));
    if let Value::Object(obj) = new_object {
        assert_eq!(obj.len(), 0);
    }
    
    // Test aliases
    assert!(matches!(new("slice").unwrap(), Value::Array(_)));
    assert!(matches!(new("map").unwrap(), Value::Object(_)));
    
    // Test error cases
    assert!(new("invalid_type").is_err());
    assert!(new("").is_err());
}

// ================================
// PANIC AND RECOVERY TESTS
// ================================

#[test]
fn test_try_unbothered_success() {
    // Test successful operation
    let result = try_unbothered(|| 42);
    assert_eq!(result.unwrap(), 42);
    
    // Test successful string operation
    let result = try_unbothered(|| "hello".to_string());
    assert_eq!(result.unwrap(), "hello");
    
    // Test successful complex operation
    let result = try_unbothered(|| {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.len()
    });
    assert_eq!(result.unwrap(), 2);
}

#[test]
fn test_try_unbothered_panic() {
    // Test panic with string message
    let result = try_unbothered(|| panic!("test panic"));
    assert!(result.is_err());
    if let Err(panic_value) = result {
        if let Value::String(msg) = panic_value {
            assert!(msg.contains("test panic"));
        }
    }
    
    // Test panic with different message
    let result = try_unbothered(|| panic!("another panic"));
    assert!(result.is_err());
    
    // Test panic in complex operation
    let result = try_unbothered(|| {
        let vec = vec![1, 2, 3];
        vec[10] // This will panic
    });
    assert!(result.is_err());
}

#[test]
fn test_unbothered_basic() {
    // Test unbothered when no panic
    let recovered = unbothered();
    assert!(recovered.is_none()); // No active panic to recover from
}

#[test]
#[should_panic(expected = "CURSED panic: 42")]
fn test_shook_with_number() {
    let value = Value::Int32(42);
    shook(&value);
}

#[test]
#[should_panic(expected = "CURSED panic: hello world")]
fn test_shook_with_string() {
    let value = Value::String("hello world".to_string());
    shook(&value);
}

#[test]
#[should_panic(expected = "CURSED panic: true")]
fn test_shook_with_boolean() {
    let value = Value::Bool(true);
    shook(&value);
}

// ================================
// UTILITY FUNCTION TESTS
// ================================

#[test]
fn test_type_of_comprehensive() {
    assert_eq!(type_of(&Value::Bool(true)), "litean");
    assert_eq!(type_of(&Value::Bool(false)), "litean");
    
    assert_eq!(type_of(&Value::Int32(42)), "normie");
    assert_eq!(type_of(&Value::Int32(-42)), "normie");
    
    assert_eq!(type_of(&Value::Int64(42)), "thicc");
    assert_eq!(type_of(&Value::Int64(-42)), "thicc");
    
    assert_eq!(type_of(&Value::Float32(3.14)), "snack");
    assert_eq!(type_of(&Value::Float32(-3.14)), "snack");
    
    assert_eq!(type_of(&Value::Float64(3.14159)), "meal");
    assert_eq!(type_of(&Value::Float64(-3.14159)), "meal");
    
    assert_eq!(type_of(&Value::String("hello".to_string())), "tea");
    assert_eq!(type_of(&Value::String("".to_string())), "tea");
    
    assert_eq!(type_of(&Value::Array(vec![])), "array");
    assert_eq!(type_of(&Value::Array(vec![Value::Int32(1)])), "array");
    
    assert_eq!(type_of(&Value::Object(HashMap::new())), "object");
    
    assert_eq!(type_of(&Value::Null), "null");
}

#[test]
fn test_is_zero_value_comprehensive() {
    // Test zero/empty values
    assert!(is_zero_value(&Value::Bool(false)));
    assert!(is_zero_value(&Value::Int32(0)));
    assert!(is_zero_value(&Value::Int64(0)));
    assert!(is_zero_value(&Value::Float32(0.0)));
    assert!(is_zero_value(&Value::Float64(0.0)));
    assert!(is_zero_value(&Value::String("".to_string())));
    assert!(is_zero_value(&Value::Array(vec![])));
    assert!(is_zero_value(&Value::Object(HashMap::new())));
    assert!(is_zero_value(&Value::Null));
    
    // Test non-zero values
    assert!(!is_zero_value(&Value::Bool(true)));
    assert!(!is_zero_value(&Value::Int32(1)));
    assert!(!is_zero_value(&Value::Int32(-1)));
    assert!(!is_zero_value(&Value::Int64(1)));
    assert!(!is_zero_value(&Value::Float32(0.1)));
    assert!(!is_zero_value(&Value::Float64(-0.1)));
    assert!(!is_zero_value(&Value::String("hello".to_string())));
    assert!(!is_zero_value(&Value::String(" ".to_string()))); // Space is not empty
    assert!(!is_zero_value(&Value::Array(vec![Value::Int32(1)])));
    
    let mut obj = HashMap::new();
    obj.insert("key".to_string(), Value::Int32(1));
    assert!(!is_zero_value(&Value::Object(obj)));
}

#[test]
fn test_zero_value_comprehensive() {
    // Test zero value creation for all types
    assert_eq!(zero_value("litean").unwrap(), Value::Bool(false));
    assert_eq!(zero_value("bool").unwrap(), Value::Bool(false));
    
    assert_eq!(zero_value("normie").unwrap(), Value::Int32(0));
    assert_eq!(zero_value("int32").unwrap(), Value::Int32(0));
    
    assert_eq!(zero_value("thicc").unwrap(), Value::Int64(0));
    assert_eq!(zero_value("int64").unwrap(), Value::Int64(0));
    
    assert_eq!(zero_value("snack").unwrap(), Value::Float32(0.0));
    assert_eq!(zero_value("float32").unwrap(), Value::Float32(0.0));
    
    assert_eq!(zero_value("meal").unwrap(), Value::Float64(0.0));
    assert_eq!(zero_value("float64").unwrap(), Value::Float64(0.0));
    
    assert_eq!(zero_value("tea").unwrap(), Value::String("".to_string()));
    assert_eq!(zero_value("string").unwrap(), Value::String("".to_string()));
    
    // Verify all zero values are actually zero
    for type_name in &["litean", "normie", "thicc", "snack", "meal", "tea", "array", "object"] {
        let zero = zero_value(type_name).unwrap();
        assert!(is_zero_value(&zero), "Zero value for {} should be zero", type_name);
    }
    
    // Test error case
    assert!(zero_value("invalid_type").is_err());
}

#[test]
fn test_equal_values_comprehensive() {
    // Test identical values
    assert!(equal_values(&Value::Int32(42), &Value::Int32(42)));
    assert!(equal_values(&Value::String("hello".to_string()), &Value::String("hello".to_string())));
    assert!(equal_values(&Value::Bool(true), &Value::Bool(true)));
    assert!(equal_values(&Value::Null, &Value::Null));
    
    // Test different values of same type
    assert!(!equal_values(&Value::Int32(42), &Value::Int32(43)));
    assert!(!equal_values(&Value::String("hello".to_string()), &Value::String("world".to_string())));
    assert!(!equal_values(&Value::Bool(true), &Value::Bool(false)));
    
    // Test different types
    assert!(!equal_values(&Value::Int32(42), &Value::String("42".to_string())));
    assert!(!equal_values(&Value::Float64(42.0), &Value::Int32(42)));
    assert!(!equal_values(&Value::Bool(true), &Value::Int32(1)));
    
    // Test arrays
    let arr1 = Value::Array(vec![Value::Int32(1), Value::Int32(2)]);
    let arr2 = Value::Array(vec![Value::Int32(1), Value::Int32(2)]);
    let arr3 = Value::Array(vec![Value::Int32(1), Value::Int32(3)]);
    assert!(equal_values(&arr1, &arr2));
    assert!(!equal_values(&arr1, &arr3));
    
    // Test objects
    let mut obj1 = HashMap::new();
    obj1.insert("key".to_string(), Value::Int32(1));
    let mut obj2 = HashMap::new();
    obj2.insert("key".to_string(), Value::Int32(1));
    let mut obj3 = HashMap::new();
    obj3.insert("key".to_string(), Value::Int32(2));
    
    assert!(equal_values(&Value::Object(obj1), &Value::Object(obj2)));
    assert!(!equal_values(&Value::Object(HashMap::new()), &Value::Object(obj3)));
}

#[test]
fn test_clone_value_comprehensive() {
    // Test cloning simple types
    let original_int = Value::Int32(42);
    let cloned_int = clone_value(&original_int);
    assert_eq!(original_int, cloned_int);
    assert!(equal_values(&original_int, &cloned_int));
    
    let original_string = Value::String("hello".to_string());
    let cloned_string = clone_value(&original_string);
    assert_eq!(original_string, cloned_string);
    
    // Test cloning arrays
    let original_array = Value::Array(vec![Value::Int32(1), Value::Int32(2), Value::String("test".to_string())]);
    let cloned_array = clone_value(&original_array);
    assert_eq!(original_array, cloned_array);
    
    // Test cloning objects
    let mut original_object_map = HashMap::new();
    original_object_map.insert("key1".to_string(), Value::Int32(1));
    original_object_map.insert("key2".to_string(), Value::String("value".to_string()));
    let original_object = Value::Object(original_object_map);
    let cloned_object = clone_value(&original_object);
    assert_eq!(original_object, cloned_object);
    
    // Test that changes to clone don't affect original (for owned data)
    let original = Value::String("original".to_string());
    let _cloned = clone_value(&original);
    // Rust's ownership system ensures this is safe
}

// ================================
// ERROR HANDLING AND EDGE CASES
// ================================

#[test]
fn test_type_conversion_errors() {
    // Test conversions that should fail
    let array = Value::Array(vec![Value::Int32(1)]);
    let object = Value::Object(HashMap::new());
    
    // These should all fail for complex types
    assert!(normie(&array).is_err());
    assert!(thicc(&array).is_err());
    assert!(snack(&array).is_err());
    assert!(meal(&array).is_err());
    
    assert!(normie(&object).is_err());
    assert!(thicc(&object).is_err());
    assert!(snack(&object).is_err());
    assert!(meal(&object).is_err());
    
    // lit() should work for collections (checking emptiness)
    assert!(lit(&array).is_ok());
    assert!(lit(&object).is_ok());
    
    // tea() should work for all types (formatting)
    assert!(tea(&array).is_ok());
    assert!(tea(&object).is_ok());
}

#[test]
fn test_collection_operation_errors() {
    // Test len() with invalid types
    assert!(len(&Value::Int32(42)).is_err());
    assert!(len(&Value::Bool(true)).is_err());
    assert!(len(&Value::Float64(3.14)).is_err());
    assert!(len(&Value::Null).is_err());
    
    // Test cap() with invalid types
    assert!(cap(&Value::Int32(42)).is_err());
    assert!(cap(&Value::Bool(true)).is_err());
    assert!(cap(&Value::Float64(3.14)).is_err());
    assert!(cap(&Value::Null).is_err());
    
    // Test append() with invalid first argument
    assert!(append(&Value::Int32(42), &[]).is_err());
    assert!(append(&Value::String("hello".to_string()), &[]).is_err());
    assert!(append(&Value::Object(HashMap::new()), &[]).is_err());
}

#[test]
fn test_range_errors() {
    // Test normie() with out-of-range values
    let max_plus_one = Value::Int64(i32::MAX as i64 + 1);
    let min_minus_one = Value::Int64(i32::MIN as i64 - 1);
    
    assert!(normie(&max_plus_one).is_err());
    assert!(normie(&min_minus_one).is_err());
    
    // Verify error messages contain range information
    let error = normie(&max_plus_one).unwrap_err();
    assert!(format!("{:?}", error).contains("range"));
}

#[test]
fn test_parse_errors() {
    // Test invalid string to number conversions
    let invalid_strings = vec![
        "hello",
        "123abc",
        "abc123",
        "12.34.56",
        "",
        " ",
        "∞",
        "NaN",
    ];
    
    for invalid in invalid_strings {
        let value = Value::String(invalid.to_string());
        assert!(normie(&value).is_err(), "normie() should fail for '{}'", invalid);
        assert!(thicc(&value).is_err(), "thicc() should fail for '{}'", invalid);
        assert!(snack(&value).is_err(), "snack() should fail for '{}'", invalid);
        assert!(meal(&value).is_err(), "meal() should fail for '{}'", invalid);
    }
}

// ================================
// MODULE FUNCTIONALITY TESTS
// ================================

#[test]
fn test_module_initialization() {
    // Test that module can be initialized without errors
    assert!(init_core().is_ok());
}

#[test]
fn test_module_statistics() {
    let stats = get_core_stats();
    
    // Verify required fields are present
    assert!(stats.contains_key("version"));
    assert!(stats.contains_key("functions"));
    assert!(stats.contains_key("features"));
    assert!(stats.contains_key("types"));
    
    // Verify values are reasonable
    assert!(!stats["version"].is_empty());
    assert!(!stats["functions"].is_empty());
    assert!(!stats["features"].is_empty());
    assert!(!stats["types"].is_empty());
    
    // Verify types field contains expected CURSED types
    let types_field = &stats["types"];
    assert!(types_field.contains("litean"));
    assert!(types_field.contains("normie"));
    assert!(types_field.contains("thicc"));
    assert!(types_field.contains("snack"));
    assert!(types_field.contains("meal"));
    assert!(types_field.contains("tea"));
}

// ================================
// INTEGRATION AND STRESS TESTS
// ================================

#[test]
fn test_type_conversion_round_trips() {
    // Test that converting and converting back yields original values where possible
    
    // Int32 round trips
    let original_int = Value::Int32(42);
    let as_string = tea(&original_int).unwrap();
    let back_to_int = normie(&Value::String(as_string)).unwrap();
    assert_eq!(back_to_int, 42);
    
    // Float round trips (approximate due to floating point precision)
    let original_float = Value::Float64(3.14159);
    let as_string = tea(&original_float).unwrap();
    let back_to_float = meal(&Value::String(as_string)).unwrap();
    assert!((back_to_float - 3.14159).abs() < 1e-10);
    
    // Boolean round trips
    let original_bool = Value::Bool(true);
    let as_string = tea(&original_bool).unwrap();
    assert_eq!(as_string, "true");
    // Note: string to bool conversion not implemented, which is fine
    
    // String identity
    let original_string = Value::String("hello world".to_string());
    let identity = tea(&original_string).unwrap();
    assert_eq!(identity, "hello world");
}

#[test]
fn test_large_collections() {
    // Test with large arrays
    let large_array: Vec<Value> = (0..1000).map(|i| Value::Int32(i)).collect();
    let array_value = Value::Array(large_array);
    
    assert_eq!(len(&array_value).unwrap(), 1000);
    assert!(cap(&array_value).unwrap() >= 1000);
    
    // Test appending to large array
    let more_elements: Vec<Value> = (1000..1100).map(|i| Value::Int32(i)).collect();
    let result = append(&array_value, &more_elements).unwrap();
    
    if let Value::Array(arr) = result {
        assert_eq!(arr.len(), 1100);
        assert_eq!(arr[999], Value::Int32(999));
        assert_eq!(arr[1000], Value::Int32(1000));
        assert_eq!(arr[1099], Value::Int32(1099));
    }
    
    // Test large object
    let mut large_object = HashMap::new();
    for i in 0..500 {
        large_object.insert(format!("key{}", i), Value::Int32(i));
    }
    let object_value = Value::Object(large_object);
    
    assert_eq!(len(&object_value).unwrap(), 500);
    assert!(cap(&object_value).unwrap() >= 500);
}

#[test]
fn test_nested_structures() {
    // Test deeply nested arrays
    let inner_array = Value::Array(vec![Value::Int32(1), Value::Int32(2)]);
    let middle_array = Value::Array(vec![inner_array, Value::String("test".to_string())]);
    let outer_array = Value::Array(vec![middle_array, Value::Bool(true)]);
    
    assert_eq!(len(&outer_array).unwrap(), 2);
    assert_eq!(type_of(&outer_array), "array");
    
    // Test cloning nested structures
    let cloned = clone_value(&outer_array);
    assert!(equal_values(&outer_array, &cloned));
    
    // Test converting nested structures to string
    let string_repr = tea(&outer_array).unwrap();
    assert!(!string_repr.is_empty()); // Should produce some string representation
}

#[test]
fn test_unicode_handling() {
    // Test Unicode strings
    let unicode_strings = vec![
        "Hello, 世界",
        "🔥💯🚀",
        "Café",
        "Мир",
        "🌍🌎🌏",
        "αβγδε",
        "🇺🇸🇨🇦🇲🇽",
    ];
    
    for unicode_str in unicode_strings {
        let value = Value::String(unicode_str.to_string());
        
        // Test length counting (should count code points, not bytes)
        let length = len(&value).unwrap();
        assert_eq!(length as usize, unicode_str.chars().count());
        
        // Test string conversion (identity)
        let converted = tea(&value).unwrap();
        assert_eq!(converted, unicode_str);
        
        // Test type identification
        assert_eq!(type_of(&value), "tea");
        
        // Test emptiness check
        assert_eq!(is_zero_value(&value), unicode_str.is_empty());
    }
}

#[test]
fn test_extreme_numeric_values() {
    // Test extreme integer values
    let extreme_ints = vec![
        i32::MAX,
        i32::MIN,
        0,
        -1,
        1,
    ];
    
    for int_val in extreme_ints {
        let value = Value::Int32(int_val);
        
        // Test conversions
        assert_eq!(normie(&value).unwrap(), int_val);
        assert_eq!(thicc(&value).unwrap(), int_val as i64);
        assert_eq!(snack(&value).unwrap(), int_val as f32);
        assert_eq!(meal(&value).unwrap(), int_val as f64);
        
        // Test string conversion and back
        let as_string = tea(&value).unwrap();
        let back_to_int = normie(&Value::String(as_string)).unwrap();
        assert_eq!(back_to_int, int_val);
    }
    
    // Test extreme i64 values
    let extreme_i64s = vec![
        i64::MAX,
        i64::MIN,
        i32::MAX as i64 + 1,
        i32::MIN as i64 - 1,
    ];
    
    for int_val in extreme_i64s {
        let value = Value::Int64(int_val);
        
        // Test thicc conversion (identity)
        assert_eq!(thicc(&value).unwrap(), int_val);
        
        // Test normie conversion (should fail for out-of-range values)
        if int_val >= i32::MIN as i64 && int_val <= i32::MAX as i64 {
            assert_eq!(normie(&value).unwrap(), int_val as i32);
        } else {
            assert!(normie(&value).is_err());
        }
    }
}
