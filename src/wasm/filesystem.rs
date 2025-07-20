//! WASM filesystem operations
//! 
//! Provides WASM-compatible filesystem operations using WASI
//! or browser-based storage APIs when available.

use std::io::{Error, ErrorKind, Result};
use std::path::Path;

/// WASM filesystem abstraction
pub struct WasmFileSystem;

impl WasmFileSystem {
    pub fn new() -> Self {
        Self
    }

    /// Read file with WASM compatibility
    #[cfg(target_arch = "wasm32")]
    pub fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>> {
        let path_str = path.as_ref().to_string_lossy();
        
        // For WASI environments, we can use std::fs
        #[cfg(target_os = "wasi")]
        {
            std::fs::read(path)
        }
        
        // For browser environments, files must be provided through other means
        #[cfg(not(target_os = "wasi"))]
        {
            Err(Error::new(
                ErrorKind::Unsupported,
                format!("File reading not supported in browser WASM for path: {}", path_str)
            ))
        }
    }

    /// Write file with WASM compatibility
    #[cfg(target_arch = "wasm32")]
    pub fn write_file<P: AsRef<Path>>(&self, path: P, contents: &[u8]) -> Result<()> {
        let path_str = path.as_ref().to_string_lossy();
        
        // For WASI environments, we can use std::fs
        #[cfg(target_os = "wasi")]
        {
            std::fs::write(path, contents)
        }
        
        // For browser environments, files must be downloaded or stored differently
        #[cfg(not(target_os = "wasi"))]
        {
            Err(Error::new(
                ErrorKind::Unsupported,
                format!("File writing not supported in browser WASM for path: {}", path_str)
            ))
        }
    }

    /// Check if file exists with WASM compatibility
    #[cfg(target_arch = "wasm32")]
    pub fn file_exists<P: AsRef<Path>>(&self, path: P) -> bool {
        #[cfg(target_os = "wasi")]
        {
            path.as_ref().exists()
        }
        
        #[cfg(not(target_os = "wasi"))]
        {
            // In browser WASM, we can't check file existence
            false
        }
    }

    /// Create directory with WASM compatibility
    #[cfg(target_arch = "wasm32")]
    pub fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        #[cfg(target_os = "wasi")]
        {
            std::fs::create_dir_all(path)
        }
        
        #[cfg(not(target_os = "wasi"))]
        {
            let path_str = path.as_ref().to_string_lossy();
            Err(Error::new(
                ErrorKind::Unsupported,
                format!("Directory creation not supported in browser WASM for path: {}", path_str)
            ))
        }
    }

    /// Native filesystem operations for non-WASM
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>> {
        std::fs::read(path)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn write_file<P: AsRef<Path>>(&self, path: P, contents: &[u8]) -> Result<()> {
        std::fs::write(path, contents)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn file_exists<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().exists()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        std::fs::create_dir_all(path)
    }
}

/// Conditional filesystem functions
#[cfg(target_arch = "wasm32")]
pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    #[cfg(target_os = "wasi")]
    {
        std::fs::read_to_string(path)
    }
    
    #[cfg(not(target_os = "wasi"))]
    {
        let path_str = path.as_ref().to_string_lossy();
        Err(Error::new(
            ErrorKind::Unsupported,
            format!("File reading not supported in browser WASM for path: {}", path_str)
        ))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    std::fs::read_to_string(path)
}

#[cfg(target_arch = "wasm32")]
pub fn write_string<P: AsRef<Path>>(path: P, contents: &str) -> Result<()> {
    #[cfg(target_os = "wasi")]
    {
        std::fs::write(path, contents)
    }
    
    #[cfg(not(target_os = "wasi"))]
    {
        let path_str = path.as_ref().to_string_lossy();
        Err(Error::new(
            ErrorKind::Unsupported,
            format!("File writing not supported in browser WASM for path: {}", path_str)
        ))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn write_string<P: AsRef<Path>>(path: P, contents: &str) -> Result<()> {
    std::fs::write(path, contents)
}
