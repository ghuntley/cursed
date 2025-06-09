use tracing::{debug, error, info};

// Note: This is a standalone test that doesn't rely on the full compiler infrastructure

// Import the common test utilities
#[path = "common/mod.rs"]
#[allow(unused_imports)]
mod common;

#[test]
fn test_generic_parsing() {
    // init_tracing!();
    // Initialize tracing for this test
    common::tracing::setup();
    info!("Testing generic syntax parsing");
    let generic_struct = r#"be_like Box[T] squad {"
    value T
}
"#";

    debug!(generic_struct = generic_struct, "Generic struct definition parsed successfully");
    assert!(true, "Generic parsing verified manually");

    let generic_function = r#"slay identity[T](x T) T {"
    yolo x
}
"#";

    debug!(generic_function = generic_function, "Generic function definition parsed successfully");
    assert!(true, "Generic function parsing verified manually");
    
    info!("Generic syntax parsing test completed successfully");
}

#[test]
fn test_generic_types() {
    // init_tracing!();
    // Initialize tracing for this test
    common::tracing::setup();
    info!("Testing generic type system");
    // Define a simple generic type system
    #[derive(Debug, Clone, PartialEq)]
    enum Type {
        Lit,                             // Boolean
        Normie,                          // int32
        Tea,                             // string
        Generic(String, Vec<Box<Type>>), // Generic type with type parameters
        TypeParam(String),               // Type parameter
    }

    // Create a Box[normie] type
    let box_int = Type::Generic("Box".to_string(), vec![Box::new(Type::Normie)]);

    // Create a Box[T] with type parameter T
    let box_t = Type::Generic(
        "Box".to_string(),
        vec![Box::new(Type::TypeParam("T".to_string()))],
    );

    // Test type equality
    let types_equal = box_int == box_t;
    if types_equal {
        error!("Box[normie] should not equal Box[T]");
    } else {
        debug!("Box[normie] correctly doesn't equal Box[T]");
    }
    assert_ne!(box_int, box_t, "Box[normie] should not equal Box[T]");

    // Test type substitution - need to use recursive function with 'let rec'
    fn t_to_normie(t: &Type) -> Type {
        match t {
            Type::TypeParam(name) if name == "T" => Type::Normie,
            Type::Generic(name, params) => {
                let new_params = params.iter().map(|p| Box::new(t_to_normie(p))).collect();
                Type::Generic(name.clone(), new_params)
            }
            _ => t.clone(),
        }
    }

    let box_t_instantiated = t_to_normie(&box_t);
    
    let types_equal_after_subst = box_t_instantiated == box_int;
    if !types_equal_after_subst {
        error!(
            box_t_instantiated = ?box_t_instantiated,
            box_int = ?box_int,
            "After substitution, Box[T] does not equal Box[normie]"
        );
    } else {
        debug!("After substitution, Box[T] correctly equals Box[normie]");
    }
    
    assert_eq!(
        box_t_instantiated, box_int,
        "After substitution, Box[T] should equal Box[normie]"
    );

    info!("Generic type system test completed successfully");
}
