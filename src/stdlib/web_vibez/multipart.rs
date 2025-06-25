use crate::error::CursedError;
/// File uploads and multipart form processing utilities
use std::collections::HashMap;

/// Internal enum for parsed multipart parts
#[derive(Debug)]
enum ParsedPart {
    Field { name: String, value: String },
    File(FileUpload),
}

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

    pub fn parse(&self, data: &[u8]) -> crate::error::Result<()> {
        if self.boundary.is_empty() {
            return Err(MultipartError::InvalidBoundary);
        }

        let boundary_bytes = format!("--{}", self.boundary).into_bytes();
        let end_boundary_bytes = format!("--{}--", self.boundary).into_bytes();
        
        let mut fields = HashMap::new();
        let mut files = Vec::new();
        let mut total_size = 0;
        
        let mut pos = 0;
        
        // Find first boundary
        if let Some(first_boundary) = Self::find_boundary(data, &boundary_bytes, pos) {
            pos = first_boundary + boundary_bytes.len();
        } else {
            return Err(MultipartError::ParseError("No starting boundary found".to_string()));
        }

        // Parse each part
        while pos < data.len() {
            // Skip CRLF after boundary
            if pos + 1 < data.len() && data[pos] == b'\r' && data[pos + 1] == b'\n' {
                pos += 2;
            } else if pos < data.len() && data[pos] == b'\n' {
                pos += 1;
            }

            // Find next boundary
            let next_boundary = Self::find_boundary(data, &boundary_bytes, pos)
                .or_else(|| Self::find_boundary(data, &end_boundary_bytes, pos));

            if let Some(boundary_pos) = next_boundary {
                let part_data = &data[pos..boundary_pos];
                total_size += part_data.len();
                
                if total_size > self.max_total_size {
                    return Err(MultipartError::FileTooLarge);
                }

                // Parse this part
                let parsed_part = self.parse_part(part_data)?;
                match parsed_part {
                    ParsedPart::Field { name, value } => {
                        fields.insert(name, value);
                    }
                    ParsedPart::File(file_upload) => {
                        if file_upload.size > self.max_file_size {
                            return Err(MultipartError::FileTooLarge);
                        }
                        
                        if !self.is_allowed_type(&file_upload.content_type) {
                            return Err(MultipartError::UnsupportedType);
                        }
                        
                        files.push(file_upload);
                    }
                }
                
                // Check if this is the end boundary
                if boundary_pos + end_boundary_bytes.len() <= data.len() &&
                   &data[boundary_pos..boundary_pos + end_boundary_bytes.len()] == end_boundary_bytes {
                    break;
                }

                pos = boundary_pos + boundary_bytes.len();
            } else {
                break;
            }
        }

        Ok(MultipartData { fields, files })
    }

    /// Find boundary in data
    fn find_boundary(data: &[u8], boundary: &[u8], start: usize) -> Option<usize> {
        if start >= data.len() || boundary.is_empty() {
            return None;
        }

        data[start..]
            .windows(boundary.len())
            .position(|window| window == boundary)
            .map(|pos| start + pos)
    }

    /// Parse individual multipart part
    fn parse_part(&self, part_data: &[u8]) -> crate::error::Result<()> {
        // Find separator between headers and body
        let header_end = if let Some(pos) = Self::find_sequence(part_data, b"\r\n\r\n") {
            pos
        } else if let Some(pos) = Self::find_sequence(part_data, b"\n\n") {
            pos
        } else {
            return Err(MultipartError::ParseError("No header-body separator found".to_string()));
        };

        let headers_data = &part_data[..header_end];
        let body_start = if part_data.len() > header_end + 4 && &part_data[header_end..header_end + 4] == b"\r\n\r\n" {
            header_end + 4
        } else {
            header_end + 2
        };
        let body_data = &part_data[body_start..];

        // Parse headers
        let mut field_name = None;
        let mut filename = None;
        let mut content_type = None;

        for line in headers_data.split(|&b| b == b'\n') {
            let line = String::from_utf8_lossy(line).trim().to_string();
            if line.is_empty() {
                continue;
            }

            if let Some(colon_pos) = line.find(':') {
                let header_name = line[..colon_pos].trim().to_lowercase();
                let header_value = line[colon_pos + 1..].trim();

                if header_name == "content-disposition" {
                    field_name = Self::parse_content_disposition_name(header_value);
                    filename = Self::parse_content_disposition_filename(header_value);
                } else if header_name == "content-type" {
                    content_type = Some(header_value.to_string());
                }
            }
        }

        let name = field_name.ok_or_else(|| {
            MultipartError::ParseError("Missing field name in Content-Disposition".to_string())
        })?;

        if let Some(fname) = filename {
            // This is a file upload
            let file_upload = FileUpload {
                name: name.clone(),
                filename: fname,
                content_type: content_type.unwrap_or_else(|| "application/octet-stream".to_string()),
                content: body_data.to_vec(),
                size: body_data.len(),
            };
            Ok(ParsedPart::File(file_upload))
        } else {
            // This is a regular form field
            let value = String::from_utf8_lossy(body_data).to_string();
            Ok(ParsedPart::Field { name, value })
        }
    }

    /// Find byte sequence in data
    fn find_sequence(data: &[u8], sequence: &[u8]) -> Option<usize> {
        data.windows(sequence.len())
            .position(|window| window == sequence)
    }

    /// Parse field name from Content-Disposition header
    fn parse_content_disposition_name(value: &str) -> Option<String> {
        Self::parse_content_disposition_param(value, "name")
    }

    /// Parse filename from Content-Disposition header
    fn parse_content_disposition_filename(value: &str) -> Option<String> {
        Self::parse_content_disposition_param(value, "filename")
    }

    /// Parse parameter from Content-Disposition header
    fn parse_content_disposition_param(value: &str, param_name: &str) -> Option<String> {
        let param_pattern = format!("{}=", param_name);
        
        if let Some(param_start) = value.find(&param_pattern) {
            let value_start = param_start + param_pattern.len();
            let remaining = &value[value_start..];
            
            // Handle quoted values
            if remaining.starts_with('"') {
                if let Some(quote_end) = remaining[1..].find('"') {
                    return Some(remaining[1..quote_end + 1].to_string());
                }
            } else {
                // Unquoted value - take until semicolon or end
                let end_pos = remaining.find(';').unwrap_or(remaining.len());
                return Some(remaining[..end_pos].trim().to_string());
            }
        }

        None
    }

    /// Check if content type is allowed
    fn is_allowed_type(&self, content_type: &str) -> bool {
        if self.allowed_types.is_empty() {
            return true; // No restrictions
        }
        
        let ct_lower = content_type.to_lowercase();
        self.allowed_types.iter().any(|allowed| {
            ct_lower.starts_with(&allowed.to_lowercase())
        })
    }

    /// Add allowed content type
    pub fn add_allowed_type(&mut self, content_type: String) {
        self.allowed_types.push(content_type);
    }

    /// Set maximum total size
    pub fn with_max_total_size(mut self, size: usize) -> Self {
        self.max_total_size = size;
        self
    }

    /// Clear allowed types (allow all)
    pub fn allow_all_types(mut self) -> Self {
        self.allowed_types.clear();
        self
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

impl MultipartData {
    /// Create new empty multipart data
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            files: Vec::new(),
        }
    }

    /// Get field value
    pub fn get_field(&self, name: &str) -> Option<&String> {
        self.fields.get(name)
    }

    /// Get file by field name
    pub fn get_file(&self, name: &str) -> Option<&FileUpload> {
        self.files.iter().find(|f| f.name == name)
    }

    /// Get all files for a field name
    pub fn get_files(&self, name: &str) -> Vec<&FileUpload> {
        self.files.iter().filter(|f| f.name == name).collect()
    }

    /// Get all files
    pub fn files(&self) -> &Vec<FileUpload> {
        &self.files
    }

    /// Get all fields
    pub fn fields(&self) -> &HashMap<String, String> {
        &self.fields
    }

    /// Check if has files
    pub fn has_files(&self) -> bool {
        !self.files.is_empty()
    }

    /// Get total file count
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Get total size of all files
    pub fn total_file_size(&self) -> usize {
        self.files.iter().map(|f| f.size).sum()
    }
}

impl Default for MultipartData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum MultipartError {
    InvalidBoundary,
    FileTooLarge,
    UnsupportedType,
    ParseError(String),
    TotalSizeTooLarge,
    MissingContentDisposition,
    InvalidEncoding,
}

// impl std::fmt::Display for MultipartError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             MultipartError::InvalidBoundary => write!(f, "Invalid multipart boundary"),
//             MultipartError::FileTooLarge => write!(f, "File too large"),
//             MultipartError::UnsupportedType => write!(f, "Unsupported file type"),
//             MultipartError::ParseError(msg) => write!(f, "Parse error: {}", msg),
//             MultipartError::TotalSizeTooLarge => write!(f, "Total multipart data size too large"),
//             MultipartError::MissingContentDisposition => write!(f, "Missing Content-Disposition header"),
//             MultipartError::InvalidEncoding => write!(f, "Invalid character encoding"),
//         }
//     }
// }

// impl std::error::CursedError for MultipartError {}
// 
/// Individual field in a multipart form
#[derive(Debug, Clone)]
pub struct MultipartField {
    /// Field name
    pub name: String,
    /// Field value (text or binary data)
    pub value: Vec<u8>,
    /// Content type (if specified)
    pub content_type: Option<String>,
    /// Filename (for file uploads)
    pub filename: Option<String>,
    /// Additional headers
    pub headers: std::collections::HashMap<String, String>,
}

impl MultipartField {
    /// Create a new multipart field
    pub fn new(name: String, value: Vec<u8>) -> Self {
        Self {
            name,
            value,
            content_type: None,
            filename: None,
            headers: std::collections::HashMap::new(),
        }
    }

    /// Create a text field
    pub fn text(name: String, value: String) -> Self {
        Self {
            name,
            value: value.into_bytes(),
            content_type: Some("text/plain".to_string()),
            filename: None,
            headers: std::collections::HashMap::new(),
        }
    }

    /// Create a file field
    pub fn file(name: String, filename: String, content_type: String, data: Vec<u8>) -> Self {
        Self {
            name,
            value: data,
            content_type: Some(content_type),
            filename: Some(filename),
            headers: std::collections::HashMap::new(),
        }
    }

    /// Get field value as string (if valid UTF-8)
    pub fn text_value(&self) -> Option<String> {
        String::from_utf8(self.value.clone()).ok()
    }

    /// Check if this is a file field
    pub fn is_file(&self) -> bool {
        self.filename.is_some()
    }

    /// Get file size in bytes
    pub fn size(&self) -> usize {
        self.value.len()
    }
}
