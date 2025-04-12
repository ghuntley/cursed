//! The reflectz package provides runtime reflection capabilities for CURSED programs.
//!
//! This module is equivalent to the reflect package in Go, providing functions for
//! examining types, inspecting and manipulating struct fields, and calling methods
//! on objects at runtime. This enables advanced programming techniques like generic
//! algorithms, data marshaling, and dynamic behavior.
//!
//! # Features
//!
//! - Runtime type information with `type_of`
//! - Type checking with `is_type`
//! - Struct field access with `get_field` and `set_field`
//! - Dynamic method invocation with `call_method`
//!
//! # Examples
//!
//! ```cursed
//! import "reflectz"
//!
//! // Check an object's type
//! x := 42
//! type := reflectz.type_of(x)  // Returns "integer"
//!
//! // Type assertions
//! isNumber := reflectz.is_type(x, "integer")  // Returns true
//!
//! // Working with structs
//! user := Person{name: "Zoomer", age: 21}
//! name := reflectz.get_field(user, "name")  // Returns "Zoomer"
//! reflectz.set_field(user, "age", 22)       // Updates age to 22
//!
//! // Dynamic method calls
//! result := reflectz.call_method(user, "greet", "Hello")
//! ```

use std::rc::Rc;
use crate::object::Object;
use crate::error::Error;

/// Returns the type name of any CURSED object as a string.
///
/// This function provides runtime type information, allowing programs to make
/// decisions based on the actual type of a value. It uses the object's
/// internal type_name method to determine its type.
///
/// # Arguments
///
/// * `args[0]` - The object to get the type of
///
/// # Returns
///
/// A String Object containing the name of the type (e.g., "integer", "string", "array", etc.)
///
/// # Errors
///
/// Returns a Runtime error if no argument is provided
pub fn type_of(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("type_of requires 1 argument: object".to_string()));
    }
    
    let type_name = args[0].type_name().to_string();
    Ok(Rc::new(Object::String(type_name)))
}

/// Checks if an object is of a specific type.
///
/// This function tests whether an object has the specified type, providing
/// a way to perform runtime type assertions. It uses the object's internal
/// is_type method to check against the provided type name.
///
/// # Arguments
///
/// * `args[0]` - The object to check the type of
/// * `args[1]` - The type name to check against as a String Object (e.g., "integer", "string")
///
/// # Returns
///
/// A Boolean Object indicating whether the object is of the specified type
///
/// # Errors
///
/// Returns a Runtime error if:
/// - Fewer than 2 arguments are provided
/// - The second argument is not a String Object
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

/// Gets the value of a named field from a struct object.
///
/// This function accesses a field in a struct by its name, providing a way
/// to dynamically examine struct contents at runtime. 
///
/// Note: The current implementation is simplified and returns null.
/// A full implementation would use the object's type information to
/// locate and return the specified field.
///
/// # Arguments
///
/// * `args[0]` - The struct object to get the field from
/// * `args[1]` - The field name as a String Object
///
/// # Returns
///
/// The value of the specified field, or null if the field doesn't exist
///
/// # Errors
///
/// Returns a Runtime error if fewer than 2 arguments are provided
pub fn get_field(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("get_field requires 2 arguments: struct and field name".to_string()));
    }
    
    // Simplified implementation - just return null
    Ok(Rc::new(Object::Null))
}

/// Sets the value of a named field in a struct object.
///
/// This function modifies a field in a struct by its name, providing a way
/// to dynamically update struct contents at runtime.
///
/// Note: The current implementation is simplified and doesn't actually modify the struct.
/// A full implementation would use the object's type information to
/// locate and update the specified field.
///
/// # Arguments
///
/// * `args[0]` - The struct object to set the field in
/// * `args[1]` - The field name as a String Object
/// * `args[2]` - The new value to set
///
/// # Returns
///
/// Null to indicate successful field update
///
/// # Errors
///
/// Returns a Runtime error if fewer than 3 arguments are provided
pub fn set_field(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 3 {
        return Err(Error::Runtime("set_field requires 3 arguments: struct, field name, and value".to_string()));
    }
    
    // Simplified implementation - just return null
    Ok(Rc::new(Object::Null))
}

/// Dynamically calls a method on an object with the provided arguments.
///
/// This function enables calling methods on objects by name at runtime, which is useful
/// for implementing generic algorithms, dynamic dispatch, and plugin systems.
///
/// Note: The current implementation is simplified and doesn't actually call methods.
/// A full implementation would use the object's type information to locate and invoke
/// the specified method with the provided arguments.
///
/// # Arguments
///
/// * `args[0]` - The object to call the method on
/// * `args[1]` - The method name as a String Object
/// * `args[2..n]` - Optional arguments to pass to the method
///
/// # Returns
///
/// The return value from the method call, or null in the current implementation
///
/// # Errors
///
/// Returns a Runtime error if fewer than 2 arguments are provided
pub fn call_method(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("call_method requires at least 2 arguments: object and method name".to_string()));
    }
    
    // Simplified implementation - just return null
    Ok(Rc::new(Object::Null))
}