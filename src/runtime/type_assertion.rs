//! Type Assertion Runtime Support for CURSED
//!
//! This module provides runtime support for type assertion operations
//! including type checking, casting, and panic handling.

use std::collections::HashMap;
use std::ffi::{CStr, CString};
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

// Type casting implementation
fn cast_value(value: *mut c_void, source: CursedTypeId, target: CursedTypeId) -> *mut c_void {
    if source == target {
        return value;
    }
    
    // For now, return the original value
    // In a full implementation, this would perform actual type conversions
    value
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

#[no_mangle]
pub extern "C" fn cursed_panic_type_assertion(source_type_id: c_int, target_type_id: c_int) -> ! {
    let source = CursedTypeId::from(source_type_id as u32);
    let target = CursedTypeId::from(target_type_id as u32);
    
    let source_name = get_type_name(source);
    let target_name = get_type_name(target);
    
    eprintln!("CURSED PANIC: Type assertion failed - cannot convert {} to {}", source_name, target_name);
    eprintln!("This is a type assertion panic in CURSED runtime");
    
    // Trigger panic with detailed message
    panic!("Type assertion failed: {} -> {}", source_name, target_name);
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
}
