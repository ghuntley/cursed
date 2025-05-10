//! # Improved Interface Type Registry for Runtime Type Information
//! 
//! This module provides an improved interface type registry that
//! fully supports runtime type information during type assertions.
//! This enhancement significantly improves error reporting and debugging capabilities.

use std::collections::HashMap;

use tracing::{debug, error, info, instrument, trace, warn, Level};

use crate::error::Error;

/// Interface type registry that maintains a mapping of type IDs to type names
/// for improved debugging and error reporting
pub struct ImprovedTypeRegistry {
    /// Maps type IDs to corresponding type names
    type_id_to_name: HashMap<u64, String>,
    
    /// Count of registered types
    type_count: usize,
}

impl ImprovedTypeRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            type_id_to_name: HashMap::new(),
            type_count: 0,
        }
    }
    
    /// Register a type with the registry
    pub fn register_type(&mut self, type_id: u64, type_name: String) {
        debug!("Registered type in registry: {} -> {}", type_id, type_name);
        self.type_id_to_name.insert(type_id, type_name);
        self.type_count = self.type_id_to_name.len();
    }
    
    /// Look up a type name by its ID
    pub fn get_type_name(&self, type_id: u64) -> Option<&String> {
        self.type_id_to_name.get(&type_id)
    }
    
    /// Get all registered types
    pub fn all_types(&self) -> Vec<(u64, String)> {
        self.type_id_to_name
            .iter()
            .map(|(&id, name)| (id, name.clone()))
            .collect()
    }
    
    /// Get the count of registered types
    pub fn type_count(&self) -> usize {
        self.type_count
    }
    
    /// Create a human-readable report of all registered types
    pub fn generate_type_report(&self) -> String {
        let mut report = String::from("Type Registry Contents:\n");
        report.push_str("=======================\n");
        
        let mut types: Vec<_> = self.type_id_to_name.iter().collect();
        types.sort_by_key(|&(id, _)| *id);
        
        for (id, name) in types {
            report.push_str(&format!("Type ID: {:10} | Name: {}\n", id, name));
        }
        
        report.push_str(&format!("\nTotal types registered: {}\n", self.type_count));
        report
    }
}

/// Default implementation creates an empty registry
impl Default for ImprovedTypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Register the improved type registry implementation
pub fn register_improved_type_registry() {
    debug!("Registering improved type registry implementation");
    // This is a placeholder for initialization during LlvmCodeGenerator creation
}