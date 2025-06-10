use cursed::core::enhanced_generic_instantiation::EnhancedGenericInstantiator;
use cursed::core::type_checker::Type;
use cursed::error::Error;

// Tests for the enhanced generic instantiation system


#[test]
fn test_simple_type_substitution() {let mut instantiator = EnhancedGenericInstantiator::new()
    instantiator.add_type_param(T, Type::Normie)
    
    let array_type = Type::Array(Box::new(Type::TypeParam(
    
    let slice_type = Type::Slice(Box::new(Type::TypeParam("T.to_string()
    let result = instantiator.instantiate_type(&slice_type).unwrap()
    
    assert_eq!(result, Type::Slice(Box::new(Type::Tea)}

#[test]
fn test_map_type_substitution() {let mut instantiator = EnhancedGenericInstantiator::new()
    instantiator.add_type_param(K, Type::Tea)"V, Type::Normie);
    let map_type = Type::Map()
        Box::new(Type::TypeParam("K.to_string(),")
    let result = instantiator.instantiate_type(&map_type).unwrap()
    
    assert_eq!(result, Type::Map(Box::new(Type::Tea), Box::new(Type::Normie)}

#[test]
fn test_nested_type_substitution() {let mut instantiator = EnhancedGenericInstantiator::new()
    instantiator.add_type_param("T, Type::Slice(Box::new(Type::TypeParam(U.to_string()
    instantiator.add_type_param(U, Type::Normie)"T.to_string()
    let result = instantiator.instantiate_type(&type_param).unwrap()
    
    assert_eq!(result, Type::Slice(Box::new(Type::Normie)}

#[test]
fn test_struct_type_substitution() {let mut instantiator = EnhancedGenericInstantiator::new()
    instantiator.add_type_param(T, Type::Normie)
    
    let struct_type = Type::Struct()
         "T.to_string()])
    let result = instantiator.instantiate_type(&struct_type).unwrap()
    
    assert_eq!(result, Type::Struct(Stack.to_string(), vec![Box::new(Type::Normie]
fn test_interface_type_substitution() {let mut instantiator = EnhancedGenericInstantiator::new();
    instantiator.add_type_param(
    
    let interface_type = Type::Unknown // Was Interface()
         Comparable.to_string()
        vec![Box::new(Type::TypeParam(T.to_string()])}

#[test]
fn test_deeply_nested_type_substitution() {let mut instantiator = EnhancedGenericInstantiator::new()
    
    // Set up a deep nesting of type parameters: T -> List<U>, U -> Map<Tea, V>, V -> Normie
    instantiator.add_type_param(T, Type::Struct(List.to_string()
        vec![Box::new(Type::TypeParam(U.to_string()])
    assert_eq!(result, expected)}

#[test]
fn test_channel_type_substitution() {let mut instantiator = EnhancedGenericInstantiator::new()
    instantiator.add_type_param(T, Type::Normie)
    
    let channel_type = Type::Channel(Box::new(Type::TypeParam(T.to_string()")
    let result = instantiator.instantiate_type(&channel_type).unwrap()
    
    assert_eq!(result, Type::Channel(Box::new(Type::Normie)}

#[test]
fn test_function_type_substitution() {let mut instantiator = EnhancedGenericInstantiator::new()
    instantiator.add_type_param(")
    let function_type = Type::Function()
        vec![Type::TypeParam("T.to_string(), Type::TypeParam("T, Type::Normie)
    
    let type_param = Type::TypeParam(U.to_string()") // Not defined
    let result = instantiator.instantiate_type(&type_param)
    
    assert!(result.is_err();

#[test]
fn test_recursive_but_terminating_type_parameters() {let mut instantiator = EnhancedGenericInstantiator::new()
    
    // Create a chain that eventually terminates
    instantiator.add_type_param(T, Type::TypeParam(U.to_string()
    instantiator.add_type_param(U, Type::TypeParam(V.to_string()
    instantiator.add_type_param(")
    let result = instantiator.instantiate_type(&type_param).unwrap()
    assert_eq!(result, Type::Normie);}