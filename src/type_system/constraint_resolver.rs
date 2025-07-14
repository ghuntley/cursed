//! Constraint resolution module for CURSED type system

use crate::error::CursedError;
use super::{TypeExpression, TypeEnvironment, ConstraintBinding, GenericConstraint};
use std::collections::HashMap;

pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED advanced features enabled".to_string())
}

// Missing types for tests
#[derive(Debug, Clone)]
pub struct ConstraintResolver {
    // Store type variable bindings
    bindings: HashMap<String, TypeExpression>,
    // Track visited constraints for cycle detection
    visited_constraints: std::collections::HashSet<String>,
}

impl ConstraintResolver {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            visited_constraints: std::collections::HashSet::new(),
        }
    }

    /// Enhanced constraint resolution for monomorphisation
    pub fn resolve_for_monomorphisation(&mut self, 
                                       type_parameters: &[String],
                                       type_arguments: &[TypeExpression],
                                       constraints: &[GenericConstraint],
                                       env: &TypeEnvironment) -> Result<ConstraintSolution, ConstraintViolation> {
        // Create substitution map from type parameters to arguments
        let mut substitutions = HashMap::new();
        for (param, arg) in type_parameters.iter().zip(type_arguments.iter()) {
            substitutions.insert(param.clone(), arg.clone());
        }

        // Verify all constraints are satisfied with these substitutions
        for constraint in constraints {
            self.verify_constraint_with_substitutions(constraint, &substitutions, env)?;
        }

        // Build enhanced constraint solution
        Ok(ConstraintSolution {
            substitutions,
            success: true,
            is_satisfied: true,
            violations: Vec::new(),
        })
    }

    /// Verify a constraint is satisfied with given type substitutions
    fn verify_constraint_with_substitutions(&self,
                                           constraint: &GenericConstraint,
                                           substitutions: &HashMap<String, TypeExpression>,
                                           env: &TypeEnvironment) -> Result<(), ConstraintViolation> {
        // For each type parameter in the constraint
        for param in &constraint.type_parameters {
            if let Some(concrete_type) = substitutions.get(param) {
                // Check that concrete type satisfies all bounds
                for bound in &constraint.bounds {
                    if !self.type_satisfies_bound(concrete_type, bound, env)? {
                        return Err(ConstraintViolation {
                            reason: ViolationReason::MissingImplementation,
                            context: format!("Type '{}' does not satisfy bound '{}' for parameter '{}'",
                                           concrete_type.name.as_ref().unwrap_or(&"unknown".to_string()),
                                           bound,
                                           param),
                        });
                    }
                }
            } else {
                return Err(ConstraintViolation {
                    reason: ViolationReason::MissingInterface(param.clone()),
                    context: format!("No substitution provided for type parameter '{}'", param),
                });
            }
        }

        Ok(())
    }

    /// Check if a concrete type satisfies a given bound
    fn type_satisfies_bound(&self, 
                           concrete_type: &TypeExpression,
                           bound: &str,
                           env: &TypeEnvironment) -> Result<bool, ConstraintViolation> {
        // Built-in bounds
        match bound {
            "any" => return Ok(true), // All types satisfy 'any'
            "comparable" => return Ok(self.is_comparable_type(concrete_type)),
            "numeric" => return Ok(self.is_numeric_type(concrete_type)),
            "ordered" => return Ok(self.is_ordered_type(concrete_type)),
            _ => {}
        }

        // Interface bounds
        if let Some(type_name) = &concrete_type.name {
            if let Some(concrete_type_def) = env.get_type(type_name) {
                if let Some(bound_interface) = env.get_type(bound) {
                    if bound_interface.kind == super::TypeKind::Interface {
                        return Ok(self.check_interface_implementation_internal(concrete_type_def, bound_interface, env));
                    }
                }
            }
        }

        // Fallback: check if type has required methods/properties
        Ok(self.type_implements_bound(concrete_type, bound, env))
    }

    /// Check if type is comparable (supports ==, !=)
    fn is_comparable_type(&self, type_expr: &TypeExpression) -> bool {
        matches!(type_expr.name.as_deref(),
                Some("normie") | Some("smol") | Some("mid") | Some("thicc") |
                Some("drip") | Some("meal") | Some("snack") |
                Some("lit") | Some("tea") | Some("sip"))
    }

    /// Check if type is numeric (supports +, -, *, /)
    fn is_numeric_type(&self, type_expr: &TypeExpression) -> bool {
        matches!(type_expr.name.as_deref(),
                Some("normie") | Some("smol") | Some("mid") | Some("thicc") |
                Some("drip") | Some("meal") | Some("snack"))
    }

    /// Check if type is ordered (supports <, >, <=, >=)
    fn is_ordered_type(&self, type_expr: &TypeExpression) -> bool {
        self.is_numeric_type(type_expr) ||
        matches!(type_expr.name.as_deref(), Some("tea") | Some("sip"))
    }
    
    pub fn validate_constraint(&self, constraint: &GenericConstraint, env: &TypeEnvironment) -> Result<(), ConstraintViolation> {
        // Check if constraint refers to valid types
        for bound in &constraint.bounds {
            if env.get_type(bound).is_none() {
                return Err(ConstraintViolation {
                    reason: ViolationReason::MissingInterface(bound.clone()),
                    context: format!("Constraint '{}' refers to unknown type '{}'", constraint.constraint_name, bound),
                });
            }
        }
        
        // Check for circular dependencies in constraints
        if self.has_circular_dependency(constraint, env) {
            return Err(ConstraintViolation {
                reason: ViolationReason::CircularDependency,
                context: format!("Circular dependency detected in constraint '{}'", constraint.constraint_name),
            });
        }
        
        Ok(())
    }
    
    pub fn check_satisfaction(&self, type_expr: &TypeExpression, constraints: &[GenericConstraint], env: &TypeEnvironment) -> Result<bool, ConstraintViolation> {
        for constraint in constraints {
            if !self.satisfies_constraint(type_expr, constraint, env)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
    
    pub fn resolve_constraints(&self, context: &super::ConstraintContext, env: &TypeEnvironment) -> Result<ConstraintSolution, ConstraintViolation> {
        let mut substitutions = HashMap::new();
        let mut violations = Vec::new();
        let mut success = true;
        
        // Process each constraint binding
        for binding in &context.active_constraints {
            match self.resolve_single_constraint(&binding.constraint, &binding.bound_types, env) {
                Ok(subs) => {
                    substitutions.extend(subs);
                }
                Err(violation) => {
                    violations.push(violation);
                    success = false;
                }
            }
        }
        
        // Check for conflicts in substitutions
        if self.has_conflicting_substitutions(&substitutions) {
            violations.push(ConstraintViolation {
                reason: ViolationReason::IncompatibleConstraints,
                context: "Conflicting type substitutions detected".to_string(),
            });
            success = false;
        }
        
        Ok(ConstraintSolution {
            substitutions,
            success,
            is_satisfied: success && violations.is_empty(),
            violations,
        })
    }
    
    pub fn determine_violation_reason(&self, constraint: &GenericConstraint, type_expr: &TypeExpression, env: &TypeEnvironment) -> ViolationReason {
        // Check if type exists
        if let Some(type_name) = &type_expr.name {
            if env.get_type(type_name).is_none() {
                return ViolationReason::MissingInterface(type_name.clone());
            }
        }
        
        // Check if required methods/traits are implemented
        for bound in &constraint.bounds {
            if !self.type_implements_bound(type_expr, bound, env) {
                return ViolationReason::MissingImplementation;
            }
        }
        
        ViolationReason::TypeMismatch
    }
    
    pub fn generate_suggested_fixes(&self, constraint: &GenericConstraint, type_expr: &TypeExpression) -> Vec<String> {
        let mut fixes = Vec::new();
        
        if let Some(type_name) = &type_expr.name {
            for bound in &constraint.bounds {
                fixes.push(format!("Implement {} for {}", bound, type_name));
                fixes.push(format!("Add {} constraint to {}", bound, type_name));
            }
        }
        
        if fixes.is_empty() {
            fixes.push("Consider implementing the missing trait".to_string());
        }
        
        fixes
    }
    
    // Helper methods
    fn has_circular_dependency(&self, constraint: &GenericConstraint, env: &TypeEnvironment) -> bool {
        // Create a unique key for this constraint
        let constraint_key = format!("{}:{}", constraint.constraint_name, constraint.bounds.join("+"));
        
        // If we've already visited this constraint in the current path, we have a cycle
        if self.visited_constraints.contains(&constraint_key) {
            return true;
        }
        
        // Check dependencies recursively
        for bound in &constraint.bounds {
            if let Some(type_def) = env.get_type(bound) {
                // Check if this type has constraints that could create cycles
                for method in &type_def.methods {
                    // Simple heuristic: if method name contains constraint name, potential cycle
                    if method.name.contains(&constraint.constraint_name) {
                        return true;
                    }
                }
            }
        }
        
        false
    }
    
    fn satisfies_constraint(&self, type_expr: &TypeExpression, constraint: &GenericConstraint, env: &TypeEnvironment) -> Result<bool, ConstraintViolation> {
        // Check if type satisfies all bounds in the constraint
        for bound in &constraint.bounds {
            if !self.type_implements_bound(type_expr, bound, env) {
                return Ok(false);
            }
        }
        Ok(true)
    }
    
    fn type_implements_bound(&self, type_expr: &TypeExpression, bound: &str, env: &TypeEnvironment) -> bool {
        if let Some(type_name) = &type_expr.name {
            if let Some(type_def) = env.get_type(type_name) {
                // Check if bound is an interface that this type implements
                if let Some(interface_def) = env.get_type(bound) {
                    if interface_def.kind == super::TypeKind::Interface {
                        return self.check_interface_implementation_internal(type_def, interface_def, env);
                    }
                }
                // Fallback: Check if type has methods required by bound
                return type_def.methods.iter().any(|m| m.name.contains(bound));
            }
        }
        false
    }
    
    /// Internal method to check interface implementation without TypeChecker
    fn check_interface_implementation_internal(&self, type_def: &super::TypeDefinition, interface_def: &super::TypeDefinition, env: &TypeEnvironment) -> bool {
        // Check if all interface methods are implemented by the type
        for interface_method in &interface_def.methods {
            if !self.type_has_compatible_method_internal(type_def, interface_method, env) {
                return false;
            }
        }
        true
    }
    
    /// Internal method to check method compatibility without TypeChecker
    fn type_has_compatible_method_internal(&self, type_def: &super::TypeDefinition, interface_method: &super::MethodSignature, env: &TypeEnvironment) -> bool {
        // Find method with matching name
        let type_method = type_def.methods.iter()
            .find(|m| m.name == interface_method.name);
        
        let type_method = match type_method {
            Some(method) => method,
            None => return false, // Method not found
        };
        
        // Check parameter compatibility
        if !self.check_method_signature_compatibility_internal(type_method, interface_method, env) {
            return false;
        }
        
        true
    }
    
    /// Internal method to check method signature compatibility
    fn check_method_signature_compatibility_internal(&self, impl_method: &super::MethodSignature, interface_method: &super::MethodSignature, _env: &TypeEnvironment) -> bool {
        // Check parameter count
        if impl_method.parameters.len() != interface_method.parameters.len() {
            return false;
        }
        
        // Check parameter types
        for (impl_param, interface_param) in impl_method.parameters.iter().zip(interface_method.parameters.iter()) {
            if !self.types_are_compatible_internal(impl_param, interface_param) {
                return false;
            }
        }
        
        // Check return type compatibility
        match (&impl_method.return_type, &interface_method.return_type) {
            (Some(impl_ret), Some(interface_ret)) => {
                if !self.types_are_compatible_internal(impl_ret, interface_ret) {
                    return false;
                }
            }
            (None, None) => {}, // Both void
            _ => return false, // Mismatch: one void, one not
        }
        
        true
    }
    
    /// Internal method to check type compatibility
    fn types_are_compatible_internal(&self, type1: &TypeExpression, type2: &TypeExpression) -> bool {
        // Direct name matching
        if type1.name == type2.name {
            // Check parameter compatibility recursively
            if type1.parameters.len() != type2.parameters.len() {
                return false;
            }
            
            for (param1, param2) in type1.parameters.iter().zip(type2.parameters.iter()) {
                if !self.types_are_compatible_internal(param1, param2) {
                    return false;
                }
            }
            
            return true;
        }
        
        false
    }
    
    /// Check interface satisfaction for constraint resolution
    pub fn check_interface_satisfaction(&self, type_expr: &TypeExpression, interface_name: &str, env: &TypeEnvironment) -> Result<bool, ConstraintViolation> {
        if let Some(type_name) = &type_expr.name {
            if let Some(type_def) = env.get_type(type_name) {
                if let Some(interface_def) = env.get_type(interface_name) {
                    if interface_def.kind == super::TypeKind::Interface {
                        return Ok(self.check_interface_implementation_internal(type_def, interface_def, env));
                    }
                }
            }
        }
        
        Ok(false)
    }
    
    /// Verify all interface constraints are satisfied
    pub fn verify_interface_constraints(&self, constraints: &[GenericConstraint], type_bindings: &std::collections::HashMap<String, TypeExpression>, env: &TypeEnvironment) -> Result<(), ConstraintViolation> {
        for constraint in constraints {
            for (type_param, bound) in constraint.type_parameters.iter().zip(constraint.bounds.iter()) {
                if let Some(bound_type) = type_bindings.get(type_param) {
                    if !self.check_interface_satisfaction(bound_type, bound, env)? {
                        return Err(ConstraintViolation {
                            reason: ViolationReason::MissingImplementation,
                            context: format!("Type '{}' does not implement interface '{}'", 
                                           bound_type.name.as_ref().unwrap_or(&"unknown".to_string()), 
                                           bound),
                        });
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn resolve_single_constraint(&self, constraint: &GenericConstraint, bound_types: &[String], env: &TypeEnvironment) -> Result<HashMap<String, TypeExpression>, ConstraintViolation> {
        let mut substitutions = HashMap::new();
        
        // Simple constraint resolution - bind type parameters to bound types
        for (i, param) in constraint.type_parameters.iter().enumerate() {
            if let Some(bound_type) = bound_types.get(i) {
                if let Some(_type_def) = env.get_type(bound_type) {
                    substitutions.insert(param.clone(), TypeExpression::named(bound_type));
                } else {
                    return Err(ConstraintViolation {
                        reason: ViolationReason::MissingInterface(bound_type.clone()),
                        context: format!("Unknown type in constraint: {}", bound_type),
                    });
                }
            }
        }
        
        Ok(substitutions)
    }
    
    fn has_conflicting_substitutions(&self, substitutions: &HashMap<String, TypeExpression>) -> bool {
        // Check if the same type variable is bound to different types
        for (var, binding) in substitutions {
            if let Some(existing_binding) = self.bindings.get(var) {
                // Simple type equality check - names must match
                if let (Some(existing_name), Some(new_name)) = (&existing_binding.name, &binding.name) {
                    if existing_name != new_name {
                        return true; // Conflicting bindings
                    }
                }
            }
        }
        
        // Check for direct conflicts within the substitution set
        let mut seen_variables = std::collections::HashSet::new();
        for (var, binding) in substitutions {
            if seen_variables.contains(var) {
                return true;
            }
            seen_variables.insert(var.clone());
            
            // Check if binding creates a cycle
            if self.binding_creates_cycle(var, binding, substitutions) {
                return true;
            }
        }
        
        false
    }
    
    fn binding_creates_cycle(&self, var: &str, binding: &TypeExpression, substitutions: &HashMap<String, TypeExpression>) -> bool {
        // Simple cycle detection: check if var appears in the binding
        if let Some(binding_name) = &binding.name {
            if binding_name == var {
                return true;
            }
            
            // Check if any substitutions create indirect cycles
            if let Some(indirect_binding) = substitutions.get(binding_name) {
                return self.binding_creates_cycle(var, indirect_binding, substitutions);
            }
        }
        
        // Check parameters for cycles
        for param in &binding.parameters {
            if self.binding_creates_cycle(var, param, substitutions) {
                return true;
            }
        }
        
        false
    }
}

#[derive(Debug, Clone)]
pub struct ConstraintSolution {
    pub substitutions: HashMap<String, TypeExpression>,
    pub success: bool,
    pub is_satisfied: bool,
    pub violations: Vec<ConstraintViolation>,
}

#[derive(Debug, Clone)]
pub struct ConstraintViolation {
    pub reason: ViolationReason,
    pub context: String,
}

#[derive(Debug, Clone)]
pub enum ViolationReason {
    TypeMismatch,
    MissingImplementation,
    CircularDependency,
    IncompatibleConstraints,
    MissingInterface(String),
}

#[derive(Debug, Clone)]
pub struct TypeUnifier {
    // Store current substitutions during unification
    substitutions: HashMap<String, TypeExpression>,
}

impl TypeUnifier {
    pub fn new() -> Self {
        Self {
            substitutions: HashMap::new(),
        }
    }
    
    pub fn unify(&mut self, t1: &TypeExpression, t2: &TypeExpression) -> Result<HashMap<String, TypeExpression>, ConstraintViolation> {
        let mut substitutions = HashMap::new();
        self.unify_recursive(t1, t2, &mut substitutions)?;
        Ok(substitutions)
    }
    
    fn unify_recursive(&self, t1: &TypeExpression, t2: &TypeExpression, substitutions: &mut HashMap<String, TypeExpression>) -> Result<(), ConstraintViolation> {
        match (&t1.name, &t2.name) {
            // Both have names - either same type or one is a type variable
            (Some(n1), Some(n2)) => {
                if n1 == n2 {
                    // Same named types - unify parameters
                    if t1.parameters.len() != t2.parameters.len() {
                        return Err(ConstraintViolation {
                            reason: ViolationReason::TypeMismatch,
                            context: format!("Parameter count mismatch: {} vs {}", t1.parameters.len(), t2.parameters.len()),
                        });
                    }
                    
                    for (p1, p2) in t1.parameters.iter().zip(t2.parameters.iter()) {
                        self.unify_recursive(p1, p2, substitutions)?;
                    }
                    
                    // Unify return types if both are function types
                    match (&t1.return_type, &t2.return_type) {
                        (Some(rt1), Some(rt2)) => self.unify_recursive(rt1, rt2, substitutions)?,
                        (None, None) => {},
                        _ => return Err(ConstraintViolation {
                            reason: ViolationReason::TypeMismatch,
                            context: "Function return type mismatch".to_string(),
                        }),
                    }
                } else if self.is_type_variable(n1) {
                    // n1 is a type variable, bind it to t2
                    if self.occurs_check(n1, t2) {
                        return Err(ConstraintViolation {
                            reason: ViolationReason::CircularDependency,
                            context: format!("Occurs check failed: {} occurs in {:?}", n1, t2),
                        });
                    }
                    substitutions.insert(n1.clone(), t2.clone());
                } else if self.is_type_variable(n2) {
                    // n2 is a type variable, bind it to t1
                    if self.occurs_check(n2, t1) {
                        return Err(ConstraintViolation {
                            reason: ViolationReason::CircularDependency,
                            context: format!("Occurs check failed: {} occurs in {:?}", n2, t1),
                        });
                    }
                    substitutions.insert(n2.clone(), t1.clone());
                } else {
                    // Different concrete types - cannot unify
                    return Err(ConstraintViolation {
                        reason: ViolationReason::TypeMismatch,
                        context: format!("Cannot unify {} with {}", n1, n2),
                    });
                }
            }
            // One has no name - should not happen in well-formed types
            _ => {
                return Err(ConstraintViolation {
                    reason: ViolationReason::TypeMismatch,
                    context: "Cannot unify unnamed types".to_string(),
                });
            }
        }
        
        Ok(())
    }
    
    fn is_type_variable(&self, name: &str) -> bool {
        // Type variables start with uppercase T followed by digits
        name.starts_with('T') && name[1..].chars().all(|c| c.is_ascii_digit())
    }
    
    fn occurs_check(&self, var: &str, type_expr: &TypeExpression) -> bool {
        if let Some(name) = &type_expr.name {
            if name == var {
                return true;
            }
        }
        
        // Check in parameters
        for param in &type_expr.parameters {
            if self.occurs_check(var, param) {
                return true;
            }
        }
        
        // Check in return type
        if let Some(return_type) = &type_expr.return_type {
            if self.occurs_check(var, return_type) {
                return true;
            }
        }
        
        false
    }
}

#[derive(Debug, Clone)]
pub struct ConstraintPropagator {
    // Track constraint resolution state
    resolved_constraints: std::collections::HashSet<String>,
    pending_constraints: Vec<String>,
}

impl ConstraintPropagator {
    pub fn new() -> Self {
        Self {
            resolved_constraints: std::collections::HashSet::new(),
            pending_constraints: Vec::new(),
        }
    }
    
    pub fn resolve_final(&mut self, constraints: &[GenericConstraint]) -> Result<HashMap<String, TypeExpression>, ConstraintViolation> {
        let mut final_substitutions = HashMap::new();
        
        // Simple resolution strategy: process constraints in order
        for constraint in constraints {
            let constraint_id = format!("{}:{}", constraint.constraint_name, constraint.bounds.join("+"));
            
            if !self.resolved_constraints.contains(&constraint_id) {
                // For basic functionality, just bind type parameters to their bounds
                for (i, param) in constraint.type_parameters.iter().enumerate() {
                    if let Some(bound) = constraint.bounds.get(i) {
                        final_substitutions.insert(param.clone(), TypeExpression::named(bound));
                    }
                }
                
                self.resolved_constraints.insert(constraint_id);
            }
        }
        
        Ok(final_substitutions)
    }
    
    pub fn build_constraint_graph(&self, bindings: &[ConstraintBinding]) -> Result<ConstraintGraph, ConstraintViolation> {
        let mut graph = ConstraintGraph::new();
        
        // Create nodes for each constraint binding
        for (i, binding) in bindings.iter().enumerate() {
            let node = ConstraintNode {
                id: format!("constraint_{}", i),
                constraint: binding.constraint.clone(),
                bound_types: binding.bound_types.clone(),
                status: binding.satisfaction_status.clone(),
                dependencies: self.find_dependencies(&binding.constraint, bindings),
                binding: binding.clone(),
            };
            graph.add_node(node);
        }
        
        Ok(graph)
    }
    
    fn find_dependencies(&self, constraint: &GenericConstraint, bindings: &[ConstraintBinding]) -> Vec<String> {
        let mut dependencies = Vec::new();
        
        // Find constraints that this constraint depends on
        for (i, binding) in bindings.iter().enumerate() {
            if self.constraint_depends_on(constraint, &binding.constraint) {
                dependencies.push(format!("constraint_{}", i));
            }
        }
        
        dependencies
    }
    
    fn constraint_depends_on(&self, constraint: &GenericConstraint, other: &GenericConstraint) -> bool {
        // Check if any of constraint's type parameters appear in other's bounds
        for param in &constraint.type_parameters {
            if other.bounds.contains(param) {
                return true;
            }
        }
        
        // Check if any of constraint's bounds are type parameters of other
        for bound in &constraint.bounds {
            if other.type_parameters.contains(bound) {
                return true;
            }
        }
        
        false
    }
}

#[derive(Debug, Clone)]
pub struct ConstraintGraph {
    pub nodes: Vec<ConstraintNode>,
}

impl ConstraintGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }
    
    pub fn add_node(&mut self, node: ConstraintNode) {
        self.nodes.push(node);
    }
    
    pub fn topological_sort(&self) -> Result<Vec<String>, ConstraintViolation> {
        let mut visited = std::collections::HashSet::new();
        let mut temp_mark = std::collections::HashSet::new();
        let mut result = Vec::new();
        
        // Kahn's algorithm for topological sorting
        for node in &self.nodes {
            if !visited.contains(&node.id) {
                self.visit_node(&node.id, &mut visited, &mut temp_mark, &mut result)?;
            }
        }
        
        result.reverse();
        Ok(result)
    }
    
    fn visit_node(&self, node_id: &str, visited: &mut std::collections::HashSet<String>, 
                  temp_mark: &mut std::collections::HashSet<String>, result: &mut Vec<String>) -> Result<(), ConstraintViolation> {
        if temp_mark.contains(node_id) {
            return Err(ConstraintViolation {
                reason: ViolationReason::CircularDependency,
                context: format!("Circular dependency detected involving node {}", node_id),
            });
        }
        
        if visited.contains(node_id) {
            return Ok(());
        }
        
        temp_mark.insert(node_id.to_string());
        
        // Visit all dependencies first
        if let Some(node) = self.nodes.iter().find(|n| n.id == node_id) {
            for dep in &node.dependencies {
                self.visit_node(dep, visited, temp_mark, result)?;
            }
        }
        
        temp_mark.remove(node_id);
        visited.insert(node_id.to_string());
        result.push(node_id.to_string());
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ConstraintNode {
    pub id: String,
    pub constraint: GenericConstraint,
    pub bound_types: Vec<String>,
    pub status: super::ConstraintStatus,
    pub dependencies: Vec<String>,
    pub binding: ConstraintBinding,
}
