//! Constraint Recovery Strategies
//!
//! This module provides error recovery strategies for interface constraint failures.
//! When a type fails to satisfy an interface constraint, these strategies can help
//! provide better error messages, alternative suggestions, and even placeholder code
//! generation to improve the developer experience.

use crate::core::interface_registry::InterfaceRegistry;
use crate::core::type_checker::Type;
use crate::error::Error;
use std::collections::{HashMap, HashSet};
use tracing::{debug, info, instrument, warn};

/// Represents the severity of a constraint failure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintFailureSeverity {
    /// Minor - can continue with warnings
    Minor,
    /// Major - should be addressed but compilation can continue
    Major,
    /// Critical - must be fixed for compilation to succeed
    Critical,
}

/// Represents a recovery strategy for constraint failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryStrategy {
    /// Suggest alternative types that would satisfy the constraint
    SuggestAlternatives,
    /// Generate placeholder code that satisfies the constraint
    GeneratePlaceholder,
    /// Generate stub implementations for the required interface methods
    GenerateStub,
    /// Fail immediately with an error
    FailImmediately,
}

/// A constraint failure recovery context, which contains information about 
/// the failed constraint and possible recovery strategies
#[derive(Debug, Clone)]
pub struct ConstraintFailureContext {
    /// The type that failed to satisfy the constraint
    pub failed_type: Type,
    /// The interface that the type failed to implement
    pub interface_name: String,
    /// The severity of the failure
    pub severity: ConstraintFailureSeverity,
    /// Missing method information (method name -> signature)
    pub missing_methods: HashMap<String, String>,
    /// Alternative types that would satisfy the constraint
    pub alternative_types: Vec<Type>,
    /// Recommended recovery strategy
    pub recommended_strategy: RecoveryStrategy,
    /// Placeholder code that could be used (if applicable)
    pub placeholder_code: Option<String>,
    /// Stub implementation code that could be used (if applicable)
    pub stub_code: Option<String>,
}

impl ConstraintFailureContext {
    /// Create a new constraint failure context
    pub fn new(failed_type: Type, interface_name: String) -> Self {
        Self {
            failed_type,
            interface_name,
            severity: ConstraintFailureSeverity::Major,
            missing_methods: HashMap::new(),
            alternative_types: Vec::new(),
            recommended_strategy: RecoveryStrategy::SuggestAlternatives,
            placeholder_code: None,
            stub_code: None,
        }
    }
    
    /// Set the severity of the failure
    pub fn with_severity(mut self, severity: ConstraintFailureSeverity) -> Self {
        self.severity = severity;
        self
    }
    
    /// Add a missing method
    pub fn add_missing_method(mut self, method_name: &str, signature: &str) -> Self {
        self.missing_methods.insert(method_name.to_string(), signature.to_string());
        self
    }
    
    /// Add an alternative type
    pub fn add_alternative_type(mut self, alternative: Type) -> Self {
        self.alternative_types.push(alternative);
        self
    }
    
    /// Set the recommended recovery strategy
    pub fn with_strategy(mut self, strategy: RecoveryStrategy) -> Self {
        self.recommended_strategy = strategy;
        self
    }
    
    /// Set the placeholder code
    pub fn with_placeholder_code(mut self, code: &str) -> Self {
        self.placeholder_code = Some(code.to_string());
        self
    }
    
    /// Set the stub implementation code
    pub fn with_stub_code(mut self, code: &str) -> Self {
        self.stub_code = Some(code.to_string());
        self
    }
    
    /// Format the context as a user-friendly error message
    pub fn to_error_message(&self) -> String {
        let mut message = format!(
            "Type {:?} does not implement interface '{}'",
            self.failed_type, self.interface_name
        );
        
        if !self.missing_methods.is_empty() {
            message.push_str("
Missing methods:
");
            for (method, signature) in &self.missing_methods {
                message.push_str(&format!("  - {} with signature {}
", method, signature));
            }
        }
        
        if !self.alternative_types.is_empty() {
            message.push_str("
Alternative types that implement this interface:
");
            for alt_type in &self.alternative_types {
                message.push_str(&format!("  - {:?}
", alt_type));
            }
        }
        
        if let Some(placeholder) = &self.placeholder_code {
            message.push_str("
Placeholder implementation:
");
            message.push_str(placeholder);
        }
        
        if let Some(stub) = &self.stub_code {
            message.push_str("
Stub implementation:
");
            message.push_str(stub);
        }
        
        message
    }
    
    /// Convert the context to an Error object
    pub fn to_error(&self) -> Error {
        Error::new("CNST02", &self.to_error_message(), None)
    }
}

/// Trait for constraint recovery strategies
pub trait ConstraintRecovery {
    /// Create a recovery context for a constraint failure
    fn create_recovery_context(&self, failed_type: &Type, interface_name: &str) -> ConstraintFailureContext;
    
    /// Find alternative types that implement the given interface
    fn find_alternative_types(&self, interface_name: &str, limit: usize) -> Vec<Type>;
    
    /// Generate placeholder code for implementing an interface
    fn generate_placeholder_code(&self, type_name: &str, interface_name: &str) -> String;
    
    /// Generate stub implementation code for an interface
    fn generate_stub_code(&self, type_name: &str, interface_name: &str) -> String;
    
    /// Recommend a recovery strategy based on the constraint failure
    fn recommend_strategy(
        &self,
        failed_type: &Type,
        interface_name: &str
    ) -> RecoveryStrategy;
    
    /// Create a comprehensive error with recovery information
    fn create_constraint_error(
        &self,
        failed_type: &Type,
        interface_name: &str
    ) -> Error;
    
    /// Check if recovery is available for this interface
    fn has_recovery_for_interface(&self, interface_name: &str) -> bool;
}

/// Extension trait for InterfaceRegistry to add constraint recovery capabilities
pub trait ConstraintRecoveryExtension: ConstraintRecovery {
    /// Register method signatures for an interface
    fn register_interface_methods(
        &mut self,
        interface_name: &str,
        methods: HashMap<String, String>
    );
    
    /// Register recovery strategies for specific interfaces
    fn register_recovery_strategy(
        &mut self,
        interface_name: &str,
        strategy: RecoveryStrategy
    );
    
    /// Register alternative types for interfaces
    fn register_alternative_for_interface(
        &mut self,
        interface_name: &str,
        alternative_type: Type
    );
}

/// Implementation of ConstraintRecovery for InterfaceRegistry
impl ConstraintRecovery for InterfaceRegistry {
    #[instrument(skip(self), level = "debug")]
    fn create_recovery_context(&self, failed_type: &Type, interface_name: &str) -> ConstraintFailureContext {
        // Start building the context
        let mut context = ConstraintFailureContext::new(failed_type.clone(), interface_name.to_string());
        
        // Determine the severity based on the interface
        let severity = match interface_name {
            "Comparable" | "Numeric" => ConstraintFailureSeverity::Critical,
            "Container" | "List" => ConstraintFailureSeverity::Major,
            _ => ConstraintFailureSeverity::Minor,
        };
        context = context.with_severity(severity);
        
        // Find alternative types
        let alternatives = self.find_alternative_types(interface_name, 5);
        for alt in alternatives {
            context = context.add_alternative_type(alt);
        }
        
        // Add missing methods based on known interface methods
        if let Some(methods) = self.get_interface_methods(interface_name) {
            for (method_name, signature) in methods {
                context = context.add_missing_method(&method_name, &signature);
            }
        }
        
        // Recommend a strategy
        let strategy = self.recommend_strategy(failed_type, interface_name);
        context = context.with_strategy(strategy.clone());
        
        // Generate placeholder code if appropriate
        if strategy == RecoveryStrategy::GeneratePlaceholder {
            let type_name = match failed_type {
                Type::Struct(name, _) => name,
                _ => "UnknownType",
            };
            let placeholder = self.generate_placeholder_code(type_name, interface_name);
            context = context.with_placeholder_code(&placeholder);
        }
        
        // Generate stub code if appropriate
        if strategy == RecoveryStrategy::GenerateStub {
            let type_name = match failed_type {
                Type::Struct(name, _) => name,
                _ => "UnknownType",
            };
            let stub = self.generate_stub_code(type_name, interface_name);
            context = context.with_stub_code(&stub);
        }
        
        context
    }
    
    fn find_alternative_types(&self, interface_name: &str, limit: usize) -> Vec<Type> {
        // Get implementers from the registry
        let implementers = self.get_interface_implementers(interface_name);
        
        // Filter to standard library types and limit the number
        let standard_lib_types: Vec<Type> = implementers
            .into_iter()
            .filter(|t| match t {
                Type::Struct(name, _) => name.starts_with("Std") || 
                                        name == "String" || 
                                        name == "Array" ||
                                        name == "List" ||
                                        name == "Map",
                _ => true,
            })
            .take(limit)
            .collect();
        
        standard_lib_types
    }
    
    fn generate_placeholder_code(&self, type_name: &str, interface_name: &str) -> String {
        match interface_name {
            "Comparable" => format!(
                "slay (a {0}, b {0}) lowkey Lit {{
    // Implement basic comparison using memory address if no other method is available
    bet a == b; // Default equality comparison
}}",
                type_name
            ),
            "Numeric" => format!(
                "slay (a {0}, b {0}) lowkey {0} {{
    // Default implementation for numeric addition
    bet a; // Return first operand as default, customize this for your specific type
}}",
                type_name
            ),
            "Container" => format!(
                "slay Size(c {0}) lowkey Normie {{
    // Default implementation returns 0 for unknown container sizes
    // For your specific container type, you should determine the actual element count
    bet 0; // Replace with actual size calculation
}}",
                type_name
            ),
            _ => format!(
                "// Implementation required: Implement interface '{}' for type '{}'
// See documentation for required methods and their signatures",
                interface_name, type_name
            ),
        }
    }
    
    fn generate_stub_code(&self, type_name: &str, interface_name: &str) -> String {
        let mut code = String::new();
        
        // Get method signatures for this interface
        if let Some(methods) = self.get_interface_methods(interface_name) {
            for (method_name, signature) in methods {
                code.push_str(&format!(
                    "// Implementation of {} for {}
",
                    method_name, type_name
                ));
                code.push_str(&format!(
                    "slay {}({}) {{
    // Implementation needed for interface method
    // This is a stub that needs to be filled with proper implementation
    panic!(\"TODO: Implement stub for this method\");
    // TODO: Implement stub
}}

",
                    method_name, signature
                ));
            }
        } else {
            // Generic stub if we don't know the methods
            code.push_str(&format!(
                "// IMPLEMENTATION REQUIRED: All methods of interface '{}' must be implemented for type '{}'
// Refer to the interface documentation for the complete list of required methods
",
                interface_name, type_name
            ));
        }
        
        code
    }
    
    fn recommend_strategy(
        &self,
        failed_type: &Type,
        interface_name: &str
    ) -> RecoveryStrategy {
        // Check if we have a strategy registered for this interface
        if let Some(strategy) = self.get_recovery_strategy(interface_name) {
            return strategy.clone();
        }
        
        // Otherwise, recommend based on the type and interface
        match failed_type {
            Type::Struct(_, _) => {
                // For user-defined types, suggest generating stubs
                match interface_name {
                    "Comparable" | "Container" | "Numeric" => RecoveryStrategy::GenerateStub,
                    _ => RecoveryStrategy::SuggestAlternatives,
                }
            },
            _ => {
                // For primitive types, suggest alternatives
                RecoveryStrategy::SuggestAlternatives
            }
        }
    }
    
    fn create_constraint_error(
        &self,
        failed_type: &Type,
        interface_name: &str
    ) -> Error {
        let context = self.create_recovery_context(failed_type, interface_name);
        context.to_error()
    }
    
    fn has_recovery_for_interface(&self, interface_name: &str) -> bool {
        // Check if we have methods registered for this interface
        self.get_interface_methods(interface_name).is_some()
    }
}

/// Extension of InterfaceRegistry with recovery capabilities
impl ConstraintRecoveryExtension for InterfaceRegistry {
    fn register_interface_methods(
        &mut self,
        interface_name: &str,
        methods: HashMap<String, String>
    ) {
        // Store the methods in the interface_methods field
        self.interface_methods.insert(interface_name.to_string(), methods);
        
        debug!(
            interface = interface_name,
            method_count = self.interface_methods.get(interface_name).map_or(0, |m| m.len()),
            "Registered methods for interface"
        );
    }
    
    fn register_recovery_strategy(
        &mut self,
        interface_name: &str,
        strategy: RecoveryStrategy
    ) {
        // Store the strategy in the recovery_strategies field
        self.recovery_strategies.insert(interface_name.to_string(), strategy.clone());
        
        debug!(
            interface = interface_name,
            strategy = ?strategy,
            "Registered recovery strategy for interface"
        );
    }
    
    fn register_alternative_for_interface(
        &mut self,
        interface_name: &str,
        alternative_type: Type
    ) {
        // This is already handled by the regular interface registration
        self.register_implementation(alternative_type, interface_name.to_string());
    }
}

// Extension methods for InterfaceRegistry to access the new capabilities
impl InterfaceRegistry {
    /// Get the method signatures for an interface
    pub fn get_interface_methods(&self, interface_name: &str) -> Option<HashMap<String, String>> {
        // First check if we have methods registered in our field
        if let Some(methods) = self.interface_methods.get(interface_name) {
            return Some(methods.clone());
        }
        
        // Fall back to hardcoded values for known interfaces
        match interface_name {
            "Comparable" => {
                let mut methods = HashMap::new();
                methods.insert("Compare".to_string(), "a Self, b Self".to_string());
                methods.insert("Equals".to_string(), "a Self, b Self".to_string());
                Some(methods)
            },
            "Numeric" => {
                let mut methods = HashMap::new();
                methods.insert("Add".to_string(), "a Self, b Self".to_string());
                methods.insert("Subtract".to_string(), "a Self, b Self".to_string());
                methods.insert("Multiply".to_string(), "a Self, b Self".to_string());
                methods.insert("Divide".to_string(), "a Self, b Self".to_string());
                Some(methods)
            },
            "Container" => {
                let mut methods = HashMap::new();
                methods.insert("Size".to_string(), "self Self".to_string());
                methods.insert("IsEmpty".to_string(), "self Self".to_string());
                Some(methods)
            },
            "List" => {
                let mut methods = HashMap::new();
                methods.insert("Get".to_string(), "self Self, index Normie".to_string());
                methods.insert("Set".to_string(), "self Self, index Normie, value T".to_string());
                methods.insert("Append".to_string(), "self Self, value T".to_string());
                Some(methods)
            },
            _ => None,
        }
    }
    
    /// Get the registered recovery strategy for an interface
    pub fn get_recovery_strategy(&self, interface_name: &str) -> Option<RecoveryStrategy> {
        // First check if we have a strategy registered in our field
        if let Some(strategy) = self.recovery_strategies.get(interface_name) {
            return Some(strategy.clone());
        }
        
        // Fall back to hardcoded values for known interfaces
        match interface_name {
            "Comparable" => Some(RecoveryStrategy::GenerateStub),
            "Numeric" => Some(RecoveryStrategy::GenerateStub),
            "Container" => Some(RecoveryStrategy::GeneratePlaceholder),
            "List" => Some(RecoveryStrategy::SuggestAlternatives),
            _ => None,
        }
    }
    
    /// Check if a type satisfies an interface constraint, with recovery information
    pub fn check_constraint_with_recovery(
        &self,
        type_: &Type,
        interface_name: &str
    ) -> Result<bool, ConstraintFailureContext> {
        match self.check_implementation(type_, interface_name) {
            Ok(true) => Ok(true),
            Ok(false) => {
                let context = self.create_recovery_context(type_, interface_name);
                Err(context)
            },
            Err(e) => {
                // Convert regular error to constraint failure
                let mut context = self.create_recovery_context(type_, interface_name);
                warn!("Error checking constraint: {}", e);
                context = context.with_severity(ConstraintFailureSeverity::Critical);
                Err(context)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constraint_recovery_basic() {
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Test with a type that doesn't implement Comparable
        let non_comparable = Type::Struct("NonComparable".to_string(), vec![]);
        
        // Get recovery context
        let context = registry.create_recovery_context(&non_comparable, "Comparable");
        
        // Verify basic properties
        assert_eq!(context.failed_type, non_comparable);
        assert_eq!(context.interface_name, "Comparable");
        assert_eq!(context.severity, ConstraintFailureSeverity::Critical);
        
        // Verify we have alternatives
        assert!(!context.alternative_types.is_empty());
        
        // Verify we have missing methods
        assert!(!context.missing_methods.is_empty());
        
        // Verify we have a recommended strategy
        assert_eq!(context.recommended_strategy, RecoveryStrategy::GenerateStub);
    }
    
    #[test]
    fn test_constraint_recovery_with_placeholder() {
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Test with a type that doesn't implement Container
        let non_container = Type::Struct("CustomCollection".to_string(), vec![]);
        
        // Check constraint with recovery
        let result = registry.check_constraint_with_recovery(&non_container, "Container");
        
        // Should fail
        assert!(result.is_err());
        
        // Get the context
        let context = result.err().unwrap();
        
        // Should have placeholder code
        assert!(context.placeholder_code.is_some());
        
        // Verify the placeholder code contains the type name
        let placeholder = context.placeholder_code.unwrap();
        assert!(placeholder.contains("CustomCollection"));
    }
    
    #[test]
    fn test_stub_generation() {
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Generate stub for Numeric interface
        let stub = registry.generate_stub_code("Vector2D", "Numeric");
        
        // Should contain implementations for all Numeric methods
        assert!(stub.contains("Add"));
        assert!(stub.contains("Subtract"));
        assert!(stub.contains("Multiply"));
        assert!(stub.contains("Divide"));
    }
    
    #[test]
    fn test_error_message_formatting() {
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Create an error for a type that doesn't implement Comparable
        let error = registry.create_constraint_error(
            &Type::Struct("NonComparable".to_string(), vec![]),
            "Comparable"
        );
        
        // Error message should be informative
        let message = error.message();
        assert!(message.contains("does not implement interface"));
        assert!(message.contains("Missing methods"));
        assert!(message.contains("Alternative types"));
    }
}