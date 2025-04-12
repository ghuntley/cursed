//! Operating system and environment functionality for CURSED
//!
//! The vibe_life package provides operating system functionality for CURSED
//! programs, similar to Go's os package. It includes functions for working with
//! environment variables, command-line arguments, the file system, and other
//! OS-related operations.
//!
//! Key features:
//!
//! - Environment variables: `getenv`, `setenv`
//! - Command-line arguments: `args`
//! - Program control: `exit`
//! - File system information: `exists`, `getwd`

use crate::error::Error;
use crate::object::Object;
use std::env;
use std::fs;
use std::path::Path;
use std::rc::Rc;

/// Gets the command-line arguments passed to the CURSED program
///
/// This function returns all arguments including the program name (as the
/// first element). It provides access to the raw command line arguments
/// that were used to invoke the CURSED program.
///
/// # Returns
///
/// An array of strings representing the command-line arguments
pub fn args(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    let cmd_args: Vec<Object> = env::args().map(|arg| Object::String(arg)).collect();

    Ok(Rc::new(Object::Array(cmd_args)))
}

/// Get an environment variable
pub fn getenv(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "getenv requires 1 argument: key".to_string(),
        ));
    }

    let key = match &*args[0] {
        Object::String(k) => k.clone(),
        _ => {
            return Err(Error::Runtime(
                "Argument to getenv must be a string".to_string(),
            ))
        }
    };

    match env::var(key) {
        Ok(value) => Ok(Rc::new(Object::String(value))),
        Err(_) => Ok(Rc::new(Object::String("".to_string()))),
    }
}

/// Set an environment variable
pub fn setenv(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "setenv requires 2 arguments: key and value".to_string(),
        ));
    }

    let key = match &*args[0] {
        Object::String(k) => k.clone(),
        _ => {
            return Err(Error::Runtime(
                "First argument to setenv must be a string".to_string(),
            ))
        }
    };

    let value = match &*args[1] {
        Object::String(v) => v.clone(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to setenv must be a string".to_string(),
            ))
        }
    };

    env::set_var(key, value);
    Ok(Rc::new(Object::Null))
}

/// Terminates the CURSED program with the specified exit code
///
/// This function immediately exits the program with the given status code.
/// A status code of 0 typically indicates success, while non-zero values
/// indicate various error conditions.
///
/// # Arguments
///
/// * `args[0]` - Optional integer exit code (defaults to 0 if not provided)
///
/// # Note
///
/// This function never returns as it terminates the program.
pub fn exit(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    let code = if args.is_empty() {
        0
    } else {
        match &*args[0] {
            Object::Integer(code) => *code as i32,
            _ => {
                return Err(Error::Runtime(
                    "Argument to exit must be an integer".to_string(),
                ))
            }
        }
    };

    std::process::exit(code);
}

/// Check if a file or directory exists
pub fn exists(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "exists requires 1 argument: path".to_string(),
        ));
    }

    let path = match &*args[0] {
        Object::String(p) => p.clone(),
        _ => {
            return Err(Error::Runtime(
                "Argument to exists must be a string".to_string(),
            ))
        }
    };

    Ok(Rc::new(Object::Boolean(Path::new(&path).exists())))
}

/// Get the current working directory
pub fn getwd(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    match env::current_dir() {
        Ok(path) => Ok(Rc::new(Object::String(path.to_string_lossy().to_string()))),
        Err(e) => Err(Error::Runtime(format!(
            "Failed to get current directory: {}",
            e
        ))),
    }
}
