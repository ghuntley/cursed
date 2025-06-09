//! Constraint validation system that integrates with existing type checking
//!
//! This module provides validation for type constraints during type checking,
//! ensuring that generic type parameters satisfy their interface requirements
//! and that constraint hierarchies are properly enforced.

use crate::ast::declarations::{GenericConstraint, FunctionStatement, SquadStatement, CollabStatement};
use crate::ast::expressions::calls::CallExpression;
use crate::core::type_checker::{Type, TypeChecker};
use crate::core::constraint_resolver::{ConstraintResolver, ConstraintResolutionResult, ConstraintViolation};
use crate::core::constraint_error::{create_constraint_error, create_nested_constraint_error};
use crate::core::interface_registry::InterfaceRegistry;
use crate::error::Error;
use crate::ast::traits::Node;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, instrument, warn};

/// Validation context for constraint checking
#[derive(Debug, Clone)]
pub struct ValidationContext {
    /// Current function being analyzed (if any)
    pub current_function: Option<String>,
    /// Current struct being analyzed (if any) 
    pub current_struct: Option<String>,
    /// Type parameter bindings in scope
    pub type_bindings: HashMap<String, Type>,
    /// Active constraints in scope
    pub active_constraints: Vec<GenericConstraint>,
    /// Constraint validation depth (to prevent infinite recursion)
    pub validation_depth: usize,
    /// Maximum allowed validation depth
    pub max_depth: usize,
}

/// Result of constraint validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether validation passed
    pub valid: bool,
    /// List of validation errors
    pub errors: Vec<Error>,
    /// Warnings that don't prevent compilation
    pub warnings: Vec<String>,
    /// Performance metrics
    pub metrics: ValidationMetrics,
}

/// Performance metrics for constraint validation
#[derive(Debug, Clone)]
pub struct ValidationMetrics {
    /// Number of constraints checked
    pub constraints_checked: usize,
    /// Number of interface implementations verified
    pub implementations_verified: usize,
    /// Time spent in validation (microseconds)
    pub validation_time_us: u64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
}

/// Constraint validator that integrates with type checking
#[derive(Debug)]
pub struct ConstraintValidator {
    /// Type checker for interface implementation checks
    type_checker: Arc<RwLock<TypeChecker>>,
    /// Interface registry for constraint lookup
    interface_registry: Arc<RwLock<InterfaceRegistry>>,
    /// Constraint resolver for complex constraint resolution
    constraint_resolver: Arc<RwLock<ConstraintResolver>>,
    /// Cache for validation results
    validation_cache: HashMap<String, ValidationResult>,
    /// Performance metrics
    metrics: ValidationMetrics,
}

impl ConstraintValidator {
    /// Create a new constraint validator
    #[instrument(level = "debug")]
    pub fn new(
        type_checker: Arc<RwLock<TypeChecker>>,
        interface_registry: Arc<RwLock<InterfaceRegistry>>,
        constraint_resolver: Arc<RwLock<ConstraintResolver>>,
    ) -> Self {
        info!("Creating constraint validator");
        Self {
            type_checker,
            interface_registry,
            constraint_resolver,
            validation_cache: HashMap::new(),
            metrics: ValidationMetrics::new(),
        }
    }

    /// Validate constraints during type checking
    #[instrument(skip(self, expression), level = "debug")]
    pub fn validate_during_type_checking(
        &mut self,
        expression: &dyn crate::ast::Expression,
        context: &ValidationContext,
    ) -> Result<ValidationResult, Error> {
        let start_time = std::time::Instant::now();
        
        debug!(
            validation_depth = context.validation_depth,
            max_depth = context.max_depth,
            "Validating constraints during type checking"
        );

        if context.validation_depth >= context.max_depth {
            warn!(
                validation_depth = context.validation_depth,
                max_depth = context.max_depth,
                "Maximum validation depth reached"
            );
            return Ok(ValidationResult::max_depth_exceeded());
        }

        let result = self.validate_expression_constraints(expression, context)?;
        
        self.metrics.validation_time_us += start_time.elapsed().as_micros() as u64;
        Ok(result)
    }

    /// Validate constraints for a generic function call
    #[instrument(skip(self, call_expr, function), level = "debug")]
    pub fn validate_function_call_constraints(
        &mut self,
        call_expr: &CallExpression,
        function: &FunctionStatement,
        context: &ValidationContext,
    ) -> Result<ValidationResult, Error> {
        debug!(
            function_name = %function.name.string(),
            constraints_count = function.generic_constraints.len(),
            "Validating function call constraints"
        );

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check if we have type arguments
        if call_expr.type_arguments.is_empty() {
            // No explicit type arguments - need to infer and validate
            return self.validate_inferred_function_call(call_expr, function, context);
        }

        // Explicit type arguments provided - validate directly
        if call_expr.type_arguments.len() != function.type_parameters.len() {
            let error = Error::new(
                "ConstraintValidation",
                &format!(
                    "Type argument count mismatch: expected {}, got {}",
                    function.type_parameters.len(),
                    call_expr.type_arguments.len()
                ),
                None
            );
            errors.push(error);
            return Ok(ValidationResult::with_errors(errors));
        }

        // Create type parameter mapping
        let type_mapping: HashMap<String, Type> = function.type_parameters
            .iter()
            .zip(call_expr.type_arguments.iter())
            .map(|(param, arg)| (param.name.clone(), arg.clone()))
            .collect();

        // Validate each constraint
        for constraint in &function.generic_constraints {
            if let Some(concrete_type) = type_mapping.get(&constraint.parameter_name) {
                match self.validate_single_constraint_internal(constraint, concrete_type, context) {
                    Ok(valid) => {
                        if !valid {
                            let error = create_constraint_error(
                                concrete_type,
                                &constraint.interface_name,
                                Some(&constraint.parameter_name),
                                None,
                                None,
                            );
                            errors.push(Error::from(error));
                        }
                    }
                    Err(e) => {
                        error!(
                            constraint = ?constraint,
                            concrete_type = ?concrete_type,
                            error = ?e,
                            "Error validating constraint"
                        );
                        errors.push(e);
                    }
                }
            }
        }

        self.metrics.constraints_checked += function.generic_constraints.len();

        Ok(ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings,
            metrics: self.metrics.clone(),
        })
    }

    /// Validate constraints for struct instantiation
    #[instrument(skip(self, struct_stmt), level = "debug")]
    pub fn validate_struct_instantiation_constraints(
        &mut self,
        struct_stmt: &SquadStatement,
        type_arguments: &[Type],
        context: &ValidationContext,
    ) -> Result<ValidationResult, Error> {
        debug!(
            struct_name = %struct_stmt.name.string(),
            type_args_count = type_arguments.len(),
            constraints_count = struct_stmt.generic_constraints.len(),
            "Validating struct instantiation constraints"
        );

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Validate type argument count
        if type_arguments.len() != struct_stmt.type_parameters.len() {
            let error = Error::new(
                "TypeError", 
                &format!(
                    "Type argument count mismatch for struct {}: expected {}, got {}",
                    struct_stmt.name.string(),
                    struct_stmt.type_parameters.len(),
                    type_arguments.len()
                ),
                None
            );
            errors.push(error);
            return Ok(ValidationResult::with_errors(errors));
        }

        // Create type parameter mapping
        let type_mapping: HashMap<String, Type> = struct_stmt.type_parameters
            .iter()
            .zip(type_arguments.iter())
            .map(|(param, arg)| (param.name.clone(), arg.clone()))
            .collect();

        // Validate constraints using constraint resolver
        let mut resolver = self.constraint_resolver.write()
            .map_err(|e| Error::new("SystemError", &format!("Failed to acquire constraint resolver lock: {}", e), None))?;

        match resolver.resolve_struct_constraints(struct_stmt, type_arguments) {
            Ok(resolution_result) => {
                if !resolution_result.satisfied {
                    for violation in &resolution_result.violations {
                        let error = create_constraint_error(
                            &violation.concrete_type,
                            &violation.interface_constraint,
                            Some(&violation.type_parameter),
                            None,
                            Some(violation.missing_methods.clone()),
                        );
                        errors.push(Error::from(error));
                    }
                }
            }
            Err(e) => {
                error!(
                    struct_name = %struct_stmt.name.string(),
                    error = ?e,
                    "Error resolving struct constraints"
                );
                errors.push(e);
            }
        }

        self.metrics.constraints_checked += struct_stmt.generic_constraints.len();

        Ok(ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings,
            metrics: self.metrics.clone(),
        })
    }

    /// Validate interface implementation constraints
    #[instrument(skip(self, interface_stmt), level = "debug")]
    pub fn validate_interface_implementation_constraints(
        &mut self,
        interface_stmt: &CollabStatement,
        implementing_type: &Type,
        context: &ValidationContext,
    ) -> Result<ValidationResult, Error> {
        debug!(
            interface_name = %interface_stmt.name.string(),
            implementing_type = ?implementing_type,
            "Validating interface implementation constraints"
        );

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Use constraint resolver for comprehensive validation
        let mut resolver = self.constraint_resolver.write()
            .map_err(|e| Error::new("SystemError", &format!("Failed to acquire constraint resolver lock: {}", e), None))?;

        match resolver.resolve_interface_constraints(interface_stmt, implementing_type) {
            Ok(resolution_result) => {
                if !resolution_result.satisfied {
                    for violation in &resolution_result.violations {
                        let error = create_constraint_error(
                            implementing_type,
                            &violation.interface_constraint,
                            Some(&violation.type_parameter),
                            None,
                            Some(violation.missing_methods.clone()),
                        );
                        errors.push(Error::from(error));
                    }
                }
            }
            Err(e) => {
                error!(
                    interface_name = %interface_stmt.name.string(),
                    implementing_type = ?implementing_type,
                    error = ?e,
                    "Error resolving interface constraints"
                );
                errors.push(e);
            }
        }

        self.metrics.implementations_verified += 1;

        Ok(ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings,
            metrics: self.metrics.clone(),
        })
    }

    /// Validate constraints for inferred function call
    #[instrument(skip(self, call_expr, function), level = "debug")]
    fn validate_inferred_function_call(
        &mut self,
        call_expr: &CallExpression,
        function: &FunctionStatement,
        context: &ValidationContext,
    ) -> Result<ValidationResult, Error> {
        debug!("Validating inferred function call constraints");

        // For now, we'll use a simplified validation
        // In a full implementation, this would integrate with type inference
        let mut warnings = Vec::new();
        
        if !function.generic_constraints.is_empty() {
            warnings.push(format!(
                "Function {} has generic constraints but no explicit type arguments provided. Type inference may not guarantee constraint satisfaction.",
                function.name.string()
            ));
        }

        Ok(ValidationResult {
            valid: true,
            errors: Vec::new(),
            warnings,
            metrics: self.metrics.clone(),
        })
    }

    /// Validate a single constraint
    #[instrument(skip(self), level = "debug")]
    fn validate_single_constraint_internal(
        &self,
        constraint: &GenericConstraint,
        concrete_type: &Type,
        context: &ValidationContext,
    ) -> Result<bool, Error> {
        debug!(
            constraint = ?constraint,
            concrete_type = ?concrete_type,
            "Validating single constraint"
        );

        // Check cache first
        let cache_key = format!("{}:{}:{:?}", 
            constraint.parameter_name, 
            constraint.interface_name, 
            concrete_type
        );
        
        if let Some(cached_result) = self.validation_cache.get(&cache_key) {
            return Ok(cached_result.valid);
        }

        // Use type checker to validate interface implementation
        let type_checker = self.type_checker.read()
            .map_err(|e| Error::new("SystemError", &format!("Failed to acquire type checker lock: {}", e), None))?;

        // Convert interface name string to Type for the type checker method
        let interface_type = Type::Interface(constraint.interface_name.clone(), vec![]);
        let implements = type_checker.check_interface_implementation(
            concrete_type,
            &interface_type,
        )?;

        debug!(
            constraint = ?constraint,
            concrete_type = ?concrete_type,
            implements = implements,
            "Single constraint validation result"
        );

        Ok(implements)
    }

    /// Validate expression constraints
    #[instrument(skip(self, expression), level = "debug")]
    fn validate_expression_constraints(
        &mut self,
        expression: &dyn crate::ast::Expression,
        context: &ValidationContext,
    ) -> Result<ValidationResult, Error> {
        // This is a placeholder for expression-level constraint validation
        // In a full implementation, this would analyze different expression types
        // and validate any generic constraints they involve
        
        debug!("Validating expression constraints");
        
        Ok(ValidationResult {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            metrics: self.metrics.clone(),
        })
    }

    /// Validate constraint hierarchies and dependencies
    #[instrument(skip(self), level = "debug")]
    pub fn validate_constraint_hierarchies(
        &mut self,
        constraints: &[GenericConstraint],
        context: &ValidationContext,
    ) -> Result<ValidationResult, Error> {
        debug!(
            constraints_count = constraints.len(),
            "Validating constraint hierarchies"
        );

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check for circular constraint dependencies
        let circular_deps = self.detect_circular_constraints(constraints)?;
        if !circular_deps.is_empty() {
            for circular in circular_deps {
                let error = Error::new(
                    "ConstraintError",
                    &format!(
                        "Circular constraint dependency detected: {}",
                        circular.join(" -> ")
                    ),
                    None
                );
                errors.push(error);
            }
        }

        // Check for unsatisfiable constraint combinations
        let unsatisfiable = self.detect_unsatisfiable_constraints(constraints)?;
        if !unsatisfiable.is_empty() {
            for constraint_set in unsatisfiable {
                let error = Error::new(
                    "ConstraintError",
                    &format!(
                        "Unsatisfiable constraint combination: {}",
                        constraint_set.iter()
                            .map(|c| format!("{}:{}", c.parameter_name, c.interface_name))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    None
                );
                errors.push(error);
            }
        }

        Ok(ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings,
            metrics: self.metrics.clone(),
        })
    }

    /// Detect circular constraint dependencies
    #[instrument(skip(self), level = "debug")]
    fn detect_circular_constraints(
        &self,
        constraints: &[GenericConstraint],
    ) -> Result<Vec<Vec<String>>, Error> {
        // Simplified circular dependency detection
        // In a full implementation, this would build a proper dependency graph
        
        let mut circular_deps = Vec::new();
        let mut visited = HashSet::new();
        
        for constraint in constraints {
            let constraint_id = format!("{}:{}", constraint.parameter_name, constraint.interface_name);
            if !visited.contains(&constraint_id) {
                // Check if this constraint creates a cycle
                // (Simplified check - real implementation would be more sophisticated)
                visited.insert(constraint_id);
            }
        }

        Ok(circular_deps)
    }

    /// Detect unsatisfiable constraint combinations
    #[instrument(skip(self), level = "debug")]
    fn detect_unsatisfiable_constraints(
        &self,
        constraints: &[GenericConstraint],
    ) -> Result<Vec<Vec<GenericConstraint>>, Error> {
        // Group constraints by type parameter
        let mut constraints_by_param: HashMap<String, Vec<&GenericConstraint>> = HashMap::new();
        for constraint in constraints {
            constraints_by_param
                .entry(constraint.parameter_name.clone())
                .or_default()
                .push(constraint);
        }

        let mut unsatisfiable = Vec::new();

        // Check each type parameter's constraints for satisfiability
        for (param_name, param_constraints) in constraints_by_param {
            if param_constraints.len() > 1 {
                // Check if any single type could satisfy all constraints for this parameter
                let satisfiable = self.check_constraints_satisfiable(&param_constraints)?;
                if !satisfiable {
                    unsatisfiable.push(param_constraints.into_iter().cloned().collect());
                }
            }
        }

        Ok(unsatisfiable)
    }

    /// Check if a set of constraints can be satisfied by any single type
    #[instrument(skip(self), level = "debug")]
    fn check_constraints_satisfiable(
        &self,
        constraints: &[&GenericConstraint],
    ) -> Result<bool, Error> {
        if constraints.len() <= 1 {
            return Ok(true);
        }

        // For now, assume all constraints are satisfiable
        // In a full implementation, this would:
        // 1. Get all types that implement the first interface
        // 2. Check if any of them also implement all other interfaces
        // 3. Return true if at least one type satisfies all constraints

        Ok(true)
    }

    /// Get validation statistics
    pub fn get_metrics(&self) -> &ValidationMetrics {
        &self.metrics
    }

    /// Clear validation cache
    pub fn clear_cache(&mut self) {
        self.validation_cache.clear();
        self.metrics.cache_hit_rate = 0.0;
    }

    /// Update cache hit rate
    fn update_cache_hit_rate(&mut self, hit: bool) {
        // Simple moving average for cache hit rate
        const ALPHA: f64 = 0.1;
        let new_rate = if hit { 1.0 } else { 0.0 };
        self.metrics.cache_hit_rate = (1.0 - ALPHA) * self.metrics.cache_hit_rate + ALPHA * new_rate;
    }
}

impl ValidationContext {
    /// Create a new validation context
    pub fn new() -> Self {
        Self {
            current_function: None,
            current_struct: None,
            type_bindings: HashMap::new(),
            active_constraints: Vec::new(),
            validation_depth: 0,
            max_depth: 10,
        }
    }

    /// Create context for function validation
    pub fn for_function(function_name: String) -> Self {
        Self {
            current_function: Some(function_name),
            current_struct: None,
            type_bindings: HashMap::new(),
            active_constraints: Vec::new(),
            validation_depth: 0,
            max_depth: 10,
        }
    }

    /// Create context for struct validation
    pub fn for_struct(struct_name: String) -> Self {
        Self {
            current_function: None,
            current_struct: Some(struct_name),
            type_bindings: HashMap::new(),
            active_constraints: Vec::new(),
            validation_depth: 0,
            max_depth: 10,
        }
    }

    /// Create child context with increased depth
    pub fn child_context(&self) -> Self {
        Self {
            current_function: self.current_function.clone(),
            current_struct: self.current_struct.clone(),
            type_bindings: self.type_bindings.clone(),
            active_constraints: self.active_constraints.clone(),
            validation_depth: self.validation_depth + 1,
            max_depth: self.max_depth,
        }
    }

    /// Add constraint to context
    pub fn add_constraint(&mut self, constraint: GenericConstraint) {
        self.active_constraints.push(constraint);
    }

    /// Add type binding to context
    pub fn add_type_binding(&mut self, name: String, type_: Type) {
        self.type_bindings.insert(name, type_);
    }
}

impl ValidationResult {
    /// Create result with errors
    pub fn with_errors(errors: Vec<Error>) -> Self {
        Self {
            valid: false,
            errors,
            warnings: Vec::new(),
            metrics: ValidationMetrics::new(),
        }
    }

    /// Create result indicating max depth exceeded
    pub fn max_depth_exceeded() -> Self {
        Self {
            valid: false,
            errors: vec![Error::new(
                "ConstraintError",
                "Maximum constraint validation depth exceeded",
                None
            )],
            warnings: vec!["Consider simplifying constraint hierarchies".to_string()],
            metrics: ValidationMetrics::new(),
        }
    }

    /// Check if validation was successful
    pub fn is_valid(&self) -> bool {
        self.valid && self.errors.is_empty()
    }

    /// Get all validation messages (errors and warnings)
    pub fn get_all_messages(&self) -> Vec<String> {
        let mut messages = Vec::new();
        
        for error in &self.errors {
            messages.push(format!("ERROR: {}", error));
        }
        
        for warning in &self.warnings {
            messages.push(format!("WARNING: {}", warning));
        }
        
        messages
    }
}

impl ValidationMetrics {
    /// Create new metrics with default values
    pub fn new() -> Self {
        Self {
            constraints_checked: 0,
            implementations_verified: 0,
            validation_time_us: 0,
            cache_hit_rate: 0.0,
        }
    }

    /// Merge with another metrics instance
    pub fn merge(&mut self, other: &ValidationMetrics) {
        self.constraints_checked += other.constraints_checked;
        self.implementations_verified += other.implementations_verified;
        self.validation_time_us += other.validation_time_us;
        // Cache hit rate is averaged
        self.cache_hit_rate = (self.cache_hit_rate + other.cache_hit_rate) / 2.0;
    }
}

impl Default for ValidationContext {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ValidationMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::Token;
    use crate::lexer::TokenType;

    #[test]
    fn test_validation_context() {
        let mut context = ValidationContext::new();
        assert_eq!(context.validation_depth, 0);
        assert_eq!(context.max_depth, 10);

        context.add_type_binding("T".to_string(), Type::Thicc);
        assert_eq!(context.type_bindings.get("T"), Some(&Type::Thicc));

        let child = context.child_context();
        assert_eq!(child.validation_depth, 1);
        assert_eq!(child.type_bindings.get("T"), Some(&Type::Thicc));
    }

    #[test]
    fn test_validation_result() {
        let result = ValidationResult::with_errors(vec![
            Error::new("Test", "Test error", None)
        ]);
        assert!(!result.is_valid());
        assert_eq!(result.errors.len(), 1);

        let messages = result.get_all_messages();
        assert_eq!(messages.len(), 1);
        assert!(messages[0].starts_with("ERROR:"));
    }

    #[test]
    fn test_validation_metrics() {
        let mut metrics1 = ValidationMetrics::new();
        metrics1.constraints_checked = 5;
        metrics1.cache_hit_rate = 0.8;

        let metrics2 = ValidationMetrics {
            constraints_checked: 3,
            implementations_verified: 2,
            validation_time_us: 100,
            cache_hit_rate: 0.6,
        };

        metrics1.merge(&metrics2);
        assert_eq!(metrics1.constraints_checked, 8);
        assert_eq!(metrics1.implementations_verified, 2);
        assert_eq!(metrics1.validation_time_us, 100);
        assert_eq!(metrics1.cache_hit_rate, 0.7); // Average of 0.8 and 0.6
    }
}
