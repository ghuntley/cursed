//! # Constraint Recovery Strategies
//!
//! This module provides strategies for recovering from constraint failures
//! in the type system. It allows the compiler to continue processing even
//! when constraints are not satisfied, providing better error messages and
//! potentially suggesting fixes or alternatives.

use crate::core::constraint_error::{create_constraint_error, create_nested_constraint_error, CONSTRAINT_ERROR_CODE_PREFIX};
use crate::core::interface_registry::{InterfaceRegistry};
use crate::core::type_checker::Type;
use crate::error_enhanced::{CursedError, ErrorKind};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{debug, error, info, instrument, warn};

/// Types of recovery strategies that can be applied
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryStrategy {
    /// Fail immediately and report the error
    Fail,
    
    /// Attempt to find alternative types that would satisfy the constraint
    FindAlternative,
    
    /// Use a placeholder implementation for testing/development
    UsePlaceholder,
    
    /// Generate a partial implementation with stub methods for missing functionality
    GenerateStubs,
}

/// The result of a recovery attempt
#[derive(Debug, Clone)]
pub enum RecoveryResult {
    /// Recovery failed, with the original error
    Failed(CursedError),
    
    /// Recovery succeeded with an alternative type
    AlternativeType(Type),
    
    /// Recovery succeeded with placeholder implementation
    Placeholder(String),
    
    /// Recovery succeeded with generated stubs
    GeneratedStubs(String),
}

/// Configuration for constraint recovery
#[derive(Debug, Clone)]
pub struct RecoveryConfig {
    /// The default strategy to use
    pub default_strategy: RecoveryStrategy,
    
    /// Specific strategies for certain interfaces
    pub interface_strategies: HashMap<String, RecoveryStrategy>,
    
    /// Maximum number of alternative types to check
    pub max_alternatives: usize,
    
    /// Whether to use type similarity for finding alternatives
    pub use_similarity: bool,
    
    /// Whether the recovery system is enabled
    pub enabled: bool,
}

impl Default for RecoveryConfig {
    fn default() -> Self {
        Self {
            default_strategy: RecoveryStrategy::Fail,
            interface_strategies: HashMap::new(),
            max_alternatives: 5,
            use_similarity: true,
            enabled: true,
        }
    }
}

/// A constraint recovery manager that provides various strategies for
/// recovering from constraint failures in the type system
#[derive(Debug)]
pub struct ConstraintRecoveryManager {
    /// The configuration for recovery strategies
    config: Arc<Mutex<RecoveryConfig>>,
    
    /// The interface registry to use for checking constraints
    registry: Arc<InterfaceRegistry>,
    
    /// Cache of previously suggested alternatives
    alternative_cache: Arc<Mutex<HashMap<(Type, String), Vec<Type>>>>,
    
    /// Cache of previously generated stubs
    stub_cache: Arc<Mutex<HashMap<(Type, String), String>>>,
    
    /// Statistics for the recovery system
    stats: Arc<Mutex<RecoveryStats>>,
}

/// Statistics for the recovery system
#[derive(Debug, Default)]
struct RecoveryStats {
    /// Number of recovery attempts
    attempts: usize,
    
    /// Number of successful recoveries
    successes: usize,
    
    /// Number of failed recoveries
    failures: usize,
    
    /// Breakdown by strategy
    by_strategy: HashMap<RecoveryStrategy, usize>,
}

impl ConstraintRecoveryManager {
    /// Create a new constraint recovery manager
    pub fn new(registry: Arc<InterfaceRegistry>) -> Self {
        Self {
            config: Arc::new(Mutex::new(RecoveryConfig::default())),
            registry,
            alternative_cache: Arc::new(Mutex::new(HashMap::new())),
            stub_cache: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(RecoveryStats::default())),
        }
    }
    
    /// Set the configuration for recovery strategies
    pub fn set_config(&self, config: RecoveryConfig) {
        let mut cfg = self.config.lock().unwrap();
        *cfg = config;
    }
    
    /// Get the current configuration
    pub fn get_config(&self) -> RecoveryConfig {
        let cfg = self.config.lock().unwrap();
        cfg.clone()
    }
    
    /// Get the strategy to use for a specific interface
    fn get_strategy_for_interface(&self, interface_name: &str) -> RecoveryStrategy {
        let cfg = self.config.lock().unwrap();
        
        // If recovery is disabled, always fail
        if !cfg.enabled {
            return RecoveryStrategy::Fail;
        }
        
        // Check for a specific strategy for this interface
        cfg.interface_strategies
            .get(interface_name)
            .copied()
            .unwrap_or(cfg.default_strategy)
    }
    
    /// Update statistics for a recovery attempt
    fn update_stats(&self, strategy: RecoveryStrategy, success: bool) {
        let mut stats = self.stats.lock().unwrap();
        stats.attempts += 1;
        
        if success {
            stats.successes += 1;
        } else {
            stats.failures += 1;
        }
        
        // Update strategy count
        *stats.by_strategy.entry(strategy).or_insert(0) += 1;
    }
    
    /// Get the current statistics
    pub fn get_stats(&self) -> (usize, usize, usize) {
        let stats = self.stats.lock().unwrap();
        (stats.attempts, stats.successes, stats.failures)
    }
    
    /// Attempt to recover from a constraint failure
    ///
    /// # Arguments
    ///
    /// * `concrete_type` - The concrete type that failed to meet the constraint
    /// * `interface_name` - The name of the interface constraint that wasn't satisfied
    /// * `type_param_name` - Optional name of the type parameter for better error context
    ///
    /// # Returns
    ///
    /// A `RecoveryResult` indicating whether recovery was successful and how
    #[instrument(skip(self), level = "debug")]
    pub fn recover_from_constraint_failure(
        &self,
        concrete_type: &Type,
        interface_name: &str,
        type_param_name: Option<&str>,
    ) -> RecoveryResult {
        // Determine the strategy to use
        let strategy = self.get_strategy_for_interface(interface_name);
        
        debug!("Attempting to recover from constraint failure: {:?} does not implement {}", 
             concrete_type, interface_name);
        debug!("Using strategy: {:?}", strategy);
        
        // For an actual implementation, we would get the interface methods here
        // In a full implementation, we would call methods like:
        // self.registry.get_interface_methods(interface_name);
        // and self.registry.get_type_methods(name)
        let required_methods = None; // Placeholder
        let available_methods = None; // Placeholder
        
        // Try to recover based on the selected strategy
        let result = match strategy {
            RecoveryStrategy::Fail => {
                // Just fail with a detailed error
                let error = create_constraint_error(
                    concrete_type,
                    interface_name,
                    type_param_name,
                    available_methods,
                    required_methods,
                );
                
                RecoveryResult::Failed(error)
            },
            
            RecoveryStrategy::FindAlternative => {
                self.find_alternative(concrete_type, interface_name, type_param_name)
            },
            
            RecoveryStrategy::UsePlaceholder => {
                self.use_placeholder(concrete_type, interface_name, type_param_name)
            },
            
            RecoveryStrategy::GenerateStubs => {
                self.generate_stubs(concrete_type, interface_name, type_param_name, 
                                   available_methods, required_methods)
            },
        };
        
        // Update statistics
        self.update_stats(strategy, !matches!(result, RecoveryResult::Failed(_)));
        
        result
    }
    
    /// Attempt to find an alternative type that satisfies the constraint
    #[instrument(skip(self), level = "debug")]
    fn find_alternative(
        &self,
        concrete_type: &Type,
        interface_name: &str,
        type_param_name: Option<&str>,
    ) -> RecoveryResult {
        // Check if we have cached alternatives for this type and interface
        let cache_key = (concrete_type.clone(), interface_name.to_string());
        
        // First check the cache
        let cached = {
            let cache = self.alternative_cache.lock().unwrap();
            cache.get(&cache_key).cloned()
        };
        
        if let Some(alternatives) = cached {
            if !alternatives.is_empty() {
                debug!("Found cached alternative for {:?} implements {}: {:?}", 
                     concrete_type, interface_name, alternatives[0]);
                return RecoveryResult::AlternativeType(alternatives[0].clone());
            }
        }
        
        // Get the configuration
        let config = self.config.lock().unwrap();
        let max_alternatives = config.max_alternatives;
        let use_similarity = config.use_similarity;
        drop(config);
        
        // Find similar types that implement the interface
        let mut alternatives = Vec::new();
        
        // Check standard library types first
        let stdlib_alternatives = self.find_stdlib_alternatives(interface_name);
        alternatives.extend(stdlib_alternatives);
        
        // If enabled, check similar user-defined types
        if use_similarity {
            let similar_alternatives = self.find_similar_types(concrete_type, interface_name);
            alternatives.extend(similar_alternatives);
        }
        
        // Cache the alternatives
        {
            let mut cache = self.alternative_cache.lock().unwrap();
            cache.insert(cache_key, alternatives.clone());
        }
        
        // Return the best alternative if any
        if !alternatives.is_empty() {
            debug!("Found alternative for {:?} implements {}: {:?}", 
                 concrete_type, interface_name, alternatives[0]);
            return RecoveryResult::AlternativeType(alternatives[0].clone());
        }
        
        // No alternatives found, return a failure
        let error = create_constraint_error(
            concrete_type,
            interface_name,
            type_param_name,
            None,
            None,
        );
        
        RecoveryResult::Failed(error)
    }
    
    /// Find standard library types that implement the interface
    fn find_stdlib_alternatives(&self, interface_name: &str) -> Vec<Type> {
        // This would typically search through the standard library types
        // that implement the given interface
        
        // For now, we'll just provide some common alternatives based on the interface
        match interface_name {
            "Comparable" => vec![
                Type::Normie,  // int
                Type::Thicc,   // int64
                Type::Snack,   // float32
                Type::Meal,    // float64
                Type::Tea,     // string
                Type::Lit,     // bool
            ],
            "Numeric" => vec![
                Type::Normie,  // int
                Type::Thicc,   // int64
                Type::Snack,   // float32
                Type::Meal,    // float64
            ],
            "Stringable" => vec![
                Type::Tea,     // string
            ],
            _ => Vec::new(),
        }
    }
    
    /// Find user-defined types similar to the concrete type that implement the interface
    fn find_similar_types(&self, concrete_type: &Type, interface_name: &str) -> Vec<Type> {
        // In a real implementation, this would analyze the codebase to find
        // similar types (by name, structure, etc.) that implement the interface
        
        // For now, we'll just return an empty list
        Vec::new()
    }
    
    /// Use a placeholder implementation for testing/development
    #[instrument(skip(self), level = "debug")]
    fn use_placeholder(
        &self,
        concrete_type: &Type,
        interface_name: &str,
        type_param_name: Option<&str>,
    ) -> RecoveryResult {
        // Generate a placeholder implementation that satisfies the interface
        // This would be used during development or testing to allow progress
        // without implementing everything
        
        let placeholder_code = format!(
            "// AUTO-GENERATED PLACEHOLDER IMPLEMENTATION FOR DEVELOPMENT ONLY
// Type: {:?}
// Interface: {}
implementation {} for {:?} {{
    // This is a placeholder implementation for testing only
    // TODO: Replace with a proper implementation
}}
",
            concrete_type, interface_name, interface_name, concrete_type
        );
        
        debug!("Generated placeholder implementation for {:?} implements {}", 
             concrete_type, interface_name);
        
        RecoveryResult::Placeholder(placeholder_code)
    }
    
    /// Generate stub implementations for missing methods
    #[instrument(skip(self, available_methods, required_methods), level = "debug")]
    fn generate_stubs(
        &self,
        concrete_type: &Type,
        interface_name: &str,
        type_param_name: Option<&str>,
        available_methods: Option<Vec<String>>,
        required_methods: Option<Vec<String>>,
    ) -> RecoveryResult {
        // Check the cache first
        let cache_key = (concrete_type.clone(), interface_name.to_string());
        
        let cached = {
            let cache = self.stub_cache.lock().unwrap();
            cache.get(&cache_key).cloned()
        };
        
        if let Some(stubs) = cached {
            debug!("Found cached stubs for {:?} implements {}", 
                 concrete_type, interface_name);
            return RecoveryResult::GeneratedStubs(stubs);
        }
        
        // Generate stub implementations for missing methods
        let mut stub_code = format!(
            "// AUTO-GENERATED STUB IMPLEMENTATION
// Type: {:?}
// Interface: {}
implementation {} for {:?} {{
",
            concrete_type, interface_name, interface_name, concrete_type
        );
        
        // Check which methods are missing
        let missing_methods = match (required_methods, available_methods) {
            (Some(required), Some(available)) => {
                required.into_iter()
                    .filter(|req| !available.contains(req))
                    .collect::<Vec<_>>()
            },
            (Some(required), None) => required,
            _ => Vec::new(),
        };
        
        // Generate stubs for each missing method
        for method in &missing_methods {
            // Parse the method signature to extract name, params, and return type
            let (name, params, return_type) = parse_method_signature(method);
            
            stub_code.push_str(&format!(
                "    // TODO: Implement this method properly
    slay {}({}) {} {{
        todo!()
    }}
    
",
                name, params, return_type
            ));
        }
        
        // Close the implementation block
        stub_code.push_str("}");
        
        // Cache the generated stubs
        {
            let mut cache = self.stub_cache.lock().unwrap();
            cache.insert(cache_key, stub_code.clone());
        }
        
        debug!("Generated stubs for {:?} implements {} with {} missing methods", 
             concrete_type, interface_name, missing_methods.len());
        
        RecoveryResult::GeneratedStubs(stub_code)
    }
    
    /// Recover from a nested constraint failure
    ///
    /// This is used when a generic type's type parameter constraints are not satisfied,
    /// which is a more complex scenario than a simple constraint failure.
    ///
    /// # Arguments
    ///
    /// * `generic_type_name` - The name of the generic type (e.g., "SortedList")
    /// * `type_param_name` - The name of the type parameter (e.g., "T")
    /// * `concrete_arg` - The concrete type argument that failed the constraint
    /// * `interface_name` - The name of the interface constraint that wasn't satisfied
    ///
    /// # Returns
    ///
    /// A `RecoveryResult` indicating whether recovery was successful and how
    #[instrument(skip(self), level = "debug")]
    pub fn recover_from_nested_constraint_failure(
        &self,
        generic_type_name: &str,
        type_param_name: &str,
        concrete_arg: &Type,
        interface_name: &str,
    ) -> RecoveryResult {
        // Determine the strategy to use
        let strategy = self.get_strategy_for_interface(interface_name);
        
        debug!("Attempting to recover from nested constraint failure: {:?} for parameter {} in {} does not implement {}", 
             concrete_arg, type_param_name, generic_type_name, interface_name);
        debug!("Using strategy: {:?}", strategy);
        
        // Try to recover based on the selected strategy
        let result = match strategy {
            RecoveryStrategy::Fail => {
                // Just fail with a detailed error
                let error = create_nested_constraint_error(
                    generic_type_name,
                    type_param_name,
                    concrete_arg,
                    interface_name,
                );
                
                RecoveryResult::Failed(error)
            },
            
            RecoveryStrategy::FindAlternative => {
                // For nested constraints, we delegate to the regular alternative finder
                // but with additional context
                self.find_alternative(concrete_arg, interface_name, Some(type_param_name))
            },
            
            RecoveryStrategy::UsePlaceholder => {
                // For nested constraints, we delegate to the regular placeholder generator
                // but with additional context
                self.use_placeholder(concrete_arg, interface_name, Some(type_param_name))
            },
            
            RecoveryStrategy::GenerateStubs => {
                // For nested constraints, we delegate to the regular stub generator
                // but with additional context
                // For an actual implementation, we would get the interface methods here
                // In a full implementation, we would call methods like:
                // self.registry.get_interface_methods(interface_name);
                // and self.registry.get_type_methods(name)
                let required_methods = None; // Placeholder
                let available_methods = None; // Placeholder
                
                self.generate_stubs(concrete_arg, interface_name, Some(type_param_name),
                                   available_methods, required_methods)
            },
        };
        
        // Update statistics
        self.update_stats(strategy, !matches!(result, RecoveryResult::Failed(_)));
        
        result
    }
}

/// Parse a method signature to extract the name, parameters, and return type
///
/// This is a simple parser that extracts components from a method signature
/// string like "add(a Normie, b Normie) Normie".
///
/// # Arguments
///
/// * `signature` - The method signature to parse
///
/// # Returns
///
/// A tuple of (name, parameters, return_type)
fn parse_method_signature(signature: &str) -> (String, String, String) {
    // Find the opening parenthesis to extract the name
    if let Some(name_end) = signature.find('(') {
        let name = signature[0..name_end].trim().to_string();
        
        // Find the closing parenthesis to extract parameters
        if let Some(params_end) = signature.find(')') {
            let params = signature[name_end+1..params_end].trim().to_string();
            
            // The return type is everything after the closing parenthesis
            let return_type = signature[params_end+1..].trim().to_string();
            
            return (name, params, return_type);
        }
    }
    
    // Fallback for unparseable signatures
    (signature.to_string(), "".to_string(), "".to_string())
}

/// Extension trait for InterfaceRegistry to add recovery capabilities
pub trait ConstraintRecovery {
    /// Attempt to recover from a constraint failure
    fn recover_from_constraint_failure(
        &self,
        concrete_type: &Type,
        interface_name: &str,
        type_param_name: Option<&str>,
    ) -> RecoveryResult;
    
    /// Recover from a nested constraint failure
    fn recover_from_nested_constraint_failure(
        &self,
        generic_type_name: &str,
        type_param_name: &str,
        concrete_arg: &Type,
        interface_name: &str,
    ) -> RecoveryResult;
    
    /// Set the recovery configuration
    fn set_recovery_config(&self, config: RecoveryConfig);
    
    /// Get the current recovery configuration
    fn get_recovery_config(&self) -> RecoveryConfig;
}

impl ConstraintRecovery for InterfaceRegistry {
    fn recover_from_constraint_failure(
        &self,
        concrete_type: &Type,
        interface_name: &str,
        type_param_name: Option<&str>,
    ) -> RecoveryResult {
        let recovery_manager = ConstraintRecoveryManager::new(Arc::new(self.clone()));
        recovery_manager.recover_from_constraint_failure(concrete_type, interface_name, type_param_name)
    }
    
    fn recover_from_nested_constraint_failure(
        &self,
        generic_type_name: &str,
        type_param_name: &str,
        concrete_arg: &Type,
        interface_name: &str,
    ) -> RecoveryResult {
        let recovery_manager = ConstraintRecoveryManager::new(Arc::new(self.clone()));
        recovery_manager.recover_from_nested_constraint_failure(
            generic_type_name, 
            type_param_name, 
            concrete_arg, 
            interface_name
        )
    }
    
    fn set_recovery_config(&self, config: RecoveryConfig) {
        let recovery_manager = ConstraintRecoveryManager::new(Arc::new(self.clone()));
        recovery_manager.set_config(config);
    }
    
    fn get_recovery_config(&self) -> RecoveryConfig {
        let recovery_manager = ConstraintRecoveryManager::new(Arc::new(self.clone()));
        recovery_manager.get_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::type_checker::Type;
    
    #[path = "../../tests/common.rs"]
    mod common;
    
    #[test]
    fn test_constraint_recovery_fail_strategy() {
        common::tracing::setup();
        
        let registry = InterfaceRegistry::new();
        let recovery_manager = ConstraintRecoveryManager::new(Arc::new(registry));
        
        // Test the Fail strategy
        let concrete_type = Type::Struct("TestStruct".to_string(), vec![]);
        let interface_name = "Comparable";
        
        let result = recovery_manager.recover_from_constraint_failure(
            &concrete_type,
            interface_name,
            None,
        );
        
        // Should be a Failed result
        match result {
            RecoveryResult::Failed(error) => {
                assert!(error.message.contains("does not implement interface"));
                assert!(error.message.contains("TestStruct"));
                assert!(error.message.contains("Comparable"));
            },
            _ => panic!("Expected Failed result"),
        }
    }
    
    #[test]
    fn test_constraint_recovery_find_alternative_strategy() {
        common::tracing::setup();
        
        let registry = InterfaceRegistry::new();
        let recovery_manager = ConstraintRecoveryManager::new(Arc::new(registry));
        
        // Configure to use the FindAlternative strategy
        let mut config = RecoveryConfig::default();
        config.default_strategy = RecoveryStrategy::FindAlternative;
        recovery_manager.set_config(config);
        
        // Test the FindAlternative strategy
        let concrete_type = Type::Struct("TestStruct".to_string(), vec![]);
        let interface_name = "Comparable";
        
        let result = recovery_manager.recover_from_constraint_failure(
            &concrete_type,
            interface_name,
            None,
        );
        
        // Should be an AlternativeType result
        match result {
            RecoveryResult::AlternativeType(alt_type) => {
                // For Comparable, we should get one of our standard alternatives
                match alt_type {
                    Type::Normie | Type::Thicc | Type::Snack | 
                    Type::Meal | Type::Tea | Type::Str => {
                        // These are expected
                    },
                    _ => panic!("Unexpected alternative type: {:?}", alt_type),
                }
            },
            _ => panic!("Expected AlternativeType result"),
        }
    }
    
    #[test]
    fn test_constraint_recovery_use_placeholder_strategy() {
        common::tracing::setup();
        
        let registry = InterfaceRegistry::new();
        let recovery_manager = ConstraintRecoveryManager::new(Arc::new(registry));
        
        // Configure to use the UsePlaceholder strategy
        let mut config = RecoveryConfig::default();
        config.default_strategy = RecoveryStrategy::UsePlaceholder;
        recovery_manager.set_config(config);
        
        // Test the UsePlaceholder strategy
        let concrete_type = Type::Struct("TestStruct".to_string(), vec![]);
        let interface_name = "Comparable";
        
        let result = recovery_manager.recover_from_constraint_failure(
            &concrete_type,
            interface_name,
            None,
        );
        
        // Should be a Placeholder result
        match result {
            RecoveryResult::Placeholder(code) => {
                assert!(code.contains("AUTO-GENERATED PLACEHOLDER"));
                assert!(code.contains("TestStruct"));
                assert!(code.contains("Comparable"));
                assert!(code.contains("implementation Comparable for"));
            },
            _ => panic!("Expected Placeholder result"),
        }
    }
    
    #[test]
    fn test_constraint_recovery_generate_stubs_strategy() {
        common::tracing::setup();
        
        // Create a registry with some interface methods
        let mut registry = InterfaceRegistry::new();
        let methods = vec![
            "equals(other Tea) Tea".to_string(),
            "compare(other Tea) Normie".to_string(),
        ];
        registry.register_interface("Comparable".to_string(), methods);
        
        let recovery_manager = ConstraintRecoveryManager::new(Arc::new(registry));
        
        // Configure to use the GenerateStubs strategy
        let mut config = RecoveryConfig::default();
        config.default_strategy = RecoveryStrategy::GenerateStubs;
        recovery_manager.set_config(config);
        
        // Test the GenerateStubs strategy
        let concrete_type = Type::Struct("TestStruct".to_string(), vec![]);
        let interface_name = "Comparable";
        
        let result = recovery_manager.recover_from_constraint_failure(
            &concrete_type,
            interface_name,
            None,
        );
        
        // Should be a GeneratedStubs result
        match result {
            RecoveryResult::GeneratedStubs(code) => {
                assert!(code.contains("AUTO-GENERATED STUB"));
                assert!(code.contains("TestStruct"));
                assert!(code.contains("Comparable"));
                assert!(code.contains("implementation Comparable for"));
                assert!(code.contains("slay equals"));
                assert!(code.contains("slay compare"));
                assert!(code.contains("TODO: Implement this method"));
            },
            _ => panic!("Expected GeneratedStubs result"),
        }
    }
    
    #[test]
    fn test_nested_constraint_recovery() {
        common::tracing::setup();
        
        let registry = InterfaceRegistry::new();
        let recovery_manager = ConstraintRecoveryManager::new(Arc::new(registry));
        
        // Configure to use the GenerateStubs strategy
        let mut config = RecoveryConfig::default();
        config.default_strategy = RecoveryStrategy::GenerateStubs;
        recovery_manager.set_config(config);
        
        // Test nested constraint recovery
        let generic_type_name = "SortedList";
        let type_param_name = "T";
        let concrete_arg = Type::Struct("UserType".to_string(), vec![]);
        let interface_name = "Comparable";
        
        let result = recovery_manager.recover_from_nested_constraint_failure(
            generic_type_name,
            type_param_name,
            &concrete_arg,
            interface_name,
        );
        
        // Should be a GeneratedStubs result
        match result {
            RecoveryResult::GeneratedStubs(code) => {
                assert!(code.contains("AUTO-GENERATED STUB"));
                assert!(code.contains("UserType"));
                assert!(code.contains("Comparable"));
                assert!(code.contains("implementation Comparable for"));
            },
            _ => panic!("Expected GeneratedStubs result"),
        }
    }
    
    #[test]
    fn test_interface_registry_extension() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        let methods = vec![
            "equals(other Tea) Tea".to_string(),
            "compare(other Tea) Normie".to_string(),
        ];
        registry.register_interface("Comparable".to_string(), methods);
        
        // Set up recovery config through the extension trait
        let mut config = RecoveryConfig::default();
        config.default_strategy = RecoveryStrategy::UsePlaceholder;
        registry.set_recovery_config(config);
        
        // Test recovery through the extension trait
        let concrete_type = Type::Struct("TestStruct".to_string(), vec![]);
        let interface_name = "Comparable";
        
        let result = registry.recover_from_constraint_failure(
            &concrete_type,
            interface_name,
            None,
        );
        
        // Should be a Placeholder result
        match result {
            RecoveryResult::Placeholder(code) => {
                assert!(code.contains("AUTO-GENERATED PLACEHOLDER"));
                assert!(code.contains("TestStruct"));
                assert!(code.contains("Comparable"));
            },
            _ => panic!("Expected Placeholder result"),
        }
        
        // Check that we can get the config back
        let retrieved_config = registry.get_recovery_config();
        assert_eq!(retrieved_config.default_strategy, RecoveryStrategy::UsePlaceholder);
    }
}