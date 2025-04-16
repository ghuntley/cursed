//! Runtime linking module for binary compiler.
//! This is a stub implementation until the full module is implemented.

use std::path::Path;

/// Library linking type (static or dynamic)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibraryLinkType {
    /// Static linking
    Static,
    /// Dynamic linking
    Dynamic,
}

/// Options for runtime library linking
#[derive(Debug, Clone)]
pub struct RuntimeLinkingOptions {
    /// Whether to link the standard runtime library
    pub link_stdlib: bool,
    /// Custom libraries to link
    pub custom_libraries: Vec<(String, String, LibraryLinkType)>,
}

impl RuntimeLinkingOptions {
    /// Create new default runtime linking options
    pub fn new() -> Self {
        Self {
            link_stdlib: true,
            custom_libraries: Vec::new(),
        }
    }
    
    /// Add a custom library to link
    pub fn add_library<P: AsRef<Path>>(&mut self, name: &str, path: P, link_type: LibraryLinkType) {
        self.custom_libraries.push((
            name.to_string(),
            path.as_ref().to_string_lossy().to_string(),
            link_type,
        ));
    }
}