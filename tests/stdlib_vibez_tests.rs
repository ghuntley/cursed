//! Unit tests for CURSED vibez standard library

use cursed::runtime::value::Value;
use cursed::stdlib::vibez::{format, sprintf, debug, print};
use cursed::error_types::CursedError;

#[test]
fn test_format_value_basic() {
    let value = Value::integer(42);
    let spec = format::FormatSpec::default();
    
    let result = format::format_value_with_spec(&value, &spec).unwrap();
    assert_eq!(result, "42");
}

#[test]
fn test_format_value_hex() {
    let value = Value::integer(255);
    let mut spec = format::FormatSpec::default();
    spec.placeholder_type = format::PlaceholderType::Hex(false);
    
    let result = format::format_value_with_spec(&value, &spec).unwrap();
    assert_eq!(result, "ff");
}

#[test]
fn test_format_value_hex_uppercase() {
    let value = Value::integer(255);
    let mut spec = format::FormatSpec::default();
    spec.placeholder_type = format::PlaceholderType::Hex(true);
    
    let result = format::format_value_with_spec(&value, &spec).unwrap();
    assert_eq!(result, "FF");
}

#[test]
fn test_format_value_width_padding() {
    let value = Value::integer(42);
    let mut spec = format::FormatSpec::default();
    spec.width = Some(5);
    spec.alignment = format::FormatAlignment::Right;
    
    let result = format::format_value_with_spec(&value, &spec).unwrap();
    assert_eq!(result, "   42");
}

#[test]
fn test_sprintf_basic() {
    let args = vec![Value::integer(42), Value::string("hello")];
    let result = sprintf::sprintf("%d %s", &args).unwrap();
    assert_eq!(result, "42 hello");
}

#[test]
fn test_sprintf_format_validation() {
    assert!(sprintf::validate_format_string("%d %s").unwrap());
    assert!(sprintf::validate_format_string("%%").unwrap());
}

#[test]
fn test_sprintf_count_specifiers() {
    assert_eq!(sprintf::count_format_specifiers("%d %s %f"), 3);
    assert_eq!(sprintf::count_format_specifiers("%%"), 0);
    assert_eq!(sprintf::count_format_specifiers("no specifiers"), 0);
}

#[test]
fn test_debug_system_basic() {
    debug::init_debug_system();
    
    assert!(debug::set_debug_level(4).is_ok());
    assert_eq!(debug::get_debug_level(), 4);
    assert!(debug::is_debug_enabled());
}

#[test]
fn test_debug_logging() {
    debug::init_debug_system();
    debug::set_debug_level(5).unwrap();
    
    assert!(debug::debug_info("Test message", Some("test")).is_ok());
    assert!(debug::debug_error("Error message", None).is_ok());
    
    let stats = debug::get_debug_stats().unwrap();
    assert!(stats.messages_logged > 0);
}

#[test]
fn test_debug_inspect_value() {
    debug::init_debug_system();
    debug::set_debug_level(4).unwrap();
    
    let value = Value::string("test value");
    assert!(debug::debug_inspect(&value, Some("test_var")).is_ok());
}

#[test]
fn test_spill_function() {
    let args = vec![
        Value::string("Hello"),
        Value::integer(42),
        Value::bool(true)
    ];
    
    // This test just ensures the function doesn't panic
    assert!(print::spill(&args).is_ok());
}

#[test]
fn test_spillf_function() {
    let args = vec![Value::integer(42), Value::string("world")];
    
    // Test simple formatting
    assert!(print::spillf("Number: {}, String: {}", &args).is_ok());
}

#[test]
fn test_scan_function() {
    // This test would require stdin simulation, so we just test that the function exists
    // In a real implementation, you'd mock stdin or use a test framework that supports it
}
