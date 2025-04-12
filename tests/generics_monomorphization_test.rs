//! Tests for generic instantiation of types

use cursed::core::generic_instantiation::GenericInstantiator;
use cursed::core::type_checker::{Type, TypeChecker};
use cursed::error::Error;

#[test]
fn test_simple() {
    assert_eq!(1 + 1, 2);
    println!("Simple test ran successfully!");
}

/// Test that generic instantiation works correctly with types
#[test]
fn test_generic_instantiation() -> Result<(), Error> {
    // Create a generic instantiator
    let mut instantiator = GenericInstantiator::new();

    // Add parameter mappings
    instantiator.add_type_param("T", Type::Tea);
    instantiator.add_type_param("K", Type::Lit);
    instantiator.add_type_param("V", Type::Normie);

    // Test basic type parameter substitution
    let param_type = Type::TypeParam("T".to_string());
    let concrete_type = instantiator.instantiate_type(&param_type)?;
    assert_eq!(concrete_type, Type::Tea);

    // Test array type instantiation
    let array_type = Type::Array(Box::new(Type::TypeParam("T".to_string())), 5);
    let concrete_array = instantiator.instantiate_type(&array_type)?;
    assert_eq!(concrete_array, Type::Array(Box::new(Type::Tea), 5));

    // Test map type instantiation
    let map_type = Type::Map(
        Box::new(Type::TypeParam("K".to_string())),
        Box::new(Type::TypeParam("V".to_string())),
    );
    let concrete_map = instantiator.instantiate_type(&map_type)?;
    assert_eq!(
        concrete_map,
        Type::Map(Box::new(Type::Lit), Box::new(Type::Normie))
    );

    // Test nested generic type instantiation
    let nested_type = Type::Struct(
        "Pair".to_string(),
        vec![
            Box::new(Type::TypeParam("K".to_string())),
            Box::new(Type::Array(Box::new(Type::TypeParam("T".to_string())), 3)),
        ],
    );
    let concrete_nested = instantiator.instantiate_type(&nested_type)?;
    assert_eq!(
        concrete_nested,
        Type::Struct(
            "Pair".to_string(),
            vec![
                Box::new(Type::Lit),
                Box::new(Type::Array(Box::new(Type::Tea), 3))
            ]
        )
    );

    Ok(())
}
