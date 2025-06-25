use crate::error::CursedError;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
// use crate::stdlib::fs::error::{FsError, FsResult};

/// Read the entire contents of a file as a string
pub fn read_file(path: &str) -> FsResult<String> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(FsError::NotFound(path.to_string_lossy().to_string()));
    }
    
    if !path.is_file() {
        return Err(FsError::NotAFile(path.to_string_lossy().to_string()));
    }
    
    fs::read_to_string(path).map_err(FsError::from)
}

/// Read the entire contents of a file as bytes
pub fn read_file_bytes(path: &str) -> FsResult<Vec<u8>> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(FsError::NotFound(path.to_string_lossy().to_string()));
    }
    
    if !path.is_file() {
        return Err(FsError::NotAFile(path.to_string_lossy().to_string()));
    }
    
    fs::read(path).map_err(FsError::from)
}

/// Write a string to a file, creating it if it doesn't exist or overwriting if it does
pub fn write_file(path: &str, content: &str) -> FsResult<()> {
    let path = Path::new(path);
    
    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(FsError::from)?;
        }
    }
    
    fs::write(path, content).map_err(FsError::from)
}

/// Write bytes to a file, creating it if it doesn't exist or overwriting if it does
pub fn write_file_bytes(path: &str, content: &[u8]) -> FsResult<()> {
    let path = Path::new(path);
    
    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(FsError::from)?;
        }
    }
    
    fs::write(path, content).map_err(FsError::from)
}

/// Append a string to a file, creating it if it doesn't exist
pub fn append_file(path: &str, content: &str) -> FsResult<()> {
    let path = Path::new(path);
    
    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(FsError::from)?;
        }
    }
    
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(FsError::from)?;
    
    file.write_all(content.as_bytes()).map_err(FsError::from)
}

/// Append bytes to a file, creating it if it doesn't exist
pub fn append_file_bytes(path: &str, content: &[u8]) -> FsResult<()> {
    let path = Path::new(path);
    
    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(FsError::from)?;
        }
    }
    
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(FsError::from)?;
    
    file.write_all(content).map_err(FsError::from)
}

/// Delete a file
pub fn delete_file(path: &str) -> FsResult<()> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(FsError::NotFound(path.to_string_lossy().to_string()));
    }
    
    if !path.is_file() {
        return Err(FsError::NotAFile(path.to_string_lossy().to_string()));
    }
    
    fs::remove_file(path).map_err(FsError::from)
}

/// Copy a file from one location to another
pub fn copy_file(from: &str, to: &str) -> FsResult<()> {
    let from_path = Path::new(from);
    let to_path = Path::new(to);
    
    if !from_path.exists() {
        return Err(FsError::NotFound(from_path.to_string_lossy().to_string()));
    }
    
    if !from_path.is_file() {
        return Err(FsError::NotAFile(from_path.to_string_lossy().to_string()));
    }
    
    // Create parent directories if they don't exist
    if let Some(parent) = to_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(FsError::from)?;
        }
    }
    
    fs::copy(from_path, to_path).map_err(FsError::from)?;
    Ok(())
}

/// Move (rename) a file from one location to another
pub fn move_file(from: &str, to: &str) -> FsResult<()> {
    let from_path = Path::new(from);
    let to_path = Path::new(to);
    
    if !from_path.exists() {
        return Err(FsError::NotFound(from_path.to_string_lossy().to_string()));
    }
    
    // Create parent directories if they don't exist
    if let Some(parent) = to_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(FsError::from)?;
        }
    }
    
    fs::rename(from_path, to_path).map_err(FsError::from)
}

/// Create a hard link to a file
pub fn create_hard_link(from: &str, to: &str) -> FsResult<()> {
    let from_path = Path::new(from);
    let to_path = Path::new(to);
    
    if !from_path.exists() {
        return Err(FsError::NotFound(from_path.to_string_lossy().to_string()));
    }
    
    if !from_path.is_file() {
        return Err(FsError::NotAFile(from_path.to_string_lossy().to_string()));
    }
    
    // Create parent directories if they don't exist
    if let Some(parent) = to_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(FsError::from)?;
        }
    }
    
    fs::hard_link(from_path, to_path).map_err(FsError::from)
}

/// Create a symbolic link to a file or directory
#[cfg(unix)]
pub fn create_symlink(from: &str, to: &str) -> FsResult<()> {
    use std::os::unix::fs::symlink;
    
    let to_path = Path::new(to);
    
    // Create parent directories if they don't exist
    if let Some(parent) = to_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(FsError::from)?;
        }
    }
    
    symlink(from, to_path).map_err(FsError::from)
}

/// Create a symbolic link to a file or directory (Windows)
#[cfg(windows)]
pub fn create_symlink(from: &str, to: &str) -> FsResult<()> {
    let from_path = Path::new(from);
    let to_path = Path::new(to);
    
    // Create parent directories if they don't exist
    if let Some(parent) = to_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(FsError::from)?;
        }
    }
    
    if from_path.is_dir() {
        std::os::windows::fs::symlink_dir(from, to_path).map_err(FsError::from)
    } else {
        std::os::windows::fs::symlink_file(from, to_path).map_err(FsError::from)
    }
}

/// Truncate a file to a specified size
pub fn truncate_file(path: &str, size: u64) -> FsResult<()> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(FsError::NotFound(path.to_string_lossy().to_string()));
    }
    
    if !path.is_file() {
        return Err(FsError::NotAFile(path.to_string_lossy().to_string()));
    }
    
    let file = fs::OpenOptions::new()
        .write(true)
        .open(path)
        .map_err(FsError::from)?;
    
    file.set_len(size).map_err(FsError::from)
}

