//! Core type constraint resolution and type checking logic
//!
//! This module implements the constraint resolution algorithms that handle:
//! - Interface-based constraints
//! - Where clause constraints  
//! - Multi-parameter generic constraints
//! - Constraint satisfaction during type checking
//! - Constraint propagation and unification

use crate::ast::declarations::{GenericConstraint, FunctionStatement, SquadStatement, CollabStatement};
use crate::core::type_checker::{Type, TypeChecker};
use crate::core::interface_registry::InterfaceRegistry;
use crate::core::constraint_error::{create_constraint_error, create_nested_constraint_error};
use crate::error::Error;
use crate::ast::traits::Node;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, instrument, warn};

/// Represents the result of constraint resolution
#[derive(Debug, Clone)]
pub struct ConstraintResolutionResult {
    /// Whether all constraints are satisfied
    pub satisfied: bool,
    /// List of constraint violations
    pub violations: Vec<ConstraintViolation>,
    /// Type substitutions computed during resolution
    pub type_substitutions: HashMap<String, Type>,
    /// Inferred types that satisfy constraints
    pub inferred_types: HashMap<String, Type>,
}

/// Represents a specific constraint violation
#[derive(Debug, Clone)]
pub struct ConstraintViolation {
    /// The type parameter that violated the constraint
    pub type_parameter: String,
    /// The concrete type that failed the constraint
    pub concrete_type: Type,
    /// The interface constraint that was violated
    pub interface_constraint: String,
    /// Additional context about the violation
    pub context: String,
    /// Missing methods that prevented satisfaction
    pub missing_methods: Vec<String>,
}

/// Core constraint resolver for the enhanced generic system
#[derive(Debug)]
pub struct ConstraintResolver {
    /// Reference to the type checker for interface implementation checks
    type_checker: Arc<RwLock<TypeChecker>>,
    /// Interface registry for constraint validation
    interface_registry: Arc<RwLock<InterfaceRegistry>>,
    /// Cache for constraint resolution results
    resolution_cache: HashMap<String, ConstraintResolutionResult>,
    /// Constraint dependency graph for complex constraint hierarchies
    dependency_graph: ConstraintDependencyGraph,
}

/// Represents dependencies between constraints
#[derive(Debug, Clone)]
struct ConstraintDependencyGraph {
    /// Maps constraint ID to its dependencies
    dependencies: HashMap<String, HashSet<String>>,
    /// Tracks circular dependencies
    circular_deps: HashSet<String>,
}

impl ConstraintResolver {
    /// Create a new constraint resolver
    #[instrument(level = "debug")]
    pub fn new(
        type_checker: Arc<RwLock<TypeChecker>>,
        interface_registry: Arc<RwLock<InterfaceRegistry>>,
    ) -> Self {
        info!("Creating new constraint resolver");
        Self {
            type_checker,
            interface_registry,
            resolution_cache: HashMap::new(),
            dependency_graph: ConstraintDependencyGraph::new(),
        }
    }

    /// Resolve constraints for a generic function call
    #[instrument(skip(self), level = "debug")]
    pub fn resolve_function_constraints(
        &mut self,
        function: &FunctionStatement,
        type_arguments: &[Type],
    ) -> Result<ConstraintResolutionResult, Error> {
        debug!(
            function_name = %function.name.string(),
            type_args = ?type_arguments,
            constraints_count = function.generic_constraints.len(),
            "Resolving function constraints"
        );

        // Create type parameter mapping
        let type_param_mapping = self.create_type_parameter_mapping(
            &function.type_parameters,
            type_arguments,
        )?;

        // Check each constraint
        let mut violations = Vec::new();
        let mut type_substitutions = HashMap::new();

        for constraint in &function.generic_constraints {
            if let Some(concrete_type) = type_param_mapping.get(&constraint.parameter_name) {
                match self.check_constraint(constraint, concrete_type) {
                    Ok(satisfied) => {
                        if !satisfied {
                            violations.push(self.create_violation(constraint, concrete_type)?);
                        } else {
                            type_substitutions.insert(
                                constraint.parameter_name.clone(),
                                concrete_type.clone(),
                            );
                        }
                    }
                    Err(e) => {
                        error!(
                            constraint = ?constraint,
                            concrete_type = ?concrete_type,
                            error = ?e,
                            "Error checking constraint"
                        );
                        violations.push(self.create_violation(constraint, concrete_type)?);
                    }
                }
            }
        }

        let result = ConstraintResolutionResult {
            satisfied: violations.is_empty(),
            violations,
            type_substitutions,
            inferred_types: HashMap::new(),
        };

        Ok(result)
    }

    /// Resolve constraints for a generic struct instantiation
    #[instrument(skip(self), level = "debug")]
    pub fn resolve_struct_constraints(
        &mut self,
        struct_stmt: &SquadStatement,
        type_arguments: &[Type],
    ) -> Result<ConstraintResolutionResult, Error> {
        debug!(
            struct_name = %struct_stmt.name.string(),
            type_args = ?type_arguments,
            constraints_count = struct_stmt.generic_constraints.len(),
            "Resolving struct constraints"
        );

        // Create type parameter mapping
        let type_param_mapping = self.create_type_parameter_mapping(
            &struct_stmt.type_parameters,
            type_arguments,
        )?;

        // Check each constraint
        let mut violations = Vec::new();
        let mut type_substitutions = HashMap::new();

        for constraint in &struct_stmt.generic_constraints {
            if let Some(concrete_type) = type_param_mapping.get(&constraint.parameter_name) {
                match self.check_constraint(constraint, concrete_type) {
                    Ok(satisfied) => {
                        if !satisfied {
                            violations.push(self.create_violation(constraint, concrete_type)?);
                        } else {
                            type_substitutions.insert(
                                constraint.parameter_name.clone(),
                                concrete_type.clone(),
                            );
                        }
                    }
                    Err(e) => {
                        error!(
                            constraint = ?constraint,
                            concrete_type = ?concrete_type,
                            error = ?e,
                            "Error checking struct constraint"
                        );
                        violations.push(self.create_violation(constraint, concrete_type)?);
                    }
                }
            }
        }

        let result = ConstraintResolutionResult {
            satisfied: violations.is_empty(),
            violations,
            type_substitutions,
            inferred_types: HashMap::new(),
        };

        Ok(result)
    }

    /// Resolve constraints for an interface implementation
    #[instrument(skip(self), level = "debug")]
    pub fn resolve_interface_constraints(
        &mut self,
        interface_stmt: &CollabStatement,
        implementing_type: &Type,
    ) -> Result<ConstraintResolutionResult, Error> {
        debug!(
            interface_name = %interface_stmt.name.string(),
            implementing_type = ?implementing_type,
            constraints_count = interface_stmt.generic_constraints.len(),
            "Resolving interface constraints"
        );

        let mut violations = Vec::new();
        let mut type_substitutions = HashMap::new();

        // For interface constraints, we need to check if the implementing type
        // satisfies all the interface's generic constraints
        for constraint in &interface_stmt.generic_constraints {
            // Interface constraints are typically self-referential or about method types
            match self.check_interface_constraint(constraint, implementing_type, interface_stmt) {
                Ok(satisfied) => {
                    if !satisfied {
                        violations.push(self.create_violation(constraint, implementing_type)?);
                    } else {
                        type_substitutions.insert(
                            constraint.parameter_name.clone(),
                            implementing_type.clone(),
                        );
                    }
                }
                Err(e) => {
                    error!(
                        constraint = ?constraint,
                        implementing_type = ?implementing_type,
                        error = ?e,
                        "Error checking interface constraint"
                    );
                    violations.push(self.create_violation(constraint, implementing_type)?);
                }
            }
        }

        let result = ConstraintResolutionResult {
            satisfied: violations.is_empty(),
            violations,
            type_substitutions,
            inferred_types: HashMap::new(),
        };

        Ok(result)
    }

    /// Infer types that satisfy given constraints
    #[instrument(skip(self), level = "debug")]
    pub fn infer_constraint_satisfying_types(
        &mut self,
        constraints: &[GenericConstraint],
        context_types: &HashMap<String, Type>,
    ) -> Result<HashMap<String, Type>, Error> {
        debug!(
            constraints_count = constraints.len(),
            context_types_count = context_types.len(),
            "Inferring types that satisfy constraints"
        );

        let mut inferred_types = HashMap::new();

        // Build constraint dependency graph
        self.build_dependency_graph(constraints)?;

        // Resolve constraints in dependency order
        let resolution_order = self.dependency_graph.topological_sort()?;

        for constraint_id in resolution_order {
            if let Some(constraint) = constraints.iter().find(|c| 
                format!("{}:{}", c.parameter_name, c.interface_name) == constraint_id
            ) {
                // Try to infer a type that satisfies this constraint
                if let Some(inferred) = self.infer_type_for_constraint(constraint, context_types)? {
                    inferred_types.insert(constraint.parameter_name.clone(), inferred);
                }
            }
        }

        Ok(inferred_types)
    }

    /// Check if a concrete type satisfies a constraint
    #[instrument(skip(self), level = "debug")]
    fn check_constraint(
        &self,
        constraint: &GenericConstraint,
        concrete_type: &Type,
    ) -> Result<bool, Error> {
        debug!(
            param = %constraint.parameter_name,
            interface = %constraint.interface_name,
            concrete_type = ?concrete_type,
            "Checking constraint satisfaction"
        );

        // Use the type checker to verify interface implementation
        let type_checker = self.type_checker.read()
            .map_err(|e| Error::new("lock_error", &format!("Failed to acquire type checker lock: {}", e), None))?;

        // Check if the concrete type implements the required interface
        let interface_type = Type::Named(constraint.interface_name.clone());
        let implements = type_checker.check_interface_implementation(
            concrete_type,
            &interface_type,
        )?;

        debug!(
            param = %constraint.parameter_name,
            interface = %constraint.interface_name,
            concrete_type = ?concrete_type,
            implements = implements,
            "Constraint check result"
        );

        Ok(implements)
    }

    /// Check interface-specific constraints
    #[instrument(skip(self), level = "debug")]
    fn check_interface_constraint(
        &self,
        constraint: &GenericConstraint,
        implementing_type: &Type,
        interface_stmt: &CollabStatement,
    ) -> Result<bool, Error> {
        debug!(
            constraint = ?constraint,
            implementing_type = ?implementing_type,
            interface_name = %interface_stmt.name.string(),
            "Checking interface-specific constraint"
        );

        // For interface constraints, we need to ensure the implementing type
        // can provide all required methods with compatible signatures
        let type_checker = self.type_checker.read()
            .map_err(|e| Error::new("lock_error", &format!("Failed to acquire type checker lock: {}", e), None))?;

        // Check if implementing type has all required methods
        for method in &interface_stmt.methods {
            let has_method = type_checker.has_method(implementing_type, &method.name.string())?;
            if !has_method {
                debug!(
                    implementing_type = ?implementing_type,
                    missing_method = %method.name.string(),
                    "Missing required method"
                );
                return Ok(false);
            }

            // Check method signature compatibility
            let return_type = if let Some(ref rt) = method.return_type {
                Type::Named(rt.string())
            } else {
                Type::Unknown
            };
            
            let signature_compatible = type_checker.check_method_signature_compatibility(
                implementing_type,
                &method.name.string(),
                &method.parameters,
                &return_type,
            )?;

            if !signature_compatible {
                debug!(
                    implementing_type = ?implementing_type,
                    method = %method.name.string(),
                    "Incompatible method signature"
                );
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Create type parameter mapping from parameters to arguments
    #[instrument(skip(self), level = "debug")]
    fn create_type_parameter_mapping(
        &self,
        type_parameters: &[crate::ast::declarations::TypeParameter],
        type_arguments: &[Type],
    ) -> Result<HashMap<String, Type>, Error> {
        if type_parameters.len() != type_arguments.len() {
            return Err(Error::new(
                "type_mismatch",
                &format!(
                    "Type parameter count mismatch: expected {}, got {}",
                    type_parameters.len(),
                    type_arguments.len()
                ),
                None
            ));
        }

        let mapping = type_parameters
            .iter()
            .zip(type_arguments.iter())
            .map(|(param, arg)| (param.name.clone(), arg.clone()))
            .collect();

        debug!(mapping = ?mapping, "Created type parameter mapping");
        Ok(mapping)
    }

    /// Create a constraint violation record
    #[instrument(skip(self), level = "debug")]
    fn create_violation(
        &self,
        constraint: &GenericConstraint,
        concrete_type: &Type,
    ) -> Result<ConstraintViolation, Error> {
        // Get missing methods information
        let missing_methods = self.get_missing_methods(concrete_type, &constraint.interface_name)?;

        let violation = ConstraintViolation {
            type_parameter: constraint.parameter_name.clone(),
            concrete_type: concrete_type.clone(),
            interface_constraint: constraint.interface_name.clone(),
            context: format!(
                "Type parameter '{}' with concrete type '{:?}' does not implement interface '{}'",
                constraint.parameter_name, concrete_type, constraint.interface_name
            ),
            missing_methods,
        };

        debug!(violation = ?violation, "Created constraint violation");
        Ok(violation)
    }

    /// Get missing methods for a constraint violation
    #[instrument(skip(self), level = "debug")]
    fn get_missing_methods(
        &self,
        concrete_type: &Type,
        interface_name: &str,
    ) -> Result<Vec<String>, Error> {
        let type_checker = self.type_checker.read()
            .map_err(|e| Error::new("lock_error", &format!("Failed to acquire type checker lock: {}", e), None))?;

        // Get required methods from interface
        let required_methods = type_checker.get_interface_methods(interface_name)?;

        // Get available methods on the type
        let available_methods = type_checker.get_type_methods(concrete_type)?;

        // Find missing methods
        let missing: Vec<String> = required_methods
            .into_iter()
            .filter(|method| !available_methods.contains(method))
            .collect();

        debug!(
            concrete_type = ?concrete_type,
            interface = %interface_name,
            missing_methods = ?missing,
            "Computed missing methods"
        );

        Ok(missing)
    }

    /// Build constraint dependency graph
    #[instrument(skip(self), level = "debug")]
    fn build_dependency_graph(&mut self, constraints: &[GenericConstraint]) -> Result<(), Error> {
        debug!(constraints_count = constraints.len(), "Building constraint dependency graph");

        self.dependency_graph.clear();

        for constraint in constraints {
            let constraint_id = format!("{}:{}", constraint.parameter_name, constraint.interface_name);
            
            // Find dependencies (other constraints that this one depends on)
            let dependencies = self.find_constraint_dependencies(constraint, constraints)?;
            
            self.dependency_graph.add_constraint(constraint_id, dependencies);
        }

        // Check for circular dependencies
        if let Err(circular) = self.dependency_graph.check_circular_dependencies() {
            warn!(circular_deps = ?circular, "Detected circular constraint dependencies");
            // For now, we'll continue but mark them for special handling
            self.dependency_graph.circular_deps.extend(circular);
        }

        Ok(())
    }

    /// Find dependencies for a specific constraint
    #[instrument(skip(self), level = "debug")]
    fn find_constraint_dependencies(
        &self,
        constraint: &GenericConstraint,
        all_constraints: &[GenericConstraint],
    ) -> Result<HashSet<String>, Error> {
        let mut dependencies = HashSet::new();

        // A constraint depends on another if:
        // 1. The interface it requires extends another interface that's also constrained
        // 2. The type parameter appears in the definition of another constrained interface

        for other_constraint in all_constraints {
            if constraint.parameter_name == other_constraint.parameter_name {
                continue; // Skip self
            }

            // Check if constraint's interface extends other_constraint's interface
            if self.interface_extends(&constraint.interface_name, &other_constraint.interface_name)? {
                let dep_id = format!("{}:{}", other_constraint.parameter_name, other_constraint.interface_name);
                dependencies.insert(dep_id);
            }
        }

        Ok(dependencies)
    }

    /// Check if one interface extends another
    #[instrument(skip(self), level = "debug")]
    fn interface_extends(&self, interface: &str, base_interface: &str) -> Result<bool, Error> {
        let registry = self.interface_registry.read()
            .map_err(|e| Error::new("lock_error", &format!("Failed to acquire interface registry lock: {}", e), None))?;

        // Check if interface extends base_interface (simplified check)
        // In a full implementation, this would check the interface hierarchy
        Ok(false) // Placeholder - implement actual interface hierarchy checking
    }

    /// Infer a type that satisfies a constraint
    #[instrument(skip(self), level = "debug")]
    fn infer_type_for_constraint(
        &self,
        constraint: &GenericConstraint,
        context_types: &HashMap<String, Type>,
    ) -> Result<Option<Type>, Error> {
        debug!(
            constraint = ?constraint,
            "Inferring type for constraint"
        );

        // Check if we already have a type for this parameter
        if let Some(existing_type) = context_types.get(&constraint.parameter_name) {
            // Verify it satisfies the constraint
            if self.check_constraint(constraint, existing_type)? {
                return Ok(Some(existing_type.clone()));
            }
        }

        // Try to find a type that implements the required interface
        let registry = self.interface_registry.read()
            .map_err(|e| Error::new("lock_error", &format!("Failed to acquire interface registry lock: {}", e), None))?;

        // Get implementations of the required interface
        if let Some(implementations) = registry.implementations().get(&constraint.interface_name) {
            // Return the first implementation that works in this context
            for implementation in implementations {
                // In a full implementation, we'd check context compatibility
                return Ok(Some(implementation.clone()));
            }
        }

        // If no specific implementation found, return a generic type parameter
        Ok(Some(Type::TypeParam(constraint.parameter_name.clone())))
    }

    /// Generate detailed error reports for constraint violations
    #[instrument(skip(self), level = "debug")]
    pub fn generate_error_report(
        &self,
        result: &ConstraintResolutionResult,
    ) -> Result<Vec<Error>, Error> {
        debug!(
            violations_count = result.violations.len(),
            "Generating error report for constraint violations"
        );

        let mut errors = Vec::new();

        for violation in &result.violations {
            // Get available and required methods for better error messages
            let type_checker = self.type_checker.read()
                .map_err(|e| Error::new("TypeChecker", &format!("Failed to acquire type checker lock: {}", e), None))?;

            let available_methods = type_checker.get_type_methods(&violation.concrete_type)
                .unwrap_or_else(|_| Vec::new());
            let required_methods = type_checker.get_interface_methods(&violation.interface_constraint)
                .unwrap_or_else(|_| Vec::new());

            // Create detailed constraint error
            let constraint_error = create_constraint_error(
                &violation.concrete_type,
                &violation.interface_constraint,
                Some(&violation.type_parameter),
                Some(available_methods),
                Some(required_methods),
            );

            errors.push(Error::from(constraint_error));
        }

        Ok(errors)
    }

    /// Propagate constraints through type relationships
    #[instrument(skip(self), level = "debug")]
    pub fn propagate_constraints(
        &mut self,
        constraints: &[GenericConstraint],
        type_relationships: &HashMap<String, Vec<String>>,
    ) -> Result<Vec<GenericConstraint>, Error> {
        debug!(
            constraints_count = constraints.len(),
            relationships_count = type_relationships.len(),
            "Propagating constraints through type relationships"
        );

        let mut propagated_constraints = constraints.to_vec();

        // For each constraint, check if it should propagate to related types
        for constraint in constraints {
            if let Some(related_types) = type_relationships.get(&constraint.parameter_name) {
                for related_type in related_types {
                    // Create propagated constraint
                    let propagated = GenericConstraint::new(
                        constraint.token.clone(),
                        related_type.clone(),
                        constraint.interface_name.clone(),
                    );

                    // Only add if not already present
                    if !propagated_constraints.iter().any(|c| 
                        c.parameter_name == propagated.parameter_name && 
                        c.interface_name == propagated.interface_name
                    ) {
                        propagated_constraints.push(propagated);
                    }
                }
            }
        }

        debug!(
            original_count = constraints.len(),
            propagated_count = propagated_constraints.len(),
            "Constraint propagation complete"
        );

        Ok(propagated_constraints)
    }

    /// Unify constraints to find common solutions
    #[instrument(skip(self), level = "debug")]
    pub fn unify_constraints(
        &mut self,
        constraints_set_a: &[GenericConstraint],
        constraints_set_b: &[GenericConstraint],
    ) -> Result<Vec<GenericConstraint>, Error> {
        debug!(
            set_a_count = constraints_set_a.len(),
            set_b_count = constraints_set_b.len(),
            "Unifying constraint sets"
        );

        let mut unified_constraints = Vec::new();

        // Combine constraints from both sets
        let mut all_constraints = constraints_set_a.to_vec();
        all_constraints.extend_from_slice(constraints_set_b);

        // Group constraints by type parameter
        let mut constraints_by_param: HashMap<String, Vec<&GenericConstraint>> = HashMap::new();
        for constraint in &all_constraints {
            constraints_by_param
                .entry(constraint.parameter_name.clone())
                .or_default()
                .push(constraint);
        }

        // For each type parameter, unify its constraints
        for (param_name, param_constraints) in constraints_by_param {
            let unified = self.unify_parameter_constraints(&param_name, &param_constraints)?;
            unified_constraints.extend(unified);
        }

        debug!(
            unified_count = unified_constraints.len(),
            "Constraint unification complete"
        );

        Ok(unified_constraints)
    }

    /// Unify constraints for a single type parameter
    #[instrument(skip(self), level = "debug")]
    fn unify_parameter_constraints(
        &self,
        param_name: &str,
        constraints: &[&GenericConstraint],
    ) -> Result<Vec<GenericConstraint>, Error> {
        if constraints.is_empty() {
            return Ok(Vec::new());
        }

        if constraints.len() == 1 {
            return Ok(vec![constraints[0].clone()]);
        }

        // Find the most specific constraint (the one that extends all others)
        let mut most_specific: Option<&GenericConstraint> = None;

        for constraint in constraints {
            let mut is_most_specific = true;

            for other_constraint in constraints {
                if constraint.interface_name == other_constraint.interface_name {
                    continue;
                }

                // Check if constraint's interface extends other's interface
                if !self.interface_extends(&constraint.interface_name, &other_constraint.interface_name)? {
                    is_most_specific = false;
                    break;
                }
            }

            if is_most_specific {
                most_specific = Some(constraint);
                break;
            }
        }

        // Return the most specific constraint, or all constraints if no hierarchy found
        if let Some(specific) = most_specific {
            Ok(vec![specific.clone()])
        } else {
            // If no clear hierarchy, keep all constraints
            Ok(constraints.iter().map(|c| (*c).clone()).collect())
        }
    }
}

impl ConstraintDependencyGraph {
    fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            circular_deps: HashSet::new(),
        }
    }

    fn clear(&mut self) {
        self.dependencies.clear();
        self.circular_deps.clear();
    }

    fn add_constraint(&mut self, constraint_id: String, dependencies: HashSet<String>) {
        self.dependencies.insert(constraint_id, dependencies);
    }

    fn topological_sort(&self) -> Result<Vec<String>, Error> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut visiting = HashSet::new();

        for constraint_id in self.dependencies.keys() {
            if !visited.contains(constraint_id) {
                self.visit_constraint(constraint_id, &mut visited, &mut visiting, &mut result)?;
            }
        }

        // Don't reverse - post-order traversal gives us the correct dependency order
        // Dependencies are added before the constraints that depend on them
        Ok(result)
    }

    fn visit_constraint(
        &self,
        constraint_id: &str,
        visited: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
        result: &mut Vec<String>,
    ) -> Result<(), Error> {
        if visiting.contains(constraint_id) {
            return Err(Error::new("ConstraintResolver", &format!("Circular dependency detected: {}", constraint_id), None));
        }

        if visited.contains(constraint_id) {
            return Ok(());
        }

        visiting.insert(constraint_id.to_string());

        if let Some(dependencies) = self.dependencies.get(constraint_id) {
            for dep in dependencies {
                self.visit_constraint(dep, visited, visiting, result)?;
            }
        }

        visiting.remove(constraint_id);
        visited.insert(constraint_id.to_string());
        result.push(constraint_id.to_string());

        Ok(())
    }

    fn check_circular_dependencies(&self) -> Result<(), Vec<String>> {
        let mut visited = HashSet::new();
        let mut visiting = HashSet::new();
        let mut circular = Vec::new();

        for constraint_id in self.dependencies.keys() {
            if !visited.contains(constraint_id) {
                if let Err(cycle) = self.detect_cycle(constraint_id, &mut visited, &mut visiting) {
                    circular.push(cycle);
                }
            }
        }

        if circular.is_empty() {
            Ok(())
        } else {
            Err(circular)
        }
    }

    fn detect_cycle(
        &self,
        constraint_id: &str,
        visited: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
    ) -> Result<(), String> {
        if visiting.contains(constraint_id) {
            return Err(constraint_id.to_string());
        }

        if visited.contains(constraint_id) {
            return Ok(());
        }

        visiting.insert(constraint_id.to_string());

        if let Some(dependencies) = self.dependencies.get(constraint_id) {
            for dep in dependencies {
                self.detect_cycle(dep, visited, visiting)?;
            }
        }

        visiting.remove(constraint_id);
        visited.insert(constraint_id.to_string());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::Token;
    use crate::lexer::TokenType;

    #[test]
    fn test_constraint_resolution_result() {
        let result = ConstraintResolutionResult {
            satisfied: true,
            violations: Vec::new(),
            type_substitutions: HashMap::new(),
            inferred_types: HashMap::new(),
        };

        assert!(result.satisfied);
        assert!(result.violations.is_empty());
    }

    #[test]
    fn test_constraint_violation() {
        let violation = ConstraintViolation {
            type_parameter: "T".to_string(),
            concrete_type: Type::Thicc,
            interface_constraint: "Display".to_string(),
            context: "Test context".to_string(),
            missing_methods: vec!["display".to_string()],
        };

        assert_eq!(violation.type_parameter, "T");
        assert_eq!(violation.interface_constraint, "Display");
        assert!(!violation.missing_methods.is_empty());
    }

    #[test]
    fn test_dependency_graph() {
        let mut graph = ConstraintDependencyGraph::new();
        
        let mut deps = HashSet::new();
        deps.insert("B:Display".to_string());
        
        // A:Comparable depends on B:Display (A depends on B)
        graph.add_constraint("A:Comparable".to_string(), deps);
        graph.add_constraint("B:Display".to_string(), HashSet::new());

        let sorted = graph.topological_sort().unwrap();
        assert_eq!(sorted.len(), 2);
        
        // B:Display should come before A:Comparable since A depends on B
        let b_index = sorted.iter().position(|x| x == "B:Display").unwrap();
        let a_index = sorted.iter().position(|x| x == "A:Comparable").unwrap();
        assert!(b_index < a_index);
    }
}
