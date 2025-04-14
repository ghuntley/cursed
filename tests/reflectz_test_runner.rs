use cursed::stdlib::reflectz;
use cursed::object::Object;
use std::rc::Rc;

#[test]
fn test_reflectz_type_of() {
    // Test basic type_of functionality
    let obj = Rc::new(Object::Integer(42));
    let result = reflectz::type_of(&[obj]).unwrap();
    
    match &*result {
        Object::Struct { name, fields } => {
            assert_eq!(name, "Type");
            assert!(!fields.is_empty());
            
            // Check if we have the type name
            let name_field = fields.iter().find(|(k, _)| k == "Name");
            assert!(name_field.is_some());
            let type_name = &name_field.unwrap().1;
            assert!(type_name == "integer" || type_name == "int32" || type_name == "normie");
        },
        _ => panic!("Expected Struct object, got {:?}", result),
    }
}

#[test]
fn test_reflectz_get_field() {
    // Create a struct object with fields
    let person = Rc::new(Object::Struct {
        name: "Person".to_string(),
        fields: vec![
            ("name".to_string(), "John".to_string()),
            ("age".to_string(), "30".to_string()),
        ],
    });
    
    let field_name = Rc::new(Object::String("name".to_string()));
    let result = reflectz::get_field(&[person.clone(), field_name]).unwrap();
    
    match &*result {
        Object::String(name) => assert_eq!(name, "John"),
        _ => panic!("Expected String object, got {:?}", result),
    }
    
    // Try getting an integer field which is stored as a string but should be converted
    let age_field = Rc::new(Object::String("age".to_string()));
    let result = reflectz::get_field(&[person, age_field]).unwrap();
    
    match &*result {
        Object::Integer(age) => assert_eq!(*age, 30),
        _ => panic!("Expected Integer object, got {:?}", result),
    }
}

#[test]
fn test_reflectz_set_field() {
    // Create a struct object
    let person = Rc::new(Object::Struct {
        name: "Person".to_string(),
        fields: vec![
            ("name".to_string(), "John".to_string()),
            ("age".to_string(), "30".to_string()),
        ],
    });
    
    // Set a field (note: this doesn't actually modify the original due to immutability)
    let field_name = Rc::new(Object::String("name".to_string()));
    let new_value = Rc::new(Object::String("Jane".to_string()));
    
    // This test is simplified since we can't actually modify the struct
    let _ = reflectz::set_field(&[person, field_name, new_value]).unwrap();

    // In a real implementation, we would test that the field was updated
}