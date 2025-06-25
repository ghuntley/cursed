use crate::error::CursedError;
/// File system operations module for CURSED standard library
/// 
/// This module provides comprehensive file and directory operations including:
/// - Reading and writing files
/// - Directory creation and traversal
/// - File metadata and information
/// - Path manipulation utilities
/// - Robust error handling
/// 
/// # Examples
/// 
/// ## Basic File Operations
/// 
/// ```rust
/// use crate::stdlib::fs;
/// 
/// // Read a file
/// let content = fs::read_file("example.txt")?;
/// 
/// // Write a file
/// fs::write_file("output.txt", "Hello, World!")?;
/// 
/// // Append to a file
/// fs::append_file("log.txt", "New log entry\n")?;
/// ```
/// 
/// ## Directory Operations
/// 
/// ```rust
/// use crate::stdlib::fs;
/// 
/// // Create a directory
/// fs::create_dir("new_folder")?;
/// 
/// // Create nested directories
/// fs::create_dir_all("path/to/nested/folder")?;
/// 
/// // List directory contents
/// let entries = fs::list_dir(".")?;
/// for entry in entries {
///     println!("{}: {} bytes", entry.name, entry.size);
/// }
/// ```
/// 
/// ## Path Utilities
/// 
/// ```rust
/// use crate::stdlib::fs;
/// 
/// // Join path components
/// let path = fs::join_path(vec!["home".to_string(), "user".to_string(), "file.txt".to_string()]);
/// 
/// // Get parent directory
/// if let Some(parent) = fs::parent_dir("/home/user/file.txt") {
///     println!("Parent: {}", parent);
/// }
/// 
/// // Get file extension
/// if let Some(ext) = fs::extension("document.pdf") {
///     println!("Extension: {}", ext);
/// }
/// ```

pub mod error;
pub mod metadata;
pub mod file_ops;
pub mod dir_ops;
pub mod path_utils;
pub mod watcher;

// Re-export error types
pub use error::{FsError, FsResult};

// Re-export metadata types
pub use metadata::{FileMetadata, DirEntry, metadata, exists, is_file, is_dir, file_size};

// Re-export file operations
pub use file_ops::{
    read_file, read_file_bytes, write_file, write_file_bytes,
    append_file, append_file_bytes, delete_file, copy_file, move_file,
    create_hard_link, create_symlink, truncate_file
};

// Re-export directory operations
pub use dir_ops::{
    create_dir, create_dir_all, remove_dir, remove_dir_all,
    list_dir, list_files, list_dirs, copy_dir_all, walk_dir,
    find_files, dir_size, count_entries
};

// Re-export path utilities
pub use path_utils::{
    join_path, parent_dir, file_name, extension, file_stem,
    absolute_path, is_absolute, is_relative, current_dir,
    normalize_path, split_path, is_ancestor, relative_path
};

// Re-export file watcher functionality
pub use watcher::{
    FileWatcher, WatchEvent, WatcherConfig, watch_path, watch_path_with_config,
    watch_paths, wait_for_changes
};

/// File system module version
pub const VERSION: &str = "1.0.0";

/// Maximum file size for safe operations (100MB)
pub const MAX_SAFE_FILE_SIZE: u64 = 100 * 1024 * 1024;

/// Default buffer size for file operations
pub const DEFAULT_BUFFER_SIZE: usize = 8192;

/// Check if a file size is safe for memory operations
pub fn is_safe_file_size(size: u64) -> bool {
    size <= MAX_SAFE_FILE_SIZE
}

/// Utility function to ensure a path is safe (no path traversal)
pub fn is_safe_path(path: &str) -> bool {
    // Check for null bytes and obvious path traversal patterns
    if path.contains('\0') || path.contains("..") {
        return false;
    }
    
    // Check for encoded path traversal
    if path.contains("%2e%2e") || path.contains("%2E%2E") {
        return false;
    }
    
    // Additional security checks for suspicious patterns
    if path.starts_with('/') && path.contains("/../") {
        return false;
    }
    
    true
}

/// Get file type from extension
pub fn file_type_from_extension(path: &str) -> Option<String> {
    extension(path).map(|ext| {
        match ext.to_lowercase().as_str() {
            "txt" | "md" | "rst" => "text".to_string(),
            "json" | "xml" | "yaml" | "yml" | "toml" => "data".to_string(),
            "jpg" | "jpeg" | "png" | "gif" | "bmp" => "image".to_string(),
            "mp4" | "avi" | "mkv" | "mov" => "video".to_string(),
            "mp3" | "wav" | "flac" | "ogg" => "audio".to_string(),
            "zip" | "tar" | "gz" | "bz2" | "7z" => "archive".to_string(),
            "exe" | "dll" | "so" | "dylib" => "binary".to_string(),
            "rs" | "py" | "js" | "ts" | "c" | "cpp" | "h" => "code".to_string(),
            _ => "unknown".to_string(),
        }
    })
}

/// Convenience function to read a small text file safely
pub fn read_text_file_safe(path: &str) -> FsResult<String> {
    if !is_safe_path(path) {
        return Err(FsError::InvalidPath(format!("Unsafe path: {}", path)));
    }
    
    let size = file_size(path)?;
    if !is_safe_file_size(size) {
        return Err(FsError::InvalidOperation(
            format!("File too large for safe operation: {} bytes", size)
        ));
    }
    
    read_file(path)
}

/// Convenience function to write a text file with safety checks
pub fn write_text_file_safe(path: &str, content: &str) -> FsResult<()> {
    if !is_safe_path(path) {
        return Err(FsError::InvalidPath(format!("Unsafe path: {}", path)));
    }
    
    if content.len() as u64 > MAX_SAFE_FILE_SIZE {
        return Err(FsError::InvalidOperation(
            format!("Content too large for safe operation: {} bytes", content.len())
        ));
    }
    
    write_file(path, content)
}

