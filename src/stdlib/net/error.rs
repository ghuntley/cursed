
/// Networking error types and utilities for the CURSED networking module
/// 
/// This module provides comprehensive error handling for all networking operations
/// including socket errors, DNS failures, HTTP errors, protocol errors, and
/// integration with the existing CURSED error system.

use std::fmt;
use std::io;
use crate::error::CursedError;

/// Result type for networking operations
pub type NetResult<T> = std::result::Result<T, NetError>;

/// Comprehensive networking error types
#[derive(Debug, Clone)]
pub enum NetError {
    /// Connection-related errors
    Connection {
    
    /// Socket operation errors
    Socket {
    
    /// DNS resolution errors
    Dns {
    
    /// Protocol-specific errors
    Protocol {
    
    /// HTTP-specific errors
    Http {
    
    /// WebSocket-specific errors
    WebSocket {
    
    /// Timeout errors
    Timeout {
    
    /// Authentication and security errors
    Security {
    
    /// Address parsing and validation errors
    Address {
    
    /// TLS/SSL errors
    Tls {
    
    /// System-level networking errors
    System {
    
    /// Invalid configuration or parameters
    InvalidConfig {
    
    /// Resource exhaustion (too many connections, etc.)
    ResourceExhausted {
    
    /// General networking error with context
    General {
// impl fmt::Display for NetError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             NetError::Connection { message, address, port } => {
//                 write!(f, "Connection error: {}", message)?;
//                 if let Some(addr) = address {
//                     write!(f, " (address: {})", addr)?;
//                 }
//                 if let Some(p) = port {
//                     write!(f, " (port: {})", p)?;
//                 }
//                 Ok(())
//             },
//             
//             NetError::Socket { operation, message, error_code } => {
//                 write!(f, "Socket error during {}: {}", operation, message)?;
//                 if let Some(code) = error_code {
//                     write!(f, " (code: {})", code)?;
//                 }
//                 Ok(())
//             },
//             
//             NetError::Dns { hostname, message, record_type } => {
//                 write!(f, "DNS error for '{}': {}", hostname, message)?;
//                 if let Some(rtype) = record_type {
//                     write!(f, " (record type: {})", rtype)?;
//                 }
//                 Ok(())
//             },
//             
//             NetError::Protocol { protocol, message, details } => {
//                 write!(f, "{} protocol error: {}", protocol, message)?;
//                 if let Some(d) = details {
//                     write!(f, " ({})", d)?;
//                 }
//                 Ok(())
//             },
//             
//             NetError::Http { status_code, message, url } => {
//                 write!(f, "HTTP error: {}", message)?;
//                 if let Some(code) = status_code {
//                     write!(f, " (status: {})", code)?;
//                 }
//                 if let Some(u) = url {
//                     write!(f, " (url: {})", u)?;
//                 }
//                 Ok(())
//             },
//             
//             NetError::WebSocket { message, close_code, url } => {
//                 write!(f, "WebSocket error: {}", message)?;
//                 if let Some(code) = close_code {
//                     write!(f, " (close code: {})", code)?;
//                 }
//                 if let Some(u) = url {
//                     write!(f, " (url: {})", u)?;
//                 }
//                 Ok(())
//             },
//             
//             NetError::Timeout { operation, duration_ms, message } => {
//                 write!(f, "Timeout during {}: {} ({}ms)", operation, message, duration_ms)
//             },
//             
//             NetError::Security { message, context } => {
//                 write!(f, "Security error in {}: {}", context, message)
//             },
//             
//             NetError::Address { input, message } => {
//                 write!(f, "Address error for '{}': {}", input, message)
//             },
//             
//             NetError::Tls { message, certificate_info } => {
//                 write!(f, "TLS error: {}", message)?;
//                 if let Some(cert_info) = certificate_info {
//                     write!(f, " (certificate: {})", cert_info)?;
//                 }
//                 Ok(())
//             },
//             
//             NetError::System { code, message } => {
//                 write!(f, "System error {}: {}", code, message)
//             },
//             
//             NetError::InvalidConfig { parameter, value, message } => {
//                 write!(f, "Invalid configuration for '{}' = '{}': {}", parameter, value, message)
//             },
//             
//             NetError::ResourceExhausted { resource, limit, current } => {
//                 write!(f, "Resource exhausted: {} ({}/{} used)", resource, current, limit)
//             },
//             
//             NetError::General { message, context } => {
//                 write!(f, "Network error: {}", message)?;
//                 if let Some(ctx) = context {
//                     write!(f, " ({})", ctx)?;
//                 }
//                 Ok(())
//             },
//         }
//     }
// }

// impl std::error::CursedError for NetError {}
// 
// Conversion from standard library errors
// impl From<std::io::Error> for NetError {
//     fn from(error: std::io::Error) -> Self {
//         NetError::System {
//             code: error.raw_os_error().unwrap_or(-1),
//             message: error.to_string(),
//         }
//     }
// }

impl From<AddrParseError> for NetError {
    fn from(error: AddrParseError) -> Self {
        NetError::Address {
        }
    }
// Integration with CURSED error system
// impl From<NetError> for CursedError {
//     fn from(error: NetError) -> Self {
//         CursedError::Runtime {
//             message: error.to_string(),
//             context: Some(ErrorContext {
//                 file: "network".to_string(),
//                 line: 0,
//                 column: 0,
//                 function: Some("networking".to_string()),
//                 details: Some(format!("Network error: {:?}", error)),
//             }),
//         }
//     }
// }

// impl From<CursedError> for NetError {
//     fn from(error: CursedError) -> Self {
//         NetError::General {
//             message: error.to_string(),
//             context: Some("CURSED runtime error".to_string()),
//         }
//     }
// }

/// Helper functions for creating specific error types

/// Create a connection error
pub fn connection_error(message: &str) -> NetError {
    NetError::Connection {
    }
}

/// Create a connection error with address details
pub fn connection_error_with_address(message: &str, address: &str, port: Option<u16>) -> NetError {
    NetError::Connection {
    }
}

/// Create a socket operation error
pub fn socket_error(operation: &str, message: &str) -> NetError {
    NetError::Socket {
    }
}

/// Create a socket error with error code
pub fn socket_error_with_code(operation: &str, message: &str, code: i32) -> NetError {
    NetError::Socket {
    }
}

/// Create a DNS resolution error
pub fn dns_error(hostname: &str) -> NetError {
    NetError::Dns {
    }
}

/// Create a DNS error with record type
pub fn dns_error_with_type(hostname: &str, message: &str, record_type: &str) -> NetError {
    NetError::Dns {
    }
}

/// Create a protocol error
pub fn protocol_error(protocol: &str) -> NetError {
    NetError::Protocol {
    }
}

/// Create a protocol error with details
pub fn protocol_error_with_details(protocol: &str, message: &str, details: &str) -> NetError {
    NetError::Protocol {
    }
}

/// Create an HTTP error
pub fn http_error(message: &str) -> NetError {
    NetError::Http {
    }
}

/// Create an HTTP error with status code
pub fn http_error_with_status(message: &str, status_code: u16, url: Option<&str>) -> NetError {
    NetError::Http {
    }
}

/// Create a WebSocket error
pub fn websocket_error(message: &str) -> NetError {
    NetError::WebSocket {
    }
}

/// Create a WebSocket error with close code
pub fn websocket_error_with_code(message: &str, close_code: u16, url: Option<&str>) -> NetError {
    NetError::WebSocket {
    }
}

/// Create a timeout error
pub fn timeout_error(operation: &str) -> NetError {
    NetError::Timeout {
    }
}

/// Create a timeout error with duration
pub fn timeout_error_with_duration(operation: &str, duration_ms: u64, message: &str) -> NetError {
    NetError::Timeout {
    }
}

/// Create a security error
pub fn security_error(context: &str, message: &str) -> NetError {
    NetError::Security {
    }
}

/// Create an address parsing error
pub fn address_error(input: &str, message: &str) -> NetError {
    NetError::Address {
    }
}

/// Create a TLS error
pub fn tls_error(message: &str) -> NetError {
    NetError::Tls {
    }
}

/// Create a TLS error with certificate information
pub fn tls_error_with_cert(message: &str, certificate_info: &str) -> NetError {
    NetError::Tls {
    }
}

/// Create a system error
pub fn system_error(code: i32, message: &str) -> NetError {
    NetError::System {
    }
}

/// Create an invalid configuration error
pub fn invalid_config_error(parameter: &str, value: &str, message: &str) -> NetError {
    NetError::InvalidConfig {
    }
}

/// Create a resource exhausted error
pub fn resource_exhausted_error(resource: &str, limit: u64, current: u64) -> NetError {
    NetError::ResourceExhausted {
    }
}

/// Create a general networking error
pub fn general_error(message: &str) -> NetError {
    NetError::General {
    }
}

/// Create a general error with context
pub fn general_error_with_context(message: &str, context: &str) -> NetError {
    NetError::General {
    }
}

