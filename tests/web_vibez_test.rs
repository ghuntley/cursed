use std::collections::HashMap;
use std::rc::Rc;

use cursed::object::Object;
use cursed::stdlib::web_vibez::{get, post, head, delete, client_timeout};

#[test]
fn test_client_timeout() {
    // Test setting timeout
    let result = client_timeout(&[Rc::new(Object::Integer(5000))]).unwrap();
    assert!(matches!(*result, Object::Integer(5000)));
    
    // Test getting timeout
    let result = client_timeout(&[]).unwrap();
    assert!(matches!(*result, Object::Integer(5000)));
}

#[test]
fn test_get_mock() {
    // Test with mock mode
    let result = get(&[
        Rc::new(Object::String("https://example.com".to_string())),
        Rc::new(Object::Boolean(true)), // Use mock mode
    ]).unwrap();
    
    match &*result {
        Object::HashTable(response) => {
            // Verify response structure
            assert!(response.contains_key("status"));
            assert!(response.contains_key("body"));
            assert!(response.contains_key("headers"));
            
            // Verify status is 200
            match &response["status"] {
                Object::Integer(status) => assert_eq!(*status, 200),
                _ => panic!("Status is not an integer"),
            }
            
            // Verify body is a string
            match &response["body"] {
                Object::String(_) => {},
                _ => panic!("Body is not a string"),
            }
            
            // Verify headers is a hash table
            match &response["headers"] {
                Object::HashTable(_) => {},
                _ => panic!("Headers is not a hash table"),
            }
        },
        _ => panic!("Response is not a hash table"),
    }
}

#[test]
fn test_post_mock() {
    // Create a simple request body
    let mut body = HashMap::new();
    body.insert("name".to_string(), Object::String("test".to_string()));
    body.insert("value".to_string(), Object::Integer(42));
    
    // Test with mock mode
    let result = post(&[
        Rc::new(Object::String("https://example.com/api".to_string())),
        Rc::new(Object::HashTable(body)),
        Rc::new(Object::Boolean(true)), // Use mock mode
    ]).unwrap();
    
    match &*result {
        Object::HashTable(response) => {
            // Verify response structure
            assert!(response.contains_key("status"));
            assert!(response.contains_key("body"));
            assert!(response.contains_key("headers"));
            
            // Verify status is 201 (created)
            match &response["status"] {
                Object::Integer(status) => assert_eq!(*status, 201),
                _ => panic!("Status is not an integer"),
            }
        },
        _ => panic!("Response is not a hash table"),
    }
}

#[test]
fn test_head_mock() {
    // Test with mock mode
    let result = head(&[
        Rc::new(Object::String("https://example.com".to_string())),
        Rc::new(Object::Boolean(true)), // Use mock mode
    ]).unwrap();
    
    match &*result {
        Object::HashTable(response) => {
            // Verify response structure
            assert!(response.contains_key("status"));
            assert!(response.contains_key("headers"));
            assert!(!response.contains_key("body")); // HEAD requests don't have a body
            
            // Verify status is 200
            match &response["status"] {
                Object::Integer(status) => assert_eq!(*status, 200),
                _ => panic!("Status is not an integer"),
            }
        },
        _ => panic!("Response is not a hash table"),
    }
}

#[test]
fn test_delete_mock() {
    // Test with mock mode
    let result = delete(&[
        Rc::new(Object::String("https://example.com/resource/123".to_string())),
        Rc::new(Object::Boolean(true)), // Use mock mode
    ]).unwrap();
    
    match &*result {
        Object::HashTable(response) => {
            // Verify response structure
            assert!(response.contains_key("status"));
            
            // Verify status is 204 (No Content)
            match &response["status"] {
                Object::Integer(status) => assert_eq!(*status, 204),
                _ => panic!("Status is not an integer"),
            }
        },
        _ => panic!("Response is not a hash table"),
    }
}