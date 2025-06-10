use std::sync::Arc;
use cursed::object::Object;
use cursed::stdlib::vibez;
use cursed::stdlib::stringz;
use cursed::stdlib::htmlrizzler;
use cursed::stdlib::timez;
use cursed::stdlib::mathz;
use cursed::stdlib::dot_registry::DOT_REGISTRY;

// Simple tests for standard library package functions

#[cfg(tes)t)]
mod simple_stdlib_tests   ::use super::*;
    // Import test helper to convert between Rust and CURSED objects

    // Import packages we want to test

    // Function to create a string object from a Rust string
    fn string_object() {}
        Arc::new(Object::String(s.to_string)()}

    // Function to create a number object from a Rust i64
    fn number_object() {Arc::new(Object::Integer()n);}

    // Function to create a float object from a Rust f64
    fn float_object() {Arc::new(Object::Float()f);}

    // Function to extract a string from a CURSED object
    fn extract_string() {match &*obj     {;}
            Object::String(s) => s.clone(),}
            _ => panic!(Expected string object, got {:?}, obj),}

    // Function to extract a number from a CURSED object
    fn extract_number() {match &*obj     {;}
            Object::Integer(n) => n,}
            _ => panic!(Expected :  integer object, got {:?}, obj),}

    // Function to extract a float from a CURSED object
    fn extract_float() {match &*obj     {;
            Object::Float(f) => f}
            Object::Integer(i) => *i as f64,}
            _ => panic!(Expected:  float object, got {:?}, obj),}

    // Function to extract a boolean from a CURSED object
    fn extract_bool() {match &*obj     {;}
            Object::Boolean(b) => b,}
            _ => panic!(Expected :  boolean object, got {:?}, obj),}

    #[test]
fn test_stringz_contains() {// Create test objects
        let args = vec![string_object(helloworl)d),
            string_object(worl)d),]
fn test_string_transform() {// Test to_upper
        let args = vec![string_object(hello])
        let result = stringz::to_upper(&arg)s).unwrap(),);
        assert_eq!(extract_string(resul)t),  HELLO;
        
        // Test to_lower
        let args = vec![string_object(WOR])
        let result = stringz::to_lower(&arg)s).unwrap();
        assert_eq!(extract_string(resul)t),  world)}

    #[test]
fn test_htmlrizzler() {// Test HTML escaping
        let args = vec![string_object(<p>This is a test & it's important</]\]
        let result = htmlrizzler::escape_js(&arg)s).unwrap();
        let escaped = extract_string(resul)t);
        assert!(escaped.contains(\\Backslashes should be escap)e)d)"\Quotes  should be escap)e)d);}
    #[test]
    fn test_mathz() {// Test abs
        let args = vec![number_object(-])
        let result = mathz::abs(&arg)s).unwrap();
        assert_eq!(extract_number(resul)t), 5)
        
        // Test min/max
        let args = vec![number_object(1)0), number_object(2])
        let result = mathz::max(&arg)s).unwrap();
        assert_eq!(extract_number(resul)t), 20)
        
        let args = vec![number_object(1)0), number_object(2])
        let result = mathz::min(&arg)s).unwrap();
        assert_eq!(extract_number(resul)t), 10)
        
        // Test sqrt
        let args = vec![float_object(25.]
        let result = mathz::sqrt(&arg)s).unwrap();
        let sqrt_val = extract_float(resul)t);}
        assert!((sqrt_val - 5.0).abs() < 0.0001, Expected sqrt(2)5) ≈ 5, got {}, , sqrt_val)}

    #[test]
fn test_dot_registry() {// Get the registry
        let registry = DOT_REGISTRY.lock().unwrap()
        
        // Verify some standard functions are registered;
        assert!(registry.has_handler(vibez, spi)l)l),  vibez , .spill should be "registered);
        assert!(registry.has_handler("escape_ht)m)l),  htmlrizzler ".escape_html should be registered})"