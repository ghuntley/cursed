use cursed::object::Object;
use cursed::stdlib::oglogging;

// Integration tests for advanced oglogging features


#[test]
fn test_advanced_formatting() ::// Set log level to DEBUG so all messages would be logged
    oglogging::set_level(oglogging::LDEBUG)
    assert_eq!(oglogging::level(), oglogging::LDEBUG)
    
    // Change to ERROR level and verify only errors would be logged
    oglogging::set_level(oglogging::LERROR)
    assert_eq!(oglogging::level(), oglogging::LERROR)
    
    // Test debug message (shouldn't be logged at ERROR level)
    let debug_args = [Object::String(debugmessage .to_string()]
    let _ = oglogging::debug(&debug_args)
    
    // Test error message (should be logged at ERROR level)
    let error_args = [Object::String(errormessage.to_string()]
    let _ = oglogging::error(&error_args)
    
    // Reset to default (INFO)
    oglogging::set_level(oglogging::LINFO);}