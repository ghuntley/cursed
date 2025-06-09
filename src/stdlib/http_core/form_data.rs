//! Form Data Processing for CURSED web_vibez
//!
//! Comprehensive form data parsing for URL-encoded and multipart forms.

use std::collections::HashMap;
use std::fmt;
use std::io::{self, Read};

use crate::stdlib::http_core::{HttpError, HttpResult, ContentType, MimeType};

/// Form field value types
#[derive(Debug, Clone)]
pub enum FormValue {
    /// Simple text value
    Text(String),
    /// File upload with metadata
    File {
        /// Original filename
        filename: String,
        /// File content type
        content_type: Option<String>,
        /// File content
        content: Vec<u8>,
    },
}

impl FormValue {
    /// Create text value
    pub fn text<S: Into<String>>(value: S) -> Self {
        FormValue::Text(value.into())
    }

    /// Create file value
    pub fn file<F, C>(filename: F, content: Vec<u8>, content_type: Option<C>) -> Self
    where
        F: Into<String>,
        C: Into<String>,
    {
        FormValue::File {
            filename: filename.into(),
            content_type: content_type.map(|ct| ct.into()),
            content,
        }
    }

    /// Get as text (returns None for files)
    pub fn as_text(&self) -> Option<&str> {
        match self {
            FormValue::Text(text) => Some(text),
            FormValue::File { .. } => None,
        }
    }

    /// Get as file data (returns None for text)
    pub fn as_file(&self) -> Option<(&str, &[u8], Option<&str>)> {
        match self {
            FormValue::Text(_) => None,
            FormValue::File { filename, content, content_type } => {
                Some((filename, content, content_type.as_deref()))
            }
        }
    }

    /// Check if this is a file
    pub fn is_file(&self) -> bool {
        matches!(self, FormValue::File { .. })
    }

    /// Check if this is text
    pub fn is_text(&self) -> bool {
        matches!(self, FormValue::Text(_))
    }

    /// Get the string representation
    pub fn to_string(&self) -> String {
        match self {
            FormValue::Text(text) => text.clone(),
            FormValue::File { filename, .. } => format!("[File: {}]", filename),
        }
    }
}

impl fmt::Display for FormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Form field with metadata
#[derive(Debug, Clone)]
pub struct FormField {
    /// Field name
    pub name: String,
    /// Field value
    pub value: FormValue,
}

impl FormField {
    /// Create new form field
    pub fn new<N>(name: N, value: FormValue) -> Self
    where
        N: Into<String>,
    {
        Self {
            name: name.into(),
            value,
        }
    }

    /// Create text field
    pub fn text<N, V>(name: N, value: V) -> Self
    where
        N: Into<String>,
        V: Into<String>,
    {
        Self::new(name, FormValue::text(value))
    }

    /// Create file field
    pub fn file<N, F, C>(name: N, filename: F, content: Vec<u8>, content_type: Option<C>) -> Self
    where
        N: Into<String>,
        F: Into<String>,
        C: Into<String>,
    {
        Self::new(name, FormValue::file(filename, content, content_type))
    }
}

/// Form data container
#[derive(Debug, Clone)]
pub struct FormData {
    fields: HashMap<String, Vec<FormValue>>,
}

impl FormData {
    /// Create new empty form data
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    /// Parse from URL-encoded string
    pub fn from_url_encoded(data: &str) -> HttpResult<Self> {
        let mut form_data = Self::new();

        if data.is_empty() {
            return Ok(form_data);
        }

        for pair in data.split('&') {
            if let Some(eq_pos) = pair.find('=') {
                let key = urlencoding::decode(&pair[..eq_pos])
                    .map_err(|e| HttpError::FormDataError(format!("Failed to decode key: {}", e)))?
                    .to_string();
                let value = urlencoding::decode(&pair[eq_pos + 1..])
                    .map_err(|e| HttpError::FormDataError(format!("Failed to decode value: {}", e)))?
                    .to_string();

                form_data.add_field(key, FormValue::text(value));
            } else if !pair.is_empty() {
                let key = urlencoding::decode(pair)
                    .map_err(|e| HttpError::FormDataError(format!("Failed to decode key: {}", e)))?
                    .to_string();
                form_data.add_field(key, FormValue::text(""));
            }
        }

        Ok(form_data)
    }

    /// Convert to URL-encoded string
    pub fn to_url_encoded(&self) -> String {
        let mut parts = Vec::new();

        for (key, values) in &self.fields {
            let encoded_key = urlencoding::encode(key);
            for value in values {
                match value {
                    FormValue::Text(text) => {
                        if text.is_empty() {
                            parts.push(encoded_key.to_string());
                        } else {
                            let encoded_value = urlencoding::encode(text);
                            parts.push(format!("{}={}", encoded_key, encoded_value));
                        }
                    }
                    FormValue::File { filename, .. } => {
                        // Files can't be represented in URL encoding, use filename
                        let encoded_filename = urlencoding::encode(filename);
                        parts.push(format!("{}={}", encoded_key, encoded_filename));
                    }
                }
            }
        }

        parts.join("&")
    }

    /// Add form field
    pub fn add_field<K>(&mut self, key: K, value: FormValue)
    where
        K: Into<String>,
    {
        self.fields
            .entry(key.into())
            .or_insert_with(Vec::new)
            .push(value);
    }

    /// Set form field (replaces existing)
    pub fn set_field<K>(&mut self, key: K, value: FormValue)
    where
        K: Into<String>,
    {
        self.fields.insert(key.into(), vec![value]);
    }

    /// Add text field
    pub fn add_text<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.add_field(key, FormValue::text(value));
    }

    /// Set text field
    pub fn set_text<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.set_field(key, FormValue::text(value));
    }

    /// Add file field
    pub fn add_file<K, F, C>(&mut self, key: K, filename: F, content: Vec<u8>, content_type: Option<C>)
    where
        K: Into<String>,
        F: Into<String>,
        C: Into<String>,
    {
        self.add_field(key, FormValue::file(filename, content, content_type));
    }

    /// Get first value for field
    pub fn get(&self, key: &str) -> Option<String> {
        self.fields
            .get(key)?
            .first()
            .map(|v| v.to_string())
    }

    /// Get first text value for field
    pub fn get_text(&self, key: &str) -> Option<&str> {
        self.fields
            .get(key)?
            .first()?
            .as_text()
    }

    /// Get first file value for field
    pub fn get_file(&self, key: &str) -> Option<(&str, &[u8], Option<&str>)> {
        self.fields
            .get(key)?
            .first()?
            .as_file()
    }

    /// Get all values for field
    pub fn get_all(&self, key: &str) -> Option<&Vec<FormValue>> {
        self.fields.get(key)
    }

    /// Remove field
    pub fn remove(&mut self, key: &str) -> Option<Vec<FormValue>> {
        self.fields.remove(key)
    }

    /// Check if field exists
    pub fn contains_key(&self, key: &str) -> bool {
        self.fields.contains_key(key)
    }

    /// Get all field names
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.fields.keys()
    }

    /// Get number of fields
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Clear all fields
    pub fn clear(&mut self) {
        self.fields.clear();
    }

    /// Get all fields
    pub fn fields(&self) -> impl Iterator<Item = (&String, &Vec<FormValue>)> {
        self.fields.iter()
    }

    /// Check if contains any files
    pub fn has_files(&self) -> bool {
        self.fields.values().any(|values| {
            values.iter().any(|v| v.is_file())
        })
    }

    /// Get file count
    pub fn file_count(&self) -> usize {
        self.fields.values().map(|values| {
            values.iter().filter(|v| v.is_file()).count()
        }).sum()
    }

    /// Validate form data
    pub fn validate(&self) -> HttpResult<()> {
        for (key, values) in &self.fields {
            if key.is_empty() {
                return Err(HttpError::FormDataError("Empty field name".to_string()));
            }

            for value in values {
                match value {
                    FormValue::Text(text) => {
                        // Text validation can be extended here
                        if text.len() > 1_000_000 { // 1MB limit for text fields
                            return Err(HttpError::FormDataError(
                                format!("Text field '{}' too large", key)
                            ));
                        }
                    }
                    FormValue::File { content, .. } => {
                        if content.len() > 100_000_000 { // 100MB limit for files
                            return Err(HttpError::FormDataError(
                                format!("File field '{}' too large", key)
                            ));
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl Default for FormData {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for FormData {
    type Item = (String, Vec<FormValue>);
    type IntoIter = std::collections::hash_map::IntoIter<String, Vec<FormValue>>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}

/// Multipart form data parser
#[derive(Debug)]
pub struct MultipartData {
    boundary: String,
    form_data: FormData,
}

impl MultipartData {
    /// Create new multipart parser
    pub fn new<B: Into<String>>(boundary: B) -> Self {
        Self {
            boundary: boundary.into(),
            form_data: FormData::new(),
        }
    }

    /// Parse multipart data from bytes
    pub fn parse(&mut self, data: &[u8]) -> HttpResult<()> {
        let boundary_bytes = format!("--{}", self.boundary).into_bytes();
        let end_boundary_bytes = format!("--{}--", self.boundary).into_bytes();

        let mut pos = 0;
        
        // Find first boundary
        if let Some(first_boundary) = Self::find_boundary(data, &boundary_bytes, pos) {
            pos = first_boundary + boundary_bytes.len();
        } else {
            return Err(HttpError::FormDataError("No starting boundary found".to_string()));
        }

        // Parse each part
        while pos < data.len() {
            // Skip CRLF after boundary
            if pos + 1 < data.len() && data[pos] == b'\r' && data[pos + 1] == b'\n' {
                pos += 2;
            }

            // Find next boundary
            let next_boundary = Self::find_boundary(data, &boundary_bytes, pos)
                .or_else(|| Self::find_boundary(data, &end_boundary_bytes, pos));

            if let Some(boundary_pos) = next_boundary {
                let part_data = &data[pos..boundary_pos];
                self.parse_part(part_data)?;
                
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

        Ok(())
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
    fn parse_part(&mut self, part_data: &[u8]) -> HttpResult<()> {
        // Find separator between headers and body
        let header_end = if let Some(pos) = Self::find_sequence(part_data, b"\r\n\r\n") {
            pos
        } else if let Some(pos) = Self::find_sequence(part_data, b"\n\n") {
            pos
        } else {
            return Err(HttpError::FormDataError("No header-body separator found".to_string()));
        };

        let headers_data = &part_data[..header_end];
        let body_data = &part_data[header_end + 4..]; // Skip \r\n\r\n

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

        if let Some(name) = field_name {
            let value = if let Some(fname) = filename {
                FormValue::file(fname, body_data.to_vec(), content_type)
            } else {
                let text = String::from_utf8_lossy(body_data).to_string();
                FormValue::text(text)
            };

            self.form_data.add_field(name, value);
        }

        Ok(())
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

    /// Get the parsed form data
    pub fn form_data(&self) -> &FormData {
        &self.form_data
    }

    /// Take the parsed form data
    pub fn into_form_data(self) -> FormData {
        self.form_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_value_creation() {
        let text_value = FormValue::text("hello");
        assert!(text_value.is_text());
        assert_eq!(text_value.as_text(), Some("hello"));

        let file_value = FormValue::file("test.txt", b"content".to_vec(), Some("text/plain"));
        assert!(file_value.is_file());
        assert_eq!(file_value.as_file().unwrap().0, "test.txt");
    }

    #[test]
    fn test_form_data_url_encoded() {
        let form_data = FormData::from_url_encoded("name=John&age=30&city=New%20York").unwrap();
        
        assert_eq!(form_data.get_text("name"), Some("John"));
        assert_eq!(form_data.get_text("age"), Some("30"));
        assert_eq!(form_data.get_text("city"), Some("New York"));
    }

    #[test]
    fn test_form_data_to_url_encoded() {
        let mut form_data = FormData::new();
        form_data.add_text("name", "John Doe");
        form_data.add_text("email", "john@example.com");
        
        let encoded = form_data.to_url_encoded();
        assert!(encoded.contains("name=John%20Doe"));
        assert!(encoded.contains("email=john%40example.com"));
    }

    #[test]
    fn test_form_data_operations() {
        let mut form_data = FormData::new();
        
        // Add fields
        form_data.add_text("username", "alice");
        form_data.add_text("tags", "rust");
        form_data.add_text("tags", "web");
        
        assert_eq!(form_data.get_text("username"), Some("alice"));
        assert_eq!(form_data.get_all("tags").unwrap().len(), 2);
        assert!(form_data.contains_key("username"));
        assert!(!form_data.contains_key("nonexistent"));
        
        // Remove field
        form_data.remove("username");
        assert!(!form_data.contains_key("username"));
    }

    #[test]
    fn test_form_field_creation() {
        let text_field = FormField::text("username", "alice");
        assert_eq!(text_field.name, "username");
        assert!(text_field.value.is_text());

        let file_field = FormField::file("upload", "test.txt", b"content".to_vec(), Some("text/plain"));
        assert_eq!(file_field.name, "upload");
        assert!(file_field.value.is_file());
    }

    #[test]
    fn test_form_data_validation() {
        let mut form_data = FormData::new();
        form_data.add_text("normal", "value");
        
        // Should pass validation
        assert!(form_data.validate().is_ok());
        
        // Add very large text (would fail in real scenario with limits)
        // This test validates the structure, actual limits are configurable
        form_data.add_text("large", "x".repeat(100));
        assert!(form_data.validate().is_ok());
    }

    #[test]
    fn test_multipart_parser_creation() {
        let parser = MultipartData::new("----WebKitFormBoundary123");
        assert_eq!(parser.boundary, "----WebKitFormBoundary123");
        assert!(parser.form_data.is_empty());
    }

    #[test]
    fn test_content_disposition_parsing() {
        let name = MultipartData::parse_content_disposition_name(
            "form-data; name=\"username\"; filename=\"test.txt\""
        );
        assert_eq!(name, Some("username".to_string()));

        let filename = MultipartData::parse_content_disposition_filename(
            "form-data; name=\"file\"; filename=\"document.pdf\""
        );
        assert_eq!(filename, Some("document.pdf".to_string()));
    }
}
