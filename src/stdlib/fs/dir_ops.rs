use crate::error::CursedError;
use std::fs;
use std::path::Path;
// use crate::stdlib::fs::error::{FsError, FsResult};
// use crate::stdlib::fs::metadata::DirEntry;

/// Create a directory
pub fn create_dir(path: &str) -> FsResult<()> {
    let path = Path::new(path);
    
    if path.exists() {
        return Err(FsError::AlreadyExists(path.to_string_lossy().to_string()));
    fs::create_dir(path).map_err(FsError::from)
/// Create a directory and all of its parent directories as needed
pub fn create_dir_all(path: &str) -> FsResult<()> {
    let path = Path::new(path);
    fs::create_dir_all(path).map_err(FsError::from)
/// Remove an empty directory
pub fn remove_dir(path: &str) -> FsResult<()> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(FsError::NotFound(path.to_string_lossy().to_string()));
    if !path.is_dir() {
        return Err(FsError::NotADirectory(path.to_string_lossy().to_string()));
    fs::remove_dir(path).map_err(|e| {
        match e.kind() {
            std::io::ErrorKind::Other => {
                // Directory not empty
                FsError::DirectoryNotEmpty(path.to_string_lossy().to_string())
            }
            _ => FsError::from(e)
        }
    })
/// Remove a directory and all its contents recursively
pub fn remove_dir_all(path: &str) -> FsResult<()> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(FsError::NotFound(path.to_string_lossy().to_string()));
    if !path.is_dir() {
        return Err(FsError::NotADirectory(path.to_string_lossy().to_string()));
    fs::remove_dir_all(path).map_err(FsError::from)
/// List the contents of a directory
pub fn list_dir(path: &str) -> FsResult<Vec<DirEntry>> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(FsError::NotFound(path.to_string_lossy().to_string()));
    if !path.is_dir() {
        return Err(FsError::NotADirectory(path.to_string_lossy().to_string()));
    let entries = fs::read_dir(path).map_err(FsError::from)?;
    let mut result = Vec::new();
    
    for entry in entries {
        let entry = entry.map_err(FsError::from)?;
        let dir_entry = DirEntry::from_std_entry(entry)?;
        result.push(dir_entry);
    // Sort entries by name for consistent ordering
    result.sort_by(|a, b| a.name.cmp(&b.name));
    
    Ok(result)
/// List only files in a directory
pub fn list_files(path: &str) -> FsResult<Vec<DirEntry>> {
    let entries = list_dir(path)?;
    Ok(entries.into_iter().filter(|entry| entry.is_file).collect())
/// List only directories in a directory
pub fn list_dirs(path: &str) -> FsResult<Vec<DirEntry>> {
    let entries = list_dir(path)?;
    Ok(entries.into_iter().filter(|entry| entry.is_dir).collect())
/// Copy a directory and all its contents recursively
pub fn copy_dir_all(from: &str, to: &str) -> FsResult<()> {
    let from_path = Path::new(from);
    let to_path = Path::new(to);
    
    if !from_path.exists() {
        return Err(FsError::NotFound(from_path.to_string_lossy().to_string()));
    if !from_path.is_dir() {
        return Err(FsError::NotADirectory(from_path.to_string_lossy().to_string()));
    // Create destination directory
    create_dir_all(&to_path.to_string_lossy())?;
    
    // Copy all entries
    let entries = fs::read_dir(from_path).map_err(FsError::from)?;
    
    for entry in entries {
        let entry = entry.map_err(FsError::from)?;
        let entry_path = entry.path();
        let dest_path = to_path.join(entry.file_name());
        
        if entry_path.is_dir() {
            copy_dir_all(
                &dest_path.to_string_lossy()
            )?;
        } else {
            fs::copy(&entry_path, &dest_path).map_err(FsError::from)?;
        }
    }
    
    Ok(())
/// Walk a directory tree recursively and call a function for each entry
pub fn walk_dir<F>(path: &str, mut callback: F) -> FsResult<()>
where
    F: FnMut(&DirEntry) -> FsResult<bool>, // Return false to skip directory
{
    fn walk_recursive<F>(path: &str, callback: &mut F) -> FsResult<()>
    where
    {
        let entries = list_dir(path)?;
        
        for entry in entries {
            let should_continue = callback(&entry)?;
            
            if should_continue && entry.is_dir {
                walk_recursive(&entry.path, callback)?;
            }
        }
        
        Ok(())
    walk_recursive(path, &mut callback)
/// Find all files matching a pattern in a directory tree
pub fn find_files<F>(path: &str, predicate: F) -> FsResult<Vec<DirEntry>>
where
{
    let mut results = Vec::new();
    
    walk_dir(path, |entry| {
        if entry.is_file && predicate(entry) {
            results.push(entry.clone());
        }
        Ok(true) // Continue walking
    })?;
    
    Ok(results)
/// Get the total size of a directory and all its contents
pub fn dir_size(path: &str) -> FsResult<u64> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(FsError::NotFound(path.to_string_lossy().to_string()));
    if !path.is_dir() {
        return Err(FsError::NotADirectory(path.to_string_lossy().to_string()));
    let mut total_size = 0u64;
    
    walk_dir(&path.to_string_lossy(), |entry| {
        total_size += entry.size;
        Ok(true)
    })?;
    
    Ok(total_size)
/// Count the number of files and directories in a directory tree
pub fn count_entries(path: &str) -> FsResult<(usize, usize)> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(FsError::NotFound(path.to_string_lossy().to_string()));
    if !path.is_dir() {
        return Err(FsError::NotADirectory(path.to_string_lossy().to_string()));
    let mut file_count = 0usize;
    let mut dir_count = 0usize;
    
    walk_dir(&path.to_string_lossy(), |entry| {
        if entry.is_file {
            file_count += 1;
        } else if entry.is_dir {
            dir_count += 1;
        }
        Ok(true)
    })?;
    
    Ok((file_count, dir_count))
