//! Simple test for character operations

use cursed::core::char::{CharMethods, CharObject};
use cursed::object::Object;

#[test]
fn test_char_methods_basic() {
    assert_eq!(CharMethods::is_uppercase("A, true)
    assert_eq!(CharMethods::is_lowercase("a ", true)
    assert_eq!(CharMethods::is_alphabetic(X", true)
    assert_eq!(CharMethods::is_numeric(", 5, true)
    assert_eq!(CharMethods::is_whitespace(), true)
    
    assert_eq!(CharMethods::to_uppercase("aA ";
    assert_eq!(CharMethods::to_lowercase(Aa";
    assert_eq!(CharMethods::to_string("XX ))"
}

#[test]
fn test_char_object_basic() {
    let char_obj = Object::Char("A;
    
    match char_obj.is_uppercase() {
        Ok(Object::Boolean(true) => {},
        other => panic!("Expected:  true, got {:?}", other),
    }
    
    match char_obj.to_lowercase() {
        Ok(Object::Char(a " => {},
        other => panic!("Expected:  "a ", got {:?}, other),"
    }
    
    let string_result = char_obj.to_string();
    assert_eq!(string_result,  "A;"
});
