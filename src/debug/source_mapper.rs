/// Source mapping utilities for enhanced debugging
///
/// Provides source mapping capabilities for precise location tracking
/// between generated and original source code.

use crate::error::Error as CursedError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::{Arc, RwLock};

/// Source mapper for handling source map operations
pub struct SourceMapper {
    /// Source content cache
    source_cache: Arc<RwLock<HashMap<PathBuf, String>>>,
    /// Line mapping cache
    line_cache: Arc<RwLock<HashMap<PathBuf, Vec<String>>>>,
}

impl SourceMapper {
    /// Create new source mapper
    pub fn new() -> Self {
        SourceMapper {
            source_cache: Arc::new(RwLock::new(HashMap::new())),
            line_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Load source file into cache
    pub fn load_source_file(&self, file_path: &Path) -> Result<(), CursedError> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| CursedError::Io(e.into()))?;

        let lines: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();

        {
            let mut source_cache = self.source_cache.write()
                .map_err(|_| CursedError::Runtime("Failed to acquire source cache lock".to_string()))?;
            source_cache.insert(file_path.to_path_buf(), content);
        }

        {
            let mut line_cache = self.line_cache.write()
                .map_err(|_| CursedError::Runtime("Failed to acquire line cache lock".to_string()))?;
            line_cache.insert(file_path.to_path_buf(), lines);
        }

        Ok(())
    }

    /// Get source snippet around a line
    pub fn get_source_snippet(
        &self,
        file_path: &Path,
        line: u32,
        context_lines: u32,
    ) -> Result<String, CursedError> {
        // Ensure file is loaded
        if !self.is_file_cached(file_path) {
            self.load_source_file(file_path)?;
        }

        let line_cache = self.line_cache.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire line cache lock".to_string()))?;

        if let Some(lines) = line_cache.get(file_path) {
            let target_line = line.saturating_sub(1) as usize;
            let start_line = target_line.saturating_sub(context_lines as usize);
            let end_line = std::cmp::min(target_line + context_lines as usize + 1, lines.len());

            if start_line >= lines.len() {
                return Err(CursedError::Runtime("Line number out of range".to_string()));
            }

            let mut snippet = String::new();
            for (i, line_content) in lines[start_line..end_line].iter().enumerate() {
                let line_number = start_line + i + 1;
                let marker = if line_number == line as usize { ">" } else { " " };
                snippet.push_str(&format!("{} {:4} | {}\n", marker, line_number, line_content));
            }

            Ok(snippet)
        } else {
            Err(CursedError::Runtime("Source file not found in cache".to_string()))
        }
    }

    /// Get specific line content
    pub fn get_line(&self, file_path: &Path, line: u32) -> Result<String, CursedError> {
        if !self.is_file_cached(file_path) {
            self.load_source_file(file_path)?;
        }

        let line_cache = self.line_cache.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire line cache lock".to_string()))?;

        if let Some(lines) = line_cache.get(file_path) {
            let line_index = (line.saturating_sub(1)) as usize;
            if line_index < lines.len() {
                Ok(lines[line_index].clone())
            } else {
                Err(CursedError::Runtime("Line number out of range".to_string()))
            }
        } else {
            Err(CursedError::Runtime("Source file not found in cache".to_string()))
        }
    }

    /// Check if file is cached
    pub fn is_file_cached(&self, file_path: &Path) -> bool {
        if let Ok(cache) = self.source_cache.read() {
            cache.contains_key(file_path)
        } else {
            false
        }
    }

    /// Get cached files
    pub fn get_cached_files(&self) -> Result<Vec<PathBuf>, CursedError> {
        let cache = self.source_cache.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire source cache lock".to_string()))?;

        Ok(cache.keys().cloned().collect())
    }

    /// Clear cache
    pub fn clear_cache(&self) -> Result<(), CursedError> {
        {
            let mut source_cache = self.source_cache.write()
                .map_err(|_| CursedError::Runtime("Failed to acquire source cache lock".to_string()))?;
            source_cache.clear();
        }

        {
            let mut line_cache = self.line_cache.write()
                .map_err(|_| CursedError::Runtime("Failed to acquire line cache lock".to_string()))?;
            line_cache.clear();
        }

        Ok(())
    }

    /// Get file line count
    pub fn get_line_count(&self, file_path: &Path) -> Result<usize, CursedError> {
        if !self.is_file_cached(file_path) {
            self.load_source_file(file_path)?;
        }

        let line_cache = self.line_cache.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire line cache lock".to_string()))?;

        if let Some(lines) = line_cache.get(file_path) {
            Ok(lines.len())
        } else {
            Err(CursedError::Runtime("Source file not found in cache".to_string()))
        }
    }

    /// Find lines containing pattern
    pub fn find_pattern(
        &self,
        file_path: &Path,
        pattern: &str,
        case_sensitive: bool,
    ) -> Result<Vec<(u32, String)>, CursedError> {
        if !self.is_file_cached(file_path) {
            self.load_source_file(file_path)?;
        }

        let line_cache = self.line_cache.read()
            .map_err(|_| CursedError::Runtime("Failed to acquire line cache lock".to_string()))?;

        if let Some(lines) = line_cache.get(file_path) {
            let mut matches = Vec::new();

            for (i, line) in lines.iter().enumerate() {
                let matches_pattern = if case_sensitive {
                    line.contains(pattern)
                } else {
                    line.to_lowercase().contains(&pattern.to_lowercase())
                };

                if matches_pattern {
                    matches.push((i as u32 + 1, line.clone()));
                }
            }

            Ok(matches)
        } else {
            Err(CursedError::Runtime("Source file not found in cache".to_string()))
        }
    }

    /// Get source excerpt with highlighted pattern
    pub fn get_highlighted_excerpt(
        &self,
        file_path: &Path,
        line: u32,
        column: u32,
        length: u32,
        context_lines: u32,
    ) -> Result<String, CursedError> {
        let snippet = self.get_source_snippet(file_path, line, context_lines)?;
        
        // TODO: Add highlighting logic for the specific column and length
        // For now, just return the snippet with a marker
        let highlighted = snippet.replace(
            &format!(" {:4} |", line),
            &format!(">{:4} |", line),
        );

        Ok(highlighted)
    }
}

/// Source location with enhanced information
#[derive(Debug, Clone)]
pub struct EnhancedSourceLocation {
    /// File path
    pub file_path: PathBuf,
    /// Line number (1-based)
    pub line: u32,
    /// Column number (1-based)
    pub column: u32,
    /// Length of the location
    pub length: Option<u32>,
    /// Source excerpt
    pub excerpt: Option<String>,
}

impl EnhancedSourceLocation {
    /// Create new enhanced source location
    pub fn new(file_path: PathBuf, line: u32, column: u32) -> Self {
        EnhancedSourceLocation {
            file_path,
            line,
            column,
            length: None,
            excerpt: None,
        }
    }

    /// Add length information
    pub fn with_length(mut self, length: u32) -> Self {
        self.length = Some(length);
        self
    }

    /// Add source excerpt
    pub fn with_excerpt(mut self, excerpt: String) -> Self {
        self.excerpt = Some(excerpt);
        self
    }

    /// Generate source location string
    pub fn to_string(&self) -> String {
        if let Some(file_name) = self.file_path.file_name() {
            format!("{}:{}:{}", 
                file_name.to_string_lossy(),
                self.line,
                self.column
            )
        } else {
            format!("{}:{}", self.line, self.column)
        }
    }

    /// Get full path string
    pub fn full_path_string(&self) -> String {
        format!("{}:{}:{}", 
            self.file_path.display(),
            self.line,
            self.column
        )
    }
}

/// Source mapping utilities
pub struct SourceMappingUtils;

impl SourceMappingUtils {
    /// Calculate line and column from byte offset
    pub fn offset_to_line_column(source: &str, offset: usize) -> (u32, u32) {
        let mut line = 1;
        let mut column = 1;
        
        for (i, ch) in source.char_indices() {
            if i >= offset {
                break;
            }
            
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        
        (line, column)
    }

    /// Calculate byte offset from line and column
    pub fn line_column_to_offset(source: &str, target_line: u32, target_column: u32) -> Option<usize> {
        let mut line = 1;
        let mut column = 1;
        
        for (i, ch) in source.char_indices() {
            if line == target_line && column == target_column {
                return Some(i);
            }
            
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        
        // Check if we're at the end of the target line
        if line == target_line && column == target_column {
            Some(source.len())
        } else {
            None
        }
    }

    /// Validate line and column against source
    pub fn validate_location(source: &str, line: u32, column: u32) -> bool {
        Self::line_column_to_offset(source, line, column).is_some()
    }

    /// Get word boundaries at location
    pub fn get_word_boundaries(source: &str, line: u32, column: u32) -> Option<(u32, u32)> {
        if let Some(offset) = Self::line_column_to_offset(source, line, column) {
            let chars: Vec<char> = source.chars().collect();
            
            if offset >= chars.len() {
                return None;
            }
            
            // Find word start
            let mut start = offset;
            while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
                start -= 1;
            }
            
            // Find word end
            let mut end = offset;
            while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
                end += 1;
            }
            
            let (start_line, start_column) = Self::offset_to_line_column(source, start);
            let (end_line, end_column) = Self::offset_to_line_column(source, end);
            
            // For single-line words
            if start_line == end_line {
                Some((start_column, end_column))
            } else {
                Some((start_column, column))
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_source_mapper_creation() {
        let mapper = SourceMapper::new();
        assert_eq!(mapper.get_cached_files().unwrap().len(), 0);
    }

    #[test]
    fn test_source_file_loading() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.csd");
        
        fs::write(&file_path, "slay test_func() {\n    facts result = true;\n    periodt result;\n}").unwrap();
        
        let mapper = SourceMapper::new();
        let result = mapper.load_source_file(&file_path);
        assert!(result.is_ok());
        
        assert!(mapper.is_file_cached(&file_path));
        assert_eq!(mapper.get_line_count(&file_path).unwrap(), 4);
    }

    #[test]
    fn test_source_snippet_extraction() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.csd");
        
        let source = "line 1\nline 2\nline 3\nline 4\nline 5";
        fs::write(&file_path, source).unwrap();
        
        let mapper = SourceMapper::new();
        let _ = mapper.load_source_file(&file_path);
        
        let snippet = mapper.get_source_snippet(&file_path, 3, 1);
        assert!(snippet.is_ok());
        
        let snippet = snippet.unwrap();
        assert!(snippet.contains("line 2"));
        assert!(snippet.contains("> 3"));
        assert!(snippet.contains("line 4"));
    }

    #[test]
    fn test_pattern_finding() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.csd");
        
        let source = "slay func1() {}\nsus x = 42;\nfacts test = true;\nslay func2() {}";
        fs::write(&file_path, source).unwrap();
        
        let mapper = SourceMapper::new();
        let _ = mapper.load_source_file(&file_path);
        
        let matches = mapper.find_pattern(&file_path, "slay", true);
        assert!(matches.is_ok());
        
        let matches = matches.unwrap();
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].0, 1);
        assert_eq!(matches[1].0, 4);
    }

    #[test]
    fn test_enhanced_source_location() {
        let location = EnhancedSourceLocation::new(PathBuf::from("test.csd"), 42, 10)
            .with_length(5)
            .with_excerpt("test code".to_string());
        
        assert_eq!(location.line, 42);
        assert_eq!(location.column, 10);
        assert_eq!(location.length, Some(5));
        assert!(location.excerpt.is_some());
        
        let location_str = location.to_string();
        assert!(location_str.contains("test.csd:42:10"));
    }

    #[test]
    fn test_source_mapping_utils() {
        let source = "line 1\nline 2\nline 3";
        
        // Test offset to line/column
        let (line, column) = SourceMappingUtils::offset_to_line_column(source, 8);
        assert_eq!(line, 2);
        assert_eq!(column, 2);
        
        // Test line/column to offset
        let offset = SourceMappingUtils::line_column_to_offset(source, 2, 2);
        assert_eq!(offset, Some(8));
        
        // Test validation
        assert!(SourceMappingUtils::validate_location(source, 2, 2));
        assert!(!SourceMappingUtils::validate_location(source, 10, 1));
    }

    #[test]
    fn test_word_boundaries() {
        let source = "slay test_function() {\n    sus variable = 42;\n}";
        
        // Test word boundaries for "test_function"
        let boundaries = SourceMappingUtils::get_word_boundaries(source, 1, 7);
        assert!(boundaries.is_some());
        
        let (start, end) = boundaries.unwrap();
        assert!(start <= 7);
        assert!(end > 7);
    }

    #[test]
    fn test_cache_management() {
        let mapper = SourceMapper::new();
        
        let dir = tempdir().unwrap();
        let file1 = dir.path().join("file1.csd");
        let file2 = dir.path().join("file2.csd");
        
        fs::write(&file1, "content 1").unwrap();
        fs::write(&file2, "content 2").unwrap();
        
        let _ = mapper.load_source_file(&file1);
        let _ = mapper.load_source_file(&file2);
        
        assert_eq!(mapper.get_cached_files().unwrap().len(), 2);
        
        let clear_result = mapper.clear_cache();
        assert!(clear_result.is_ok());
        assert_eq!(mapper.get_cached_files().unwrap().len(), 0);
    }
}
