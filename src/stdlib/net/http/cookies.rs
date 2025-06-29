//! Network functionality for cookies

use crate::error::CursedError;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

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
pub fn init_cookies() -> NetworkResult<()> {
    let handler = NetworkHandler::new();
    let localhost = handler.localhost_ip();
    if !handler.is_localhost(&localhost) {
        return Err(CursedError::runtime_error("Network localhost test failed"));
    }
    println!("🌐 Network processing (cookies) initialized");
    Ok(())
}

/// Test network functionality
pub fn test_cookies() -> NetworkResult<()> {
    let handler = NetworkHandler::new();
    let ip = handler.parse_ip("127.0.0.1")?;
    let socket_addr = handler.create_socket_addr(ip, 8080);
    if socket_addr.port() != 8080 {
        return Err(CursedError::runtime_error("Network socket test failed"));
    }
    Ok(())
}
