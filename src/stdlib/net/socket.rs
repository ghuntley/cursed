use crate::error::Error;
/// Socket operations for the CURSED networking module
/// 
/// This module provides TCP and UDP socket implementations with comprehensive
/// functionality including connection management, data transmission, socket
/// configuration, and async-compatible operations.

use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener as StdTcpListener, UdpSocket as StdUdpSocket};
use std::time::Duration;
use std::sync::{Arc, Mutex};
use crate::stdlib::net::error::{NetError, NetResult, connection_error, socket_error, timeout_error};
use crate::stdlib::net::address::{SocketAddr, IpAddr};

/// Socket type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Tcp,
    Udp,
}

/// Socket state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    Closed,
    Connecting,
    Connected,
    Listening,
    Error,
}

/// Protocol type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolType {
    IPv4,
    IPv6,
}

/// Socket configuration options
#[derive(Debug, Clone)]
pub struct SocketConfig {
    pub connect_timeout: Option<Duration>,
    pub read_timeout: Option<Duration>,
    pub write_timeout: Option<Duration>,
    pub keep_alive: Option<Duration>,
    pub reuse_addr: bool,
    pub reuse_port: bool,
    pub nodelay: bool,
    pub buffer_size: Option<usize>,
}

impl Default for SocketConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Some(Duration::from_secs(30)),
            read_timeout: Some(Duration::from_secs(30)),
            write_timeout: Some(Duration::from_secs(30)),
            keep_alive: None,
            reuse_addr: false,
            reuse_port: false,
            nodelay: false,
            buffer_size: Some(8192),
        }
    }
}

/// Socket options for fine-grained control
#[derive(Debug, Clone)]
pub struct SocketOptions {
    pub send_buffer_size: Option<usize>,
    pub receive_buffer_size: Option<usize>,
    pub linger: Option<Duration>,
    pub broadcast: bool,
    pub multicast_loop: bool,
    pub multicast_ttl: Option<u32>,
    pub ttl: Option<u32>,
    pub only_v6: bool,
}

impl Default for SocketOptions {
    fn default() -> Self {
        Self {
            send_buffer_size: None,
            receive_buffer_size: None,
            linger: None,
            broadcast: false,
            multicast_loop: true,
            multicast_ttl: Some(1),
            ttl: Some(64),
            only_v6: false,
        }
    }
}

/// TCP socket implementation
#[derive(Debug)]
pub struct TcpSocket {
    stream: Arc<Mutex<Option<TcpStream>>>,
    state: Arc<Mutex<SocketState>>,
    local_addr: Arc<Mutex<Option<SocketAddr>>>,
    peer_addr: Arc<Mutex<Option<SocketAddr>>>,
    config: SocketConfig,
}

impl TcpSocket {
    /// Create a new TCP socket
    pub fn new() -> Self {
        Self {
            stream: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(SocketState::Closed)),
            local_addr: Arc::new(Mutex::new(None)),
            peer_addr: Arc::new(Mutex::new(None)),
            config: SocketConfig::default(),
        }
    }
    
    /// Create a TCP socket with custom configuration
    pub fn with_config(config: SocketConfig) -> Self {
        Self {
            stream: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(SocketState::Closed)),
            local_addr: Arc::new(Mutex::new(None)),
            peer_addr: Arc::new(Mutex::new(None)),
            config,
        }
    }
    
    /// Connect to a remote address
    pub fn connect(addr: &str) -> NetResult<Self> {
        let socket_addr: SocketAddr = addr.parse()
            .map_err(|e| connection_error(&format!("Invalid address '{}': {}", addr, e)))?;
        
        let std_addr: std::net::SocketAddr = socket_addr.into();
        
        let stream = TcpStream::connect_timeout(&std_addr, Duration::from_secs(30))
            .map_err(|e| connection_error(&format!("Failed to connect to {}: {}", addr, e)))?;
        
        let local_addr = stream.local_addr()
            .map(|addr| addr.into())
            .ok();
            
        let peer_addr = stream.peer_addr()
            .map(|addr| addr.into())
            .ok();
        
        Ok(Self {
            stream: Arc::new(Mutex::new(Some(stream))),
            state: Arc::new(Mutex::new(SocketState::Connected)),
            local_addr: Arc::new(Mutex::new(local_addr)),
            peer_addr: Arc::new(Mutex::new(peer_addr)),
            config: SocketConfig::default(),
        })
    }
    
    /// Connect with custom timeout
    pub fn connect_timeout(addr: &str, timeout: Duration) -> NetResult<Self> {
        let socket_addr: SocketAddr = addr.parse()
            .map_err(|e| connection_error(&format!("Invalid address '{}': {}", addr, e)))?;
        
        let std_addr: std::net::SocketAddr = socket_addr.into();
        
        let stream = TcpStream::connect_timeout(&std_addr, timeout)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::TimedOut {
                    timeout_error(&format!("Connection to {} timed out after {:?}", addr, timeout))
                } else {
                    connection_error(&format!("Failed to connect to {}: {}", addr, e))
                }
            })?;
        
        let local_addr = stream.local_addr()
            .map(|addr| addr.into())
            .ok();
            
        let peer_addr = stream.peer_addr()
            .map(|addr| addr.into())
            .ok();
        
        Ok(Self {
            stream: Arc::new(Mutex::new(Some(stream))),
            state: Arc::new(Mutex::new(SocketState::Connected)),
            local_addr: Arc::new(Mutex::new(local_addr)),
            peer_addr: Arc::new(Mutex::new(peer_addr)),
            config: SocketConfig::default(),
        })
    }
    
    /// Create TCP socket from existing stream (used by TcpListener)
    pub(crate) fn from_stream(stream: TcpStream) -> NetResult<Self> {
        let local_addr = stream.local_addr()
            .map(|addr| addr.into())
            .ok();
            
        let peer_addr = stream.peer_addr()
            .map(|addr| addr.into())
            .ok();
        
        Ok(Self {
            stream: Arc::new(Mutex::new(Some(stream))),
            state: Arc::new(Mutex::new(SocketState::Connected)),
            local_addr: Arc::new(Mutex::new(local_addr)),
            peer_addr: Arc::new(Mutex::new(peer_addr)),
            config: SocketConfig::default(),
        })
    }
    
    /// Write data to the socket
    pub fn write(&self, data: &[u8]) -> NetResult<usize> {
        let mut stream_guard = self.stream.lock().unwrap();
        if let Some(ref mut stream) = *stream_guard {
            stream.write(data)
                .map_err(|e| socket_error("write", &e.to_string()))
        } else {
            Err(socket_error("write", "Socket not connected"))
        }
    }
    
    /// Write a string to the socket
    pub fn write_string(&self, data: &str) -> NetResult<usize> {
        self.write(data.as_bytes())
    }
    
    /// Write all data to the socket
    pub fn write_all(&self, data: &[u8]) -> NetResult<()> {
        let mut stream_guard = self.stream.lock().unwrap();
        if let Some(ref mut stream) = *stream_guard {
            stream.write_all(data)
                .map_err(|e| socket_error("write_all", &e.to_string()))
        } else {
            Err(socket_error("write_all", "Socket not connected"))
        }
    }
    
    /// Read data from the socket
    pub fn read(&self, buffer: &mut [u8]) -> NetResult<usize> {
        let mut stream_guard = self.stream.lock().unwrap();
        if let Some(ref mut stream) = *stream_guard {
            stream.read(buffer)
                .map_err(|e| socket_error("read", &e.to_string()))
        } else {
            Err(socket_error("read", "Socket not connected"))
        }
    }
    
    /// Read exact number of bytes from the socket
    pub fn read_exact(&self, buffer: &mut [u8]) -> NetResult<()> {
        let mut stream_guard = self.stream.lock().unwrap();
        if let Some(ref mut stream) = *stream_guard {
            stream.read_exact(buffer)
                .map_err(|e| socket_error("read_exact", &e.to_string()))
        } else {
            Err(socket_error("read_exact", "Socket not connected"))
        }
    }
    
    /// Read data into a string
    pub fn read_string(&self, max_len: usize) -> NetResult<String> {
        let mut buffer = vec![0u8; max_len];
        let bytes_read = self.read(&mut buffer)?;
        buffer.truncate(bytes_read);
        
        String::from_utf8(buffer)
            .map_err(|e| socket_error("read_string", &format!("Invalid UTF-8: {}", e)))
    }
    
    /// Read a line from the socket (until \n or \r\n)
    pub fn read_line(&self) -> NetResult<String> {
        let mut line = String::new();
        let mut buffer = [0u8; 1];
        
        loop {
            let bytes_read = self.read(&mut buffer)?;
            if bytes_read == 0 {
                break; // EOF
            }
            
            let ch = buffer[0] as char;
            if ch == '\n' {
                break;
            } else if ch != '\r' {
                line.push(ch);
            }
        }
        
        Ok(line)
    }
    
    /// Set socket timeouts
    pub fn set_timeout(&self, read_timeout: Option<Duration>, write_timeout: Option<Duration>) -> NetResult<()> {
        let stream_guard = self.stream.lock().unwrap();
        if let Some(ref stream) = *stream_guard {
            if let Some(timeout) = read_timeout {
                stream.set_read_timeout(Some(timeout))
                    .map_err(|e| socket_error("set_read_timeout", &e.to_string()))?;
            }
            if let Some(timeout) = write_timeout {
                stream.set_write_timeout(Some(timeout))
                    .map_err(|e| socket_error("set_write_timeout", &e.to_string()))?;
            }
            Ok(())
        } else {
            Err(socket_error("set_timeout", "Socket not connected"))
        }
    }
    
    /// Set TCP_NODELAY option
    pub fn set_nodelay(&self, nodelay: bool) -> NetResult<()> {
        let stream_guard = self.stream.lock().unwrap();
        if let Some(ref stream) = *stream_guard {
            stream.set_nodelay(nodelay)
                .map_err(|e| socket_error("set_nodelay", &e.to_string()))
        } else {
            Err(socket_error("set_nodelay", "Socket not connected"))
        }
    }
    
    /// Get local address
    pub fn local_addr(&self) -> Option<SocketAddr> {
        *self.local_addr.lock().unwrap()
    }
    
    /// Get peer address
    pub fn peer_addr(&self) -> Option<SocketAddr> {
        *self.peer_addr.lock().unwrap()
    }
    
    /// Get current socket state
    pub fn state(&self) -> SocketState {
        *self.state.lock().unwrap()
    }
    
    /// Check if socket is connected
    pub fn is_connected(&self) -> bool {
        matches!(self.state(), SocketState::Connected)
    }
    
    /// Shutdown the socket connection
    pub fn shutdown(&self) -> NetResult<()> {
        let mut stream_guard = self.stream.lock().unwrap();
        if let Some(ref stream) = *stream_guard {
            stream.shutdown(std::net::Shutdown::Both)
                .map_err(|e| socket_error("shutdown", &e.to_string()))?;
            *self.state.lock().unwrap() = SocketState::Closed;
            Ok(())
        } else {
            Err(socket_error("shutdown", "Socket not connected"))
        }
    }
    
    /// Close the socket
    pub fn close(&self) -> NetResult<()> {
        let mut stream_guard = self.stream.lock().unwrap();
        *stream_guard = None;
        *self.state.lock().unwrap() = SocketState::Closed;
        Ok(())
    }
}

/// TCP listener for accepting incoming connections
#[derive(Debug)]
pub struct TcpListener {
    listener: StdTcpListener,
    local_addr: SocketAddr,
}

impl TcpListener {
    /// Bind to a local address and start listening
    pub fn bind(addr: &str) -> NetResult<Self> {
        let socket_addr: SocketAddr = addr.parse()
            .map_err(|e| connection_error(&format!("Invalid address '{}': {}", addr, e)))?;
        
        let std_addr: std::net::SocketAddr = socket_addr.into();
        
        let listener = StdTcpListener::bind(std_addr)
            .map_err(|e| connection_error(&format!("Failed to bind to {}: {}", addr, e)))?;
        
        let local_addr = listener.local_addr()
            .map(|addr| addr.into())
            .map_err(|e| socket_error("local_addr", &e.to_string()))?;
        
        Ok(Self {
            listener,
            local_addr,
        })
    }
    
    /// Accept an incoming connection
    pub fn accept(&self) -> NetResult<TcpSocket> {
        let (stream, _peer_addr) = self.listener.accept()
            .map_err(|e| socket_error("accept", &e.to_string()))?;
        
        TcpSocket::from_stream(stream)
    }
    
    /// Accept with timeout
    pub fn accept_timeout(&self, timeout: Duration) -> NetResult<TcpSocket> {
        self.listener.set_nonblocking(true)
            .map_err(|e| socket_error("set_nonblocking", &e.to_string()))?;
        
        let start = std::time::Instant::now();
        loop {
            match self.listener.accept() {
                Ok((stream, _)) => {
                    self.listener.set_nonblocking(false)
                        .map_err(|e| socket_error("set_blocking", &e.to_string()))?;
                    return TcpSocket::from_stream(stream);
                },
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if start.elapsed() >= timeout {
                        self.listener.set_nonblocking(false)
                            .map_err(|e| socket_error("set_blocking", &e.to_string()))?;
                        return Err(timeout_error("accept"));
                    }
                    std::thread::sleep(Duration::from_millis(10));
                },
                Err(e) => {
                    self.listener.set_nonblocking(false)
                        .map_err(|e| socket_error("set_blocking", &e.to_string()))?;
                    return Err(socket_error("accept", &e.to_string()));
                },
            }
        }
    }
    
    /// Get local address
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }
}

/// UDP socket implementation
#[derive(Debug)]
pub struct UdpSocket {
    socket: StdUdpSocket,
    local_addr: SocketAddr,
}

impl UdpSocket {
    /// Bind to a local address
    pub fn bind(addr: &str) -> NetResult<Self> {
        let socket_addr: SocketAddr = addr.parse()
            .map_err(|e| connection_error(&format!("Invalid address '{}': {}", addr, e)))?;
        
        let std_addr: std::net::SocketAddr = socket_addr.into();
        
        let socket = StdUdpSocket::bind(std_addr)
            .map_err(|e| connection_error(&format!("Failed to bind to {}: {}", addr, e)))?;
        
        let local_addr = socket.local_addr()
            .map(|addr| addr.into())
            .map_err(|e| socket_error("local_addr", &e.to_string()))?;
        
        Ok(Self {
            socket,
            local_addr,
        })
    }
    
    /// Connect to a remote address (for connected UDP)
    pub fn connect(&self, addr: &str) -> NetResult<()> {
        let socket_addr: SocketAddr = addr.parse()
            .map_err(|e| connection_error(&format!("Invalid address '{}': {}", addr, e)))?;
        
        let std_addr: std::net::SocketAddr = socket_addr.into();
        
        self.socket.connect(std_addr)
            .map_err(|e| connection_error(&format!("Failed to connect to {}: {}", addr, e)))
    }
    
    /// Send data to a specific address
    pub fn send_to(&self, data: &[u8], addr: &str) -> NetResult<usize> {
        let socket_addr: SocketAddr = addr.parse()
            .map_err(|e| connection_error(&format!("Invalid address '{}': {}", addr, e)))?;
        
        let std_addr: std::net::SocketAddr = socket_addr.into();
        
        self.socket.send_to(data, std_addr)
            .map_err(|e| socket_error("send_to", &e.to_string()))
    }
    
    /// Send data (for connected UDP)
    pub fn send(&self, data: &[u8]) -> NetResult<usize> {
        self.socket.send(data)
            .map_err(|e| socket_error("send", &e.to_string()))
    }
    
    /// Receive data and get sender address
    pub fn recv_from(&self, buffer: &mut [u8]) -> NetResult<(usize, SocketAddr)> {
        let (size, addr) = self.socket.recv_from(buffer)
            .map_err(|e| socket_error("recv_from", &e.to_string()))?;
        
        Ok((size, addr.into()))
    }
    
    /// Receive data (for connected UDP)
    pub fn recv(&self, buffer: &mut [u8]) -> NetResult<usize> {
        self.socket.recv(buffer)
            .map_err(|e| socket_error("recv", &e.to_string()))
    }
    
    /// Set socket timeouts
    pub fn set_timeout(&self, read_timeout: Option<Duration>, write_timeout: Option<Duration>) -> NetResult<()> {
        if let Some(timeout) = read_timeout {
            self.socket.set_read_timeout(Some(timeout))
                .map_err(|e| socket_error("set_read_timeout", &e.to_string()))?;
        }
        if let Some(timeout) = write_timeout {
            self.socket.set_write_timeout(Some(timeout))
                .map_err(|e| socket_error("set_write_timeout", &e.to_string()))?;
        }
        Ok(())
    }
    
    /// Set broadcast option
    pub fn set_broadcast(&self, broadcast: bool) -> NetResult<()> {
        self.socket.set_broadcast(broadcast)
            .map_err(|e| socket_error("set_broadcast", &e.to_string()))
    }
    
    /// Get local address
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }
}

/// Utility functions for socket operations

/// Check if a port is available on the local machine
pub fn is_port_available(port: u16) -> bool {
    TcpListener::bind(&format!("127.0.0.1:{}", port)).is_ok()
}

/// Find an available port in a given range
pub fn find_available_port(start_port: u16, end_port: u16) -> Option<u16> {
    for port in start_port..=end_port {
        if is_port_available(port) {
            return Some(port);
        }
    }
    None
}

/// Get the next available port starting from a given port
pub fn get_available_port(start_port: u16) -> Option<u16> {
    find_available_port(start_port, 65535)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_socket_config_default() {
        let config = SocketConfig::default();
        assert!(config.connect_timeout.is_some());
        assert!(config.read_timeout.is_some());
        assert!(config.write_timeout.is_some());
    }

    #[test]
    fn test_socket_creation() {
        let socket = TcpSocket::new();
        assert_eq!(socket.state(), SocketState::Closed);
        assert!(!socket.is_connected());
    }

    #[test]
    fn test_port_availability() {
        // Port 0 should be available (will be assigned by OS)
        assert!(is_port_available(0));
        
        // Try to find an available port
        let port = find_available_port(50000, 60000);
        assert!(port.is_some());
    }

    #[test]
    fn test_tcp_listener_creation() {
        // Bind to any available port
        let listener = TcpListener::bind("127.0.0.1:0");
        assert!(listener.is_ok());
        
        if let Ok(listener) = listener {
            let addr = listener.local_addr();
            assert!(addr.port() > 0);
        }
    }

    #[test]
    fn test_udp_socket_creation() {
        let socket = UdpSocket::bind("127.0.0.1:0");
        assert!(socket.is_ok());
        
        if let Ok(socket) = socket {
            let addr = socket.local_addr();
            assert!(addr.port() > 0);
        }
    }

    #[test]
    fn test_tcp_connection_simulation() {
        // Start a simple echo server in a separate thread
        thread::spawn(|| {
            if let Ok(listener) = TcpListener::bind("127.0.0.1:0") {
                // This is just a test - in real code we'd handle the connection
                let _ = listener.accept();
            }
        });
        
        // Give the server time to start
        thread::sleep(Duration::from_millis(100));
        
        // Test connection attempt (will likely fail since we don't have a real server)
        let result = TcpSocket::connect_timeout("127.0.0.1:12345", Duration::from_millis(100));
        // We expect this to fail since there's no server listening
        assert!(result.is_err());
    }

    #[test]
    fn test_udp_socket_operations() {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        
        // Test timeout setting
        let result = socket.set_timeout(Some(Duration::from_secs(1)), None);
        assert!(result.is_ok());
        
        // Test broadcast setting
        let result = socket.set_broadcast(false);
        assert!(result.is_ok());
    }
}
