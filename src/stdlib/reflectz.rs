//! The reflectz package provides runtime reflection.
//! This is equivalent to the reflect package in Go.

use std::rc::Rc;
use crate::object::Object;
use crate::error::Error;

/// Get the type of an object
pub fn type_of(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("type_of requires 1 argument: object".to_string()));
    }
    
    let type_name = args[0].type_name().to_string();
    Ok(Rc::new(Object::String(type_name)))
}

/// Check if an object is a certain type
pub fn is_type(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("is_type requires 2 arguments: object and type name".to_string()));
    }
    
    let type_name = match &*args[1] {
        Object::String(name) => name,
        _ => return Err(Error::Runtime("Second argument to is_type must be a string".to_string())),
    };
    
    let result = args[0].is_type(type_name);
    Ok(Rc::new(Object::Boolean(result)))
}

/// Get the value of a field in a struct
pub fn get_field(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("get_field requires 2 arguments: struct and field name".to_string()));
    }
    
    // Simplified implementation - just return null
    Ok(Rc::new(Object::Null))
}

/// Set the value of a field in a struct
pub fn set_field(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 3 {
        return Err(Error::Runtime("set_field requires 3 arguments: struct, field name, and value".to_string()));
    }
    
    // Simplified implementation - just return null
    Ok(Rc::new(Object::Null))
}

/// Call a method on an object
pub fn call_method(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("call_method requires at least 2 arguments: object and method name".to_string()));
    }
    
    // Simplified implementation - just return null
    Ok(Rc::new(Object::Null))
}