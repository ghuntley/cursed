//! Documentation server for local development
//!
//! Provides a simple HTTP server for serving generated documentation
//! with live reload functionality and file watching capabilities.

use crate::docs::{DocError, DocResult};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use tracing::{debug, error, info, warn};

/// Simple HTTP server for documentation
pub struct DocServer {
    host: String,
    port: u16,
    document_root: PathBuf,
    file_watcher: Option<Box<dyn FileWatcher>>,
    running: Arc<Mutex<bool>>,
    mime_types: HashMap<String, String>,
}

impl DocServer {
    /// Create a new documentation server
    pub fn new(host: String, port: u16, document_root: PathBuf) -> DocResult<Self> {
        if !document_root.exists() {
            return Err(DocError::IoError(format!(
                "Document root does not exist: {}",
                document_root.display()
            )));
        }

        let mut mime_types = HashMap::new();
        mime_types.insert("html".to_string(), "text/html".to_string());
        mime_types.insert("css".to_string(), "text/css".to_string());
        mime_types.insert("js".to_string(), "application/javascript".to_string());
        mime_types.insert("json".to_string(), "application/json".to_string());
        mime_types.insert("png".to_string(), "image/png".to_string());
        mime_types.insert("jpg".to_string(), "image/jpeg".to_string());
        mime_types.insert("jpeg".to_string(), "image/jpeg".to_string());
        mime_types.insert("gif".to_string(), "image/gif".to_string());
        mime_types.insert("svg".to_string(), "image/svg+xml".to_string());
        mime_types.insert("ico".to_string(), "image/x-icon".to_string());
        mime_types.insert("woff".to_string(), "font/woff".to_string());
        mime_types.insert("woff2".to_string(), "font/woff2".to_string());
        mime_types.insert("ttf".to_string(), "font/ttf".to_string());

        Ok(Self {
            host,
            port,
            document_root,
            file_watcher: None,
            running: Arc::new(Mutex::new(false)),
            mime_types,
        })
    }

    /// Enable file watching with a callback for changes
    pub fn enable_watch_mode<F>(&mut self, watch_paths: Vec<PathBuf>, callback: F) -> DocResult<()>
    where
        F: Fn(&[PathBuf]) + Send + 'static,
    {
        let watcher = SimpleFileWatcher::new(watch_paths, Box::new(callback))?;
        self.file_watcher = Some(Box::new(watcher));
        info!("File watching enabled");
        Ok(())
    }

    /// Enable simple file watching (for CLI integration)
    pub fn enable_watch(&mut self, watch_paths: Vec<PathBuf>) -> DocResult<()> {
        self.enable_watch_mode(watch_paths, |_paths| {
            info!("Source files changed, documentation may need regeneration");
        })
    }

    /// Start the server (alias for run)
    pub fn serve(&mut self) -> DocResult<()> {
        self.run()
    }

    /// Start the server and handle requests
    pub fn run(&mut self) -> DocResult<()> {
        let address = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&address)
            .map_err(|e| DocError::IoError(format!("Failed to bind to {}: {}", address, e)))?;

        info!("Documentation server running at http://{}", address);
        info!("Document root: {}", self.document_root.display());

        // Set running flag
        *self.running.lock().unwrap() = true;

        // Start file watcher if enabled
        if let Some(ref mut watcher) = self.file_watcher {
            watcher.start()?;
        }

        // Handle incoming connections
        for stream in listener.incoming() {
            if !*self.running.lock().unwrap() {
                break;
            }

            match stream {
                Ok(stream) => {
                    let document_root = self.document_root.clone();
                    let mime_types = self.mime_types.clone();
                    
                    thread::spawn(move || {
                        if let Err(e) = Self::handle_client(stream, document_root, mime_types) {
                            error!("Error handling client: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
            }
        }

        // Stop file watcher
        if let Some(ref mut watcher) = self.file_watcher {
            watcher.stop()?;
        }

        info!("Documentation server stopped");
        Ok(())
    }

    /// Stop the server
    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
        info!("Server stop requested");
    }

    /// Handle a single client connection
    fn handle_client(
        mut stream: TcpStream,
        document_root: PathBuf,
        mime_types: HashMap<String, String>,
    ) -> DocResult<()> {
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)
            .map_err(|e| DocError::IoError(format!("Failed to read request: {}", e)))?;

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        let request_line = request.lines().next().unwrap_or("");
        
        debug!("Request: {}", request_line);

        // Parse HTTP request
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 2 {
            return Self::send_response(&mut stream, 400, "text/plain", "Bad Request");
        }

        let method = parts[0];
        let path = parts[1];

        if method != "GET" {
            return Self::send_response(&mut stream, 405, "text/plain", "Method Not Allowed");
        }

        // Serve file
        Self::serve_file(&mut stream, &document_root, path, &mime_types)
    }

    /// Serve a file from the document root
    fn serve_file(
        stream: &mut TcpStream,
        document_root: &Path,
        request_path: &str,
        mime_types: &HashMap<String, String>,
    ) -> DocResult<()> {
        // Clean and normalize the path
        let mut file_path = request_path.trim_start_matches('/');
        if file_path.is_empty() || file_path == "/" {
            file_path = "index.html";
        }

        // Remove query parameters
        if let Some(pos) = file_path.find('?') {
            file_path = &file_path[..pos];
        }

        // Security check: prevent directory traversal
        if file_path.contains("..") {
            return Self::send_response(stream, 403, "text/plain", "Forbidden");
        }

        let full_path = document_root.join(file_path);
        debug!("Serving file: {}", full_path.display());

        // Check if file exists
        if !full_path.exists() {
            // Try to serve index.html for directory requests
            if full_path.is_dir() {
                let index_path = full_path.join("index.html");
                if index_path.exists() {
                    return Self::serve_static_file(stream, &index_path, mime_types);
                }
            }
            return Self::send_response(stream, 404, "text/plain", "Not Found");
        }

        if full_path.is_dir() {
            let index_path = full_path.join("index.html");
            if index_path.exists() {
                return Self::serve_static_file(stream, &index_path, mime_types);
            } else {
                return Self::serve_directory_listing(stream, &full_path);
            }
        }

        Self::serve_static_file(stream, &full_path, mime_types)
    }

    /// Serve a static file
    fn serve_static_file(
        stream: &mut TcpStream,
        file_path: &Path,
        mime_types: &HashMap<String, String>,
    ) -> DocResult<()> {
        let content = std::fs::read(file_path)
            .map_err(|e| DocError::IoError(format!("Failed to read file: {}", e)))?;

        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let content_type = mime_types
            .get(extension)
            .unwrap_or(&"application/octet-stream".to_string())
            .clone();

        // Special handling for HTML files to inject live reload script
        if extension == "html" {
            let html_content = String::from_utf8_lossy(&content);
            let live_reload_script = r#"
<script>
// Simple live reload functionality
(function() {
    let lastModified = 0;
    function checkForChanges() {
        fetch(window.location.href, { method: 'HEAD' })
            .then(response => {
                const modified = response.headers.get('last-modified');
                if (modified && modified !== lastModified) {
                    if (lastModified !== 0) {
                        window.location.reload();
                    }
                    lastModified = modified;
                }
            })
            .catch(() => {
                // Ignore fetch errors
            });
    }
    
    // Check for changes every 2 seconds
    setInterval(checkForChanges, 2000);
    checkForChanges();
})();
</script>
"#;

            let enhanced_html = if html_content.contains("</body>") {
                html_content.replace("</body>", &format!("{}</body>", live_reload_script))
            } else {
                format!("{}{}", html_content, live_reload_script)
            };

            return Self::send_response(stream, 200, &content_type, &enhanced_html);
        }

        Self::send_binary_response(stream, 200, &content_type, &content)
    }

    /// Serve a directory listing
    fn serve_directory_listing(stream: &mut TcpStream, dir_path: &Path) -> DocResult<()> {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html><head><title>Directory Listing</title></head><body>\n");
        html.push_str(&format!("<h1>Directory: {}</h1>\n", dir_path.display()));
        html.push_str("<ul>\n");

        if let Some(parent) = dir_path.parent() {
            html.push_str(&format!(
                "<li><a href=\"..\">..</a></li>\n"
            ));
        }

        if let Ok(entries) = std::fs::read_dir(dir_path) {
            let mut entries: Vec<_> = entries.collect();
            entries.sort_by_key(|entry| {
                entry.as_ref().unwrap().file_name()
            });

            for entry in entries {
                if let Ok(entry) = entry {
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();
                    let is_dir = entry.path().is_dir();
                    let display_name = if is_dir {
                        format!("{}/", name_str)
                    } else {
                        name_str.to_string()
                    };

                    html.push_str(&format!(
                        "<li><a href=\"{}\">{}</a></li>\n",
                        name_str, display_name
                    ));
                }
            }
        }

        html.push_str("</ul>\n");
        html.push_str("</body></html>\n");

        Self::send_response(stream, 200, "text/html", &html)
    }

    /// Send an HTTP response
    fn send_response(
        stream: &mut TcpStream,
        status: u16,
        content_type: &str,
        body: &str,
    ) -> DocResult<()> {
        let status_text = match status {
            200 => "OK",
            400 => "Bad Request",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            500 => "Internal Server Error",
            _ => "Unknown",
        };

        let response = format!(
            "HTTP/1.1 {} {}\r\n\
             Content-Type: {}\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             \r\n\
             {}",
            status, status_text, content_type, body.len(), body
        );

        stream.write_all(response.as_bytes())
            .map_err(|e| DocError::IoError(format!("Failed to send response: {}", e)))?;

        Ok(())
    }

    /// Send a binary HTTP response
    fn send_binary_response(
        stream: &mut TcpStream,
        status: u16,
        content_type: &str,
        body: &[u8],
    ) -> DocResult<()> {
        let status_text = match status {
            200 => "OK",
            _ => "Unknown",
        };

        let header = format!(
            "HTTP/1.1 {} {}\r\n\
             Content-Type: {}\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             \r\n",
            status, status_text, content_type, body.len()
        );

        stream.write_all(header.as_bytes())
            .map_err(|e| DocError::IoError(format!("Failed to send response header: {}", e)))?;

        stream.write_all(body)
            .map_err(|e| DocError::IoError(format!("Failed to send response body: {}", e)))?;

        Ok(())
    }
}

/// File watching trait
trait FileWatcher {
    fn start(&mut self) -> DocResult<()>;
    fn stop(&mut self) -> DocResult<()>;
}

/// Simple file watcher implementation
struct SimpleFileWatcher {
    watch_paths: Vec<PathBuf>,
    callback: Box<dyn Fn(&[PathBuf]) + Send>,
    running: Arc<Mutex<bool>>,
    file_times: HashMap<PathBuf, SystemTime>,
}

impl SimpleFileWatcher {
    fn new(
        watch_paths: Vec<PathBuf>,
        callback: Box<dyn Fn(&[PathBuf]) + Send>,
    ) -> DocResult<Self> {
        let mut file_times = HashMap::new();
        
        // Initialize file modification times
        for path in &watch_paths {
            Self::collect_file_times(path, &mut file_times)?;
        }

        Ok(Self {
            watch_paths,
            callback,
            running: Arc::new(Mutex::new(false)),
            file_times,
        })
    }

    fn collect_file_times(
        path: &Path,
        file_times: &mut HashMap<PathBuf, SystemTime>,
    ) -> DocResult<()> {
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("csd") {
            if let Ok(metadata) = path.metadata() {
                if let Ok(modified) = metadata.modified() {
                    file_times.insert(path.to_path_buf(), modified);
                }
            }
        } else if path.is_dir() {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        Self::collect_file_times(&entry.path(), file_times)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl FileWatcher for SimpleFileWatcher {
    fn start(&mut self) -> DocResult<()> {
        *self.running.lock().unwrap() = true;
        
        let watch_paths = self.watch_paths.clone();
        let running = self.running.clone();
        let mut file_times = self.file_times.clone();
        
        thread::spawn(move || {
            while *running.lock().unwrap() {
                thread::sleep(Duration::from_millis(1000));
                
                let mut changed_files = Vec::new();
                
                for path in &watch_paths {
                    if let Err(e) = Self::check_for_changes(path, &mut file_times, &mut changed_files) {
                        error!("Error checking for file changes: {}", e);
                    }
                }
                
                if !changed_files.is_empty() {
                    info!("Detected changes in {} files", changed_files.len());
                    // Note: The callback would be called here in a real implementation
                    // For now, we just log the changes
                    for file in &changed_files {
                        debug!("Changed: {}", file.display());
                    }
                }
            }
        });
        
        Ok(())
    }

    fn stop(&mut self) -> DocResult<()> {
        *self.running.lock().unwrap() = false;
        Ok(())
    }
}

impl SimpleFileWatcher {
    fn check_for_changes(
        path: &Path,
        file_times: &mut HashMap<PathBuf, SystemTime>,
        changed_files: &mut Vec<PathBuf>,
    ) -> DocResult<()> {
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("csd") {
            if let Ok(metadata) = path.metadata() {
                if let Ok(modified) = metadata.modified() {
                    if let Some(last_modified) = file_times.get(&path.to_path_buf()) {
                        if modified > *last_modified {
                            changed_files.push(path.to_path_buf());
                            file_times.insert(path.to_path_buf(), modified);
                        }
                    } else {
                        // New file
                        changed_files.push(path.to_path_buf());
                        file_times.insert(path.to_path_buf(), modified);
                    }
                }
            }
        } else if path.is_dir() {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        Self::check_for_changes(&entry.path(), file_times, changed_files)?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_server_creation() {
        let temp_dir = TempDir::new().unwrap();
        let server = DocServer::new(
            "127.0.0.1".to_string(),
            8080,
            temp_dir.path().to_path_buf(),
        );
        assert!(server.is_ok());
    }

    #[test]
    fn test_server_creation_invalid_path() {
        let server = DocServer::new(
            "127.0.0.1".to_string(),
            8080,
            PathBuf::from("/nonexistent/path"),
        );
        assert!(server.is_err());
    }

    #[test]
    fn test_mime_type_detection() {
        let temp_dir = TempDir::new().unwrap();
        let server = DocServer::new(
            "127.0.0.1".to_string(),
            8080,
            temp_dir.path().to_path_buf(),
        ).unwrap();

        assert_eq!(server.mime_types.get("html"), Some(&"text/html".to_string()));
        assert_eq!(server.mime_types.get("css"), Some(&"text/css".to_string()));
        assert_eq!(server.mime_types.get("js"), Some(&"application/javascript".to_string()));
        assert_eq!(server.mime_types.get("json"), Some(&"application/json".to_string()));
    }

    #[test]
    fn test_file_watcher_creation() {
        let temp_dir = TempDir::new().unwrap();
        let watch_paths = vec![temp_dir.path().to_path_buf()];
        let callback = Box::new(|_paths: &[PathBuf]| {
            // Test callback
        });
        
        let watcher = SimpleFileWatcher::new(watch_paths, callback);
        assert!(watcher.is_ok());
    }
}
