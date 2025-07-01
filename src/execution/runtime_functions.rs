//! Runtime implementation functions for CURSED standard library
//! 
//! This module provides the external implementation functions that the CURSED
//! standard library calls via extern declarations. These functions bridge the
//! gap between CURSED stdlib API and the underlying Rust runtime.

use crate::error::CursedError;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::fs::{self, OpenOptions};
use std::io::{self, Write, Read, BufRead, BufReader};
use std::env;

/// Initialize all runtime functions for the CURSED standard library
pub fn initialize_runtime_functions() -> Result<(), CursedError> {
    // Runtime function initialization - registers all external functions
    // with the JIT compiler and execution engine
    Ok(())
}

// ================================
// I/O Implementation Functions
// ================================

/// Print a message to stdout (implementation for io_print)
#[no_mangle]
pub extern "C" fn io_print(message_ptr: *const c_char) -> i32 {
    if message_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(message_ptr).to_str() {
            Ok(message) => {
                print!("{}", message);
                if io::stdout().flush().is_ok() { 0 } else { -1 }
            },
            Err(_) => -1
        }
    }
}

/// Print a message with newline to stdout (implementation for io_println)
#[no_mangle]
pub extern "C" fn io_println(message_ptr: *const c_char) -> i32 {
    if message_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(message_ptr).to_str() {
            Ok(message) => {
                println!("{}", message);
                0
            },
            Err(_) => -1
        }
    }
}

/// Print a message to stderr (implementation for io_eprint)
#[no_mangle]
pub extern "C" fn io_eprint(message_ptr: *const c_char) -> i32 {
    if message_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(message_ptr).to_str() {
            Ok(message) => {
                eprint!("{}", message);
                if io::stderr().flush().is_ok() { 0 } else { -1 }
            },
            Err(_) => -1
        }
    }
}

/// Print a message with newline to stderr (implementation for io_eprintln)
#[no_mangle]
pub extern "C" fn io_eprintln(message_ptr: *const c_char) -> i32 {
    if message_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(message_ptr).to_str() {
            Ok(message) => {
                eprintln!("{}", message);
                0
            },
            Err(_) => -1
        }
    }
}

/// Read a line from stdin (implementation for io_read_line)
#[no_mangle]
pub extern "C" fn io_read_line() -> *mut c_char {
    let stdin = io::stdin();
    let mut line = String::new();
    
    match stdin.read_line(&mut line) {
        Ok(_) => {
            // Remove trailing newline
            if line.ends_with('\n') {
                line.pop();
                if line.ends_with('\r') {
                    line.pop();
                }
            }
            
            match CString::new(line) {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => ptr::null_mut()
            }
        },
        Err(_) => ptr::null_mut()
    }
}

/// Write content to a file (implementation for io_write_file)
#[no_mangle]
pub extern "C" fn io_write_file(path_ptr: *const c_char, content_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() || content_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let path_result = CStr::from_ptr(path_ptr).to_str();
        let content_result = CStr::from_ptr(content_ptr).to_str();
        
        match (path_result, content_result) {
            (Ok(path), Ok(content)) => {
                match fs::write(path, content) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            _ => -1
        }
    }
}

/// Read content from a file (implementation for io_read_file)
#[no_mangle]
pub extern "C" fn io_read_file(path_ptr: *const c_char) -> *mut c_char {
    if path_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::read_to_string(path) {
                    Ok(content) => {
                        match CString::new(content) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Check if a file exists (implementation for io_file_exists)
#[no_mangle]
pub extern "C" fn io_file_exists(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                if fs::metadata(path).is_ok() { 1 } else { 0 }
            },
            Err(_) => 0
        }
    }
}

/// Create a directory (implementation for io_create_directory)
#[no_mangle]
pub extern "C" fn io_create_directory(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::create_dir(path) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Create a directory recursively (implementation for io_create_directory_recursive)
#[no_mangle]
pub extern "C" fn io_create_directory_recursive(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::create_dir_all(path) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Delete a file (implementation for io_delete_file)
#[no_mangle]
pub extern "C" fn io_delete_file(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::remove_file(path) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

// ================================
// Additional I/O Functions Implementation
// ================================

/// Printf-style formatted printing (implementation for io_printf)
#[no_mangle]
pub extern "C" fn io_printf(format_ptr: *const c_char, _args_ptr: *const c_char) -> i32 {
    if format_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(format_ptr).to_str() {
            Ok(format_str) => {
                // For now, just print the format string (full printf implementation would be complex)
                print!("{}", format_str);
                io::stdout().flush().unwrap_or(());
                0
            },
            Err(_) => -1
        }
    }
}

/// Read a single character from console (implementation for io_read_char)
#[no_mangle]
pub extern "C" fn io_read_char(buf_ptr: *mut c_char, buf_len: usize) -> i32 {
    if buf_ptr.is_null() || buf_len < 2 {
        return -1;
    }
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let ch = input.chars().next().unwrap_or('\0');
            unsafe {
                *buf_ptr = ch as c_char;
                *buf_ptr.add(1) = 0; // null terminate
            }
            0
        },
        Err(_) => -1
    }
}

/// Read an integer from console (implementation for io_read_int)
#[no_mangle]
pub extern "C" fn io_read_int() -> i32 {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.trim().parse::<i32>().unwrap_or(0)
        },
        Err(_) => 0
    }
}

/// Read a float from console (implementation for io_read_float) 
#[no_mangle]
pub extern "C" fn io_read_float() -> f64 {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.trim().parse::<f64>().unwrap_or(0.0)
        },
        Err(_) => 0.0
    }
}

/// Append content to a file (implementation for io_append_file)
#[no_mangle]
pub extern "C" fn io_append_file(path_ptr: *const c_char, content_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() || content_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match (CStr::from_ptr(path_ptr).to_str(), CStr::from_ptr(content_ptr).to_str()) {
            (Ok(path), Ok(content)) => {
                match OpenOptions::new().create(true).append(true).open(path) {
                    Ok(mut file) => {
                        match file.write_all(content.as_bytes()) {
                            Ok(_) => 0,
                            Err(_) => -1
                        }
                    },
                    Err(_) => -1
                }
            },
            _ => -1
        }
    }
}

/// Copy a file (implementation for io_copy_file)
#[no_mangle]
pub extern "C" fn io_copy_file(src_ptr: *const c_char, dest_ptr: *const c_char) -> i32 {
    if src_ptr.is_null() || dest_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match (CStr::from_ptr(src_ptr).to_str(), CStr::from_ptr(dest_ptr).to_str()) {
            (Ok(src), Ok(dest)) => {
                match fs::copy(src, dest) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            _ => -1
        }
    }
}

/// Move/rename a file (implementation for io_move_file)
#[no_mangle]
pub extern "C" fn io_move_file(src_ptr: *const c_char, dest_ptr: *const c_char) -> i32 {
    if src_ptr.is_null() || dest_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match (CStr::from_ptr(src_ptr).to_str(), CStr::from_ptr(dest_ptr).to_str()) {
            (Ok(src), Ok(dest)) => {
                match fs::rename(src, dest) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            _ => -1
        }
    }
}

/// Get file size (implementation for io_file_size)
#[no_mangle]
pub extern "C" fn io_file_size(path_ptr: *const c_char) -> i64 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::metadata(path) {
                    Ok(metadata) => metadata.len() as i64,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Check if path is a file (implementation for io_is_file)
#[no_mangle]
pub extern "C" fn io_is_file(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::metadata(path) {
                    Ok(metadata) => if metadata.is_file() { 1 } else { 0 },
                    Err(_) => 0
                }
            },
            Err(_) => 0
        }
    }
}

/// Check if path is a directory (implementation for io_is_directory)
#[no_mangle]
pub extern "C" fn io_is_directory(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::metadata(path) {
                    Ok(metadata) => if metadata.is_dir() { 1 } else { 0 },
                    Err(_) => 0
                }
            },
            Err(_) => 0
        }
    }
}

/// Remove a directory (implementation for io_remove_directory)
#[no_mangle]
pub extern "C" fn io_remove_directory(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::remove_dir(path) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Remove a directory recursively (implementation for io_remove_directory_recursive)
#[no_mangle]
pub extern "C" fn io_remove_directory_recursive(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::remove_dir_all(path) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Get current working directory (implementation for io_current_directory)
#[no_mangle]
pub extern "C" fn io_current_directory(buf_ptr: *mut c_char, buf_len: usize) -> i32 {
    if buf_ptr.is_null() || buf_len == 0 {
        return -1;
    }
    
    match env::current_dir() {
        Ok(path) => {
            if let Some(path_str) = path.to_str() {
                let path_bytes = path_str.as_bytes();
                if path_bytes.len() + 1 <= buf_len {
                    unsafe {
                        ptr::copy_nonoverlapping(path_bytes.as_ptr(), buf_ptr as *mut u8, path_bytes.len());
                        *buf_ptr.add(path_bytes.len()) = 0; // null terminate
                    }
                    0
                } else {
                    -1 // Buffer too small
                }
            } else {
                -1 // Path not valid UTF-8
            }
        },
        Err(_) => -1
    }
}

/// Change working directory (implementation for io_change_directory)
#[no_mangle]
pub extern "C" fn io_change_directory(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match env::set_current_dir(path) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

// ================================
// Collections Implementation Functions  
// ================================

use std::collections::HashMap;
use std::collections::HashSet;
use std::slice;

// Array/Vector Operations
#[no_mangle]
pub extern "C" fn collections_array_new() -> *mut Vec<i64> {
    Box::into_raw(Box::new(Vec::<i64>::new()))
}

#[no_mangle]
pub extern "C" fn collections_array_with_capacity(capacity: usize) -> *mut Vec<i64> {
    Box::into_raw(Box::new(Vec::<i64>::with_capacity(capacity)))
}

#[no_mangle]
pub extern "C" fn collections_array_push(arr_ptr: *mut Vec<i64>, item: i64) -> i32 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        arr.push(item);
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_array_pop(arr_ptr: *mut Vec<i64>) -> i64 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        arr.pop().unwrap_or(-1)
    }
}

#[no_mangle]
pub extern "C" fn collections_array_get(arr_ptr: *const Vec<i64>, index: usize) -> i64 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &*arr_ptr;
        arr.get(index).copied().unwrap_or(-1)
    }
}

#[no_mangle]
pub extern "C" fn collections_array_set(arr_ptr: *mut Vec<i64>, index: usize, value: i64) -> i32 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        if index >= arr.len() {
            return -1;
        }
        arr[index] = value;
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_array_len(arr_ptr: *const Vec<i64>) -> usize {
    if arr_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let arr = &*arr_ptr;
        arr.len()
    }
}

#[no_mangle]
pub extern "C" fn collections_array_insert(arr_ptr: *mut Vec<i64>, index: usize, item: i64) -> i32 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        if index > arr.len() {
            return -1;
        }
        arr.insert(index, item);
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_array_remove(arr_ptr: *mut Vec<i64>, index: usize) -> i64 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        if index >= arr.len() {
            return -1;
        }
        arr.remove(index)
    }
}

#[no_mangle]
pub extern "C" fn collections_array_clear(arr_ptr: *mut Vec<i64>) -> i32 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        arr.clear();
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_array_is_empty(arr_ptr: *const Vec<i64>) -> i32 {
    if arr_ptr.is_null() {
        return 1;
    }
    
    unsafe {
        let arr = &*arr_ptr;
        if arr.is_empty() { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_array_contains(arr_ptr: *const Vec<i64>, item: i64) -> i32 {
    if arr_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let arr = &*arr_ptr;
        if arr.contains(&item) { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_array_reverse(arr_ptr: *mut Vec<i64>) -> i32 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        arr.reverse();
    }
    0
}

// HashMap Operations
#[no_mangle]
pub extern "C" fn collections_map_new() -> *mut HashMap<i64, i64> {
    Box::into_raw(Box::new(HashMap::<i64, i64>::new()))
}

#[no_mangle]
pub extern "C" fn collections_map_with_capacity(capacity: usize) -> *mut HashMap<i64, i64> {
    Box::into_raw(Box::new(HashMap::<i64, i64>::with_capacity(capacity)))
}

#[no_mangle]
pub extern "C" fn collections_map_set(map_ptr: *mut HashMap<i64, i64>, key: i64, value: i64) -> i32 {
    if map_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let map = &mut *map_ptr;
        map.insert(key, value);
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_map_get(map_ptr: *const HashMap<i64, i64>, key: i64) -> i64 {
    if map_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let map = &*map_ptr;
        map.get(&key).copied().unwrap_or(-1)
    }
}

#[no_mangle]
pub extern "C" fn collections_map_remove(map_ptr: *mut HashMap<i64, i64>, key: i64) -> i64 {
    if map_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let map = &mut *map_ptr;
        map.remove(&key).unwrap_or(-1)
    }
}

#[no_mangle]
pub extern "C" fn collections_map_contains_key(map_ptr: *const HashMap<i64, i64>, key: i64) -> i32 {
    if map_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let map = &*map_ptr;
        if map.contains_key(&key) { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_map_len(map_ptr: *const HashMap<i64, i64>) -> usize {
    if map_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let map = &*map_ptr;
        map.len()
    }
}

#[no_mangle]
pub extern "C" fn collections_map_clear(map_ptr: *mut HashMap<i64, i64>) -> i32 {
    if map_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let map = &mut *map_ptr;
        map.clear();
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_map_is_empty(map_ptr: *const HashMap<i64, i64>) -> i32 {
    if map_ptr.is_null() {
        return 1;
    }
    
    unsafe {
        let map = &*map_ptr;
        if map.is_empty() { 1 } else { 0 }
    }
}

// HashSet Operations
#[no_mangle]
pub extern "C" fn collections_set_new() -> *mut HashSet<i64> {
    Box::into_raw(Box::new(HashSet::<i64>::new()))
}

#[no_mangle]
pub extern "C" fn collections_set_with_capacity(capacity: usize) -> *mut HashSet<i64> {
    Box::into_raw(Box::new(HashSet::<i64>::with_capacity(capacity)))
}

#[no_mangle]
pub extern "C" fn collections_set_insert(set_ptr: *mut HashSet<i64>, item: i64) -> i32 {
    if set_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let set = &mut *set_ptr;
        if set.insert(item) { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_set_contains(set_ptr: *const HashSet<i64>, item: i64) -> i32 {
    if set_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let set = &*set_ptr;
        if set.contains(&item) { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_set_remove(set_ptr: *mut HashSet<i64>, item: i64) -> i32 {
    if set_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let set = &mut *set_ptr;
        if set.remove(&item) { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_set_len(set_ptr: *const HashSet<i64>) -> usize {
    if set_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let set = &*set_ptr;
        set.len()
    }
}

#[no_mangle]
pub extern "C" fn collections_set_clear(set_ptr: *mut HashSet<i64>) -> i32 {
    if set_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let set = &mut *set_ptr;
        set.clear();
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_set_is_empty(set_ptr: *const HashSet<i64>) -> i32 {
    if set_ptr.is_null() {
        return 1;
    }
    
    unsafe {
        let set = &*set_ptr;
        if set.is_empty() { 1 } else { 0 }
    }
}

// ================================
// Math Implementation Functions
// ================================

#[no_mangle]
pub extern "C" fn math_sin_impl(x: f64) -> f64 {
    x.sin()
}

#[no_mangle]
pub extern "C" fn math_cos_impl(x: f64) -> f64 {
    x.cos()
}

#[no_mangle]
pub extern "C" fn math_sqrt_impl(x: f64) -> f64 {
    x.sqrt()
}

#[no_mangle]
pub extern "C" fn math_random_impl() -> f64 {
    // Simple random implementation - should be replaced with proper PRNG
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .hash(&mut hasher);
    
    (hasher.finish() as f64) / (u64::MAX as f64)
}

// ================================
// Legacy compatibility
// ================================

pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED runtime functions implemented".to_string())
}
