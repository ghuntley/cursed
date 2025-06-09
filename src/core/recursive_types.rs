//! Recursive type support for the CURSED language.
//!
//! This module provides the core infrastructure for handling recursive type definitions,
//! including cycle detection, forward declarations, and lazy type resolution.
//!
//! ## Features
//!
//! - **Recursive Type Detection**: Identifies direct and indirect recursion in type definitions
//! - **Cycle Detection**: Prevents infinite loops during type resolution
//! - **Forward Declarations**: Enables mutually recursive types through forward declarations
//! - **Lazy Resolution**: Defers type resolution to break circular dependencies
//! - **Dependency Tracking**: Tracks type dependencies for proper resolution order

use crate::core::type_checker::Type;
use crate::error::Error;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use tracing::{debug, info, warn};

/// Represents a recursive type definition that may contain self-references
#[derive(Debug, Clone, PartialEq)]
pub struct RecursiveType {
    /// The name of the type
    pub name: String,
    /// The underlying type definition (may contain recursive references)
    pub definition: Type,
    /// Whether this type is currently being resolved (for cycle detection)
    pub resolving: bool,
    /// Direct dependencies of this type
    pub dependencies: HashSet<String>,
    /// Whether this type has been fully resolved
    pub resolved: bool,
}

impl RecursiveType {
    /// Create a new recursive type
    pub fn new(name: String, definition: Type) -> Self {
        let dependencies = Self::extract_dependencies(&definition);
        Self {
            name,
            definition,
            resolving: false,
            dependencies,
            resolved: false,
        }
    }

    /// Extract type dependencies from a type definition
    fn extract_dependencies(type_def: &Type) -> HashSet<String> {
        let mut deps = HashSet::new();
        
        match type_def {
            Type::Named(name) => {
                deps.insert(name.clone());
            }
            Type::Struct(_name, type_args) => {
                // Don't add the struct's own name as a dependency
                for arg in type_args {
                    deps.extend(Self::extract_dependencies(arg));
                }
            }
            Type::Interface(_name, type_args) => {
                // Don't add the interface's own name as a dependency
                for arg in type_args {
                    deps.extend(Self::extract_dependencies(arg));
                }
            }
            Type::Pointer(inner) => {
                deps.extend(Self::extract_dependencies(inner));
            }
            Type::Array(inner, _) => {
                deps.extend(Self::extract_dependencies(inner));
            }
            Type::Slice(inner) => {
                deps.extend(Self::extract_dependencies(inner));
            }
            Type::Map(key, value) => {
                deps.extend(Self::extract_dependencies(key));
                deps.extend(Self::extract_dependencies(value));
            }
            Type::Function(params, return_type) => {
                for param in params {
                    deps.extend(Self::extract_dependencies(param));
                }
                deps.extend(Self::extract_dependencies(return_type));
            }
            Type::Channel(inner) => {
                deps.extend(Self::extract_dependencies(inner));
            }
            Type::Generic(_name, type_args) => {
                // Don't add the generic's own name as a dependency
                for arg in type_args {
                    deps.extend(Self::extract_dependencies(arg));
                }
            }
            _ => {} // Primitive types have no dependencies
        }
        
        deps
    }

    /// Check if this type is directly recursive (refers to itself)
    pub fn is_directly_recursive(&self) -> bool {
        self.dependencies.contains(&self.name)
    }
}

/// Registry for managing recursive type definitions and resolution
#[derive(Debug, Default)]
pub struct RecursiveTypeRegistry {
    /// Map of type names to their recursive type definitions
    types: HashMap<String, RecursiveType>,
    /// Forward declarations for types that are referenced but not yet defined
    forward_declarations: HashSet<String>,
    /// Resolution stack for cycle detection
    resolution_stack: Vec<String>,
}

impl RecursiveTypeRegistry {
    /// Create a new registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a type definition
    pub fn register_type(&mut self, name: String, definition: Type) -> Result<(), Error> {
        debug!(type_name = %name, "Registering recursive type");
        
        let recursive_type = RecursiveType::new(name.clone(), definition);
        
        // Check for direct recursion
        if recursive_type.is_directly_recursive() {
            info!(type_name = %name, "Detected directly recursive type");
        }
        
        self.types.insert(name.clone(), recursive_type);
        self.forward_declarations.remove(&name);
        
        Ok(())
    }

    /// Add a forward declaration for a type
    pub fn add_forward_declaration(&mut self, name: String) {
        debug!(type_name = %name, "Adding forward declaration");
        self.forward_declarations.insert(name);
    }

    /// Resolve a type by name, handling recursive references
    pub fn resolve_type(&mut self, name: &str) -> Result<Type, Error> {
        self.resolve_type_internal(name, &mut HashSet::new())
    }

    /// Internal recursive type resolution with cycle detection
    fn resolve_type_internal(&mut self, name: &str, visiting: &mut HashSet<String>) -> Result<Type, Error> {
        debug!(type_name = %name, "Resolving type");

        // Check for cycles
        if visiting.contains(name) {
            debug!(type_name = %name, "Detected cycle during type resolution");
            // For recursive types, return a pointer to break the cycle
            return Ok(Type::Pointer(Box::new(Type::Named(name.to_string()))));
        }

        // Check if this is a forward declaration that hasn't been defined yet
        if self.forward_declarations.contains(name) {
            debug!(type_name = %name, "Type is forward declared but not yet defined");
            return Ok(Type::Named(name.to_string()));
        }

        // Get the type definition
        let recursive_type = self.types.get(name).cloned();
        let recursive_type = match recursive_type {
            Some(rt) => rt,
            None => {
                debug!(type_name = %name, "Type not found, treating as primitive or external");
                return Ok(Type::Named(name.to_string()));
            }
        };

        // If already resolved, return the resolved type
        if recursive_type.resolved {
            debug!(type_name = %name, "Type already resolved");
            return Ok(recursive_type.definition);
        }

        // Mark as visiting to detect cycles
        visiting.insert(name.to_string());

        // Resolve the type definition recursively
        let resolved_def = self.resolve_type_definition(&recursive_type.definition, visiting)?;

        // Mark as resolved and update
        let mut updated_type = recursive_type;
        updated_type.definition = resolved_def.clone();
        updated_type.resolved = true;
        self.types.insert(name.to_string(), updated_type);

        // Remove from visiting set
        visiting.remove(name);

        Ok(resolved_def)
    }

    /// Resolve a type definition recursively
    fn resolve_type_definition(&mut self, type_def: &Type, visiting: &mut HashSet<String>) -> Result<Type, Error> {
        match type_def {
            Type::Named(name) => {
                self.resolve_type_internal(name, visiting)
            }
            Type::Struct(name, type_args) => {
                let mut resolved_args = Vec::new();
                for arg in type_args {
                    resolved_args.push(Box::new(self.resolve_type_definition(arg, visiting)?));
                }
                Ok(Type::Struct(name.clone(), resolved_args))
            }
            Type::Interface(name, type_args) => {
                let mut resolved_args = Vec::new();
                for arg in type_args {
                    resolved_args.push(Box::new(self.resolve_type_definition(arg, visiting)?));
                }
                Ok(Type::Interface(name.clone(), resolved_args))
            }
            Type::Pointer(inner) => {
                let resolved_inner = self.resolve_type_definition(inner, visiting)?;
                Ok(Type::Pointer(Box::new(resolved_inner)))
            }
            Type::Array(inner, size) => {
                let resolved_inner = self.resolve_type_definition(inner, visiting)?;
                Ok(Type::Array(Box::new(resolved_inner), *size))
            }
            Type::Slice(inner) => {
                let resolved_inner = self.resolve_type_definition(inner, visiting)?;
                Ok(Type::Slice(Box::new(resolved_inner)))
            }
            Type::Map(key, value) => {
                let resolved_key = self.resolve_type_definition(key, visiting)?;
                let resolved_value = self.resolve_type_definition(value, visiting)?;
                Ok(Type::Map(Box::new(resolved_key), Box::new(resolved_value)))
            }
            Type::Function(params, return_type) => {
                let mut resolved_params = Vec::new();
                for param in params {
                    resolved_params.push(Box::new(self.resolve_type_definition(param, visiting)?));
                }
                let resolved_return = self.resolve_type_definition(return_type, visiting)?;
                Ok(Type::Function(resolved_params, Box::new(resolved_return)))
            }
            Type::Channel(inner) => {
                let resolved_inner = self.resolve_type_definition(inner, visiting)?;
                Ok(Type::Channel(Box::new(resolved_inner)))
            }
            Type::Generic(name, type_args) => {
                let mut resolved_args = Vec::new();
                for arg in type_args {
                    resolved_args.push(Box::new(self.resolve_type_definition(arg, visiting)?));
                }
                Ok(Type::Generic(name.clone(), resolved_args))
            }
            // Primitive types don't need resolution
            _ => Ok(type_def.clone()),
        }
    }

    /// Detect cycles in the type dependency graph
    pub fn detect_cycles(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for type_name in self.types.keys() {
            if !visited.contains(type_name) {
                self.detect_cycles_dfs(type_name, &mut visited, &mut rec_stack, &mut path, &mut cycles);
            }
        }

        cycles
    }

    /// DFS-based cycle detection
    fn detect_cycles_dfs(
        &self,
        type_name: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(type_name.to_string());
        rec_stack.insert(type_name.to_string());
        path.push(type_name.to_string());

        if let Some(recursive_type) = self.types.get(type_name) {
            for dep in &recursive_type.dependencies {
                if !visited.contains(dep) {
                    self.detect_cycles_dfs(dep, visited, rec_stack, path, cycles);
                } else if rec_stack.contains(dep) {
                    // Found a cycle
                    if let Some(start_pos) = path.iter().position(|x| x == dep) {
                        let cycle = path[start_pos..].to_vec();
                        cycles.push(cycle);
                    }
                }
            }
        }

        path.pop();
        rec_stack.remove(type_name);
    }

    /// Get dependency order for type resolution (topological sort)
    pub fn get_resolution_order(&self) -> Result<Vec<String>, Error> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();

        // Initialize in-degree and adjacency lists
        for (type_name, recursive_type) in &self.types {
            in_degree.entry(type_name.clone()).or_insert(0);
            adjacency.entry(type_name.clone()).or_insert_with(Vec::new);

            for dep in &recursive_type.dependencies {
                adjacency.entry(dep.clone()).or_insert_with(Vec::new).push(type_name.clone());
                *in_degree.entry(type_name.clone()).or_insert(0) += 1;
            }
        }

        // Kahn's algorithm for topological sorting
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        // Add all nodes with in-degree 0 to the queue
        for (type_name, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(type_name.clone());
            }
        }

        while let Some(current) = queue.pop_front() {
            result.push(current.clone());

            if let Some(neighbors) = adjacency.get(&current) {
                for neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        // Check for cycles
        if result.len() != self.types.len() {
            warn!("Circular dependency detected in type definitions");
            // Return partial order for types that can be resolved
            Ok(result)
        } else {
            Ok(result)
        }
    }

    /// Check if a type is recursive
    pub fn is_recursive(&self, type_name: &str) -> bool {
        if let Some(recursive_type) = self.types.get(type_name) {
            recursive_type.is_directly_recursive()
        } else {
            false
        }
    }

    /// Get all registered types
    pub fn get_types(&self) -> &HashMap<String, RecursiveType> {
        &self.types
    }

    /// Get forward declarations
    pub fn get_forward_declarations(&self) -> &HashSet<String> {
        &self.forward_declarations
    }
}

/// Trait for types that can handle recursive type resolution
pub trait RecursiveTypeResolver {
    /// Resolve recursive type references
    fn resolve_recursive_types(&mut self, registry: &mut RecursiveTypeRegistry) -> Result<(), Error>;
    
    /// Check if a type contains recursive references
    fn contains_recursive_references(&self, type_def: &Type) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_type_detection() {
        let mut registry = RecursiveTypeRegistry::new();
        
        // Define a recursive linked list node
        let node_type = Type::Struct(
            "Node".to_string(),
            vec![
                Box::new(Type::Normie), // value field
                Box::new(Type::Pointer(Box::new(Type::Named("Node".to_string())))), // next field
            ],
        );
        
        registry.register_type("Node".to_string(), node_type).unwrap();
        
        assert!(registry.is_recursive("Node"));
    }

    #[test]
    fn test_mutually_recursive_types() {
        let mut registry = RecursiveTypeRegistry::new();
        
        // Define mutually recursive types A and B
        let type_a = Type::Struct(
            "A".to_string(),
            vec![Box::new(Type::Pointer(Box::new(Type::Named("B".to_string()))))],
        );
        
        let type_b = Type::Struct(
            "B".to_string(),
            vec![Box::new(Type::Pointer(Box::new(Type::Named("A".to_string()))))],
        );
        
        registry.register_type("A".to_string(), type_a).unwrap();
        registry.register_type("B".to_string(), type_b).unwrap();
        
        let cycles = registry.detect_cycles();
        assert!(!cycles.is_empty());
    }

    #[test]
    fn test_forward_declarations() {
        let mut registry = RecursiveTypeRegistry::new();
        
        // Add forward declaration
        registry.add_forward_declaration("Node".to_string());
        
        // Try to resolve before definition
        let resolved = registry.resolve_type("Node").unwrap();
        assert_eq!(resolved, Type::Named("Node".to_string()));
    }

    #[test]
    fn test_dependency_resolution_order() {
        let mut registry = RecursiveTypeRegistry::new();
        
        // Define types with dependencies: C depends on B, B depends on A
        let type_a = Type::Struct("A".to_string(), vec![Box::new(Type::Normie)]);
        let type_b = Type::Struct(
            "B".to_string(),
            vec![Box::new(Type::Named("A".to_string()))],
        );
        let type_c = Type::Struct(
            "C".to_string(),
            vec![Box::new(Type::Named("B".to_string()))],
        );
        
        registry.register_type("C".to_string(), type_c).unwrap();
        registry.register_type("A".to_string(), type_a).unwrap();
        registry.register_type("B".to_string(), type_b).unwrap();
        
        let order = registry.get_resolution_order().unwrap();
        
        // A should come before B, B should come before C
        let pos_a = order.iter().position(|x| x == "A").unwrap();
        let pos_b = order.iter().position(|x| x == "B").unwrap();
        let pos_c = order.iter().position(|x| x == "C").unwrap();
        
        assert!(pos_a < pos_b);
        assert!(pos_b < pos_c);
    }
}
