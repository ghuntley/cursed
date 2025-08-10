//! Live Documentation Server
//! 
//! Provides a live development server for CURSED documentation with hot reload,
//! file watching, and real-time updates during development.

use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use crate::error::CursedError;
use crate::documentation::{DocumentationGenerator, DocConfig};

/// Live documentation server
pub struct LiveDocServer {
    config: DocConfig,
    port: u16,
    host: String,
    file_watcher: Option<FileWatcher>,
    documentation_cache: Arc<Mutex<Option<String>>>,
    last_build_time: Arc<Mutex<SystemTime>>,
}

/// File system watcher for detecting changes
pub struct FileWatcher {
    watch_paths: Vec<PathBuf>,
    last_modified: HashMap<PathBuf, SystemTime>,
}

/// HTTP response structure
pub struct HttpResponse {
    status: u16,
    headers: HashMap<String, String>,
    body: String,
}

impl LiveDocServer {
    /// Create a new live documentation server
    pub fn new(config: DocConfig, port: Option<u16>, host: Option<String>) -> Self {
        Self {
            config,
            port: port.unwrap_or(8080),
            host: host.unwrap_or_else(|| "localhost".to_string()),
            file_watcher: None,
            documentation_cache: Arc::new(Mutex::new(None)),
            last_build_time: Arc::new(Mutex::new(SystemTime::UNIX_EPOCH)),
        }
    }

    /// Start the live documentation server
    pub fn start(&mut self) -> Result<(), CursedError> {
        println!("Starting CURSED Documentation Live Server...");
        println!("Server: http://{}:{}", self.host, self.port);
        println!("Press Ctrl+C to stop");

        // Initialize file watcher
        self.setup_file_watcher()?;

        // Initial documentation build
        self.rebuild_documentation()?;

        // Start the HTTP server
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port))
            .map_err(|e| CursedError::IoError(format!("Failed to bind server: {}", e)))?;

        // Start file watching in background thread
        self.start_file_watcher()?;

        // Handle incoming connections
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_connection(stream)?;
                }
                Err(e) => {
                    eprintln!("Connection error: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Setup file system watcher
    fn setup_file_watcher(&mut self) -> Result<(), CursedError> {
        let mut watch_paths = Vec::new();
        
        // Add source directories to watch
        for source_dir in &self.config.input.source_dirs {
            let path = PathBuf::from(source_dir);
            if path.exists() {
                watch_paths.push(path);
            }
        }

        // Add config file to watch
        watch_paths.push(PathBuf::from(".cursed-doc.toml"));

        let mut last_modified = HashMap::new();
        for path in &watch_paths {
            if let Ok(metadata) = fs::metadata(path) {
                if let Ok(modified) = metadata.modified() {
                    last_modified.insert(path.clone(), modified);
                }
            }
        }

        self.file_watcher = Some(FileWatcher {
            watch_paths,
            last_modified,
        });

        Ok(())
    }

    /// Start file watcher in background thread
    fn start_file_watcher(&self) -> Result<(), CursedError> {
        let watcher = self.file_watcher.as_ref()
            .ok_or_else(|| CursedError::InternalError("File watcher not initialized".to_string()))?;

        let watch_paths = watcher.watch_paths.clone();
        let last_modified = Arc::new(Mutex::new(watcher.last_modified.clone()));
        let cache = Arc::clone(&self.documentation_cache);
        let build_time = Arc::clone(&self.last_build_time);
        let config = self.config.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(500));

                let mut should_rebuild = false;
                let mut last_mod = last_modified.lock().unwrap();

                for watch_path in &watch_paths {
                    if let Ok(current_modified) = Self::get_directory_modified_time(watch_path) {
                        if let Some(last_time) = last_mod.get(watch_path) {
                            if current_modified > *last_time {
                                should_rebuild = true;
                                last_mod.insert(watch_path.clone(), current_modified);
                            }
                        }
                    }
                }

                if should_rebuild {
                    println!("Files changed, rebuilding documentation...");
                    if let Err(e) = Self::rebuild_documentation_static(&config, &cache, &build_time) {
                        eprintln!("Failed to rebuild documentation: {}", e);
                    } else {
                        println!("Documentation rebuilt successfully");
                    }
                }
            }
        });

        Ok(())
    }

    /// Get the latest modification time for a directory
    fn get_directory_modified_time(path: &Path) -> Result<SystemTime, io::Error> {
        let mut latest = SystemTime::UNIX_EPOCH;

        if path.is_file() {
            return fs::metadata(path)?.modified();
        }

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            
            if metadata.is_file() {
                if let Ok(modified) = metadata.modified() {
                    if modified > latest {
                        latest = modified;
                    }
                }
            } else if metadata.is_dir() {
                if let Ok(dir_modified) = Self::get_directory_modified_time(&entry.path()) {
                    if dir_modified > latest {
                        latest = dir_modified;
                    }
                }
            }
        }

        Ok(latest)
    }

    /// Rebuild documentation
    fn rebuild_documentation(&self) -> Result<(), CursedError> {
        Self::rebuild_documentation_static(&self.config, &self.documentation_cache, &self.last_build_time)
    }

    /// Static method for rebuilding documentation (used in thread)
    fn rebuild_documentation_static(
        config: &DocConfig,
        cache: &Arc<Mutex<Option<String>>>,
        build_time: &Arc<Mutex<SystemTime>>,
    ) -> Result<(), CursedError> {
        let mut generator = DocumentationGenerator::new(None)?;
        generator.generate()?;

        // Cache the index page
        let index_path = Path::new(&config.output.output_dir).join("index.html");
        if let Ok(content) = fs::read_to_string(&index_path) {
            *cache.lock().unwrap() = Some(content);
        }

        *build_time.lock().unwrap() = SystemTime::now();

        Ok(())
    }

    /// Handle incoming HTTP connection
    fn handle_connection(&self, mut stream: TcpStream) -> Result<(), CursedError> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)
            .map_err(|e| CursedError::IoError(format!("Failed to read request: {}", e)))?;

        let request = String::from_utf8_lossy(&buffer);
        let request_line = request.lines().next().unwrap_or("");
        
        let response = self.route_request(request_line)?;
        self.send_response(stream, response)?;

        Ok(())
    }

    /// Route HTTP request to appropriate handler
    fn route_request(&self, request_line: &str) -> Result<HttpResponse, CursedError> {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 2 {
            return Ok(self.create_error_response(400, "Bad Request"));
        }

        let method = parts[0];
        let path = parts[1];

        if method != "GET" {
            return Ok(self.create_error_response(405, "Method Not Allowed"));
        }

        match path {
            "/" | "/index.html" => self.serve_index(),
            path if path.starts_with("/api/") => self.handle_api_request(path),
            path if path.starts_with("/assets/") => self.serve_static_file(path),
            path if path.ends_with(".html") => self.serve_static_file(path),
            path if path.starts_with("/modules/") => self.serve_static_file(path),
            _ => Ok(self.create_error_response(404, "Not Found")),
        }
    }

    /// Serve the main index page
    fn serve_index(&self) -> Result<HttpResponse, CursedError> {
        if let Some(cached_content) = self.documentation_cache.lock().unwrap().clone() {
            // Inject live reload script
            let live_reload_script = r#"
<script>
(function() {
    let lastBuildTime = 0;
    
    function checkForUpdates() {
        fetch('/api/status')
            .then(response => response.json())
            .then(data => {
                if (data.buildTime > lastBuildTime) {
                    lastBuildTime = data.buildTime;
                    if (lastBuildTime > 0) {
                        console.log('Documentation updated, reloading...');
                        window.location.reload();
                    }
                }
            })
            .catch(error => console.log('Live reload check failed:', error));
    }
    
    // Check for updates every 2 seconds
    setInterval(checkForUpdates, 2000);
    checkForUpdates();
})();
</script>
</body>"#;

            let content = cached_content.replace("</body>", live_reload_script);
            
            Ok(HttpResponse {
                status: 200,
                headers: self.create_html_headers(),
                body: content,
            })
        } else {
            Ok(self.create_error_response(503, "Documentation not ready"))
        }
    }

    /// Handle API requests
    fn handle_api_request(&self, path: &str) -> Result<HttpResponse, CursedError> {
        match path {
            "/api/status" => {
                let build_time = self.last_build_time.lock().unwrap()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                let status_json = format!(r#"{{"buildTime": {}, "status": "ready"}}"#, build_time);
                
                Ok(HttpResponse {
                    status: 200,
                    headers: self.create_json_headers(),
                    body: status_json,
                })
            }
            "/api/rebuild" => {
                self.rebuild_documentation()?;
                let response_json = r#"{"status": "rebuilt", "message": "Documentation rebuilt successfully"}"#;
                
                Ok(HttpResponse {
                    status: 200,
                    headers: self.create_json_headers(),
                    body: response_json.to_string(),
                })
            }
            _ => Ok(self.create_error_response(404, "API endpoint not found")),
        }
    }

    /// Serve static files
    fn serve_static_file(&self, path: &str) -> Result<HttpResponse, CursedError> {
        let file_path = Path::new(&self.config.output.output_dir)
            .join(path.trim_start_matches('/'));

        if !file_path.exists() {
            return Ok(self.create_error_response(404, "File not found"));
        }

        let content = fs::read_to_string(&file_path)
            .map_err(|e| CursedError::IoError(format!("Failed to read file: {}", e)))?;

        let headers = if path.ends_with(".css") {
            self.create_css_headers()
        } else if path.ends_with(".js") {
            self.create_js_headers()
        } else if path.ends_with(".json") {
            self.create_json_headers()
        } else {
            self.create_html_headers()
        };

        Ok(HttpResponse {
            status: 200,
            headers,
            body: content,
        })
    }

    /// Create error response
    fn create_error_response(&self, status: u16, message: &str) -> HttpResponse {
        let body = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Error {}</title>
    <style>
        body {{ font-family: Arial, sans-serif; padding: 2rem; text-align: center; }}
        .error {{ background: #f8d7da; color: #721c24; padding: 1rem; border-radius: 4px; }}
    </style>
</head>
<body>
    <div class="error">
        <h1>Error {}</h1>
        <p>{}</p>
        <a href="/">Return to Documentation</a>
    </div>
</body>
</html>
"#, status, status, message);

        HttpResponse {
            status,
            headers: self.create_html_headers(),
            body,
        }
    }

    /// Send HTTP response
    fn send_response(&self, mut stream: TcpStream, response: HttpResponse) -> Result<(), CursedError> {
        let mut response_text = format!("HTTP/1.1 {} OK\r\n", response.status);
        
        for (key, value) in response.headers {
            response_text.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        response_text.push_str(&format!("Content-Length: {}\r\n", response.body.len()));
        response_text.push_str("\r\n");
        response_text.push_str(&response.body);

        stream.write_all(response_text.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write response: {}", e)))?;

        stream.flush()
            .map_err(|e| CursedError::IoError(format!("Failed to flush response: {}", e)))?;

        Ok(())
    }

    /// Create HTML headers
    fn create_html_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());
        headers.insert("Cache-Control".to_string(), "no-cache".to_string());
        headers
    }

    /// Create CSS headers
    fn create_css_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/css".to_string());
        headers.insert("Cache-Control".to_string(), "no-cache".to_string());
        headers
    }

    /// Create JavaScript headers
    fn create_js_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/javascript".to_string());
        headers.insert("Cache-Control".to_string(), "no-cache".to_string());
        headers
    }

    /// Create JSON headers
    fn create_json_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Cache-Control".to_string(), "no-cache".to_string());
        headers
    }
}

/// CLI interface for live server
pub struct LiveServerCli;

impl LiveServerCli {
    /// Run live server from command line
    pub fn run(args: Vec<String>) -> Result<(), CursedError> {
        let config_path = args.get(1).map(|s| s.as_str());
        let port = args.get(2).and_then(|s| s.parse().ok());
        let host = args.get(3).map(|s| s.to_string());

        let config = if let Some(path) = config_path {
            Self::load_config(path)?
        } else {
            Self::default_config()
        };

        let mut server = LiveDocServer::new(config, port, host);
        server.start()
    }

    /// Load configuration from file
    fn load_config(config_path: &str) -> Result<DocConfig, CursedError> {
        if !Path::new(config_path).exists() {
            return Ok(Self::default_config());
        }

        let content = fs::read_to_string(config_path)
            .map_err(|e| CursedError::IoError(format!("Failed to read config file: {}", e)))?;

        toml::from_str(&content)
            .map_err(|e| CursedError::ParseError(format!("Invalid config file: {}", e)))
    }

    /// Generate default configuration
    fn default_config() -> DocConfig {
        crate::documentation::DocumentationGenerator::default_config()
    }
}

// Re-export for use in other modules
pub use self::LiveDocServer as LiveServer;
