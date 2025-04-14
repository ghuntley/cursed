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
        let result = stdlib::lit(Object::Integer(0));
        assert_eq!(result, Object::Boolean(false));

        let result = stdlib::lit(Object::Integer(42));
        assert_eq!(result, Object::Boolean(true));

        let result = stdlib::lit(Object::String("hello".to_string()));
        assert_eq!(result, Object::Boolean(true));

        let result = stdlib::lit(Object::String("".to_string()));
        assert_eq!(result, Object::Boolean(false));

        // Test normie (int32 conversion)
        let result = stdlib::normie(Object::Float(42.7));
        assert_eq!(result, Object::Integer(42));

        let result = stdlib::normie(Object::Boolean(true));
        assert_eq!(result, Object::Integer(1));

        let result = stdlib::normie(Object::String("123".to_string()));
        assert_eq!(result, Object::Integer(123));

        // Test thicc (int64 conversion)
        let result = stdlib::thicc(Object::Float(42.7));
        assert_eq!(result, Object::Integer(42));

        // Test snack (float32 conversion)
        let result = stdlib::snack(Object::Integer(42));
        assert!(matches!(result, Object::Float(f) if f == 42.0));

        // Test meal (float64 conversion)
        let result = stdlib::meal(Object::Boolean(true));
        assert!(matches!(result, Object::Float(f) if f == 1.0));

        // Test tea (string conversion)
        let result = stdlib::tea(Object::Integer(42));
        assert_eq!(result, Object::String("42".to_string()));
    }

    #[test]
    fn test_len_and_cap() {
        // Test len with string
        let result = stdlib::len(Object::String("hello".to_string()));
        assert_eq!(result, Object::Integer(5));

        // Test len with array
        let array = Object::Array(vec![Object::Integer(1), Object::Integer(2)]);
        let result = stdlib::len(array.clone());
        assert_eq!(result, Object::Integer(2));

        // Test cap with array
        let mut vec = Vec::with_capacity(10);
        vec.push(Object::Integer(1));
        vec.push(Object::Integer(2));
        let array = Object::Array(vec);
        let result = stdlib::cap(array);
        assert_eq!(result, Object::Integer(10));
    }

    #[test]
    fn test_append() {
        // Test append with array
        let original = Object::Array(vec![Object::Integer(1), Object::Integer(2)]);
        let elems = vec![Object::Integer(3), Object::Integer(4)];
        
        let result = stdlib::append(original.clone(), elems);
        
        // Check original is unchanged
        if let Object::Array(arr) = original {
            assert_eq!(arr.len(), 2);
        } else {
            panic!("Expected array");
        }
        
        // Check result has all elements
        if let Object::Array(arr) = result {
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
        let result = stdlib::make("slice", Some(3), None);
        if let Object::Array(arr) = result {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr.capacity(), 3);
            // Check all elements are null
            for elem in arr {
                assert_eq!(elem, Object::Null);
            }
        } else {
            panic!("Expected array");
        }
        
        // Test make with zero size
        let result = stdlib::make("slice", Some(0), None);
        if let Object::Array(arr) = result {
            assert_eq!(arr.len(), 0);
        } else {
            panic!("Expected array");
        }
        
        // Test make map (placeholder implementation)
        let result = stdlib::make("map", None, None);
        assert!(matches!(result, Object::Array(arr) if arr.is_empty()));
    }
    
    #[test]
    fn test_new() {
        // We need to update these tests since the Object::Pointer type doesn't exist
        // Let's just test that we get the right type of values
        
        let result = stdlib::new("normie");
        assert!(matches!(result, Object::Integer(0)));
        
        let result = stdlib::new("tea");
        assert!(matches!(result, Object::String(s) if s.is_empty()));
        
        let result = stdlib::new("lit");
        assert!(matches!(result, Object::Boolean(false)));
    }
    
    #[test]
    fn test_panic_and_recover() {
        // Test that panic causes a panic
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            stdlib::panic(Object::String("Test panic".to_string()));
        }));
        assert!(result.is_err());
        
        // Test recover when not in a panic
        let result = stdlib::recover();
        assert_eq!(result, Object::Null);
        
        // Test recover during a panic is difficult to test directly in unit tests
        // A more complete test would be part of the language integration tests
    }
}