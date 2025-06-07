//! Constraint Recovery Extension System
//!
//! This module extends the constraint recovery system with additional capabilities
//! for visualizing constraint paths, generating improved error messages,
//! and providing more sophisticated recovery strategies for complex interface constraints.

use crate::core::constraint_recovery::{ConstraintFailureContext, ConstraintFailureSeverity, RecoveryStrategy, ConstraintRecovery};
use crate::core::interface_registry::InterfaceRegistry;
use crate::core::type_checker::Type;
use crate::error::Error;
use std::collections::{HashMap, HashSet, VecDeque};
use tracing::{debug, error, info, instrument, warn};

/// Represents a constraint path from one type to another through interface implementations
#[derive(Debug, Clone)]
pub struct ConstraintPath {
    /// The source type that needs to implement the interface
    pub source_type: Type,
    
    /// The target interface
    pub target_interface: String,
    
    /// The path of intermediate types/interfaces that connect source to target
    pub path: Vec<(Type, String)>,
    
    /// Whether a valid path was found
    pub valid: bool,
    
    /// The missing link in the path, if invalid
    pub missing_link: Option<(Type, String)>,
}

/// Extension trait for InterfaceRegistry to support enhanced constraint recovery
pub trait InterfaceRegistryExtensionChecking {
    /// Find a path between a type and an interface requirement
    fn find_constraint_path(&self, source_type: &Type, target_interface: &str) -> ConstraintPath;
    
    /// Suggest possible fixes for a failed constraint
    fn suggest_constraint_fixes(&self, source_type: &Type, target_interface: &str) -> Vec<String>;
    
    /// Find alternative types that are "close" to the source type but implement the target interface
    fn find_similar_alternatives(&self, source_type: &Type, target_interface: &str) -> Vec<Type>;
    
    /// Generate detailed guidance for implementing the required interface
    fn generate_implementation_guide(&self, source_type: &Type, target_interface: &str) -> String;
    
    /// Extended error creation with path visualization
    fn create_extended_constraint_error(&self, source_type: &Type, target_interface: &str) -> Error;
    
    /// Check if a type is "close" to implementing an interface (missing just a few methods)
    fn is_close_to_implementing(&self, type_: &Type, interface_name: &str) -> bool;
}

impl InterfaceRegistryExtensionChecking for InterfaceRegistry {
    #[instrument(skip(self), level = "debug")]
    fn find_constraint_path(&self, source_type: &Type, target_interface: &str) -> ConstraintPath {
        // Start building the path
        let mut path = ConstraintPath {
            source_type: source_type.clone(),
            target_interface: target_interface.to_string(),
            path: Vec::new(),
            valid: false,
            missing_link: None,
        };
        
        // Direct check - if the type implements the interface directly
        if let Ok(true) = self.check_implementation(source_type, target_interface) {
            path.valid = true;
            path.path.push((source_type.clone(), target_interface.to_string()));
            return path;
        }
        
        // For constraint paths, we only consider direct implementations.
        // The BFS logic was incorrectly creating paths through unrelated types
        // that happened to implement the same interfaces. This is not a valid
        // constraint path - just because Type A and Type B both implement 
        // Interface X doesn't mean there's a constraint path from A to B.
        //
        // The direct implementation check above already handles the valid case.
        
        // If we got here, no path was found
        // Identify the missing link - find the closest we got
        if let Some(closest_interface) = self.find_closest_interface(source_type, target_interface) {
            path.missing_link = Some((source_type.clone(), closest_interface));
        }
        
        path
    }
    
    #[instrument(skip(self), level = "debug")]
    fn suggest_constraint_fixes(&self, source_type: &Type, target_interface: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // Suggestion 1: Implement the interface directly
        let method_list = self.get_interface_methods(target_interface)
            .map(|methods| methods.keys().cloned().collect::<Vec<_>>().join(", "));
        
        if let Some(methods) = method_list {
            suggestions.push(format!(
                "Implement interface '{}' directly for type {:?} by adding methods: {}",
                target_interface, source_type, methods
            ));
        } else {
            suggestions.push(format!(
                "Implement interface '{}' directly for type {:?}",
                target_interface, source_type
            ));
        }
        
        // Suggestion 2: Use a wrapper type
        suggestions.push(format!(
            "Create a wrapper type that implements '{}' and contains {:?}",
            target_interface, source_type
        ));
        
        // Suggestion 3: Use an alternative type
        let alternatives = self.find_similar_alternatives(source_type, target_interface);
        if !alternatives.is_empty() {
            let alt_list = alternatives.iter()
                .map(|t| format!("{:?}", t))
                .collect::<Vec<_>>().join(", ");
                
            suggestions.push(format!(
                "Use an alternative type that already implements '{}': {}",
                target_interface, alt_list
            ));
        }
        
        // Suggestion 4: If the type is close to implementing the interface
        if self.is_close_to_implementing(source_type, target_interface) {
            suggestions.push(format!(
                "Your type is close to implementing '{}' - consider adding just the missing methods",
                target_interface
            ));
        }
        
        suggestions
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_similar_alternatives(&self, source_type: &Type, target_interface: &str) -> Vec<Type> {
        let mut alternatives = Vec::new();
        let implementers = self.get_interface_implementers(target_interface);
        
        // Return all types that implement the target interface as alternatives
        // This gives users all their options when they need a type that implements a specific interface
        for impl_type in implementers {
            // Don't include the source type itself if it already implements the interface
            if impl_type != *source_type {
                alternatives.push(impl_type);
            }
        }
        
        // Limit number of alternatives
        if alternatives.len() > 5 {
            alternatives.truncate(5);
        }
        
        alternatives
    }
    
    #[instrument(skip(self), level = "debug")]
    fn generate_implementation_guide(&self, source_type: &Type, target_interface: &str) -> String {
        let mut guide = format!("# Implementation Guide for {} on {:?}\n\n", target_interface, source_type);
        
        // Get the methods required by the interface
        if let Some(methods) = self.get_interface_methods(target_interface) {
            guide.push_str("## Required Methods\n\n");
            
            for (method_name, signature) in methods {
                guide.push_str(&format!("### {}\n", method_name));
                guide.push_str(&format!("Signature: `{}`\n\n", signature));
                
                // Add example implementation
                guide.push_str("Example implementation:\n\n```\n");
                let type_name = match source_type {
                    Type::Struct(name, _) => name,
                    _ => "YourType"
                };
                
                guide.push_str(&format!("slay {}({}) {{\n", method_name, signature.replace("Self", type_name)));
                guide.push_str("    // TODO: Implement this method\n");
                guide.push_str("}\n```\n\n");
            }
        }
        
        // Check if there are other types that implement this interface
        let other_impls = self.get_interface_implementers(target_interface);
        if !other_impls.is_empty() {
            guide.push_str("## Examples from Other Types\n\n");
            guide.push_str("These types already implement this interface:\n\n");
            
            for impl_type in other_impls.iter().take(5) {
                guide.push_str(&format!("- {:?}\n", impl_type));
            }
        }
        
        guide
    }
    
    #[instrument(skip(self), level = "debug")]
    fn create_extended_constraint_error(&self, source_type: &Type, target_interface: &str) -> Error {
        // Create a base context using the regular constraint recovery
        let base_context = self.create_recovery_context(source_type, target_interface);
        
        // Find constraint path
        let path = self.find_constraint_path(source_type, target_interface);
        
        // Generate implementation guide
        let guide = self.generate_implementation_guide(source_type, target_interface);
        
        // Combine into an enhanced error message
        let mut message = base_context.to_error_message();
        
        // Add constraint path information
        message.push_str("\n\nConstraint Path:\n");
        if path.valid {
            message.push_str("A valid path was found:\n");
            for (type_, interface) in &path.path {
                message.push_str(&format!("  - {:?} implements {}\n", type_, interface));
            }
        } else {
            message.push_str("No valid path was found.\n");
            if let Some((type_, interface)) = &path.missing_link {
                message.push_str(&format!("Missing link: {:?} needs to implement {}\n", type_, interface));
            }
        }
        
        // Add fix suggestions
        let suggestions = self.suggest_constraint_fixes(source_type, target_interface);
        if !suggestions.is_empty() {
            message.push_str("\nSuggested fixes:\n");
            for (i, suggestion) in suggestions.iter().enumerate() {
                message.push_str(&format!("{:}. {}\n", i+1, suggestion));
            }
        }
        
        // Add implementation guide reference
        message.push_str("\nSee implementation guide for more details.");
        
        Error::new("CNST03", &message, None)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn is_close_to_implementing(&self, type_: &Type, interface_name: &str) -> bool {
        // Check if the type already implements some interfaces
        let implemented = self.get_implemented_interfaces(type_);
        if implemented.is_empty() {
            // If the type doesn't implement any interfaces, it's not close
            return false;
        }
        
        // If the type implements at least one related interface, it might be close
        // For now, just check if it implements any interface
        true
    }
}

// Add extension helpers
impl InterfaceRegistry {
    /// Find the interface that is closest to the target interface
    /// that the source type implements
    fn find_closest_interface(&self, source_type: &Type, target_interface: &str) -> Option<String> {
        // Get interfaces implemented by the source type
        let implemented = self.get_implemented_interfaces(source_type);
        
        // Get implementations of the target interface
        let target_implementers = self.get_interface_implementers(target_interface);
        
        // For each implemented interface, check how many types implement both
        // that interface and the target interface
        let mut best_interface = None;
        let mut best_score = 0;
        
        for interface in implemented {
            let impls = self.get_interface_implementers(&interface);
            
            // Count types that implement both interfaces
            let common_count = impls.iter()
                .filter(|t| target_implementers.contains(t))
                .count();
                
            if common_count > best_score {
                best_score = common_count;
                best_interface = Some(interface);
            }
        }
        
        best_interface
    }
    
    /// Check if two types are "similar" for suggesting alternatives
    fn are_types_similar(&self, a: &Type, b: &Type) -> bool {
        match (a, b) {
            // Same primitive types are similar
            (Type::Normie, Type::Normie) => true,
            (Type::Thicc, Type::Thicc) => true,
            (Type::Tea, Type::Tea) => true,
            (Type::Lit, Type::Lit) => true,
            
            // Numeric types are similar to each other
            (Type::Normie, Type::Thicc) | (Type::Thicc, Type::Normie) => true,
            (Type::Normie, Type::Snack) | (Type::Snack, Type::Normie) => true,
            (Type::Normie, Type::Meal) | (Type::Meal, Type::Normie) => true,
            (Type::Thicc, Type::Snack) | (Type::Snack, Type::Thicc) => true,
            (Type::Thicc, Type::Meal) | (Type::Meal, Type::Thicc) => true,
            (Type::Snack, Type::Meal) | (Type::Meal, Type::Snack) => true,
            
            // Structs with the same name are similar
            (Type::Struct(name_a, _), Type::Struct(name_b, _)) => name_a == name_b,
            
            // Otherwise, not similar
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_constraint_path() {
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Test with a type that implements the interface directly
        let result = registry.find_constraint_path(&Type::Normie, "Numeric");
        assert!(result.valid);
        assert!(!result.path.is_empty());
        
        // Test with a type that doesn't implement the interface
        let result = registry.find_constraint_path(&Type::Lit, "Numeric");
        assert!(!result.valid);
    }
    
    #[test]
    fn test_suggest_constraint_fixes() {
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Test with a type that doesn't implement the interface
        let suggestions = registry.suggest_constraint_fixes(&Type::Lit, "Numeric");
        assert!(!suggestions.is_empty());
        
        // Should include suggestion to implement directly
        assert!(suggestions.iter().any(|s| s.contains("Implement interface 'Numeric' directly")));
    }
    
    #[test]
    fn test_find_similar_alternatives() {
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Test finding alternatives for a Lit type that doesn't implement Numeric
        let alternatives = registry.find_similar_alternatives(&Type::Lit, "Numeric");
        
        // Should not include Lit itself as it doesn't implement Numeric
        assert!(!alternatives.contains(&Type::Lit));
        
        // Should include numeric types
        assert!(alternatives.iter().any(|t| matches!(t, Type::Normie | Type::Thicc | Type::Snack | Type::Meal)));
    }
    
    #[test]
    fn test_generate_implementation_guide() {
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Generate guide for implementing Comparable on a custom type
        let guide = registry.generate_implementation_guide(
            &Type::Struct("CustomType".to_string(), vec![]),
            "Comparable"
        );
        
        // Guide should mention required methods
        assert!(guide.contains("Required Methods"));
        assert!(guide.contains("Compare"));
        assert!(guide.contains("Equals"));
        
        // Guide should include example implementation
        assert!(guide.contains("Example implementation"));
    }
    
    #[test]
    fn test_extended_constraint_error() {
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Create an extended error
        let error = registry.create_extended_constraint_error(
            &Type::Lit,
            "Numeric"
        );
        
        // Error should include basic constraint failure info
        let message = error.message();
        assert!(message.contains("does not implement interface"));
        
        // Should include constraint path info
        assert!(message.contains("Constraint Path"));
        
        // Should include suggestions
        assert!(message.contains("Suggested fixes"));
    }
    
    #[test]
    fn test_is_close_to_implementing() {
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Lit implements Comparable but not Numeric
        assert!(registry.is_close_to_implementing(&Type::Lit, "Numeric"));
        
        // A custom type that doesn't implement anything should not be close
        let custom_type = Type::Struct("EmptyType".to_string(), vec![]);
        assert!(!registry.is_close_to_implementing(&custom_type, "Comparable"));
    }
}