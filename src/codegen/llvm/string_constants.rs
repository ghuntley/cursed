//! Centralized String Constants Manager for LLVM Codegen
//! 
//! This module provides a global, thread-safe string constant manager
//! that ensures all string constants are properly deduplicated and
//! assigned unique identifiers across all compilation units.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
            let len = content.len() + 1; // +1 for null terminator
            return format!("getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0", len, len, const_name);
        }
        
        // Create new constant
        let const_name = format!("@.str.{}", inner.counter);
        inner.counter += 1;
        
        let len = content.len() + 1; // +1 for null terminator
        let escaped_content = content.replace("\"", "\\\"");
        let constant_definition = format!("{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1", 
            const_name, len, escaped_content);
        
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
static mut GLOBAL_STRING_MANAGER: Option<StringConstantManager> = None;
static INIT_ONCE: std::sync::Once = std::sync::Once::new();

/// Get the global string constant manager instance
pub fn get_global_string_manager() -> StringConstantManager {
    unsafe {
        INIT_ONCE.call_once(|| {
            GLOBAL_STRING_MANAGER = Some(StringConstantManager::new());
        });
        GLOBAL_STRING_MANAGER.as_ref().unwrap().clone()
    }
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
