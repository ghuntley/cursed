//! Tests for the core standard library module
//! This file contains tests for the core functionality that is automatically
//! available in all CURSED programs.

#[cfg(test)]
mod tests {
    use cursed::object::Object;
    use cursed::stdlib;
    use std::panic;
    use std::panic::AssertUnwindSafe;

    #[test]
    fn test_type_conversions() {
        // Test lit (boolean conversion)
        let arg = std::rc::Rc::new(Object::Integer(0));
        let result = stdlib::lit(&[arg]).unwrap();
        assert_eq!(*result, Object::Boolean(false));

        let arg = std::rc::Rc::new(Object::Integer(42));
        let result = stdlib::lit(&[arg]).unwrap();
        assert_eq!(*result, Object::Boolean(true));

        let arg = std::rc::Rc::new(Object::String("hello".to_string()));
        let result = stdlib::lit(&[arg]).unwrap();
        assert_eq!(*result, Object::Boolean(true));

        let arg = std::rc::Rc::new(Object::String("".to_string()));
        let result = stdlib::lit(&[arg]).unwrap();
        assert_eq!(*result, Object::Boolean(false));

        // Test normie (int32 conversion)
        let arg = std::rc::Rc::new(Object::Float(42.7));
        let result = stdlib::normie(&[arg]).unwrap();
        assert_eq!(*result, Object::Integer(42));

        let arg = std::rc::Rc::new(Object::Boolean(true));
        let result = stdlib::normie(&[arg]).unwrap();
        assert_eq!(*result, Object::Integer(1));

        let arg = std::rc::Rc::new(Object::String("123".to_string()));
        let result = stdlib::normie(&[arg]).unwrap();
        assert_eq!(*result, Object::Integer(123));

        // Test thicc (int64 conversion)
        let arg = std::rc::Rc::new(Object::Float(42.7));
        let result = stdlib::thicc(&[arg]).unwrap();
        assert_eq!(*result, Object::Integer(42));

        // Test snack (float32 conversion)
        let arg = std::rc::Rc::new(Object::Integer(42));
        let result = stdlib::snack(&[arg]).unwrap();
        if let Object::Float(f) = &*result {
            assert_eq!(*f, 42.0);
        } else {
            panic!("Expected float");
        }

        // Test meal (float64 conversion)
        let arg = std::rc::Rc::new(Object::Boolean(true));
        let result = stdlib::meal(&[arg]).unwrap();
        if let Object::Float(f) = &*result {
            assert!(f == &1.0);
        } else {
            panic!("Expected float");
        }

        // Test tea (string conversion)
        let arg = std::rc::Rc::new(Object::Integer(42));
        let result = stdlib::tea(&[arg]).unwrap();
        assert_eq!(*result, Object::String("42".to_string()));
    }

    #[test]
    fn test_len_and_cap() {
        // Test len with string
        let string_obj = std::rc::Rc::new(Object::String("hello".to_string()));
        let result = stdlib::len(&[string_obj]).unwrap();
        assert_eq!(*result, Object::Integer(5));

        // Test len with array
        let array = Object::Array(vec![Object::Integer(1), Object::Integer(2)]);
        let array_obj = std::rc::Rc::new(array.clone());
        let result = stdlib::len(&[array_obj]).unwrap();
        assert_eq!(*result, Object::Integer(2));

        // Test cap with array
        let mut vec = Vec::with_capacity(10);
        vec.push(Object::Integer(1));
        vec.push(Object::Integer(2));
        let array = Object::Array(vec);
        let array_obj = std::rc::Rc::new(array);
        let result = stdlib::cap(&[array_obj]).unwrap();
        assert_eq!(*result, Object::Integer(10));
    }

    #[test]
    fn test_append() {
        // Test append with array
        let original = std::rc::Rc::new(Object::Array(vec![Object::Integer(1), Object::Integer(2)]));
        let elem1 = std::rc::Rc::new(Object::Integer(3));
        let elem2 = std::rc::Rc::new(Object::Integer(4));
        
        let args = vec![original.clone(), elem1, elem2];
        let result = stdlib::append(&args).unwrap();
        
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
    }
    
    #[test]
    fn test_make() {
        // Test make slice
        let type_arg = std::rc::Rc::new(Object::String("slice".to_string()));
        let size_arg = std::rc::Rc::new(Object::Integer(3));
        
        let result = stdlib::make(&[type_arg, size_arg]).unwrap();
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
        let type_arg = std::rc::Rc::new(Object::String("slice".to_string()));
        let size_arg = std::rc::Rc::new(Object::Integer(0));
        
        let result = stdlib::make(&[type_arg, size_arg]).unwrap();
        if let Object::Array(arr) = &*result {
            assert_eq!(arr.len(), 0);
        } else {
            panic!("Expected array");
        }
        
        // Test make map (placeholder implementation)
        let type_arg = std::rc::Rc::new(Object::String("map".to_string()));
        
        let result = stdlib::make(&[type_arg]).unwrap();
        if let Object::Array(arr) = &*result {
            assert!(arr.is_empty());
        } else {
            panic!("Expected array");
        }
    }
    
    #[test]
    fn test_new() {
        // We need to update these tests since the Object::Pointer type doesn't exist
        // Let's just test that we get the right type of values
        
        let type_arg = std::rc::Rc::new(Object::String("normie".to_string()));
        let result = stdlib::core_new(&[type_arg]).unwrap();
        if let Object::Integer(i) = &*result {
            assert_eq!(*i, 0);
        } else {
            panic!("Expected integer");
        }
        
        let type_arg = std::rc::Rc::new(Object::String("tea".to_string()));
        let result = stdlib::core_new(&[type_arg]).unwrap();
        if let Object::String(s) = &*result {
            assert!(s.is_empty());
        } else {
            panic!("Expected string");
        }
        
        let type_arg = std::rc::Rc::new(Object::String("lit".to_string()));
        let result = stdlib::core_new(&[type_arg]).unwrap();
        if let Object::Boolean(b) = &*result {
            assert_eq!(*b, false);
        } else {
            panic!("Expected boolean");
        }
    }
    
    #[test]
    fn test_panic_and_recover() {
        // Test that panic causes a panic
        let panic_message = std::rc::Rc::new(Object::String("Test panic".to_string()));
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            stdlib::panic(&[panic_message]).unwrap();
        }));
        assert!(result.is_err());
        
        // Test recover when not in a panic
        let result = stdlib::recover(&[]).unwrap();
        assert!(matches!(*result, Object::Null));
        
        // Test recover during a panic is difficult to test directly in unit tests
        // A more complete test would be part of the language integration tests
    }
}