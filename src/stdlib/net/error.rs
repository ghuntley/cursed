/// Networking error types and utilities for the CURSED networking module
/// 
/// This module provides comprehensive error handling for all networking operations
/// including socket errors, DNS failures, HTTP errors, protocol errors, and
/// integration with the existing CURSED error system.

use std::fmt;
use std::io;
use std::net::AddrParseError;
use crate::error::{CursedError, ErrorContext};

/// Result type for networking operations
pub type NetResult<T> = Result<T, NetError>;

/// Comprehensive networking error types
#[derive(Debug, Clone)]
pub enum NetError {
    /// Connection-related errors
    Connection {
        message: String,
        address: Option<String>,
        port: Option<u16>,
    },
    
    /// Socket operation errors
    Socket {
        operation: String,
        message: String,
        error_code: Option<i32>,
    },
    
    /// DNS resolution errors
    Dns {
        hostname: String,
        message: String,
        record_type: Option<String>,
    },
    
    /// Protocol-specific errors
    Protocol {
        protocol: String,
        message: String,
        details: Option<String>,
    },
    
    /// HTTP-specific errors
    Http {
        status_code: Option<u16>,
        message: String,
        url: Option<String>,
    },
    
    /// WebSocket-specific errors
    WebSocket {
        message: String,
        close_code: Option<u16>,
        url: Option<String>,
    },
    
    /// Timeout errors
    Timeout {
        operation: String,
        duration_ms: u64,
        message: String,
    },
    
    /// Authentication and security errors
    Security {
        message: String,
        context: String,
    },
    
    /// Address parsing and validation errors
    Address {
        input: String,
        message: String,
    },
    
    /// TLS/SSL errors
    Tls {
        message: String,
        certificate_info: Option<String>,
    },
    
    /// System-level networking errors
    System {
        code: i32,
        message: String,
    },
    
    /// Invalid configuration or parameters
    InvalidConfig {
        parameter: String,
        value: String,
        message: String,
    },
    
    /// Resource exhaustion (too many connections, etc.)
    ResourceExhausted {
        resource: String,
        limit: u64,
        current: u64,
    },
    
    /// General networking error with context
    General {
        message: String,
        context: Option<String>,
    },
}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetError::Connection { message, address, port } => {
                write!(f, "Connection error: {}", message)?;
                if let Some(addr) = address {
                    write!(f, " (address: {})", addr)?;
                }
                if let Some(p) = port {
                    write!(f, " (port: {})", p)?;
                }
                Ok(())
            },
            
            NetError::Socket { operation, message, error_code } => {
                write!(f, "Socket error during {}: {}", operation, message)?;
                if let Some(code) = error_code {
                    write!(f, " (code: {})", code)?;
                }
                Ok(())
            },
            
            NetError::Dns { hostname, message, record_type } => {
                write!(f, "DNS error for '{}': {}", hostname, message)?;
                if let Some(rtype) = record_type {
                    write!(f, " (record type: {})", rtype)?;
                }
                Ok(())
            },
            
            NetError::Protocol { protocol, message, details } => {
                write!(f, "{} protocol error: {}", protocol, message)?;
                if let Some(d) = details {
                    write!(f, " ({})", d)?;
                }
                Ok(())
            },
            
            NetError::Http { status_code, message, url } => {
                write!(f, "HTTP error: {}", message)?;
                if let Some(code) = status_code {
                    write!(f, " (status: {})", code)?;
                }
                if let Some(u) = url {
                    write!(f, " (url: {})", u)?;
                }
                Ok(())
            },
            
            NetError::WebSocket { message, close_code, url } => {
                write!(f, "WebSocket error: {}", message)?;
                if let Some(code) = close_code {
                    write!(f, " (close code: {})", code)?;
                }
                if let Some(u) = url {
                    write!(f, " (url: {})", u)?;
                }
                Ok(())
            },
            
            NetError::Timeout { operation, duration_ms, message } => {
                write!(f, "Timeout during {}: {} ({}ms)", operation, message, duration_ms)
            },
            
            NetError::Security { message, context } => {
                write!(f, "Security error in {}: {}", context, message)
            },
            
            NetError::Address { input, message } => {
                write!(f, "Address error for '{}': {}", input, message)
            },
            
            NetError::Tls { message, certificate_info } => {
                write!(f, "TLS error: {}", message)?;
                if let Some(cert_info) = certificate_info {
                    write!(f, " (certificate: {})", cert_info)?;
                }
                Ok(())
            },
            
            NetError::System { code, message } => {
                write!(f, "System error {}: {}", code, message)
            },
            
            NetError::InvalidConfig { parameter, value, message } => {
                write!(f, "Invalid configuration for '{}' = '{}': {}", parameter, value, message)
            },
            
            NetError::ResourceExhausted { resource, limit, current } => {
                write!(f, "Resource exhausted: {} ({}/{} used)", resource, current, limit)
            },
            
            NetError::General { message, context } => {
                write!(f, "Network error: {}", message)?;
                if let Some(ctx) = context {
                    write!(f, " ({})", ctx)?;
                }
                Ok(())
            },
        }
    }
}

impl std::error::Error for NetError {}

// Conversion from standard library errors
impl From<io::Error> for NetError {
    fn from(error: io::Error) -> Self {
        NetError::System {
            code: error.raw_os_error().unwrap_or(-1),
            message: error.to_string(),
        }
    }
}

impl From<AddrParseError> for NetError {
    fn from(error: AddrParseError) -> Self {
        NetError::Address {
            input: "unknown".to_string(),
            message: error.to_string(),
        }
    }
}

// Integration with CURSED error system
impl From<NetError> for CursedError {
    fn from(error: NetError) -> Self {
        CursedError::Runtime {
            message: error.to_string(),
            context: Some(ErrorContext {
                file: "network".to_string(),
                line: 0,
                column: 0,
                function: Some("networking".to_string()),
                details: Some(format!("Network error: {:?}", error)),
            }),
        }
    }
}

impl From<CursedError> for NetError {
    fn from(error: CursedError) -> Self {
        NetError::General {
            message: error.to_string(),
            context: Some("CURSED runtime error".to_string()),
        }
    }
}

/// Helper functions for creating specific error types

/// Create a connection error
pub fn connection_error(message: &str) -> NetError {
    NetError::Connection {
        message: message.to_string(),
        address: None,
        port: None,
    }
}

/// Create a connection error with address details
pub fn connection_error_with_address(message: &str, address: &str, port: Option<u16>) -> NetError {
    NetError::Connection {
        message: message.to_string(),
        address: Some(address.to_string()),
        port,
    }
}

/// Create a socket operation error
pub fn socket_error(operation: &str, message: &str) -> NetError {
    NetError::Socket {
        operation: operation.to_string(),
        message: message.to_string(),
        error_code: None,
    }
}

/// Create a socket error with error code
pub fn socket_error_with_code(operation: &str, message: &str, code: i32) -> NetError {
    NetError::Socket {
        operation: operation.to_string(),
        message: message.to_string(),
        error_code: Some(code),
    }
}

/// Create a DNS resolution error
pub fn dns_error(hostname: &str) -> NetError {
    NetError::Dns {
        hostname: hostname.to_string(),
        message: "DNS resolution failed".to_string(),
        record_type: None,
    }
}

/// Create a DNS error with record type
pub fn dns_error_with_type(hostname: &str, message: &str, record_type: &str) -> NetError {
    NetError::Dns {
        hostname: hostname.to_string(),
        message: message.to_string(),
        record_type: Some(record_type.to_string()),
    }
}

/// Create a protocol error
pub fn protocol_error(protocol: &str) -> NetError {
    NetError::Protocol {
        protocol: protocol.to_string(),
        message: "Protocol error".to_string(),
        details: None,
    }
}

/// Create a protocol error with details
pub fn protocol_error_with_details(protocol: &str, message: &str, details: &str) -> NetError {
    NetError::Protocol {
        protocol: protocol.to_string(),
        message: message.to_string(),
        details: Some(details.to_string()),
    }
}

/// Create an HTTP error
pub fn http_error(message: &str) -> NetError {
    NetError::Http {
        status_code: None,
        message: message.to_string(),
        url: None,
    }
}

/// Create an HTTP error with status code
pub fn http_error_with_status(message: &str, status_code: u16, url: Option<&str>) -> NetError {
    NetError::Http {
        status_code: Some(status_code),
        message: message.to_string(),
        url: url.map(|s| s.to_string()),
    }
}

/// Create a WebSocket error
pub fn websocket_error(message: &str) -> NetError {
    NetError::WebSocket {
        message: message.to_string(),
        close_code: None,
        url: None,
    }
}

/// Create a WebSocket error with close code
pub fn websocket_error_with_code(message: &str, close_code: u16, url: Option<&str>) -> NetError {
    NetError::WebSocket {
        message: message.to_string(),
        close_code: Some(close_code),
        url: url.map(|s| s.to_string()),
    }
}

/// Create a timeout error
pub fn timeout_error(operation: &str) -> NetError {
    NetError::Timeout {
        operation: operation.to_string(),
        duration_ms: 0,
        message: "Operation timed out".to_string(),
    }
}

/// Create a timeout error with duration
pub fn timeout_error_with_duration(operation: &str, duration_ms: u64, message: &str) -> NetError {
    NetError::Timeout {
        operation: operation.to_string(),
        duration_ms,
        message: message.to_string(),
    }
}

/// Create a security error
pub fn security_error(context: &str, message: &str) -> NetError {
    NetError::Security {
        message: message.to_string(),
        context: context.to_string(),
    }
}

/// Create an address parsing error
pub fn address_error(input: &str, message: &str) -> NetError {
    NetError::Address {
        input: input.to_string(),
        message: message.to_string(),
    }
}

/// Create a TLS error
pub fn tls_error(message: &str) -> NetError {
    NetError::Tls {
        message: message.to_string(),
        certificate_info: None,
    }
}

/// Create a TLS error with certificate information
pub fn tls_error_with_cert(message: &str, certificate_info: &str) -> NetError {
    NetError::Tls {
        message: message.to_string(),
        certificate_info: Some(certificate_info.to_string()),
    }
}

/// Create a system error
pub fn system_error(code: i32, message: &str) -> NetError {
    NetError::System {
        code,
        message: message.to_string(),
    }
}

/// Create an invalid configuration error
pub fn invalid_config_error(parameter: &str, value: &str, message: &str) -> NetError {
    NetError::InvalidConfig {
        parameter: parameter.to_string(),
        value: value.to_string(),
        message: message.to_string(),
    }
}

/// Create a resource exhausted error
pub fn resource_exhausted_error(resource: &str, limit: u64, current: u64) -> NetError {
    NetError::ResourceExhausted {
        resource: resource.to_string(),
        limit,
        current,
    }
}

/// Create a general networking error
pub fn general_error(message: &str) -> NetError {
    NetError::General {
        message: message.to_string(),
        context: None,
    }
}

/// Create a general error with context
pub fn general_error_with_context(message: &str, context: &str) -> NetError {
    NetError::General {
        message: message.to_string(),
        context: Some(context.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = connection_error("Failed to connect");
        assert!(matches!(err, NetError::Connection { .. }));
        assert!(err.to_string().contains("Failed to connect"));
    }

    #[test]
    fn test_error_with_details() {
        let err = connection_error_with_address("Connection refused", "127.0.0.1", Some(8080));
        assert!(matches!(err, NetError::Connection { .. }));
        let err_str = err.to_string();
        assert!(err_str.contains("Connection refused"));
        assert!(err_str.contains("127.0.0.1"));
        assert!(err_str.contains("8080"));
    }

    #[test]
    fn test_dns_error() {
        let err = dns_error_with_type("example.com", "No such host", "A");
        assert!(matches!(err, NetError::Dns { .. }));
        let err_str = err.to_string();
        assert!(err_str.contains("example.com"));
        assert!(err_str.contains("No such host"));
        assert!(err_str.contains("A"));
    }

    #[test]
    fn test_http_error() {
        let err = http_error_with_status("Not Found", 404, Some("https://example.com"));
        assert!(matches!(err, NetError::Http { .. }));
        let err_str = err.to_string();
        assert!(err_str.contains("Not Found"));
        assert!(err_str.contains("404"));
        assert!(err_str.contains("https://example.com"));
    }

    #[test]
    fn test_timeout_error() {
        let err = timeout_error_with_duration("connect", 5000, "Connection timed out");
        assert!(matches!(err, NetError::Timeout { .. }));
        let err_str = err.to_string();
        assert!(err_str.contains("connect"));
        assert!(err_str.contains("5000ms"));
        assert!(err_str.contains("Connection timed out"));
    }

    #[test]
    fn test_error_conversion_from_io() {
        let io_err = io::Error::new(io::ErrorKind::ConnectionRefused, "Connection refused");
        let net_err = NetError::from(io_err);
        assert!(matches!(net_err, NetError::System { .. }));
    }

    #[test]
    fn test_error_conversion_to_cursed() {
        let net_err = connection_error("Test error");
        let cursed_err = CursedError::from(net_err);
        assert!(matches!(cursed_err, CursedError::Runtime { .. }));
    }
}
