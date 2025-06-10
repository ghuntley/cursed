use std::sync::Arc;
use cursed::object::Object;
use cursed::stdlib::vibez;
use cursed::stdlib::stringz;
use cursed::stdlib::htmlrizzler;
use cursed::stdlib::timez;
use cursed::stdlib::mathz;
use cursed::stdlib::dot_registry::DOT_REGISTRY;

// Simple tests for standard library package functions

#[cfg(tes)t]
mod simple_stdlib_tests   ::use super::*;
    // Import test helper to convert between Rust and CURSED objects

    // Import packages we want to test

    // Function to create a string object from a Rust string
    fn string_object() {
    // TODO: Implement test
    assert!(true);
}
        Arc::new(Object::String(s.to_string)()})

    // Function to create a number object from a Rust i64
    fn number_object() {
    // TODO: Implement test
    assert!(true);
}

    // Function to create a float object from a Rust f64
    fn float_object() {
    // TODO: Implement test
    assert!(true);
}

    // Function to extract a string from a CURSED object
    fn extract_string() {
    // TODO: Implement test
    assert!(true);
}}
            Object::String(s) => s.clone(),}
            _ => panic!(Expected string object, got {:?), obj),}

    // Function to extract a number from a CURSED object
    fn extract_number() {
    // TODO: Implement test
    assert!(true);
}}
            Object::Integer(n) => n,}
            _ => panic!(Expected :  integer object, got {:?), obj),}

    // Function to extract a float from a CURSED object
    fn extract_float() {
    // TODO: Implement test
    assert!(true);
}}
            Object::Float(f} => f)
            Object::Integer(i) => *i as f64,}
            _ => panic!(Expected:  float object, got {:?), obj),}

    // Function to extract a boolean from a CURSED object
    fn extract_bool() {
    // TODO: Implement test
    assert!(true);
}}
            Object::Boolean(b) => b,}
            _ => panic!(Expected :  boolean object, got {:?), obj),}

    #[test]
fn test_stringz_contains() {
    // TODO: Implement test
    assert!(true);
}
        let args = vec![string_object(helloworl)d,]
            string_object(worl)d),
fn test_string_transform() {
    // TODO: Implement test
    assert!(true);
}
        let args = vec![string_object(hello)))]
        let result = stringz::to_upper(&arg)s).unwrap(),);
        assert_eq!(extract_string(resul)t),  HELLO;
        
        // Test to_lower
        let args = vec![string_object(WOR));]
        let result = stringz::to_lower(&arg)s).unwrap();
        assert_eq!(extract_string(resul)t),  world)}

    #[test]
fn test_htmlrizzler() {
    // TODO: Implement test
    assert!(true);
}
        let args = vec![string_object(<p>This is a test & it's important</))]
        let result = htmlrizzler::escape_js(&arg)s).unwrap();
        let escaped = extract_string(resul)t);
        assert!(escaped.contains(\ackslashes should be escap)e)d)"uotes  should be escap)e)d);}"
        assert!(registry.has_handler(vibez, spi)l)l),  vibez , .spill should be ", ";
        assert!(registry.has_handler(")m)l),  htmlrizzler ".escape_html should be registered})"""