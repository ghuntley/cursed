//! Tests for the CURSED type system

use super::*;
use crate::ast::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_expression_creation() {
        let int_type = TypeExpression::named("int");
        assert_eq!(int_type.name, Some("int".to_string()));
        assert_eq!(int_type.kind, TypeKind::Primitive);
        
        let func_type = TypeExpression::function(
            vec![TypeExpression::named("int"), TypeExpression::named("string")],
            TypeExpression::named("bool")
        );
        assert_eq!(func_type.kind, TypeKind::Function);
        assert_eq!(func_type.parameters.len(), 2);
        assert!(func_type.return_type.is_some());
    }

    #[test]
    fn test_type_substitution() {
        let mut subst = TypeSubstitution::new();
        subst.add("T".to_string(), TypeExpression::named("int"));
        
        let type_var = TypeExpression::named("T");
        let result = subst.apply(&type_var);
        assert_eq!(result.name, Some("int".to_string()));
    }

    #[test]
    fn test_type_unification() {
        let mut subst = TypeSubstitution::new();
        let t1 = TypeExpression::named("int");
        let t2 = TypeExpression::named("int");
        
        assert!(subst.unify(&t1, &t2).is_ok());
        
        let t3 = TypeExpression::named("T0");
        let t4 = TypeExpression::named("int");
        assert!(subst.unify(&t3, &t4).is_ok());
        assert_eq!(subst.mappings.get("T0").unwrap().name, Some("int".to_string()));
    }

    #[test]
    fn test_type_system_initialization() {
        let type_system = TypeSystem::new();
        
        // Check that built-in types are available
        assert!(type_system.environment.get_type("int").is_some());
        assert!(type_system.environment.get_type("string").is_some());
        assert!(type_system.environment.get_type("bool").is_some());
        assert!(type_system.environment.get_type("void").is_some());
        
        // Check that vibez object is available
        assert!(type_system.environment.get_type("vibez").is_some());
        let vibez_type = type_system.environment.get_type("vibez").unwrap();
        assert!(!vibez_type.methods.is_empty());
        assert_eq!(vibez_type.methods[0].name, "spill");
    }

    #[test]
    fn test_expression_type_checking() {
        let mut type_system = TypeSystem::new();
        
        // Test integer literal
        let int_expr = Expression::Integer(42);
        let result = type_system.check_expression(&int_expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, Some("normie".to_string()));
        
        // Test string literal
        let string_expr = Expression::String("hello".to_string());
        let result = type_system.check_expression(&string_expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, Some("tea".to_string()));
        
        // Test boolean literal
        let bool_expr = Expression::Boolean(true);
        let result = type_system.check_expression(&bool_expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, Some("vibes".to_string()));
    }

    #[test]
    fn test_member_access_type_checking() {
        let mut type_system = TypeSystem::new();
        
        // Test vibez.spill access
        let member_access = Expression::MemberAccess(MemberAccessExpression {
            object: Box::new(Expression::Identifier("vibez".to_string())),
            property: "spill".to_string(),
        });
        
        let result = type_system.check_expression(&member_access);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, Some("cap".to_string()));
    }

    #[test]
    fn test_function_call_type_checking() {
        let mut type_system = TypeSystem::new();
        
        // Test vibez.spill("test") call
        let call_expr = Expression::Call(CallExpression {
            function: Box::new(Expression::MemberAccess(MemberAccessExpression {
                object: Box::new(Expression::Identifier("vibez".to_string())),
                property: "spill".to_string(),
            })),
            arguments: vec![Expression::String("test".to_string())],
        });
        
        let result = type_system.check_expression(&call_expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, Some("cap".to_string()));
    }

    #[test]
    fn test_binary_operation_type_checking() {
        let mut type_system = TypeSystem::new();
        
        // Test 1 + 2
        let binary_expr = Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Integer(1)),
            operator: "+".to_string(),
            right: Box::new(Expression::Integer(2)),
        });
        
        let result = type_system.check_expression(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, Some("normie".to_string()));
        
        // Test 1 == 2
        let comparison_expr = Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Integer(1)),
            operator: "==".to_string(),
            right: Box::new(Expression::Integer(2)),
        });
        
        let result = type_system.check_expression(&comparison_expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, Some("vibes".to_string()));
    }

    #[test]
    fn test_constraint_resolver() {
        let resolver = ConstraintResolver::new();
        let env = TypeEnvironment::new();
        
        let constraint = GenericConstraint {
            constraint_name: "Display".to_string(),
            type_parameters: vec!["T".to_string()],
            bounds: vec![],
        };
        
        assert!(resolver.validate_constraint(&constraint, &env).is_ok());
    }

    #[test]
    fn test_type_unifier() {
        let mut unifier = TypeUnifier::new();
        
        let t1 = TypeExpression::named("int");
        let t2 = TypeExpression::named("int");
        
        let result = unifier.unify(&t1, &t2);
        assert!(result.is_ok());
        
        let t3 = TypeExpression::named("T0");
        let t4 = TypeExpression::named("string");
        
        let result = unifier.unify(&t3, &t4);
        assert!(result.is_ok());
        let substitutions = result.unwrap();
        assert!(substitutions.contains_key("T0"));
    }

    #[test]
    fn test_constraint_graph() {
        let propagator = ConstraintPropagator::new();
        let bindings = vec![
            ConstraintBinding {
                constraint: GenericConstraint {
                    constraint_name: "Display".to_string(),
                    type_parameters: vec!["T".to_string()],
                    bounds: vec![],
                },
                bound_types: vec!["int".to_string()],
                satisfaction_status: ConstraintStatus::Pending,
            }
        ];
        
        let result = propagator.build_constraint_graph(&bindings);
        assert!(result.is_ok());
        
        let graph = result.unwrap();
        assert_eq!(graph.nodes.len(), 1);
    }

    #[test]
    fn test_instantiated_type() {
        let base_type = TypeExpression::generic("Array", vec![TypeExpression::named("T")]);
        let type_args = vec![TypeExpression::named("int")];
        
        let instantiated = InstantiatedType::new(base_type, type_args);
        let result = instantiated.instantiate();
        
        assert!(result.name.is_some());
    }

    #[test]
    fn test_inference_context() {
        let mut context = InferenceContext::new();
        
        let fresh_var = context.fresh_type_var();
        assert!(fresh_var.name.is_some());
        assert!(fresh_var.name.unwrap().starts_with("T"));
        
        context.bind_type_var("T0", TypeExpression::named("int"));
        let resolved = context.resolve_type(&TypeExpression::named("T0"));
        assert_eq!(resolved.name, Some("int".to_string()));
    }

    #[test]
    fn test_error_cases() {
        let mut type_system = TypeSystem::new();
        
        // Test unknown identifier
        let unknown_expr = Expression::Identifier("unknown".to_string());
        let result = type_system.check_expression(&unknown_expr);
        assert!(result.is_err());
        
        // Test incompatible binary operation
        let bad_binary = Expression::Binary(BinaryExpression {
            left: Box::new(Expression::String("hello".to_string())),
            operator: "+".to_string(),
            right: Box::new(Expression::Integer(42)),
        });
        
        let result = type_system.check_expression(&bad_binary);
        assert!(result.is_err());
    }
}
