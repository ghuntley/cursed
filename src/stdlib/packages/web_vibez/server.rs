/// fr fr HTTP server implementation for web_vibez - the main server engine
use std::sync::Arc;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io::{Read, Write, BufReader, BufRead};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

use crate::stdlib::packages::web_vibez::{
    request::{HttpRequest, RequestBuilder},
    response::HttpResponse,
    router::Router,
    handler::Handler,
    middleware::MiddlewareChain,
    method::HttpMethod,
    types::{RequestBody, ContentType},
    error::{WebError, WebResult, NetworkErrorKind},
    status::StatusCode,
};

/// fr fr Server configuration options - comprehensive setup
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// fr fr Server bind address
    pub address: SocketAddr,
    /// fr fr Maximum concurrent connections
    pub max_connections: usize,
    /// fr fr Connection timeout duration
    pub connection_timeout: Duration,
    /// fr fr Read timeout for requests
    pub read_timeout: Duration,
    /// fr fr Write timeout for responses
    pub write_timeout: Duration,
    /// fr fr Maximum request body size in bytes
    pub max_body_size: usize,
    /// fr fr Server name for headers
    pub server_name: String,
    /// fr fr Enable keep-alive connections
    pub keep_alive: bool,
    /// fr fr Keep-alive timeout
    pub keep_alive_timeout: Duration,
    /// fr fr Thread pool size
    pub thread_pool_size: usize,
}

impl ServerConfig {
    /// fr fr Create new server config with defaults - basic setup
    pub fn new(address: SocketAddr) -> Self {
        Self {
            address,
            max_connections: 1000,
            connection_timeout: Duration::from_secs(30),
            read_timeout: Duration::from_secs(30),
            write_timeout: Duration::from_secs(30),
            max_body_size: 1024 * 1024 * 16, // 16MB
            server_name: "web_vibez/1.0".to_string(),
            keep_alive: true,
            keep_alive_timeout: Duration::from_secs(60),
            thread_pool_size: num_cpus::get() * 2,
        }
    }

    /// fr fr Set maximum connections - concurrency limit
    pub fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }

    /// fr fr Set connection timeout - how long to wait
    pub fn connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self
    }

    /// fr fr Set maximum body size - request size limit
    pub fn max_body_size(mut self, size: usize) -> Self {
        self.max_body_size = size;
        self
    }

    /// fr fr Set server name - identification header
    pub fn server_name(mut self, name: String) -> Self {
        self.server_name = name;
        self
    }

    /// fr fr Enable/disable keep-alive - connection reuse
    pub fn keep_alive(mut self, enable: bool) -> Self {
        self.keep_alive = enable;
        self
    }

    /// fr fr Set thread pool size - worker threads
    pub fn thread_pool_size(mut self, size: usize) -> Self {
        self.thread_pool_size = size;
        self
    }
}

/// fr fr HTTP server implementation - the main server engine
pub struct HttpServer {
    config: ServerConfig,
    router: Arc<Router>,
    middleware_chain: MiddlewareChain,
    listener: Option<TcpListener>,
    running: Arc<std::sync::atomic::AtomicBool>,
}

impl HttpServer {
    /// fr fr Create new HTTP server - basic setup
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            router: Arc::new(Router::new()),
            middleware_chain: MiddlewareChain::new(),
            listener: None,
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    /// fr fr Set router for the server - routing configuration
    pub fn with_router(mut self, router: Router) -> Self {
        self.router = Arc::new(router);
        self
    }

    /// fr fr Set middleware chain - request processing pipeline
    pub fn with_middleware(mut self, middleware_chain: MiddlewareChain) -> Self {
        self.middleware_chain = middleware_chain;
        self
    }

    /// fr fr Start the server - begin accepting connections
    pub fn start(&mut self) -> WebResult<()> {
        // Create TCP listener
        let listener = TcpListener::bind(self.config.address).map_err(|e| {
            WebError::network(
                NetworkErrorKind::Other,
                format!("Failed to bind to {}: {}", self.config.address, e),
            )
        })?;

        println!("🚀 Server starting on {} - let's get this bread!", self.config.address);

        // Set non-blocking mode for the listener
        listener.set_nonblocking(true).map_err(|e| {
            WebError::network(
                NetworkErrorKind::Other,
                format!("Failed to set non-blocking mode: {}", e),
            )
        })?;

        self.listener = Some(listener);
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);

        // Start accepting connections
        self.accept_loop()
    }

    /// fr fr Stop the server - graceful shutdown
    pub fn stop(&mut self) {
        println!("🛑 Server stopping - peace out!");
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    /// fr fr Check if server is running - status check
    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// fr fr Main connection acceptance loop - handle incoming connections
    fn accept_loop(&self) -> WebResult<()> {
        let listener = self.listener.as_ref().ok_or_else(|| {
            WebError::Configuration {
                setting: "listener".to_string(),
                value: "None".to_string(),
                message: "Server not properly initialized".to_string(),
            }
        })?;

        let mut active_connections = 0;

        while self.is_running() {
            match listener.accept() {
                Ok((stream, client_addr)) => {
                    if active_connections >= self.config.max_connections {
                        println!("⚠️ Max connections reached, dropping new connection from {}", client_addr);
                        drop(stream);
                        continue;
                    }

                    active_connections += 1;
                    println!("📞 New connection from {} (active: {})", client_addr, active_connections);

                    // Handle connection in thread pool
                    let router = self.router.clone();
                    let middleware_chain = self.middleware_chain.clone();
                    let config = self.config.clone();
                    let running = self.running.clone();

                    thread::spawn(move || {
                        let result = Self::handle_connection(stream, client_addr, router, middleware_chain, config);
                        if let Err(e) = result {
                            println!("❌ Connection error from {}: {}", client_addr, e);
                        }
                        // Note: In a real implementation, you'd decrement active_connections here
                        // This would require shared state or a channel
                    });
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // No new connections, sleep briefly
                    thread::sleep(Duration::from_millis(10));
                }
                Err(e) => {
                    println!("❌ Accept error: {}", e);
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }

        Ok(())
    }

    /// fr fr Handle individual connection - process requests
    fn handle_connection(
        mut stream: TcpStream,
        client_addr: SocketAddr,
        router: Arc<Router>,
        middleware_chain: MiddlewareChain,
        config: ServerConfig,
    ) -> WebResult<()> {
        // Set timeouts
        stream.set_read_timeout(Some(config.read_timeout)).ok();
        stream.set_write_timeout(Some(config.write_timeout)).ok();

        let mut buf_reader = BufReader::new(&stream);
        
        loop {
            // Parse HTTP request
            let request = match Self::parse_request(&mut buf_reader, client_addr, &config) {
                Ok(req) => req,
                Err(e) => {
                    // Send error response and close connection
                    let error_response = HttpResponse::from_error(&e);
                    let _ = Self::send_response(&mut stream, error_response, &config);
                    break;
                }
            };

            // Handle the request through router and middleware
            let response = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async {
                    // Process through middleware
                    let mut req_copy = request.clone();
                    if let Err(e) = middleware_chain.process_request(&mut req_copy).await {
                        return HttpResponse::from_error(&e);
                    }

                    // Route the request
                    let mut response = match router.route_request(req_copy.clone()).await {
                        Ok(resp) => resp,
                        Err(e) => HttpResponse::from_error(&e),
                    };

                    // Process response through middleware
                    if let Err(e) = middleware_chain.process_response(&req_copy, &mut response).await {
                        return HttpResponse::from_error(&e);
                    }

                    response
                });

            // Send response
            if let Err(e) = Self::send_response(&mut stream, response, &config) {
                println!("❌ Failed to send response to {}: {}", client_addr, e);
                break;
            }

            // Check for connection close
            if let Some(connection) = request.header("connection") {
                if connection.to_lowercase() == "close" || !config.keep_alive {
                    break;
                }
            }

            // For HTTP/1.0, close by default unless keep-alive
            if request.version == "HTTP/1.0" {
                if let Some(connection) = request.header("connection") {
                    if connection.to_lowercase() != "keep-alive" {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        Ok(())
    }

    /// fr fr Parse HTTP request from stream - request parsing
    fn parse_request(
        reader: &mut BufReader<&TcpStream>,
        client_addr: SocketAddr,
        config: &ServerConfig,
    ) -> WebResult<HttpRequest> {
        // Read request line
        let mut request_line = String::new();
        reader.read_line(&mut request_line).map_err(|e| {
            WebError::RequestParsing {
                message: format!("Failed to read request line: {}", e),
                field: Some("request_line".to_string()),
            }
        })?;

        // Parse request line: "METHOD /path HTTP/1.1"
        let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
        if parts.len() != 3 {
            return Err(WebError::bad_request("Invalid request line format"));
        }

        let method = parts[0].parse::<HttpMethod>().map_err(|_| {
            WebError::bad_request(format!("Invalid HTTP method: {}", parts[0]))
        })?;

        let path_and_query = parts[1];
        let version = parts[2].to_string();

        // Parse path and query
        let (path, query) = if let Some(pos) = path_and_query.find('?') {
            let path = path_and_query[..pos].to_string();
            let query_string = &path_and_query[pos + 1..];
            let query = Self::parse_query_string(query_string);
            (path, query)
        } else {
            (path_and_query.to_string(), HashMap::new())
        };

        // Parse headers
        let mut headers = HashMap::new();
        loop {
            let mut header_line = String::new();
            reader.read_line(&mut header_line).map_err(|e| {
                WebError::RequestParsing {
                    message: format!("Failed to read header: {}", e),
                    field: Some("headers".to_string()),
                }
            })?;

            let header_line = header_line.trim();
            if header_line.is_empty() {
                break; // End of headers
            }

            if let Some(pos) = header_line.find(':') {
                let name = header_line[..pos].trim().to_lowercase();
                let value = header_line[pos + 1..].trim().to_string();
                headers.insert(name, value);
            }
        }

        // Parse body if present
        let body = if let Some(content_length) = headers.get("content-length") {
            let length: usize = content_length.parse().map_err(|_| {
                WebError::bad_request("Invalid Content-Length header")
            })?;

            if length > config.max_body_size {
                return Err(WebError::RequestParsing {
                    message: format!("Request body too large: {} > {}", length, config.max_body_size),
                    field: Some("body".to_string()),
                });
            }

            if length > 0 {
                let mut body_bytes = vec![0; length];
                reader.read_exact(&mut body_bytes).map_err(|e| {
                    WebError::RequestParsing {
                        message: format!("Failed to read request body: {}", e),
                        field: Some("body".to_string()),
                    }
                })?;

                // Determine body type based on content-type
                if let Some(content_type) = headers.get("content-type") {
                    if content_type.starts_with("application/json") {
                        let body_text = String::from_utf8_lossy(&body_bytes);
                        match serde_json::from_str(&body_text) {
                            Ok(json_value) => RequestBody::Json(json_value),
                            Err(_) => RequestBody::Text(body_text.to_string()),
                        }
                    } else if content_type.starts_with("application/x-www-form-urlencoded") {
                        let body_text = String::from_utf8_lossy(&body_bytes);
                        let form_data = Self::parse_form_data(&body_text);
                        RequestBody::Form(form_data)
                    } else if content_type.starts_with("text/") {
                        RequestBody::Text(String::from_utf8_lossy(&body_bytes).to_string())
                    } else {
                        RequestBody::Binary(body_bytes)
                    }
                } else {
                    // Default to text
                    RequestBody::Text(String::from_utf8_lossy(&body_bytes).to_string())
                }
            } else {
                RequestBody::Empty
            }
        } else {
            RequestBody::Empty
        };

        // Create request
        let request = RequestBuilder::new(method, path)
            .query_map(query)
            .headers_map(headers)
            .body(body)
            .remote_addr(client_addr)
            .version(version)
            .build();

        Ok(request)
    }

    /// fr fr Parse query string parameters - URL parameter extraction
    fn parse_query_string(query_string: &str) -> HashMap<String, String> {
        let mut params = HashMap::new();
        
        for pair in query_string.split('&') {
            if let Some(pos) = pair.find('=') {
                let key = pair[..pos].to_string();
                let value = pair[pos + 1..].to_string();
                params.insert(key, value);
            } else if !pair.is_empty() {
                params.insert(pair.to_string(), String::new());
            }
        }
        
        params
    }

    /// fr fr Parse form data - form field extraction
    fn parse_form_data(form_string: &str) -> HashMap<String, String> {
        // For now, same as query string parsing
        // In a real implementation, you'd handle URL decoding
        Self::parse_query_string(form_string)
    }

    /// fr fr Send HTTP response to client - response transmission
    fn send_response(
        stream: &mut TcpStream,
        mut response: HttpResponse,
        config: &ServerConfig,
    ) -> WebResult<()> {
        // Add server header
        if !response.headers.contains_key("server") {
            response.headers.insert("server".to_string(), config.server_name.clone());
        }

        // Add date header
        if !response.headers.contains_key("date") {
            response.headers.insert(
                "date".to_string(),
                chrono::Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string(),
            );
        }

        // Add connection header
        if !response.headers.contains_key("connection") {
            let connection = if config.keep_alive { "keep-alive" } else { "close" };
            response.headers.insert("connection".to_string(), connection.to_string());
        }

        // Send response
        let http_response = response.to_http_string();
        stream.write_all(http_response.as_bytes()).map_err(|e| {
            WebError::network(
                NetworkErrorKind::Other,
                format!("Failed to write response: {}", e),
            )
        })?;

        stream.flush().map_err(|e| {
            WebError::network(
                NetworkErrorKind::Other,
                format!("Failed to flush response: {}", e),
            )
        })?;

        Ok(())
    }
}

// Helper trait extensions for RequestBuilder
trait RequestBuilderExt {
    fn query_map(self, query: HashMap<String, String>) -> Self;
    fn headers_map(self, headers: HashMap<String, String>) -> Self;
}

impl RequestBuilderExt for RequestBuilder {
    fn query_map(mut self, query: HashMap<String, String>) -> Self {
        for (key, value) in query {
            self = self.query(key, value);
        }
        self
    }

    fn headers_map(mut self, headers: HashMap<String, String>) -> Self {
        for (key, value) in headers {
            self = self.header(key, value);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_server_config() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let config = ServerConfig::new(addr)
            .max_connections(500)
            .server_name("test-server/1.0".to_string());

        assert_eq!(config.address, addr);
        assert_eq!(config.max_connections, 500);
        assert_eq!(config.server_name, "test-server/1.0");
    }

    #[test]
    fn test_parse_query_string() {
        let query = "name=john&age=30&city=boston";
        let params = HttpServer::parse_query_string(query);
        
        assert_eq!(params.get("name"), Some(&"john".to_string()));
        assert_eq!(params.get("age"), Some(&"30".to_string()));
        assert_eq!(params.get("city"), Some(&"boston".to_string()));
    }

    #[test]
    fn test_server_creation() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let config = ServerConfig::new(addr);
        let server = HttpServer::new(config);
        
        assert!(!server.is_running());
    }
}
