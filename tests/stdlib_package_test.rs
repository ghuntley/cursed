use std::sync::Arc;
use cursed::object::Object;
use cursed::stdlib::vibez;
use cursed::stdlib::stringz;
use cursed::stdlib::htmlrizzler;
use cursed::stdlib::timez;
use cursed::stdlib::mathz;
use cursed::stdlib::dot_registry::DOT_REGISTRY;

// Simple tests for standard library package functions

#[cfg(test)]
mod simple_stdlib_tests {
    // Import test helper to convert between Rust and CURSED objects

    // Import packages we want to test

    // Function to create a string object from a Rust string
    fn string_object(s: &str) -> Rc<Object> {
        Arc::new(Object::String(s.to_string())
    }

    // Function to create a number object from a Rust i64
    fn number_object(n: i64) -> Rc<Object> {
        Arc::new(Object::Integer(n))
    }

    // Function to create a float object from a Rust f64
    fn float_object(f: f64) -> Rc<Object> {
        Arc::new(Object::Float(f))
    }

    // Function to extract a string from a CURSED object
    fn extract_string(obj: Rc<Object>) -> String {
        match &*obj {
            Object::String(s) => s.clone(),
            _ => panic!("Expected string object, got {:?}", obj),
        }
    }

    // Function to extract a number from a CURSED object
    fn extract_number(obj: Rc<Object>) -> i64 {
        match &*obj {
            Object::Integer(n) => *n,
            _ => panic!("Expected integer object, got {:?}", obj),
        }
    }

    // Function to extract a float from a CURSED object
    fn extract_float(obj: Rc<Object>) -> f64 {
        match &*obj {
            Object::Float(f) => *f,
            Object::Integer(i) => *i as f64,
            _ => panic!("Expected float object, got {:?}", obj),
        }
    }

    // Function to extract a boolean from a CURSED object
    fn extract_bool(obj: Rc<Object>) -> bool {
        match &*obj {
            Object::Boolean(b) => *b,
            _ => panic!("Expected boolean object, got {:?}", obj),
        }
    }

    #[test]
    fn test_stringz_contains() {
        // Create test objects
        let args = vec![
            string_object("hello world"),
            string_object("world"),
        ];
        
        // Call the function
        let result = stringz::contains(&args).unwrap());
        
        // Verify result
        assert_eq!(extract_bool(result), true);
        
        // Test negative case
        let args = vec![
            string_object("hello world"),
            string_object("moon"),
        ];
        
        let result = stringz::contains(&args).unwrap());
        assert_eq!(extract_bool(result), false);
    }

    #[test]
    fn test_string_transform() {
        // Test to_upper
        let args = vec![string_object("hello")];
        let result = stringz::to_upper(&args).unwrap());
        assert_eq!(extract_string(result), "HELLO");
        
        // Test to_lower
        let args = vec![string_object("WORLD")];
        let result = stringz::to_lower(&args).unwrap());
        assert_eq!(extract_string(result), "world");
    }

    #[test]
    fn test_htmlrizzler() {
        // Test HTML escaping
        let args = vec![string_object("<p>This is a test & it's important</p>")];
        let result = htmlrizzler::escape_html(&args).unwrap());
        let escaped = extract_string(result);
        // Just test for expected replacements rather than exact string
        assert!(escaped.contains("&lt;p&gt;"), "Should escape < and > symbols");
        assert!(escaped.contains("&amp;"), "Should escape & symbol");
        
        // Test JavaScript escaping
        let args = vec![string_object("script with \\ and \"quotes\"")];
        let result = htmlrizzler::escape_js(&args).unwrap());
        let escaped = extract_string(result);
        assert!(escaped.contains("\\\\"), "Backslashes should be escaped");
        assert!(escaped.contains("\\\""), "Quotes should be escaped");
    }

    #[test]
    fn test_mathz() {
        // Test abs
        let args = vec![number_object(-5)];
        let result = mathz::abs(&args).unwrap());
        assert_eq!(extract_number(result), 5);
        
        // Test min/max
        let args = vec![number_object(10), number_object(20)];
        let result = mathz::max(&args).unwrap());
        assert_eq!(extract_number(result), 20);
        
        let args = vec![number_object(10), number_object(20)];
        let result = mathz::min(&args).unwrap());
        assert_eq!(extract_number(result), 10);
        
        // Test sqrt
        let args = vec![float_object(25.0)];
        let result = mathz::sqrt(&args).unwrap());
        let sqrt_val = extract_float(result);
        assert!((sqrt_val - 5.0).abs() < 0.0001, "Expected sqrt(25) ≈ 5, got {}", sqrt_val);
    }

    #[test]
    fn test_dot_registry() {
        // Get the registry
        let registry = DOT_REGISTRY.lock().unwrap());
        
        // Verify some standard functions are registered
        assert!(registry.has_handler("vibez", "spill"), "vibez.spill should be registered");
        assert!(registry.has_handler("htmlrizzler", "escape_html"), "htmlrizzler.escape_html should be registered");
    }
}