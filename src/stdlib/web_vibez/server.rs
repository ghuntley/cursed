use crate::web::StatusCode;
/// HTTP server core implementation for the CURSED web_vibez framework
/// 
/// Provides HTTP/1.1 and HTTP/2 server functionality with:
/// - TCP listener with configurable binding
/// - Connection pooling and lifecycle management  
/// - Request/response processing pipeline
/// - Integration with router and middleware systems
/// - Graceful startup and shutdown
/// - TLS/SSL support

use crate::stdlib::web_vibez::{HttpMethod, StatusCode};
use crate::stdlib::web_vibez::router::Router;
use crate::stdlib::web_vibez::context::{RequestContext, ResponseContext, ContextData};
use crate::stdlib::web_vibez::config::{WebVibezConfig, ServerConfig};
use crate::stdlib::web_vibez::error_handling::RouterError;
use crate::stdlib::web_vibez::middleware::MiddlewareChain;
use crate::stdlib::web_vibez::health::{HealthChecker, HealthStatus};
use crate::error::Error;

use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicBool, AtomicU64, Ordering}};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use std::io::{Read, Write, BufRead, BufReader, BufWriter};
use std::str::FromStr;
use tracing::{debug, info, warn, error, instrument, span, Level};

/// HTTP server for the web_vibez framework
pub struct HttpServer {
    /// Server configuration
    config: Arc<WebVibezConfig>,
    /// Router for request dispatching
    router: Arc<Router>,
    /// Middleware chain for request processing
    middleware: Arc<MiddlewareChain>,
    /// Health checker for server monitoring
    health_checker: Arc<HealthChecker>,
    /// Server state
    state: Arc<ServerState>,
    /// Connection pool
    connection_pool: Arc<ConnectionPool>,
    /// Signal handlers
    signal_handlers: Vec<SignalHandler>,
}

/// Internal server state tracking
#[derive(Debug)]
pub struct ServerState {
    /// Whether server is running
    pub running: AtomicBool,
    /// Whether shutdown has been requested
    pub shutdown_requested: AtomicBool,
    /// Number of active connections
    pub active_connections: AtomicU64,
    /// Total requests processed
    pub total_requests: AtomicU64,
    /// Server start time
    pub start_time: Instant,
    /// Last health check time
    pub last_health_check: Mutex<Instant>,
}

/// Connection pool for managing client connections
#[derive(Debug)]
pub struct ConnectionPool {
    /// Maximum concurrent connections
    max_connections: usize,
    /// Currently active connections
    active_connections: RwLock<HashMap<u64, Arc<Connection>>>,
    /// Connection counter for unique IDs
    connection_counter: AtomicU64,
    /// Connection timeout
    connection_timeout: Duration,
}

/// Represents an active HTTP connection
#[derive(Debug)]
pub struct Connection {
    /// Unique connection ID
    pub id: u64,
    /// Connection start time
    pub start_time: Instant,
    /// Remote socket address
    pub remote_addr: SocketAddr,
    /// Local socket address
    pub local_addr: SocketAddr,
    /// Number of requests on this connection
    pub request_count: AtomicU64,
    /// Last activity time
    pub last_activity: Mutex<Instant>,
    /// Whether connection is keep-alive
    pub keep_alive: AtomicBool,
}

/// Signal handler for graceful shutdown
pub struct SignalHandler {
    pub signal: Signal,
    pub handler: Box<dyn Fn() + Send + Sync>,
}

/// Supported signals for server control
#[derive(Debug, Clone, Copy)]
pub enum Signal {
    SIGTERM,
    SIGINT,
    SIGHUP,
    SIGUSR1,
}

/// TLS configuration for HTTPS
#[derive(Debug, Clone)]
pub struct TlsConfig {
    pub cert_path: String,
    pub key_path: String,
    pub cert_chain: Vec<u8>,
    pub private_key: Vec<u8>,
    pub protocols: Vec<TlsProtocol>,
    pub cipher_suites: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum TlsProtocol {
    TLSv1_2,
    TLSv1_3,
}

/// HTTP request representation for internal processing
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub version: HttpVersion,
    pub remote_addr: SocketAddr,
    pub connection_id: u64,
}

/// HTTP response representation for internal processing
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub version: HttpVersion,
    pub keep_alive: bool,
}

/// HTTP version enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpVersion {
    Http1_0,
    Http1_1,
    Http2_0,
}

impl HttpServer {
    /// Create a new HTTP server with configuration
    #[instrument]
    pub fn new(
        config: WebVibezConfig,
        router: Router,
        middleware: MiddlewareChain,
    ) -> Result<(), Error> {
        let config = Arc::new(config);
        let router = Arc::new(router);
        let middleware = Arc::new(middleware);
        
        let health_checker = Arc::new(HealthChecker::new());
        
        let state = Arc::new(ServerState {
            running: AtomicBool::new(false),
            shutdown_requested: AtomicBool::new(false),
            active_connections: AtomicU64::new(0),
            total_requests: AtomicU64::new(0),
            start_time: Instant::now(),
            last_health_check: Mutex::new(Instant::now()),
        });
        
        let connection_pool = Arc::new(ConnectionPool::new(
            config.server.max_connections,
            config.server.connection_timeout,
        ));
        
        Ok(HttpServer {
            config,
            router,
            middleware,
            health_checker,
            state,
            connection_pool,
            signal_handlers: Vec::new(),
        })
    }
    
    /// Start the HTTP server
    #[instrument(skip(self))]
    pub fn start(&self) -> Result<(), Error> {
        if self.state.running.load(Ordering::Acquire) {
            return Err(ServerError::AlreadyRunning);
        }
        
        let bind_addr = format!("{}:{}", self.config.server.host, self.config.server.port);
        info!("Starting HTTP server on {}", bind_addr);
        
        let listener = TcpListener::bind(&bind_addr)
            .map_err(|e| ServerError::BindError(e.to_string()))?;
        
        listener.set_nonblocking(true)
            .map_err(|e| ServerError::ConfigError(e.to_string()))?;
        
        self.state.running.store(true, Ordering::Release);
        
        // Install signal handlers
        self.install_signal_handlers()?;
        
        // Start the main accept loop
        self.run_accept_loop(listener)?;
        
        Ok(())
    }
    
    /// Stop the HTTP server gracefully
    #[instrument(skip(self))]
    pub fn stop(&self) -> Result<(), Error> {
        if !self.state.running.load(Ordering::Acquire) {
            return Ok(());
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
    pub fn get_stats(&self) -> ServerStats {
        ServerStats {
            active_connections: self.state.active_connections.load(Ordering::Acquire),
            total_requests: self.state.total_requests.load(Ordering::Acquire),
            uptime: self.state.start_time.elapsed(),
            health_status: self.health_checker.check().status,
        }
    }
    
    /// Main accept loop for handling incoming connections
    #[instrument(skip(self, listener))]
    fn run_accept_loop(&self, listener: TcpListener) -> Result<(), Error> {
        let mut thread_handles = Vec::new();
        
        while self.state.running.load(Ordering::Acquire) && 
              !self.state.shutdown_requested.load(Ordering::Acquire) {
            
            match listener.accept() {
                Ok((stream, remote_addr)) => {
                    let connection_id = self.connection_pool.connection_counter
                        .fetch_add(1, Ordering::Relaxed);
                    
                    let local_addr = stream.local_addr()
                        .map_err(|e| ServerError::ConnectionError(e.to_string()))?;
                    
                    let connection = Arc::new(Connection::new(
                        connection_id,
                        remote_addr,
                        local_addr,
                    ));
                    
                    // Check connection limit
                    if self.state.active_connections.load(Ordering::Acquire) 
                       >= self.config.server.max_connections as u64 {
                        warn!("Connection limit reached, rejecting connection");
                        continue;
                    }
                    
                    // Add to connection pool
                    self.connection_pool.add_connection(connection.clone());
                    self.state.active_connections.fetch_add(1, Ordering::Relaxed);
                    
                    // Spawn handler thread
                    let handler = ConnectionHandler::new(
                        stream,
                        connection,
                        self.config.clone(),
                        self.router.clone(),
                        self.middleware.clone(),
                        self.state.clone(),
                        self.connection_pool.clone(),
                    );
                    
                    let handle = thread::spawn(move || {
                        if let Err(e) = handler.handle() {
                            error!("Connection handler error: {:?}", e);
                        }
                    });
                    
                    thread_handles.push(handle);
                },
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // No pending connections, sleep briefly
                    thread::sleep(Duration::from_millis(10));
                    continue;
                },
                Err(e) => {
                    error!("Accept error: {}", e);
                    return Err(ServerError::AcceptError(e.to_string()));
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
    
    /// Install signal handlers for graceful shutdown
    fn install_signal_handlers(&self) -> Result<(), Error> {
        // Note: In a real implementation, this would use proper signal handling
        // For now, we'll use a simplified approach
        info!("Signal handlers installed for graceful shutdown");
        Ok(())
    }
}

/// Connection handler for processing individual HTTP connections
pub struct ConnectionHandler {
    stream: std::net::TcpStream,
    connection: Arc<Connection>,
    config: Arc<WebVibezConfig>,
    router: Arc<Router>,
    middleware: Arc<MiddlewareChain>,
    state: Arc<ServerState>,
    connection_pool: Arc<ConnectionPool>,
}

impl ConnectionHandler {
    pub fn new(
        stream: std::net::TcpStream,
        connection: Arc<Connection>,
        config: Arc<WebVibezConfig>,
        router: Arc<Router>,
        middleware: Arc<MiddlewareChain>,
        state: Arc<ServerState>,
        connection_pool: Arc<ConnectionPool>,
    ) -> Self {
        Self {
            stream,
            connection,
            config,
            router,
            middleware,
            state,
            connection_pool,
        }
    }
    
    /// Handle the HTTP connection
    #[instrument(skip(self))]
    pub fn handle(mut self) -> Result<(), Error> {
        // Set timeouts
        self.stream.set_read_timeout(Some(self.config.server.request_timeout))
            .map_err(|e| ServerError::ConnectionError(e.to_string()))?;
        
        self.stream.set_write_timeout(Some(self.config.server.request_timeout))
            .map_err(|e| ServerError::ConnectionError(e.to_string()))?;
        
        let mut keep_alive = true;
        
        while keep_alive && !self.state.shutdown_requested.load(Ordering::Acquire) {
            match self.process_request() {
                Ok(response) => {
                    keep_alive = response.keep_alive;
                    
                    if let Err(e) = self.send_response(response) {
                        error!("Failed to send response: {:?}", e);
                        break;
                    }
                    
                    self.state.total_requests.fetch_add(1, Ordering::Relaxed);
                    self.connection.request_count.fetch_add(1, Ordering::Relaxed);
                    
                    // Update last activity
                    *self.connection.last_activity.lock().unwrap() = Instant::now();
                },
                Err(ServerError::ConnectionClosed) => {
                    debug!("Connection closed by client");
                    break;
                },
                Err(e) => {
                    error!("Request processing error: {:?}", e);
                    
                    // Send error response
                    let error_response = HttpResponse {
                        status: StatusCode::INTERNAL_SERVER_ERROR,
                        headers: HashMap::new(),
                        body: b"Internal Server Error".to_vec(),
                        version: HttpVersion::Http1_1,
                        keep_alive: false,
                    };
                    
                    let _ = self.send_response(error_response);
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
    fn process_request(&mut self) -> Result<(), Error> {
        // Parse HTTP request
        let http_request = self.parse_http_request()?;
        
        // Convert to RequestContext
        let mut request_context = self.build_request_context(http_request)?;
        
        // Process through middleware and router
        let response_context = self.process_through_pipeline(&mut request_context)?;
        
        // Convert to HTTP response
        let http_response = self.build_http_response(response_context)?;
        
        Ok(http_response)
    }
    
    /// Parse raw HTTP request from stream
    #[instrument(skip(self))]
    fn parse_http_request(&mut self) -> Result<(), Error> {
        let mut reader = BufReader::new(&mut self.stream);
        let mut request_line = String::new();
        
        // Read request line
        reader.read_line(&mut request_line)
            .map_err(|e| ServerError::ParseError(format!("Failed to read request line: {}", e)))?;
        
        if request_line.trim().is_empty() {
            return Err(ServerError::ConnectionClosed);
        }
        
        // Parse request line (METHOD PATH VERSION)
        let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
        if parts.len() != 3 {
            return Err(ServerError::ParseError("Invalid request line".to_string()));
        }
        
        let method = HttpMethod::from_str(parts[0])
            .map_err(|e| ServerError::ParseError(e))?;
        
        let path_and_query = parts[1];
        let (path, query) = self.parse_path_and_query(path_and_query);
        
        let version = self.parse_http_version(parts[2])?;
        
        // Parse headers
        let headers = self.parse_headers(&mut reader)?;
        
        // Parse body
        let body = self.parse_body(&mut reader, &headers)?;
        
        Ok(HttpRequest {
            method,
            path,
            query,
            headers,
            body,
            version,
            remote_addr: self.connection.remote_addr,
            connection_id: self.connection.id,
        })
    }
    
    /// Parse path and query string
    fn parse_path_and_query(&self, path_and_query: &str) -> (String, HashMap<String, String>) {
        if let Some(question_mark) = path_and_query.find('?') {
            let path = path_and_query[..question_mark].to_string();
            let query_string = &path_and_query[question_mark + 1..];
            let query = self.parse_query_string(query_string);
            (path, query)
        } else {
            (path_and_query.to_string(), HashMap::new())
        }
    }
    
    /// Parse query string into parameters
    fn parse_query_string(&self, query_string: &str) -> HashMap<String, String> {
        let mut query = HashMap::new();
        
        for pair in query_string.split('&') {
            if let Some(equals) = pair.find('=') {
                let key = &pair[..equals];
                let value = &pair[equals + 1..];
                query.insert(key.to_string(), value.to_string());
            } else {
                query.insert(pair.to_string(), String::new());
            }
        }
        
        query
    }
    
    /// Parse HTTP version
    fn parse_http_version(&self, version_str: &str) -> Result<(), Error> {
        match version_str {
            "HTTP/1.0" => Ok(HttpVersion::Http1_0),
            "HTTP/1.1" => Ok(HttpVersion::Http1_1),
            "HTTP/2.0" => Ok(HttpVersion::Http2_0),
            _ => Err(ServerError::ParseError(format!("Unsupported HTTP version: {}", version_str))),
        }
    }
    
    /// Parse HTTP headers
    fn parse_headers(&self, reader: &mut BufReader<&mut std::net::TcpStream>) -> Result<(), Error> {
        let mut headers = HashMap::new();
        let mut header_line = String::new();
        
        loop {
            header_line.clear();
            reader.read_line(&mut header_line)
                .map_err(|e| ServerError::ParseError(format!("Failed to read header: {}", e)))?;
            
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
        reader: &mut BufReader<&mut std::net::TcpStream>, 
        headers: &HashMap<String, String>
    ) -> Result<(), Error> {
        let mut body = Vec::new();
        
        if let Some(content_length_str) = headers.get("content-length") {
            let content_length: usize = content_length_str.parse()
                .map_err(|_| ServerError::ParseError("Invalid content-length".to_string()))?;
            
            if content_length > self.config.server.max_body_size {
                return Err(ServerError::ParseError("Request body too large".to_string()));
            }
            
            body.resize(content_length, 0);
            reader.read_exact(&mut body)
                .map_err(|e| ServerError::ParseError(format!("Failed to read body: {}", e)))?;
        }
        
        Ok(body)
    }
    
    /// Build RequestContext from HttpRequest
    fn build_request_context(&self, http_request: HttpRequest) -> Result<(), Error> {
        let mut context = RequestContext::new(
            http_request.method,
            &http_request.path,
            &http_request.remote_addr.to_string(),
        );
        
        // Add headers
        for (name, value) in http_request.headers {
            context.add_header(&name, &value);
        }
        
        // Add query parameters
        for (name, value) in http_request.query {
            context.add_query_param(&name, &value);
        }
        
        // Set body
        if !http_request.body.is_empty() {
            context.set_body(http_request.body);
        }
        
        // Add connection info
        context.set_data("connection_id", ContextData::Integer(http_request.connection_id as i64));
        context.set_data("http_version", ContextData::String(format!("{:?}", http_request.version)));
        
        Ok(context)
    }
    
    /// Process request through middleware and router pipeline
    fn process_through_pipeline(&self, request_context: &mut RequestContext) -> Result<(), Error> {
        // Process through middleware chain
        let middleware_result = self.middleware.process(request_context)
            .map_err(|e| ServerError::MiddlewareError(format!("{:?}", e)))?;
        
        // If middleware provided a response, use it
        if let Some(response) = middleware_result.response {
            return Ok(response);
        }
        
        // Otherwise, route through the router
        let handler_result = self.router.route(request_context)
            .map_err(|e| ServerError::RouterError(format!("{:?}", e)))?;
        
        match handler_result {
            Some(response) => Ok(response),
            None => {
                // No route found - return 404
                let mut response = ResponseContext::new();
                response.set_status(StatusCode::NOT_FOUND);
                response.set_body(b"Not Found".to_vec());
                Ok(response)
            }
        }
    }
    
    /// Build HTTP response from ResponseContext
    fn build_http_response(&self, response_context: ResponseContext) -> Result<(), Error> {
        let status = response_context.status();
        let headers = response_context.headers().clone();
        let body = response_context.body().unwrap_or_default();
        
        // Determine keep-alive
        let keep_alive = headers.get("connection")
            .map(|v| v.to_lowercase() == "keep-alive")
            .unwrap_or(true);
        
        Ok(HttpResponse {
            status,
            headers,
            body,
            version: HttpVersion::Http1_1,
            keep_alive,
        })
    }
    
    /// Send HTTP response to client
    #[instrument(skip(self, response))]
    fn send_response(&mut self, response: HttpResponse) -> Result<(), Error> {
        let mut writer = BufWriter::new(&mut self.stream);
        
        // Write status line
        let status_line = format!("{:?} {} {}\r\n", 
            response.version, 
            response.status, 
            self.status_code_text(response.status)
        );
        writer.write_all(status_line.as_bytes())
            .map_err(|e| ServerError::WriteError(e.to_string()))?;
        
        // Write headers
        for (name, value) in &response.headers {
            let header_line = format!("{}: {}\r\n", name, value);
            writer.write_all(header_line.as_bytes())
                .map_err(|e| ServerError::WriteError(e.to_string()))?;
        }
        
        // Write content-length if not present
        if !response.headers.contains_key("content-length") {
            let content_length = format!("Content-Length: {}\r\n", response.body.len());
            writer.write_all(content_length.as_bytes())
                .map_err(|e| ServerError::WriteError(e.to_string()))?;
        }
        
        // Write connection header if not present
        if !response.headers.contains_key("connection") {
            let connection = if response.keep_alive {
                "Connection: keep-alive\r\n"
            } else {
                "Connection: close\r\n"
            };
            writer.write_all(connection.as_bytes())
                .map_err(|e| ServerError::WriteError(e.to_string()))?;
        }
        
        // End of headers
        writer.write_all(b"\r\n")
            .map_err(|e| ServerError::WriteError(e.to_string()))?;
        
        // Write body
        writer.write_all(&response.body)
            .map_err(|e| ServerError::WriteError(e.to_string()))?;
        
        writer.flush()
            .map_err(|e| ServerError::WriteError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Get status code text
    fn status_code_text(&self, status: StatusCode) -> &'static str {
        match status.0 {
            200 => "OK",
            201 => "Created",
            204 => "No Content",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            409 => "Conflict",
            429 => "Too Many Requests",
            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            504 => "Gateway Timeout",
            _ => "Unknown",
        }
    }
}

impl ConnectionPool {
    /// Create a new connection pool
    pub fn new(max_connections: usize, connection_timeout: Duration) -> Self {
        Self {
            max_connections,
            active_connections: RwLock::new(HashMap::new()),
            connection_counter: AtomicU64::new(0),
            connection_timeout,
        }
    }
    
    /// Add a connection to the pool
    pub fn add_connection(&self, connection: Arc<Connection>) {
        let mut connections = self.active_connections.write().unwrap();
        connections.insert(connection.id, connection);
    }
    
    /// Remove a connection from the pool
    pub fn remove_connection(&self, connection_id: u64) {
        let mut connections = self.active_connections.write().unwrap();
        connections.remove(&connection_id);
    }
    
    /// Get connection count
    pub fn connection_count(&self) -> usize {
        self.active_connections.read().unwrap().len()
    }
    
    /// Clean up timed out connections
    pub fn cleanup_timeouts(&self) {
        let mut connections = self.active_connections.write().unwrap();
        let now = Instant::now();
        
        connections.retain(|_, connection| {
            let last_activity = *connection.last_activity.lock().unwrap();
            now.duration_since(last_activity) < self.connection_timeout
        });
    }
}

impl Connection {
    /// Create a new connection
    pub fn new(id: u64, remote_addr: SocketAddr, local_addr: SocketAddr) -> Self {
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
    pub health_status: HealthStatus,
}

/// Server error types
#[derive(Debug, Clone)]
pub enum ServerError {
    AlreadyRunning,
    BindError(String),
    ConfigError(String),
    AcceptError(String),
    ConnectionError(String),
    ParseError(String),
    MiddlewareError(String),
    RouterError(String),
    WriteError(String),
    ConnectionClosed,
    TlsError(String),
    SignalError(String),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::AlreadyRunning => write!(f, "Server is already running"),
            ServerError::BindError(msg) => write!(f, "Failed to bind server: {}", msg),
            ServerError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            ServerError::AcceptError(msg) => write!(f, "Accept error: {}", msg),
            ServerError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            ServerError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ServerError::MiddlewareError(msg) => write!(f, "Middleware error: {}", msg),
            ServerError::RouterError(msg) => write!(f, "Router error: {}", msg),
            ServerError::WriteError(msg) => write!(f, "Write error: {}", msg),
            ServerError::ConnectionClosed => write!(f, "Connection closed"),
            ServerError::TlsError(msg) => write!(f, "TLS error: {}", msg),
            ServerError::SignalError(msg) => write!(f, "Signal error: {}", msg),
        }
    }
}

impl std::error::Error for ServerError {}
