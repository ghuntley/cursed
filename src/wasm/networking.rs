//! WASM networking stub
//! 
//! Provides stub implementations for networking operations in WASM
//! where standard networking is not available.

use std::io::{Error, ErrorKind, Result};

/// WASM networking stub that provides no-op implementations
/// for networking operations that are not available in WASM
pub struct WasmNetworking;

impl WasmNetworking {
    pub fn new() -> Self {
        Self
    }

    /// Stub for TCP listener - not available in WASM
    pub fn listen_tcp(&self, _addr: &str) -> Result<()> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "TCP listening not supported in WASM"
        ))
    }

    /// Stub for TCP connection - use fetch API instead
    pub fn connect_tcp(&self, _addr: &str) -> Result<()> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "Direct TCP connections not supported in WASM - use fetch API or WebSockets"
        ))
    }

    /// Stub for UDP socket - not available in WASM
    pub fn bind_udp(&self, _addr: &str) -> Result<()> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "UDP sockets not supported in WASM"
        ))
    }

    /// WASM HTTP request using fetch API (placeholder)
    #[cfg(target_arch = "wasm32")]
    pub fn http_request(&self, url: &str, method: &str, body: Option<&[u8]>) -> Result<Vec<u8>> {
        // This would use wasm-bindgen and web-sys to make fetch requests
        // For now, return an error indicating the feature needs implementation
        Err(Error::new(
            ErrorKind::Unimplemented,
            &format!("HTTP {} request to {} not yet implemented in WASM", method, url)
        ))
    }

    /// Native HTTP request fallback
    #[cfg(not(target_arch = "wasm32"))]
    pub fn http_request(&self, _url: &str, _method: &str, _body: Option<&[u8]>) -> Result<Vec<u8>> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "Use native networking for non-WASM targets"
        ))
    }
}

/// Conditional compilation wrapper for networking operations
#[cfg(target_arch = "wasm32")]
pub fn create_tcp_listener(_addr: &str) -> Result<()> {
    Err(Error::new(
        ErrorKind::Unsupported,
        "TCP listeners not supported in WASM"
    ))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn create_tcp_listener(addr: &str) -> Result<std::net::TcpListener> {
    std::net::TcpListener::bind(addr)
}

#[cfg(target_arch = "wasm32")]
pub fn create_tcp_stream(_addr: &str) -> Result<()> {
    Err(Error::new(
        ErrorKind::Unsupported,
        "TCP streams not supported in WASM - use WebSockets or fetch API"
    ))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn create_tcp_stream(addr: &str) -> Result<std::net::TcpStream> {
    std::net::TcpStream::connect(addr)
}

#[cfg(target_arch = "wasm32")]
pub fn create_udp_socket(_addr: &str) -> Result<()> {
    Err(Error::new(
        ErrorKind::Unsupported,
        "UDP sockets not supported in WASM"
    ))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn create_udp_socket(addr: &str) -> Result<std::net::UdpSocket> {
    std::net::UdpSocket::bind(addr)
}
