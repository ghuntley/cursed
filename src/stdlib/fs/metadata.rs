use crate::error::CursedError;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
// use crate::stdlib::fs::error::{FsError, FsResult};

/// File or directory metadata
#[derive(Debug, Clone)]
pub struct FileMetadata {
    /// Size in bytes (0 for directories)
    /// Whether this is a file
    /// Whether this is a directory
    /// Whether this is a symbolic link
    /// Whether the file is read-only
    /// Created timestamp (if available)
    /// Last modified timestamp
    /// Last accessed timestamp (if available)
impl FileMetadata {
    /// Create metadata from std::fs::Metadata
    pub fn from_std_metadata(metadata: fs::Metadata) -> Self {
        Self {
        }
    }
/// Directory entry information
#[derive(Debug, Clone)]
pub struct DirEntry {
    /// Full path to the entry
    /// Entry name (filename or directory name)
    /// Whether this entry is a file
    /// Whether this entry is a directory
    /// Whether this entry is a symbolic link
    /// File size (0 for directories)
impl DirEntry {
    /// Create DirEntry from std::fs::DirEntry
    pub fn from_std_entry(entry: fs::DirEntry) -> FsResult<Self> {
        let path = entry.path();
        let metadata = entry.metadata().map_err(FsError::from)?;
        
        Ok(Self {
        })
    }
}

/// Get metadata for a file or directory
pub fn metadata(path: &str) -> FsResult<FileMetadata> {
    let path = Path::new(path);
    let metadata = fs::metadata(path).map_err(FsError::from)?;
    Ok(FileMetadata::from_std_metadata(metadata))
/// Check if a path exists
pub fn exists(path: &str) -> bool {
    Path::new(path).exists()
/// Check if a path is a file
pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
/// Check if a path is a directory
pub fn is_dir(path: &str) -> bool {
    Path::new(path).is_dir()
/// Get the size of a file in bytes
pub fn file_size(path: &str) -> FsResult<u64> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(FsError::NotFound(path.to_string_lossy().to_string()));
    }
    if !path.is_file() {
        return Err(FsError::NotAFile(path.to_string_lossy().to_string()));
    let metadata = fs::metadata(path).map_err(FsError::from)?;
    Ok(metadata.len())
