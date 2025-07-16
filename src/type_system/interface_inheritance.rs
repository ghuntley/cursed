//! Enhanced Interface Inheritance and Composition System for CURSED
//!
//! This module implements advanced interface inheritance patterns including:
//! - Multiple interface inheritance
//! - Interface composition and mixins
//! - Method resolution optimization
//! - Diamond inheritance handling
//! - Interface constraint propagation

use crate::ast::{InterfaceStatement, MethodSignature, Parameter, Type as AstType, TypeParameter};
use crate::error_types::CursedError;
use crate::type_system::interface_compliance::{
    InterfaceComplianceChecker, 
    InterfaceMethodRequirement, 
    ReceiverType,
    GenericInterfaceDefinition
};
use crate::type_system::generic_interfaces::{GenericInterface};
use crate::ast::InterfaceComposition;
use std::collections::{HashMap, HashSet, BTreeMap};

/// Interface inheritance tree for optimization
#[derive(Debug, Clone)]
pub struct InterfaceInheritanceTree {
    /// Map from interface name to its direct children
    children: HashMap<String, Vec<String>>,
    /// Map from interface name to its direct parents
    parents: HashMap<String, Vec<String>>,
    /// Map from interface name to all transitive parents (flattened hierarchy)
    transitive_parents: HashMap<String, Vec<String>>,
    /// Map from interface name to its method set (including inherited methods)
    flattened_methods: HashMap<String, Vec<InterfaceMethodRequirement>>,
    /// Interface composition relationships
    compositions: HashMap<String, Vec<InterfaceComposition>>,
}

// InterfaceComposition is defined in ast.rs

/// Enhanced interface inheritance checker with composition support
#[derive(Debug, Clone)]
pub struct InterfaceInheritanceChecker {
    /// Base compliance checker
    compliance_checker: InterfaceComplianceChecker,
    /// Inheritance tree for optimization
    inheritance_tree: InterfaceInheritanceTree,
    /// Interface definitions for reference
    interface_definitions: HashMap<String, InterfaceStatement>,
    /// Generic interface definitions
    generic_interfaces: HashMap<String, GenericInterfaceDefinition>,
}

impl InterfaceInheritanceTree {
    /// Create a new inheritance tree
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            parents: HashMap::new(),
            transitive_parents: HashMap::new(),
            flattened_methods: HashMap::new(),
            compositions: HashMap::new(),
        }
    }

    /// Add an interface to the inheritance tree
    pub fn add_interface(&mut self, interface: &InterfaceStatement) -> Result<(), CursedError> {
        let interface_name = &interface.name;

        // Add parent relationships
        if !interface.extends.is_empty() {
            self.parents.insert(interface_name.clone(), interface.extends.clone());

            // Update children relationships
            for parent in &interface.extends {
                self.children
                    .entry(parent.clone())
                    .or_insert_with(Vec::new)
                    .push(interface_name.clone());
            }
        }

        Ok(())
    }

    /// Add interface composition relationship
    pub fn add_composition(&mut self, interface_name: &str, composition: InterfaceComposition) {
        self.compositions
            .entry(interface_name.to_string())
            .or_insert_with(Vec::new)
            .push(composition);
    }

    /// Build transitive parent relationships (optimization)
    pub fn build_transitive_relationships(&mut self) -> Result<(), CursedError> {
        for interface_name in self.parents.keys() {
            let mut transitive = Vec::new();
            let mut visited = HashSet::new();
            self.collect_transitive_parents(interface_name, &mut transitive, &mut visited)?;
            self.transitive_parents.insert(interface_name.clone(), transitive);
        }
        Ok(())
    }

    /// Recursively collect all transitive parents
    fn collect_transitive_parents(
        &self,
        interface_name: &str,
        transitive: &mut Vec<String>,
        visited: &mut HashSet<String>,
    ) -> Result<(), CursedError> {
        if visited.contains(interface_name) {
            return Err(CursedError::Runtime(format!(
                "Circular interface inheritance detected in transitive parent collection: {}",
                interface_name
            )));
        }

        visited.insert(interface_name.to_string());

        if let Some(direct_parents) = self.parents.get(interface_name) {
            for parent in direct_parents {
                if !transitive.contains(parent) {
                    transitive.push(parent.clone());
                }
                
                // Recursively collect transitive parents
                self.collect_transitive_parents(parent, transitive, visited)?;
            }
        }

        visited.remove(interface_name);
        Ok(())
    }

    /// Get all parent interfaces (optimized lookup)
    pub fn get_all_parents(&self, interface_name: &str) -> Vec<String> {
        self.transitive_parents
            .get(interface_name)
            .cloned()
            .unwrap_or_default()
    }

    /// Check if one interface extends another (optimized)
    pub fn interface_extends(&self, derived: &str, base: &str) -> bool {
        if derived == base {
            return true;
        }

        if let Some(parents) = self.transitive_parents.get(derived) {
            parents.contains(&base.to_string())
        } else {
            false
        }
    }

    /// Get interface composition relationships
    pub fn get_compositions(&self, interface_name: &str) -> Vec<InterfaceComposition> {
        self.compositions
            .get(interface_name)
            .cloned()
            .unwrap_or_default()
    }

    /// Build flattened method sets for all interfaces
    pub fn build_flattened_methods(
        &mut self,
        interface_definitions: &HashMap<String, InterfaceStatement>,
    ) -> Result<(), CursedError> {
        for (interface_name, interface_def) in interface_definitions {
            let mut all_methods = Vec::new();
            let mut visited = HashSet::new();
            
            self.collect_interface_methods(
                interface_name,
                interface_definitions,
                &mut all_methods,
                &mut visited,
            )?;

            // Process compositions
            if let Some(compositions) = self.compositions.get(interface_name) {
                for composition in compositions {
                    self.apply_composition(
                        &composition,
                        interface_definitions,
                        &mut all_methods,
                    )?;
                }
            }

            // Remove duplicate methods (derived methods override base methods)
            self.deduplicate_methods(&mut all_methods);

            self.flattened_methods.insert(interface_name.clone(), all_methods);
        }

        Ok(())
    }

    /// Collect all methods for an interface including inherited ones
    fn collect_interface_methods(
        &self,
        interface_name: &str,
        interface_definitions: &HashMap<String, InterfaceStatement>,
        methods: &mut Vec<InterfaceMethodRequirement>,
        visited: &mut HashSet<String>,
    ) -> Result<(), CursedError> {
        if visited.contains(interface_name) {
            return Err(CursedError::Runtime(format!(
                "Circular interface inheritance detected: {}",
                interface_name
            )));
        }

        visited.insert(interface_name.to_string());

        // Add this interface's methods
        if let Some(interface_def) = interface_definitions.get(interface_name) {
            for method in &interface_def.methods {
                methods.push(InterfaceMethodRequirement {
                    name: method.name.clone(),
                    parameters: method.parameters.clone(),
                    return_type: method.return_type.clone(),
                    receiver_type: ReceiverType::Value, // Default, should be determined from signature
                });
            }

            // Add inherited methods
            for parent_interface in &interface_def.extends {
                self.collect_interface_methods(
                    parent_interface,
                    interface_definitions,
                    methods,
                    visited,
                )?;
            }
        }

        visited.remove(interface_name);
        Ok(())
    }

    /// Apply interface composition
    fn apply_composition(
        &self,
        composition: &InterfaceComposition,
        interface_definitions: &HashMap<String, InterfaceStatement>,
        methods: &mut Vec<InterfaceMethodRequirement>,
    ) -> Result<(), CursedError> {
        if let Some(composed_interface_def) = interface_definitions.get(&composition.composed_interface) {
            for method in &composed_interface_def.methods {
                // Skip excluded methods
                if composition.excluded_methods.contains(&method.name) {
                    continue;
                }

                // Apply method renames
                let method_name = composition
                    .method_renames
                    .get(&method.name)
                    .cloned()
                    .unwrap_or_else(|| method.name.clone());

                methods.push(InterfaceMethodRequirement {
                    name: method_name,
                    parameters: method.parameters.clone(),
                    return_type: method.return_type.clone(),
                    receiver_type: ReceiverType::Value,
                });
            }
        }

        Ok(())
    }

    /// Remove duplicate methods (later methods override earlier ones)
    fn deduplicate_methods(&self, methods: &mut Vec<InterfaceMethodRequirement>) {
        let mut seen_methods = BTreeMap::new();
        let mut to_remove = Vec::new();

        // Keep track of methods by name, last occurrence wins
        for (i, method) in methods.iter().enumerate() {
            if let Some(previous_index) = seen_methods.insert(method.name.clone(), i) {
                to_remove.push(previous_index);
            }
        }

        // Remove duplicates in reverse order to preserve indices
        to_remove.sort_by(|a, b| b.cmp(a));
        for &i in &to_remove {
            methods.remove(i);
        }
    }

    /// Get flattened method set for an interface (optimized)
    pub fn get_flattened_methods(&self, interface_name: &str) -> Vec<InterfaceMethodRequirement> {
        self.flattened_methods
            .get(interface_name)
            .cloned()
            .unwrap_or_default()
    }
}

impl InterfaceInheritanceChecker {
    /// Create a new interface inheritance checker
    pub fn new() -> Self {
        Self {
            compliance_checker: InterfaceComplianceChecker::new(),
            inheritance_tree: InterfaceInheritanceTree::new(),
            interface_definitions: HashMap::new(),
            generic_interfaces: HashMap::new(),
        }
    }

    /// Register an interface with enhanced inheritance support
    pub fn register_interface(&mut self, interface: &InterfaceStatement) -> Result<(), CursedError> {
        // Register with base compliance checker
        self.compliance_checker.register_interface(interface)?;

        // Add to inheritance tree
        self.inheritance_tree.add_interface(interface)?;

        // Store interface definition
        self.interface_definitions.insert(interface.name.clone(), interface.clone());

        // If generic, store in generic interfaces
        if !interface.type_parameters.is_empty() {
            let generic_def = GenericInterfaceDefinition {
                name: interface.name.clone(),
                type_parameters: interface.type_parameters.iter().map(|tp| {
                    crate::type_system::interface_compliance::TypeParameter {
                        name: tp.name.clone(),
                        bounds: tp.bounds.clone(),
                    }
                }).collect(),
                extends: interface.extends.clone(),
                methods: interface.methods.iter().map(|method| {
                    InterfaceMethodRequirement {
                        name: method.name.clone(),
                        parameters: method.parameters.clone(),
                        return_type: method.return_type.clone(),
                        receiver_type: ReceiverType::Value,
                    }
                }).collect(),
            };
            self.generic_interfaces.insert(interface.name.clone(), generic_def);
        }

        Ok(())
    }

    /// Add interface composition relationship
    pub fn add_interface_composition(
        &mut self,
        interface_name: &str,
        composed_interface: &str,
        options: InterfaceCompositionOptions,
    ) -> Result<(), CursedError> {
        let composition = InterfaceComposition {
            composed_interface: composed_interface.to_string(),
            alias: options.alias,
            excluded_methods: options.excluded_methods,
            method_renames: options.method_renames,
        };

        self.inheritance_tree.add_composition(interface_name, composition);
        Ok(())
    }

    /// Build optimized inheritance relationships
    pub fn build_inheritance_optimization(&mut self) -> Result<(), CursedError> {
        // Build transitive relationships
        self.inheritance_tree.build_transitive_relationships()?;

        // Build flattened method sets
        self.inheritance_tree.build_flattened_methods(&self.interface_definitions)?;

        Ok(())
    }

    /// Check interface compliance with inheritance optimization
    pub fn check_interface_compliance_optimized(
        &self,
        type_name: &str,
        interface_name: &str,
    ) -> Result<bool, CursedError> {
        // Use optimized flattened method set
        let interface_methods = self.inheritance_tree.get_flattened_methods(interface_name);

        // Get type methods from compliance checker
        let type_methods = self.compliance_checker
            .get_type_methods(type_name)
            .ok_or_else(|| CursedError::Runtime(format!("Type '{}' not found", type_name)))?;

        // Check each method requirement
        for requirement in &interface_methods {
            let mut method_found = false;
            for implementation in type_methods {
                if implementation.name == requirement.name {
                    // Check detailed compatibility
                    if self.is_method_compatible(requirement, implementation)? {
                        method_found = true;
                        break;
                    }
                }
            }

            if !method_found {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Check method compatibility with enhanced rules
    fn is_method_compatible(
        &self,
        requirement: &InterfaceMethodRequirement,
        implementation: &crate::type_system::interface_compliance::ConcreteMethodImplementation,
    ) -> Result<bool, CursedError> {
        // Parameter count must match
        if requirement.parameters.len() != implementation.parameters.len() {
            return Ok(false);
        }

        // Check parameter types
        for (req_param, impl_param) in requirement.parameters.iter().zip(implementation.parameters.iter()) {
            if !self.are_option_types_compatible(&req_param.param_type, &impl_param.param_type)? {
                return Ok(false);
            }
        }

        // Check return types
        match (&requirement.return_type, &implementation.return_type) {
            (None, None) => {},
            (Some(req_ret), Some(impl_ret)) => {
                if !self.are_types_compatible(req_ret, impl_ret)? {
                    return Ok(false);
                }
            },
            _ => return Ok(false),
        }

        // Check receiver compatibility
        self.are_receivers_compatible(&requirement.receiver_type, &implementation.receiver_type)
    }

    /// Check compatibility for optional types (from Parameter)
    fn are_option_types_compatible(&self, required: &Option<AstType>, provided: &Option<AstType>) -> Result<bool, CursedError> {
        match (required, provided) {
            (None, None) => Ok(true),
            (Some(req), Some(prov)) => self.are_types_compatible(req, prov),
            _ => Ok(false),
        }
    }

    /// Enhanced type compatibility checking
    fn are_types_compatible(&self, required: &AstType, provided: &AstType) -> Result<bool, CursedError> {
        // Use existing type compatibility logic from compliance checker
        // This is a simplified version - in practice, would use the full type system
        match (required, provided) {
            (AstType::Normie, AstType::Normie) => Ok(true),
            (AstType::Tea, AstType::Tea) => Ok(true),
            (AstType::Lit, AstType::Lit) => Ok(true),
            (AstType::Custom(req), AstType::Custom(prov)) => Ok(req == prov),
            (AstType::Collab(req), AstType::Collab(prov)) => {
                // Interface types are compatible if one extends the other
                Ok(req == prov || self.inheritance_tree.interface_extends(prov, req))
            },
            _ => Ok(false),
        }
    }

    /// Check receiver type compatibility
    fn are_receivers_compatible(
        &self,
        required: &ReceiverType,
        provided: &ReceiverType,
    ) -> Result<bool, CursedError> {
        match (required, provided) {
            (ReceiverType::Any, _) => Ok(true),
            (ReceiverType::Value, ReceiverType::Value) => Ok(true),
            (ReceiverType::Pointer, ReceiverType::Pointer) => Ok(true),
            (ReceiverType::Value, ReceiverType::Pointer) => Ok(true), // Auto-dereference
            _ => Ok(false),
        }
    }

    /// Get interface hierarchy information
    pub fn get_interface_hierarchy(&self, interface_name: &str) -> InterfaceHierarchyInfo {
        InterfaceHierarchyInfo {
            name: interface_name.to_string(),
            direct_parents: self.inheritance_tree.parents
                .get(interface_name)
                .cloned()
                .unwrap_or_default(),
            all_parents: self.inheritance_tree.get_all_parents(interface_name),
            direct_children: self.inheritance_tree.children
                .get(interface_name)
                .cloned()
                .unwrap_or_default(),
            compositions: self.inheritance_tree.get_compositions(interface_name),
            flattened_methods: self.inheritance_tree.get_flattened_methods(interface_name),
        }
    }

    /// Validate interface hierarchy for cycles and conflicts
    pub fn validate_interface_hierarchy(&self) -> Result<Vec<InterfaceHierarchyValidationError>, CursedError> {
        let mut errors = Vec::new();

        // Check for circular inheritance
        for interface_name in self.interface_definitions.keys() {
            if let Err(e) = self.check_circular_inheritance(interface_name) {
                errors.push(InterfaceHierarchyValidationError::CircularInheritance {
                    interface: interface_name.clone(),
                    cycle: vec![interface_name.clone()], // Simplified
                });
            }
        }

        // Check for method conflicts in multiple inheritance
        for (interface_name, interface_def) in &self.interface_definitions {
            if interface_def.extends.len() > 1 {
                if let Some(conflicts) = self.check_method_conflicts(interface_name)? {
                    errors.push(InterfaceHierarchyValidationError::MethodConflict {
                        interface: interface_name.clone(),
                        conflicting_methods: conflicts,
                    });
                }
            }
        }

        Ok(errors)
    }

    /// Check for circular inheritance starting from an interface
    fn check_circular_inheritance(&self, interface_name: &str) -> Result<(), CursedError> {
        let mut visited = HashSet::new();
        self.detect_cycle(interface_name, &mut visited)
    }

    /// Recursive cycle detection
    fn detect_cycle(&self, interface_name: &str, visited: &mut HashSet<String>) -> Result<(), CursedError> {
        if visited.contains(interface_name) {
            return Err(CursedError::Runtime(format!("Circular inheritance detected: {}", interface_name)));
        }

        visited.insert(interface_name.to_string());

        if let Some(interface_def) = self.interface_definitions.get(interface_name) {
            for parent in &interface_def.extends {
                self.detect_cycle(parent, visited)?;
            }
        }

        visited.remove(interface_name);
        Ok(())
    }

    /// Check for method conflicts in multiple inheritance
    fn check_method_conflicts(&self, interface_name: &str) -> Result<Option<Vec<MethodConflict>>, CursedError> {
        let interface_def = self.interface_definitions.get(interface_name)
            .ok_or_else(|| CursedError::Runtime(format!("Interface '{}' not found", interface_name)))?;

        if interface_def.extends.len() <= 1 {
            return Ok(None);
        }

        let mut method_sources: HashMap<String, Vec<String>> = HashMap::new();

        // Collect methods from all parent interfaces
        for parent_interface in &interface_def.extends {
            let parent_methods = self.inheritance_tree.get_flattened_methods(parent_interface);
            for method in parent_methods {
                method_sources
                    .entry(method.name.clone())
                    .or_insert_with(Vec::new)
                    .push(parent_interface.clone());
            }
        }

        // Find conflicts (methods defined in multiple parents)
        let mut conflicts = Vec::new();
        for (method_name, sources) in method_sources {
            if sources.len() > 1 {
                conflicts.push(MethodConflict {
                    method_name,
                    conflicting_interfaces: sources,
                });
            }
        }

        if conflicts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(conflicts))
        }
    }
}

/// Options for interface composition
#[derive(Debug, Clone, Default)]
pub struct InterfaceCompositionOptions {
    /// Optional alias for the composed interface
    pub alias: Option<String>,
    /// Methods to exclude from composition
    pub excluded_methods: Vec<String>,
    /// Method renames (old_name -> new_name)
    pub method_renames: HashMap<String, String>,
}

/// Interface hierarchy information
#[derive(Debug, Clone)]
pub struct InterfaceHierarchyInfo {
    pub name: String,
    pub direct_parents: Vec<String>,
    pub all_parents: Vec<String>,
    pub direct_children: Vec<String>,
    pub compositions: Vec<InterfaceComposition>,
    pub flattened_methods: Vec<InterfaceMethodRequirement>,
}

/// Interface hierarchy validation errors
#[derive(Debug, Clone)]
pub enum InterfaceHierarchyValidationError {
    CircularInheritance {
        interface: String,
        cycle: Vec<String>,
    },
    MethodConflict {
        interface: String,
        conflicting_methods: Vec<MethodConflict>,
    },
}

/// Method conflict in multiple inheritance
#[derive(Debug, Clone)]
pub struct MethodConflict {
    pub method_name: String,
    pub conflicting_interfaces: Vec<String>,
}

/// Global interface inheritance checker
static mut GLOBAL_INHERITANCE_CHECKER: Option<InterfaceInheritanceChecker> = None;

/// Initialize global inheritance checker
pub fn initialize_interface_inheritance_checker() {
    unsafe {
        GLOBAL_INHERITANCE_CHECKER = Some(InterfaceInheritanceChecker::new());
    }
}

/// Get global inheritance checker
pub fn get_global_inheritance_checker() -> Result<&'static mut InterfaceInheritanceChecker, CursedError> {
    unsafe {
        GLOBAL_INHERITANCE_CHECKER
            .as_mut()
            .ok_or_else(|| CursedError::Runtime("Interface inheritance checker not initialized".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn test_interface_inheritance_tree() {
        let mut tree = InterfaceInheritanceTree::new();

        // Create interfaces
        let base_interface = InterfaceStatement {
            name: "BaseInterface".to_string(),
            type_parameters: vec![],
            extends: vec![],
            compositions: vec![],
            methods: vec![],
            visibility: Visibility::Public,
        };

        let derived_interface = InterfaceStatement {
            name: "DerivedInterface".to_string(),
            type_parameters: vec![],
            extends: vec!["BaseInterface".to_string()],
            compositions: vec![],
            methods: vec![],
            visibility: Visibility::Public,
        };

        assert!(tree.add_interface(&base_interface).is_ok());
        assert!(tree.add_interface(&derived_interface).is_ok());
        assert!(tree.build_transitive_relationships().is_ok());

        assert!(tree.interface_extends("DerivedInterface", "BaseInterface"));
        assert!(!tree.interface_extends("BaseInterface", "DerivedInterface"));
    }

    #[test]
    fn test_interface_composition() {
        let mut checker = InterfaceInheritanceChecker::new();

        // Register interfaces
        let drawable = InterfaceStatement {
            name: "Drawable".to_string(),
            type_parameters: vec![],
            extends: vec![],
            compositions: vec![],
            methods: vec![
                MethodSignature {
                    name: "draw".to_string(),
                    receiver: None,
                    parameters: vec![],
                    return_type: None,
                }
            ],
            visibility: Visibility::Public,
        };

        let clickable = InterfaceStatement {
            name: "Clickable".to_string(),
            type_parameters: vec![],
            extends: vec![],
            compositions: vec![],
            methods: vec![
                MethodSignature {
                    name: "click".to_string(),
                    receiver: None,
                    parameters: vec![],
                    return_type: None,
                }
            ],
            visibility: Visibility::Public,
        };

        assert!(checker.register_interface(&drawable).is_ok());
        assert!(checker.register_interface(&clickable).is_ok());

        // Add composition
        let options = InterfaceCompositionOptions::default();
        assert!(checker.add_interface_composition("UIElement", "Drawable", options).is_ok());

        let options = InterfaceCompositionOptions::default();
        assert!(checker.add_interface_composition("UIElement", "Clickable", options).is_ok());

        assert!(checker.build_inheritance_optimization().is_ok());
    }

    #[test]
    fn test_multiple_inheritance_validation() {
        let mut checker = InterfaceInheritanceChecker::new();

        // Create interfaces with conflicting methods
        let interface_a = InterfaceStatement {
            name: "InterfaceA".to_string(),
            type_parameters: vec![],
            extends: vec![],
            compositions: vec![],
            methods: vec![
                MethodSignature {
                    name: "conflicting_method".to_string(),
                    receiver: None,
                    parameters: vec![],
                    return_type: Some(AstType::Normie),
                }
            ],
            visibility: Visibility::Public,
        };

        let interface_b = InterfaceStatement {
            name: "InterfaceB".to_string(),
            type_parameters: vec![],
            extends: vec![],
            compositions: vec![],
            methods: vec![
                MethodSignature {
                    name: "conflicting_method".to_string(),
                    receiver: None,
                    parameters: vec![],
                    return_type: Some(AstType::Tea), // Different return type
                }
            ],
            visibility: Visibility::Public,
        };

        let derived_interface = InterfaceStatement {
            name: "DerivedInterface".to_string(),
            type_parameters: vec![],
            extends: vec!["InterfaceA".to_string(), "InterfaceB".to_string()],
            compositions: vec![],
            methods: vec![],
            visibility: Visibility::Public,
        };

        assert!(checker.register_interface(&interface_a).is_ok());
        assert!(checker.register_interface(&interface_b).is_ok());
        assert!(checker.register_interface(&derived_interface).is_ok());
        assert!(checker.build_inheritance_optimization().is_ok());

        let validation_errors = checker.validate_interface_hierarchy().unwrap();
        assert!(!validation_errors.is_empty());
    }
}
