//! Generic type instantiation for CURSED
//! 
//! This module provides complete generic type instantiation with constraint checking,
//! bounds verification, and integration with the monomorphisation pipeline.

use crate::error_types::Error as CursedError;
use crate::core::Type;
use crate::type_system::{TypeExpression, TypeEnvironment, GenericConstraint};
use crate::type_system::constraint_resolver::{ConstraintResolver, ConstraintSolution};
use crate::type_system::generics_bounds_checker::TypeBoundsChecker;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GenericInstantiator {
    type_parameters: HashMap<String, Type>,
    instantiations: Vec<TypeInstantiation>,
    /// Enhanced constraint resolver for bounds checking
    constraint_resolver: ConstraintResolver,
    /// Type parameter bounds checking
    bounds_checker: TypeBoundsChecker,
    /// Enable enhanced constraint checking
    enhanced_checking: bool,
}

#[derive(Debug, Clone)]
pub struct TypeInstantiation {
    pub generic_type: String,
    pub concrete_type: Type,
    pub context: String,
}

impl GenericInstantiator {
    pub fn new() -> Self {
        Self {
            type_parameters: HashMap::new(),
            instantiations: Vec::new(),
            constraint_resolver: ConstraintResolver::new(),
            bounds_checker: TypeBoundsChecker::new(),
            enhanced_checking: false,
        }
    }

    pub fn with_enhanced_checking(mut self) -> Self {
        self.enhanced_checking = true;
        self
    }

    pub fn add_type_parameter(&mut self, name: String, bound: Type) {
        self.type_parameters.insert(name, bound);
    }

    pub fn instantiate(&mut self, generic_type: String, concrete_type: Type, context: String) -> Result<Type, CursedError> {
        let instantiation = TypeInstantiation {
            generic_type: generic_type.clone(),
            concrete_type: concrete_type.clone(),
            context,
        };
        
        self.instantiations.push(instantiation);
        Ok(concrete_type)
    }

    pub fn get_instantiations(&self) -> &[TypeInstantiation] {
        &self.instantiations
    }

    /// Enhanced instantiation with complete constraint checking
    pub fn instantiate_with_constraints(&mut self,
                                        generic_name: &str,
                                        type_parameters: &[String],
                                        type_arguments: &[TypeExpression],
                                        constraints: &[GenericConstraint],
                                        env: &TypeEnvironment) -> Result<InstantiatedGeneric, CursedError> {
        // 1. Validate type parameter count
        if type_parameters.len() != type_arguments.len() {
            return Err(CursedError::TypeParameterMismatch {
                expected: type_parameters.len(),
                provided: type_arguments.len(),
                context: generic_name.to_string(),
            });
        }

        // 2. Check bounds for each type argument
        for (param, arg) in type_parameters.iter().zip(type_arguments.iter()) {
            self.bounds_checker.check_type_bounds(param, arg, constraints, env)?;
        }

        // 3. Resolve constraints with concrete types
        let constraint_solution = self.constraint_resolver
            .resolve_for_monomorphisation(type_parameters, type_arguments, constraints, env)
            .map_err(|e| CursedError::ConstraintResolutionError(format!("{:?}", e)))?;

        if !constraint_solution.is_satisfied {
            return Err(CursedError::ConstraintViolation(
                format!("Type constraints not satisfied for {}: {:?}", 
                       generic_name, constraint_solution.violations)
            ));
        }

        // 4. Create instantiated generic with resolved constraints
        let instance = InstantiatedGeneric {
            generic_name: generic_name.to_string(),
            type_parameters: type_parameters.to_vec(),
            type_arguments: type_arguments.to_vec(),
            constraints: constraints.to_vec(),
            constraint_solution,
            instance_id: self.generate_instance_id(generic_name, type_arguments),
        };

        // 5. Optional: Enhanced checking if enabled
        if self.enhanced_checking {
            // Additional validation can be added here
        }

        Ok(instance)
    }

    /// Check if a type parameter satisfies all its bounds
    pub fn verify_type_parameter_bounds(&self,
                                       param_name: &str,
                                       concrete_type: &TypeExpression,
                                       constraints: &[GenericConstraint],
                                       env: &TypeEnvironment) -> Result<bool, CursedError> {
        for constraint in constraints {
            if constraint.type_parameters.contains(&param_name.to_string()) {
                for bound in &constraint.bounds {
                    if !self.type_satisfies_bound(concrete_type, bound, env)? {
                        return Ok(false);
                    }
                }
            }
        }
        Ok(true)
    }

    /// Generate a unique instance identifier
    fn generate_instance_id(&self, generic_name: &str, type_args: &[TypeExpression]) -> String {
        let type_suffix = type_args.iter()
            .map(|t| t.name.as_deref().unwrap_or("unknown"))
            .collect::<Vec<_>>()
            .join("_");
        format!("{}_{}", generic_name, type_suffix)
    }

    /// Check if a concrete type satisfies a bound
    fn type_satisfies_bound(&self, concrete_type: &TypeExpression, bound: &str, env: &TypeEnvironment) -> Result<bool, CursedError> {
        // Delegate to bounds checker
        self.bounds_checker.check_bound_satisfaction(concrete_type, bound, env)
    }
}

/// Result of generic instantiation with constraint resolution
#[derive(Debug, Clone)]
pub struct InstantiatedGeneric {
    pub generic_name: String,
    pub type_parameters: Vec<String>,
    pub type_arguments: Vec<TypeExpression>,
    pub constraints: Vec<GenericConstraint>,
    pub constraint_solution: ConstraintSolution,
    pub instance_id: String,
}



impl Default for GenericInstantiator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_id_generation() {
        let type_env = TypeEnvironment::new();
        let instantiator = GenericInstantiator::new();
        
        let type_args = vec![
            TypeExpression::named("normie"),
            TypeExpression::named("tea"),
        ];
        
        let instance_id = instantiator.generate_instance_id("Container", &type_args);
        assert_eq!(instance_id, "Container_normie_tea");
    }

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
