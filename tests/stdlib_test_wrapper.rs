use cursed::object::Object;
use cursed::error::Error;
use std::sync::Arc;

// Test wrappers for stdlib functions
// 
// This file provides wrapper functions for the standard library tests
// to convert between raw string/number inputs and the Object-based API.


/// Wrapper for vibez::spill
pub fn spill(message: &str) -> Result<(), Error> {
    let args = vec![Arc::new(Object::String(message.to_string())];
    cursed::stdlib::vibez::spill(&args).map(|_| ())
}

/// Wrapper for stringz::contains
pub fn contains(s: &str, substr: &str) -> bool {
    let args = vec![
        Arc::new(Object::String(s.to_string(),
        Arc::new(Object::String(substr.to_string(),
    ];
    match cursed::stdlib::stringz::contains(&args) {
        Ok(obj) => {
            if let Object::Boolean(b) = &*obj {
                *b
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

/// Wrapper for stringz::len
pub fn len(s: &str) -> i64 {
    let args = vec![Arc::new(Object::String(s.to_string())];
    match cursed::stdlib::stringz::len(&args) {
        Ok(obj) => {
            if let Object::Integer(i) = &*obj {
                *i
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

/// Wrapper for stringz::to_upper
pub fn to_upper(s: &str) -> String {
    let args = vec![Arc::new(Object::String(s.to_string())];
    match cursed::stdlib::stringz::to_upper(&args) {
        Ok(obj) => {
            if let Object::String(s) = &*obj {
                s.clone()
            } else {
                String::new()
            }
        }
        Err(_) => String::new(),
    }
}

/// Wrapper for stringz::to_lower
pub fn to_lower(s: &str) -> String {
    let args = vec![Arc::new(Object::String(s.to_string())];
    match cursed::stdlib::stringz::to_lower(&args) {
        Ok(obj) => {
            if let Object::String(s) = &*obj {
                s.clone()
            } else {
                String::new()
            }
        }
        Err(_) => String::new(),
    }
}

/// Wrapper for htmlrizzler::escape_html
pub fn escape_html(html: &str) -> String {
    let args = vec![Arc::new(Object::String(html.to_string())];
    match cursed::stdlib::htmlrizzler::escape_html(&args) {
        Ok(obj) => {
            if let Object::String(s) = &*obj {
                s.clone()
            } else {
                String::new()
            }
        }
        Err(_) => String::new(),
    }
}

/// Wrapper for htmlrizzler::escape_js
pub fn escape_js(js: &str) -> String {
    let args = vec![Arc::new(Object::String(js.to_string())];
    match cursed::stdlib::htmlrizzler::escape_js(&args) {
        Ok(obj) => {
            if let Object::String(s) = &*obj {
                s.clone()
            } else {
                String::new()
            }
        }
        Err(_) => String::new(),
    }
}

/// Wrapper for mathz::abs
pub fn abs(value: i64) -> i64 {
    let args = vec![Arc::new(Object::Integer(value))];
    match cursed::stdlib::mathz::abs(&args) {
        Ok(obj) => {
            if let Object::Integer(i) = &*obj {
                *i
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

/// Wrapper for mathz::max
pub fn max(a: i64, b: i64) -> i64 {
    let args = vec![
        Arc::new(Object::Integer(a)),
        Arc::new(Object::Integer(b)),
    ];
    match cursed::stdlib::mathz::max(&args) {
        Ok(obj) => {
            if let Object::Integer(i) = &*obj {
                *i
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

/// Wrapper for mathz::min
pub fn min(a: i64, b: i64) -> i64 {
    let args = vec![
        Arc::new(Object::Integer(a)),
        Arc::new(Object::Integer(b)),
    ];
    match cursed::stdlib::mathz::min(&args) {
        Ok(obj) => {
            if let Object::Integer(i) = &*obj {
                *i
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

/// Wrapper for mathz::sqrt
pub fn sqrt(value: f64) -> f64 {
    let args = vec![Arc::new(Object::Float(value))];
    match cursed::stdlib::mathz::sqrt(&args) {
        Ok(obj) => {
            match &*obj {
                Object::Float(f) => *f,
                Object::Integer(i) => *i as f64,
                _ => 0.0,
            }
        }
        Err(_) => 0.0,
    }
}

/// Wrapper for mathz::sin
pub fn sin(value: f64) -> f64 {
    let args = vec![Arc::new(Object::Float(value))];
    match cursed::stdlib::mathz::sin(&args) {
        Ok(obj) => {
            if let Object::Float(f) = &*obj {
                *f
            } else {
                0.0
            }
        }
        Err(_) => 0.0,
    }
}

/// Wrapper for mathz::cos
pub fn cos(value: f64) -> f64 {
    let args = vec![Arc::new(Object::Float(value))];
    match cursed::stdlib::mathz::cos(&args) {
        Ok(obj) => {
            if let Object::Float(f) = &*obj {
                *f
            } else {
                0.0
            }
        }
        Err(_) => 0.0,
    }
}

/// Wrapper for timez::now
pub fn now() -> i64 {
    let args: Vec<Rc<Object>> = vec![];
    match cursed::stdlib::timez::now(&args) {
        Ok(obj) => {
            if let Object::Integer(i) = &*obj {
                *i
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

/// Wrapper for timez::format_time
pub fn format_time(timestamp: i64, format: &str) -> String {
    let args = vec![
        Arc::new(Object::Integer(timestamp)),
        Arc::new(Object::String(format.to_string(),
    ];
    match cursed::stdlib::timez::format_time(&args) {
        Ok(obj) => {
            if let Object::String(s) = &*obj {
                s.clone()
            } else {
                String::new()
            }
        }
        Err(_) => String::new(),
    }
}