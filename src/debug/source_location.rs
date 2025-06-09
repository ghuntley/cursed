/// Source location tracking for debug information
use std::path::PathBuf;
use tracing::{debug, instrument};

/// Represents a location in source code
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line: u32,
    pub column: u32,
    pub end_line: Option<u32>,
    pub end_column: Option<u32>,
}

impl SourceLocation {
    /// Create a new source location
    pub fn new(file: PathBuf, line: u32, column: u32) -> Self {
        Self {
            file,
            line,
            column,
            end_line: None,
            end_column: None,
        }
    }

    /// Create a source location with end position
    pub fn new_range(
        file: PathBuf,
        line: u32,
        column: u32,
        end_line: u32,
        end_column: u32,
    ) -> Self {
        Self {
            file,
            line,
            column,
            end_line: Some(end_line),
            end_column: Some(end_column),
        }
    }

    /// Get the file name as a string
    pub fn file_name(&self) -> String {
        self.file
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "<unknown>".to_string())
    }

    /// Get the directory path
    pub fn directory(&self) -> PathBuf {
        self.file.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf()
    }

    /// Format as a human-readable string
    pub fn display(&self) -> String {
        if let (Some(end_line), Some(end_column)) = (self.end_line, self.end_column) {
            format!(
                "{}:{}:{}-{}:{}",
                self.file_name(),
                self.line,
                self.column,
                end_line,
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
            file: PathBuf::from("<unknown>"),
            line: 1,
            column: 1,
            end_line: None,
            end_column: None,
        }
    }
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

/// Source location manager for tracking locations throughout compilation
#[derive(Debug)]
pub struct SourceLocationManager {
    current_file: Option<PathBuf>,
    location_stack: Vec<SourceLocation>,
}

impl SourceLocationManager {
    /// Create a new source location manager
    pub fn new() -> Self {
        Self {
            current_file: None,
            location_stack: Vec::new(),
        }
    }

    /// Set the current file being processed
    #[instrument(skip(self))]
    pub fn set_current_file(&mut self, file: PathBuf) {
        debug!(file = ?file, "Setting current file");
        self.current_file = Some(file);
    }

    /// Push a location onto the stack
    #[instrument(skip(self))]
    pub fn push_location(&mut self, location: SourceLocation) {
        debug!(location = ?location, "Pushing location");
        self.location_stack.push(location);
    }

    /// Pop a location from the stack
    #[instrument(skip(self))]
    pub fn pop_location(&mut self) -> Option<SourceLocation> {
        let location = self.location_stack.pop();
        debug!(location = ?location, "Popping location");
        location
    }

    /// Get the current location
    pub fn current_location(&self) -> Option<&SourceLocation> {
        self.location_stack.last()
    }

    /// Create a new location in the current file
    pub fn create_location(&self, line: u32, column: u32) -> SourceLocation {
        let file = self.current_file.clone().unwrap_or_else(|| PathBuf::from("<unknown>"));
        SourceLocation::new(file, line, column)
    }

    /// Create a location range in the current file
    pub fn create_range(&self, line: u32, column: u32, end_line: u32, end_column: u32) -> SourceLocation {
        let file = self.current_file.clone().unwrap_or_else(|| PathBuf::from("<unknown>"));
        SourceLocation::new_range(file, line, column, end_line, end_column)
    }

    /// Get the current file path
    pub fn current_file(&self) -> Option<&PathBuf> {
        self.current_file.as_ref()
    }

    /// Get the location stack depth
    pub fn stack_depth(&self) -> usize {
        self.location_stack.len()
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_source_location_creation() {
        let file = PathBuf::from("test.csd");
        let loc = SourceLocation::new(file.clone(), 10, 5);
        
        assert_eq!(loc.file, file);
        assert_eq!(loc.line, 10);
        assert_eq!(loc.column, 5);
        assert!(loc.is_valid());
    }

    #[test]
    fn test_source_location_range() {
        let file = PathBuf::from("test.csd");
        let loc = SourceLocation::new_range(file.clone(), 10, 5, 12, 8);
        
        assert_eq!(loc.end_line, Some(12));
        assert_eq!(loc.end_column, Some(8));
    }

    #[test]
    fn test_source_location_display() {
        let file = PathBuf::from("test.csd");
        let loc = SourceLocation::new(file, 10, 5);
        
        assert_eq!(loc.display(), "test.csd:10:5");
    }

    #[test]
    fn test_location_manager() {
        let mut manager = SourceLocationManager::new();
        let file = PathBuf::from("test.csd");
        
        manager.set_current_file(file.clone());
        assert_eq!(manager.current_file(), Some(&file));
        
        let loc = manager.create_location(10, 5);
        manager.push_location(loc.clone());
        
        assert_eq!(manager.current_location(), Some(&loc));
        assert_eq!(manager.stack_depth(), 1);
        
        let popped = manager.pop_location();
        assert_eq!(popped, Some(loc));
        assert_eq!(manager.stack_depth(), 0);
    }

    #[test]
    fn test_invalid_location() {
        let loc = SourceLocation::new(PathBuf::from("test.csd"), 0, 5);
        assert!(!loc.is_valid());
        
        let loc = SourceLocation::new(PathBuf::from("test.csd"), 5, 0);
        assert!(!loc.is_valid());
    }
}
