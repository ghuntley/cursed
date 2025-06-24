use crate::stdlib::embed_that::core::{ThatFile, ThatFiles, tea, lit};
use crate::stdlib::embed_that::error::{EmbedError, EmbedResult};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::path::Path;
use crate::error::Error;

/// Global registry for embedded files
static EMBEDDED_REGISTRY: once_cell::sync::Lazy<Arc<Mutex<HashMap<tea, ThatFiles>>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Dynamic resource loading functions for embedded files
pub struct ResourceLoader;

impl ResourceLoader {
    /// Register embedded files in the global registry
    pub fn register_embedded_files(namespace: &tea, files: ThatFiles) -> EmbedResult<()> {
        let mut registry = EMBEDDED_REGISTRY.lock()
            .map_err(|e| EmbedError::General { message: format!("Registry lock failed: {}", e) })?;
        
        registry.insert(namespace.clone(), files);
        Ok(())
    }
    
    /// Load a single embedded file by path
    pub fn load_that_file(path: &tea) -> EmbedResult<ThatFile> {
        let registry = EMBEDDED_REGISTRY.lock()
            .map_err(|e| EmbedError::General { message: format!("Registry lock failed: {}", e) })?;
        
        // Try to find the file in any registered namespace
        for (_, files) in registry.iter() {
            let (file, found) = files.get(path);
            if found {
                return Ok(file);
            }
        }
        
        Err(EmbedError::FileNotFound { file: path.clone() })
    }
    
    /// Load all files from a directory pattern
    pub fn load_that_dir(path: &tea) -> EmbedResult<ThatFiles> {
        let registry = EMBEDDED_REGISTRY.lock()
            .map_err(|e| EmbedError::General { message: format!("Registry lock failed: {}", e) })?;
        
        let mut result_files = ThatFiles::new();
        let pattern = if path.ends_with('/') {
            format!("{}*", path)
        } else {
            format!("{}/*", path)
        };
        
        // Search all namespaces for matching files
        for (_, files) in registry.iter() {
            let filtered = files.filter(&pattern);
            for file in filtered.list() {
                result_files.add_file(file);
            }
        }
        
        if result_files.count() == 0 {
            return Err(EmbedError::FileNotFound { file: path.clone() });
        }
        
        Ok(result_files)
    }
    
    /// Load files matching a pattern
    pub fn load_that_pattern(pattern: &tea) -> EmbedResult<ThatFiles> {
        let registry = EMBEDDED_REGISTRY.lock()
            .map_err(|e| EmbedError::General { message: format!("Registry lock failed: {}", e) })?;
        
        let mut result_files = ThatFiles::new();
        
        // Search all namespaces for matching files
        for (_, files) in registry.iter() {
            let filtered = files.filter(pattern);
            for file in filtered.list() {
                result_files.add_file(file);
            }
        }
        
        if result_files.count() == 0 {
            return Err(EmbedError::InvalidPattern { pattern: pattern.clone() });
        }
        
        Ok(result_files)
    }
    
    /// Get all registered namespaces
    pub fn get_namespaces() -> EmbedResult<Vec<tea>> {
        let registry = EMBEDDED_REGISTRY.lock()
            .map_err(|e| EmbedError::General { message: format!("Registry lock failed: {}", e) })?;
        
        Ok(registry.keys().cloned().collect())
    }
    
    /// Get files from a specific namespace
    pub fn get_namespace_files(namespace: &tea) -> EmbedResult<ThatFiles> {
        let registry = EMBEDDED_REGISTRY.lock()
            .map_err(|e| EmbedError::General { message: format!("Registry lock failed: {}", e) })?;
        
        registry.get(namespace)
            .cloned()
            .ok_or_else(|| EmbedError::FileNotFound { file: namespace.clone() })
    }
    
    /// Check if a file exists in the embedded registry
    pub fn file_exists(path: &tea) -> lit {
        if let Ok(registry) = EMBEDDED_REGISTRY.lock() {
            for (_, files) in registry.iter() {
                let (_, found) = files.get(path);
                if found {
                    return true;
                }
            }
        }
        false
    }
    
    /// Get statistics about embedded files
    pub fn get_statistics() -> EmbedResult<EmbedStatistics> {
        let registry = EMBEDDED_REGISTRY.lock()
            .map_err(|e| EmbedError::General { message: format!("Registry lock failed: {}", e) })?;
        
        let mut total_files = 0;
        let mut total_size = 0i64;
        let mut namespaces = 0;
        let mut file_types = HashMap::new();
        
        for (_, files) in registry.iter() {
            namespaces += 1;
            total_files += files.count();
            total_size += files.total_size();
            
            // Count file types
            for file in files.list() {
                let extension = file.extension().to_lowercase();
                let extension = if extension.is_empty() { "no_extension".to_string() } else { extension };
                *file_types.entry(extension).or_insert(0) += 1;
            }
        }
        
        Ok(EmbedStatistics {
            total_files,
            total_size,
            namespaces,
            file_types,
        })
    }
    
    /// Clear all embedded files (primarily for testing)
    pub fn clear_registry() -> EmbedResult<()> {
        let mut registry = EMBEDDED_REGISTRY.lock()
            .map_err(|e| EmbedError::General { message: format!("Registry lock failed: {}", e) })?;
        
        registry.clear();
        Ok(())
    }
    
    /// Load embedded files from a directory on disk (for development/testing)
    #[cfg(feature = "filesystem_embedding")]
    pub fn load_from_filesystem(directory: &tea, namespace: &tea) -> EmbedResult<ThatFiles> {
        use std::fs;
        use walkdir::WalkDir;
        
        let mut files = ThatFiles::new();
        let base_path = Path::new(directory);
        
        if !base_path.exists() {
            return Err(EmbedError::FileNotFound { file: directory.clone() });
        }
        
        for entry in WalkDir::new(base_path) {
            let entry = entry.map_err(|e| EmbedError::IoError { reason: e.to_string() })?;
            
            if entry.file_type().is_file() {
                let path = entry.path();
                let relative_path = path.strip_prefix(base_path)
                    .map_err(|e| EmbedError::General { message: e.to_string() })?;
                
                let content = fs::read(path)
                    .map_err(|e| EmbedError::IoError { reason: e.to_string() })?;
                
                let mod_time = fs::metadata(path)
                    .and_then(|m| m.modified())
                    .unwrap_or_else(|_| std::time::SystemTime::now());
                
                let file_name = relative_path.to_string_lossy().to_string();
                let that_file = ThatFile::with_metadata(file_name, content, mod_time);
                
                files.add_file(that_file);
            }
        }
        
        Self::register_embedded_files(namespace, files.clone())?;
        Ok(files)
    }
    
    /// Build embedded files from a manifest (compile-time embedding)
    pub fn build_from_manifest(manifest: &EmbedManifest) -> EmbedResult<ThatFiles> {
        let mut files = ThatFiles::new();
        
        for item in &manifest.items {
            match item {
                ManifestItem::File { path, embedded_path } => {
                    if let Ok(content) = std::fs::read(path) {
                        let mod_time = std::fs::metadata(path)
                            .and_then(|m| m.modified())
                            .unwrap_or_else(|_| std::time::SystemTime::now());
                        
                        let that_file = ThatFile::with_metadata(
                            embedded_path.clone(), 
                            content, 
                            mod_time
                        );
                        files.add_file(that_file);
                    }
                },
                ManifestItem::Directory { path, pattern, prefix } => {
                    if let Ok(entries) = std::fs::read_dir(path) {
                        for entry in entries.flatten() {
                            if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                                let file_path = entry.path();
                                let file_name = file_path.file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("")
                                    .to_string();
                                
                                // Check pattern matching
                                if pattern.as_ref().map_or(true, |p| glob_matches(p, &file_name)) {
                                    if let Ok(content) = std::fs::read(&file_path) {
                                        let mod_time = std::fs::metadata(&file_path)
                                            .and_then(|m| m.modified())
                                            .unwrap_or_else(|_| std::time::SystemTime::now());
                                        
                                        let embedded_path = if let Some(ref prefix) = prefix {
                                            format!("{}/{}", prefix, file_name)
                                        } else {
                                            file_name
                                        };
                                        
                                        let that_file = ThatFile::with_metadata(
                                            embedded_path, 
                                            content, 
                                            mod_time
                                        );
                                        files.add_file(that_file);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(files)
    }
}

/// Statistics about embedded files
#[derive(Debug, Clone)]
pub struct EmbedStatistics {
    pub total_files: i32,
    pub total_size: i64,
    pub namespaces: i32,
    pub file_types: HashMap<tea, i32>,
}

/// Manifest for build-time embedding
#[derive(Debug, Clone)]
pub struct EmbedManifest {
    pub namespace: tea,
    pub items: Vec<ManifestItem>,
}

/// Items that can be embedded
#[derive(Debug, Clone)]
pub enum ManifestItem {
    File {
        path: tea,
        embedded_path: tea,
    },
    Directory {
        path: tea,
        pattern: Option<tea>,
        prefix: Option<tea>,
    },
}

/// Public API functions for dynamic resource loading
pub fn load_that_file(path: &tea) -> EmbedResult<ThatFile> {
    ResourceLoader::load_that_file(path)
}

pub fn load_that_dir(path: &tea) -> EmbedResult<ThatFiles> {
    ResourceLoader::load_that_dir(path)
}

pub fn load_that_pattern(pattern: &tea) -> EmbedResult<ThatFiles> {
    ResourceLoader::load_that_pattern(pattern)
}

pub fn file_exists(path: &tea) -> lit {
    ResourceLoader::file_exists(path)
}

pub fn get_embed_statistics() -> EmbedResult<EmbedStatistics> {
    ResourceLoader::get_statistics()
}

/// Simple glob pattern matching
fn glob_matches(pattern: &str, text: &str) -> bool {
    let regex_pattern = super::core::glob_to_regex(pattern);
    regex_pattern.is_match(text)
}

/// Initialize the resource loader with default embedded files
pub fn initialize_resource_loader() -> EmbedResult<()> {
    // This would typically be called by the build system
    // to register compile-time embedded files
    Ok(())
}

/// Helper macro for registering embedded files at compile time
#[macro_export]
macro_rules! embed_files {
    ($namespace:expr, $($path:expr),*) => {
        {
            let mut files = $crate::stdlib::embed_that::core::ThatFiles::new();
            $(
                if let Ok(content) = std::fs::read($path) {
                    let file = $crate::stdlib::embed_that::core::ThatFile::new(
                        $path.to_string(), 
                        content
                    );
                    files.add_file(file);
                }
            )*
            $crate::stdlib::embed_that::resource_loader::ResourceLoader::register_embedded_files(
                &$namespace.to_string(), 
                files
            )
        }
    };
}

/// Helper macro for embedding a directory
#[macro_export]
macro_rules! embed_dir {
    ($namespace:expr, $dir:expr) => {
        {
            #[cfg(feature = "filesystem_embedding")]
            {
                $crate::stdlib::embed_that::resource_loader::ResourceLoader::load_from_filesystem(
                    &$dir.to_string(),
                    &$namespace.to_string()
                )
            }
            #[cfg(not(feature = "filesystem_embedding"))]
            {
                Ok($crate::stdlib::embed_that::core::ThatFiles::new())
            }
        }
    };
}
