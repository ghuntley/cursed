//! Simple test executable for character functions
use cursed::object::Object;
use cursed::stdlib::is_uppercase;
use std::sync::Arc;

fn main() {
    // Test is_uppercase
    let result = is_uppercase::is_uppercase(&[Arc::new(Object::String("A".to_string()))]);
    match result {
        Ok(obj) => {
            match *obj {
                Object::Boolean(true) => println!("✓ is_uppercase(A) = true"),
                _ => println!("❌ is_uppercase(A) should be true, got {:?}", obj),
            }
        },
        Err(e) => println!("❌ is_uppercase(A) error: {:?}", e),
    }
    
    // Test is_lowercase
    let result = is_uppercase::is_lowercase(&[Arc::new(Object::String("a".to_string()))]);
    match result {
        Ok(obj) => {
            match *obj {
                Object::Boolean(true) => println!("✓ is_lowercase(a) = true"),
                _ => println!("❌ is_lowercase(a) should be true, got {:?}", obj),
            }
        },
        Err(e) => println!("❌ is_lowercase(a) error: {:?}", e),
    }
    
    // Test is_digit
    let result = is_uppercase::is_digit(&[Arc::new(Object::String("9".to_string()))]);
    match result {
        Ok(obj) => {
            match *obj {
                Object::Boolean(true) => println!("✓ is_digit(9) = true"),
                _ => println!("❌ is_digit(9) should be true, got {:?}", obj),
            }
        },
        Err(e) => println!("❌ is_digit(9) error: {:?}", e),
    }

    // Test is_alpha
    let result = is_uppercase::is_alpha(&[Arc::new(Object::String("A".to_string()))]);
    match result {
        Ok(obj) => {
            match *obj {
                Object::Boolean(true) => println!("✓ is_alpha(A) = true"),
                _ => println!("❌ is_alpha(A) should be true, got {:?}", obj),
            }
        },
        Err(e) => println!("❌ is_alpha(A) error: {:?}", e),
    }

    // Test to_uppercase
    let result = is_uppercase::to_uppercase(&[Arc::new(Object::String("a".to_string()))]);
    match result {
        Ok(obj) => {
            match &*obj {
                Object::String(s) if s == "A" => println!("✓ to_uppercase(a) = A"),
                _ => println!("❌ to_uppercase(a) should be A, got {:?}", obj),
            }
        },
        Err(e) => println!("❌ to_uppercase(a) error: {:?}", e),
    }

    // Test to_lowercase 
    let result = is_uppercase::to_lowercase(&[Arc::new(Object::String("A".to_string()))]);
    match result {
        Ok(obj) => {
            match &*obj {
                Object::String(s) if s == "a" => println!("✓ to_lowercase(A) = a"),
                _ => println!("❌ to_lowercase(A) should be a, got {:?}", obj),
            }
        },
        Err(e) => println!("❌ to_lowercase(A) error: {:?}", e),
    }
}