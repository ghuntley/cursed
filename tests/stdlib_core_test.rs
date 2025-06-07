use cursed::Error;
use cursed::object::Object;
use cursed::stdlib::core;
use std::sync::Arc;

// Tests for the core standard library module
// This file contains tests for the Core module functionality

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lit_function() {
        let arg = Arc::new(Object::Integer(0));
        let result = core::lit(&[arg]).unwrap();
        assert_eq!(*result, Object::Boolean(false));
        
        let arg = Arc::new(Object::Integer(42));
        let result = core::lit(&[arg]).unwrap();
        assert_eq!(*result, Object::Boolean(true));
        
        let arg = Arc::new(Object::String("hello".to_string()));
        let result = core::lit(&[arg]).unwrap();
        assert_eq!(*result, Object::Boolean(true));
        
        let arg = Arc::new(Object::String("".to_string()));
        let result = core::lit(&[arg]).unwrap();
        assert_eq!(*result, Object::Boolean(false));
        
        // Test error case
        let result = core::lit(&[]);
        assert!(result.is_err())
    }
    
    #[test]
    fn test_normie_function() {
        let arg = Arc::new(Object::Float(42.7));
        let result = core::normie(&[arg]).unwrap();
        assert_eq!(*result, Object::Integer(42));
        
        let arg = Arc::new(Object::Boolean(true));
        let result = core::normie(&[arg]).unwrap();
        assert_eq!(*result, Object::Integer(1));
        
        let arg = Arc::new(Object::String("123".to_string()));
        let result = core::normie(&[arg]).unwrap();
        assert_eq!(*result, Object::Integer(123));
        
        // Test error case
        let result = core::normie(&[]);
        assert!(result.is_err())
    }
    
    #[test]
    fn test_thicc_function() {
        let arg = Arc::new(Object::Float(42.7));
        let result = core::thicc(&[arg]).unwrap();
        assert_eq!(*result, Object::Integer(42));
        
        // Test error case
        let result = core::thicc(&[]);
        assert!(result.is_err())
    }
    
    #[test]
    fn test_snack_function() {
        let arg = Arc::new(Object::Integer(42));
        let result = core::snack(&[arg]).unwrap();
        if let Object::Float(f) = &*result {
            assert_eq!(*f, 42.0);
        } else {
            panic!("Expected float");
        }
        
        // Test error case
        let result = core::snack(&[]);
        assert!(result.is_err())
    }
    
    #[test]
    fn test_meal_function() {
        let arg = Arc::new(Object::Boolean(true));
        let result = core::meal(&[arg]).unwrap();
        if let Object::Float(f) = &*result {
            assert_eq!(*f, 1.0);
        } else {
            panic!("Expected float");
        }
        
        // Test error case
        let result = core::meal(&[]);
        assert!(result.is_err())
    }
    
    #[test]
    fn test_tea_function() {
        let arg = Arc::new(Object::Integer(42));
        let result = core::tea(&[arg]).unwrap();
        assert_eq!(*result, Object::String("42".to_string()));
        
        // Test error case
        let result = core::tea(&[]);
        assert!(result.is_err())
    }
    
    #[test]
    fn test_len_function() {
        let arg = Arc::new(Object::String("hello".to_string()));
        let result = core::len(&[arg]).unwrap();
        assert_eq!(*result, Object::Integer(5));
        
        let array = Arc::new(Object::Array(vec![Object::Integer(1), Object::Integer(2)]));
        let result = core::len(&[array]).unwrap();
        assert_eq!(*result, Object::Integer(2));
        
        // Test error case
        let result = core::len(&[]);
        assert!(result.is_err())
    }
    
    #[test]
    fn test_cap_function() {
        let mut vec = Vec::with_capacity(10);
        vec.push(Object::Integer(1));
        vec.push(Object::Integer(2));
        let array = Arc::new(Object::Array(vec));
        let result = core::cap(&[array]).unwrap();
        assert_eq!(*result, Object::Integer(10));
        
        // Test error case
        let result = core::cap(&[]);
        assert!(result.is_err())
    }
    
    #[test]
    fn test_append_function() {
        let original = Arc::new(Object::Array(vec![Object::Integer(1), Object::Integer(2)]));
        let elem1 = Arc::new(Object::Integer(3));
        let elem2 = Arc::new(Object::Integer(4));
        
        let args = vec![original.clone(), elem1, elem2];
        let result = core::append(&args).unwrap();
        
        // Check original is unchanged
        if let Object::Array(arr) = &*original {
            assert_eq!(arr.len(), 2);
        } else {
            panic!("Expected array");
        }
        
        // Check result has all elements
        if let Object::Array(arr) = &*result {
            assert_eq!(arr.len(), 4);
            assert_eq!(arr[0], Object::Integer(1));
            assert_eq!(arr[1], Object::Integer(2));
            assert_eq!(arr[2], Object::Integer(3));
            assert_eq!(arr[3], Object::Integer(4));
        } else {
            panic!("Expected array");
        }
        
        // Test error case
        let result = core::append(&[]);
        assert!(result.is_err());
        
        // Test error case with non-array first arg
        let invalid_args = vec![Arc::new(Object::Integer(1)), Arc::new(Object::Integer(2))];
        let result = core::append(&invalid_args);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_make_function() {
        // Test make slice
        let type_arg = Arc::new(Object::String("slice".to_string()));
        let size_arg = Arc::new(Object::Integer(3));
        
        let result = core::make(&[type_arg, size_arg]).unwrap();
        if let Object::Array(arr) = &*result {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr.capacity(), 3);
            // Check all elements are null
            for elem in arr {
                assert_eq!(*elem, Object::Null);
            }
        } else {
            panic!("Expected array");
        }
        
        // Test make with zero size
        let type_arg = Arc::new(Object::String("slice".to_string()));
        let size_arg = Arc::new(Object::Integer(0));
        
        let result = core::make(&[type_arg, size_arg]).unwrap();
        if let Object::Array(arr) = &*result {
            assert_eq!(arr.len(), 0);
        } else {
            panic!("Expected array");
        }
        
        // Test make map (placeholder implementation)
        let type_arg = Arc::new(Object::String("map".to_string()));
        
        let result = core::make(&[type_arg]).unwrap();
        if let Object::Array(arr) = &*result {
            assert!(arr.is_empty())
        } else {
            panic!("Expected array");
        }
        
        // Test error cases
        let result = core::make(&[]);
        assert!(result.is_err());
        
        let invalid_type = Arc::new(Object::Integer(1));
        let result = core::make(&[invalid_type]);
        assert!(result.is_err());
        
        let invalid_type = Arc::new(Object::String("invalid_type".to_string()));
        let result = core::make(&[invalid_type]);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_new_function() {
        let type_arg = Arc::new(Object::String("normie".to_string()));
        let result = core::new(&[type_arg]).unwrap();
        if let Object::Integer(i) = &*result {
            assert_eq!(*i, 0);
        } else {
            panic!("Expected integer");
        }
        
        let type_arg = Arc::new(Object::String("tea".to_string()));
        let result = core::new(&[type_arg]).unwrap();
        if let Object::String(s) = &*result {
            assert!(s.is_empty())
        } else {
            panic!("Expected string");
        }
        
        let type_arg = Arc::new(Object::String("lit".to_string()));
        let result = core::new(&[type_arg]).unwrap();
        if let Object::Boolean(b) = &*result {
            assert_eq!(*b, false);
        } else {
            panic!("Expected boolean");
        }
        
        // Test error cases
        let result = core::new(&[]);
        assert!(result.is_err());
        
        let invalid_type = Arc::new(Object::Integer(1));
        let result = core::new(&[invalid_type]);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_panic_and_recover() {
        // Test that panic causes a panic
        let panic_message = Arc::new(Object::String("Test panic".to_string()));
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            core::panic(&[panic_message]).unwrap()
        }));
        assert!(result.is_err());
        
        // Test recover when not in a panic
        let result = core::recover(&[]).unwrap();
        assert!(matches!(*result, Object::Null));
    }
}