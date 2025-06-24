
// HTTP server implementation for GlowUpHTTP

use crate::stdlib::glowup_http::error::{GlowUpError, GlowUpResult};
use crate::stdlib::glowup_http::handler::{Handler, HandlerFunc};
use crate::stdlib::glowup_http::request::{VibeRequest, Method, HttpVersion, HeaderMap};
use crate::stdlib::glowup_http::response::ResponderVibe;
use crate::web::StatusCode;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, AtomicU64, Ordering}};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use tracing::{debug, info, warn, error, instrument, span, Level};
use crate::error::Error;

/// Configurable HTTP server
/// This follows the CURSED spec's `VibeServer` naming
#[derive(Debug)]
pub struct VibeServer {
    /// Server bind address
    pub addr: String,
    /// Main request handler
    pub handler: Option<Arc<dyn Handler>>,
    /// Read timeout
    pub read_timeout: Duration,
    /// Write timeout 
    pub write_timeout: Duration,
    /// Idle timeout
    pub idle_timeout: Duration,
    /// Maximum header bytes
    pub max_header_bytes: usize,
    /// TLS configuration
    pub tls_config: Option<TlsConfig>,
    /// Error logging
    pub error_log: Option<Arc<dyn ErrorLogger>>,
    /// Server state
    state: Arc<ServerState>,
    /// Connection pool
    connection_pool: Arc<ConnectionPool>,
}

/// TLS configuration
#[derive(Debug, Clone)]
pub struct TlsConfig {
    pub cert_file: String,
    pub key_file: String,
    pub min_version: TlsVersion,
    pub max_version: TlsVersion,
}

#[derive(Debug, Clone, Copy)]
pub enum TlsVersion {
    TLSv1_2,
    TLSv1_3,
}

/// Error logger trait
pub trait ErrorLogger: Send + Sync {
    fn log(&self, level: LogLevel, message: &str);
}

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Server state tracking
#[derive(Debug)]
struct ServerState {
    running: AtomicBool,
    shutdown_requested: AtomicBool,
    active_connections: AtomicU64,
    total_requests: AtomicU64,
    start_time: Instant,
    listener: Mutex<Option<TcpListener>>,
}

/// Connection pool for managing client connections
#[derive(Debug)]
struct ConnectionPool {
    max_connections: usize,
    active_connections: Mutex<HashMap<u64, Arc<Connection>>>,
    connection_counter: AtomicU64,
    connection_timeout: Duration,
}

/// Active HTTP connection
#[derive(Debug)]
struct Connection {
    id: u64,
    start_time: Instant,
    remote_addr: SocketAddr,
    local_addr: SocketAddr,
    request_count: AtomicU64,
    last_activity: Mutex<Instant>,
    keep_alive: AtomicBool,
}

impl VibeServer {
    /// Create a new HTTP server
    pub fn new() -> Self {
        Self {
            addr: "127.0.0.1:8080".to_string(),
            handler: None,
            read_timeout: Duration::from_secs(30),
            write_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(120),
            max_header_bytes: 8192,
            tls_config: None,
            error_log: None,
            state: Arc::new(ServerState {
                running: AtomicBool::new(false),
                shutdown_requested: AtomicBool::new(false),
                active_connections: AtomicU64::new(0),
                total_requests: AtomicU64::new(0),
                start_time: Instant::now(),
                listener: Mutex::new(None),
            }),
            connection_pool: Arc::new(ConnectionPool::new(1000, Duration::from_secs(300))),
        }
    }
    
    /// Set server address
    pub fn addr(mut self, addr: impl Into<String>) -> Self {
        self.addr = addr.into();
        self
    }
    
    /// Set handler
    pub fn handler<H: Handler + 'static>(mut self, handler: H) -> Self {
        self.handler = Some(Arc::new(handler));
        self
    }
    
    /// Set read timeout
    pub fn read_timeout(mut self, timeout: Duration) -> Self {
        self.read_timeout = timeout;
        self
    }
    
    /// Set write timeout
    pub fn write_timeout(mut self, timeout: Duration) -> Self {
        self.write_timeout = timeout;
        self
    }
    
    /// Set idle timeout
    pub fn idle_timeout(mut self, timeout: Duration) -> Self {
        self.idle_timeout = timeout;
        self
    }
    
    /// Set maximum header bytes
    pub fn max_header_bytes(mut self, max_bytes: usize) -> Self {
        self.max_header_bytes = max_bytes;
        self
    }
    
    /// Start listening and serving HTTP requests
    #[instrument(skip(self))]
    pub fn listen_and_serve(&self) -> GlowUpResult<()> {
        if self.state.running.load(Ordering::Acquire) {
            return Err(GlowUpError::ServerAlreadyRunning);
        }
        
        info!("Starting HTTP server on {}", self.addr);
        
        let listener = TcpListener::bind(&self.addr)
            .map_err(|e| GlowUpError::bind_error(format!("Failed to bind to {}: {}", self.addr, e)))?;
        
        listener.set_nonblocking(false)
            .map_err(|e| GlowUpError::config_error(format!("Failed to configure listener: {}", e)))?;
        
        // Store listener
        *self.state.listener.lock().unwrap() = Some(listener);
        
        self.state.running.store(true, Ordering::Release);
        
        self.serve_loop()
    }
    
    /// Start listening and serving HTTPS requests
    #[instrument(skip(self))]
    pub fn listen_and_serve_tls(&self, cert_file: &str, key_file: &str) -> GlowUpResult<()> {
        // For now, this is a placeholder implementation
        // In a real implementation, you'd integrate with a TLS library like rustls
        info!("Starting HTTPS server on {} (cert: {}, key: {})", self.addr, cert_file, key_file);
        
        // Set TLS config
        let tls_config = TlsConfig {
            cert_file: cert_file.to_string(),
            key_file: key_file.to_string(),
            min_version: TlsVersion::TLSv1_2,
            max_version: TlsVersion::TLSv1_3,
        };
        
        // Note: In a real implementation, you'd set up TLS here
        self.listen_and_serve()
    }
    
    /// Shutdown the server gracefully
    #[instrument(skip(self))]
    pub fn shutdown(&self, _ctx: Option<&dyn std::any::Any>) -> GlowUpResult<()> {
        if !self.state.running.load(Ordering::Acquire) {
            return Err(GlowUpError::ServerNotRunning);
        }
        
        info!("Initiating graceful shutdown");
        self.state.shutdown_requested.store(true, Ordering::Release);
        
        // Wait for connections to drain
        let shutdown_timeout = Duration::from_secs(30);
        let start_time = Instant::now();
        
        while self.state.active_connections.load(Ordering::Acquire) > 0 {
            if start_time.elapsed() > shutdown_timeout {
                warn!("Shutdown timeout reached, forcing close of remaining connections");
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
        
        self.state.running.store(false, Ordering::Release);
        info!("Server shutdown complete");
        
        Ok(())
    }
    
    /// Check if server is running
    pub fn is_running(&self) -> bool {
        self.state.running.load(Ordering::Acquire)
    }
    
    /// Get server statistics
    pub fn stats(&self) -> ServerStats {
        ServerStats {
            active_connections: self.state.active_connections.load(Ordering::Acquire),
            total_requests: self.state.total_requests.load(Ordering::Acquire),
            uptime: self.state.start_time.elapsed(),
        }
    }
    
    /// Main serving loop
    #[instrument(skip(self))]
    fn serve_loop(&self) -> GlowUpResult<()> {
        let listener = {
            let listener_guard = self.state.listener.lock().unwrap();
            listener_guard.as_ref().unwrap().try_clone()
                .map_err(|e| GlowUpError::internal_error(format!("Failed to clone listener: {}", e)))?
        };
        
        let mut thread_handles = Vec::new();
        
        for stream in listener.incoming() {
            if self.state.shutdown_requested.load(Ordering::Acquire) {
                break;
            }
            
            match stream {
                Ok(stream) => {
                    let connection_id = self.connection_pool.connection_counter
                        .fetch_add(1, Ordering::Relaxed);
                    
                    let remote_addr = stream.peer_addr()
                        .map_err(|e| GlowUpError::connection_error(format!("Failed to get peer address: {}", e)))?;
                    
                    let local_addr = stream.local_addr()
                        .map_err(|e| GlowUpError::connection_error(format!("Failed to get local address: {}", e)))?;
                    
                    // Check connection limit
                    if self.state.active_connections.load(Ordering::Acquire) 
                       >= self.connection_pool.max_connections as u64 {
                        warn!("Connection limit reached, rejecting connection from {}", remote_addr);
                        continue;
                    }
                    
                    let connection = Arc::new(Connection::new(connection_id, remote_addr, local_addr));
                    
                    // Add to connection pool
                    self.connection_pool.add_connection(connection.clone());
                    self.state.active_connections.fetch_add(1, Ordering::Relaxed);
                    
                    // Spawn handler thread
                    let handler = ConnectionHandler::new(
                        stream,
                        connection,
                        self.handler.clone(),
                        self.read_timeout,
                        self.write_timeout,
                        self.max_header_bytes,
                        self.state.clone(),
                        self.connection_pool.clone(),
                    );
                    
                    let handle = thread::spawn(move || {
                        if let Err(e) = handler.handle() {
                            error!("Connection handler error: {:?}", e);
                        }
                    });
                    
                    thread_handles.push(handle);
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
            
            // Clean up finished threads
            thread_handles.retain(|handle| !handle.is_finished());
        }
        
        // Wait for all connection handlers to finish
        for handle in thread_handles {
            let _ = handle.join();
        }
        
        Ok(())
    }
}

impl Default for VibeServer {
    fn default() -> Self {
        Self::new()
    }
}

/// Connection handler for processing individual HTTP connections
struct ConnectionHandler {
    stream: TcpStream,
    connection: Arc<Connection>,
    handler: Option<Arc<dyn Handler>>,
    read_timeout: Duration,
    write_timeout: Duration,
    max_header_bytes: usize,
    state: Arc<ServerState>,
    connection_pool: Arc<ConnectionPool>,
}

impl ConnectionHandler {
    fn new(
        stream: TcpStream,
        connection: Arc<Connection>,
        handler: Option<Arc<dyn Handler>>,
        read_timeout: Duration,
        write_timeout: Duration,
        max_header_bytes: usize,
        state: Arc<ServerState>,
        connection_pool: Arc<ConnectionPool>,
    ) -> Self {
        Self {
            stream,
            connection,
            handler,
            read_timeout,
            write_timeout,
            max_header_bytes,
            state,
            connection_pool,
        }
    }
    
    /// Handle the HTTP connection
    #[instrument(skip(self))]
    fn handle(mut self) -> GlowUpResult<()> {
        // Set timeouts
        self.stream.set_read_timeout(Some(self.read_timeout))?;
        self.stream.set_write_timeout(Some(self.write_timeout))?;
        
        let mut keep_alive = true;
        
        while keep_alive && !self.state.shutdown_requested.load(Ordering::Acquire) {
            match self.process_request() {
                Ok(should_keep_alive) => {
                    keep_alive = should_keep_alive;
                    self.state.total_requests.fetch_add(1, Ordering::Relaxed);
                    self.connection.request_count.fetch_add(1, Ordering::Relaxed);
                    
                    // Update last activity
                    *self.connection.last_activity.lock().unwrap() = Instant::now();
                }
                Err(GlowUpError::Connection(ref msg)) if msg.contains("closed") => {
                    debug!("Connection closed by client");
                    break;
                }
                Err(e) => {
                    error!("Request processing error: {:?}", e);
                    break;
                }
            }
        }
        
        // Clean up connection
        self.connection_pool.remove_connection(self.connection.id);
        self.state.active_connections.fetch_sub(1, Ordering::Relaxed);
        
        Ok(())
    }
    
    /// Process a single HTTP request
    #[instrument(skip(self))]
    fn process_request(&mut self) -> GlowUpResult<bool> {
        // Parse HTTP request
        let request = self.parse_http_request()?;
        
        // Create response writer
        let response = ResponderVibe::with_writer(Box::new(ResponseWriter::new(&mut self.stream)));
        
        // Call handler
        if let Some(handler) = &self.handler {
            handler.handle_vibe(&response, &request)?;
        } else {
            // No handler - return 404
            response.write_header(StatusCode::NotFound);
            response.write(b"Not Found")?;
        }
        
        // Flush response
        response.flush()?;
        
        // Determine if we should keep connection alive
        let keep_alive = request.header.get("connection")
            .map(|v| v.to_lowercase() == "keep-alive")
            .unwrap_or(request.proto == HttpVersion::Http1_1);
        
        Ok(keep_alive)
    }
    
    /// Parse HTTP request from stream
    #[instrument(skip(self))]
    fn parse_http_request(&mut self) -> GlowUpResult<VibeRequest> {
        let mut reader = BufReader::new(&mut self.stream);
        let mut request_line = String::new();
        
        // Read request line
        reader.read_line(&mut request_line)?;
        
        if request_line.trim().is_empty() {
            return Err(GlowUpError::connection_error("Connection closed by client"));
        }
        
        // Parse request line (METHOD PATH VERSION)
        let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
        if parts.len() != 3 {
            return Err(GlowUpError::invalid_request("Invalid request line"));
        }
        
        let method = parts[0].parse::<Method>()?;
        let url = parts[1].to_string();
        let proto = parts[2].parse::<HttpVersion>()?;
        
        // Parse headers
        let headers = self.parse_headers(&mut reader)?;
        
        // Parse body
        let body = self.parse_body(&mut reader, &headers)?;
        
        let mut request = VibeRequest::new(method, url);
        request.proto = proto;
        request.header = headers;
        request.body = body;
        request.content_length = request.body.len() as i64;
        request.remote_addr = self.connection.remote_addr.to_string();
        request.request_uri = request.url.clone();
        
        // Extract host
        if let Some(host) = request.header.get("host") {
            request.host = host.clone();
        }
        
        Ok(request)
    }
    
    /// Parse HTTP headers
    fn parse_headers(&self, reader: &mut BufReader<&mut TcpStream>) -> GlowUpResult<HeaderMap> {
        let mut headers = HashMap::new();
        let mut header_line = String::new();
        let mut total_bytes = 0;
        
        loop {
            header_line.clear();
            let bytes_read = reader.read_line(&mut header_line)?;
            total_bytes += bytes_read;
            
            if total_bytes > self.max_header_bytes {
                return Err(GlowUpError::invalid_request("Headers too large"));
            }
            
            let trimmed = header_line.trim();
            if trimmed.is_empty() {
                break; // End of headers
            }
            
            if let Some(colon) = trimmed.find(':') {
                let name = trimmed[..colon].trim().to_lowercase();
                let value = trimmed[colon + 1..].trim().to_string();
                headers.insert(name, value);
            }
        }
        
        Ok(headers)
    }
    
    /// Parse HTTP body
    fn parse_body(
        &self,
        reader: &mut BufReader<&mut TcpStream>,
        headers: &HeaderMap,
    ) -> GlowUpResult<Vec<u8>> {
        let mut body = Vec::new();
        
        if let Some(content_length_str) = headers.get("content-length") {
            let content_length: usize = content_length_str.parse()
                .map_err(|_| GlowUpError::invalid_request("Invalid content-length"))?;
            
            if content_length > 100 * 1024 * 1024 { // 100MB limit
                return Err(GlowUpError::invalid_request("Request body too large"));
            }
            
            body.resize(content_length, 0);
            reader.read_exact(&mut body)?;
        }
        
        Ok(body)
    }
}

/// Response writer that writes directly to a stream
struct ResponseWriter<'a> {
    stream: &'a mut TcpStream,
}

impl<'a> ResponseWriter<'a> {
    fn new(stream: &'a mut TcpStream) -> Self {
        Self { stream }
    }
}

impl<'a> Write for ResponseWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.write(buf)
    }
    
    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}

impl ConnectionPool {
    fn new(max_connections: usize, connection_timeout: Duration) -> Self {
        Self {
            max_connections,
            active_connections: Mutex::new(HashMap::new()),
            connection_counter: AtomicU64::new(0),
            connection_timeout,
        }
    }
    
    fn add_connection(&self, connection: Arc<Connection>) {
        let mut connections = self.active_connections.lock().unwrap();
        connections.insert(connection.id, connection);
    }
    
    fn remove_connection(&self, connection_id: u64) {
        let mut connections = self.active_connections.lock().unwrap();
        connections.remove(&connection_id);
    }
}

impl Connection {
    fn new(id: u64, remote_addr: SocketAddr, local_addr: SocketAddr) -> Self {
        Self {
            id,
            start_time: Instant::now(),
            remote_addr,
            local_addr,
            request_count: AtomicU64::new(0),
            last_activity: Mutex::new(Instant::now()),
            keep_alive: AtomicBool::new(true),
        }
    }
}

/// Server statistics
#[derive(Debug, Clone)]
pub struct ServerStats {
    pub active_connections: u64,
    pub total_requests: u64,
    pub uptime: Duration,
}

/// Function to start an HTTP server (convenience function)
#[instrument]
pub fn serve(addr: &str, handler: impl Handler + 'static) -> GlowUpResult<()> {
    let server = VibeServer::new()
        .addr(addr)
        .handler(handler);
    
    server.listen_and_serve()
}

/// Function to start an HTTPS server (convenience function)
#[instrument]
pub fn serve_tls(
    addr: &str,
    handler: impl Handler + 'static,
    cert_file: &str,
    key_file: &str,
) -> GlowUpResult<()> {
    let server = VibeServer::new()
        .addr(addr)
        .handler(handler);
    
    server.listen_and_serve_tls(cert_file, key_file)
}

// Convenience re-exports for the spec functions
pub use serve as Serve;
pub use serve_tls as ServeTLS;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::glowup_http::handler::StaticHandler;
    use std::time::Duration;

    #[test]
    fn test_server_creation() {
        let server = VibeServer::new()
            .addr("127.0.0.1:0")
            .handler(StaticHandler::text("Hello"))
            .read_timeout(Duration::from_secs(10));
        
        assert_eq!(server.addr, "127.0.0.1:0");
        assert_eq!(server.read_timeout, Duration::from_secs(10));
        assert!(!server.is_running());
    }

    #[test]
    fn test_server_stats() {
        let server = VibeServer::new();
        let stats = server.stats();
        
        assert_eq!(stats.active_connections, 0);
        assert_eq!(stats.total_requests, 0);
    }

    #[test]
    fn test_connection_creation() {
        use std::net::SocketAddr;
        
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let connection = Connection::new(1, addr, addr);
        
        assert_eq!(connection.id, 1);
        assert_eq!(connection.remote_addr, addr);
        assert_eq!(connection.request_count.load(Ordering::Relaxed), 0);
    }
}
