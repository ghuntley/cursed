//! Runtime support for type switches in CURSED
//!
//! This module provides runtime type information and type checking
//! capabilities for type switch expressions.

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

/// Runtime type information structure
#[repr(C)]
pub struct RuntimeTypeInfo {
    pub type_name: *const c_char,
    pub type_id: u64,
    pub size: usize,
    pub is_interface: bool,
    pub interface_methods: *const InterfaceMethodInfo,
    pub interface_method_count: usize,
}

/// Interface method information for runtime dispatch
#[repr(C)]
pub struct InterfaceMethodInfo {
    pub method_name: *const c_char,
    pub method_signature: *const c_char,
    pub method_index: usize,
}

/// Global type registry for runtime type checking
static mut TYPE_REGISTRY: Option<HashMap<u64, RuntimeTypeInfo>> = None;

/// Initialize the type registry
pub fn initialize_type_registry() {
    unsafe {
        TYPE_REGISTRY = Some(HashMap::new());
    }
}

/// Register a type with runtime type information
pub fn register_type(type_info: RuntimeTypeInfo) {
    unsafe {
        if let Some(ref mut registry) = TYPE_REGISTRY {
            registry.insert(type_info.type_id, type_info);
        }
    }
}

/// Get runtime type information for a value
#[no_mangle]
pub extern "C" fn cursed_get_runtime_type_info(value: *const c_void) -> *const RuntimeTypeInfo {
    if value.is_null() {
        return std::ptr::null();
    }
    
    unsafe {
        // For now, we'll use a simplified approach based on memory patterns
        // In a real implementation, this would examine the value's metadata
        
        // Read the first few bytes to determine type
        let value_ptr = value as *const u8;
        let first_byte = *value_ptr;
        
        // Simple heuristic based on value patterns
        // This is a simplified implementation for demonstration
        let type_id = match first_byte {
            0x00..=0x01 => get_type_id_by_name("lit"),    // Boolean values
            0x02..=0x7F => get_type_id_by_name("normie"), // Small integers
            0x80..=0xFF => get_type_id_by_name("tea"),    // String pointers
        };
        
        // Look up in registry
        if let Some(ref registry) = TYPE_REGISTRY {
            if let Some(type_info) = registry.get(&type_id) {
                return type_info as *const RuntimeTypeInfo;
            }
        }
        
        std::ptr::null()
    }
}

/// Check if a value is of a specific type
#[no_mangle]
pub extern "C" fn cursed_check_type(type_info: *const RuntimeTypeInfo, expected_type: *const c_char) -> bool {
    if type_info.is_null() || expected_type.is_null() {
        return false;
    }
    
    unsafe {
        let type_info_ref = &*type_info;
        let expected_type_str = CStr::from_ptr(expected_type).to_str().unwrap_or("");
        let actual_type_str = CStr::from_ptr(type_info_ref.type_name).to_str().unwrap_or("");
        
        actual_type_str == expected_type_str
    }
}

/// Check if a value implements a specific interface
#[no_mangle]
pub extern "C" fn cursed_check_interface(type_info: *const RuntimeTypeInfo, interface_name: *const c_char) -> bool {
    if type_info.is_null() || interface_name.is_null() {
        return false;
    }
    
    unsafe {
        let type_info_ref = &*type_info;
        let interface_name_str = CStr::from_ptr(interface_name).to_str().unwrap_or("");
        
        // If this is already an interface type, check the name
        if type_info_ref.is_interface {
            let actual_interface_str = CStr::from_ptr(type_info_ref.type_name).to_str().unwrap_or("");
            return actual_interface_str == interface_name_str;
        }
        
        // Check if the type implements the interface by checking method table
        if type_info_ref.interface_method_count > 0 && !type_info_ref.interface_methods.is_null() {
            let methods = std::slice::from_raw_parts(type_info_ref.interface_methods, type_info_ref.interface_method_count);
            
            // For now, we'll check if the interface name matches any method's declaring interface
            // In a real implementation, this would be more sophisticated
            for method in methods {
                if !method.method_name.is_null() {
                    let method_name = CStr::from_ptr(method.method_name).to_str().unwrap_or("");
                    if method_name.starts_with(interface_name_str) {
                        return true;
                    }
                }
            }
        }
        
        false
    }
}

/// Cast a value to a specific type (used in type switch variable binding)
#[no_mangle]
pub extern "C" fn cursed_type_cast(value: *const c_void, from_type: *const RuntimeTypeInfo, to_type: *const RuntimeTypeInfo) -> *const c_void {
    if value.is_null() || from_type.is_null() || to_type.is_null() {
        return std::ptr::null();
    }
    
    unsafe {
        let from_type_ref = &*from_type;
        let to_type_ref = &*to_type;
        
        // Direct type match - no casting needed
        if from_type_ref.type_id == to_type_ref.type_id {
            return value;
        }
        
        // Handle interface unwrapping
        if from_type_ref.is_interface && !to_type_ref.is_interface {
            // Extract the underlying value from the interface wrapper
            // Interface objects in CURSED have a vtable pointer followed by the actual value
            let interface_ptr = value as *const *const c_void;
            let actual_value = *interface_ptr.offset(1); // Skip vtable pointer
            return actual_value;
        }
        
        // Handle interface wrapping
        if !from_type_ref.is_interface && to_type_ref.is_interface {
            // Create an interface wrapper
            // This would typically allocate memory for the interface object
            // For now, we'll return the value as-is
            return value;
        }
        
        // Handle numeric conversions
        let from_name = CStr::from_ptr(from_type_ref.type_name).to_str().unwrap_or("");
        let to_name = CStr::from_ptr(to_type_ref.type_name).to_str().unwrap_or("");
        
        match (from_name, to_name) {
            ("normie", "thicc") | ("smol", "normie") | ("mid", "normie") => {
                // Integer widening - return as-is for now
                value
            }
            ("thicc", "normie") | ("normie", "smol") | ("normie", "mid") => {
                // Integer narrowing - return as-is for now
                value
            }
            ("normie", "meal") | ("snack", "meal") => {
                // Integer/float to double - return as-is for now
                value
            }
            _ => {
                // Default case - return the value as-is
                value
            }
        }
    }
}

/// Generate type information for primitive CURSED types
pub fn create_primitive_type_info(type_name: &str) -> RuntimeTypeInfo {
    let type_name_cstring = CString::new(type_name).unwrap();
    let type_name_ptr = type_name_cstring.into_raw();
    
    // Simple hash function for type ID
    let type_id = type_name.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
    
    let size = match type_name {
        "normie" => 4,  // i32
        "tea" => 8,     // string pointer
        "lit" => 1,     // bool
        "sip" => 1,     // char
        "smol" => 1,    // i8
        "mid" => 2,     // i16
        "thicc" => 8,   // i64
        "snack" => 4,   // f32
        "meal" => 8,    // f64
        _ => 8,         // default pointer size
    };
    
    RuntimeTypeInfo {
        type_name: type_name_ptr,
        type_id,
        size,
        is_interface: false,
        interface_methods: std::ptr::null(),
        interface_method_count: 0,
    }
}

/// Get type ID by name using the same hash function as create_primitive_type_info
pub fn get_type_id_by_name(type_name: &str) -> u64 {
    type_name.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64))
}

/// Initialize runtime type system with CURSED primitive types
pub fn initialize_cursed_runtime_types() {
    initialize_type_registry();
    
    let primitive_types = [
        "normie", "tea", "lit", "sip", "smol", 
        "mid", "thicc", "snack", "meal"
    ];
    
    for type_name in &primitive_types {
        let type_info = create_primitive_type_info(type_name);
        register_type(type_info);
    }
}

/// Enhanced type checking with value inspection
#[no_mangle]
pub extern "C" fn cursed_check_value_type(value: *const c_void, type_name: *const c_char) -> bool {
    if value.is_null() || type_name.is_null() {
        return false;
    }
    
    unsafe {
        let type_name_str = CStr::from_ptr(type_name).to_str().unwrap_or("");
        let type_info = cursed_get_runtime_type_info(value);
        
        if type_info.is_null() {
            return false;
        }
        
        let type_info_ref = &*type_info;
        let actual_type_str = CStr::from_ptr(type_info_ref.type_name).to_str().unwrap_or("");
        
        actual_type_str == type_name_str
    }
}
