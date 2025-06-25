use crate::error::CursedError;
/// Unix domain sockets implementation for CURSED IPC
/// 
/// Provides Unix domain sockets for local inter-process communication

use std::collections::HashMap;
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::net::Shutdown;
use std::os::unix::net::{UnixStream, UnixListener, UnixDatagram};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::Duration;

// use crate::stdlib::ipc::error::{IpcError, IpcResult, unix_socket_error, system_error, timeout_error, connection_error};

/// Unix socket registry for cleanup
static SOCKET_REGISTRY: std::sync::OnceLock<Arc<RwLock<HashMap<String, PathBuf>>>> = std::sync::OnceLock::new();

fn get_socket_registry() -> &'static Arc<RwLock<HashMap<String, PathBuf>>> {
    SOCKET_REGISTRY.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
/// Unix socket type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnixSocketType {
    /// Stream socket (TCP-like, reliable, ordered)
    /// Datagram socket (UDP-like, connectionless)
/// Unix socket configuration
#[derive(Debug, Clone)]
pub struct UnixSocketConfig {
    /// Socket type
    /// Buffer size for I/O operations
    /// Socket file permissions
    /// Whether to remove existing socket file
    /// Timeout for operations
    /// Maximum pending connections (server mode)
impl Default for UnixSocketConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Cross-platform Unix socket
#[derive(Debug)]
pub struct UnixSocket {
impl UnixSocket {
    /// Create a new Unix socket
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
        }
    }

    /// Create with configuration
    pub fn with_config<P: AsRef<Path>>(path: P, config: UnixSocketConfig) -> Self {
        Self {
        }
    }

    /// Connect to a Unix socket server
    pub fn connect(&mut self) -> IpcResult<()> {
        match self.config.socket_type {
        }
    }

    /// Connect with timeout
    pub fn connect_timeout(&mut self, timeout: Duration) -> IpcResult<()> {
        // Set timeout in config temporarily
        let original_timeout = self.config.timeout;
        self.config.timeout = Some(timeout);
        
        let result = self.connect();
        
        // Restore original timeout
        self.config.timeout = original_timeout;
        
        result
    /// Send data
    pub fn send(&mut self, data: &[u8]) -> IpcResult<usize> {
        match self.config.socket_type {
        }
    }

    /// Receive data
    pub fn receive(&mut self, buffer: &mut [u8]) -> IpcResult<usize> {
        match self.config.socket_type {
        }
    }

    /// Send a message (convenience method for strings)
    pub fn send_message(&mut self, message: &str) -> IpcResult<()> {
        let data = message.as_bytes();
        self.send(data)?;
        Ok(())
    /// Receive a message (convenience method for strings)
    pub fn receive_message(&mut self) -> IpcResult<String> {
        let mut buffer = vec![0u8; self.config.buffer_size];
        let bytes_received = self.receive(&mut buffer)?;
        buffer.truncate(bytes_received);
        
        String::from_utf8(buffer)
            .map_err(|e| unix_socket_error(Some(self.path.to_str()), "receive_message", &e.to_string()))
    /// Send line (adds newline)
    pub fn send_line(&mut self, line: &str) -> IpcResult<()> {
        let mut data = line.to_string();
        data.push('\n');
        self.send(data.as_bytes())?;
        Ok(())
    /// Receive line
    pub fn receive_line(&mut self) -> IpcResult<String> {
        match &mut self.stream {
            Some(stream) => {
                let mut reader = BufReader::new(stream);
                let mut line = String::new();
                reader.read_line(&mut line)
                    .map_err(|e| unix_socket_error(Some(self.path.to_str()), "receive_line", &e.to_string()))?;
                
                // Remove trailing newline
                if line.ends_with('\n') {
                    line.pop();
                    if line.ends_with('\r') {
                        line.pop();
                    }
                }
                
                Ok(line)
            }
        }
    }

    /// Shutdown the socket
    pub fn shutdown(&mut self, how: Shutdown) -> IpcResult<()> {
        if let Some(stream) = &self.stream {
            stream.shutdown(how)
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "shutdown", &e.to_string()))?;
        }
        Ok(())
    /// Close the socket
    pub fn close(&mut self) -> IpcResult<()> {
        if self.stream.is_some() {
            self.stream = None;
        }
        if self.datagram.is_some() {
            self.datagram = None;
        }
        self.is_connected = false;
        Ok(())
    /// Get socket path
    pub fn path(&self) -> &Path {
        &self.path
    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.is_connected
    fn connect_stream(&mut self) -> IpcResult<()> {
        let stream = UnixStream::connect(&self.path)
            .map_err(|e| connection_error(&self.path.to_string_lossy(), &e.to_string()))?;
        
        // Set timeout if specified
        if let Some(timeout) = self.config.timeout {
            stream.set_read_timeout(Some(timeout))
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "set_read_timeout", &e.to_string()))?;
            stream.set_write_timeout(Some(timeout))
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "set_write_timeout", &e.to_string()))?;
        self.stream = Some(stream);
        self.is_connected = true;
        self.register_socket();
        Ok(())
    fn connect_datagram(&mut self) -> IpcResult<()> {
        let datagram = UnixDatagram::unbound()
            .map_err(|e| unix_socket_error(Some(self.path.to_str()), "connect_datagram", &e.to_string()))?;
        
        datagram.connect(&self.path)
            .map_err(|e| connection_error(&self.path.to_string_lossy(), &e.to_string()))?;
        
        // Set timeout if specified
        if let Some(timeout) = self.config.timeout {
            datagram.set_read_timeout(Some(timeout))
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "set_read_timeout", &e.to_string()))?;
            datagram.set_write_timeout(Some(timeout))
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "set_write_timeout", &e.to_string()))?;
        self.datagram = Some(datagram);
        self.is_connected = true;
        self.register_socket();
        Ok(())
    fn send_stream(&mut self, data: &[u8]) -> IpcResult<usize> {
        if let Some(stream) = &mut self.stream {
            stream.write(data)
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "send_stream", &e.to_string()))
        } else {
            Err(unix_socket_error(Some(self.path.to_str()), "send_stream", "Stream socket not connected"))
        }
    }

    fn send_datagram(&mut self, data: &[u8]) -> IpcResult<usize> {
        if let Some(datagram) = &self.datagram {
            datagram.send(data)
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "send_datagram", &e.to_string()))
        } else {
            Err(unix_socket_error(Some(self.path.to_str()), "send_datagram", "Datagram socket not connected"))
        }
    }

    fn receive_stream(&mut self, buffer: &mut [u8]) -> IpcResult<usize> {
        if let Some(stream) = &mut self.stream {
            stream.read(buffer)
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "receive_stream", &e.to_string()))
        } else {
            Err(unix_socket_error(Some(self.path.to_str()), "receive_stream", "Stream socket not connected"))
        }
    }

    fn receive_datagram(&mut self, buffer: &mut [u8]) -> IpcResult<usize> {
        if let Some(datagram) = &self.datagram {
            datagram.recv(buffer)
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "receive_datagram", &e.to_string()))
        } else {
            Err(unix_socket_error(Some(self.path.to_str()), "receive_datagram", "Datagram socket not connected"))
        }
    }

    fn register_socket(&self) {
        let registry = get_socket_registry();
        if let Ok(mut sockets) = registry.write() {
            sockets.insert(
            );
        }
    }
impl Drop for UnixSocket {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

/// Unix socket server for handling multiple connections
#[derive(Debug)]
pub struct UnixSocketServer {
impl UnixSocketServer {
    /// Create a new Unix socket server
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
        }
    }

    /// Create with configuration
    pub fn with_config<P: AsRef<Path>>(path: P, config: UnixSocketConfig) -> Self {
        Self {
        }
    }

    /// Start listening for connections
    pub fn listen(&mut self) -> IpcResult<()> {
        // Remove existing socket file if requested
        if self.config.remove_existing && self.path.exists() {
            std::fs::remove_file(&self.path)
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "remove_socket", &e.to_string()))?;
        let listener = UnixListener::bind(&self.path)
            .map_err(|e| unix_socket_error(Some(self.path.to_str()), "bind", &e.to_string()))?;
        
        self.listener = Some(listener);
        self.is_listening = true;
        self.register_server();
        Ok(())
    /// Accept a connection
    pub fn accept(&mut self) -> IpcResult<UnixSocket> {
        if let Some(listener) = &self.listener {
            let (stream, _addr) = listener.accept()
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "accept", &e.to_string()))?;
            
            // Set timeout if specified
            if let Some(timeout) = self.config.timeout {
                stream.set_read_timeout(Some(timeout))
                    .map_err(|e| unix_socket_error(Some(self.path.to_str()), "set_read_timeout", &e.to_string()))?;
                stream.set_write_timeout(Some(timeout))
                    .map_err(|e| unix_socket_error(Some(self.path.to_str()), "set_write_timeout", &e.to_string()))?;
            let mut socket = UnixSocket::with_config(&self.path, self.config.clone());
            socket.stream = Some(stream);
            socket.is_connected = true;
            Ok(socket)
        } else {
            Err(unix_socket_error(Some(self.path.to_str()), "accept", "Server not listening"))
        }
    }

    /// Stop listening
    pub fn stop(&mut self) -> IpcResult<()> {
        if self.listener.is_some() {
            self.listener = None;
            self.is_listening = false;
            
            // Remove socket file
            if self.path.exists() {
                std::fs::remove_file(&self.path)
                    .map_err(|e| unix_socket_error(Some(self.path.to_str()), "remove_socket", &e.to_string()))?;
            }
        }
        Ok(())
    /// Get server path
    pub fn path(&self) -> &Path {
        &self.path
    /// Check if listening
    pub fn is_listening(&self) -> bool {
        self.is_listening
    fn register_server(&self) {
        let registry = get_socket_registry();
        if let Ok(mut sockets) = registry.write() {
            sockets.insert(
            );
        }
    }
impl Drop for UnixSocketServer {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Unix socket client for connecting to servers
#[derive(Debug)]
pub struct UnixSocketClient {
impl UnixSocketClient {
    /// Create a new Unix socket client
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create with configuration
    pub fn with_config(config: UnixSocketConfig) -> Self {
        Self {
        }
    }

    /// Connect to a Unix socket server
    pub fn connect<P: AsRef<Path>>(&self, path: P) -> IpcResult<UnixSocket> {
        let mut socket = UnixSocket::with_config(path, self.config.clone());
        socket.connect()?;
        Ok(socket)
    /// Connect with timeout
    pub fn connect_timeout<P: AsRef<Path>>(&self, path: P, timeout: Duration) -> IpcResult<UnixSocket> {
        let mut socket = UnixSocket::with_config(path, self.config.clone());
        socket.connect_timeout(timeout)?;
        Ok(socket)
    }
}

impl Default for UnixSocketClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Unix datagram socket server
#[derive(Debug)]
pub struct UnixDatagramServer {
impl UnixDatagramServer {
    /// Create a new Unix datagram server
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            config: UnixSocketConfig {
                ..Default::default()
        }
    }

    /// Bind to the socket path
    pub fn bind(&mut self) -> IpcResult<()> {
        // Remove existing socket file if requested
        if self.config.remove_existing && self.path.exists() {
            std::fs::remove_file(&self.path)
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "remove_socket", &e.to_string()))?;
        let socket = UnixDatagram::bind(&self.path)
            .map_err(|e| unix_socket_error(Some(self.path.to_str()), "bind", &e.to_string()))?;
        
        // Set timeout if specified
        if let Some(timeout) = self.config.timeout {
            socket.set_read_timeout(Some(timeout))
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "set_read_timeout", &e.to_string()))?;
            socket.set_write_timeout(Some(timeout))
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "set_write_timeout", &e.to_string()))?;
        self.socket = Some(socket);
        self.is_bound = true;
        self.register_server();
        Ok(())
    /// Receive data from any client
    pub fn receive_from(&mut self, buffer: &mut [u8]) -> IpcResult<(usize, PathBuf)> {
        if let Some(socket) = &self.socket {
            let (bytes_received, addr) = socket.recv_from(buffer)
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "recv_from", &e.to_string()))?;
            
            // Extract path from SocketAddr
            let path = addr.as_pathname()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("unknown"));
            
            Ok((bytes_received, path))
        } else {
            Err(unix_socket_error(Some(self.path.to_str()), "receive_from", "Socket not bound"))
        }
    }

    /// Send data to a specific client
    pub fn send_to<P: AsRef<Path>>(&mut self, data: &[u8], path: P) -> IpcResult<usize> {
        if let Some(socket) = &self.socket {
            socket.send_to(data, path)
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "send_to", &e.to_string()))
        } else {
            Err(unix_socket_error(Some(self.path.to_str()), "send_to", "Socket not bound"))
        }
    }

    /// Stop the server
    pub fn stop(&mut self) -> IpcResult<()> {
        if self.socket.is_some() {
            self.socket = None;
            self.is_bound = false;
            
            // Remove socket file
            if self.path.exists() {
                std::fs::remove_file(&self.path)
                    .map_err(|e| unix_socket_error(Some(self.path.to_str()), "remove_socket", &e.to_string()))?;
            }
        }
        Ok(())
    /// Get server path
    pub fn path(&self) -> &Path {
        &self.path
    /// Check if bound
    pub fn is_bound(&self) -> bool {
        self.is_bound
    fn register_server(&self) {
        let registry = get_socket_registry();
        if let Ok(mut sockets) = registry.write() {
            sockets.insert(
            );
        }
    }
impl Drop for UnixDatagramServer {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Cleanup all registered Unix sockets
pub fn cleanup_sockets() -> IpcResult<()> {
    let registry = get_socket_registry();
    if let Ok(mut sockets) = registry.write() {
        for (name, path) in sockets.drain() {
            if path.exists() {
                let _ = std::fs::remove_file(&path);
                tracing::debug!(socket_name = name, socket_path = ?path, "Cleaned up Unix socket file");
            }
        }
    }
    Ok(())
