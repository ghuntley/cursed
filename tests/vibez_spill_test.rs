/// Tests for Vibez spill functions - Gen Z I/O operations with CURSED flair
/// 
/// This test suite validates the spill functions added to the Vibez module,
/// ensuring all Gen Z I/O operations work correctly with CURSED Value types.

#[cfg(test)]
mod tests {
    use cursed::stdlib::vibez::*;
    use cursed::stdlib::value::Value;
    use std::io::Write;

    #[test]
    fn test_spillstr_basic_formatting() {
        // Test basic string formatting
        let result = spillstr("Hello %s!", &[Value::String("World".to_string())]).unwrap();
        assert_eq!(result, "Hello World!");
        
        // Test integer formatting
        let result = spillstr("Number: %d", &[Value::Int(42)]).unwrap();
        assert_eq!(result, "Number: 42");
        
        // Test float formatting
        let result = spillstr("Float: %f", &[Value::Float(3.14159)]).unwrap();
        assert!(result.contains("3.14159"));
        
        // Test multiple arguments
        let result = spillstr("Hello %s, you are %d years old", &[
            Value::String("Alice".to_string()),
            Value::Int(25)
        ]).unwrap();
        assert_eq!(result, "Hello Alice, you are 25 years old");
    }

    #[test]
    fn test_spillstr_format_specifiers() {
        // Test integer specifiers
        let result = spillstr("Decimal: %d", &[Value::Int(42)]).unwrap();
        assert_eq!(result, "Decimal: 42");
        
        let result = spillstr("Hex: %x", &[Value::Int(255)]).unwrap();
        assert_eq!(result, "Hex: ff");
        
        let result = spillstr("HEX: %X", &[Value::Int(255)]).unwrap();
        assert_eq!(result, "HEX: FF");
        
        let result = spillstr("Octal: %o", &[Value::Int(64)]).unwrap();
        assert_eq!(result, "Octal: 100");
        
        // Test float specifiers
        let result = spillstr("Float: %.2f", &[Value::Float(3.14159)]).unwrap();
        assert_eq!(result, "Float: 3.14");
        
        // Test character specifier
        let result = spillstr("Char: %c", &[Value::Int(65)]).unwrap();
        assert_eq!(result, "Char: A");
        
        let result = spillstr("Char: %c", &[Value::String("Hello".to_string())]).unwrap();
        assert_eq!(result, "Char: H");
        
        // Test string specifier
        let result = spillstr("String: %s", &[Value::String("test".to_string())]).unwrap();
        assert_eq!(result, "String: test");
        
        // Test value specifier (default format)
        let result = spillstr("Value: %v", &[Value::Bool(true)]).unwrap();
        assert_eq!(result, "Value: true");
    }

    #[test]
    fn test_spillstr_escaped_percent() {
        // Test escaped percent signs
        let result = spillstr("100%% complete", &[]).unwrap();
        assert_eq!(result, "100% complete");
        
        let result = spillstr("Progress: %d%%", &[Value::Int(50)]).unwrap();
        assert_eq!(result, "Progress: 50%");
        
        let result = spillstr("%%s is not a format", &[]).unwrap();
        assert_eq!(result, "%s is not a format");
    }

    #[test]
    fn test_spillstr_missing_arguments() {
        // Test with more format specifiers than arguments
        let result = spillstr("Hello %s, you are %d", &[Value::String("Alice".to_string())]).unwrap();
        assert_eq!(result, "Hello Alice, you are %d");
        
        // Test with no arguments
        let result = spillstr("No format specifiers", &[]).unwrap();
        assert_eq!(result, "No format specifiers");
        
        // Test empty format string
        let result = spillstr("", &[]).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_spillstr_type_conversions() {
        // Test automatic type conversions
        
        // Float to integer
        let result = spillstr("Float as int: %d", &[Value::Float(3.7)]).unwrap();
        assert_eq!(result, "Float as int: 3");
        
        // String to integer (valid)
        let result = spillstr("String as int: %d", &[Value::String("123".to_string())]).unwrap();
        assert_eq!(result, "String as int: 123");
        
        // String to integer (invalid)
        let result = spillstr("Invalid as int: %d", &[Value::String("abc".to_string())]);
        assert!(result.is_err());
        
        // Integer to float
        let result = spillstr("Int as float: %f", &[Value::Int(42)]).unwrap();
        assert!(result.contains("42."));
        
        // String to float (valid)
        let result = spillstr("String as float: %f", &[Value::String("3.14".to_string())]).unwrap();
        assert!(result.contains("3.14"));
        
        // String to float (invalid)
        let result = spillstr("Invalid as float: %f", &[Value::String("not_a_number".to_string())]);
        assert!(result.is_err());
    }

    #[test]
    fn test_spillstr_complex_values() {
        // Test with complex CURSED Value types
        
        // Array formatting
        let array = Value::Array(vec![
            Value::Int(1),
            Value::Int(2),
            Value::Int(3)
        ]);
        let result = spillstr("Array: %s", &[array]).unwrap();
        assert!(result.contains("[1, 2, 3]"));
        
        // Boolean formatting
        let result = spillstr("Bool true: %s", &[Value::Bool(true)]).unwrap();
        assert_eq!(result, "Bool true: true");
        
        let result = spillstr("Bool false: %s", &[Value::Bool(false)]).unwrap();
        assert_eq!(result, "Bool false: false");
        
        // Nil formatting
        let result = spillstr("Nil: %s", &[Value::Nil]).unwrap();
        assert_eq!(result, "Nil: nil");
    }

    #[test]
    fn test_spillstr_precision_formatting() {
        // Test precision with floats
        let result = spillstr("%.0f", &[Value::Float(3.14159)]).unwrap();
        assert_eq!(result, "3");
        
        let result = spillstr("%.1f", &[Value::Float(3.14159)]).unwrap();
        assert_eq!(result, "3.1");
        
        let result = spillstr("%.3f", &[Value::Float(3.14159)]).unwrap();
        assert_eq!(result, "3.142");
        
        let result = spillstr("%.10f", &[Value::Float(3.14159)]).unwrap();
        assert!(result.len() > 10);
    }

    #[test]
    fn test_spillf_output() {
        // Since spillf writes to stdout, we can't easily capture its output in tests
        // We'll test that it doesn't panic and returns Ok
        
        let result = spillf("Hello %s!", &[Value::String("World".to_string())]);
        assert!(result.is_ok());
        
        let result = spillf("Number: %d", &[Value::Int(42)]);
        assert!(result.is_ok());
        
        let result = spillf("", &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_spill_output() {
        // Test spill function (prints with newline)
        
        let result = spill(&[Value::String("Hello".to_string()), Value::String("World".to_string())]);
        assert!(result.is_ok());
        
        let result = spill(&[Value::Int(42), Value::Float(3.14)]);
        assert!(result.is_ok());
        
        // Test empty spill (just newline)
        let result = spill(&[]);
        assert!(result.is_ok());
        
        // Test with different value types
        let result = spill(&[
            Value::String("Test".to_string()),
            Value::Int(123),
            Value::Bool(true),
            Value::Nil
        ]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_edge_cases() {
        // Test edge cases in format string parsing
        
        // Format specifier at end of string
        let result = spillstr("End with format %s", &[Value::String("test".to_string())]).unwrap();
        assert_eq!(result, "End with format test");
        
        // Format specifier at beginning
        let result = spillstr("%s at beginning", &[Value::String("Format".to_string())]).unwrap();
        assert_eq!(result, "Format at beginning");
        
        // Multiple consecutive format specifiers
        let result = spillstr("%s%s%s", &[
            Value::String("A".to_string()),
            Value::String("B".to_string()),
            Value::String("C".to_string())
        ]).unwrap();
        assert_eq!(result, "ABC");
        
        // Invalid format specifier
        let result = spillstr("Invalid: %z", &[Value::String("test".to_string())]).unwrap();
        assert_eq!(result, "Invalid: test"); // Should default to string format
    }

    #[test]
    fn test_unicode_formatting() {
        // Test Unicode string formatting
        let result = spillstr("Unicode: %s", &[Value::String("🔥💯".to_string())]).unwrap();
        assert_eq!(result, "Unicode: 🔥💯");
        
        let result = spillstr("Mixed: %s and %s", &[
            Value::String("Hello".to_string()),
            Value::String("🌍".to_string())
        ]).unwrap();
        assert_eq!(result, "Mixed: Hello and 🌍");
        
        // Test Unicode in format string
        let result = spillstr("Test 🚀 %s", &[Value::String("rocket".to_string())]).unwrap();
        assert_eq!(result, "Test 🚀 rocket");
    }

    #[test]
    fn test_large_format_strings() {
        // Test with large format strings
        let mut large_format = String::new();
        let mut args = Vec::new();
        
        for i in 0..100 {
            large_format.push_str(&format!("Item {}: %s ", i));
            args.push(Value::String(format!("value{}", i)));
        }
        
        let result = spillstr(&large_format, &args);
        assert!(result.is_ok());
        
        let formatted = result.unwrap();
        assert!(formatted.contains("Item 0: value0"));
        assert!(formatted.contains("Item 99: value99"));
    }

    #[test]
    fn test_format_with_special_characters() {
        // Test formatting with special characters
        let result = spillstr("Special: %s", &[Value::String("line1\nline2\ttab".to_string())]).unwrap();
        assert_eq!(result, "Special: line1\nline2\ttab");
        
        let result = spillstr("Quotes: %s", &[Value::String("\"quoted\"".to_string())]).unwrap();
        assert_eq!(result, "Quotes: \"quoted\"");
        
        let result = spillstr("Backslash: %s", &[Value::String("path\\file".to_string())]).unwrap();
        assert_eq!(result, "Backslash: path\\file");
    }

    #[test]
    fn test_scan_simulation() {
        // Note: scan functions require actual stdin input, so we can't easily test them
        // in unit tests. We'll test the helper functions instead.
        
        // Test value parsing
        let target = Value::String(String::new());
        let result = super::parse_value_from_str("hello", &target);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::String("hello".to_string()));
        
        let target = Value::Int(0);
        let result = super::parse_value_from_str("42", &target);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Int(42));
        
        let target = Value::Float(0.0);
        let result = super::parse_value_from_str("3.14", &target);
        assert!(result.is_ok());
        if let Value::Float(f) = result.unwrap() {
            assert!((f - 3.14).abs() < 1e-10);
        } else {
            panic!("Expected float value");
        }
        
        let target = Value::Bool(false);
        let result = super::parse_value_from_str("true", &target);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Bool(true));
        
        let result = super::parse_value_from_str("false", &target);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Bool(false));
        
        // Test invalid parsing
        let target = Value::Int(0);
        let result = super::parse_value_from_str("not_a_number", &target);
        assert!(result.is_err());
        
        let target = Value::Float(0.0);
        let result = super::parse_value_from_str("not_a_float", &target);
        assert!(result.is_err());
    }

    #[test]
    fn test_format_arg_with_spec() {
        // Test individual format specifier handling
        
        // Test decimal integer
        let result = super::format_arg_with_spec("%d", &Value::Int(42)).unwrap();
        assert_eq!(result, "42");
        
        // Test hexadecimal
        let result = super::format_arg_with_spec("%x", &Value::Int(255)).unwrap();
        assert_eq!(result, "ff");
        
        let result = super::format_arg_with_spec("%X", &Value::Int(255)).unwrap();
        assert_eq!(result, "FF");
        
        // Test octal
        let result = super::format_arg_with_spec("%o", &Value::Int(64)).unwrap();
        assert_eq!(result, "100");
        
        // Test float
        let result = super::format_arg_with_spec("%f", &Value::Float(3.14159)).unwrap();
        assert!(result.contains("3.14159"));
        
        // Test character
        let result = super::format_arg_with_spec("%c", &Value::Int(65)).unwrap();
        assert_eq!(result, "A");
        
        // Test string
        let result = super::format_arg_with_spec("%s", &Value::String("test".to_string())).unwrap();
        assert_eq!(result, "test");
        
        // Test default value format
        let result = super::format_arg_with_spec("%v", &Value::Bool(true)).unwrap();
        assert_eq!(result, "true");
    }

    #[test]
    fn test_boolean_parsing_variations() {
        let target = Value::Bool(false);
        
        // Test various true values
        assert_eq!(super::parse_value_from_str("true", &target).unwrap(), Value::Bool(true));
        assert_eq!(super::parse_value_from_str("t", &target).unwrap(), Value::Bool(true));
        assert_eq!(super::parse_value_from_str("1", &target).unwrap(), Value::Bool(true));
        assert_eq!(super::parse_value_from_str("yes", &target).unwrap(), Value::Bool(true));
        assert_eq!(super::parse_value_from_str("y", &target).unwrap(), Value::Bool(true));
        assert_eq!(super::parse_value_from_str("TRUE", &target).unwrap(), Value::Bool(true));
        assert_eq!(super::parse_value_from_str("YES", &target).unwrap(), Value::Bool(true));
        
        // Test various false values
        assert_eq!(super::parse_value_from_str("false", &target).unwrap(), Value::Bool(false));
        assert_eq!(super::parse_value_from_str("f", &target).unwrap(), Value::Bool(false));
        assert_eq!(super::parse_value_from_str("0", &target).unwrap(), Value::Bool(false));
        assert_eq!(super::parse_value_from_str("no", &target).unwrap(), Value::Bool(false));
        assert_eq!(super::parse_value_from_str("n", &target).unwrap(), Value::Bool(false));
        assert_eq!(super::parse_value_from_str("FALSE", &target).unwrap(), Value::Bool(false));
        assert_eq!(super::parse_value_from_str("NO", &target).unwrap(), Value::Bool(false));
        
        // Test invalid boolean values
        assert!(super::parse_value_from_str("maybe", &target).is_err());
        assert!(super::parse_value_from_str("2", &target).is_err());
        assert!(super::parse_value_from_str("", &target).is_err());
    }
}
