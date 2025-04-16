// Utility functions for CURSED language
use crate::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
pub fn new_string(value: &str) -> Rc<Object> {
    thread_local! {
        static STRING_CACHE: RefCell<HashMap<String, Rc<Object>>> = RefCell::new(HashMap::new());
    }

    STRING_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(cached) = cache.get(value) {
            tracing::trace!("String cache hit");
            cached.clone()
        } else {
            tracing::trace!("String cache miss, creating new string");
            let s = Rc::new(Object::String(value.to_string()));
            cache.insert(value.to_string(), s.clone());
            s
        }
    })
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
