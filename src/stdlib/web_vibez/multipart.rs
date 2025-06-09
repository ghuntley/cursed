/// File uploads and multipart form processing utilities
use std::collections::HashMap;

/// File upload structure
#[derive(Debug, Clone)]
pub struct FileUpload {
    pub name: String,
    pub filename: String,
    pub content_type: String,
    pub content: Vec<u8>,
    pub size: usize,
}

/// Multipart form processor
pub struct MultipartProcessor {
    boundary: String,
    max_file_size: usize,
    max_total_size: usize,
    allowed_types: Vec<String>,
}

impl MultipartProcessor {
    pub fn new() -> Self {
        Self {
            boundary: String::new(),
            max_file_size: 10 * 1024 * 1024, // 10MB
            max_total_size: 50 * 1024 * 1024, // 50MB
            allowed_types: vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
                "image/gif".to_string(),
                "text/plain".to_string(),
                "application/pdf".to_string(),
            ],
        }
    }

    pub fn with_boundary(mut self, boundary: String) -> Self {
        self.boundary = boundary;
        self
    }

    pub fn with_max_file_size(mut self, size: usize) -> Self {
        self.max_file_size = size;
        self
    }

    pub fn parse(&self, data: &[u8]) -> Result<MultipartData, MultipartError> {
        // Placeholder implementation
        Ok(MultipartData {
            fields: HashMap::new(),
            files: Vec::new(),
        })
    }
}

impl Default for MultipartProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct MultipartData {
    pub fields: HashMap<String, String>,
    pub files: Vec<FileUpload>,
}

#[derive(Debug)]
pub enum MultipartError {
    InvalidBoundary,
    FileTooLarge,
    UnsupportedType,
    ParseError(String),
}

impl std::fmt::Display for MultipartError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MultipartError::InvalidBoundary => write!(f, "Invalid multipart boundary"),
            MultipartError::FileTooLarge => write!(f, "File too large"),
            MultipartError::UnsupportedType => write!(f, "Unsupported file type"),
            MultipartError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for MultipartError {}
