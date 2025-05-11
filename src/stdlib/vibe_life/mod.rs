//! Operating system and environment functionality for CURSED
//!
//! The vibe_life package provides operating system functionality for CURSED
//! programs, similar to Go's os package. It includes functions for working with
//! environment variables, command-line arguments, the file system, and other
//! OS-related operations.

use crate::error::Error;
use crate::object::Object;
use std::env;
use std::fs;
use std::path::Path;
use std::rc::Rc;

// Re-export existing functions from the original module
mod original;
pub use original::*;

/// Gets the hostname of the machine
///
/// # Returns
///
/// The hostname as a string, or an empty string if it couldn't be determined
pub fn hostname(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    let hostname = match hostname::get() {
        Ok(name) => name.to_string_lossy().to_string(),
        Err(_) => "".to_string()
    };
    
    Ok(Rc::new(Object::String(hostname)))
}

/// Gets the path to the current executable
///
/// # Returns
///
/// The path to the executable as a string, or an empty string if it couldn't be determined
pub fn executable(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    match std::env::current_exe() {
        Ok(path) => Ok(Rc::new(Object::String(path.to_string_lossy().to_string()))),
        Err(_) => Ok(Rc::new(Object::String("".to_string())))
    }
}

/// Gets the path to a temporary directory
///
/// # Returns
///
/// The path to the temporary directory as a string
pub fn temp_dir(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    let temp_dir = std::env::temp_dir();
    Ok(Rc::new(Object::String(temp_dir.to_string_lossy().to_string())))
}

/// Gets the current working directory
///
/// # Returns
///
/// The current working directory as a string
pub fn working_dir(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    match std::env::current_dir() {
        Ok(path) => Ok(Rc::new(Object::String(path.to_string_lossy().to_string()))),
        Err(e) => Err(Error::Runtime(format!("Failed to get working directory: {}", e)))
    }
}