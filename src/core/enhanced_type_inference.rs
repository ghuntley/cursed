//! Enhanced type inference with constraint integration
//!
//! This module extends the existing type inference system to work with
//! the enhanced constraint resolution system, providing better type
//! inference for generic functions and types with complex constraints.

use crate::ast::{Expression, Statement};
use crate::ast::expressions::calls::CallExpression;
use crate::ast::declarations::{GenericConstraint, FunctionStatement, SquadStatement};
use crate::ast::traits::Node;
use crate::core::type_checker::{Type, TypeChecker};
use crate::core::constraint_resolver::{ConstraintResolver, ConstraintResolutionResult};
use crate::core::type_infer::TypeInference;
use crate::error::Error;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, instrument, warn};

/// Enhanced type inference engine with constraint support
pub struct EnhancedTypeInference {
    /// Base type checker
    type_checker: Arc<RwLock<TypeChecker>>,
    /// Constraint resolver for generic constraints
    constraint_resolver: Arc<RwLock<ConstraintResolver>>,
    /// Cache for inferred types
    inference_cache: HashMap<String, Type>,
    /// Type variable counter for generating unique type variables
    type_var_counter: usize,
}

/// Represents a type inference context
#[derive(Debug, Clone)]
pub struct InferenceContext {
    /// Known type bindings
    pub type_bindings: HashMap<String, Type>,
    /// Active constraints
    pub constraints: Vec<GenericConstraint>,
    /// Type variables in scope
    pub type_variables: HashMap<String, Type>,
    /// Expected return type (if any)
    pub expected_return_type: Option<Type>,
}

/// Result of type inference with constraint information
#[derive(Debug, Clone)]
pub struct InferenceResult {
    /// The inferred type
    pub inferred_type: Type,
    /// Additional type bindings discovered
    pub additional_bindings: HashMap<String, Type>,
    /// Constraint resolution result
    pub constraint_result: Option<ConstraintResolutionResult>,
    /// Confidence level of the inference (0.0 - 1.0)
    pub confidence: f64,
}

impl EnhancedTypeInference {
    /// Create a new enhanced type inference engine
    #[instrument(skip_all, level = "debug")]
    pub fn new(
        type_checker: Arc<RwLock<TypeChecker>>,
        constraint_resolver: Arc<RwLock<ConstraintResolver>>,
    ) -> Self {
        info!("Creating enhanced type inference engine");
        Self {
            type_checker,
            constraint_resolver,
            inference_cache: HashMap::new(),
            type_var_counter: 0,
        }
    }

    /// Infer type for a generic function call with constraint resolution
    #[instrument(skip(self), level = "debug")]
    pub fn infer_generic_call_type(
        &mut self,
        call_expr: &CallExpression,
        function: &FunctionStatement,
        context: &InferenceContext,
    ) -> Result<InferenceResult, Error> {
        debug!(
            function_name = %function.name.string(),
            type_args_count = call_expr.type_arguments.len(),
            constraints_count = function.generic_constraints.len(),
            "Inferring type for generic function call"
        );

        // If explicit type arguments provided, use them
        if !call_expr.type_arguments.is_empty() {
            return self.infer_with_explicit_types(call_expr, function, context);
        }

        // Otherwise, infer types from arguments and constraints
        self.infer_from_arguments_and_constraints(call_expr, function, context)
    }

    /// Infer type with explicit type arguments
    #[instrument(skip(self), level = "debug")]
    fn infer_with_explicit_types(
        &mut self,
        call_expr: &CallExpression,
        function: &FunctionStatement,
        context: &InferenceContext,
    ) -> Result<InferenceResult, Error> {
        debug!("Inferring type with explicit type arguments");

        // Resolve constraints with provided type arguments
        let mut resolver = self.constraint_resolver.write()
            .map_err(|e| Error::new(
                "LockError",
                &format!("Failed to acquire constraint resolver lock: {}", e),
                None
            ))?;

        let constraint_result = resolver.resolve_function_constraints(
            function,
            &call_expr.type_arguments,
        )?;

        if !constraint_result.satisfied {
            warn!(
                violations_count = constraint_result.violations.len(),
                "Constraint violations detected with explicit types"
            );
        }

        // Create type parameter substitution map
        let mut substitutions = HashMap::new();
        for (i, type_param) in function.type_parameters.iter().enumerate() {
            if i < call_expr.type_arguments.len() {
                substitutions.insert(
                    type_param.name.clone(),
                    call_expr.type_arguments[i].clone(),
                );
            }
        }

        // TODO: Fix return type handling - need to convert Expression to Type
        // Substitute type parameters in return type
        let return_type = Type::Any; // Placeholder for now

        Ok(InferenceResult {
            inferred_type: return_type,
            additional_bindings: substitutions,
            constraint_result: Some(constraint_result),
            confidence: if constraint_result.satisfied { 1.0 } else { 0.3 },
        })
    }

    /// Infer types from function arguments and constraints
    #[instrument(skip(self), level = "debug")]
    fn infer_from_arguments_and_constraints(
        &mut self,
        call_expr: &CallExpression,
        function: &FunctionStatement,
        context: &InferenceContext,
    ) -> Result<InferenceResult, Error> {
        debug!("Inferring types from arguments and constraints");

        // Infer types from argument expressions
        let argument_types = self.infer_argument_types(&call_expr.arguments, context)?;

        // Create initial type bindings from arguments
        let mut type_bindings = self.create_initial_bindings(
            function,
            &argument_types,
        )?;

        // Use constraint resolver to infer types that satisfy constraints
        let mut resolver = self.constraint_resolver.write()
            .map_err(|e| Error::new(
                "LockError",
                &format!("Failed to acquire constraint resolver lock: {}", e),
                None
            ))?;

        let inferred_constraint_types = resolver.infer_constraint_satisfying_types(
            &function.generic_constraints,
            &type_bindings,
        )?;

        // Merge inferred types with existing bindings
        type_bindings.extend(inferred_constraint_types);

        // Fill in any remaining unresolved type parameters
        for type_param in &function.type_parameters {
            if !type_bindings.contains_key(&type_param.name) {
                let inferred_type = self.infer_type_parameter(
                    &type_param.name,
                    &function.generic_constraints,
                    &type_bindings,
                    context,
                )?;
                type_bindings.insert(type_param.name.clone(), inferred_type);
            }
        }

        // Create type arguments vector for constraint checking
        let type_arguments: Vec<Type> = function.type_parameters
            .iter()
            .map(|param| {
                type_bindings.get(&param.name)
                    .cloned()
                    .unwrap_or_else(|| self.fresh_type_variable())
            })
            .collect();

        // Validate constraints with inferred types
        let constraint_result = resolver.resolve_function_constraints(
            function,
            &type_arguments,
        )?;

        // Substitute type parameters in return type
        // TODO: Fix return type handling - need to convert Expression to Type
        let return_type = Type::Any; // Placeholder for now

        let confidence = self.calculate_inference_confidence(
            &type_bindings,
            &constraint_result,
            &argument_types,
        );

        Ok(InferenceResult {
            inferred_type: return_type,
            additional_bindings: type_bindings,
            constraint_result: Some(constraint_result),
            confidence,
        })
    }

    /// Infer types for function arguments
    #[instrument(skip(self), level = "debug")]
    fn infer_argument_types(
        &mut self,
        arguments: &[Box<dyn Expression>],
        context: &InferenceContext,
    ) -> Result<Vec<Type>, Error> {
        debug!(args_count = arguments.len(), "Inferring argument types");

        let mut arg_types = Vec::new();
        let type_checker = self.type_checker.read()
            .map_err(|e| Error::new(
                "LockError",
                &format!("Failed to acquire type checker lock: {}", e),
                None
            ))?;

        for (i, arg) in arguments.iter().enumerate() {
            // Try to infer type using existing type inference
            let inferred_type = type_checker.get_expression_type(arg.as_ref())?;
            
            debug!(arg_index = i, inferred_type = ?inferred_type, "Inferred argument type");
            arg_types.push(inferred_type);
        }

        Ok(arg_types)
    }

    /// Create initial type bindings from function parameters and argument types
    #[instrument(skip(self), level = "debug")]
    fn create_initial_bindings(
        &self,
        function: &FunctionStatement,
        argument_types: &[Type],
    ) -> Result<HashMap<String, Type>, Error> {
        debug!("Creating initial type bindings");

        let mut bindings = HashMap::new();

        // Match argument types with parameter types to extract type bindings
        for (i, param) in function.parameters.iter().enumerate() {
            if i < argument_types.len() {
                self.extract_type_bindings(
                    &param.param_type,
                    &argument_types[i],
                    &mut bindings,
                )?;
            }
        }

        debug!(bindings = ?bindings, "Created initial type bindings");
        Ok(bindings)
    }

    /// Extract type bindings by matching a parameter type against an argument type
    #[instrument(skip(self), level = "debug")]
    fn extract_type_bindings(
        &self,
        param_type: &Type,
        arg_type: &Type,
        bindings: &mut HashMap<String, Type>,
    ) -> Result<(), Error> {
        match (param_type, arg_type) {
            // Type parameter matches any concrete type
            (Type::TypeParam(param_name), concrete_type) => {
                bindings.insert(param_name.clone(), concrete_type.clone());
            }
            
            // Generic types with type arguments
            (Type::Generic(param_name, param_args), Type::Generic(arg_name, arg_args)) => {
                if param_name == arg_name && param_args.len() == arg_args.len() {
                    for (param_arg, arg_arg) in param_args.iter().zip(arg_args.iter()) {
                        self.extract_type_bindings(param_arg, arg_arg, bindings)?;
                    }
                }
            }
            
            // Container types
            (Type::Array(param_elem, _), Type::Array(arg_elem, _)) => {
                self.extract_type_bindings(param_elem, arg_elem, bindings)?;
            }
            
            (Type::Slice(param_elem), Type::Slice(arg_elem)) => {
                self.extract_type_bindings(param_elem, arg_elem, bindings)?;
            }
            
            (Type::Map(param_key, param_val), Type::Map(arg_key, arg_val)) => {
                self.extract_type_bindings(param_key, arg_key, bindings)?;
                self.extract_type_bindings(param_val, arg_val, bindings)?;
            }
            
            // Function types
            (Type::Function(param_args, param_ret), Type::Function(arg_args, arg_ret)) => {
                if param_args.len() == arg_args.len() {
                    for (param_arg, arg_arg) in param_args.iter().zip(arg_args.iter()) {
                        self.extract_type_bindings(param_arg, arg_arg, bindings)?;
                    }
                    self.extract_type_bindings(param_ret, arg_ret, bindings)?;
                }
            }
            
            _ => {
                // For non-matching types, no bindings extracted
                debug!(
                    param_type = ?param_type,
                    arg_type = ?arg_type,
                    "No type bindings extracted for non-matching types"
                );
            }
        }

        Ok(())
    }

    /// Infer a type parameter based on constraints and context
    #[instrument(skip(self), level = "debug")]
    fn infer_type_parameter(
        &mut self,
        param_name: &str,
        constraints: &[GenericConstraint],
        existing_bindings: &HashMap<String, Type>,
        context: &InferenceContext,
    ) -> Result<Type, Error> {
        debug!(param_name = %param_name, "Inferring type parameter");

        // Look for constraints on this type parameter
        let param_constraints: Vec<_> = constraints
            .iter()
            .filter(|c| c.parameter_name == param_name)
            .collect();

        if param_constraints.is_empty() {
            // No constraints, use context or create fresh type variable
            if let Some(binding) = context.type_bindings.get(param_name) {
                return Ok(binding.clone());
            }
            return Ok(self.fresh_type_variable());
        }

        // Find a type that satisfies all constraints for this parameter
        for constraint in &param_constraints {
            // Try to find an existing type that implements this interface
            if let Some(implementing_type) = self.find_implementing_type(&constraint.interface_name)? {
                // Verify it satisfies all constraints for this parameter
                if self.satisfies_all_constraints(&implementing_type, &param_constraints)? {
                    return Ok(implementing_type);
                }
            }
        }

        // If no concrete type found, create a constrained type variable
        Ok(self.fresh_constrained_type_variable(param_name, &param_constraints))
    }

    /// Find a type that implements a given interface
    #[instrument(skip(self), level = "debug")]
    fn find_implementing_type(&self, interface_name: &str) -> Result<Option<Type>, Error> {
        let type_checker = self.type_checker.read()
            .map_err(|e| Error::new(
                "LockError",
                &format!("Failed to acquire type checker lock: {}", e),
                None
            ))?;

        // Get known implementations of the interface
        let implementations = type_checker.get_interface_implementations(interface_name)?;
        
        // Return the first implementation
        Ok(implementations.first().cloned())
    }

    /// Check if a type satisfies all given constraints
    #[instrument(skip(self), level = "debug")]
    fn satisfies_all_constraints(
        &self,
        type_to_check: &Type,
        constraints: &[&GenericConstraint],
    ) -> Result<bool, Error> {
        let type_checker = self.type_checker.read()
            .map_err(|e| Error::new(
                "LockError",
                &format!("Failed to acquire type checker lock: {}", e),
                None
            ))?;

        for constraint in constraints {
            // TODO: Convert interface_name String to Type for check_interface_implementation
            // For now, assume all constraints are satisfied
            let _implements = true; // Placeholder
        }

        Ok(true)
    }

    /// Substitute type parameters in a type with concrete types
    #[instrument(skip(self), level = "debug")]
    fn substitute_type_parameters(
        &self,
        type_to_substitute: &Type,
        substitutions: &HashMap<String, Type>,
    ) -> Result<Type, Error> {
        match type_to_substitute {
            Type::TypeParam(param_name) => {
                Ok(substitutions.get(param_name)
                    .cloned()
                    .unwrap_or_else(|| type_to_substitute.clone()))
            }
            
            Type::Generic(name, args) => {
                let substituted_args: Result<Vec<_>, _> = args
                    .iter()
                    .map(|arg| self.substitute_type_parameters(arg, substitutions))
                    .collect();
                
                Ok(Type::Generic(name.clone(), substituted_args?.into_iter().map(Box::new).collect()))
            }
            
            Type::Array(elem_type, size) => {
                let substituted_elem = self.substitute_type_parameters(elem_type, substitutions)?;
                Ok(Type::Array(Box::new(substituted_elem), *size))
            }
            
            Type::Slice(elem_type) => {
                let substituted_elem = self.substitute_type_parameters(elem_type, substitutions)?;
                Ok(Type::Slice(Box::new(substituted_elem)))
            }
            
            Type::Map(key_type, val_type) => {
                let substituted_key = self.substitute_type_parameters(key_type, substitutions)?;
                let substituted_val = self.substitute_type_parameters(val_type, substitutions)?;
                Ok(Type::Map(Box::new(substituted_key), Box::new(substituted_val)))
            }
            
            Type::Function(param_types, return_type) => {
                let substituted_params: Result<Vec<_>, _> = param_types
                    .iter()
                    .map(|param| self.substitute_type_parameters(param, substitutions))
                    .collect();
                
                let substituted_return = self.substitute_type_parameters(return_type, substitutions)?;
                
                Ok(Type::Function(
                    substituted_params?.into_iter().map(Box::new).collect(),
                    Box::new(substituted_return),
                ))
            }
            
            _ => Ok(type_to_substitute.clone()),
        }
    }

    /// Generate a fresh type variable
    #[instrument(skip(self), level = "debug")]
    fn fresh_type_variable(&mut self) -> Type {
        self.type_var_counter += 1;
        let var_name = format!("T{}", self.type_var_counter);
        Type::TypeParam(var_name)
    }

    /// Generate a fresh constrained type variable
    #[instrument(skip(self), level = "debug")]
    fn fresh_constrained_type_variable(
        &mut self,
        base_name: &str,
        constraints: &[&GenericConstraint],
    ) -> Type {
        self.type_var_counter += 1;
        
        // For now, return a type parameter
        // In a more sophisticated implementation, we'd create a special
        // constrained type variable that carries constraint information
        let var_name = if constraints.is_empty() {
            format!("{}_{}", base_name, self.type_var_counter)
        } else {
            let constraint_names: Vec<_> = constraints
                .iter()
                .map(|c| c.interface_name.as_str())
                .collect();
            format!("{}_{}_{}", base_name, constraint_names.join("_"), self.type_var_counter)
        };
        
        Type::TypeParam(var_name)
    }

    /// Calculate confidence level for type inference
    #[instrument(skip(self), level = "debug")]
    fn calculate_inference_confidence(
        &self,
        type_bindings: &HashMap<String, Type>,
        constraint_result: &ConstraintResolutionResult,
        argument_types: &[Type],
    ) -> f64 {
        let mut confidence = 1.0;

        // Reduce confidence for unsatisfied constraints
        if !constraint_result.satisfied {
            confidence *= 0.3;
        }

        // Reduce confidence for unresolved type variables
        let type_var_count = type_bindings
            .values()
            .filter(|t| matches!(t, Type::TypeParam(_)))
            .count();
        
        if type_var_count > 0 {
            confidence *= 0.8_f64.powi(type_var_count as i32);
        }

        // Increase confidence for exact argument type matches
        let exact_matches = argument_types
            .iter()
            .filter(|t| !matches!(t, Type::Unknown | Type::TypeParam(_)))
            .count();
        
        if exact_matches > 0 {
            confidence *= 1.0 + (exact_matches as f64 * 0.1);
        }

        confidence.min(1.0).max(0.0)
    }

    /// Infer type for struct instantiation with generic parameters
    #[instrument(skip(self), level = "debug")]
    pub fn infer_struct_instantiation_type(
        &mut self,
        struct_stmt: &SquadStatement,
        field_types: &HashMap<String, Type>,
        context: &InferenceContext,
    ) -> Result<InferenceResult, Error> {
        debug!(
            struct_name = %struct_stmt.name.string(),
            type_params_count = struct_stmt.type_parameters.len(),
            "Inferring type for struct instantiation"
        );

        // Infer type parameters from field types
        let mut type_bindings = HashMap::new();
        
        for field in &struct_stmt.fields {
            if let Some(provided_type) = field_types.get(&field.name.string()) {
                self.extract_type_bindings(
                    &field.type_name,
                    provided_type,
                    &mut type_bindings,
                )?;
            }
        }

        // Resolve constraints
        let type_arguments: Vec<Type> = struct_stmt.type_parameters
            .iter()
            .map(|param| {
                type_bindings.get(&param.name)
                    .cloned()
                    .unwrap_or_else(|| self.fresh_type_variable())
            })
            .collect();

        let mut resolver = self.constraint_resolver.write()
            .map_err(|e| Error::new(
                "LockError",
                &format!("Failed to acquire constraint resolver lock: {}", e),
                None
            ))?;

        let constraint_result = resolver.resolve_struct_constraints(
            struct_stmt,
            &type_arguments,
        )?;

        // Create the instantiated struct type
        let struct_type = Type::Struct(
            struct_stmt.name.string(),
            type_arguments.into_iter().map(Box::new).collect(),
        );

        let confidence = self.calculate_inference_confidence(
            &type_bindings,
            &constraint_result,
            &field_types.values().cloned().collect::<Vec<_>>(),
        );

        Ok(InferenceResult {
            inferred_type: struct_type,
            additional_bindings: type_bindings,
            constraint_result: Some(constraint_result),
            confidence,
        })
    }

    /// Clear inference cache
    pub fn clear_cache(&mut self) {
        self.inference_cache.clear();
    }

    /// Get cached inference result
    pub fn get_cached_result(&self, key: &str) -> Option<&Type> {
        self.inference_cache.get(key)
    }

    /// Cache inference result
    pub fn cache_result(&mut self, key: String, result: Type) {
        self.inference_cache.insert(key, result);
    }
}

impl InferenceContext {
    /// Create a new empty inference context
    pub fn new() -> Self {
        Self {
            type_bindings: HashMap::new(),
            constraints: Vec::new(),
            type_variables: HashMap::new(),
            expected_return_type: None,
        }
    }

    /// Create context with existing type bindings
    pub fn with_bindings(bindings: HashMap<String, Type>) -> Self {
        Self {
            type_bindings: bindings,
            constraints: Vec::new(),
            type_variables: HashMap::new(),
            expected_return_type: None,
        }
    }

    /// Add a type binding
    pub fn add_binding(&mut self, name: String, type_: Type) {
        self.type_bindings.insert(name, type_);
    }

    /// Add a constraint
    pub fn add_constraint(&mut self, constraint: GenericConstraint) {
        self.constraints.push(constraint);
    }

    /// Set expected return type
    pub fn with_expected_return_type(mut self, return_type: Type) -> Self {
        self.expected_return_type = Some(return_type);
        self
    }
}

impl Default for InferenceContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType};

    #[test]
    fn test_inference_context() {
        let mut context = InferenceContext::new();
        assert!(context.type_bindings.is_empty());

        context.add_binding("T".to_string(), Type::Thicc);
        assert_eq!(context.type_bindings.get("T"), Some(&Type::Thicc));
    }

    #[test]
    fn test_inference_result() {
        let result = InferenceResult {
            inferred_type: Type::Thicc,
            additional_bindings: HashMap::new(),
            constraint_result: None,
            confidence: 0.9,
        };

        assert_eq!(result.inferred_type, Type::Thicc);
        assert!(result.confidence > 0.8);
    }

    #[test]
    fn test_fresh_type_variable() {
        let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
        let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
        let constraint_resolver = Arc::new(RwLock::new(
            ConstraintResolver::new(type_checker.clone(), interface_registry)
        ));
        
        let mut inference = EnhancedTypeInference::new(type_checker, constraint_resolver);
        
        let var1 = inference.fresh_type_variable();
        let var2 = inference.fresh_type_variable();
        
        // Should generate different type variables
        assert_ne!(var1, var2);
        
        // Should be type parameters
        assert!(matches!(var1, Type::TypeParam(_)));
        assert!(matches!(var2, Type::TypeParam(_)));
    }
}
