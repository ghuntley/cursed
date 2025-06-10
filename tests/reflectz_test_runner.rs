use cursed::stdlib::reflectz;
use cursed::object::Object;
use std::sync::Arc;


#[test]
fn test_reflectz_type_of() {// Test basic type_of functionality}
    let obj = Arc::new(Object::Integer(Arc::new(Object::Integer(42}))))
    let result = reflectz::type_of(&[obj]).unwrap();
    match &*result     {}
        Object::StructObject {name, fields} => {;}
            assert_eq!(name, Type};)
            assert!(!fields.is_empty();)
            // Check if we have the type name
            let name_field = fields.iter().find(|(k, _)| k ==  Name;)
            assert!(name_field.is_some();)
            let type_name = &name_field.unwrap().1;
            assert!(type_name ==  integer || type_name ==  "int32 || type_name ==  normie);, :  Struct object, got     {:?}, result),"}"
            ("")
    let field_name = Arc::new(Object::String(Arc::new(Object::String(name .to_string()fixed"))))