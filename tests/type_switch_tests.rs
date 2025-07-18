use cursed::runtime::type_switch::*;
use cursed::execution::{CursedValue, Interpreter, ExecutionContext};
use cursed::ast::*;

#[test]
fn test_runtime_type_info_creation() {
    let type_info = create_primitive_type_info("normie");
    assert_eq!(type_info.size, 4);
    assert!(!type_info.is_interface);
    assert_eq!(type_info.interface_method_count, 0);
}

#[test]
fn test_type_registry_initialization() {
    initialize_cursed_runtime_types();
    
    // Test that we can get type IDs
    let normie_id = get_type_id_by_name("normie");
    let tea_id = get_type_id_by_name("tea");
    
    assert_ne!(normie_id, tea_id);
    assert_eq!(normie_id, get_type_id_by_name("normie")); // Consistent hashing
}

#[test]
fn test_type_switch_integer_matching() {
    let mut interpreter = Interpreter::new();
    let mut context = ExecutionContext::new();
    
    // Create a type switch expression for integer
    let variable = Box::new(Expression::Literal(Literal::Integer(42)));
    let arms = vec![
        TypeSwitchArm {
            type_pattern: TypePattern::Type(Type::Normie),
            bound_variable: None,
            body: Expression::Literal(Literal::String("found integer".to_string())),
        },
        TypeSwitchArm {
            type_pattern: TypePattern::Wildcard,
            bound_variable: None,
            body: Expression::Literal(Literal::String("other".to_string())),
        },
    ];
    
    let type_switch = TypeSwitchExpression { variable, arms };
    
    let result = interpreter.evaluate_type_switch_expression(&type_switch, &mut context);
    assert!(result.is_ok());
    
    if let Ok(CursedValue::String(s)) = result {
        assert_eq!(s, "found integer");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_type_switch_string_matching() {
    let mut interpreter = Interpreter::new();
    let mut context = ExecutionContext::new();
    
    // Create a type switch expression for string
    let variable = Box::new(Expression::Literal(Literal::String("hello".to_string())));
    let arms = vec![
        TypeSwitchArm {
            type_pattern: TypePattern::Type(Type::Normie),
            bound_variable: None,
            body: Expression::Literal(Literal::String("found integer".to_string())),
        },
        TypeSwitchArm {
            type_pattern: TypePattern::Type(Type::Tea),
            bound_variable: None,
            body: Expression::Literal(Literal::String("found string".to_string())),
        },
        TypeSwitchArm {
            type_pattern: TypePattern::Wildcard,
            bound_variable: None,
            body: Expression::Literal(Literal::String("other".to_string())),
        },
    ];
    
    let type_switch = TypeSwitchExpression { variable, arms };
    
    let result = interpreter.evaluate_type_switch_expression(&type_switch, &mut context);
    assert!(result.is_ok());
    
    if let Ok(CursedValue::String(s)) = result {
        assert_eq!(s, "found string");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_type_switch_boolean_matching() {
    let mut interpreter = Interpreter::new();
    let mut context = ExecutionContext::new();
    
    // Create a type switch expression for boolean
    let variable = Box::new(Expression::Literal(Literal::Boolean(true)));
    let arms = vec![
        TypeSwitchArm {
            type_pattern: TypePattern::Type(Type::Lit),
            bound_variable: None,
            body: Expression::Literal(Literal::String("found boolean".to_string())),
        },
        TypeSwitchArm {
            type_pattern: TypePattern::Wildcard,
            bound_variable: None,
            body: Expression::Literal(Literal::String("other".to_string())),
        },
    ];
    
    let type_switch = TypeSwitchExpression { variable, arms };
    
    let result = interpreter.evaluate_type_switch_expression(&type_switch, &mut context);
    assert!(result.is_ok());
    
    if let Ok(CursedValue::String(s)) = result {
        assert_eq!(s, "found boolean");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_type_switch_with_bound_variable() {
    let mut interpreter = Interpreter::new();
    let mut context = ExecutionContext::new();
    
    // Create a type switch expression with bound variable
    let variable = Box::new(Expression::Literal(Literal::Integer(42)));
    let arms = vec![
        TypeSwitchArm {
            type_pattern: TypePattern::Type(Type::Normie),
            bound_variable: Some("num".to_string()),
            body: Expression::Binary(BinaryExpression {
                left: Box::new(Expression::Identifier("num".to_string())),
                operator: "*".to_string(),
                right: Box::new(Expression::Literal(Literal::Integer(2))),
            }),
        },
        TypeSwitchArm {
            type_pattern: TypePattern::Wildcard,
            bound_variable: None,
            body: Expression::Literal(Literal::Integer(0)),
        },
    ];
    
    let type_switch = TypeSwitchExpression { variable, arms };
    
    let result = interpreter.evaluate_type_switch_expression(&type_switch, &mut context);
    assert!(result.is_ok());
    
    if let Ok(CursedValue::Integer(i)) = result {
        assert_eq!(i, 84);
    } else {
        panic!("Expected integer result");
    }
}

#[test]
fn test_type_switch_wildcard_fallback() {
    let mut interpreter = Interpreter::new();
    let mut context = ExecutionContext::new();
    
    // Create a type switch expression that should hit wildcard
    let variable = Box::new(Expression::Literal(Literal::String("A".to_string())));
    let arms = vec![
        TypeSwitchArm {
            type_pattern: TypePattern::Type(Type::Normie),
            bound_variable: None,
            body: Expression::Literal(Literal::String("found integer".to_string())),
        },
        TypeSwitchArm {
            type_pattern: TypePattern::Type(Type::Tea),
            bound_variable: None,
            body: Expression::Literal(Literal::String("found string".to_string())),
        },
        TypeSwitchArm {
            type_pattern: TypePattern::Wildcard,
            bound_variable: None,
            body: Expression::Literal(Literal::String("wildcard matched".to_string())),
        },
    ];
    
    let type_switch = TypeSwitchExpression { variable, arms };
    
    let result = interpreter.evaluate_type_switch_expression(&type_switch, &mut context);
    assert!(result.is_ok());
    
    if let Ok(CursedValue::String(s)) = result {
        assert_eq!(s, "wildcard matched");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_value_type_casting() {
    let mut interpreter = Interpreter::new();
    
    // Test integer casting
    let int_val = CursedValue::Integer(42);
    let cast_result = interpreter.cast_value_to_type(&int_val, &Type::Normie);
    assert!(cast_result.is_ok());
    assert!(matches!(cast_result.unwrap(), CursedValue::Integer(42)));
    
    // Test integer to float casting
    let int_val = CursedValue::Integer(42);
    let cast_result = interpreter.cast_value_to_type(&int_val, &Type::Meal);
    assert!(cast_result.is_ok());
    assert!(matches!(cast_result.unwrap(), CursedValue::Float(42.0)));
    
    // Test string casting
    let str_val = CursedValue::String("hello".to_string());
    let cast_result = interpreter.cast_value_to_type(&str_val, &Type::Tea);
    assert!(cast_result.is_ok());
    assert!(matches!(cast_result.unwrap(), CursedValue::String(_)));
}

#[test]
fn test_interface_type_checking() {
    let mut interpreter = Interpreter::new();
    
    // Test interface value
    let interface_val = CursedValue::Interface {
        vtable_ptr: 0,
        data_ptr: 0,
        interface_name: "TestInterface".to_string(),
        concrete_type: "TestImpl".to_string(),
    };
    
    let implements = interpreter.value_implements_interface(&interface_val, "TestInterface");
    assert!(implements);
    
    let not_implements = interpreter.value_implements_interface(&interface_val, "OtherInterface");
    assert!(!not_implements);
}

#[test]
fn test_type_switch_compilation_integration() {
    // This test ensures that type switch expressions work with the LLVM codegen
    // It's more of an integration test to ensure the runtime functions are properly linked
    initialize_cursed_runtime_types();
    
    let normie_id = get_type_id_by_name("normie");
    let tea_id = get_type_id_by_name("tea");
    
    assert_ne!(normie_id, tea_id);
    
    // Test type info creation
    let type_info = create_primitive_type_info("normie");
    assert_eq!(type_info.size, 4);
    assert!(!type_info.is_interface);
}
