//! Unit tests for the quick_test module

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use cursed::object::Object;
    use cursed::stdlib::quick_test;

    #[test]
    fn test_int_range() {
        // Test that int_range generates values within the specified range
        for _ in 0..100 {
            let val = quick_test::int_range(-10, 10);
            assert!(val >= -10 && val <= 10);
        }
    }

    #[test]
    fn test_boolean() {
        // Just check that boolean can be called without errors
        let _val = quick_test::boolean();
    }

    #[test]
    fn test_string() {
        // Check that string generates non-empty strings
        for _ in 0..10 {
            let val = quick_test::string();
            assert!(!val.is_empty());
        }
    }

    #[test]
    fn test_int_array() {
        // Test that int_array generates arrays with the correct length range and value range
        for _ in 0..10 {
            let arr = quick_test::int_array(3, 7, 0, 100);
            assert!(arr.len() >= 3 && arr.len() <= 7);
            for val in arr.iter() {
                assert!(*val >= 0 && *val <= 100);
            }
        }
    }

    #[test]
    fn test_float_range() {
        // Test that float_range generates values within the specified range
        for _ in 0..100 {
            let val = quick_test::float_range(-1.0, 1.0);
            assert!(val >= -1.0 && val <= 1.0);
        }
    }

    #[test]
    fn test_one_of_type() {
        // Test the one_of_type function with different types
        let string_val = quick_test::one_of_type("string", 5, 10);
        if let Object::String(s) = &*string_val {
            assert!(s.len() >= 5 && s.len() <= 10);
        } else {
            panic!("Expected string value, got {:?}", string_val);
        }

        let int_val = quick_test::one_of_type("int", 0, 100);
        if let Object::Integer(i) = &*int_val {
            assert!(*i >= 0 && *i <= 100);
        } else {
            panic!("Expected integer value, got {:?}", int_val);
        }
    }

    #[test]
    fn test_config() {
        // Test the default config
        let config = quick_test::Config::default();
        assert_eq!(config.max_count, 100);
        assert_eq!(config.min_success_rate, 1.0);
        assert_eq!(config.max_shrink_iters, 100);

        // Test a custom config
        let custom_config = quick_test::Config {
            max_count: 50,
            seed: Some(42),
            min_success_rate: 0.9,
            shrink: quick_test::SMART_SHRINK,
            max_shrink_iters: 50,
        };
        assert_eq!(custom_config.max_count, 50);
        assert_eq!(custom_config.seed, Some(42));
    }
}