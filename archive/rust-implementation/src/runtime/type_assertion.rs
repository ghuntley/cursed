//! Type Assertion Runtime Support for CURSED
//!
//! This module provides runtime support for type assertion operations
//! including type checking, casting, and proper error handling following
//! CURSED error patterns with yikes/shook/fam keywords.

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::{c_char, c_int, c_void};
use std::sync::{LazyLock, RwLock};

// Type IDs for runtime type checking
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursedTypeId {
    Unknown = 0,
    Integer = 1,
    Float = 2,
    String = 3,
    Boolean = 4,
    Byte = 5,
    Character = 6,
    Array = 100,
    Slice = 101,
    Reference = 200,
    Function = 300,
    Interface = 400,
    Generic = 500,
    Map = 600,
    Channel = 700,
}

impl From<u32> for CursedTypeId {
    fn from(value: u32) -> Self {
        match value {
            0 => CursedTypeId::Unknown,
            1 => CursedTypeId::Integer,
            2 => CursedTypeId::Float,
            3 => CursedTypeId::String,
            4 => CursedTypeId::Boolean,
            5 => CursedTypeId::Byte,
            6 => CursedTypeId::Character,
            100 => CursedTypeId::Array,
            101 => CursedTypeId::Slice,
            200 => CursedTypeId::Reference,
            300 => CursedTypeId::Function,
            400 => CursedTypeId::Interface,
            500 => CursedTypeId::Generic,
            600 => CursedTypeId::Map,
            700 => CursedTypeId::Channel,
            _ => CursedTypeId::Unknown,
        }
    }
}

// CURSED error type for type assertions following yikes/shook/fam pattern
#[derive(Debug, Clone)]
pub struct TypeAssertionError {
    pub message: String,
    pub source_type: CursedTypeId,
    pub target_type: CursedTypeId,
    pub context: Option<String>,
}

impl fmt::Display for TypeAssertionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let source_name = get_type_name(self.source_type);
        let target_name = get_type_name(self.target_type);
        
        if let Some(context) = &self.context {
            write!(f, "Type assertion yikes: {} (context: {})", self.message, context)
        } else {
            write!(f, "Type assertion yikes: cannot convert {} to {}", source_name, target_name)
        }
    }
}

impl std::error::Error for TypeAssertionError {}

// Result type for CURSED type assertions following the yikes pattern
pub type TypeAssertionResult<T> = Result<T, TypeAssertionError>;

// Runtime type information structure
#[derive(Debug, Clone)]
pub struct CursedTypeInfo {
    pub type_id: CursedTypeId,
    pub type_name: String,
    pub size: usize,
    pub metadata: Option<String>,
}

// Global type registry
static TYPE_REGISTRY: LazyLock<RwLock<HashMap<CursedTypeId, CursedTypeInfo>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

// Type names for error messages
fn get_type_name(type_id: CursedTypeId) -> &'static str {
    match type_id {
        CursedTypeId::Unknown => "unknown",
        CursedTypeId::Integer => "normie",
        CursedTypeId::Float => "drip",
        CursedTypeId::String => "tea",
        CursedTypeId::Boolean => "lit",
        CursedTypeId::Byte => "byte",
        CursedTypeId::Character => "sip",
        CursedTypeId::Array => "array",
        CursedTypeId::Slice => "slice",
        CursedTypeId::Reference => "reference",
        CursedTypeId::Function => "function",
        CursedTypeId::Interface => "interface",
        CursedTypeId::Generic => "generic",
        CursedTypeId::Map => "map",
        CursedTypeId::Channel => "channel",
    }
}

// Type compatibility checking
fn is_compatible_type(source: CursedTypeId, target: CursedTypeId) -> bool {
    match source {
        CursedTypeId::Integer => {
            matches!(target, CursedTypeId::Float | CursedTypeId::Boolean | CursedTypeId::Byte)
        }
        CursedTypeId::Float => matches!(target, CursedTypeId::Integer),
        CursedTypeId::Boolean => matches!(target, CursedTypeId::Integer),
        CursedTypeId::Byte => matches!(target, CursedTypeId::Integer | CursedTypeId::Character),
        CursedTypeId::Character => matches!(target, CursedTypeId::Byte | CursedTypeId::Integer),
        CursedTypeId::String => matches!(target, CursedTypeId::Array), // String to []byte
        CursedTypeId::Array => matches!(target, CursedTypeId::Slice),
        CursedTypeId::Slice => matches!(target, CursedTypeId::Array),
        CursedTypeId::Reference => true, // References can be cast to most types
        CursedTypeId::Interface => true, // Interface can be cast to concrete types
        CursedTypeId::Generic => true,   // Generic types can be cast based on constraints
        _ => false,
    }
}

// Safe type casting implementation with proper error handling
fn safe_cast_value(value: *mut c_void, source: CursedTypeId, target: CursedTypeId) -> TypeAssertionResult<*mut c_void> {
    if source == target {
        return Ok(value);
    }
    
    if !is_compatible_type(source, target) {
        return Err(TypeAssertionError {
            message: format!("Incompatible type conversion from {} to {}", 
                           get_type_name(source), get_type_name(target)),
            source_type: source,
            target_type: target,
            context: Some("safe_cast_value".to_string()),
        });
    }
    
    // For now, return the original value if compatible
    // In a full implementation, this would perform actual type conversions
    Ok(value)
}

// Legacy wrapper for C compatibility (unsafe)
fn cast_value(value: *mut c_void, source: CursedTypeId, target: CursedTypeId) -> *mut c_void {
    match safe_cast_value(value, source, target) {
        Ok(result) => result,
        Err(_) => std::ptr::null_mut(), // Return null on error for C compatibility
    }
}

// Runtime initialization
pub fn initialize_type_assertion_runtime() {
    let mut registry = TYPE_REGISTRY.write().unwrap();
    
    // Register built-in types
    registry.insert(CursedTypeId::Integer, CursedTypeInfo {
        type_id: CursedTypeId::Integer,
        type_name: "normie".to_string(),
        size: 4,
        metadata: None,
    });
    
    registry.insert(CursedTypeId::Float, CursedTypeInfo {
        type_id: CursedTypeId::Float,
        type_name: "drip".to_string(),
        size: 8,
        metadata: None,
    });
    
    registry.insert(CursedTypeId::String, CursedTypeInfo {
        type_id: CursedTypeId::String,
        type_name: "tea".to_string(),
        size: 8,
        metadata: None,
    });
    
    registry.insert(CursedTypeId::Boolean, CursedTypeInfo {
        type_id: CursedTypeId::Boolean,
        type_name: "lit".to_string(),
        size: 1,
        metadata: None,
    });
    
    registry.insert(CursedTypeId::Byte, CursedTypeInfo {
        type_id: CursedTypeId::Byte,
        type_name: "byte".to_string(),
        size: 1,
        metadata: None,
    });
    
    registry.insert(CursedTypeId::Character, CursedTypeInfo {
        type_id: CursedTypeId::Character,
        type_name: "sip".to_string(),
        size: 1,
        metadata: None,
    });
}

// Runtime cleanup
pub fn cleanup_type_assertion_runtime() {
    let mut registry = TYPE_REGISTRY.write().unwrap();
    registry.clear();
}

// C-compatible runtime functions

#[no_mangle]
pub extern "C" fn cursed_check_type_compatibility(
    _value: *mut c_void,
    source_type_id: c_int,
    target_type_id: c_int,
) -> bool {
    let source = CursedTypeId::from(source_type_id as u32);
    let target = CursedTypeId::from(target_type_id as u32);
    
    // Same type is always compatible
    if source == target {
        return true;
    }
    
    is_compatible_type(source, target)
}

#[no_mangle]
pub extern "C" fn cursed_check_interface_type(_value: *mut c_void) -> bool {
    // For now, simplified - would check vtable compatibility
    // In a full implementation, this would check if the value implements the interface
    true
}

#[no_mangle]
pub extern "C" fn cursed_check_generic_type(_value: *mut c_void) -> bool {
    // For now, simplified - would check type parameters
    // In a full implementation, this would check generic type constraints
    true
}

#[no_mangle]
pub extern "C" fn cursed_check_array_type(_value: *mut c_void) -> bool {
    // For now, simplified - would check element types and dimensions
    // In a full implementation, this would check array bounds and element types
    true
}

#[no_mangle]
pub extern "C" fn cursed_check_function_type(_value: *mut c_void) -> bool {
    // For now, simplified - would check signature compatibility
    // In a full implementation, this would check function signatures
    true
}

#[no_mangle]
pub extern "C" fn cursed_cast_type(
    value: *mut c_void,
    source_type_id: c_int,
    target_type_id: c_int,
) -> *mut c_void {
    let source = CursedTypeId::from(source_type_id as u32);
    let target = CursedTypeId::from(target_type_id as u32);
    
    cast_value(value, source, target)
}

#[no_mangle]
pub extern "C" fn cursed_empty_string() -> *mut c_char {
    static EMPTY_STRING: &[u8] = b"\0";
    EMPTY_STRING.as_ptr() as *mut c_char
}

#[no_mangle]
pub extern "C" fn cursed_null_value() -> *mut c_void {
    std::ptr::null_mut()
}

// Safe type assertion with proper error handling (CURSED pattern)
#[no_mangle]
pub extern "C" fn cursed_safe_type_assertion(
    value: *mut c_void,
    source_type_id: c_int,
    target_type_id: c_int,
    error_out: *mut *mut c_char,
) -> *mut c_void {
    let source = CursedTypeId::from(source_type_id as u32);
    let target = CursedTypeId::from(target_type_id as u32);
    
    match safe_cast_value(value, source, target) {
        Ok(result) => {
            // Clear error pointer
            if !error_out.is_null() {
                unsafe { *error_out = std::ptr::null_mut(); }
            }
            result
        },
        Err(error) => {
            // Set error message following CURSED yikes pattern
            if !error_out.is_null() {
                let error_msg = CString::new(format!("yikes: {}", error.message))
                    .unwrap_or_else(|_| CString::new("yikes: type assertion failed").unwrap());
                unsafe { *error_out = error_msg.into_raw(); }
            }
            std::ptr::null_mut()
        }
    }
}

// Create a TypeAssertionError from C types
#[no_mangle]
pub extern "C" fn cursed_create_type_assertion_error(
    source_type_id: c_int,
    target_type_id: c_int,
    context: *const c_char,
) -> *mut TypeAssertionError {
    let source = CursedTypeId::from(source_type_id as u32);
    let target = CursedTypeId::from(target_type_id as u32);
    
    let context_str = if context.is_null() {
        None
    } else {
        unsafe {
            CStr::from_ptr(context)
                .to_string_lossy()
                .to_string()
                .into()
        }
    };
    
    let error = TypeAssertionError {
        message: format!("Type assertion failed: {} -> {}", 
                        get_type_name(source), get_type_name(target)),
        source_type: source,
        target_type: target,
        context: context_str,
    };
    
    Box::into_raw(Box::new(error))
}

// Recovery-friendly type assertion (fam pattern)
#[no_mangle]
pub extern "C" fn cursed_recoverable_type_assertion(
    value: *mut c_void,
    source_type_id: c_int,
    target_type_id: c_int,
) -> *mut c_void {
    let source = CursedTypeId::from(source_type_id as u32);
    let target = CursedTypeId::from(target_type_id as u32);
    
    // Use safe casting for recovery-friendly assertions
    match safe_cast_value(value, source, target) {
        Ok(result) => result,
        Err(error) => {
            // Log error for debugging but don't panic
            eprintln!("CURSED: Type assertion error in fam block: {}", error);
            std::ptr::null_mut()
        }
    }
}

// Legacy panic function (deprecated, use safe versions)
#[no_mangle]
#[deprecated(note = "Use cursed_safe_type_assertion or cursed_recoverable_type_assertion instead")]
pub extern "C" fn cursed_panic_type_assertion(source_type_id: c_int, target_type_id: c_int) -> ! {
    let source = CursedTypeId::from(source_type_id as u32);
    let target = CursedTypeId::from(target_type_id as u32);
    
    let source_name = get_type_name(source);
    let target_name = get_type_name(target);
    
    eprintln!("CURSED YIKES: Type assertion failed - cannot convert {} to {}", source_name, target_name);
    eprintln!("Consider using cursed_safe_type_assertion for proper error handling");
    
    // Create a proper TypeAssertionError instead of panicking
    let error = TypeAssertionError {
        message: format!("Type assertion panic: {} -> {}", source_name, target_name),
        source_type: source,
        target_type: target,
        context: Some("legacy_panic_function".to_string()),
    };
    
    // Still panic for backward compatibility, but with structured error
    eprintln!("CURSED Runtime Error: {}", error);
    std::process::exit(1);
}

#[no_mangle]
pub extern "C" fn cursed_register_type(
    type_id: c_int,
    type_name: *const c_char,
    size: usize,
    _metadata: *mut c_void,
) {
    let type_id = CursedTypeId::from(type_id as u32);
    
    let type_name_str = if type_name.is_null() {
        "unknown".to_string()
    } else {
        unsafe {
            CStr::from_ptr(type_name)
                .to_string_lossy()
                .to_string()
        }
    };
    
    let mut registry = TYPE_REGISTRY.write().unwrap();
    registry.insert(type_id, CursedTypeInfo {
        type_id,
        type_name: type_name_str,
        size,
        metadata: None,
    });
}

#[no_mangle]
pub extern "C" fn cursed_get_type_info(type_id: c_int) -> *mut CursedTypeInfo {
    let type_id = CursedTypeId::from(type_id as u32);
    let registry = TYPE_REGISTRY.read().unwrap();
    
    if let Some(info) = registry.get(&type_id) {
        let boxed_info = Box::new(info.clone());
        Box::into_raw(boxed_info)
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn cursed_init_type_assertion_runtime() {
    initialize_type_assertion_runtime();
}

#[no_mangle]
pub extern "C" fn cursed_cleanup_type_assertion_runtime() {
    cleanup_type_assertion_runtime();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    
    #[test]
    fn test_type_compatibility() {
        // Test same type compatibility
        assert!(is_compatible_type(CursedTypeId::Integer, CursedTypeId::Integer));
        
        // Test integer to float compatibility
        assert!(is_compatible_type(CursedTypeId::Integer, CursedTypeId::Float));
        
        // Test float to integer compatibility
        assert!(is_compatible_type(CursedTypeId::Float, CursedTypeId::Integer));
        
        // Test incompatible types
        assert!(!is_compatible_type(CursedTypeId::String, CursedTypeId::Integer));
    }
    
    #[test]
    fn test_type_registry() {
        initialize_type_assertion_runtime();
        
        let registry = TYPE_REGISTRY.read().unwrap();
        assert!(registry.contains_key(&CursedTypeId::Integer));
        assert!(registry.contains_key(&CursedTypeId::Float));
        assert!(registry.contains_key(&CursedTypeId::String));
        assert!(registry.contains_key(&CursedTypeId::Boolean));
        
        cleanup_type_assertion_runtime();
    }
    
    #[test]
    fn test_runtime_functions() {
        // Test type compatibility check
        assert!(cursed_check_type_compatibility(
            std::ptr::null_mut(),
            CursedTypeId::Integer as c_int,
            CursedTypeId::Float as c_int
        ));
        
        // Test empty string
        let empty_str = cursed_empty_string();
        assert!(!empty_str.is_null());
        
        // Test null value
        let null_val = cursed_null_value();
        assert!(null_val.is_null());
    }
    
    #[test]
    fn test_safe_cast_value_success() {
        // Test successful cast (same type)
        let dummy_value = 42i32 as *mut c_void;
        let result = safe_cast_value(dummy_value, CursedTypeId::Integer, CursedTypeId::Integer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), dummy_value);
        
        // Test successful cast (compatible types)
        let result = safe_cast_value(dummy_value, CursedTypeId::Integer, CursedTypeId::Float);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_safe_cast_value_failure() {
        let dummy_value = 42i32 as *mut c_void;
        
        // Test failed cast (incompatible types)
        let result = safe_cast_value(dummy_value, CursedTypeId::String, CursedTypeId::Integer);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.source_type, CursedTypeId::String);
        assert_eq!(error.target_type, CursedTypeId::Integer);
        assert!(error.message.contains("Incompatible type conversion"));
    }
    
    #[test]
    fn test_type_assertion_error_display() {
        let error = TypeAssertionError {
            message: "Test error message".to_string(),
            source_type: CursedTypeId::String,
            target_type: CursedTypeId::Integer,
            context: Some("test_context".to_string()),
        };
        
        let display_str = format!("{}", error);
        assert!(display_str.contains("Type assertion yikes"));
        assert!(display_str.contains("test_context"));
    }
    
    #[test]
    fn test_cursed_safe_type_assertion_success() {
        let dummy_value = 42i32 as *mut c_void;
        let mut error_ptr: *mut c_char = std::ptr::null_mut();
        
        // Test successful assertion
        let result = cursed_safe_type_assertion(
            dummy_value,
            CursedTypeId::Integer as c_int,
            CursedTypeId::Integer as c_int,
            &mut error_ptr as *mut *mut c_char,
        );
        
        assert_eq!(result, dummy_value);
        assert!(error_ptr.is_null());
    }
    
    #[test]
    fn test_cursed_safe_type_assertion_failure() {
        let dummy_value = 42i32 as *mut c_void;
        let mut error_ptr: *mut c_char = std::ptr::null_mut();
        
        // Test failed assertion
        let result = cursed_safe_type_assertion(
            dummy_value,
            CursedTypeId::String as c_int,
            CursedTypeId::Integer as c_int,
            &mut error_ptr as *mut *mut c_char,
        );
        
        assert!(result.is_null());
        assert!(!error_ptr.is_null());
        
        // Verify error message follows CURSED yikes pattern
        let error_str = unsafe { CStr::from_ptr(error_ptr).to_string_lossy() };
        assert!(error_str.starts_with("yikes:"));
        
        // Cleanup
        unsafe {
            let _ = CString::from_raw(error_ptr);
        }
    }
    
    #[test]
    fn test_cursed_recoverable_type_assertion() {
        let dummy_value = 42i32 as *mut c_void;
        
        // Test successful recoverable assertion
        let result = cursed_recoverable_type_assertion(
            dummy_value,
            CursedTypeId::Integer as c_int,
            CursedTypeId::Float as c_int,
        );
        assert_eq!(result, dummy_value);
        
        // Test failed recoverable assertion (should not panic)
        let result = cursed_recoverable_type_assertion(
            dummy_value,
            CursedTypeId::String as c_int,
            CursedTypeId::Integer as c_int,
        );
        assert!(result.is_null());
    }
    
    #[test]
    fn test_cursed_create_type_assertion_error() {
        let context = CString::new("test_context").unwrap();
        let error_ptr = cursed_create_type_assertion_error(
            CursedTypeId::String as c_int,
            CursedTypeId::Integer as c_int,
            context.as_ptr(),
        );
        
        assert!(!error_ptr.is_null());
        
        let error = unsafe { Box::from_raw(error_ptr) };
        assert_eq!(error.source_type, CursedTypeId::String);
        assert_eq!(error.target_type, CursedTypeId::Integer);
        assert!(error.context.is_some());
        assert_eq!(error.context.unwrap(), "test_context");
    }
    
    #[test]
    fn test_error_propagation_patterns() {
        // Test error propagation following CURSED shook pattern
        fn test_function() -> TypeAssertionResult<*mut c_void> {
            let dummy_value = 42i32 as *mut c_void;
            safe_cast_value(dummy_value, CursedTypeId::String, CursedTypeId::Integer)
        }
        
        let result = test_function();
        assert!(result.is_err());
        
        // Test error propagation chain
        fn wrapper_function() -> TypeAssertionResult<*mut c_void> {
            test_function()
        }
        
        let wrapped_result = wrapper_function();
        assert!(wrapped_result.is_err());
    }
    
    #[test]
    fn test_fam_recovery_pattern() {
        // Simulate fam recovery pattern
        fn recoverable_operation() -> Option<*mut c_void> {
            let dummy_value = 42i32 as *mut c_void;
            match safe_cast_value(dummy_value, CursedTypeId::String, CursedTypeId::Integer) {
                Ok(result) => Some(result),
                Err(_) => None, // Error handled gracefully
            }
        }
        
        let result = recoverable_operation();
        assert!(result.is_none()); // Should handle error gracefully
    }
    
    #[test]
    fn test_multiple_error_handling_scenarios() {
        let test_cases = vec![
            (CursedTypeId::Integer, CursedTypeId::Integer, true),
            (CursedTypeId::Integer, CursedTypeId::Float, true),
            (CursedTypeId::Float, CursedTypeId::Integer, true),
            (CursedTypeId::String, CursedTypeId::Integer, false),
            (CursedTypeId::Boolean, CursedTypeId::String, false),
        ];
        
        for (source, target, should_succeed) in test_cases {
            let dummy_value = 42i32 as *mut c_void;
            let result = safe_cast_value(dummy_value, source, target);
            
            if should_succeed {
                assert!(result.is_ok(), "Expected success for {:?} -> {:?}", source, target);
            } else {
                assert!(result.is_err(), "Expected failure for {:?} -> {:?}", source, target);
            }
        }
    }
}
