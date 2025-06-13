/// Comprehensive networking and protocol support module for CURSED programming language
/// 
/// This module provides production-ready networking capabilities including:
/// - TCP/UDP socket operations with async support
/// - IP address handling (IPv4/IPv6) 
/// - DNS resolution and hostname lookup
/// - HTTP client library with authentication
/// - WebSocket support for real-time communication
/// - Protocol implementations (SMTP, FTP, SSH, TLS)
/// - Network interface enumeration
/// - Socket configuration and options
/// 
/// # Features
/// 
/// ## Core Networking
/// - Socket creation and management (TCP/UDP)
/// - IPv4 and IPv6 address handling
/// - DNS resolution and reverse DNS lookup
/// - Network interface discovery
/// - Port scanning and availability checking
/// 
/// ## HTTP Client
/// - HTTP/1.1 and HTTP/2 support
/// - Request/response handling with headers
/// - Authentication (Basic, Bearer, OAuth2)
/// - Cookie management and session handling
/// - Connection pooling and keepalive
/// - Timeout and retry mechanisms
/// 
/// ## WebSocket Support
/// - WebSocket client and server implementation
/// - Real-time bidirectional communication
/// - Frame handling and message types
/// - Compression support (per-message-deflate)
/// - Connection management and heartbeat
/// 
/// ## Protocol Implementations
/// - SMTP client for email sending
/// - FTP client for file transfer
/// - SSH client for secure shell access
/// - Basic TLS/SSL wrapper functionality
/// 
/// # Examples
/// 
/// ## Basic TCP Client
/// ```cursed
/// import "stdlib::net";
/// 
/// let socket = TcpSocket::connect("127.0.0.1:8080")?;
/// socket.write("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")?;
/// let response = socket.read_string(1024)?;
/// socket.close()?;
/// ```
/// 
/// ## HTTP Client
/// ```cursed
/// import "stdlib::net::http";
/// 
/// let client = HttpClient::new()?;
/// let response = client.get("https://api.example.com/data")?;
/// 
/// if response.status == 200 {
///     println("Response: {}", response.body)?;
/// }
/// ```
/// 
/// ## WebSocket Client
/// ```cursed
/// import "stdlib::net::websocket";
/// 
/// let ws = WebSocketClient::connect("ws://localhost:8080/socket")?;
/// ws.send_text("Hello, WebSocket!")?;
/// 
/// while let Some(message) = ws.receive()? {
///     println("Received: {}", message)?;
/// }
/// ```
/// 
/// ## DNS Resolution
/// ```cursed
/// import "stdlib::net";
/// 
/// let addresses = resolve_hostname("example.com")?;
/// for addr in addresses {
///     println("IP: {}", addr)?;
/// }
/// ```

pub mod error;
pub mod socket;
pub mod address;
pub mod dns;
pub mod interfaces;
pub mod http;
pub mod websocket;
pub mod protocols;
pub mod utils;
pub mod http2;

// Re-export main types and functions for easy access
pub use error::{NetError, NetResult, connection_error, timeout_error, dns_error, protocol_error};

// Core networking types
pub use address::{IpAddr, IpAddrV4, IpAddrV6, SocketAddr, SocketAddrV4, SocketAddrV6};
pub use socket::{
    TcpSocket, UdpSocket, TcpListener, SocketConfig, SocketOptions,
    SocketType, SocketState, ProtocolType
};

// DNS operations
pub use dns::{
    DnsResolver, DnsRecord, DnsRecordType, DnsQuery, DnsResponse,
    resolve_hostname, resolve_ip, lookup_mx, lookup_txt, lookup_cname
};

// Network interface utilities
pub use interfaces::{
    NetworkInterface, InterfaceType, InterfaceStats, InterfaceConfig,
    list_interfaces, get_interface_by_name, get_default_interface
};

// HTTP client functionality
pub use http::{
    HttpClient, HttpRequest, HttpResponse, HttpHeaders, HttpMethod, StatusCode,
    RequestBuilder, ConnectionPool, Cookie, HttpAuth, HttpConfig
};

// WebSocket functionality
pub use websocket::{
    WebSocketClient, WebSocketServer, WebSocketMessage, WebSocketFrame,
    MessageType, CloseCode, WebSocketConfig, CompressionConfig
};

// Protocol implementations
pub use protocols::{
    SmtpClient, FtpClient, SshClient, TlsConfig,
    EmailMessage, FtpTransferMode, SshCommand, SshKey
};

// Utility functions
pub use utils::{
    is_port_available, scan_ports, ping_host, trace_route,
    get_public_ip, get_local_ips, validate_email, validate_url,
    parse_url, format_bandwidth, network_diagnostics
};

/// Initialize the networking subsystem
/// 
/// This function should be called once at program startup to initialize
/// platform-specific networking components and set up proper socket handling.
pub fn initialize() -> NetResult<()> {
    #[cfg(windows)]
    {
        // Initialize Winsock on Windows
        use std::mem;
        use std::ptr;
        
        #[repr(C)]
        struct WSAData {
            version: u16,
            high_version: u16,
            description: [u8; 257],
            system_status: [u8; 129],
            max_sockets: u16,
            max_udp_dg: u16,
            vendor_info: *mut u8,
        }
        
        extern "system" {
            fn WSAStartup(version_requested: u16, wsa_data: *mut WSAData) -> i32;
        }
        
        let mut wsa_data: WSAData = unsafe { mem::zeroed() };
        let result = unsafe { WSAStartup(0x0202, &mut wsa_data) };
        
        if result != 0 {
            return Err(NetError::System {
                code: result,
                message: "Failed to initialize Winsock".to_string(),
            });
        }
    }
    
    Ok(())
}

/// Shutdown the networking subsystem
/// 
/// This function should be called at program shutdown to properly clean up
/// networking resources and close any remaining connections.
pub fn shutdown() -> NetResult<()> {
    #[cfg(windows)]
    {
        extern "system" {
            fn WSACleanup() -> i32;
        }
        
        let result = unsafe { WSACleanup() };
        if result != 0 {
            return Err(NetError::System {
                code: result,
                message: "Failed to cleanup Winsock".to_string(),
            });
        }
    }
    
    Ok(())
}

/// Get networking module statistics
pub fn get_network_statistics() -> NetworkStatistics {
    NetworkStatistics {
        active_connections: 0, // TODO: Track active connections
        total_bytes_sent: 0,   // TODO: Track bytes sent
        total_bytes_received: 0, // TODO: Track bytes received
        dns_queries: 0,        // TODO: Track DNS queries
        failed_connections: 0, // TODO: Track failed connections
    }
}

/// Network statistics for monitoring
#[derive(Debug, Clone)]
pub struct NetworkStatistics {
    pub active_connections: u64,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
    pub dns_queries: u64,
    pub failed_connections: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_initialization() {
        // Test that the module can be initialized without errors
        assert!(initialize().is_ok());
        assert!(shutdown().is_ok());
    }

    #[test]
    fn test_network_statistics() {
        let stats = get_network_statistics();
        assert_eq!(stats.active_connections, 0);
        assert_eq!(stats.total_bytes_sent, 0);
        assert_eq!(stats.total_bytes_received, 0);
    }

    #[test]
    fn test_core_types_exist() {
        // Test that all expected types are exported
        let _ = std::any::type_name::<TcpSocket>();
        let _ = std::any::type_name::<UdpSocket>();
        let _ = std::any::type_name::<IpAddr>();
        let _ = std::any::type_name::<SocketAddr>();
        let _ = std::any::type_name::<HttpClient>();
        let _ = std::any::type_name::<WebSocketClient>();
    }

    #[test]
    fn test_error_functions_exist() {
        let _ = connection_error("test");
        let _ = timeout_error("test");
        let _ = dns_error("test");
        let _ = protocol_error("test");
    }
}
