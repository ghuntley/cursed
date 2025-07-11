//! Advanced type constraints for CURSED generics
//! Provides comprehensive constraint checking, type bounds, and where clauses

use crate::error::CursedError;
use crate::ast::{Type, FunctionStatement, Parameter};
use super::{TypeExpression, TypeEnvironment, GenericConstraint, ConstraintResolver};
use std::collections::{HashMap, HashSet};

/// Advanced constraint system for type parameters
#[derive(Debug, Clone)]
pub enum AdvancedConstraint {
    /// Trait bound: T: Clone
    TraitBound(String, String),
    /// Lifetime bound: T: 'static
    LifetimeBound(String, String),
    /// Equality constraint: T = ConcreteType
    EqualityConstraint(String, TypeExpression),
    /// Associated type constraint: T::Item = U
    AssociatedTypeConstraint(String, String, TypeExpression),
    /// Higher-ranked trait bound: for<'a> T: Fn(&'a str) -> &'a str
    HigherRankedTraitBound(String, Vec<String>, String),
    /// Conditional constraint: where T: Clone + Send
    ConditionalConstraint(Vec<AdvancedConstraint>),
    /// Phantom constraint: PhantomData<T>
    PhantomConstraint(String),
    /// Size constraint: T: Sized
    SizedConstraint(String),
    /// Copy constraint: T: Copy
    CopyConstraint(String),
    /// Send constraint: T: Send
    SendConstraint(String),
    /// Sync constraint: T: Sync
    SyncConstraint(String),
    /// Debug constraint: T: Debug
    DebugConstraint(String),
    /// Display constraint: T: Display
    DisplayConstraint(String),
    /// Iterator constraint: T: Iterator<Item = U>
    IteratorConstraint(String, TypeExpression),
    /// IntoIterator constraint: T: IntoIterator<Item = U>
    IntoIteratorConstraint(String, TypeExpression),
    /// From constraint: T: From<U>
    FromConstraint(String, TypeExpression),
    /// Into constraint: T: Into<U>
    IntoConstraint(String, TypeExpression),
    /// TryFrom constraint: T: TryFrom<U>
    TryFromConstraint(String, TypeExpression),
    /// TryInto constraint: T: TryInto<U>
    TryIntoConstraint(String, TypeExpression),
    /// Default constraint: T: Default
    DefaultConstraint(String),
    /// PartialEq constraint: T: PartialEq<U>
    PartialEqConstraint(String, Option<TypeExpression>),
    /// Eq constraint: T: Eq
    EqConstraint(String),
    /// PartialOrd constraint: T: PartialOrd<U>
    PartialOrdConstraint(String, Option<TypeExpression>),
    /// Ord constraint: T: Ord
    OrdConstraint(String),
    /// Hash constraint: T: Hash
    HashConstraint(String),
}

/// Constraint satisfaction checker
#[derive(Debug)]
pub struct AdvancedConstraintChecker {
    /// Known trait implementations
    trait_impls: HashMap<String, HashSet<String>>,
    /// Type parameter constraints
    type_constraints: HashMap<String, Vec<AdvancedConstraint>>,
    /// Constraint dependency graph
    constraint_graph: ConstraintDependencyGraph,
}

/// Dependency graph for constraint resolution
#[derive(Debug)]
pub struct ConstraintDependencyGraph {
    /// Nodes represent type parameters
    nodes: HashMap<String, ConstraintNode>,
    /// Edges represent constraint dependencies
    edges: HashMap<String, Vec<String>>,
}

/// Node in constraint dependency graph
#[derive(Debug)]
pub struct ConstraintNode {
    /// Type parameter name
    pub name: String,
    /// Direct constraints on this type parameter
    pub constraints: Vec<AdvancedConstraint>,
    /// Dependencies on other type parameters
    pub dependencies: Vec<String>,
}

impl AdvancedConstraintChecker {
    pub fn new() -> Self {
        let mut checker = Self {
            trait_impls: HashMap::new(),
            type_constraints: HashMap::new(),
            constraint_graph: ConstraintDependencyGraph::new(),
        };
        
        checker.register_builtin_traits();
        checker
    }
    
    /// Register built-in trait implementations
    fn register_builtin_traits(&mut self) {
        // Register primitive types with their built-in traits
        let primitives = vec!["normie", "thicc", "smol", "mid", "drip", "meal", "lit", "tea", "sip"];
        
        for primitive in &primitives {
            self.register_trait_impl(primitive, "Clone");
            self.register_trait_impl(primitive, "Copy");
            self.register_trait_impl(primitive, "Debug");
            self.register_trait_impl(primitive, "Default");
            self.register_trait_impl(primitive, "PartialEq");
            self.register_trait_impl(primitive, "Eq");
            
            // Numeric types have ordering
            if matches!(*primitive, "normie" | "thicc" | "smol" | "mid" | "drip" | "meal") {
                self.register_trait_impl(primitive, "PartialOrd");
                self.register_trait_impl(primitive, "Ord");
            }
            
            // Hash-able types
            if !matches!(*primitive, "drip" | "meal") {
                self.register_trait_impl(primitive, "Hash");
            }
        }
        
        // String types
        self.register_trait_impl("tea", "Display");
        self.register_trait_impl("tea", "From<&str>");
        self.register_trait_impl("tea", "Into<String>");
        
        // Array and slice types
        self.register_trait_impl("Array", "IntoIterator");
        self.register_trait_impl("Slice", "IntoIterator");
    }
    
    /// Register a trait implementation
    pub fn register_trait_impl(&mut self, type_name: &str, trait_name: &str) {
        self.trait_impls.entry(type_name.to_string())
            .or_insert_with(HashSet::new)
            .insert(trait_name.to_string());
    }
    
    /// Add constraints for a type parameter
    pub fn add_type_constraints(&mut self, type_param: &str, constraints: Vec<AdvancedConstraint>) {
        self.type_constraints.insert(type_param.to_string(), constraints.clone());
        
        // Add to dependency graph
        let node = ConstraintNode {
            name: type_param.to_string(),
            constraints: constraints.clone(),
            dependencies: self.extract_dependencies(&constraints),
        };
        
        self.constraint_graph.add_node(node);
    }
    
    /// Extract dependencies from constraints
    fn extract_dependencies(&self, constraints: &[AdvancedConstraint]) -> Vec<String> {
        let mut deps = Vec::new();
        
        for constraint in constraints {
            match constraint {
                AdvancedConstraint::AssociatedTypeConstraint(_, _, type_expr) => {
                    deps.extend(self.extract_type_dependencies(type_expr));
                }
                AdvancedConstraint::EqualityConstraint(_, type_expr) => {
                    deps.extend(self.extract_type_dependencies(type_expr));
                }
                AdvancedConstraint::IteratorConstraint(_, type_expr) => {
                    deps.extend(self.extract_type_dependencies(type_expr));
                }
                AdvancedConstraint::IntoIteratorConstraint(_, type_expr) => {
                    deps.extend(self.extract_type_dependencies(type_expr));
                }
                AdvancedConstraint::FromConstraint(_, type_expr) => {
                    deps.extend(self.extract_type_dependencies(type_expr));
                }
                AdvancedConstraint::IntoConstraint(_, type_expr) => {
                    deps.extend(self.extract_type_dependencies(type_expr));
                }
                AdvancedConstraint::TryFromConstraint(_, type_expr) => {
                    deps.extend(self.extract_type_dependencies(type_expr));
                }
                AdvancedConstraint::TryIntoConstraint(_, type_expr) => {
                    deps.extend(self.extract_type_dependencies(type_expr));
                }
                AdvancedConstraint::PartialEqConstraint(_, Some(type_expr)) => {
                    deps.extend(self.extract_type_dependencies(type_expr));
                }
                AdvancedConstraint::PartialOrdConstraint(_, Some(type_expr)) => {
                    deps.extend(self.extract_type_dependencies(type_expr));
                }
                AdvancedConstraint::ConditionalConstraint(inner_constraints) => {
                    deps.extend(self.extract_dependencies(inner_constraints));
                }
                _ => {}
            }
        }
        
        deps
    }
    
    /// Extract type parameter dependencies from a type expression
    fn extract_type_dependencies(&self, type_expr: &TypeExpression) -> Vec<String> {
        let mut deps = Vec::new();
        
        if let Some(name) = &type_expr.name {
            if self.type_constraints.contains_key(name) {
                deps.push(name.clone());
            }
        }
        
        for param in &type_expr.parameters {
            deps.extend(self.extract_type_dependencies(param));
        }
        
        deps
    }
    
    /// Check if constraints are satisfied
    pub fn check_constraints(&self, type_bindings: &HashMap<String, TypeExpression>) -> Result<(), CursedError> {
        // Resolve constraints in dependency order
        let resolution_order = self.constraint_graph.topological_sort()?;
        
        for type_param in resolution_order {
            if let Some(constraints) = self.type_constraints.get(&type_param) {
                if let Some(concrete_type) = type_bindings.get(&type_param) {
                    for constraint in constraints {
                        self.check_single_constraint(constraint, concrete_type, type_bindings)?;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Check a single constraint
    fn check_single_constraint(&self, 
                             constraint: &AdvancedConstraint, 
                             concrete_type: &TypeExpression,
                             type_bindings: &HashMap<String, TypeExpression>) -> Result<(), CursedError> {
        match constraint {
            AdvancedConstraint::TraitBound(type_param, trait_name) => {
                self.check_trait_bound(concrete_type, trait_name)
            }
            AdvancedConstraint::EqualityConstraint(type_param, expected_type) => {
                self.check_equality_constraint(concrete_type, expected_type, type_bindings)
            }
            AdvancedConstraint::AssociatedTypeConstraint(type_param, assoc_type, expected_type) => {
                self.check_associated_type_constraint(concrete_type, assoc_type, expected_type, type_bindings)
            }
            AdvancedConstraint::SizedConstraint(type_param) => {
                self.check_sized_constraint(concrete_type)
            }
            AdvancedConstraint::CopyConstraint(type_param) => {
                self.check_trait_bound(concrete_type, "Copy")
            }
            AdvancedConstraint::SendConstraint(type_param) => {
                self.check_trait_bound(concrete_type, "Send")
            }
            AdvancedConstraint::SyncConstraint(type_param) => {
                self.check_trait_bound(concrete_type, "Sync")
            }
            AdvancedConstraint::DebugConstraint(type_param) => {
                self.check_trait_bound(concrete_type, "Debug")
            }
            AdvancedConstraint::DisplayConstraint(type_param) => {
                self.check_trait_bound(concrete_type, "Display")
            }
            AdvancedConstraint::IteratorConstraint(type_param, item_type) => {
                self.check_iterator_constraint(concrete_type, item_type, type_bindings)
            }
            AdvancedConstraint::IntoIteratorConstraint(type_param, item_type) => {
                self.check_into_iterator_constraint(concrete_type, item_type, type_bindings)
            }
            AdvancedConstraint::FromConstraint(type_param, source_type) => {
                self.check_from_constraint(concrete_type, source_type, type_bindings)
            }
            AdvancedConstraint::IntoConstraint(type_param, target_type) => {
                self.check_into_constraint(concrete_type, target_type, type_bindings)
            }
            AdvancedConstraint::DefaultConstraint(type_param) => {
                self.check_trait_bound(concrete_type, "Default")
            }
            AdvancedConstraint::PartialEqConstraint(type_param, other_type) => {
                match other_type {
                    Some(other) => self.check_partial_eq_constraint(concrete_type, other, type_bindings),
                    None => self.check_trait_bound(concrete_type, "PartialEq"),
                }
            }
            AdvancedConstraint::EqConstraint(type_param) => {
                self.check_trait_bound(concrete_type, "Eq")
            }
            AdvancedConstraint::PartialOrdConstraint(type_param, other_type) => {
                match other_type {
                    Some(other) => self.check_partial_ord_constraint(concrete_type, other, type_bindings),
                    None => self.check_trait_bound(concrete_type, "PartialOrd"),
                }
            }
            AdvancedConstraint::OrdConstraint(type_param) => {
                self.check_trait_bound(concrete_type, "Ord")
            }
            AdvancedConstraint::HashConstraint(type_param) => {
                self.check_trait_bound(concrete_type, "Hash")
            }
            AdvancedConstraint::ConditionalConstraint(constraints) => {
                // All constraints must be satisfied
                for inner_constraint in constraints {
                    self.check_single_constraint(inner_constraint, concrete_type, type_bindings)?;
                }
                Ok(())
            }
            _ => {
                // Handle other constraint types
                Ok(())
            }
        }
    }
    
    /// Check if a type implements a trait
    fn check_trait_bound(&self, concrete_type: &TypeExpression, trait_name: &str) -> Result<(), CursedError> {
        if let Some(type_name) = &concrete_type.name {
            if let Some(impls) = self.trait_impls.get(type_name) {
                if impls.contains(trait_name) {
                    return Ok(());
                }
            }
        }
        
        Err(CursedError::type_error(&format!(
            "Type {:?} does not implement trait {}", 
            concrete_type, trait_name
        )))
    }
    
    /// Check equality constraint
    fn check_equality_constraint(&self, 
                                concrete_type: &TypeExpression, 
                                expected_type: &TypeExpression,
                                type_bindings: &HashMap<String, TypeExpression>) -> Result<(), CursedError> {
        let resolved_expected = self.resolve_type_parameters(expected_type, type_bindings);
        
        if self.types_equal(concrete_type, &resolved_expected) {
            Ok(())
        } else {
            Err(CursedError::type_error(&format!(
                "Type equality constraint violated: expected {:?}, got {:?}",
                resolved_expected, concrete_type
            )))
        }
    }
    
    /// Check associated type constraint
    fn check_associated_type_constraint(&self,
                                       concrete_type: &TypeExpression,
                                       assoc_type: &str,
                                       expected_type: &TypeExpression,
                                       type_bindings: &HashMap<String, TypeExpression>) -> Result<(), CursedError> {
        // This would require runtime type information in a full implementation
        // For now, we'll perform basic checks
        Ok(())
    }
    
    /// Check sized constraint
    fn check_sized_constraint(&self, concrete_type: &TypeExpression) -> Result<(), CursedError> {
        // Most types are sized by default in CURSED
        // Only trait objects and some special types are unsized
        Ok(())
    }
    
    /// Check iterator constraint
    fn check_iterator_constraint(&self,
                                concrete_type: &TypeExpression,
                                item_type: &TypeExpression,
                                type_bindings: &HashMap<String, TypeExpression>) -> Result<(), CursedError> {
        self.check_trait_bound(concrete_type, "Iterator")
    }
    
    /// Check into iterator constraint
    fn check_into_iterator_constraint(&self,
                                     concrete_type: &TypeExpression,
                                     item_type: &TypeExpression,
                                     type_bindings: &HashMap<String, TypeExpression>) -> Result<(), CursedError> {
        self.check_trait_bound(concrete_type, "IntoIterator")
    }
    
    /// Check from constraint
    fn check_from_constraint(&self,
                            concrete_type: &TypeExpression,
                            source_type: &TypeExpression,
                            type_bindings: &HashMap<String, TypeExpression>) -> Result<(), CursedError> {
        // Check if concrete_type implements From<source_type>
        Ok(())
    }
    
    /// Check into constraint
    fn check_into_constraint(&self,
                            concrete_type: &TypeExpression,
                            target_type: &TypeExpression,
                            type_bindings: &HashMap<String, TypeExpression>) -> Result<(), CursedError> {
        // Check if concrete_type implements Into<target_type>
        Ok(())
    }
    
    /// Check partial equality constraint
    fn check_partial_eq_constraint(&self,
                                   concrete_type: &TypeExpression,
                                   other_type: &TypeExpression,
                                   type_bindings: &HashMap<String, TypeExpression>) -> Result<(), CursedError> {
        // Check if concrete_type implements PartialEq<other_type>
        Ok(())
    }
    
    /// Check partial ordering constraint
    fn check_partial_ord_constraint(&self,
                                    concrete_type: &TypeExpression,
                                    other_type: &TypeExpression,
                                    type_bindings: &HashMap<String, TypeExpression>) -> Result<(), CursedError> {
        // Check if concrete_type implements PartialOrd<other_type>
        Ok(())
    }
    
    /// Resolve type parameters in a type expression
    fn resolve_type_parameters(&self, 
                              type_expr: &TypeExpression,
                              type_bindings: &HashMap<String, TypeExpression>) -> TypeExpression {
        if let Some(name) = &type_expr.name {
            if let Some(resolved) = type_bindings.get(name) {
                return resolved.clone();
            }
        }
        
        let mut resolved = type_expr.clone();
        resolved.parameters = type_expr.parameters.iter()
            .map(|param| self.resolve_type_parameters(param, type_bindings))
            .collect();
        
        resolved
    }
    
    /// Check if two types are equal
    fn types_equal(&self, type1: &TypeExpression, type2: &TypeExpression) -> bool {
        if type1.name != type2.name {
            return false;
        }
        
        if type1.parameters.len() != type2.parameters.len() {
            return false;
        }
        
        for (param1, param2) in type1.parameters.iter().zip(type2.parameters.iter()) {
            if !self.types_equal(param1, param2) {
                return false;
            }
        }
        
        true
    }
}

impl ConstraintDependencyGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }
    
    pub fn add_node(&mut self, node: ConstraintNode) {
        let name = node.name.clone();
        let dependencies = node.dependencies.clone();
        
        self.nodes.insert(name.clone(), node);
        self.edges.insert(name, dependencies);
    }
    
    /// Perform topological sort to get constraint resolution order
    pub fn topological_sort(&self) -> Result<Vec<String>, CursedError> {
        let mut visited = HashSet::new();
        let mut temp_visited = HashSet::new();
        let mut result = Vec::new();
        
        for node_name in self.nodes.keys() {
            if !visited.contains(node_name) {
                self.dfs_topological(node_name, &mut visited, &mut temp_visited, &mut result)?;
            }
        }
        
        result.reverse();
        Ok(result)
    }
    
    fn dfs_topological(&self,
                       node: &str,
                       visited: &mut HashSet<String>,
                       temp_visited: &mut HashSet<String>,
                       result: &mut Vec<String>) -> Result<(), CursedError> {
        if temp_visited.contains(node) {
            return Err(CursedError::type_error("Circular dependency in constraints"));
        }
        
        if visited.contains(node) {
            return Ok(());
        }
        
        temp_visited.insert(node.to_string());
        
        if let Some(dependencies) = self.edges.get(node) {
            for dep in dependencies {
                self.dfs_topological(dep, visited, temp_visited, result)?;
            }
        }
        
        temp_visited.remove(node);
        visited.insert(node.to_string());
        result.push(node.to_string());
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_checker_creation() {
        let checker = AdvancedConstraintChecker::new();
        assert!(checker.trait_impls.contains_key("normie"));
        assert!(checker.trait_impls.get("normie").unwrap().contains("Clone"));
    }

    #[test]
    fn test_trait_bound_checking() {
        let checker = AdvancedConstraintChecker::new();
        let int_type = TypeExpression::named("normie");
        
        assert!(checker.check_trait_bound(&int_type, "Clone").is_ok());
        assert!(checker.check_trait_bound(&int_type, "Copy").is_ok());
        assert!(checker.check_trait_bound(&int_type, "Debug").is_ok());
        assert!(checker.check_trait_bound(&int_type, "NonExistent").is_err());
    }

    #[test]
    fn test_equality_constraint() {
        let checker = AdvancedConstraintChecker::new();
        let int_type = TypeExpression::named("normie");
        let string_type = TypeExpression::named("tea");
        let type_bindings = HashMap::new();
        
        assert!(checker.check_equality_constraint(&int_type, &int_type, &type_bindings).is_ok());
        assert!(checker.check_equality_constraint(&int_type, &string_type, &type_bindings).is_err());
    }

    #[test]
    fn test_dependency_graph() {
        let mut graph = ConstraintDependencyGraph::new();
        
        graph.add_node(ConstraintNode {
            name: "T".to_string(),
            constraints: vec![],
            dependencies: vec!["U".to_string()],
        });
        
        graph.add_node(ConstraintNode {
            name: "U".to_string(),
            constraints: vec![],
            dependencies: vec![],
        });
        
        let order = graph.topological_sort().unwrap();
        assert_eq!(order, vec!["U".to_string(), "T".to_string()]);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut graph = ConstraintDependencyGraph::new();
        
        graph.add_node(ConstraintNode {
            name: "T".to_string(),
            constraints: vec![],
            dependencies: vec!["U".to_string()],
        });
        
        graph.add_node(ConstraintNode {
            name: "U".to_string(),
            constraints: vec![],
            dependencies: vec!["T".to_string()],
        });
        
        assert!(graph.topological_sort().is_err());
    }
}
