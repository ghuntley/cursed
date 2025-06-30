use crate::error::CursedError;
use std::net::SocketAddr;

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub addr: SocketAddr,
    pub max_connections: usize,
    pub timeout: std::time::Duration,
}

impl ServerConfig {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            max_connections: 1000,
            timeout: std::time::Duration::from_secs(30),
        }
    }
}

/// HTTP Server
pub struct HttpServer {
    config: ServerConfig,
}

impl HttpServer {
    pub fn new(config: ServerConfig) -> Self {
        Self { config }
    }
    
    pub fn addr(&self) -> SocketAddr {
        self.config.addr
    }
}
