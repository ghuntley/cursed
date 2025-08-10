//! Safety checks for FFI operations
//!
//! This module provides comprehensive safety validation for FFI operations
//! including type checking, memory validation, and runtime safety checks.

use crate::error::CursedError;
use super::{FfiValue, FfiType};

/// Safety checker for FFI operations
pub struct SafetyChecker {
    /// Safety rules configuration
    config: SafetyConfig,
}

/// Safety configuration
#[derive(Debug, Clone)]
pub struct SafetyConfig {
    /// Enable type validation
    pub enable_type_validation: bool,
    
    /// Enable memory validation
    pub enable_memory_validation: bool,
    
    /// Enable null pointer checks
    pub enable_null_checks: bool,
    
    /// Enable range checks
    pub enable_range_checks: bool,
    
    /// Enable buffer overflow checks
    pub enable_buffer_checks: bool,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            enable_type_validation: true,
            enable_memory_validation: true,
            enable_null_checks: true,
            enable_range_checks: true,
            enable_buffer_checks: true,
        }
    }
}

impl SafetyChecker {
    /// Create new safety checker
    pub fn new() -> Self {
        Self {
            config: SafetyConfig::default(),
        }
    }
    
    /// Create safety checker with custom configuration
    pub fn with_config(config: SafetyConfig) -> Self {
        Self { config }
    }
    
    /// Validate function call safety
    pub fn validate_function_call(&self, function_name: &str, args: &[FfiValue]) -> Result<(), CursedError> {
        // Type validation
        if self.config.enable_type_validation {
            self.validate_argument_types(function_name, args)?;
        }
        
        // Memory validation
        if self.config.enable_memory_validation {
            self.validate_memory_safety(args)?;
        }
        
        // Null pointer checks
        if self.config.enable_null_checks {
            self.check_null_pointers(args)?;
        }
        
        // Range checks
        if self.config.enable_range_checks {
            self.check_value_ranges(args)?;
        }
        
        // Buffer overflow checks
        if self.config.enable_buffer_checks {
            self.check_buffer_safety(args)?;
        }
        
        Ok(())
    }
    
    /// Validate result safety
    pub fn validate_result(&self, result: &FfiValue) -> Result<(), CursedError> {
        // Basic result validation
        match result {
            FfiValue::Pointer(ptr) => {
                if ptr.is_null() {
                    return Err(CursedError::General("Function returned null pointer".to_string()));
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    fn validate_argument_types(&self, function_name: &str, args: &[FfiValue]) -> Result<(), CursedError> {
        // This would contain actual type validation logic
        // For now, just check for basic type consistency
        for (i, arg) in args.iter().enumerate() {
            match arg {
                FfiValue::Void => {
                    return Err(CursedError::General(format!(
                        "Argument {} to {} cannot be void",
                        i, function_name
                    )));
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    fn validate_memory_safety(&self, args: &[FfiValue]) -> Result<(), CursedError> {
        // Memory safety validation
        for arg in args {
            match arg {
                FfiValue::Pointer(ptr) => {
                    if !ptr.is_null() {
                        // Would validate memory region
                    }
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    fn check_null_pointers(&self, args: &[FfiValue]) -> Result<(), CursedError> {
        for (i, arg) in args.iter().enumerate() {
            match arg {
                FfiValue::Pointer(ptr) => {
                    if ptr.is_null() {
                        return Err(CursedError::General(format!(
                            "Null pointer passed as argument {}",
                            i
                        )));
                    }
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    fn check_value_ranges(&self, args: &[FfiValue]) -> Result<(), CursedError> {
        for (i, arg) in args.iter().enumerate() {
            match arg {
                FfiValue::SignedInteger(val) => {
                    if *val < i32::MIN as i64 || *val > i32::MAX as i64 {
                        return Err(CursedError::General(format!(
                            "Integer argument {} out of range: {}",
                            i, val
                        )));
                    }
                }
                FfiValue::UnsignedInteger(val) => {
                    if *val > u32::MAX as u64 {
                        return Err(CursedError::General(format!(
                            "Unsigned integer argument {} out of range: {}",
                            i, val
                        )));
                    }
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    fn check_buffer_safety(&self, args: &[FfiValue]) -> Result<(), CursedError> {
        for (i, arg) in args.iter().enumerate() {
            match arg {
                FfiValue::Array(arr) => {
                    if arr.len() > 1024 * 1024 {
                        return Err(CursedError::General(format!(
                            "Array argument {} too large: {} elements",
                            i, arr.len()
                        )));
                    }
                }
                FfiValue::String(s) => {
                    if s.len() > 1024 * 1024 {
                        return Err(CursedError::General(format!(
                            "String argument {} too large: {} bytes",
                            i, s.len()
                        )));
                    }
                }
                _ => {}
            }
        }
        
        Ok(())
    }
}

impl Default for SafetyChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_safety_checker_creation() {
        let checker = SafetyChecker::new();
        assert!(checker.config.enable_type_validation);
        assert!(checker.config.enable_memory_validation);
        assert!(checker.config.enable_null_checks);
        assert!(checker.config.enable_range_checks);
        assert!(checker.config.enable_buffer_checks);
    }
    
    #[test]
    fn test_valid_function_call() {
        let checker = SafetyChecker::new();
        let args = vec![FfiValue::SignedInteger(42), FfiValue::String("test".to_string())];
        
        let result = checker.validate_function_call("test_function", &args);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_null_pointer_check() {
        let checker = SafetyChecker::new();
        let args = vec![FfiValue::Pointer(std::ptr::null_mut())];
        
        let result = checker.validate_function_call("test_function", &args);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_range_check() {
        let checker = SafetyChecker::new();
        let args = vec![FfiValue::SignedInteger(i64::MAX)];
        
        let result = checker.validate_function_call("test_function", &args);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_buffer_size_check() {
        let checker = SafetyChecker::new();
        let large_array = vec![FfiValue::SignedInteger(0); 2 * 1024 * 1024];
        let args = vec![FfiValue::Array(large_array)];
        
        let result = checker.validate_function_call("test_function", &args);
        assert!(result.is_err());
    }
}
