//! File and I/O operations for CURSED programs
//!
//! The dropz package provides file system and I/O operations for CURSED programs,
//! similar to Go's io and ioutil packages. It includes functions for reading from
//! and writing to files, checking file properties, and manipulating the file system.
//!
//! Key functions include:
//!
//! - File operations: `read_file`, `write_file`, `append_file`, `remove_file`
//! - File information: `file_exists`, `is_readable`, `is_writable`, `file_info`
//! - I/O utilities: `copy` for copying data between readers and writers

use crate::error::Error;
use crate::object::Object;
use std::fs;
use std::fs::{metadata, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::rc::Rc;

/// Reads an entire file into a byte array
///
/// This function reads the entire contents of the file at the specified path
/// and returns it as an array of byte values (integers in the range 0-255).
///
/// # Arguments
///
/// * `args[0]` - The file path as a string
///
/// # Returns
///
/// An array of integers representing the file bytes, or an error if the file
/// cannot be read
#[tracing::instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn read_file(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "read_file requires 1 argument: path".to_string(),
        ));
    }

    let path = match &*args[0] {
        Object::String(p) => p.clone(),
        _ => {
            return Err(Error::Runtime(
                "Argument to read_file must be a string".to_string(),
            ))
        }
    };

    tracing::debug!(path = %path, "Reading file");
    match fs::read(&path) {
        Ok(bytes) => {
            let byte_objects: Vec<Object> = bytes
                .into_iter()
                .map(|b| Object::Integer(b as i64))
                .collect();
            Ok(Rc::new(Object::Array(byte_objects)))
        }
        Err(e) => Err(Error::Runtime(format!(
            "Failed to read file {}: {}",
            path, e
        ))),
    }
}

/// Read a file into a string
pub fn read_file_string(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "read_file_string requires 1 argument: path".to_string(),
        ));
    }

    let path = match &*args[0] {
        Object::String(p) => p.clone(),
        _ => {
            return Err(Error::Runtime(
                "Argument to read_file_string must be a string".to_string(),
            ))
        }
    };

    match fs::read_to_string(&path) {
        Ok(content) => Ok(Rc::new(Object::String(content))),
        Err(e) => Err(Error::Runtime(format!(
            "Failed to read file {}: {}",
            path, e
        ))),
    }
}

/// Write a byte array to a file
#[tracing::instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn write_file(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "write_file requires 2 arguments: path and data".to_string(),
        ));
    }

    let path = match &*args[0] {
        Object::String(p) => p.clone(),
        _ => {
            return Err(Error::Runtime(
                "First argument to write_file must be a string".to_string(),
            ))
        }
    };

    let data = match &*args[1] {
        Object::Array(arr) => {
            let mut bytes = Vec::with_capacity(arr.len());
            for obj in arr {
                match obj {
                    Object::Integer(i) => bytes.push(*i as u8),
                    _ => {
                        return Err(Error::Runtime(
                            "Data array must contain only integers".to_string(),
                        ))
                    }
                }
            }
            bytes
        }
        Object::String(s) => s.as_bytes().to_vec(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to write_file must be an array of bytes or a string".to_string(),
            ))
        }
    };

    match fs::write(&path, data) {
        Ok(_) => Ok(Rc::new(Object::Null)),
        Err(e) => Err(Error::Runtime(format!(
            "Failed to write file {}: {}",
            path, e
        ))),
    }
}

/// Copy from a reader to a writer
pub fn copy(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // Simplified implementation
    if args.len() < 2 {
        return Err(Error::Runtime(
            "copy requires 2 arguments: reader and writer".to_string(),
        ));
    }

    // For now, just return the number of bytes that would be copied
    Ok(Rc::new(Object::Integer(0)))
}

/// Check if a file exists
pub fn file_exists(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "file_exists requires 1 argument: path".to_string(),
        ));
    }

    let path = match &*args[0] {
        Object::String(p) => p.clone(),
        _ => {
            return Err(Error::Runtime(
                "Argument to file_exists must be a string".to_string(),
            ))
        }
    };

    let exists = Path::new(&path).exists();
    Ok(Rc::new(Object::Boolean(exists)))
}

/// Check if a file is readable
pub fn is_readable(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "is_readable requires 1 argument: path".to_string(),
        ));
    }

    let path = match &*args[0] {
        Object::String(p) => p.clone(),
        _ => {
            return Err(Error::Runtime(
                "Argument to is_readable must be a string".to_string(),
            ))
        }
    };

    let path_obj = Path::new(&path);

    // Check if file exists first
    if !path_obj.exists() {
        return Ok(Rc::new(Object::Boolean(false)));
    }

    // Try to open the file for reading
    match File::open(path_obj) {
        Ok(_) => Ok(Rc::new(Object::Boolean(true))),
        Err(_) => Ok(Rc::new(Object::Boolean(false))),
    }
}

/// Check if a file is writable
pub fn is_writable(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "is_writable requires 1 argument: path".to_string(),
        ));
    }

    let path = match &*args[0] {
        Object::String(p) => p.clone(),
        _ => {
            return Err(Error::Runtime(
                "Argument to is_writable must be a string".to_string(),
            ))
        }
    };

    let path_obj = Path::new(&path);

    // If path doesn't exist, check if parent directory is writable
    if !path_obj.exists() {
        if let Some(parent) = path_obj.parent() {
            if !parent.exists() {
                return Ok(Rc::new(Object::Boolean(false)));
            }

            // Try to open a temporary file in the parent directory
            let temp_path = parent.join("tmp_write_test");
            let result = OpenOptions::new().write(true).create(true).open(&temp_path);
            if let Ok(_) = result {
                // Clean up the temporary file
                let _ = fs::remove_file(&temp_path);
                return Ok(Rc::new(Object::Boolean(true)));
            }
            return Ok(Rc::new(Object::Boolean(false)));
        }
        return Ok(Rc::new(Object::Boolean(false)));
    }

    // If path exists, try to open it for writing
    match OpenOptions::new().write(true).open(path_obj) {
        Ok(_) => Ok(Rc::new(Object::Boolean(true))),
        Err(_) => Ok(Rc::new(Object::Boolean(false))),
    }
}

/// Gets detailed information about a file
///
/// This function retrieves metadata about the file at the specified path,
/// including size, type (directory or file), and timestamps (creation,
/// modification, access).
///
/// # Arguments
///
/// * `args[0]` - The file path as a string
///
/// # Returns
///
/// A hash table containing file metadata with keys:
/// - "size": file size in bytes
/// - "is_dir": whether the path is a directory
/// - "is_file": whether the path is a regular file
/// - "modified": modification timestamp (if available)
/// - "created": creation timestamp (if available)
/// - "accessed": last access timestamp (if available)
pub fn file_info(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "file_info requires 1 argument: path".to_string(),
        ));
    }

    let path = match &*args[0] {
        Object::String(p) => p.clone(),
        _ => {
            return Err(Error::Runtime(
                "Argument to file_info must be a string".to_string(),
            ))
        }
    };

    match metadata(&path) {
        Ok(meta) => {
            // Create a hash table to store file metadata
            let mut info = std::collections::HashMap::new();

            // Add file information to the hash table
            info.insert("size".to_string(), Object::Integer(meta.len() as i64));
            info.insert("is_dir".to_string(), Object::Boolean(meta.is_dir()));
            info.insert("is_file".to_string(), Object::Boolean(meta.is_file()));

            // Add modification time if available
            if let Ok(modified) = meta.modified() {
                if let Ok(since_epoch) = modified.duration_since(std::time::UNIX_EPOCH) {
                    info.insert(
                        "modified".to_string(),
                        Object::Integer(since_epoch.as_secs() as i64),
                    );
                }
            }

            // Add creation time if available
            if let Ok(created) = meta.created() {
                if let Ok(since_epoch) = created.duration_since(std::time::UNIX_EPOCH) {
                    info.insert(
                        "created".to_string(),
                        Object::Integer(since_epoch.as_secs() as i64),
                    );
                }
            }

            // Add access time if available
            if let Ok(accessed) = meta.accessed() {
                if let Ok(since_epoch) = accessed.duration_since(std::time::UNIX_EPOCH) {
                    info.insert(
                        "accessed".to_string(),
                        Object::Integer(since_epoch.as_secs() as i64),
                    );
                }
            }

            Ok(Rc::new(Object::HashTable(info)))
        }
        Err(_) => Ok(Rc::new(Object::HashTable(std::collections::HashMap::new()))),
    }
}

/// Remove a file
pub fn remove_file(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "remove_file requires 1 argument: path".to_string(),
        ));
    }

    let path = match &*args[0] {
        Object::String(p) => p.clone(),
        _ => {
            return Err(Error::Runtime(
                "Argument to remove_file must be a string".to_string(),
            ))
        }
    };

    match fs::remove_file(&path) {
        Ok(_) => Ok(Rc::new(Object::Boolean(true))),
        Err(e) => Err(Error::Runtime(format!(
            "Failed to remove file {}: {}",
            path, e
        ))),
    }
}

/// Append to a file
pub fn append_file(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "append_file requires 2 arguments: path and data".to_string(),
        ));
    }

    let path = match &*args[0] {
        Object::String(p) => p.clone(),
        _ => {
            return Err(Error::Runtime(
                "First argument to append_file must be a string".to_string(),
            ))
        }
    };

    let data = match &*args[1] {
        Object::Array(arr) => {
            let mut bytes = Vec::with_capacity(arr.len());
            for obj in arr {
                match obj {
                    Object::Integer(i) => bytes.push(*i as u8),
                    _ => {
                        return Err(Error::Runtime(
                            "Data array must contain only integers".to_string(),
                        ))
                    }
                }
            }
            bytes
        }
        Object::String(s) => s.as_bytes().to_vec(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to append_file must be an array of bytes or a string".to_string(),
            ))
        }
    };

    // Open file in append mode
    match OpenOptions::new().create(true).append(true).open(&path) {
        Ok(mut file) => match file.write_all(&data) {
            Ok(_) => Ok(Rc::new(Object::Boolean(true))),
            Err(e) => Err(Error::Runtime(format!(
                "Failed to append to file {}: {}",
                path, e
            ))),
        },
        Err(e) => Err(Error::Runtime(format!(
            "Failed to open file {} for appending: {}",
            path, e
        ))),
    }
}
