//! Generic type parameter bounds checking for CURSED
//! 
//! This module provides bounds checking for generic type parameters

use crate::error_types::Error as CursedError;
use crate::type_system::{TypeExpression, TypeEnvironment, GenericConstraint};
use std::collections::HashMap;

/// Type bounds checker for generic type parameters
#[derive(Debug)]
pub struct TypeBoundsChecker {
    /// Built-in bounds and their validation rules
    builtin_bounds: HashMap<String, fn(&TypeExpression) -> bool>,
}

impl TypeBoundsChecker {
    pub fn new() -> Self {
        let mut checker = Self {
            builtin_bounds: HashMap::new(),
        };
        
        // Register built-in bounds
        checker.builtin_bounds.insert("any".to_string(), Self::validate_any_bound);
        checker.builtin_bounds.insert("comparable".to_string(), Self::validate_comparable_bound);
        checker.builtin_bounds.insert("numeric".to_string(), Self::validate_numeric_bound);
        checker.builtin_bounds.insert("ordered".to_string(), Self::validate_ordered_bound);
        
        checker
    }

    /// Check if a type satisfies the required bounds
    pub fn check_type_bounds(&self,
                            param_name: &str,
                            concrete_type: &TypeExpression,
                            constraints: &[GenericConstraint],
                            env: &TypeEnvironment) -> Result<(), CursedError> {
        for constraint in constraints {
            if constraint.type_parameters.contains(&param_name.to_string()) {
                for bound in &constraint.bounds {
                    if !self.check_bound_satisfaction(concrete_type, bound, env)? {
                        return Err(CursedError::BoundViolation {
                            type_param: param_name.to_string(),
                            concrete_type: concrete_type.name.clone().unwrap_or_default(),
                            bound: bound.clone(),
                            reason: format!("Type '{}' does not satisfy bound '{}'", 
                                          concrete_type.name.as_ref().unwrap_or(&"unknown".to_string()), 
                                          bound),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    /// Check if a concrete type satisfies a specific bound
    pub fn check_bound_satisfaction(&self,
                                   concrete_type: &TypeExpression,
                                   bound: &str,
                                   _env: &TypeEnvironment) -> Result<bool, CursedError> {
        // Check built-in bounds first
        if let Some(validator) = self.builtin_bounds.get(bound) {
            return Ok(validator(concrete_type));
        }

        // For now, return false for unknown bounds
        Ok(false)
    }

    // Built-in bound validators

    fn validate_any_bound(_type_expr: &TypeExpression) -> bool {
        true // All types satisfy 'any'
    }

    fn validate_comparable_bound(type_expr: &TypeExpression) -> bool {
        matches!(type_expr.name.as_deref(),
                Some("normie") | Some("smol") | Some("mid") | Some("thicc") |
                Some("drip") | Some("meal") | Some("snack") |
                Some("lit") | Some("tea") | Some("sip"))
    }

    fn validate_numeric_bound(type_expr: &TypeExpression) -> bool {
        matches!(type_expr.name.as_deref(),
                Some("normie") | Some("smol") | Some("mid") | Some("thicc") |
                Some("drip") | Some("meal") | Some("snack"))
    }

    fn validate_ordered_bound(type_expr: &TypeExpression) -> bool {
        Self::validate_numeric_bound(type_expr) ||
        matches!(type_expr.name.as_deref(), Some("tea") | Some("sip"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds_checker_comparable() {
        let bounds_checker = TypeBoundsChecker::new();
        let env = TypeEnvironment::new();
        
        let int_type = TypeExpression::named("normie");
        assert!(bounds_checker.check_bound_satisfaction(&int_type, "comparable", &env).unwrap());
        
        let string_type = TypeExpression::named("tea");
        assert!(bounds_checker.check_bound_satisfaction(&string_type, "comparable", &env).unwrap());
    }

    #[test]
    fn test_bounds_checker_numeric() {
        let bounds_checker = TypeBoundsChecker::new();
        let env = TypeEnvironment::new();
        
        let int_type = TypeExpression::named("normie");
        assert!(bounds_checker.check_bound_satisfaction(&int_type, "numeric", &env).unwrap());
        
        let string_type = TypeExpression::named("tea");
        assert!(!bounds_checker.check_bound_satisfaction(&string_type, "numeric", &env).unwrap());
    }
}
