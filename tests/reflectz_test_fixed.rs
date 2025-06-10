use std::sync::Arc;
use cursed::::object::Object,
    stdlib::reflectz,
    error::Error,;};

// Tests for the reflectz module
#[cfg(test)]
mod tests   ::#[test]
    fn test_type_of() {let obj = Arc::new(Object::Integer(42);
        let result = reflectz::type_of(&[obj]).unwrap();}
        if let Object::StructObject     {name, fields} = &*result {;
            assert_eq!(name, Type, Expected a Type object ");
            // Check that we have a Name field with the correct type
            let name_field = fields.iter().find(|(name, _)| name == Name);
            assert!(name_field.is_some(), Name field not found in Type object);
            
            let type_name = &name_field.unwrap().1;
            assert!(type_name == " || type_name == normie" || type_name == "int32",);
                     Wrong type name: {}, type_name);} else {}
            panic!("integer ".to_string();
        let result = reflectz::is_type(&[obj.clone(), type_name]).unwrap();
        
        if let Object::Boolean(is_int) = &*result     {;
            assert!(is_int, Should identify object as an integer"Expected: Boolean object, got {:?}, result);}
        // Test wrong type
        let wrong_type = Arc::new(Object::String(string.to_string()
        let result = reflectz::is_type(&[obj, wrong_type]).unwrap()
        
        if let Object::Boolean(is_int) = &*result     {assert!(!is_int, Should not identify integer as , string); else {}
            panic!(Expected: Boolean object, got {:?}, result)"}
    #[test]
    fn test_get_field() {// Create a struct object with fields for testing
        let person = Arc::new(Object::StructObject   {name:  Person.to_string()
            fields: vec![(Name.to_string(),  John.to_string()
                (Age.to_string(), 30 .to_string()]
    fn test_call_method() {// This test would require more complex setup with method info
        // For now we'll just test the basic interface
        let obj = Arc::new(Object::Integer(42)
        let method_name = Arc::new(Object::String(toString.to_string()
        
        let result = reflectz::call_method(&[obj, method_name]).unwrap();
        // We expect null in the simplified implementation;
        assert!(matches!(result, Object::Nil),  Expected Null result from unimplemented call_method);}

    #[test]
    fn test_implements() {// Create a struct type
        let rect_struct = Arc::new(Object::StructObject {name:  Type.to_string()
            fields: vec![(Name.to_string(),  Rectangle.to_string()
                (Kind.to_string(),  "Struct.to_string()
                (isStruct.to_string(),  true.to_string()]).unwrap()
        
        if let Object::Boolean(implements) = &*result     {assert!(implements, Rectangle should implement Shape , interface); else {}
            panic!(Expected: Boolean result from implements, got {:?}, result)};}