/// Comprehensive test suite for the CURSED Vibez formatting/printing module
/// 
/// Tests all aspects of the vibez module including basic printing, advanced formatting,
/// printf-style formatting, debug utilities, and integration with the CURSED type system.

use cursed::value::Value;
use cursed::stdlib::vibez::*;
use std::collections::HashMap;
use std::io::Write;

#[test]
fn test_basic_printing_functionality() {
    // Test print_to and println_to with buffer
    let mut buffer = Vec::new();
    
    let args = vec![
        Value::String("Hello".to_string()),
        Value::String("World".to_string()),
        Value::Int(42)
    ];
    
    print_to(&mut buffer, &args).unwrap();
    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "Hello World 42");
    
    // Test println_to
    let mut buffer = Vec::new();
    println_to(&mut buffer, &args).unwrap();
    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "Hello World 42\n");
}

#[test]
fn test_value_formatting() {
    // Test basic types
    let values = vec![
        (Value::Nil, "nil"),
        (Value::Bool(true), "true"),
        (Value::Bool(false), "false"),
        (Value::Int(42), "42"),
        (Value::Float(3.14), "3.14"),
        (Value::String("hello".to_string()), "hello"),
    ];
    
    for (value, expected) in values {
        let mut buffer = Vec::new();
        print_to(&mut buffer, &[value]).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, expected);
    }
}

#[test]
fn test_collection_formatting() {
    // Test array formatting
    let array = vec![Value::Int(1), Value::Int(2), Value::Int(3)];
    let mut buffer = Vec::new();
    print_to(&mut buffer, &[Value::Array(array)]).unwrap();
    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "[1, 2, 3]");
    
    // Test object formatting
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Value::String("Alice".to_string()));
    obj.insert("age".to_string(), Value::Int(25));
    
    let mut buffer = Vec::new();
    print_to(&mut buffer, &[Value::Object(obj)]).unwrap();
    let output = String::from_utf8(buffer).unwrap();
    
    // Object formatting might vary in order, so check for key components
    assert!(output.contains("name: Alice"));
    assert!(output.contains("age: 25"));
    assert!(output.starts_with('{') && output.ends_with('}'));
}

#[test]
fn test_format_basic_functionality() {
    // Test basic positional formatting
    let args = vec![Value::String("World".to_string())];
    let result = format("Hello {}", &args).unwrap();
    assert_eq!(result, "Hello World");
    
    // Test multiple arguments
    let args = vec![
        Value::String("John".to_string()),
        Value::Int(25),
        Value::Float(180.5)
    ];
    let result = format("Name: {}, Age: {}, Height: {}", &args).unwrap();
    assert_eq!(result, "Name: John, Age: 25, Height: 180.5");
}

#[test]
fn test_format_positional_arguments() {
    let args = vec![
        Value::String("World".to_string()),
        Value::String("Hello".to_string()),
        Value::Int(2024)
    ];
    
    // Test positional indexing
    let result = format("{1} {0} in {2}", &args).unwrap();
    assert_eq!(result, "Hello World in 2024");
    
    // Test mixed auto and positional
    let result = format("{1} {} {0}", &args).unwrap();
    assert_eq!(result, "Hello World World");
}

#[test]
fn test_format_error_handling() {
    // Test missing argument
    let args = vec![Value::String("Hello".to_string())];
    let result = format("Hello {} {}", &args);
    assert!(matches!(result, Err(FormatError::MissingArgument(1))));
    
    // Test invalid placeholder
    let args = vec![Value::String("test".to_string())];
    let result = format("Hello {name}", &args);
    assert!(matches!(result, Err(FormatError::InvalidPlaceholder(_))));
}

#[test]
fn test_format_with_context() {
    let mut context = FormatContext::default();
    context.variables.insert("name".to_string(), Value::String("Alice".to_string()));
    context.variables.insert("age".to_string(), Value::Int(30));
    context.variables.insert("city".to_string(), Value::String("New York".to_string()));
    
    // Test named placeholders
    let result = format_with_context("Hello {name}!", &[], &context).unwrap();
    assert_eq!(result, "Hello Alice!");
    
    // Test multiple named placeholders
    let result = format_with_context(
        "{name} is {age} years old and lives in {city}",
        &[],
        &context
    ).unwrap();
    assert_eq!(result, "Alice is 30 years old and lives in New York");
    
    // Test mixed positional and named
    let args = vec![Value::String("greeting".to_string())];
    let result = format_with_context("{}: Hello {name}!", &args, &context).unwrap();
    assert_eq!(result, "greeting: Hello Alice!");
}

#[test]
fn test_interpolation() {
    let mut context = FormatContext::default();
    context.variables.insert("name".to_string(), Value::String("Bob".to_string()));
    context.variables.insert("score".to_string(), Value::Int(95));
    
    let result = interpolate(
        "Player ${name} scored ${score} points!",
        &context
    ).unwrap();
    assert_eq!(result, "Player Bob scored 95 points!");
    
    // Test with missing variable
    let result = interpolate("Hello ${unknown}!", &context);
    assert!(matches!(result, Err(FormatError::InvalidContext(_))));
}

#[test]
fn test_format_args() {
    let args = vec![
        Value::String("Hello".to_string()),
        Value::Int(42),
        Value::Bool(true),
        Value::Float(3.14)
    ];
    
    let result = format_args(&args);
    assert_eq!(result, "Hello 42 true 3.14");
    
    // Test empty args
    let result = format_args(&[]);
    assert_eq!(result, "");
    
    // Test single arg
    let result = format_args(&[Value::String("single".to_string())]);
    assert_eq!(result, "single");
}

#[test]
fn test_sprintf_basic_functionality() {
    // Test string formatting
    let args = vec![Value::String("World".to_string())];
    let result = sprintf("Hello %s", &args).unwrap();
    assert_eq!(result, "Hello World");
    
    // Test integer formatting
    let args = vec![Value::Int(42)];
    let result = sprintf("The answer is %d", &args).unwrap();
    assert_eq!(result, "The answer is 42");
    
    // Test float formatting
    let args = vec![Value::Float(3.14159)];
    let result = sprintf("Pi is %.2f", &args).unwrap();
    assert_eq!(result, "Pi is 3.14");
}

#[test]
fn test_sprintf_multiple_specifiers() {
    let args = vec![
        Value::String("John".to_string()),
        Value::Int(25),
        Value::Float(180.5)
    ];
    
    let result = sprintf("%s is %d years old and %.1f cm tall", &args).unwrap();
    assert_eq!(result, "John is 25 years old and 180.5 cm tall");
}

#[test]
fn test_sprintf_hex_formatting() {
    let args = vec![Value::Int(255)];
    
    // Test lowercase hex
    let result = sprintf("0x%x", &args).unwrap();
    assert_eq!(result, "0xff");
    
    // Test uppercase hex
    let result = sprintf("0x%X", &args).unwrap();
    assert_eq!(result, "0xFF");
    
    // Test octal
    let result = sprintf("0%o", &args).unwrap();
    assert_eq!(result, "0377");
}

#[test]
fn test_sprintf_width_formatting() {
    let args = vec![Value::Int(42)];
    
    // Test right-aligned width
    let result = sprintf("%5d", &args).unwrap();
    assert_eq!(result, "   42");
    
    // Test left-aligned width
    let result = sprintf("%-5d", &args).unwrap();
    assert_eq!(result, "42   ");
    
    // Test zero padding
    let result = sprintf("%05d", &args).unwrap();
    assert_eq!(result, "00042");
}

#[test]
fn test_sprintf_precision() {
    // Test float precision
    let args = vec![Value::Float(3.14159)];
    let result = sprintf("%.3f", &args).unwrap();
    assert_eq!(result, "3.142");
    
    // Test zero precision
    let result = sprintf("%.0f", &args).unwrap();
    assert_eq!(result, "3");
}

#[test]
fn test_sprintf_character_formatting() {
    // Test character from integer
    let args = vec![Value::Int(65)]; // ASCII 'A'
    let result = sprintf("%c", &args).unwrap();
    assert_eq!(result, "A");
    
    // Test character from string
    let args = vec![Value::String("B".to_string())];
    let result = sprintf("%c", &args).unwrap();
    assert_eq!(result, "B");
}

#[test]
fn test_sprintf_error_handling() {
    // Test missing argument
    let args = vec![Value::String("Hello".to_string())];
    let result = sprintf("Hello %s %d", &args);
    assert!(matches!(result, Err(SprintfError::MissingArgument(1))));
    
    // Test too many arguments
    let args = vec![
        Value::String("Hello".to_string()),
        Value::Int(42)
    ];
    let result = sprintf("Hello %s", &args);
    assert!(matches!(result, Err(SprintfError::TooManyArguments)));
    
    // Test type mismatch
    let args = vec![Value::String("not_a_number".to_string())];
    let result = sprintf("%d", &args);
    assert!(matches!(result, Err(SprintfError::TypeMismatch(_))));
}

#[test]
fn test_snprintf() {
    let args = vec![Value::String("Hello World".to_string())];
    
    // Test within limit
    let result = snprintf(20, "Message: %s", &args).unwrap();
    assert_eq!(result, "Message: Hello World");
    
    // Test buffer overflow
    let result = snprintf(5, "Message: %s", &args);
    assert!(matches!(result, Err(SprintfError::BufferOverflow)));
}

#[test]
fn test_sprintf_format_validation() {
    // Test valid format string
    assert!(validate_format_string("Hello %s, age %d").is_ok());
    
    // Test count specifiers
    assert_eq!(count_format_specifiers("Hello %s %d %f").unwrap(), 3);
    assert_eq!(count_format_specifiers("No specifiers").unwrap(), 0);
    assert_eq!(count_format_specifiers("%%escaped%%").unwrap(), 0);
}

#[test]
fn test_sprintf_to_writer() {
    let mut buffer = Vec::new();
    let args = vec![
        Value::String("test".to_string()),
        Value::Int(123)
    ];
    
    sprintf_to_writer(&mut buffer, "String: %s, Number: %d", &args).unwrap();
    
    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "String: test, Number: 123");
}

#[test]
fn test_debug_level_management() {
    let original_level = get_debug_level();
    
    // Test setting and getting debug level
    set_debug_level(DebugLevel::Trace);
    assert_eq!(get_debug_level(), DebugLevel::Trace);
    assert!(is_debug_enabled(DebugLevel::Debug));
    assert!(is_debug_enabled(DebugLevel::Info));
    
    set_debug_level(DebugLevel::Error);
    assert_eq!(get_debug_level(), DebugLevel::Error);
    assert!(!is_debug_enabled(DebugLevel::Warning));
    assert!(is_debug_enabled(DebugLevel::Error));
    
    // Restore original level
    set_debug_level(original_level);
}

#[test]
fn test_debug_value_inspection() {
    // Test basic value inspection
    let value = Value::String("test string".to_string());
    let inspection = debug_inspect(&value);
    
    assert!(inspection.contains("Type: string"));
    assert!(inspection.contains("Value: \"test string\""));
    assert!(inspection.contains("Length: 11"));
    
    // Test array inspection
    let array = vec![Value::Int(1), Value::Int(2), Value::Int(3)];
    let value = Value::Array(array);
    let inspection = debug_inspect(&value);
    
    assert!(inspection.contains("Type: array"));
    assert!(inspection.contains("Length: 3"));
    assert!(inspection.contains("[0]: 1"));
    assert!(inspection.contains("[1]: 2"));
    assert!(inspection.contains("[2]: 3"));
    
    // Test object inspection
    let mut obj = HashMap::new();
    obj.insert("key1".to_string(), Value::String("value1".to_string()));
    obj.insert("key2".to_string(), Value::Int(42));
    let value = Value::Object(obj);
    let inspection = debug_inspect(&value);
    
    assert!(inspection.contains("Type: object"));
    assert!(inspection.contains("Properties: 2"));
}

#[test]
fn test_debug_bytes_inspection() {
    let bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello" in ASCII
    let value = Value::Bytes(bytes);
    let inspection = debug_inspect(&value);
    
    assert!(inspection.contains("Type: bytes"));
    assert!(inspection.contains("Size: 5 bytes"));
    assert!(inspection.contains("Hex preview: 48 65 6c 6c 6f"));
}

#[test]
fn test_pretty_printing() {
    // Test simple pretty printing
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Value::String("Alice".to_string()));
    obj.insert("age".to_string(), Value::Int(30));
    
    let nested_array = vec![Value::Int(1), Value::Int(2), Value::Int(3)];
    obj.insert("numbers".to_string(), Value::Array(nested_array));
    
    let value = Value::Object(obj);
    
    // This should not panic and should produce formatted output
    assert!(pretty_print(&value).is_ok());
}

#[test]
fn test_format_specifier_parsing() {
    // Test format specifier structure
    let args = vec![Value::Int(42)];
    
    // These should all parse and format correctly
    assert!(sprintf("%d", &args).is_ok());
    assert!(sprintf("%5d", &args).is_ok());
    assert!(sprintf("%-5d", &args).is_ok());
    assert!(sprintf("%05d", &args).is_ok());
    assert!(sprintf("%+d", &args).is_ok());
}

#[test]
fn test_format_placeholder_parsing() {
    // Test different placeholder types
    assert!(format("Simple {}", &[Value::Int(1)]).is_ok());
    assert!(format("Positional {0}", &[Value::Int(1)]).is_ok());
    assert!(format("Multiple {} {}", &[Value::Int(1), Value::Int(2)]).is_ok());
    assert!(format("Mixed {1} {} {0}", &[Value::Int(1), Value::Int(2)]).is_ok());
}

#[test]
fn test_error_types_and_messages() {
    // Test FormatError types
    let error = FormatError::MissingArgument(5);
    assert_eq!(error.to_string(), "Missing argument at index: 5");
    
    let error = FormatError::InvalidPlaceholder("bad".to_string());
    assert_eq!(error.to_string(), "Invalid placeholder: bad");
    
    // Test SprintfError types
    let error = SprintfError::TypeMismatch("test".to_string());
    assert_eq!(error.to_string(), "Type mismatch: test");
    
    let error = SprintfError::BufferOverflow;
    assert_eq!(error.to_string(), "Buffer overflow");
}

#[test]
fn test_module_initialization() {
    // Test module initialization
    initialize();
    
    // Test module information
    assert_eq!(version(), "1.0.0");
    
    let caps = capabilities();
    assert!(caps.contains(&"basic_printing"));
    assert!(caps.contains(&"advanced_formatting"));
    assert!(caps.contains(&"printf_compatibility"));
    assert!(caps.contains(&"debug_utilities"));
}

#[test]
fn test_edge_cases() {
    // Test empty format string
    let result = format("", &[]).unwrap();
    assert_eq!(result, "");
    
    // Test empty sprintf
    let result = sprintf("", &[]).unwrap();
    assert_eq!(result, "");
    
    // Test escaped braces
    let result = format("{{hello}}", &[]).unwrap();
    assert_eq!(result, "{hello}");
    
    // Test escaped percent signs
    let result = sprintf("%%hello%%", &[]).unwrap();
    assert_eq!(result, "%hello%");
}

#[test]
fn test_complex_nested_structures() {
    // Create a complex nested structure
    let mut inner_obj = HashMap::new();
    inner_obj.insert("inner_key".to_string(), Value::String("inner_value".to_string()));
    
    let mut outer_obj = HashMap::new();
    outer_obj.insert("nested".to_string(), Value::Object(inner_obj));
    outer_obj.insert("array".to_string(), Value::Array(vec![
        Value::Int(1),
        Value::Array(vec![Value::String("nested_array".to_string())]),
        Value::Bool(true)
    ]));
    
    let complex_value = Value::Object(outer_obj);
    
    // Test that complex structures can be formatted without errors
    let mut buffer = Vec::new();
    assert!(print_to(&mut buffer, &[complex_value.clone()]).is_ok());
    
    // Test debug inspection of complex structure
    let inspection = debug_inspect(&complex_value);
    assert!(inspection.contains("Type: object"));
    assert!(inspection.contains("Properties:"));
    
    // Test pretty printing
    assert!(pretty_print(&complex_value).is_ok());
}
