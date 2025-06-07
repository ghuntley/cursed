use cursed::object::Object;
use cursed::stdlib::oglogging;

//! Integration tests for advanced oglogging features


#[test]
fn test_advanced_formatting() {
    // Testing the enhanced formatting capabilities
    let args = [Object::Integer(42)];
    
    // Format with width specifier
    let width_result = oglogging::format_with_args("%5d", &args);
    assert_eq!(width_result, "   42");
    
    // Format with precision
    let float_args = [Object::Float(3.14159)];
    let precision_result = oglogging::format_with_args("%.2f", &float_args);
    assert_eq!(precision_result, "3.14");
    
    // Format with positional arguments
    let multi_args = [Object::String("first".to_string()), Object::String("second".to_string())];
    let positional_result = oglogging::format_with_args("%[1]v %[2]v %[1]v", &multi_args);
    assert_eq!(positional_result, "first second first");
    
    // Format with hex/binary/octal
    let hex_args = [Object::Integer(255)];
    let hex_result = oglogging::format_with_args("%x", &hex_args);
    assert_eq!(hex_result, "ff");
    
    let bin_result = oglogging::format_with_args("%b", &hex_args);
    assert_eq!(bin_result, "11111111");
}

#[test]
fn test_log_levels() {
    // Set log level to DEBUG so all messages would be logged
    oglogging::set_level(oglogging::LDEBUG);
    assert_eq!(oglogging::level(), oglogging::LDEBUG);
    
    // Change to ERROR level and verify only errors would be logged
    oglogging::set_level(oglogging::LERROR);
    assert_eq!(oglogging::level(), oglogging::LERROR);
    
    // Test debug message (shouldn't be logged at ERROR level)
    let debug_args = [Object::String("debug message".to_string())];
    let _ = oglogging::debug(&debug_args);
    
    // Test error message (should be logged at ERROR level)
    let error_args = [Object::String("error message".to_string())];
    let _ = oglogging::error(&error_args);
    
    // Reset to default (INFO)
    oglogging::set_level(oglogging::LINFO);
}