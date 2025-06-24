use crate::error::Error;
use std::fs;
use std::path::Path;
use crate::stdlib::fs::error::{FsError, FsResult};
use crate::stdlib::fs::metadata::DirEntry;

/// Create a directory
pub fn create_dir(path: &str) -> FsResult<()> {
    let path = Path::new(path);
    
    if path.exists() {
        return Err(FsError::AlreadyExists(path.to_string_lossy().to_string()));
    }
    
    fs::create_dir(path).map_err(FsError::from)
}

/// Create a directory and all of its parent directories as needed
pub fn create_dir_all(path: &str) -> FsResult<()> {
    let path = Path::new(path);
    fs::create_dir_all(path).map_err(FsError::from)
}

/// Remove an empty directory
pub fn remove_dir(path: &str) -> FsResult<()> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(FsError::NotFound(path.to_string_lossy().to_string()));
    }
    
    if !path.is_dir() {
        return Err(FsError::NotADirectory(path.to_string_lossy().to_string()));
    }
    
    fs::remove_dir(path).map_err(|e| {
        match e.kind() {
            std::io::ErrorKind::Other => {
                // Directory not empty
                FsError::DirectoryNotEmpty(path.to_string_lossy().to_string())
            }
            _ => FsError::from(e)
        }
    })
}

/// Remove a directory and all its contents recursively
pub fn remove_dir_all(path: &str) -> FsResult<()> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(FsError::NotFound(path.to_string_lossy().to_string()));
    }
    
    if !path.is_dir() {
        return Err(FsError::NotADirectory(path.to_string_lossy().to_string()));
    }
    
    fs::remove_dir_all(path).map_err(FsError::from)
}

/// List the contents of a directory
pub fn list_dir(path: &str) -> FsResult<Vec<DirEntry>> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(FsError::NotFound(path.to_string_lossy().to_string()));
    }
    
    if !path.is_dir() {
        return Err(FsError::NotADirectory(path.to_string_lossy().to_string()));
    }
    
    let entries = fs::read_dir(path).map_err(FsError::from)?;
    let mut result = Vec::new();
    
    for entry in entries {
        let entry = entry.map_err(FsError::from)?;
        let dir_entry = DirEntry::from_std_entry(entry)?;
        result.push(dir_entry);
    }
    
    // Sort entries by name for consistent ordering
    result.sort_by(|a, b| a.name.cmp(&b.name));
    
    Ok(result)
}

/// List only files in a directory
pub fn list_files(path: &str) -> FsResult<Vec<DirEntry>> {
    let entries = list_dir(path)?;
    Ok(entries.into_iter().filter(|entry| entry.is_file).collect())
}

/// List only directories in a directory
pub fn list_dirs(path: &str) -> FsResult<Vec<DirEntry>> {
    let entries = list_dir(path)?;
    Ok(entries.into_iter().filter(|entry| entry.is_dir).collect())
}

/// Copy a directory and all its contents recursively
pub fn copy_dir_all(from: &str, to: &str) -> FsResult<()> {
    let from_path = Path::new(from);
    let to_path = Path::new(to);
    
    if !from_path.exists() {
        return Err(FsError::NotFound(from_path.to_string_lossy().to_string()));
    }
    
    if !from_path.is_dir() {
        return Err(FsError::NotADirectory(from_path.to_string_lossy().to_string()));
    }
    
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
                &entry_path.to_string_lossy(),
                &dest_path.to_string_lossy()
            )?;
        } else {
            fs::copy(&entry_path, &dest_path).map_err(FsError::from)?;
        }
    }
    
    Ok(())
}

/// Walk a directory tree recursively and call a function for each entry
pub fn walk_dir<F>(path: &str, mut callback: F) -> FsResult<()>
where
    F: FnMut(&DirEntry) -> FsResult<bool>, // Return false to skip directory
{
    fn walk_recursive<F>(path: &str, callback: &mut F) -> FsResult<()>
    where
        F: FnMut(&DirEntry) -> FsResult<bool>,
    {
        let entries = list_dir(path)?;
        
        for entry in entries {
            let should_continue = callback(&entry)?;
            
            if should_continue && entry.is_dir {
                walk_recursive(&entry.path, callback)?;
            }
        }
        
        Ok(())
    }
    
    walk_recursive(path, &mut callback)
}

/// Find all files matching a pattern in a directory tree
pub fn find_files<F>(path: &str, predicate: F) -> FsResult<Vec<DirEntry>>
where
    F: Fn(&DirEntry) -> bool,
{
    let mut results = Vec::new();
    
    walk_dir(path, |entry| {
        if entry.is_file && predicate(entry) {
            results.push(entry.clone());
        }
        Ok(true) // Continue walking
    })?;
    
    Ok(results)
}

/// Get the total size of a directory and all its contents
pub fn dir_size(path: &str) -> FsResult<u64> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(FsError::NotFound(path.to_string_lossy().to_string()));
    }
    
    if !path.is_dir() {
        return Err(FsError::NotADirectory(path.to_string_lossy().to_string()));
    }
    
    let mut total_size = 0u64;
    
    walk_dir(&path.to_string_lossy(), |entry| {
        total_size += entry.size;
        Ok(true)
    })?;
    
    Ok(total_size)
}

/// Count the number of files and directories in a directory tree
pub fn count_entries(path: &str) -> FsResult<(usize, usize)> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(FsError::NotFound(path.to_string_lossy().to_string()));
    }
    
    if !path.is_dir() {
        return Err(FsError::NotADirectory(path.to_string_lossy().to_string()));
    }
    
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::fs::file_ops::write_file;
    use tempfile::TempDir;

    #[test]
    fn test_create_and_remove_dir() {
        let temp_dir = TempDir::new().unwrap();
        let test_dir = temp_dir.path().join("test_directory");
        let test_dir_str = test_dir.to_string_lossy().to_string();
        
        // Create directory
        create_dir(&test_dir_str).unwrap();
        assert!(test_dir.exists());
        assert!(test_dir.is_dir());
        
        // Remove directory
        remove_dir(&test_dir_str).unwrap();
        assert!(!test_dir.exists());
    }

    #[test]
    fn test_create_dir_all() {
        let temp_dir = TempDir::new().unwrap();
        let nested_dir = temp_dir.path().join("level1").join("level2").join("level3");
        let nested_dir_str = nested_dir.to_string_lossy().to_string();
        
        // Create nested directories
        create_dir_all(&nested_dir_str).unwrap();
        assert!(nested_dir.exists());
        assert!(nested_dir.is_dir());
    }

    #[test]
    fn test_list_dir() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().to_string_lossy().to_string();
        
        // Create some files and directories
        let file1_path = temp_dir.path().join("file1.txt");
        let file2_path = temp_dir.path().join("file2.txt");
        let subdir_path = temp_dir.path().join("subdir");
        
        write_file(&file1_path.to_string_lossy(), "content1").unwrap();
        write_file(&file2_path.to_string_lossy(), "content2").unwrap();
        create_dir(&subdir_path.to_string_lossy()).unwrap();
        
        // List directory contents
        let entries = list_dir(&base_path).unwrap();
        assert_eq!(entries.len(), 3);
        
        // Check that we have the expected entries
        let names: Vec<_> = entries.iter().map(|e| &e.name).collect();
        assert!(names.contains(&&"file1.txt".to_string()));
        assert!(names.contains(&&"file2.txt".to_string()));
        assert!(names.contains(&&"subdir".to_string()));
    }

    #[test]
    fn test_copy_dir_all() {
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("source");
        let dst_dir = temp_dir.path().join("destination");
        
        // Create source structure
        create_dir(&src_dir.to_string_lossy()).unwrap();
        let file_path = src_dir.join("test.txt");
        let subdir_path = src_dir.join("subdir");
        let subfile_path = subdir_path.join("subfile.txt");
        
        write_file(&file_path.to_string_lossy(), "content").unwrap();
        create_dir(&subdir_path.to_string_lossy()).unwrap();
        write_file(&subfile_path.to_string_lossy(), "subcontent").unwrap();
        
        // Copy directory
        copy_dir_all(
            &src_dir.to_string_lossy(),
            &dst_dir.to_string_lossy()
        ).unwrap();
        
        // Verify destination
        assert!(dst_dir.exists());
        assert!(dst_dir.join("test.txt").exists());
        assert!(dst_dir.join("subdir").exists());
        assert!(dst_dir.join("subdir").join("subfile.txt").exists());
    }

    #[test]
    fn test_find_files() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().to_string_lossy().to_string();
        
        // Create test files
        write_file(&temp_dir.path().join("test1.txt").to_string_lossy(), "content").unwrap();
        write_file(&temp_dir.path().join("test2.rs").to_string_lossy(), "content").unwrap();
        write_file(&temp_dir.path().join("README.md").to_string_lossy(), "content").unwrap();
        
        // Find .txt files
        let txt_files = find_files(&base_path, |entry| {
            entry.name.ends_with(".txt")
        }).unwrap();
        
        assert_eq!(txt_files.len(), 1);
        assert_eq!(txt_files[0].name, "test1.txt");
    }

    #[test]
    fn test_dir_size() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().to_string_lossy().to_string();
        
        // Create files with known sizes
        write_file(&temp_dir.path().join("file1.txt").to_string_lossy(), "12345").unwrap(); // 5 bytes
        write_file(&temp_dir.path().join("file2.txt").to_string_lossy(), "123").unwrap();   // 3 bytes
        
        let size = dir_size(&base_path).unwrap();
        assert_eq!(size, 8); // 5 + 3 bytes
    }
}
