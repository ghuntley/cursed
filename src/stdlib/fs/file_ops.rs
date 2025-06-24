use crate::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use crate::stdlib::fs::error::{FsError, FsResult};

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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_write_and_read_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let file_path_str = file_path.to_string_lossy().to_string();
        
        let content = "Hello, World!";
        
        // Write file
        write_file(&file_path_str, content).unwrap();
        
        // Read file
        let read_content = read_file(&file_path_str).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_append_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let file_path_str = file_path.to_string_lossy().to_string();
        
        // Write initial content
        write_file(&file_path_str, "Hello").unwrap();
        
        // Append content
        append_file(&file_path_str, ", World!").unwrap();
        
        // Read and verify
        let content = read_file(&file_path_str).unwrap();
        assert_eq!(content, "Hello, World!");
    }

    #[test]
    fn test_copy_file() {
        let temp_dir = TempDir::new().unwrap();
        let src_path = temp_dir.path().join("source.txt");
        let dst_path = temp_dir.path().join("dest.txt");
        
        let src_str = src_path.to_string_lossy().to_string();
        let dst_str = dst_path.to_string_lossy().to_string();
        
        let content = "Test content";
        
        // Create source file
        write_file(&src_str, content).unwrap();
        
        // Copy file
        copy_file(&src_str, &dst_str).unwrap();
        
        // Verify destination
        let read_content = read_file(&dst_str).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_move_file() {
        let temp_dir = TempDir::new().unwrap();
        let src_path = temp_dir.path().join("source.txt");
        let dst_path = temp_dir.path().join("dest.txt");
        
        let src_str = src_path.to_string_lossy().to_string();
        let dst_str = dst_path.to_string_lossy().to_string();
        
        let content = "Test content";
        
        // Create source file
        write_file(&src_str, content).unwrap();
        
        // Move file
        move_file(&src_str, &dst_str).unwrap();
        
        // Verify source is gone and destination exists
        assert!(!Path::new(&src_str).exists());
        let read_content = read_file(&dst_str).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_delete_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let file_path_str = file_path.to_string_lossy().to_string();
        
        // Create file
        write_file(&file_path_str, "content").unwrap();
        assert!(Path::new(&file_path_str).exists());
        
        // Delete file
        delete_file(&file_path_str).unwrap();
        assert!(!Path::new(&file_path_str).exists());
    }

    #[test]
    fn test_truncate_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let file_path_str = file_path.to_string_lossy().to_string();
        
        // Create file with content
        write_file(&file_path_str, "Hello, World!").unwrap();
        
        // Truncate to 5 bytes
        truncate_file(&file_path_str, 5).unwrap();
        
        // Verify truncation
        let content = read_file(&file_path_str).unwrap();
        assert_eq!(content, "Hello");
    }
}
