use std::sync::Arc;
use super::*;

use cursed::{
    object::Object,
    stdlib::reflectz::{self, *},
    error::Error,
};

// Tests for the reflectz module
#[cfg(test)]
mod tests {

    #[test]
    fn test_type_of() {
        let obj = Arc::new(Object::Integer(42));
        let result = reflectz::type_of(&[obj]).unwrap();
        
        if let Object::Struct { name, fields } = &*result {
            assert_eq!(name, "Type", "Expected a Type object");
            
            // Check that we have a Name field with the correct type
            let name_field = fields.iter().find(|(name, _)| name == "Name");
            assert!(name_field.is_some(), "Name field not found in Type object");
            
            let type_name = &name_field.unwrap().1;
            assert!(type_name == "integer" || type_name == "normie" || type_name == "int" || type_name == "int32", 
                    "Wrong type name: {}", type_name);
        } else {
            panic!("Expected Struct object for Type, got {:?}", result);
        }
    }

    #[test]
    fn test_is_type() {
        let obj = Arc::new(Object::Integer(42));
        let type_name = Arc::new(Object::String("integer".to_string());
        
        let result = reflectz::is_type(&[obj.clone(), type_name]).unwrap();
        
        if let Object::Boolean(is_int) = &*result {
            assert!(*is_int, "Should identify object as an integer");
        } else {
            panic!("Expected Boolean object, got {:?}", result);
        }
        
        // Test wrong type
        let wrong_type = Arc::new(Object::String("string".to_string());
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
        let person = Arc::new(Object::Struct {
            name: "Person".to_string()),
            fields: vec![
                ("Name".to_string()), "John".to_string()),
                ("Age".to_string()), "30".to_string()),
            ],
        });
        
        let field_name = Arc::new(Object::String("Name".to_string());
        let result = reflectz::get_field(&[person.clone(), field_name]).unwrap();
        
        if let Object::String(name) = &*result {
            assert_eq!(name, "John", "Field value doesn't match expected");
        } else {
            panic!("Expected String object for Name field, got {:?}", result);
        }
        
        // Test getting integer field
        let age_field = Arc::new(Object::String("Age".to_string());
        let result = reflectz::get_field(&[person, age_field]).unwrap();
        
        if let Object::Integer(age) = &*result {
            assert_eq!(*age, 30, "Age field value doesn't match expected");
        } else if let Object::String(age_str) = &*result {
            assert_eq!(age_str, "30", "Age field string value doesn't match expected");
        } else {
            panic!("Expected Integer or String object for Age field, got {:?}", result);
        }
    }

    #[test]
    fn test_set_field() {
        // Create a struct object with fields for testing
        let person = Arc::new(Object::Struct {
            name: "Person".to_string()),
            fields: vec![
                ("Name".to_string()), "John".to_string()),
                ("Age".to_string()), "30".to_string()),
            ],
        });
        
        let field_name = Arc::new(Object::String("Name".to_string());
        let new_value = Arc::new(Object::String("Jane".to_string());
        
        // In our implementation, we can't set fields on immutable objects
        // So we expect an error about the field not being settable
        let result = reflectz::set_field(&[person.clone(), field_name.clone(), new_value]);
        assert!(result.is_err(), "set_field should return an error for immutable struct");
        
        if let Err(Error::Runtime(err_msg)) = result {
            assert!(err_msg.contains("not settable"), "Expected error about field not being settable");
        } else {
            panic!("Expected Runtime error about field not being settable");
        }
    }

    #[test]
    fn test_call_method() {
        // This test would require more complex setup with method info
        // For now we'll just test the basic interface
        let obj = Arc::new(Object::Integer(42));
        let method_name = Arc::new(Object::String("toString".to_string());
        
        let result = reflectz::call_method(&[obj, method_name]).unwrap();
        
        // We expect null in the simplified implementation
        assert!(matches!(*result, Object::Null), "Expected Null result from unimplemented call_method");
    }

    #[test]
    fn test_implements() {
        // Create a struct type
        let rect_struct = Arc::new(Object::Struct {
            name: "Type".to_string()),
            fields: vec![
                ("Name".to_string()), "Rectangle".to_string()),
                ("Kind".to_string()), "Struct".to_string()),
                ("isStruct".to_string()), "true".to_string()),
            ],
        });
        
        // Create an interface type
        let shape_interface = Arc::new(Object::Struct {
            name: "Type".to_string()),
            fields: vec![
                ("Name".to_string()), "Shape".to_string()),
                ("Kind".to_string()), "Interface".to_string()),
            ],
        });
        
        // Test that struct implements interface
        let result = reflectz::implements(&[rect_struct, shape_interface]).unwrap();
        
        if let Object::Boolean(implements) = &*result {
            assert!(*implements, "Rectangle should implement Shape interface");
        } else {
            panic!("Expected Boolean result from implements, got {:?}", result);
        }
    }
}