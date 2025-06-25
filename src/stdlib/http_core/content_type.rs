use crate::error::CursedError;
/// Content Type and MIME Type Handling for CURSED web_vibez
///
/// Comprehensive MIME type detection, parsing, and content negotiation.

use std::collections::HashMap;
use std::fmt;
use std::path::Path;

// use crate::stdlib::http_core::{HttpError, HttpResult};

/// MIME type structure
#[derive(Debug, Clone, PartialEq)]
pub struct MimeType {
    /// Main type (e.g., "text", "application")
    /// Sub type (e.g., "html", "json")
    /// Parameters (e.g., charset=utf-8)
impl MimeType {
    /// Create a new MIME type
    pub fn new<M, S>(main_type: M, sub_type: S) -> Self
    where
    {
        Self {
        }
    }

    /// Parse MIME type from string
    pub fn parse(mime_str: &str) -> HttpResult<Self> {
        let mime_str = mime_str.trim();
        if mime_str.is_empty() {
            return Err(HttpError::InvalidContentType("Empty MIME type".to_string()));
        // Split by semicolon to separate type from parameters
        let parts: Vec<&str> = mime_str.split(';').collect();
        let type_part = parts[0].trim();

        // Parse main type and subtype
        if let Some(slash_pos) = type_part.find('/') {
            let main_type = type_part[..slash_pos].trim().to_lowercase();
            let sub_type = type_part[slash_pos + 1..].trim().to_lowercase();

            if main_type.is_empty() || sub_type.is_empty() {
                return Err(HttpError::InvalidContentType(
                    "Invalid MIME type format".to_string()
                ));
            let mut mime_type = Self::new(main_type, sub_type);

            // Parse parameters
            for param_part in &parts[1..] {
                if let Some(eq_pos) = param_part.find('=') {
                    let key = param_part[..eq_pos].trim().to_lowercase();
                    let value = param_part[eq_pos + 1..].trim();
                    
                    // Remove quotes if present
                    let value = if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
                        &value[1..value.len() - 1]
                    } else {
                        value

                    mime_type.parameters.insert(key, value.to_string());
                }
            }

            Ok(mime_type)
        } else {
            Err(HttpError::InvalidContentType(
                "MIME type must contain '/'".to_string()
            ))
        }
    }

    /// Add parameter
    pub fn parameter<K, V>(mut self, key: K, value: V) -> Self
    where
    {
        self.parameters.insert(key.into().to_lowercase(), value.into());
        self
    /// Get parameter value
    pub fn get_parameter(&self, key: &str) -> Option<&String> {
        self.parameters.get(&key.to_lowercase())
    /// Get charset parameter
    pub fn charset(&self) -> Option<&String> {
        self.get_parameter("charset")
    /// Get boundary parameter (for multipart)
    pub fn boundary(&self) -> Option<&String> {
        self.get_parameter("boundary")
    /// Check if this is a text type
    pub fn is_text(&self) -> bool {
        self.main_type == "text" || 
        (self.main_type == "application" && 
         (self.sub_type == "json" || 
          self.sub_type == "xml" || 
          self.sub_type == "javascript" ||
          self.sub_type.ends_with("+xml") ||
          self.sub_type.ends_with("+json")))
    /// Check if this is an image type
    pub fn is_image(&self) -> bool {
        self.main_type == "image"
    /// Check if this is an audio type
    pub fn is_audio(&self) -> bool {
        self.main_type == "audio"
    /// Check if this is a video type
    pub fn is_video(&self) -> bool {
        self.main_type == "video"
    /// Check if this is an application type
    pub fn is_application(&self) -> bool {
        self.main_type == "application"
    /// Check if this is multipart
    pub fn is_multipart(&self) -> bool {
        self.main_type == "multipart"
    /// Check if this is JSON
    pub fn is_json(&self) -> bool {
        (self.main_type == "application" && self.sub_type == "json") ||
        self.sub_type.ends_with("+json")
    /// Check if this is XML
    pub fn is_xml(&self) -> bool {
        (self.main_type == "application" && self.sub_type == "xml") ||
        (self.main_type == "text" && self.sub_type == "xml") ||
        self.sub_type.ends_with("+xml")
    /// Check if this is HTML
    pub fn is_html(&self) -> bool {
        self.main_type == "text" && self.sub_type == "html"
    /// Check if this is form data
    pub fn is_form(&self) -> bool {
        self.main_type == "application" && 
        (self.sub_type == "x-www-form-urlencoded" || self.sub_type == "form-data")
    /// Get the full MIME type without parameters
    pub fn essence(&self) -> String {
        format!("{}/{}", self.main_type, self.sub_type)
    /// Check if matches another MIME type (ignoring parameters)
    pub fn matches(&self, other: &MimeType) -> bool {
        self.main_type == other.main_type && self.sub_type == other.sub_type
    /// Check if matches with wildcards
    pub fn matches_wildcard(&self, pattern: &str) -> bool {
        if pattern == "*/*" {
            return true;
        if let Ok(pattern_mime) = MimeType::parse(pattern) {
            if pattern_mime.main_type == "*" {
                return true;
            if pattern_mime.sub_type == "*" {
                return self.main_type == pattern_mime.main_type;
            return self.matches(&pattern_mime);
        false
    }
}

impl fmt::Display for MimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.main_type, self.sub_type)?;

        for (key, value) in &self.parameters {
            // Quote value if it contains special characters
            if value.chars().any(|c| c.is_whitespace() || "()<>@,;:\\\"/[]?={}".contains(c)) {
                write!(f, "; {}=\"{}\"", key, value)?;
            } else {
                write!(f, "; {}={}", key, value)?;
            }
        }

        Ok(())
    }
}

/// Content type manager with built-in MIME types
#[derive(Debug, Clone)]
pub struct ContentType {
impl ContentType {
    /// Create from MIME type
    pub fn from_mime_type(mime_type: MimeType) -> Self {
        Self { mime_type }
    }

    /// Parse from string
    pub fn parse(content_type_str: &str) -> HttpResult<Self> {
        let mime_type = MimeType::parse(content_type_str)?;
        Ok(Self { mime_type })
    /// Create text/plain
    pub fn text_plain() -> Self {
        Self::from_mime_type(MimeType::new("text", "plain").parameter("charset", "utf-8"))
    /// Create text/html
    pub fn text_html() -> Self {
        Self::from_mime_type(MimeType::new("text", "html").parameter("charset", "utf-8"))
    /// Create application/json
    pub fn application_json() -> Self {
        Self::from_mime_type(MimeType::new("application", "json").parameter("charset", "utf-8"))
    /// Create application/xml
    pub fn application_xml() -> Self {
        Self::from_mime_type(MimeType::new("application", "xml").parameter("charset", "utf-8"))
    /// Create application/x-www-form-urlencoded
    pub fn form_urlencoded() -> Self {
        Self::from_mime_type(MimeType::new("application", "x-www-form-urlencoded"))
    /// Create multipart/form-data
    pub fn multipart_form_data() -> Self {
        Self::from_mime_type(MimeType::new("multipart", "form-data"))
    /// Create application/octet-stream
    pub fn octet_stream() -> Self {
        Self::from_mime_type(MimeType::new("application", "octet-stream"))
    /// Get the underlying MIME type
    pub fn mime_type(&self) -> &MimeType {
        &self.mime_type
    /// Detect content type from file extension
    pub fn from_extension(extension: &str) -> Option<Self> {
        let ext = extension.to_lowercase();
        let mime_type = match ext.as_str() {
            // Text files

            // Images

            // Audio

            // Video

            // Documents

            // Archives

            // Fonts


        Some(Self::from_mime_type(mime_type))
    /// Detect content type from file path
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_extension)
    /// Detect content type from file content (magic bytes)
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.is_empty() {
            return None;
        // Check for common file signatures
        if bytes.len() >= 8 {
            match &bytes[0..8] {
                [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] => {
                    return Some(Self::from_mime_type(MimeType::new("image", "png")));
                }
                [0xFF, 0xD8, 0xFF, ..] => {
                    return Some(Self::from_mime_type(MimeType::new("image", "jpeg")));
                }
                [0x47, 0x49, 0x46, 0x38, 0x37, 0x61, ..] |
                [0x47, 0x49, 0x46, 0x38, 0x39, 0x61, ..] => {
                    return Some(Self::from_mime_type(MimeType::new("image", "gif")));
                }
                [0x52, 0x49, 0x46, 0x46, _, _, _, _] if bytes.len() >= 12 && &bytes[8..12] == b"WEBP" => {
                    return Some(Self::from_mime_type(MimeType::new("image", "webp")));
                }
                [0x25, 0x50, 0x44, 0x46, ..] => {
                    return Some(Self::from_mime_type(MimeType::new("application", "pdf")));
                }
                _ => {}
            }
        }

        // Check for text content
        if bytes.iter().all(|&b| b.is_ascii() || b == 0x09 || b == 0x0A || b == 0x0D) {
            // Try to detect specific text formats
            let text = String::from_utf8_lossy(bytes);
            
            if text.trim_start().starts_with("<!DOCTYPE html") || 
               text.trim_start().starts_with("<html") {
                return Some(Self::text_html());
            if text.trim_start().starts_with('{') || text.trim_start().starts_with('[') {
                if serde_json::from_str::<serde_json::Value>(&text).is_ok() {
                    return Some(Self::application_json());
                }
            }

            if text.trim_start().starts_with("<?xml") || text.trim_start().starts_with('<') {
                return Some(Self::application_xml());
            return Some(Self::text_plain());
        // Default to binary
        Some(Self::octet_stream())
    /// Get the best matching content type from Accept header
    pub fn negotiate(accept_header: &str, available: &[ContentType]) -> Option<ContentType> {
        if accept_header.is_empty() || available.is_empty() {
            return available.first().cloned();
        // Parse Accept header
        let mut accept_types: Vec<(MimeType, f32)> = Vec::new();
        
        for part in accept_header.split(',') {
            let trimmed = part.trim();
            if let Ok(mime_type) = MimeType::parse(trimmed) {
                let quality = mime_type.get_parameter("q")
                    .and_then(|q| q.parse().ok())
                    .unwrap_or(1.0);
                accept_types.push((mime_type, quality));
            }
        }

        // Sort by quality (highest first)
        accept_types.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Find best match
        for (accept_mime, _) in accept_types {
            for content_type in available {
                if content_type.mime_type.matches_wildcard(&accept_mime.to_string()) {
                    return Some(content_type.clone());
                }
            }
        // No match found
        None
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.mime_type)
    }
}

