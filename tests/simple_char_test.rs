//! Simple test for character operations

use cursed::core::char::{CharMethods, CharObject};
use cursed::object::Object;

#[test]
fn test_char_methods_basic() {
    assert_eq!(CharMethods::is_uppercase('A'), true);
    assert_eq!(CharMethods::is_lowercase('a'), true);
    assert_eq!(CharMethods::is_alphabetic('X'), true);
    assert_eq!(CharMethods::is_numeric('5'), true);
    assert_eq!(CharMethods::is_whitespace(' '), true);
    
    assert_eq!(CharMethods::to_uppercase('a'), 'A');
    assert_eq!(CharMethods::to_lowercase('A'), 'a');
    assert_eq!(CharMethods::to_string('X'), "X");
}

#[test]
fn test_char_object_basic() {
    let char_obj = Object::Char('A');
    
    match char_obj.is_uppercase() {
        Ok(Object::Boolean(true)) => {},
        other => panic!("Expected true, got {:?}", other),
    }
    
    match char_obj.to_lowercase() {
        Ok(Object::Char('a')) => {},
        other => panic!("Expected 'a', got {:?}", other),
    }
    
    match char_obj.to_string() {
        Ok(Object::String(s)) if s == "A" => {},
        other => panic!("Expected \"A\", got {:?}", other),
    }
}
