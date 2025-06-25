use crate::error::CursedError;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
// use crate::stdlib::fs::error::{FsError, FsResult};

/// File or directory metadata
#[derive(Debug, Clone)]
pub struct FileMetadata {
    /// Size in bytes (0 for directories)
    pub size: u64,
    /// Whether this is a file
    pub is_file: bool,
    /// Whether this is a directory
    pub is_dir: bool,
    /// Whether this is a symbolic link
    pub is_symlink: bool,
    /// Whether the file is read-only
    pub readonly: bool,
    /// Created timestamp (if available)
    pub created: Option<SystemTime>,
    /// Last modified timestamp
    pub modified: Option<SystemTime>,
    /// Last accessed timestamp (if available)
    pub accessed: Option<SystemTime>,
}

impl FileMetadata {
    /// Create metadata from std::fs::Metadata
    pub fn from_std_metadata(metadata: fs::Metadata) -> Self {
        Self {
            size: metadata.len(),
            is_file: metadata.is_file(),
            is_dir: metadata.is_dir(),
            is_symlink: metadata.is_symlink(),
            readonly: metadata.permissions().readonly(),
            created: metadata.created().ok(),
            modified: metadata.modified().ok(),
            accessed: metadata.accessed().ok(),
        }
    }
}

/// Directory entry information
#[derive(Debug, Clone)]
pub struct DirEntry {
    /// Full path to the entry
    pub path: String,
    /// Entry name (filename or directory name)
    pub name: String,
    /// Whether this entry is a file
    pub is_file: bool,
    /// Whether this entry is a directory
    pub is_dir: bool,
    /// Whether this entry is a symbolic link
    pub is_symlink: bool,
    /// File size (0 for directories)
    pub size: u64,
}

impl DirEntry {
    /// Create DirEntry from std::fs::DirEntry
    pub fn from_std_entry(entry: fs::DirEntry) -> FsResult<Self> {
        let path = entry.path();
        let metadata = entry.metadata().map_err(FsError::from)?;
        
        Ok(Self {
            path: path.to_string_lossy().to_string(),
            name: entry.file_name().to_string_lossy().to_string(),
            is_file: metadata.is_file(),
            is_dir: metadata.is_dir(),
            is_symlink: metadata.is_symlink(),
            size: metadata.len(),
        })
    }
}

/// Get metadata for a file or directory
pub fn metadata(path: &str) -> FsResult<FileMetadata> {
    let path = Path::new(path);
    let metadata = fs::metadata(path).map_err(FsError::from)?;
    Ok(FileMetadata::from_std_metadata(metadata))
}

/// Check if a path exists
pub fn exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// Check if a path is a file
pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}

/// Check if a path is a directory
pub fn is_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}

/// Get the size of a file in bytes
pub fn file_size(path: &str) -> FsResult<u64> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(FsError::NotFound(path.to_string_lossy().to_string()));
    }
    if !path.is_file() {
        return Err(FsError::NotAFile(path.to_string_lossy().to_string()));
    }
    
    let metadata = fs::metadata(path).map_err(FsError::from)?;
    Ok(metadata.len())
}

