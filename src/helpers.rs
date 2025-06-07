// Utility functions for CURSED language
use crate::object::Object;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, OnceLock};

/// Check if a value is truthy
#[tracing::instrument(skip(obj), fields(obj_type = ?obj.type_name()), level = "trace")]
pub fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Boolean(b) => *b,
        Object::Null => false,
        Object::Integer(i) => *i != 0,
        _ => true,
    }
}

/// Check if two objects are equal
#[tracing::instrument(skip(left, right), fields(left_type = ?left.type_name(), right_type = ?right.type_name()), level = "trace")]
pub fn objects_equal(left: &Object, right: &Object) -> bool {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => l == r,
        (Object::Float(l), Object::Float(r)) => l == r,
        (Object::Boolean(l), Object::Boolean(r)) => l == r,
        (Object::String(l), Object::String(r)) => l == r,
        _ => std::ptr::eq(left, right),
    }
}

/// Create a cached string
#[tracing::instrument(skip(value), fields(value_len = value.len()), level = "trace")]
pub fn new_string(value: &str) -> Arc<Object> {
    static STRING_CACHE: OnceLock<RwLock<HashMap<String, Arc<Object>>>> = OnceLock::new();
    
    let cache = STRING_CACHE.get_or_init(|| RwLock::new(HashMap::new()));

    // First try to read with shared lock
    if let Ok(read_cache) = cache.read() {
        if let Some(cached) = read_cache.get(value) {
            tracing::trace!("String cache hit");
            return cached.clone();
        }
    }

    // If not found, acquire write lock to insert
    if let Ok(mut write_cache) = cache.write() {
        // Double-check in case another thread inserted while we were waiting
        if let Some(cached) = write_cache.get(value) {
            tracing::trace!("String cache hit after write lock");
            cached.clone()
        } else {
            tracing::trace!("String cache miss, creating new string");
            let s = Arc::new(Object::String(value.to_string()));
            write_cache.insert(value.to_string(), s.clone());
            s
        }
    } else {
        // Fallback if lock is poisoned
        tracing::warn!("String cache lock poisoned, creating uncached string");
        Arc::new(Object::String(value.to_string()))
    }
}

/// Format an object for display
pub fn format_object(obj: &Object) -> String {
    match obj {
        Object::Integer(i) => i.to_string(),
        Object::Float(f) => f.to_string(),
        Object::Boolean(b) => b.to_string(),
        Object::String(s) => s.clone(),
        Object::Null => "null".to_string(),
        Object::Array(arr) => {
            let elements: Vec<String> = arr.iter().map(|obj| format_object(obj)).collect();
            format!("[{}]", elements.join(", "))
        }
        _ => format!("{:?}", obj),
    }
}
