//! Integration tests for the generics monomorphisation system

use crate::type_system::{TypeExpression, TypeEnvironment, GenericConstraint, GenericInstantiator, TypeBoundsChecker};
use crate::type_system::constraint_resolver::ConstraintResolver;
use crate::error_types::Error as CursedError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_generics_pipeline() {
        let type_env = TypeEnvironment::new();
        let mut instantiator = GenericInstantiator::new().with_enhanced_checking();
        
        // Test generic function instantiation: identity[T](x T) T
        let generic_name = "identity";
        let type_parameters = vec!["T".to_string()];
        let type_arguments = vec![TypeExpression::named("normie")];
        let constraints = vec![];
        
        let result = instantiator.instantiate_with_constraints(
            generic_name,
            &type_parameters,
            &type_arguments,
            &constraints,
            &type_env
        );
        
        assert!(result.is_ok());
        let instance = result.unwrap();
        assert_eq!(instance.generic_name, "identity");
        assert_eq!(instance.type_arguments[0].name, Some("normie".to_string()));
        assert_eq!(instance.instance_id, "identity_normie");
    }

    #[test]
    fn test_constraint_violation() {
        let type_env = TypeEnvironment::new();
        let bounds_checker = TypeBoundsChecker::new();
        
        // Test that string type violates numeric constraint
        let string_type = TypeExpression::named("tea");
        let result = bounds_checker.check_bound_satisfaction(&string_type, "numeric", &type_env);
        
        assert!(result.is_ok());
        assert!(!result.unwrap()); // String should not satisfy numeric bound
    }

    #[test]
    fn test_comparable_constraint_satisfied() {
        let type_env = TypeEnvironment::new();
        let bounds_checker = TypeBoundsChecker::new();
        
        // Test that numeric types satisfy comparable constraint
        let int_type = TypeExpression::named("normie");
        let result = bounds_checker.check_bound_satisfaction(&int_type, "comparable", &type_env);
        
        assert!(result.is_ok());
        assert!(result.unwrap()); // Integer should satisfy comparable bound
    }

    #[test]
    fn test_multiple_type_parameters() {
        let type_env = TypeEnvironment::new();
        let mut instantiator = GenericInstantiator::new();
        
        // Test generic function with multiple type parameters: map[T, U](input T, fn stan(T) U) U
        let generic_name = "map";
        let type_parameters = vec!["T".to_string(), "U".to_string()];
        let type_arguments = vec![
            TypeExpression::named("normie"),
            TypeExpression::named("tea"),
        ];
        let constraints = vec![];
        
        let result = instantiator.instantiate_with_constraints(
            generic_name,
            &type_parameters,
            &type_arguments,
            &constraints,
            &type_env
        );
        
        assert!(result.is_ok());
        let instance = result.unwrap();
        assert_eq!(instance.generic_name, "map");
        assert_eq!(instance.type_arguments.len(), 2);
        assert_eq!(instance.instance_id, "map_normie_tea");
    }

    #[test]
    fn test_constraint_resolver_integration() {
        let type_env = TypeEnvironment::new();
        let mut resolver = ConstraintResolver::new();
        
        // Test constraint resolution for generic types
        let type_parameters = vec!["T".to_string()];
        let type_arguments = vec![TypeExpression::named("normie")];
        let constraints = vec![GenericConstraint {
            constraint_name: "numeric_constraint".to_string(),
            type_parameters: vec!["T".to_string()],
            bounds: vec!["numeric".to_string()],
        }];
        
        let result = resolver.resolve_for_monomorphisation(
            &type_parameters,
            &type_arguments,
            &constraints,
            &type_env
        );
        
        assert!(result.is_ok());
        let solution = result.unwrap();
        assert!(solution.is_satisfied);
        assert!(solution.substitutions.contains_key("T"));
    }

    #[test]
    fn test_bounds_checking_with_constraints() {
        let type_env = TypeEnvironment::new();
        let bounds_checker = TypeBoundsChecker::new();
        
        // Test type bounds checking with generic constraints
        let constraints = vec![GenericConstraint {
            constraint_name: "ordered_constraint".to_string(),
            type_parameters: vec!["T".to_string()],
            bounds: vec!["ordered".to_string()],
        }];
        
        // String should satisfy ordered constraint
        let string_type = TypeExpression::named("tea");
        let result = bounds_checker.check_type_bounds(
            "T",
            &string_type,
            &constraints,
            &type_env
        );
        
        assert!(result.is_ok());
        
        // Function type should NOT satisfy ordered constraint
        let func_type = TypeExpression::named("function");
        let result = bounds_checker.check_type_bounds(
            "T",
            &func_type,
            &constraints,
            &type_env
        );
        
        // This should fail with a BoundViolation error
        assert!(result.is_err());
        match result.err().unwrap() {
            CursedError::BoundViolation { .. } => {}, // Expected
            _ => panic!("Expected BoundViolation error"),
        }
    }

    #[test]
    fn test_type_parameter_mismatch() {
        let type_env = TypeEnvironment::new();
        let mut instantiator = GenericInstantiator::new();
        
        // Test mismatch between type parameters and arguments
        let generic_name = "container";
        let type_parameters = vec!["T".to_string(), "U".to_string()]; // 2 parameters
        let type_arguments = vec![TypeExpression::named("normie")]; // 1 argument
        let constraints = vec![];
        
        let result = instantiator.instantiate_with_constraints(
            generic_name,
            &type_parameters,
            &type_arguments,
            &constraints,
            &type_env
        );
        
        assert!(result.is_err());
        match result.err().unwrap() {
            CursedError::TypeParameterMismatch { expected: 2, provided: 1, .. } => {}, // Expected
            _ => panic!("Expected TypeParameterMismatch error"),
        }
    }
}
