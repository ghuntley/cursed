/// Source location tracking for debug information
use std::path::PathBuf;
use tracing::{debug, instrument};

/// Represents a location in source code
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceLocation {
impl SourceLocation {
    /// Create a new source location
    pub fn new(file: PathBuf, line: u32, column: u32) -> Self {
        Self {
        }
    }

    /// Create a source location with end position
    pub fn new_range(
    ) -> Self {
        Self {
        }
    }

    /// Get the file name as a string
    pub fn file_name(&self) -> String {
        self.file
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "<unknown>".to_string())
    /// Get the directory path
    pub fn directory(&self) -> PathBuf {
        self.file.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf()
    /// Format as a human-readable string
    pub fn display(&self) -> String {
        if let (Some(end_line), Some(end_column)) = (self.end_line, self.end_column) {
            format!(
                end_column
            )
        } else {
            format!("{}:{}:{}", self.file_name(), self.line, self.column)
        }
    }

    /// Check if this location is valid
    pub fn is_valid(&self) -> bool {
        self.line > 0 && self.column > 0
    }
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self {
        }
    }
impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

/// Convert from error::SourceLocation to debug::SourceLocation
impl From<crate::error::SourceLocation> for SourceLocation {
    fn from(error_loc: crate::error::SourceLocation) -> Self {
        let file = error_loc.file.map(PathBuf::from).unwrap_or_else(|| PathBuf::from("<unknown>"));
        SourceLocation::new(file, error_loc.line as u32, error_loc.column as u32)
    }
}

/// Convert from debug::SourceLocation to error::SourceLocation  
impl From<SourceLocation> for crate::error::SourceLocation {
    fn from(debug_loc: SourceLocation) -> Self {
        crate::error::SourceLocation {
        }
    }
/// Source location manager for tracking locations throughout compilation
#[derive(Debug)]
pub struct SourceLocationManager {
impl SourceLocationManager {
    /// Create a new source location manager
    pub fn new() -> Self {
        Self {
        }
    }

    /// Set the current file being processed
    #[instrument(skip(self))]
    pub fn set_current_file(&mut self, file: PathBuf) {
        debug!(file = ?file, "Setting current file");
        self.current_file = Some(file);
    /// Push a location onto the stack
    #[instrument(skip(self))]
    pub fn push_location(&mut self, location: SourceLocation) {
        debug!(location = ?location, "Pushing location");
        self.location_stack.push(location);
    /// Pop a location from the stack
    #[instrument(skip(self))]
    pub fn pop_location(&mut self) -> Option<SourceLocation> {
        let location = self.location_stack.pop();
        debug!(location = ?location, "Popping location");
        location
    /// Get the current location
    pub fn current_location(&self) -> Option<&SourceLocation> {
        self.location_stack.last()
    /// Create a new location in the current file
    pub fn create_location(&self, line: u32, column: u32) -> SourceLocation {
        let file = self.current_file.clone().unwrap_or_else(|| PathBuf::from("<unknown>"));
        SourceLocation::new(file, line, column)
    /// Create a location range in the current file
    pub fn create_range(&self, line: u32, column: u32, end_line: u32, end_column: u32) -> SourceLocation {
        let file = self.current_file.clone().unwrap_or_else(|| PathBuf::from("<unknown>"));
        SourceLocation::new_range(file, line, column, end_line, end_column)
    /// Get the current file path
    pub fn current_file(&self) -> Option<&PathBuf> {
        self.current_file.as_ref()
    /// Get the location stack depth
    pub fn stack_depth(&self) -> usize {
        self.location_stack.len()
    /// Clear all locations
    pub fn clear(&mut self) {
        self.location_stack.clear();
        self.current_file = None;
    }
}

impl Default for SourceLocationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for AST nodes that can provide source location information
pub trait WithSourceLocation {
    /// Get the source location for this node
    fn source_location(&self) -> Option<&SourceLocation>;

    /// Set the source location for this node
    fn set_source_location(&mut self, location: SourceLocation);
/// Alias for compatibility
pub type SourceLocationInfo = SourceLocation;

