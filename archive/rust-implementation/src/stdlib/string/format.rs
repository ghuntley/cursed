//! String processing functionality for format

use crate::error::CursedError;

/// Result type for string operations
pub type StringResult<T> = Result<T, CursedError>;

/// String processing utilities
pub struct StringFormatProcessor {
    case_sensitive: bool,
}

impl StringFormatProcessor {
    /// Create a new string processor
    pub fn new() -> Self {
        Self {
            case_sensitive: true,
        }
    }
    
    /// Set case sensitivity
    pub fn case_sensitive(mut self, sensitive: bool) -> Self {
        self.case_sensitive = sensitive;
        self
    }
    
    /// Process a string
    pub fn process(&self, input: &str) -> StringResult<String> {
        if self.case_sensitive {
            Ok(input.to_string())
        } else {
            Ok(input.to_lowercase())
        }
    }
    
    /// Get string length
    pub fn length(&self, input: &str) -> usize {
        input.len()
    }
    
    /// Check if string is empty
    pub fn is_empty(&self, input: &str) -> bool {
        input.is_empty()
    }
}

impl Default for StringFormatProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize string processing
pub fn init_format() -> StringResult<()> {
    let processor = StringFormatProcessor::new();
    let test_result = processor.process("test")?;
    if test_result.is_empty() {
        return Err(CursedError::runtime_error("String processing test failed"));
    }
    println!("📝 String processing (format) initialized");
    Ok(())
}

/// Test string functionality
pub fn test_format() -> StringResult<()> {
    let processor = StringFormatProcessor::new();
    let result = processor.process("Hello, CURSED!")?;
    if result != "Hello, CURSED!" {
        return Err(CursedError::runtime_error("String test failed"));
    }
    Ok(())
}
