//! # Automatic Interface Implementation Registration
//!
//! This module integrates the type checker with the interface registry to automatically
//! register interface implementations found during type checking. This enables the
//! compiler to track which types implement which interfaces without requiring explicit
//! registration.

use crate::core::interface_registry::{InterfaceRegistry, GenericInterfaceImpl};
use crate::core::interface_registry_cache_merged::{InterfaceImplementationCache, ThreadSafeInterfaceCache};
use crate::core::type_checker::{Type, TypeChecker};
use crate::error::Error;
use std::sync::{Arc, Mutex};
use tracing::{debug, error, info, instrument, trace, warn};

/// Trait for automatically registering interface implementations
/// during type checking
pub trait AutoInterfaceRegistration {
    /// Register that a type implements an interface
    fn register_interface_implementation(
        &mut self,
        implementing_type: &Type,
        interface_name: &str
    ) -> Result<(), Error>;
    
    /// Register a generic type that implements an interface
    fn register_generic_interface_implementation(
        &mut self,
        type_name: &str,
        type_params: &[String],
        interface_name: &str,
        constraints: Vec<(String, String)>
    ) -> Result<(), Error>;
    
    /// Check interface implementation during type checking and automatically
    /// register successful implementations
    fn check_and_register_interface_implementation(
        &mut self,
        implementing_type: &Type,
        interface: &Type
    ) -> Result<bool, Error>;
    
    /// Get the interface registry instance
    fn get_interface_registry(&self) -> Arc<Mutex<InterfaceRegistry>>;
}

impl AutoInterfaceRegistration for TypeChecker {
    /// Register a concrete type as implementing an interface
    #[instrument(skip(self), level = "debug")]
    fn register_interface_implementation(
        &mut self,
        implementing_type: &Type,
        interface_name: &str
    ) -> Result<(), Error> {
        debug!(
            "Registering {} as implementing interface {}",
            format!("{:?}", implementing_type),
            interface_name
        );
        
        // Get the registry instance
        let registry = self.get_interface_registry();
        let mut registry = registry.lock().unwrap();
        
        // Register the implementation
        registry.register_implementation(implementing_type.clone(), interface_name.to_string());
        
        debug!(
            "Successfully registered {} as implementing {}",
            format!("{:?}", implementing_type),
            interface_name
        );
        
        Ok(())
    }
    
    /// Register a generic type as implementing an interface
    #[instrument(skip(self), level = "debug")]
    fn register_generic_interface_implementation(
        &mut self,
        type_name: &str,
        type_params: &[String],
        interface_name: &str,
        constraints: Vec<(String, String)>
    ) -> Result<(), Error> {
        debug!(
            "Registering generic type {} with type parameters {:?} as implementing interface {}",
            type_name,
            type_params,
            interface_name
        );
        
        // Get the registry instance
        let registry = self.get_interface_registry();
        let mut registry = registry.lock().unwrap();
        
        // Convert type parameters to owned Strings
        let type_param_strings: Vec<String> = type_params.iter()
            .map(|s| s.clone())
            .collect();
        
        // Register the generic implementation
        registry.register_generic_implementation(
            type_name.to_string(),
            type_param_strings,
            interface_name.to_string(),
            constraints
        );
        
        debug!(
            "Successfully registered generic type {} as implementing {}",
            type_name,
            interface_name
        );
        
        Ok(())
    }
    
    /// Check if a type implements an interface and register it if so
    #[instrument(skip(self), level = "debug")]
    fn check_and_register_interface_implementation(
        &mut self,
        implementing_type: &Type,
        interface: &Type
    ) -> Result<bool, Error> {
        // Extract the interface name
        let (interface_name, _) = match interface {
            Type::Interface(name, type_args) => (name, type_args),
            _ => return Err(Error::from_str("Expected an interface type")),
        };
        
        // First check if the type implements the interface
        let implements = self.check_interface_implementation(implementing_type, interface)?;
        
        // If it implements the interface, register it
        if implements {
            match implementing_type {
                // For concrete structs
                Type::Struct(name, type_args) if type_args.is_empty() => {
                    self.register_interface_implementation(implementing_type, interface_name)?;
                },
                
                // For generic structs
                Type::Struct(name, _) => {
                    // Get the type parameters for this struct
                    // Get type parameters first - clone them to avoid borrowing issues
                    let type_params_opt = self.type_params_map.get(name).cloned();
                    if let Some(type_params) = type_params_opt {
                        // Extract constraints
                        // For this implementation, we assume no constraints
                        // A more complete implementation would extract constraints from the code
                        let constraints = Vec::new();
                        
                        self.register_generic_interface_implementation(
                            name, 
                            &type_params, 
                            interface_name,
                            constraints
                        )?;
                    } else {
                        // If no type parameters found, treat as concrete type
                        self.register_interface_implementation(implementing_type, interface_name)?;
                    }
                },
                
                // For all other types
                _ => {
                    self.register_interface_implementation(implementing_type, interface_name)?;
                }
            }
            
            debug!(
                "Successfully verified and registered {:?} as implementing {}",
                implementing_type,
                interface_name
            );
        }
        
        Ok(implements)
    }
    
    /// Get the shared interface registry instance
    fn get_interface_registry(&self) -> Arc<Mutex<InterfaceRegistry>> {
        // Return the registry that's now stored in the TypeChecker
        self.interface_registry.clone()
    }
}

// Extension trait to add caching to the interface registry
pub trait CachedInterfaceRegistry {
    /// Check if a type implements an interface with caching
    fn check_implementation_cached(
        &mut self,
        type_: &Type,
        interface_name: &str
    ) -> Result<bool, Error>;
    
    /// Clear the cache
    fn clear_cache(&mut self);
    
    /// Get cache statistics (entries, hits, misses)
    fn cache_stats(&self) -> (usize, usize, usize);
    
    /// Get cache hit rate
    fn cache_hit_rate(&self) -> f64;
}

/// A cached version of the interface registry
pub struct CachedRegistry {
    /// The underlying registry
    registry: InterfaceRegistry,
    
    /// The cache for implementation checks
    cache: InterfaceImplementationCache,
}

impl CachedRegistry {
    /// Create a new cached registry
    pub fn new(registry: InterfaceRegistry) -> Self {
        Self {
            registry,
            cache: InterfaceImplementationCache::new(),
        }
    }
    
    /// Create a new cached registry with default implementations
    pub fn new_with_defaults() -> Self {
        Self {
            registry: InterfaceRegistry::new_with_defaults(),
            cache: InterfaceImplementationCache::new(),
        }
    }
    
    /// Create a new cached registry with a specific cache capacity
    pub fn with_capacity(registry: InterfaceRegistry, capacity: usize) -> Self {
        Self {
            registry,
            cache: InterfaceImplementationCache::with_capacity(capacity),
        }
    }
    
    /// Get a reference to the underlying registry
    pub fn registry(&self) -> &InterfaceRegistry {
        &self.registry
    }
    
    /// Get a mutable reference to the underlying registry
    pub fn registry_mut(&mut self) -> &mut InterfaceRegistry {
        &mut self.registry
    }
}

/// A thread-safe cached interface registry that can be shared between components
pub struct ThreadSafeCachedRegistry {
    /// The registry with a mutex for thread safety
    registry: Arc<Mutex<InterfaceRegistry>>,
    
    /// The thread-safe cache
    cache: ThreadSafeInterfaceCache,
}

impl ThreadSafeCachedRegistry {
    /// Create a new thread-safe cached registry
    pub fn new(registry: InterfaceRegistry) -> Self {
        Self {
            registry: Arc::new(Mutex::new(registry)),
            cache: ThreadSafeInterfaceCache::new(),
        }
    }
    
    /// Create a new thread-safe cached registry with default implementations
    pub fn new_with_defaults() -> Self {
        Self {
            registry: Arc::new(Mutex::new(InterfaceRegistry::new_with_defaults())),
            cache: ThreadSafeInterfaceCache::new(),
        }
    }
    
    /// Create a new thread-safe cached registry with a specific cache capacity
    pub fn with_capacity(registry: InterfaceRegistry, capacity: usize) -> Self {
        Self {
            registry: Arc::new(Mutex::new(registry)),
            cache: ThreadSafeInterfaceCache::with_capacity(capacity),
        }
    }
    
    /// Get the underlying registry (acquires lock)
    pub fn registry(&self) -> Arc<Mutex<InterfaceRegistry>> {
        self.registry.clone()
    }
    
    /// Check if a type implements an interface with caching
    #[instrument(skip(self), level = "debug")]
    pub fn check_implementation(&self, type_: &Type, interface_name: &str) -> Result<bool, Error> {
        // First check the cache
        if let Some(result) = self.cache.lookup(type_, interface_name) {
            debug!("Thread-safe cache hit for {:?} implements {}: {}", type_, interface_name, result);
            return Ok(result);
        }
        
        // Not in cache, check the implementation (acquires lock)
        debug!("Thread-safe cache miss for {:?} implements {}", type_, interface_name);
        let registry = self.registry.lock().unwrap();
        let result = registry.check_implementation(type_, interface_name)?;
        
        // Store the result in the cache
        self.cache.store(type_, interface_name, result);
        
        Ok(result)
    }
    
    /// Clear the cache
    pub fn clear_cache(&self) {
        self.cache.clear();
    }
    
    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize, usize) {
        self.cache.stats()
    }
    
    /// Get cache hit rate
    pub fn cache_hit_rate(&self) -> f64 {
        self.cache.hit_rate()
    }
}

// Forward registry methods to the underlying registry
impl CachedInterfaceRegistry for CachedRegistry {
    #[instrument(skip(self), level = "debug")]
    fn check_implementation_cached(
        &mut self,
        type_: &Type,
        interface_name: &str
    ) -> Result<bool, Error> {
        // First check the cache
        if let Some(result) = self.cache.lookup(type_, interface_name) {
            debug!("Cache hit for {:?} implements {}: {}", type_, interface_name, result);
            return Ok(result);
        }
        
        // Not in cache, check the implementation
        debug!("Cache miss for {:?} implements {}, checking implementation", type_, interface_name);
        let result = self.registry.check_implementation(type_, interface_name)?;
        
        // Store the result in the cache
        self.cache.store(type_, interface_name, result);
        
        Ok(result)
    }
    
    fn clear_cache(&mut self) {
        debug!("Clearing interface implementation cache");
        self.cache.clear();
    }
    
    fn cache_stats(&self) -> (usize, usize, usize) {
        self.cache.stats()
    }
    
    fn cache_hit_rate(&self) -> f64 {
        self.cache.hit_rate()
    }
}

// Implement cache wrapper around the registry
impl CachedInterfaceRegistry for InterfaceRegistry {
    #[instrument(skip(self), level = "debug")]
    fn check_implementation_cached(
        &mut self,
        type_: &Type,
        interface_name: &str
    ) -> Result<bool, Error> {
        // Create a temporary cache for this operation
        // This is less efficient than using a persistent cache
        // but provides a way to use caching with existing registry instances
        let mut temp_cache = InterfaceImplementationCache::new();
        
        // Check the implementation
        debug!("Checking if {:?} implements {} (with temporary cache)", type_, interface_name);
        let result = self.check_implementation(type_, interface_name)?;
        
        Ok(result)
    }
    
    fn clear_cache(&mut self) {
        // With a temporary cache approach, this is a no-op
        debug!("No persistent cache to clear");
    }
    
    fn cache_stats(&self) -> (usize, usize, usize) {
        // No persistent cache with the temporary approach
        (0, 0, 0)
    }
    
    fn cache_hit_rate(&self) -> f64 {
        0.0 // No cache stats available
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    
    use crate::tests::common;
    
    #[test]
    fn test_auto_registration_concrete_type() {
        // Set up tracing for the test
        common::tracing::setup();
        
        // Create a type checker with automatic registration
        let mut type_checker = TypeChecker::new();
        
        // Add a struct and interface to the type checker
        let mut fields = std::collections::HashMap::new();
        fields.insert("name".to_string(), Type::Tea);
        fields.insert("age".to_string(), Type::Normie);
        type_checker.register_struct("Person", fields, Vec::new());
        
        // Register methods for the struct
        let method_signatures = vec![
            ("getName".to_string(), Vec::new(), Some(Type::Tea)),
            ("getAge".to_string(), Vec::new(), Some(Type::Normie)),
        ];
        type_checker.struct_methods_map.insert("Person".to_string(), method_signatures);
        
        // Register an interface
        let interface_methods = vec![
            ("getName".to_string(), Vec::new(), Some(Type::Tea)),
        ];
        type_checker.interface_map.insert("Named".to_string(), interface_methods);
        
        // Check and register implementation
        let person_type = Type::Struct("Person".to_string(), Vec::new());
        let named_interface = Type::Interface("Named".to_string(), Vec::new());
        
        // This should check and register automatically
        let result = type_checker.check_and_register_interface_implementation(
            &person_type,
            &named_interface
        ).unwrap();
        
        assert!(result, "Person should implement Named interface");
        
        // Verify that Person is registered as implementing Named
        let registry = type_checker.get_interface_registry();
        let registry = registry.lock().unwrap();
        
        assert!(registry.check_implementation(&person_type, "Named").unwrap());
    }
    
    #[test]
    fn test_auto_registration_generic_type() {
        // Set up tracing for the test
        common::tracing::setup();
        
        // Create a type checker with automatic registration
        let mut type_checker = TypeChecker::new();
        
        // Register a generic struct
        let mut fields = std::collections::HashMap::new();
        fields.insert("items".to_string(), Type::Unknown);
        fields.insert("size".to_string(), Type::Normie);
        type_checker.register_struct("Stack", fields, vec!["T".to_string()]);
        
        // Register methods for the struct
        let method_signatures = vec![
            ("push".to_string(), vec![Type::TypeParam("T".to_string())], None),
            ("pop".to_string(), Vec::new(), Some(Type::TypeParam("T".to_string()))),
            ("isEmpty".to_string(), Vec::new(), Some(Type::Lit)),
        ];
        type_checker.struct_methods_map.insert("Stack".to_string(), method_signatures);
        
        // Register an interface
        let interface_methods = vec![
            ("isEmpty".to_string(), Vec::new(), Some(Type::Lit)),
        ];
        type_checker.interface_map.insert("Container".to_string(), interface_methods);
        
        // Create a generic Stack type
        let stack_type = Type::Struct("Stack".to_string(), Vec::new());
        let container_interface = Type::Interface("Container".to_string(), Vec::new());
        
        // This should check and register automatically
        let result = type_checker.check_and_register_interface_implementation(
            &stack_type,
            &container_interface
        ).unwrap();
        
        assert!(result, "Stack should implement Container interface");
        
        // Verify that Stack is registered as implementing Container
        let registry = type_checker.get_interface_registry();
        let registry = registry.lock().unwrap();
        
        // Check a concrete instantiation of Stack
        let stack_string = Type::Struct(
            "Stack".to_string(), 
            vec![Box::new(Type::Tea)]
        );
        
        assert!(registry.check_implementation(&stack_string, "Container").unwrap());
    }
}