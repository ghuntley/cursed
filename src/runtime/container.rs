//! Runtime support for container operations in CURSED
//! 
//! This module provides runtime functions for container iteration, including:
//! - Length determination for different container types
//! - Element access for arrays, slices, and custom containers
//! - Iterator creation and manipulation for maps
//!
//! These functions are called from the LLVM codegen for range clause iteration.

use crate::object::{Object, ObjectRef};
use tracing::{debug, info, instrument};
use std::os::raw::{c_char, c_int, c_void};

/// Get the length of a container
/// 
/// # Safety
/// 
/// This function is called from generated code and must follow FFI conventions.
/// The container pointer must point to a valid container object.
#[no_mangle]
pub unsafe extern "C" fn container_length(container: *const c_void) -> c_int {
    debug!("container_length called");
    
    if container.is_null() {
        debug!("Null container pointer");
        return 0;
    }
    
    // Cast the container to an ObjectRef and determine its type
    let obj = ObjectRef::from_raw(container as *mut Object);
    
    // Get the length based on the object type
    let length = if obj.is_array() {
        // For arrays, get the length directly
        obj.as_array().map(|arr| arr.len()).unwrap_or(0)
    } else if obj.is_string() {
        // For strings, get the character count
        obj.as_string().map(|s| s.chars().count()).unwrap_or(0)
    } else if obj.is_map() {
        // For maps, get the entry count
        obj.as_map().map(|m| m.len()).unwrap_or(0)
    } else {
        // For other container types, check for a length method
        debug!("Unknown container type");
        0
    };
    
    // Prevent the object from being freed by the Rust side
    std::mem::forget(obj);
    
    debug!("Container length: {}", length);
    length as c_int
}

/// Get an element from a container at the specified index
/// 
/// # Safety
/// 
/// This function is called from generated code and must follow FFI conventions.
/// The container pointer must point to a valid container object.
#[no_mangle]
pub unsafe extern "C" fn container_get_element(container: *const c_void, index: c_int) -> *mut c_void {
    debug!("container_get_element called with index {}", index);
    
    if container.is_null() {
        debug!("Null container pointer");
        return std::ptr::null_mut();
    }
    
    if index < 0 {
        debug!("Negative index: {}", index);
        return std::ptr::null_mut();
    }
    
    // Cast the container to an ObjectRef and determine its type
    let obj = ObjectRef::from_raw(container as *mut Object);
    
    // Get the element based on the object type
    let element = if obj.is_array() {
        // For arrays, get the element at the index
        obj.as_array()
            .and_then(|arr| {
                let idx = index as usize;
                if idx < arr.len() {
                    Some(arr[idx].clone())
                } else {
                    debug!("Array index out of bounds: {} >= {}", idx, arr.len());
                    None
                }
            })
    } else if obj.is_string() {
        // For strings, get the character at the index
        obj.as_string()
            .and_then(|s| {
                let idx = index as usize;
                s.chars().nth(idx).map(|c| {
                    // Create a new character object
                    Object::new_char(c)
                })
            })
    } else {
        // For other container types, we don't have a standard way to access elements
        debug!("Unknown container type or invalid index");
        None
    };
    
    // Prevent the original object from being freed by the Rust side
    std::mem::forget(obj);
    
    // Return the element or null if not found
    match element {
        Some(elem) => {
            debug!("Returning element at index {}", index);
            Box::into_raw(Box::new(elem)) as *mut c_void
        },
        None => {
            debug!("No element found at index {}", index);
            std::ptr::null_mut()
        }
    }
}

/// Get the length of a C-style string (null-terminated)
/// 
/// # Safety
/// 
/// This function is called from generated code and must follow FFI conventions.
/// The string pointer must point to a valid null-terminated string.
#[no_mangle]
pub unsafe extern "C" fn string_length(str_ptr: *const c_char) -> c_int {
    if str_ptr.is_null() {
        return 0;
    }
    
    // Count characters until null terminator
    let mut length = 0;
    let mut current = str_ptr;
    
    while *current != 0 {
        length += 1;
        current = current.add(1);
    }
    
    length
}

/// Create an iterator for a map container
/// 
/// # Safety
/// 
/// This function is called from generated code and must follow FFI conventions.
/// The map pointer must point to a valid map object.
#[no_mangle]
pub unsafe extern "C" fn map_iterator_create(map: *const c_void) -> *mut c_void {
    debug!("map_iterator_create called");
    
    if map.is_null() {
        debug!("Null map pointer");
        return std::ptr::null_mut();
    }
    
    // Cast the map to an ObjectRef
    let obj = ObjectRef::from_raw(map as *mut Object);
    
    // Check if it's a map
    if !obj.is_map() {
        debug!("Object is not a map");
        std::mem::forget(obj);
        return std::ptr::null_mut();
    }
    
    // Get the map and create an iterator state
    let map_obj = obj.as_map().unwrap();
    
    // Create an iterator object that contains:
    // 1. The map (to keep it alive)
    // 2. The keys as a vector
    // 3. The current index
    let keys: Vec<ObjectRef> = map_obj.keys().cloned().collect();
    let iterator = MapIterator {
        map: obj,
        keys,
        current_index: 0,
    };
    
    // Return a pointer to the iterator
    Box::into_raw(Box::new(iterator)) as *mut c_void
}

/// Check if a map iterator has more elements
/// 
/// # Safety
/// 
/// This function is called from generated code and must follow FFI conventions.
/// The iterator pointer must point to a valid MapIterator object.
#[no_mangle]
pub unsafe extern "C" fn map_iterator_has_next(iterator: *const c_void) -> c_int {
    debug!("map_iterator_has_next called");
    
    if iterator.is_null() {
        debug!("Null iterator pointer");
        return 0;
    }
    
    // Cast the iterator to a MapIterator reference
    let iter = &*(iterator as *const MapIterator);
    
    // Check if there are more elements
    let has_next = iter.current_index < iter.keys.len();
    
    debug!("Iterator has next: {}", has_next);
    has_next as c_int
}

/// Advance a map iterator to the next element
/// 
/// # Safety
/// 
/// This function is called from generated code and must follow FFI conventions.
/// The iterator pointer must point to a valid MapIterator object.
#[no_mangle]
pub unsafe extern "C" fn map_iterator_next(iterator: *mut c_void) {
    debug!("map_iterator_next called");
    
    if iterator.is_null() {
        debug!("Null iterator pointer");
        return;
    }
    
    // Cast the iterator to a MapIterator reference
    let iter = &mut *(iterator as *mut MapIterator);
    
    // Advance the iterator if possible
    if iter.current_index < iter.keys.len() {
        iter.current_index += 1;
        debug!("Advanced iterator to index {}", iter.current_index);
    }
}

/// Get the current key-value pair from a map iterator
/// 
/// # Safety
/// 
/// This function is called from generated code and must follow FFI conventions.
/// The iterator pointer must point to a valid MapIterator object.
/// The key_ptr and value_ptr must be valid pointers to locations where the key and value can be stored.
#[no_mangle]
pub unsafe extern "C" fn map_iterator_get_current(
    iterator: *const c_void,
    key_ptr: *mut *mut c_void,
    value_ptr: *mut *mut c_void
) -> c_int {
    debug!("map_iterator_get_current called");
    
    if iterator.is_null() || key_ptr.is_null() || value_ptr.is_null() {
        debug!("Null pointer in map_iterator_get_current");
        return 0;
    }
    
    // Cast the iterator to a MapIterator reference
    let iter = &*(iterator as *const MapIterator);
    
    // Check if the current index is valid
    if iter.current_index >= iter.keys.len() {
        debug!("Iterator index out of bounds");
        return 0;
    }
    
    // Get the current key
    let key = iter.keys[iter.current_index].clone();
    
    // Get the corresponding value from the map
    let map_obj = iter.map.as_map().unwrap();
    let value = map_obj.get(&key).cloned().unwrap_or_else(|| {
        debug!("No value found for key");
        Object::null()
    });
    
    // Store the key and value at the provided pointers
    *key_ptr = Box::into_raw(Box::new(key)) as *mut c_void;
    *value_ptr = Box::into_raw(Box::new(value)) as *mut c_void;
    
    debug!("Retrieved key-value pair");
    1 // Success
}

/// Map iterator structure used by runtime functions
struct MapIterator {
    map: ObjectRef,        // Reference to the map (keeps it alive)
    keys: Vec<ObjectRef>,  // List of keys for iteration order
    current_index: usize,  // Current position in the keys list
}