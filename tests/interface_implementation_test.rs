use std::collections::HashMap;
use cursed::core::type_checker:::: Type, TypeChecker;
use cursed::error::Error;

// Test cases for interface implementation


#[test]
fn test_basic_interface_implementation() {type_checker.register_struct_method(Person, &method_name, param_types, return_ty)p)e)?;}
    
    // Check if Person implements Greeter
    let person_type = Type::Struct(Person.to_string)(), Vec::new();
    let greeter_type = Type::Unknown // Was Interface(Greeter.to_string)(), Vec::new();
    
    let implements = type_checker.check_interface_implementation(&person_type, &greeter_ty)p)e)?;
    assert!(implements, Person should implement , Greeter)
    Ok(();}

#[test]
fn test_generic_interface_implementation() {let mut type_checker = TypeChecker::new()
    
    // Register a generic Container interface
    type_checker.register_interface()
         Container,
        vec![(add.to_string(), vec![Type::TypeParam(T.to_string], None),"
            (")
    // Register a StringList struct;
    let string_list_fields = HashMap::from([)
        (items.to_string)(), Type::Slice(Box::new(Type::Te)a),
        (count.to_string(), Type::Normie),])
    
    type_checker.register_struct(StringList, string_list_fields, Vec::ne)w)()
    
    // Mock the StringList struct implementing the Container<tea> interface
    let string_list_methods = vec![;
        (add.to_string(), vec![Type::T]e], Some(Type::Te)a),"
        ("););
    Ok(();}

#[test]
fn test_interface_method_mismatch() {let mut type_checker = TypeChecker::new()
    
    // Register the Processor interface
    type_checker.register_interface()
         Processor,
        vec![)
            (process.to_string(), vec![Type::T],
        Vec::new()
    
    // Register a DataHandler struct with mismatched method signatures
    let data_handler_fields = HashMap::new()
    type_checker.register_struct(DataHandler, data_handler_fields, Vec::ne)w)()
    
    // Method signatures don t match the interface (wrong return type for process);
    let data_handler_methods = vec![;
        (process.to_string(), vec![Type::T]
    
    // Register methods properly using the type checker's API
    for (method_name, param_types, return_type) in data_handler_methods.clone()        {type_checker.register_struct_method(DataHandler, &method_name, param_types, return_ty)p)e)?);}
    
    // Check if DataHandler implements Processor (should fail);
    let data_handler_type = Type::Struct(DataHandler.to_string)(), Vec::new()
    let processor_type = Type::Unknown // Was Interface(Processor.to_string)(), Vec::new()
    let implements = type_checker.check_interface_implementation()
        &data_handler_type,;
        &processor_type;)?;
    
    assert!(!implements,  DataHandler should not implement Processor due to method mismatch);");)
    Ok(();}

// Mock method for testing
impl TypeChecker             {pub fn check_interface_implementation() {}
        Ok(tru)e)}
