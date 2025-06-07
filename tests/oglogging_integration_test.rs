use cursed::object::Object;
use cursed::stdlib::oglogging;

// Integration test for oglogging package


#[test]
fn test_basic_logging() {
    // This is mostly a compile-time test since we can't easily capture stdout
    // Just ensure the functions exist and don't panic
    let args = [Object::String("Test message".to_string())];
    oglogging::set_prefix("TEST: ".to_string());
    let _ = oglogging::spill(&args);
    
    // Verify flag constants exist
    assert_eq!(oglogging::LDATE | oglogging::LTIME, oglogging::LSTDFLAGS);
    
    // Test setting and getting flags
    oglogging::set_flags(oglogging::LDATE | oglogging::LSHORTFILE);
    assert_eq!(oglogging::flags(), oglogging::LDATE | oglogging::LSHORTFILE);
    
    // Test prefix getter
    assert_eq!(oglogging::prefix(), "TEST: ".to_string());
    
    // Test formatted logging
    let format_args = [Object::Integer(42)];
    let _ = oglogging::spillf("Value: %v", &format_args);
}

#[test]
fn test_logging_constants() {
    // Ensure flag constants are correct
    assert_eq!(oglogging::LDATE, 1 << 0);
    assert_eq!(oglogging::LTIME, 1 << 1);
    assert_eq!(oglogging::LMICROSECONDS, 1 << 2);
    assert_eq!(oglogging::LLONGFILE, 1 << 3);
    assert_eq!(oglogging::LSHORTFILE, 1 << 4);
    assert_eq!(oglogging::LUTC, 1 << 5);
    assert_eq!(oglogging::LMSGPREFIX, 1 << 6);
    assert_eq!(oglogging::LSTDFLAGS, oglogging::LDATE | oglogging::LTIME);
}