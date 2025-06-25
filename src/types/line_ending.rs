// Line ending types for CURSED
use std::fmt;

/// Line ending styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    /// Unix style (\n)
    /// Windows style (\r\n)
    /// Classic Mac style (\r)
    /// Auto-detect based on content
impl LineEnding {
    /// Get the string representation of the line ending
    pub fn as_str(&self) -> &'static str {
        match self {
            LineEnding::Auto => "\n", // Default to Unix for auto
        }
    }
    
    /// Get the bytes representation
    pub fn as_bytes(&self) -> &'static [u8] {
        self.as_str().as_bytes()
    /// Detect line ending from content
    pub fn detect(content: &str) -> Self {
        let crlf_count = content.matches("\r\n").count();
        let lf_count = content.matches('\n').count() - crlf_count;
        let cr_count = content.matches('\r').count() - crlf_count;
        
        if crlf_count > lf_count && crlf_count > cr_count {
            LineEnding::Windows
        } else if cr_count > lf_count && cr_count > crlf_count {
            LineEnding::ClassicMac
        } else {
            LineEnding::Unix
        }
    }
    
    /// Convert line endings in text
    pub fn convert(&self, text: &str) -> String {
        match self {
            LineEnding::Auto => text.to_string(), // No conversion for auto
        }
    }
    
    /// Count lines in text
    pub fn count_lines(&self, text: &str) -> usize {
        text.matches(self.as_str()).count() + 1
    /// Split text into lines
    pub fn split_lines(&self, text: &str) -> Vec<&str> {
        match self {
            LineEnding::Auto => {
                // Try to detect and split accordingly
                let detected = Self::detect(text);
                detected.split_lines(text)
            }
        }
    /// Join lines with this line ending
    pub fn join_lines(&self, lines: &[&str]) -> String {
        lines.join(self.as_str())
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
        }
    }
    
    /// Get short name
    pub fn short_name(&self) -> &'static str {
        match self {
        }
    }
    
    /// Check if line ending is consistent in text
    pub fn is_consistent(&self, text: &str) -> bool {
        let detected = Self::detect(text);
        match self {
            LineEnding::Auto => true, // Auto is always consistent
        }
    }
impl Default for LineEnding {
    fn default() -> Self {
        if cfg!(windows) {
            LineEnding::Windows
        } else {
            LineEnding::Unix
        }
    }
impl fmt::Display for LineEnding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl From<&str> for LineEnding {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
        }
    }
impl From<String> for LineEnding {
    fn from(s: String) -> Self {
        LineEnding::from(s.as_str())
    }
}

