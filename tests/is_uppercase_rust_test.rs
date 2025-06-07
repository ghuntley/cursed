use std::sync::Arc;
use cursed::object::Object;
use cursed::stdlib::is_uppercase;


#[test]
fn test_is_uppercase() {
    // Test uppercase letter
    let result = is_uppercase::is_uppercase(&[Arc::new(Object::String("A".to_string())]);
    assert!(result.is_ok())
    match result.unwrap().as_ref() {
        Object::Boolean(b) => assert_eq!(*b, true),
        _ => panic!("Expected boolean result")
    }
    
    // Test lowercase letter
    let result = is_uppercase::is_uppercase(&[Arc::new(Object::String("a".to_string())]);
    assert!(result.is_ok())
    match result.unwrap().as_ref() {
        Object::Boolean(b) => assert_eq!(*b, false),
        _ => panic!("Expected boolean result")
    }
}

#[test]
fn test_with_char_type() {
    // Test with Char type
    let result = is_uppercase::is_uppercase(&[Arc::new(Object::Char('A'))]);
    assert!(result.is_ok())
    match result.unwrap().as_ref() {
        Object::Boolean(b) => assert_eq!(*b, true),
        _ => panic!("Expected boolean result")
    }
}