use crate::web::StatusCode;
// HTTP handler types and utilities for GlowUpHTTP

use crate::stdlib::glowup_http::error::GlowUpResult;
use crate::stdlib::glowup_http::request::VibeRequest;
use crate::stdlib::glowup_http::response::ResponderVibe;
use std::sync::Arc;
use tracing::{debug, instrument};
use crate::error::Error;

/// Primary trait for handling HTTP requests
/// This follows the CURSED spec's `Handler` naming
pub trait Handler: Send + Sync {
    /// Handle an HTTP request and write response
    fn handle_vibe(&self, w: &ResponderVibe, r: &VibeRequest) -> GlowUpResult<()>;
}

/// Function type that can be used as a handler
/// This follows the CURSED spec's `HandlerFunc` naming
pub type HandlerFunc = Arc<dyn Fn(&ResponderVibe, &VibeRequest) -> GlowUpResult<()> + Send + Sync>;

/// Adapter to use functions as handlers
impl Handler for HandlerFunc {
    #[instrument(skip(self, w, r))]
    fn handle_vibe(&self, w: &ResponderVibe, r: &VibeRequest) -> GlowUpResult<()> {
        debug!("Executing handler function for {} {}", r.method, r.url);
        self(w, r)
    }
}

/// Box wrapper for handlers
impl Handler for Box<dyn Handler> {
    fn handle_vibe(&self, w: &ResponderVibe, r: &VibeRequest) -> GlowUpResult<()> {
        (**self).handle_vibe(w, r)
    }
}

/// Arc wrapper for handlers
impl Handler for Arc<dyn Handler> {
    fn handle_vibe(&self, w: &ResponderVibe, r: &VibeRequest) -> GlowUpResult<()> {
        (**self).handle_vibe(w, r)
    }
}

/// Simple handler that returns a fixed response
#[derive(Debug)]
pub struct StaticHandler {
    content: String,
    content_type: String,
}

impl StaticHandler {
    pub fn new(content: impl Into<String>, content_type: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            content_type: content_type.into(),
        }
    }
    
    pub fn text(content: impl Into<String>) -> Self {
        Self::new(content, "text/plain")
    }
    
    pub fn html(content: impl Into<String>) -> Self {
        Self::new(content, "text/html")
    }
    
    pub fn json(content: impl Into<String>) -> Self {
        Self::new(content, "application/json")
    }
}

impl Handler for StaticHandler {
    #[instrument(skip(self, w, r))]
    fn handle_vibe(&self, w: &ResponderVibe, r: &VibeRequest) -> GlowUpResult<()> {
        debug!("Serving static content for {} {}", r.method, r.url);
        
        // Set content type
        {
            let mut headers = w.header().lock().unwrap();
            headers.insert("content-type".to_string(), self.content_type.clone());
        }
        
        w.write(self.content.as_bytes())?;
        Ok(())
    }
}

/// File server handler
#[derive(Debug)]
pub struct FileHandler {
    root: String,
    index_file: String,
}

impl FileHandler {
    pub fn new(root: impl Into<String>) -> Self {
        Self {
            root: root.into(),
            index_file: "index.html".to_string(),
        }
    }
    
    pub fn with_index(mut self, index_file: impl Into<String>) -> Self {
        self.index_file = index_file.into();
        self
    }
}

impl Handler for FileHandler {
    #[instrument(skip(self, w, r))]
    fn handle_vibe(&self, w: &ResponderVibe, r: &VibeRequest) -> GlowUpResult<()> {
        use std::path::{Path, PathBuf};
        use std::fs;
        use crate::stdlib::glowup_http::response::StatusCode;
        
        debug!("Serving file for {} {}", r.method, r.url);
        
        // Clean the path and prevent directory traversal
        let mut path = r.url.trim_start_matches('/');
        if path.is_empty() {
            path = &self.index_file;
        }
        
        // Prevent directory traversal
        if path.contains("..") {
            w.write_header(StatusCode::BAD_REQUEST);
            w.write(b"Bad Request")?;
            return Ok(());
        }
        
        let file_path = PathBuf::from(&self.root).join(path);
        
        // Check if file exists and is not a directory
        match fs::metadata(&file_path) {
            Ok(metadata) if metadata.is_file() => {
                // Read file content
                match fs::read(&file_path) {
                    Ok(content) => {
                        // Determine content type
                        let content_type = self.guess_content_type(&file_path);
                        
                        {
                            let mut headers = w.header().lock().unwrap();
                            headers.insert("content-type".to_string(), content_type);
                        }
                        
                        w.write(&content)?;
                    }
                    Err(_) => {
                        w.write_header(StatusCode::INTERNAL_SERVER_ERROR);
                        w.write(b"Internal Server Error")?;
                    }
                }
            }
            Ok(_) => {
                // It's a directory, try to serve index file
                let index_path = file_path.join(&self.index_file);
                if index_path.exists() {
                    match fs::read(&index_path) {
                        Ok(content) => {
                            let content_type = self.guess_content_type(&index_path);
                            
                            {
                                let mut headers = w.header().lock().unwrap();
                                headers.insert("content-type".to_string(), content_type);
                            }
                            
                            w.write(&content)?;
                        }
                        Err(_) => {
                            w.write_header(StatusCode::INTERNAL_SERVER_ERROR);
                            w.write(b"Internal Server Error")?;
                        }
                    }
                } else {
                    w.write_header(StatusCode::NOT_FOUND);
                    w.write(b"Not Found")?;
                }
            }
            Err(_) => {
                w.write_header(StatusCode::NOT_FOUND);
                w.write(b"Not Found")?;
            }
        }
        
        Ok(())
    }
}

impl FileHandler {
    fn guess_content_type(&self, path: &Path) -> String {
        match path.extension().and_then(|s| s.to_str()) {
            Some("html") | Some("htm") => "text/html",
            Some("css") => "text/css",
            Some("js") => "application/javascript",
            Some("json") => "application/json",
            Some("xml") => "application/xml",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("svg") => "image/svg+xml",
            Some("ico") => "image/x-icon",
            Some("pdf") => "application/pdf",
            Some("txt") => "text/plain",
            Some("md") => "text/markdown",
            Some("woff") => "font/woff",
            Some("woff2") => "font/woff2",
            Some("ttf") => "font/ttf",
            Some("otf") => "font/otf",
            _ => "application/octet-stream",
        }.to_string()
    }
}

/// Redirect handler
#[derive(Debug)]
pub struct RedirectHandler {
    url: String,
    permanent: bool,
}

impl RedirectHandler {
    pub fn new(url: impl Into<String>, permanent: bool) -> Self {
        Self {
            url: url.into(),
            permanent,
        }
    }
    
    pub fn temporary(url: impl Into<String>) -> Self {
        Self::new(url, false)
    }
    
    pub fn permanent(url: impl Into<String>) -> Self {
        Self::new(url, true)
    }
}

impl Handler for RedirectHandler {
    #[instrument(skip(self, w, r))]
    fn handle_vibe(&self, w: &ResponderVibe, r: &VibeRequest) -> GlowUpResult<()> {
        use crate::stdlib::glowup_http::response::StatusCode;
        
        debug!("Redirecting {} {} to {}", r.method, r.url, self.url);
        
        let status_code = if self.permanent {
            StatusCode::MOVED_PERMANENTLY
        } else {
            StatusCode::FOUND
        };
        
        w.redirect(&self.url, status_code)?;
        Ok(())
    }
}

/// Helper function to create a handler from a closure
pub fn handler_func<F>(f: F) -> HandlerFunc
where
    F: Fn(&ResponderVibe, &VibeRequest) -> GlowUpResult<()> + Send + Sync + 'static,
{
    Arc::new(f)
}

/// Helper function to create a JSON handler
pub fn json_handler<T, F>(f: F) -> HandlerFunc
where
    T: serde::Serialize,
    F: Fn(&VibeRequest) -> GlowUpResult<T> + Send + Sync + 'static,
{
    Arc::new(move |w: &ResponderVibe, r: &VibeRequest| {
        let data = f(r)?;
        w.write_json(&data)?;
        Ok(())
    })
}

/// Helper function to create a text handler
pub fn text_handler<F>(f: F) -> HandlerFunc
where
    F: Fn(&VibeRequest) -> GlowUpResult<String> + Send + Sync + 'static,
{
    Arc::new(move |w: &ResponderVibe, r: &VibeRequest| {
        let text = f(r)?;
        
        {
            let mut headers = w.header().lock().unwrap();
            headers.insert("content-type".to_string(), "text/plain".to_string());
        }
        
        w.write(text.as_bytes())?;
        Ok(())
    })
}

/// Helper function to create an HTML handler
pub fn html_handler<F>(f: F) -> HandlerFunc
where
    F: Fn(&VibeRequest) -> GlowUpResult<String> + Send + Sync + 'static,
{
    Arc::new(move |w: &ResponderVibe, r: &VibeRequest| {
        let html = f(r)?;
        
        {
            let mut headers = w.header().lock().unwrap();
            headers.insert("content-type".to_string(), "text/html".to_string());
        }
        
        w.write(html.as_bytes())?;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::glowup_http::request::Method;
    use crate::stdlib::glowup_http::response::StatusCode;

    #[test]
    fn test_static_handler() {
        let handler = StaticHandler::text("Hello, World!");
        let request = VibeRequest::new(Method::GET, "/test");
        let response = ResponderVibe::new();
        
        handler.handle_vibe(&response, &request).unwrap();
        
        let body = response.get_body();
        assert_eq!(body, b"Hello, World!");
        
        let headers = response.get_headers();
        assert_eq!(headers.get("content-type"), Some(&"text/plain".to_string()));
    }

    #[test]
    fn test_redirect_handler() {
        let handler = RedirectHandler::temporary("/new-location");
        let request = VibeRequest::new(Method::GET, "/old-location");
        let response = ResponderVibe::new();
        
        handler.handle_vibe(&response, &request).unwrap();
        
        let headers = response.get_headers();
        assert_eq!(headers.get("location"), Some(&"/new-location".to_string()));
        assert_eq!(response.get_status(), Some(StatusCode::FOUND));
    }

    #[test]
    fn test_handler_func() {
        let handler = handler_func(|w: &ResponderVibe, r: &VibeRequest| {
            w.write(format!("Hello from {}", r.url).as_bytes())?;
            Ok(())
        });
        
        let request = VibeRequest::new(Method::GET, "/test");
        let response = ResponderVibe::new();
        
        handler.handle_vibe(&response, &request).unwrap();
        
        let body = response.get_body();
        assert_eq!(body, b"Hello from /test");
    }
}
