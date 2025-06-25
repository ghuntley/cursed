use crate::error::CursedError;
/// Network-specific error types for VibeNet
/// 
/// This module provides comprehensive error handling for network operations,
/// integrating with the CURSED error system while providing specific network
/// error context and recovery information.

use std::fmt;
use std::io;

/// Network-specific error types
#[derive(Debug, Clone)]
pub enum NetError {
    /// Address resolution failed
    
    /// Connection failed
    
    /// Connection timeout
    
    /// Invalid network protocol
    
    /// DNS resolution failed
    
    /// Network interface error
    
    /// Socket operation failed
    
    /// TLS/SSL error
    
    /// Protocol-specific error
    
    /// Rate limiting error
    
    /// Circuit breaker open
    
    /// Connection pool exhausted
    
    /// General I/O error
    
    /// Permission denied
    
    /// Resource unavailable
    
    /// Invalid configuration
// impl fmt::Display for NetError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             NetError::AddressResolution(msg) => write!(f, "Address resolution error: {}", msg),
//             NetError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
//             NetError::Timeout(msg) => write!(f, "Network timeout: {}", msg),
//             NetError::InvalidProtocol(msg) => write!(f, "Invalid protocol: {}", msg),
//             NetError::DnsResolution(msg) => write!(f, "DNS resolution error: {}", msg),
//             NetError::InterfaceError(msg) => write!(f, "Network interface error: {}", msg),
//             NetError::SocketError(msg) => write!(f, "Socket error: {}", msg),
//             NetError::TlsError(msg) => write!(f, "TLS/SSL error: {}", msg),
//             NetError::ProtocolError(msg) => write!(f, "Protocol error: {}", msg),
//             NetError::RateLimit(msg) => write!(f, "Rate limit exceeded: {}", msg),
//             NetError::CircuitBreakerOpen(msg) => write!(f, "Circuit breaker open: {}", msg),
//             NetError::PoolExhausted(msg) => write!(f, "Connection pool exhausted: {}", msg),
//             NetError::Io(msg) => write!(f, "I/O error: {}", msg),
//             NetError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
//             NetError::ResourceUnavailable(msg) => write!(f, "Resource unavailable: {}", msg),
//             NetError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for NetError {}
// 
// impl From<std::io::Error> for NetError {
//     fn from(err: std::io::Error) -> Self {
//         match err.kind() {
//             io::ErrorKind::TimedOut => NetError::Timeout(err.to_string()),
//             io::ErrorKind::PermissionDenied => NetError::PermissionDenied(err.to_string()),
//             io::ErrorKind::ConnectionRefused 
//             | io::ErrorKind::ConnectionAborted 
//             | io::ErrorKind::ConnectionReset => NetError::ConnectionFailed(err.to_string()),
//             _ => NetError::Io(err.to_string()),
//         }
//     }
// }

impl From<AddrParseError> for NetError {
    fn from(err: AddrParseError) -> Self {
        NetError::AddressResolution(err.to_string())
    }
}

// impl From<NetError> for CursedError {
//     fn from(err: NetError) -> Self {
//         CursedError::new(&err.to_string())
//     }
// }

/// Helper functions for creating network errors

/// Create an address resolution error
pub fn address_resolution_error(msg: &str) -> NetError {
    NetError::AddressResolution(msg.to_string())
/// Create a connection failed error
pub fn connection_failed_error(msg: &str) -> NetError {
    NetError::ConnectionFailed(msg.to_string())
/// Create a timeout error
pub fn timeout_error(msg: &str) -> NetError {
    NetError::Timeout(msg.to_string())
/// Create an invalid protocol error
pub fn invalid_protocol_error(msg: &str) -> NetError {
    NetError::InvalidProtocol(msg.to_string())
/// Create a DNS resolution error
pub fn dns_resolution_error(msg: &str) -> NetError {
    NetError::DnsResolution(msg.to_string())
/// Create an interface error
pub fn interface_error(msg: &str) -> NetError {
    NetError::InterfaceError(msg.to_string())
/// Create a socket error
pub fn socket_error(msg: &str) -> NetError {
    NetError::SocketError(msg.to_string())
/// Create a TLS error
pub fn tls_error(msg: &str) -> NetError {
    NetError::TlsError(msg.to_string())
/// Create a protocol error
pub fn protocol_error(msg: &str) -> NetError {
    NetError::ProtocolError(msg.to_string())
/// Create a rate limit error
pub fn rate_limit_error(msg: &str) -> NetError {
    NetError::RateLimit(msg.to_string())
/// Create a circuit breaker error
pub fn circuit_breaker_error(msg: &str) -> NetError {
    NetError::CircuitBreakerOpen(msg.to_string())
/// Create a pool exhausted error
pub fn pool_exhausted_error(msg: &str) -> NetError {
    NetError::PoolExhausted(msg.to_string())
/// Create an I/O error
pub fn io_error(msg: &str) -> NetError {
    NetError::Io(msg.to_string())
/// Create a permission denied error
pub fn permission_denied_error(msg: &str) -> NetError {
    NetError::PermissionDenied(msg.to_string())
/// Create a resource unavailable error
pub fn resource_unavailable_error(msg: &str) -> NetError {
    NetError::ResourceUnavailable(msg.to_string())
/// Create an invalid configuration error
pub fn invalid_config_error(msg: &str) -> NetError {
    NetError::InvalidConfig(msg.to_string())
