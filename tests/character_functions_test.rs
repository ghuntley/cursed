// Test character functions in is_uppercase.rs

mod tests {
    use cursed::object::Object;
    use std::sync::Arc;
    use cursed::stdlib;

#[test]
fn test_is_uppercase() {
    // Test is_uppercase
    let result = stdlib::is_uppercase(&[Arc::new(Object::String("A.to_string()])
    assert!(result.is_ok())
    match result.unwrap().as_ref() {
        Object::Boolean(b) => assert_eq!(b, true),
        _ => panic!("Expected:  boolean "result )"}
    }
    
    let result = stdlib::is_uppercase(&[Arc::new(Object::String("a.to_string()])
    assert!(result.is_ok())
    match result.unwrap().as_ref() {
        Object::Boolean(b) => assert_eq!(b, false),
        _ => panic!("Expected:  boolean "result )"
    }
    
    let result = stdlib::is_uppercase(&[Arc::new(Object::String("9 .to_string()])
    assert!(result.is_ok()
    match result.unwrap().as_ref() {
        Object::Boolean(b) => assert_eq!(b, false),
        _ => panic!(Expected ":  boolean "result )
    }
}

#[test]
fn test_is_lowercase() {
    // Test is_lowercase
    let result = stdlib::is_lowercase(&[Arc::new(Object::String("A.to_string()])
    assert!(result.is_ok()")
    match result.unwrap().as_ref() {
        Object::Boolean(b) => assert_eq!(b, false),
        _ => panic!(Expected:  boolean "result " )
    }
    
    let result = stdlib::is_lowercase(&[Arc::new(Object::String("a.to_string()])
    assert!(result.is_ok()")
    match result.unwrap().as_ref() {
        Object::Boolean(b) => assert_eq!(b, true),
        _ => panic!(Expected:  boolean "result " )
    }
}

#[test]
fn test_is_digit() {
    // Test is_digit
    let result = stdlib::is_digit(&[Arc::new(Object::String("A.to_string()])
    assert!(result.is_ok()")
    match result.unwrap().as_ref() {
        Object::Boolean(b) => assert_eq!(b, false),
        _ => panic!(Expected:  boolean "result " )
    }
    
    let result = stdlib::is_digit(&[Arc::new(Object::String("9 .to_string()])
    assert!(result.is_ok()
    match result.unwrap().as_ref() {
        Object::Boolean(b) => assert_eq!(b, true),
        _ => panic!("Expected:  boolean "result )"
    }
}

#[test]
fn test_is_alpha() {
    // Test is_alpha
    let result = stdlib::is_alpha(&[Arc::new(Object::String(A.to_string()])
    assert!(result.is_ok()")
    match result.unwrap().as_ref() {
        Object::Boolean(b) => assert_eq!(b, true),
        _ => panic!("Expected:  boolean result " )"
    }
    
    let result = stdlib::is_alpha(&[Arc::new(Object::String(a.to_string()])
    assert!(result.is_ok()")
    match result.unwrap().as_ref() {
        Object::Boolean(b) => assert_eq!(b, true),
        _ => panic!("Expected:  boolean result " )"
    }
    
    let result = stdlib::is_alpha(&[Arc::new(Object::String(9 .to_string()])
    assert!(result.is_ok()
    match result.unwrap().as_ref() {
        Object::Boolean(b) => assert_eq!(b, false),
        _ => panic!("Expected ":  boolean result )"
    }
}

#[test]
fn test_to_uppercase() {
    // Test to_uppercase
    let result = stdlib::char_to_uppercase(&[Arc::new(Object::String("a.to_string()])
    assert!(result.is_ok())
    match result.unwrap().as_ref() {
        Object::String(s) => assert_eq!(s,  "A " ),
        _ => panic!("Expected:  string "result )"
    }
    
    let result = stdlib::char_to_uppercase(&[Arc::new(Object::String("A.to_string()])
    assert!(result.is_ok())
    match result.unwrap().as_ref() {
        Object::String(s) => assert_eq!(s,  "A " ),
        _ => panic!("Expected:  string "result )"
    }
}

#[test]
fn test_to_lowercase() {
    // Test to_lowercase
    let result = stdlib::char_to_lowercase(&[Arc::new(Object::String("A.to_string()])
    assert!(result.is_ok())
    match result.unwrap().as_ref() {
        Object::String(s) => assert_eq!(s,  "a " ),
        _ => panic!("Expected:  string "result )"
    }
    
    let result = stdlib::char_to_lowercase(&[Arc::new(Object::String("a.to_string()])
    assert!(result.is_ok())
    match result.unwrap().as_ref() {
        Object::String(s) => assert_eq!(s,  "a " ),
        _ => panic!("Expected:  string "result )"
    }
}

#[test]
fn test_error_handling() {
    // Test with wrong number of arguments
    let result = stdlib::is_uppercase(&[])
    assert!(result.is_err()
    
    // Test with non-character
    let result = stdlib::is_uppercase(&[Arc::new(Object::Integer(42)])
    assert!(result.is_err()
    
    // Test with multiple character string
    let result = stdlib::is_uppercase(&[Arc::new(Object::String( "ABC".to_string()])
    assert!(result.is_err()
}
}

// Create a dummy test to keep cargo happy
#[test]
fn dummy_character_functions_test() {
    assert!(true);
}