//! Network functionality for request

use crate::error::CursedError;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::collections::HashMap;

/// HTTP methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Patch,
}

/// HTTP request structure
#[derive(Debug, Clone)]
pub struct HttpRequest {
    method: HttpMethod,
    url: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl HttpRequest {
    pub fn new(method: HttpMethod, url: String) -> Self {
        Self {
            method,
            url,
            headers: HashMap::new(),
            body: None,
        }
    }
}

/// Request builder for constructing HTTP requests
#[derive(Debug, Default)]
pub struct RequestBuilder {
    method: Option<HttpMethod>,
    url: Option<String>,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn method(mut self, method: HttpMethod) -> Self {
        self.method = Some(method);
        self
    }
    
    pub fn url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }
    
    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
    
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }
    
    pub fn build(self) -> Result<HttpRequest, CursedError> {
        let method = self.method.ok_or_else(|| CursedError::Parse("Method required".to_string()))?;
        let url = self.url.ok_or_else(|| CursedError::Parse("URL required".to_string()))?;
        
        Ok(HttpRequest {
            method,
            url,
            headers: self.headers,
            body: self.body,
        })
    }
}

/// Result type for network operations
pub type NetworkResult<T> = Result<T, CursedError>;

/// Network operations handler
pub struct NetworkHandler {
    timeout_seconds: u64,
}

impl NetworkHandler {
    /// Create a new network handler
    pub fn new() -> Self {
        Self {
            timeout_seconds: 30,
        }
    }
    
    /// Set timeout
    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }
    
    /// Parse IP address
    pub fn parse_ip(&self, ip_str: &str) -> NetworkResult<IpAddr> {
        ip_str.parse().map_err(|e| CursedError::runtime_error(&format!("IP parse error: {}", e)))
    }
    
    /// Parse socket address
    pub fn parse_socket_addr(&self, addr_str: &str) -> NetworkResult<SocketAddr> {
        addr_str.parse().map_err(|e| CursedError::runtime_error(&format!("Socket address parse error: {}", e)))
    }
    
    /// Get localhost IP
    pub fn localhost_ip(&self) -> IpAddr {
        IpAddr::V4(Ipv4Addr::LOCALHOST)
    }
    
    /// Check if IP is localhost
    pub fn is_localhost(&self, ip: &IpAddr) -> bool {
        match ip {
            IpAddr::V4(ipv4) => ipv4.is_loopback(),
            IpAddr::V6(ipv6) => ipv6.is_loopback(),
        }
    }
    
    /// Create socket address
    pub fn create_socket_addr(&self, ip: IpAddr, port: u16) -> SocketAddr {
        SocketAddr::new(ip, port)
    }
}

impl Default for NetworkHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize network processing
pub fn init_request() -> NetworkResult<()> {
    let handler = NetworkHandler::new();
    let localhost = handler.localhost_ip();
    if !handler.is_localhost(&localhost) {
        return Err(CursedError::runtime_error("Network localhost test failed"));
    }
    println!("🌐 Network processing (request) initialized");
    Ok(())
}

/// Test network functionality
pub fn test_request() -> NetworkResult<()> {
    let handler = NetworkHandler::new();
    let ip = handler.parse_ip("127.0.0.1")?;
    let socket_addr = handler.create_socket_addr(ip, 8080);
    if socket_addr.port() != 8080 {
        return Err(CursedError::runtime_error("Network socket test failed"));
    }
    Ok(())
}
