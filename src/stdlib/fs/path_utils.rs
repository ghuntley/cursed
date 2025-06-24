use crate::error::Error;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use crate::stdlib::fs::error::{FsError, FsResult};

/// Join multiple path components into a single path
pub fn join_path(parts: Vec<String>) -> String {
    let mut path = PathBuf::new();
    for part in parts {
        path.push(part);
    }
    path.to_string_lossy().to_string()
}

/// Get the parent directory of a path
pub fn parent_dir(path: &str) -> Option<String> {
    Path::new(path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
}

/// Get the filename from a path
pub fn file_name(path: &str) -> Option<String> {
    Path::new(path)
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
}

/// Get the file extension from a path
pub fn extension(path: &str) -> Option<String> {
    Path::new(path)
        .extension()
        .map(|ext| ext.to_string_lossy().to_string())
}

/// Get the file stem (filename without extension)
pub fn file_stem(path: &str) -> Option<String> {
    Path::new(path)
        .file_stem()
        .map(|stem| stem.to_string_lossy().to_string())
}

/// Convert a path to an absolute path
pub fn absolute_path(path: &str) -> FsResult<String> {
    let path = Path::new(path);
    let absolute = path.canonicalize()
        .map_err(FsError::from)?;
    Ok(absolute.to_string_lossy().to_string())
}

/// Check if a path is absolute
pub fn is_absolute(path: &str) -> bool {
    Path::new(path).is_absolute()
}

/// Check if a path is relative
pub fn is_relative(path: &str) -> bool {
    Path::new(path).is_relative()
}

/// Get the current working directory
pub fn current_dir() -> FsResult<String> {
    std::env::current_dir()
        .map_err(FsError::from)
        .map(|path| path.to_string_lossy().to_string())
}

/// Normalize a path by resolving `.` and `..` components
pub fn normalize_path(path: &str) -> String {
    let path = Path::new(path);
    let mut components = Vec::new();
    
    for component in path.components() {
        match component {
            std::path::Component::Normal(name) => {
                components.push(name.to_string_lossy().to_string());
            }
            std::path::Component::ParentDir => {
                if !components.is_empty() {
                    components.pop();
                }
            }
            std::path::Component::CurDir => {
                // Skip current directory references
            }
            std::path::Component::RootDir => {
                components.clear();
                components.push(String::new()); // Will be joined as root
            }
            std::path::Component::Prefix(_) => {
                // Handle Windows drive prefixes
                components.push(component.as_os_str().to_string_lossy().to_string());
            }
        }
    }
    
    if components.is_empty() {
        ".".to_string()
    } else if components.len() == 1 && components[0].is_empty() {
        MAIN_SEPARATOR.to_string()
    } else {
        components.join(&MAIN_SEPARATOR.to_string())
    }
}

/// Split a path into directory and filename components
pub fn split_path(path: &str) -> (Option<String>, Option<String>) {
    let path = Path::new(path);
    let parent = path.parent().map(|p| p.to_string_lossy().to_string());
    let filename = path.file_name().map(|name| name.to_string_lossy().to_string());
    (parent, filename)
}

/// Check if one path is an ancestor of another
pub fn is_ancestor(ancestor: &str, descendant: &str) -> FsResult<bool> {
    let ancestor_abs = absolute_path(ancestor)?;
    let descendant_abs = absolute_path(descendant)?;
    
    let ancestor_path = Path::new(&ancestor_abs);
    let descendant_path = Path::new(&descendant_abs);
    
    Ok(descendant_path.starts_with(ancestor_path))
}

/// Get the relative path from one path to another
pub fn relative_path(from: &str, to: &str) -> FsResult<String> {
    let from_abs = absolute_path(from)?;
    let to_abs = absolute_path(to)?;
    
    let from_path = Path::new(&from_abs);
    let to_path = Path::new(&to_abs);
    
    // Find common prefix
    let mut from_components: Vec<_> = from_path.components().collect();
    let mut to_components: Vec<_> = to_path.components().collect();
    
    // Remove common prefix
    while !from_components.is_empty() && !to_components.is_empty() 
        && from_components[0] == to_components[0] {
        from_components.remove(0);
        to_components.remove(0);
    }
    
    // Build relative path
    let mut result = PathBuf::new();
    
    // Add parent directory references for remaining from components
    for _ in from_components {
        result.push("..");
    }
    
    // Add remaining to components
    for component in to_components {
        result.push(component);
    }
    
    Ok(result.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_path() {
        let parts = vec![
            "home".to_string(),
            "user".to_string(),
            "documents".to_string(),
            "file.txt".to_string()
        ];
        let joined = join_path(parts);
        assert!(joined.contains("home"));
        assert!(joined.contains("user"));
        assert!(joined.contains("documents"));
        assert!(joined.contains("file.txt"));
    }

    #[test]
    fn test_parent_dir() {
        assert_eq!(parent_dir("/home/user/file.txt"), Some("/home/user".to_string()));
        assert_eq!(parent_dir("file.txt"), Some("".to_string()));
        assert_eq!(parent_dir("/"), None);
    }

    #[test]
    fn test_file_name() {
        assert_eq!(file_name("/home/user/file.txt"), Some("file.txt".to_string()));
        assert_eq!(file_name("/home/user/"), Some("user".to_string()));
        assert_eq!(file_name("file.txt"), Some("file.txt".to_string()));
    }

    #[test]
    fn test_extension() {
        assert_eq!(extension("file.txt"), Some("txt".to_string()));
        assert_eq!(extension("archive.tar.gz"), Some("gz".to_string()));
        assert_eq!(extension("filename"), None);
        assert_eq!(extension(".hidden"), None);
    }

    #[test]
    fn test_file_stem() {
        assert_eq!(file_stem("file.txt"), Some("file".to_string()));
        assert_eq!(file_stem("archive.tar.gz"), Some("archive.tar".to_string()));
        assert_eq!(file_stem("filename"), Some("filename".to_string()));
    }

    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path("./file.txt"), "file.txt");
        assert_eq!(normalize_path("../file.txt"), "file.txt");
        assert_eq!(normalize_path("dir/../file.txt"), "file.txt");
        assert_eq!(normalize_path("./dir/./file.txt"), join_path(vec!["dir".to_string(), "file.txt".to_string()]));
    }

    #[test]
    fn test_split_path() {
        let (dir, file) = split_path("/home/user/file.txt");
        assert_eq!(dir, Some("/home/user".to_string()));
        assert_eq!(file, Some("file.txt".to_string()));
        
        let (dir, file) = split_path("file.txt");
        assert_eq!(dir, Some("".to_string()));
        assert_eq!(file, Some("file.txt".to_string()));
    }
}
