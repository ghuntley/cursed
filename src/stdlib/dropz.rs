//! The dropz package provides basic I/O primitives.
//! This is equivalent to the io package in Go.

use std::rc::Rc;
use std::fs;
use std::io::{Read, Write, Seek, SeekFrom};
use crate::object::Object;
use crate::error::Error;

/// Read a file into a byte array
pub fn read_file(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("read_file requires 1 argument: path".to_string()));
    }
    
    let path = match &*args[0] {
        Object::String(p) => p.clone(),
        _ => return Err(Error::Runtime("Argument to read_file must be a string".to_string())),
    };
    
    match fs::read(&path) {
        Ok(bytes) => {
            let byte_objects: Vec<Object> = bytes.into_iter()
                .map(|b| Object::Integer(b as i64))
                .collect();
            Ok(Rc::new(Object::Array(byte_objects)))
        },
        Err(e) => Err(Error::Runtime(format!("Failed to read file {}: {}", path, e))),
    }
}

/// Read a file into a string
pub fn read_file_string(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("read_file_string requires 1 argument: path".to_string()));
    }
    
    let path = match &*args[0] {
        Object::String(p) => p.clone(),
        _ => return Err(Error::Runtime("Argument to read_file_string must be a string".to_string())),
    };
    
    match fs::read_to_string(&path) {
        Ok(content) => Ok(Rc::new(Object::String(content))),
        Err(e) => Err(Error::Runtime(format!("Failed to read file {}: {}", path, e))),
    }
}

/// Write a byte array to a file
pub fn write_file(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("write_file requires 2 arguments: path and data".to_string()));
    }
    
    let path = match &*args[0] {
        Object::String(p) => p.clone(),
        _ => return Err(Error::Runtime("First argument to write_file must be a string".to_string())),
    };
    
    let data = match &*args[1] {
        Object::Array(arr) => {
            let mut bytes = Vec::with_capacity(arr.len());
            for obj in arr {
                match obj {
                    Object::Integer(i) => bytes.push(*i as u8),
                    _ => return Err(Error::Runtime("Data array must contain only integers".to_string())),
                }
            }
            bytes
        },
        Object::String(s) => s.as_bytes().to_vec(),
        _ => return Err(Error::Runtime("Second argument to write_file must be an array of bytes or a string".to_string())),
    };
    
    match fs::write(&path, data) {
        Ok(_) => Ok(Rc::new(Object::Null)),
        Err(e) => Err(Error::Runtime(format!("Failed to write file {}: {}", path, e))),
    }
}

/// Copy from a reader to a writer
pub fn copy(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // Simplified implementation
    if args.len() < 2 {
        return Err(Error::Runtime("copy requires 2 arguments: reader and writer".to_string()));
    }
    
    // For now, just return the number of bytes that would be copied
    Ok(Rc::new(Object::Integer(0)))
}