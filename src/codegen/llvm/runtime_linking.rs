//! Runtime library linking support for the CURSED compiler.
//!
//! This module provides functionality for linking CURSED programs with
//! runtime libraries, including the standard library and system libraries.

use std::path::{Path, PathBuf};

/// Type of library linking to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibraryLinkType {
    /// Link the library statically
    Static,
    
    /// Link the library dynamically
    Dynamic,
}

/// Information about a library to link with.
#[derive(Debug, Clone)]
pub struct Library {
    /// Name of the library
    pub name: String,
    
    /// Path to the library file (if it's not a system library)
    pub path: Option<PathBuf>,
    
    /// Type of linking to use
    pub link_type: LibraryLinkType,
}

impl Library {
    /// Creates a new system library reference.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the system library
    /// * `link_type` - The type of linking to use
    ///
    /// # Returns
    ///
    /// A new Library instance representing a system library
    pub fn new_system(name: &str, link_type: LibraryLinkType) -> Self {
        Library {
            name: name.to_string(),
            path: None,
            link_type,
        }
    }
    
    /// Creates a new custom library reference.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the library
    /// * `path` - The path to the library file
    /// * `link_type` - The type of linking to use
    ///
    /// # Returns
    ///
    /// A new Library instance representing a custom library
    pub fn new_custom<P: AsRef<Path>>(name: &str, path: P, link_type: LibraryLinkType) -> Self {
        Library {
            name: name.to_string(),
            path: Some(path.as_ref().to_path_buf()),
            link_type,
        }
    }
    
    /// Gets the linker argument for this library.
    ///
    /// # Returns
    ///
    /// A string containing the appropriate linker argument
    pub fn get_linker_arg(&self) -> String {
        match (&self.path, self.link_type) {
            // Custom library with explicit path
            (Some(path), _) => path.to_string_lossy().to_string(),
            
            // System library with static linking
            (None, LibraryLinkType::Static) => format!("-l:lib{}.a", self.name),
            
            // System library with dynamic linking (default)
            (None, LibraryLinkType::Dynamic) => format!("-l{}", self.name),
        }
    }
}

/// Runtime linking options for compiled CURSED programs.
#[derive(Debug, Clone)]
pub struct RuntimeLinkingOptions {
    /// Whether to link with the standard library
    stdlib_enabled: bool,
    
    /// List of libraries to link with
    libraries: Vec<Library>,
    
    /// Additional linker arguments
    linker_args: Vec<String>,
    
    /// Library search paths
    lib_paths: Vec<PathBuf>,
}

impl RuntimeLinkingOptions {
    /// Creates new runtime linking options with default values.
    ///
    /// # Returns
    ///
    /// Default runtime linking options
    pub fn new() -> Self {
        RuntimeLinkingOptions {
            stdlib_enabled: true,
            libraries: Vec::new(),
            linker_args: Vec::new(),
            lib_paths: Vec::new(),
        }
    }
    
    /// Enables or disables linking with the standard library.
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to link with the standard library
    pub fn enable_stdlib(&mut self, enable: bool) {
        self.stdlib_enabled = enable;
    }
    
    /// Adds a system library to link with.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the system library
    pub fn add_system_library(&mut self, name: &str) {
        self.libraries.push(Library::new_system(name, LibraryLinkType::Dynamic));
    }
    
    /// Adds a custom library to link with.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the library
    /// * `path` - The path to the library file
    /// * `link_type` - The type of linking to use
    pub fn add_custom_library<P: AsRef<Path>>(
        &mut self,
        name: &str,
        path: P,
        link_type: LibraryLinkType,
    ) {
        self.libraries.push(Library::new_custom(name, path, link_type));
    }
    
    /// Adds a library search path.
    ///
    /// # Arguments
    ///
    /// * `path` - The library search path
    pub fn add_lib_path<P: AsRef<Path>>(&mut self, path: P) {
        self.lib_paths.push(path.as_ref().to_path_buf());
    }
    
    /// Adds a raw linker argument.
    ///
    /// # Arguments
    ///
    /// * `arg` - The linker argument
    pub fn add_linker_arg(&mut self, arg: &str) {
        self.linker_args.push(arg.to_string());
    }
    
    /// Gets the list of all linker arguments based on the configuration.
    ///
    /// # Returns
    ///
    /// A vector of strings containing all linker arguments
    pub fn get_linker_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        
        // Add standard library if enabled
        if self.stdlib_enabled {
            args.push("-lcursed_runtime".to_string());
        }
        
        // Add library search paths
        for path in &self.lib_paths {
            args.push(format!("-L{}", path.to_string_lossy()));
        }
        
        // Add libraries
        for lib in &self.libraries {
            args.push(lib.get_linker_arg());
        }
        
        // Add raw linker args
        args.extend(self.linker_args.clone());
        
        args
    }
}

impl Default for RuntimeLinkingOptions {
    fn default() -> Self {
        Self::new()
    }
}