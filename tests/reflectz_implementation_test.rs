use cursed::{
    object::Object,
    stdlib::reflectz::{self, *},
    error::Error,
};
use std::rc::Rc;

#[test]
fn test_type_of() {
    let obj = Rc::new(Object::Integer(42));
    let result = reflectz::type_of(&[obj]).unwrap();
    
    if let Object::String(type_name) = &*result {
        assert!(type_name == "integer" || type_name == "normie" || type_name == "int32", 
                "Wrong type name: {}", type_name);
    } else {
        panic!("Expected String object, got {:?}", result);
    }
}

#[test]
fn test_is_type() {
    let obj = Rc::new(Object::Integer(42));
    let type_name = Rc::new(Object::String("integer".to_string()));
    
    let result = reflectz::is_type(&[obj.clone(), type_name]).unwrap();
    
    if let Object::Boolean(is_int) = &*result {
        assert!(*is_int, "Should identify object as an integer");
    } else {
        panic!("Expected Boolean object, got {:?}", result);
    }
    
    // Test wrong type
    let wrong_type = Rc::new(Object::String("string".to_string()));
    let result = reflectz::is_type(&[obj, wrong_type]).unwrap();
    
    if let Object::Boolean(is_int) = &*result {
        assert!(!*is_int, "Should not identify integer as string");
    } else {
        panic!("Expected Boolean object, got {:?}", result);
    }
}

#[test]
fn test_get_field() {
    // Create a struct object with fields for testing
    let person = Rc::new(Object::Struct {
        name: "Person".to_string(),
        fields: vec![
            ("Name".to_string(), Rc::new(Object::String("John".to_string()))),
            ("Age".to_string(), Rc::new(Object::Integer(30))),
        ],
    });
    
    let field_name = Rc::new(Object::String("Name".to_string()));
    let result = reflectz::get_field(&[person.clone(), field_name]).unwrap();
    
    if let Object::String(name) = &*result {
        assert_eq!(name, "John", "Field value doesn't match expected");
    } else {
        panic!("Expected String object for Name field, got {:?}", result);
    }
    
    // Test getting integer field
    let age_field = Rc::new(Object::String("Age".to_string()));
    let result = reflectz::get_field(&[person, age_field]).unwrap();
    
    if let Object::Integer(age) = &*result {
        assert_eq!(*age, 30, "Age field value doesn't match expected");
    } else {
        panic!("Expected Integer object for Age field, got {:?}", result);
    }
}

#[test]
fn test_set_field() {
    // Create a struct object with fields for testing
    let person = Rc::new(Object::Struct {
        name: "Person".to_string(),
        fields: vec![
            ("Name".to_string(), Rc::new(Object::String("John".to_string()))),
            ("Age".to_string(), Rc::new(Object::Integer(30))),
        ],
    });
    
    let field_name = Rc::new(Object::String("Name".to_string()));
    let new_value = Rc::new(Object::String("Jane".to_string()));
    
    let _ = reflectz::set_field(&[person.clone(), field_name.clone(), new_value]).unwrap();
    
    // Verify the field was updated
    let result = reflectz::get_field(&[person, field_name]).unwrap();
    
    if let Object::String(name) = &*result {
        assert_eq!(name, "Jane", "Field value wasn't updated correctly");
    } else {
        panic!("Expected String object for updated Name field, got {:?}", result);
    }
}

#[test]
fn test_call_method() {
    // This test would require more complex setup with method info
    // For now we'll just test the basic interface
    let obj = Rc::new(Object::Integer(42));
    let method_name = Rc::new(Object::String("toString".to_string()));
    
    let result = reflectz::call_method(&[obj, method_name]).unwrap();
    
    // We expect null in the simplified implementation
    assert!(matches!(*result, Object::Null), "Expected Null result from unimplemented call_method");
}