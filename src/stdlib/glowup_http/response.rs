// HTTP response types and utilities for GlowUpHTTP

// use crate::stdlib::glowup_http::error::{GlowUpError, GlowUpResult};
// use crate::stdlib::glowup_http::request::{HeaderMap, Cookie};
use std::io::Write;
use std::sync::{Arc, Mutex};
use tracing::{debug, instrument};
use crate::error::CursedError;

// Use the main StatusCode type from crate::web instead of defining our own
pub use crate::web::StatusCode;

/// Content that can be written to an HTTP response
#[derive(Debug, Clone)]
pub enum ResponseBody {
    Empty,
    Text(String),
    Json(String),
    Html(String),
    Bytes(Vec<u8>),
}

/// Response writer for HTTP responses
pub trait ResponderVibe {
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::CursedError + Send + Sync>>;
    fn write_header(&mut self, status: StatusCode) -> Result<(), Box<dyn std::error::CursedError + Send + Sync>>;
    fn set_header(&mut self, key: &str, value: &str) -> Result<(), Box<dyn std::error::CursedError + Send + Sync>>;
    fn get_status(&self) -> Option<StatusCode>;
    fn get_headers(&self) -> &HeaderMap;
}

/// HTTP Response structure for complete response handling
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: Option<StatusCode>,
    pub headers: HeaderMap,
    pub body: ResponseBody,
    pub cookies: Vec<Cookie>, 
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpResponse {
    /// Create a new empty response
    pub fn new() -> Self {
        Self {
            status: None,
            headers: HeaderMap::new(),
            body: ResponseBody::Empty,
            cookies: Vec::new(),
        }
    }

    /// Set the status code
    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = Some(status);
        self
    }

    /// Set a header
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Set the body as text
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.body = ResponseBody::Text(text.into());
        self
    }

    /// Set the body as JSON
    pub fn json(mut self, json: impl Into<String>) -> Self {
        self.body = ResponseBody::Json(json.into());
        self
    }

    /// Set the body as HTML
    pub fn html(mut self, html: impl Into<String>) -> Self {
        self.body = ResponseBody::Html(html.into());
        self
    }

    /// Set the body as bytes
    pub fn bytes(mut self, bytes: Vec<u8>) -> Self {
        self.body = ResponseBody::Bytes(bytes);
        self
    }

    /// Add a cookie
    pub fn cookie(mut self, cookie: Cookie) -> Self {
        self.cookies.push(cookie);
        self
    }

    /// Get the status code
    pub fn get_status(&self) -> Option<StatusCode> {
        self.status
    }

    /// Get the headers
    pub fn get_headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Write the response to a writer
    pub fn write_to<W: Write>(&self, writer: &mut W) -> GlowUpResult<()> {
        // Write status line
        let status = self.status.unwrap_or(StatusCode::OK);
        writeln!(writer, "HTTP/1.1 {} {}", status as u16, status.canonical_reason())?;

        // Write headers
        for (key, value) in &self.headers {
            writeln!(writer, "{}: {}", key, value)?;
        }

        // Write cookies
        for cookie in &self.cookies {
            writeln!(writer, "Set-Cookie: {}", cookie)?;
        }

        // Write blank line
        writeln!(writer)?;

        // Write body
        match &self.body {
            ResponseBody::Empty => {},
            ResponseBody::Text(text) => writer.write_all(text.as_bytes())?,
            ResponseBody::Json(json) => writer.write_all(json.as_bytes())?,
            ResponseBody::Html(html) => writer.write_all(html.as_bytes())?,
            ResponseBody::Bytes(bytes) => writer.write_all(bytes)?,
        }

        Ok(())
    }
}

impl ResponderVibe for HttpResponse {
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::CursedError + Send + Sync>> {
        match &mut self.body {
            ResponseBody::Empty => {
                self.body = ResponseBody::Bytes(data.to_vec());
            },
            ResponseBody::Bytes(existing) => {
                existing.extend_from_slice(data);
            },
            _ => {
                // Convert existing body to bytes and append
                let existing_bytes = match &self.body {
                    ResponseBody::Text(s) => s.as_bytes().to_vec(),
                    ResponseBody::Json(s) => s.as_bytes().to_vec(),
                    ResponseBody::Html(s) => s.as_bytes().to_vec(),
                    _ => Vec::new(),
                };
                let mut new_bytes = existing_bytes;
                new_bytes.extend_from_slice(data);
                self.body = ResponseBody::Bytes(new_bytes);
            }
        }
        Ok(())
    }

    fn write_header(&mut self, status: StatusCode) -> Result<(), Box<dyn std::error::CursedError + Send + Sync>> {
        self.status = Some(status);
        Ok(())
    }

    fn set_header(&mut self, key: &str, value: &str) -> Result<(), Box<dyn std::error::CursedError + Send + Sync>> {
        self.headers.insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn get_status(&self) -> Option<StatusCode> {
        self.status
    }

    fn get_headers(&self) -> &HeaderMap {
        &self.headers
    }
}

/// Helper to create common responses
pub struct ResponseBuilder;

impl ResponseBuilder {
    /// Create a 200 OK response
    pub fn ok() -> HttpResponse {
        HttpResponse::new().status(StatusCode::OK)
    }

    /// Create a 201 Created response
    pub fn created() -> HttpResponse {
        HttpResponse::new().status(StatusCode::Created)
    }

    /// Create a 404 Not Found response
    pub fn not_found() -> HttpResponse {
        HttpResponse::new().status(StatusCode::NotFound)
    }

    /// Create a 500 Internal Server CursedError response
    pub fn internal_error() -> HttpResponse {
        HttpResponse::new().status(StatusCode::InternalServerError)
    }

    /// Create a 400 Bad Request response
    pub fn bad_request() -> HttpResponse {
        HttpResponse::new().status(StatusCode::BadRequest)
    }
}


impl VibeResponse {
    /// Create a new response
    pub fn new(status_code: StatusCode) -> Self {
        Self {
            status: status_code.as_u16(),
            headers: HeaderMap::new(),
            body: Vec::new(),
            status_line: format!("{}", status_code),
            status_code,
            proto: "HTTP/1.1".to_string(),
            content_length: -1,
            transfer_encoding: Vec::new(),
            close: false,
            uncompressed: false,
            trailer: HeaderMap::new(),
        }
    }

    /// Get response body as string  
    pub fn text(&self) -> Result<String, std::str::Utf8Error> {
        std::str::from_utf8(&self.body).map(|s| s.to_string())
    }

    /// Get response body as bytes
    pub fn bytes(&self) -> &[u8] {
        &self.body
    }

    /// Check if response is successful (status 200-299)
    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    /// Get header value
    pub fn get_header(&self, name: &str) -> Option<String> {
        self.headers.get(name)
    }
}
