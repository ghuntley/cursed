use cursed::core::generic_instantiation::{GenericInstantiator, GenericTypeChecker};
use cursed::core::type_checker::Type;
use cursed::error::Error;
use cursed::ast::{Expression, Node};
use cursed::ast::expressions::{Identifier, StringLiteral};
use cursed::lexer::Token;

#[cfg(test)]
mod core_type_system_tests {

    // Helper struct to implement the GenericTypeChecker trait for testing
    struct TestTypeChecker;

    impl GenericTypeChecker for TestTypeChecker {
        fn check_generic_type(&self, generic_type: &Type, type_params: &[String]) -> Result<(), Error> {
            // Simple implementation for tests
            match generic_type {
                Type::TypeParam(name) => {
                    if type_params.contains(name) {
                        Ok(())
                    } else {
                        Err(Error::from_str(&format!("Unknown type parameter: {}", name)))
                    }
                }
                _ => Ok(()),
            }
        }

        fn check_generic_type_args(&self, generic_type: &Type, type_args: &[Type]) -> Result<(), Error> {
            // Simple implementation for tests
            match generic_type {
                Type::Struct(_, type_params) | Type::Interface(_, type_params) => {
                    if type_params.len() != type_args.len() {
                        Err(Error::from_str(&format!(
                            "Type argument count mismatch: expected {}, got {}",
                            type_params.len(),
                            type_args.len()
                        )))
                    } else {
                        Ok(())
                    }
                }
                _ => Ok(()),
            }
        }
    }

    #[test]
    fn test_expression_to_type_conversion() {
        // Test setup
        let instantiator = GenericInstantiator::new();
        
        // Create test expressions
        let ident = Box::new(Identifier {
            token: "token".to_string(),
            value: "normie".to_string(),
        }) as Box<dyn Expression>;
        
        let string_literal = Box::new(StringLiteral {
            token: "token".to_string(),
            value: "tea".to_string(),
        }) as Box<dyn Expression>;
        
        // Test the implementation that was a placeholder
        let result1 = instantiator.expression_to_type(ident.as_ref();
        let result2 = instantiator.expression_to_type(string_literal.as_ref();
        
        // Assert that we get proper types now instead of just Unknown
        assert!(result1.is_ok())
        assert_eq!(result1.unwrap(), Type::Normie);
        
        assert!(result2.is_ok())
        assert_eq!(result2.unwrap(), Type::Tea);
    }
    
    #[test]
    fn test_type_to_expression_conversion() {
        // Test setup
        let instantiator = GenericInstantiator::new();
        
        // Create test types
        let normie_type = Type::Normie;
        let tea_type = Type::Tea;
        let array_type = Type::Array(Box::new(Type::Normie), 5);
        
        // Test the implementation that was a placeholder
        let result1 = instantiator.type_to_expression(&normie_type);
        let result2 = instantiator.type_to_expression(&tea_type);
        let result3 = instantiator.type_to_expression(&array_type);
        
        // Assert that conversion works properly now
        assert!(result1.is_ok())
        let expr1 = result1.unwrap();
        assert_eq!(expr1, "IDENT");
        assert!(expr1.as_any().downcast_ref::<Identifier>().is_some())
        assert_eq!(
            expr1.as_any().downcast_ref::<Identifier>().unwrap().value,
            "normie"
        );
        
        assert!(result2.is_ok())
        let expr2 = result2.unwrap();
        assert_eq!(expr2, "IDENT");
        assert!(expr2.as_any().downcast_ref::<Identifier>().is_some())
        assert_eq!(
            expr2.as_any().downcast_ref::<Identifier>().unwrap().value, 
            "tea"
        );
        
        assert!(result3.is_ok())
        let expr3 = result3.unwrap();
        assert_eq!(expr3, "IDENT");
        assert!(expr3.as_any().downcast_ref::<Identifier>().is_some())
        assert_eq!(
            expr3.as_any().downcast_ref::<Identifier>().unwrap().value,
            "[5]normie"
        );
    }
}