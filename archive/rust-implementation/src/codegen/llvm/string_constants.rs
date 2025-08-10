//! Centralized String Constants Manager for LLVM Codegen
//! 
//! This module provides a global, thread-safe string constant manager
//! that ensures all string constants are properly deduplicated and
//! assigned unique identifiers across all compilation units.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

/// Global string constant manager that tracks all string constants
/// and ensures unique naming across all compilation units
#[derive(Debug, Clone)]
pub struct StringConstantManager {
    constants: Arc<Mutex<StringConstantManagerInner>>,
}

#[derive(Debug)]
struct StringConstantManagerInner {
    /// Maps string content to constant name and definition
    string_to_constant: HashMap<String, (String, String)>,
    /// Current counter for generating unique constant names
    counter: usize,
}

impl StringConstantManager {
    /// Create a new string constant manager
    pub fn new() -> Self {
        Self {
            constants: Arc::new(Mutex::new(StringConstantManagerInner {
                string_to_constant: HashMap::new(),
                counter: 0,
            })),
        }
    }

    /// Add a string constant and return its reference
    /// If the string already exists, returns the existing reference
    pub fn add_string_constant(&self, content: &str) -> String {
        let mut inner = self.constants.lock().unwrap();
        
        // Check if this string already exists
        if let Some((const_name, _)) = inner.string_to_constant.get(content) {
            // Process escape sequences to get the actual string content for correct length
            let processed_content = content
                .replace("\\n", "\n")
                .replace("\\t", "\t")
                .replace("\\r", "\r")
                .replace("\\\\", "\\")
                .replace("\\\"", "\"");
            let len = processed_content.len() + 1; // +1 for null terminator
            return format!("getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0", len, len, const_name);
        }
        
        // Create new constant
        let const_name = format!("@.str.{}", inner.counter);
        inner.counter += 1;
        
        // Process escape sequences to get the actual string content
        let processed_content = content
            .replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\r", "\r")
            .replace("\\\\", "\\")
            .replace("\\\"", "\"");
        
        let len = processed_content.len() + 1; // +1 for null terminator
        
        // For LLVM IR, we need to convert special characters to their escape sequences
        let llvm_escaped_content = processed_content
            .replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("\n", "\\0A")
            .replace("\t", "\\09")
            .replace("\r", "\\0D");
        
        let constant_definition = format!("{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1", 
            const_name, len, llvm_escaped_content);
        
        inner.string_to_constant.insert(content.to_string(), (const_name.clone(), constant_definition));
        
        // Return getelementptr expression
        format!("getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0", len, len, const_name)
    }

    /// Get all string constant definitions for output
    pub fn get_all_constants(&self) -> Vec<String> {
        let inner = self.constants.lock().unwrap();
        inner.string_to_constant.values()
            .map(|(_, definition)| definition.clone())
            .collect()
    }

    /// Reset the manager (for testing purposes)
    pub fn reset(&self) {
        let mut inner = self.constants.lock().unwrap();
        inner.string_to_constant.clear();
        inner.counter = 0;
    }

    /// Get current counter value (for debugging)
    pub fn get_counter(&self) -> usize {
        let inner = self.constants.lock().unwrap();
        inner.counter
    }
}

impl Default for StringConstantManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global singleton instance of the string constant manager
static GLOBAL_STRING_MANAGER: Lazy<StringConstantManager> = Lazy::new(|| StringConstantManager::new());

/// Get the global string constant manager instance
pub fn get_global_string_manager() -> StringConstantManager {
    GLOBAL_STRING_MANAGER.clone()
}

/// Reset the global string constant manager (for testing)
pub fn reset_global_string_manager() {
    let manager = get_global_string_manager();
    manager.reset();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_constant_deduplication() {
        let manager = StringConstantManager::new();
        
        // Add same string twice
        let ref1 = manager.add_string_constant("Hello, World!");
        let ref2 = manager.add_string_constant("Hello, World!");
        
        // Should get the same reference
        assert_eq!(ref1, ref2);
        
        // Should only have one constant definition
        let constants = manager.get_all_constants();
        assert_eq!(constants.len(), 1);
        assert!(constants[0].contains("Hello, World!"));
    }

    #[test]
    fn test_different_strings() {
        let manager = StringConstantManager::new();
        
        let ref1 = manager.add_string_constant("Hello");
        let ref2 = manager.add_string_constant("World");
        
        // Should get different references
        assert_ne!(ref1, ref2);
        
        // Should have two constant definitions
        let constants = manager.get_all_constants();
        assert_eq!(constants.len(), 2);
    }

    #[test]
    fn test_counter_increment() {
        let manager = StringConstantManager::new();
        
        manager.add_string_constant("First");
        assert_eq!(manager.get_counter(), 1);
        
        manager.add_string_constant("Second");
        assert_eq!(manager.get_counter(), 2);
        
        // Adding duplicate shouldn't increment counter
        manager.add_string_constant("First");
        assert_eq!(manager.get_counter(), 2);
    }
}
