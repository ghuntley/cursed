//! Tests for generic struct monomorphization

use cursed::ast::declarations::SquadStatement;
use cursed::ast::expressions::Identifier;
use cursed::ast::statements::fields::FieldStatement;
use cursed::ast::traits::Node;
use cursed::core::generic_instantiation::GenericInstantiator;
use cursed::core::type_checker::Type;

/// Helper function to create a generic struct AST
fn create_generic_struct(
    name: &str,
    type_params: Vec<&str>,
    fields: Vec<(&str, &str)>,
) -> SquadStatement {
    // Create type parameters
    let type_parameters: Vec<Identifier> = type_params
        .iter()
        .map(|param| Identifier {
            token: "IDENT".to_string(),
            value: param.to_string(),
        })
        .collect();

    // Create fields
    let struct_fields: Vec<FieldStatement> = fields
        .iter()
        .map(|(field_name, field_type)| FieldStatement {
            token: "IDENT".to_string(),
            name: Identifier {
                token: "IDENT".to_string(),
                value: field_name.to_string(),
            },
            type_name: Identifier {
                token: "IDENT".to_string(),
                value: field_type.to_string(),
            },
        })
        .collect();

    // Create the struct statement
    SquadStatement {
        token: "be_like".to_string(),
        name: Identifier {
            token: "IDENT".to_string(),
            value: name.to_string(),
        },
        type_parameters,
        fields: struct_fields,
    }
}

#[test]
fn test_simple_struct_monomorphization() {
    let generic_struct = create_generic_struct(
        "Pair",
        vec!["T", "U"],
        vec![("first", "T"), ("second", "U")],
    );

    // Create a GenericInstantiator
    let instantiator = GenericInstantiator::new();

    // Create a specialized version of the struct
    let specialized = instantiator.monomorphize_struct(
        &generic_struct,
        &[Type::Normie, Type::Tea],
    ).unwrap();

    // Verify the specialized struct
    assert_eq!(specialized.name.string(), "Pairnormie_tea");
    assert!(specialized.type_parameters.is_empty());
    assert_eq!(specialized.fields.len(), 2);

    // Check first field
    assert_eq!(specialized.fields[0].name.string(), "first");
    assert_eq!(specialized.fields[0].type_name.string(), "normie");

    // Check second field
    assert_eq!(specialized.fields[1].name.string(), "second");
    assert_eq!(specialized.fields[1].type_name.string(), "tea");
}

#[test]
fn test_nested_struct_monomorphization() {
    // Define LinkedList<T> with 'value' of type T and 'next' of type LinkedList<T>
    let generic_struct = create_generic_struct(
        "LinkedList",
        vec!["T"],
        vec![("value", "T"), ("next", "LinkedList[T]")],
    );

    // Create a specialized version of the struct
    let instantiator = GenericInstantiator::new();
    let specialized = instantiator.monomorphize_struct(
        &generic_struct,
        &[Type::Normie],
    ).unwrap();

    // Verify the specialized struct
    assert_eq!(specialized.name.string(), "LinkedListnormie");
    assert!(specialized.type_parameters.is_empty());
    assert_eq!(specialized.fields.len(), 2);

    // Check fields
    assert_eq!(specialized.fields[0].name.string(), "value");
    assert_eq!(specialized.fields[0].type_name.string(), "normie");

    // The 'next' field should be of type LinkedList<Normie>
    assert_eq!(specialized.fields[1].name.string(), "next");
    assert_eq!(specialized.fields[1].type_name.string(), "LinkedList[normie]");
}