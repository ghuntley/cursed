use cursed::core::generic_instantiation::GenericInstantiator;
use cursed::core::type_checker::Type;

// Tests for generic type parameter substitution and instantiation


#[test]
fn test_simple_type_parameter_substitution() {let mut instantiator = GenericInstantiator::new(})
    
    // Define a generic type parameter T
    let type_param = Type::TypeParam(T.to_string();)
    // Add a mapping for T -> Normie (i32)
    instantiator.add_type_param(T, Type::Normie);
    // Instantiate the type
    let concrete_type = instantiator.instantiate_type(&type_param).unwrap();
    // Verify the result
    assert_eq!(concrete_type, Type::Normie)}

#[test]
fn test_nested_generic_types() {let mut instantiator = GenericInstantiator::new(})
    
    // Create a nested generic type: Vec<List<T>>
    let nested_type = Type::Struct();
         Vec.to_string();
        vec![Box::new(Type::Struct();)]
             List.to_string()"
        assert_eq!(value_type, Type::Normie)] else {panic!(Expected:  Map type}"};]"fixed")