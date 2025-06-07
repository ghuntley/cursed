//! Type assertion error types and handling
//!
//! This module provides specialized error types and utilities for interface type assertions,
//! allowing for better error messages, debugging, and propagation of type assertion failures.
//! It integrates with the enhanced error system for improved diagnostic capabilities.

use std::fmt;
use std::sync::Arc;

use crate::error_enhanced::{CursedError, ErrorKind};
use crate::error::SourceLocation;

/// Specialized error type for type assertion operations
/// 
/// This struct provides detailed context for type assertion failures, including
/// the source interface type, expected concrete type, and the location of the assertion.
/// It's designed to be integrated with the enhanced error system through the From trait.
#[derive(Debug, Clone)]
pub struct TypeAssertionError {
    /// The name or description of the interface type being asserted
    pub interface_type: String,
    /// The concrete type being asserted to
    pub target_type: String,
    /// Optional location in the source code where the assertion happened
    pub location: Option<SourceLocation>,
    /// Optional additional context message
    pub message: Option<String>,
    /// Type ID of the interface (if available)
    pub interface_type_id: Option<u64>,
    /// Type ID of the expected concrete type (if available)
    pub target_type_id: Option<u64>,
    /// The actual type found in the interface (if available)
    pub actual_type: Option<String>,
    /// Type ID of the actual type (if available)
    pub actual_type_id: Option<u64>,
}

impl TypeAssertionError {
    /// Create a new type assertion error with the minimal required information
    pub fn new(interface_type: impl Into<String>, target_type: impl Into<String>) -> Self {
        Self {
            interface_type: interface_type.into(),
            target_type: target_type.into(),
            location: None,
            message: None,
            interface_type_id: None,
            target_type_id: None,
            actual_type: None,
            actual_type_id: None,
        }
    }
    
    /// Add source location information to the error
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
    
    /// Add a custom message to the error
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
    
    /// Add type ID information for the interface
    pub fn with_interface_type_id(mut self, type_id: u64) -> Self {
        self.interface_type_id = Some(type_id);
        self
    }
    
    /// Add type ID information for the target type
    pub fn with_target_type_id(mut self, type_id: u64) -> Self {
        self.target_type_id = Some(type_id);
        self
    }
    
    /// Add information about the actual type found in the interface
    pub fn with_actual_type(mut self, actual_type: impl Into<String>, actual_type_id: Option<u64>) -> Self {
        self.actual_type = Some(actual_type.into());
        self.actual_type_id = actual_type_id;
        self
    }
    
    /// Get a descriptive message for the error
    pub fn get_description(&self) -> String {
        let mut message = format!("Failed to assert that {} is a {}", 
                                  self.interface_type, self.target_type);
        
        if let Some(actual_type) = &self.actual_type {
            message.push_str(&format!(". Actual type was {}", actual_type));
        }
        
        if let Some(custom) = &self.message {
            message.push_str(&format!(": {}", custom));
        }
        
        message
    }
    
    /// Convert to a detailed debugging representation
    pub fn to_detailed_string(&self) -> String {
        let mut result = self.get_description();
        
        // Add type ID information if available
        if let (Some(interface_id), Some(target_id)) = (self.interface_type_id, self.target_type_id) {
            result.push_str(&format!(
                "\nType IDs: interface=0x{:016x}, target=0x{:016x}", 
                interface_id, target_id
            ));
        }
        
        // Add actual type ID if available
        if let Some(actual_id) = self.actual_type_id {
            result.push_str(&format!("\nActual type ID: 0x{:016x}", actual_id));
        }
        
        // Add location if available
        if let Some(loc) = &self.location {
            result.push_str(&format!("\nLocation: {}", loc));
        }
        
        result
    }
}

impl fmt::Display for TypeAssertionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_description())
    }
}

// Conversion from TypeAssertionError to the enhanced CursedError
impl From<TypeAssertionError> for CursedError {
    fn from(err: TypeAssertionError) -> Self {
        let mut error = CursedError::new(ErrorKind::TypeAssertion, err.get_description());
        
        // Add location if available
        if let Some(loc) = err.location {
            error = error.with_location(loc);
        }
        
        // Add type information as context
        error = error.with_context("interface_type", err.interface_type.clone());
        error = error.with_context("target_type", err.target_type.clone());
        
        // Add type IDs if available
        if let Some(interface_id) = err.interface_type_id {
            error = error.with_context("interface_type_id", format!("{:016x}", interface_id));
        }
        
        if let Some(target_id) = err.target_type_id {
            error = error.with_context("target_type_id", format!("{:016x}", target_id));
        }
        
        // Add actual type information if available
        if let Some(actual_type) = err.actual_type {
            error = error.with_context("actual_type", actual_type);
        }
        
        if let Some(actual_id) = err.actual_type_id {
            error = error.with_context("actual_type_id", format!("{:016x}", actual_id));
        }
        
        // Add error code
        error.with_code("ASSERT-001")
    }
}

// Conversion from the old error type to the newer TypeAssertionError
impl From<crate::error::Error> for TypeAssertionError {
    fn from(err: crate::error::Error) -> Self {
        match err {
            crate::error::Error::TypeAssertion(assertion_error) => {
                let assertion_error = assertion_error.clone();
                let mut result = TypeAssertionError::new("unknown", "unknown");
                
                for (key, value) in assertion_error.context() {
                    match key.as_str() {
                        "interface_type" => result.interface_type = value.to_string(),
                        "target_type" => result.target_type = value.to_string(),
                        "actual_type" => result = result.with_actual_type(value.to_string(), None),
                        _ => {}
                    }
                }
                
                let location_opt = assertion_error.location().map(|loc| {
                    SourceLocation::new(loc.line as usize, loc.column as usize)
                });
                
                if let Some(loc) = location_opt {
                    result = result.with_location(loc);
                }
                
                result.with_message(assertion_error.message())
            },
            _ => TypeAssertionError::new("unknown", "unknown")
                .with_message(format!("Original error: {}", err))
        }
    }
}

/// Helper functions for working with type assertion errors
pub mod helpers {
    use super::*;
    
    /// Create a type assertion error with detailed information
    pub fn create_type_assertion_error(
        interface_type: impl Into<String>,
        target_type: impl Into<String>,
        location: Option<SourceLocation>,
        message: Option<String>,
    ) -> TypeAssertionError {
        let mut error = TypeAssertionError::new(interface_type, target_type);
        
        if let Some(loc) = location {
            error = error.with_location(loc);
        }
        
        if let Some(msg) = message {
            error = error.with_message(msg);
        }
        
        error
    }
    
    /// Create a type assertion error with full type ID information
    pub fn create_detailed_assertion_error(
        interface_type: impl Into<String>,
        target_type: impl Into<String>,
        interface_type_id: Option<u64>,
        target_type_id: Option<u64>,
        actual_type: Option<String>,
        actual_type_id: Option<u64>,
        location: Option<SourceLocation>,
    ) -> TypeAssertionError {
        let mut error = TypeAssertionError::new(interface_type, target_type);
        
        if let Some(interface_id) = interface_type_id {
            error = error.with_interface_type_id(interface_id);
        }
        
        if let Some(target_id) = target_type_id {
            error = error.with_target_type_id(target_id);
        }
        
        if let Some(actual) = actual_type {
            error = error.with_actual_type(actual, actual_type_id);
        }
        
        if let Some(loc) = location {
            error = error.with_location(loc);
        }
        
        error
    }
    
    /// Convert from a general error to a type assertion error if possible
    pub fn try_convert_to_assertion_error(err: crate::error::Error) -> TypeAssertionError {
        TypeAssertionError::from(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_assertion_error_creation() {
        let error = TypeAssertionError::new("Stringer", "Person");
        assert_eq!(error.interface_type, "Stringer");
        assert_eq!(error.target_type, "Person");
        assert_eq!(error.get_description(), "Failed to assert that Stringer is a Person");
    }
    
    #[test]
    fn test_type_assertion_error_with_location() {
        let location = SourceLocation {
            line: 42,
            column: 10,
            file: Some("test.csd".to_string()),
            source_line: "    val, ok = obj.(Person)".to_string(),
        };
        
        let error = TypeAssertionError::new("Stringer", "Person")
            .with_location(location.clone());
            
        assert_eq!(error.location.unwrap().line, 42);
    }
    
    #[test]
    fn test_type_assertion_error_with_actual_type() {
        let error = TypeAssertionError::new("Stringer", "Person")
            .with_actual_type("Dog", Some(0x1234567890ABCDEF));
            
        assert_eq!(error.actual_type.unwrap(), "Dog");
        assert_eq!(error.actual_type_id.unwrap(), 0x1234567890ABCDEF);
        assert!(error.get_description().contains("Actual type was Dog"));
    }
    
    #[test]
    fn test_conversion_to_cursed_error() {
        let assertion_error = TypeAssertionError::new("Stringer", "Person")
            .with_actual_type("Dog", Some(0x1234567890ABCDEF));
            
        let cursed_error: CursedError = assertion_error.into();
        
        assert_eq!(cursed_error.kind(), &ErrorKind::TypeAssertion);
        assert!(cursed_error.code().is_some());
    }
}