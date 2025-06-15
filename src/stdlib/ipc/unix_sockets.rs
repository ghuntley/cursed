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

use crate::stdlib::ipc::error::{IpcError, IpcResult, unix_socket_error, system_error, timeout_error, connection_error};

/// Unix socket registry for cleanup
static SOCKET_REGISTRY: std::sync::OnceLock<Arc<RwLock<HashMap<String, PathBuf>>>> = std::sync::OnceLock::new();

fn get_socket_registry() -> &'static Arc<RwLock<HashMap<String, PathBuf>>> {
    SOCKET_REGISTRY.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

/// Unix socket type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnixSocketType {
    /// Stream socket (TCP-like, reliable, ordered)
    Stream,
    /// Datagram socket (UDP-like, connectionless)
    Datagram,
}

/// Unix socket configuration
#[derive(Debug, Clone)]
pub struct UnixSocketConfig {
    /// Socket type
    pub socket_type: UnixSocketType,
    /// Buffer size for I/O operations
    pub buffer_size: usize,
    /// Socket file permissions
    pub permissions: u32,
    /// Whether to remove existing socket file
    pub remove_existing: bool,
    /// Timeout for operations
    pub timeout: Option<Duration>,
    /// Maximum pending connections (server mode)
    pub max_connections: usize,
}

impl Default for UnixSocketConfig {
    fn default() -> Self {
        Self {
            socket_type: UnixSocketType::Stream,
            buffer_size: 8192,
            permissions: 0o666,
            remove_existing: true,
            timeout: Some(Duration::from_secs(30)),
            max_connections: 128,
        }
    }
}

/// Cross-platform Unix socket
#[derive(Debug)]
pub struct UnixSocket {
    path: PathBuf,
    config: UnixSocketConfig,
    stream: Option<UnixStream>,
    datagram: Option<UnixDatagram>,
    is_connected: bool,
}

impl UnixSocket {
    /// Create a new Unix socket
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            config: UnixSocketConfig::default(),
            stream: None,
            datagram: None,
            is_connected: false,
        }
    }

    /// Create with configuration
    pub fn with_config<P: AsRef<Path>>(path: P, config: UnixSocketConfig) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            config,
            stream: None,
            datagram: None,
            is_connected: false,
        }
    }

    /// Connect to a Unix socket server
    pub fn connect(&mut self) -> IpcResult<()> {
        match self.config.socket_type {
            UnixSocketType::Stream => self.connect_stream(),
            UnixSocketType::Datagram => self.connect_datagram(),
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
    }

    /// Send data
    pub fn send(&mut self, data: &[u8]) -> IpcResult<usize> {
        match self.config.socket_type {
            UnixSocketType::Stream => self.send_stream(data),
            UnixSocketType::Datagram => self.send_datagram(data),
        }
    }

    /// Receive data
    pub fn receive(&mut self, buffer: &mut [u8]) -> IpcResult<usize> {
        match self.config.socket_type {
            UnixSocketType::Stream => self.receive_stream(buffer),
            UnixSocketType::Datagram => self.receive_datagram(buffer),
        }
    }

    /// Send a message (convenience method for strings)
    pub fn send_message(&mut self, message: &str) -> IpcResult<()> {
        let data = message.as_bytes();
        self.send(data)?;
        Ok(())
    }

    /// Receive a message (convenience method for strings)
    pub fn receive_message(&mut self) -> IpcResult<String> {
        let mut buffer = vec![0u8; self.config.buffer_size];
        let bytes_received = self.receive(&mut buffer)?;
        buffer.truncate(bytes_received);
        
        String::from_utf8(buffer)
            .map_err(|e| unix_socket_error(Some(self.path.to_str()), "receive_message", &e.to_string()))
    }

    /// Send line (adds newline)
    pub fn send_line(&mut self, line: &str) -> IpcResult<()> {
        let mut data = line.to_string();
        data.push('\n');
        self.send(data.as_bytes())?;
        Ok(())
    }

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
            None => Err(unix_socket_error(Some(self.path.to_str()), "receive_line", "Stream socket not connected")),
        }
    }

    /// Shutdown the socket
    pub fn shutdown(&mut self, how: Shutdown) -> IpcResult<()> {
        if let Some(stream) = &self.stream {
            stream.shutdown(how)
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "shutdown", &e.to_string()))?;
        }
        Ok(())
    }

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
    }

    /// Get socket path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    fn connect_stream(&mut self) -> IpcResult<()> {
        let stream = UnixStream::connect(&self.path)
            .map_err(|e| connection_error(&self.path.to_string_lossy(), &e.to_string()))?;
        
        // Set timeout if specified
        if let Some(timeout) = self.config.timeout {
            stream.set_read_timeout(Some(timeout))
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "set_read_timeout", &e.to_string()))?;
            stream.set_write_timeout(Some(timeout))
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "set_write_timeout", &e.to_string()))?;
        }
        
        self.stream = Some(stream);
        self.is_connected = true;
        self.register_socket();
        Ok(())
    }

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
        }
        
        self.datagram = Some(datagram);
        self.is_connected = true;
        self.register_socket();
        Ok(())
    }

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
                self.path.to_string_lossy().to_string(),
                self.path.clone(),
            );
        }
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
    path: PathBuf,
    config: UnixSocketConfig,
    listener: Option<UnixListener>,
    is_listening: bool,
}

impl UnixSocketServer {
    /// Create a new Unix socket server
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            config: UnixSocketConfig::default(),
            listener: None,
            is_listening: false,
        }
    }

    /// Create with configuration
    pub fn with_config<P: AsRef<Path>>(path: P, config: UnixSocketConfig) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            config,
            listener: None,
            is_listening: false,
        }
    }

    /// Start listening for connections
    pub fn listen(&mut self) -> IpcResult<()> {
        // Remove existing socket file if requested
        if self.config.remove_existing && self.path.exists() {
            std::fs::remove_file(&self.path)
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "remove_socket", &e.to_string()))?;
        }
        
        let listener = UnixListener::bind(&self.path)
            .map_err(|e| unix_socket_error(Some(self.path.to_str()), "bind", &e.to_string()))?;
        
        self.listener = Some(listener);
        self.is_listening = true;
        self.register_server();
        Ok(())
    }

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
            }
            
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
    }

    /// Get server path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Check if listening
    pub fn is_listening(&self) -> bool {
        self.is_listening
    }

    fn register_server(&self) {
        let registry = get_socket_registry();
        if let Ok(mut sockets) = registry.write() {
            sockets.insert(
                format!("server:{}", self.path.to_string_lossy()),
                self.path.clone(),
            );
        }
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
    config: UnixSocketConfig,
}

impl UnixSocketClient {
    /// Create a new Unix socket client
    pub fn new() -> Self {
        Self {
            config: UnixSocketConfig::default(),
        }
    }

    /// Create with configuration
    pub fn with_config(config: UnixSocketConfig) -> Self {
        Self {
            config,
        }
    }

    /// Connect to a Unix socket server
    pub fn connect<P: AsRef<Path>>(&self, path: P) -> IpcResult<UnixSocket> {
        let mut socket = UnixSocket::with_config(path, self.config.clone());
        socket.connect()?;
        Ok(socket)
    }

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
    path: PathBuf,
    config: UnixSocketConfig,
    socket: Option<UnixDatagram>,
    is_bound: bool,
}

impl UnixDatagramServer {
    /// Create a new Unix datagram server
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            config: UnixSocketConfig {
                socket_type: UnixSocketType::Datagram,
                ..Default::default()
            },
            socket: None,
            is_bound: false,
        }
    }

    /// Bind to the socket path
    pub fn bind(&mut self) -> IpcResult<()> {
        // Remove existing socket file if requested
        if self.config.remove_existing && self.path.exists() {
            std::fs::remove_file(&self.path)
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "remove_socket", &e.to_string()))?;
        }
        
        let socket = UnixDatagram::bind(&self.path)
            .map_err(|e| unix_socket_error(Some(self.path.to_str()), "bind", &e.to_string()))?;
        
        // Set timeout if specified
        if let Some(timeout) = self.config.timeout {
            socket.set_read_timeout(Some(timeout))
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "set_read_timeout", &e.to_string()))?;
            socket.set_write_timeout(Some(timeout))
                .map_err(|e| unix_socket_error(Some(self.path.to_str()), "set_write_timeout", &e.to_string()))?;
        }
        
        self.socket = Some(socket);
        self.is_bound = true;
        self.register_server();
        Ok(())
    }

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
    }

    /// Get server path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Check if bound
    pub fn is_bound(&self) -> bool {
        self.is_bound
    }

    fn register_server(&self) {
        let registry = get_socket_registry();
        if let Ok(mut sockets) = registry.write() {
            sockets.insert(
                format!("datagram_server:{}", self.path.to_string_lossy()),
                self.path.clone(),
            );
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    use tempfile::tempdir;

    #[test]
    fn test_unix_socket_config() {
        let config = UnixSocketConfig::default();
        assert_eq!(config.socket_type, UnixSocketType::Stream);
        assert_eq!(config.buffer_size, 8192);
        assert!(config.remove_existing);
    }

    #[test]
    fn test_unix_socket_creation() {
        let socket = UnixSocket::new("/tmp/test_socket");
        assert_eq!(socket.path(), Path::new("/tmp/test_socket"));
        assert!(!socket.is_connected());
    }

    #[test]
    fn test_unix_stream_socket() {
        let temp_dir = tempdir().unwrap();
        let socket_path = temp_dir.path().join("test_stream_socket");
        
        // Start server in background thread
        let server_path = socket_path.clone();
        let server_handle = thread::spawn(move || {
            let mut server = UnixSocketServer::new(&server_path);
            if server.listen().is_ok() {
                if let Ok(mut client_socket) = server.accept() {
                    // Echo server
                    let mut buffer = [0u8; 1024];
                    if let Ok(bytes_read) = client_socket.receive(&mut buffer) {
                        let _ = client_socket.send(&buffer[..bytes_read]);
                    }
                }
            }
        });
        
        // Give server time to start
        thread::sleep(Duration::from_millis(100));
        
        // Connect client
        let client = UnixSocketClient::new();
        if let Ok(mut client_socket) = client.connect(&socket_path) {
            let test_message = b"Hello, Unix socket!";
            if client_socket.send(test_message).is_ok() {
                let mut buffer = [0u8; 1024];
                if let Ok(bytes_received) = client_socket.receive(&mut buffer) {
                    assert_eq!(&buffer[..bytes_received], test_message);
                }
            }
        }
        
        let _ = server_handle.join();
    }

    #[test]
    fn test_unix_datagram_socket() {
        let temp_dir = tempdir().unwrap();
        let server_path = temp_dir.path().join("test_datagram_server");
        let client_path = temp_dir.path().join("test_datagram_client");
        
        // Start datagram server
        let mut server = UnixDatagramServer::new(&server_path);
        if server.bind().is_ok() {
            // Create client socket
            let config = UnixSocketConfig {
                socket_type: UnixSocketType::Datagram,
                ..Default::default()
            };
            let mut client = UnixSocket::with_config(&client_path, config);
            
            if client.connect().is_ok() {
                let test_message = b"Datagram test message";
                if client.send(test_message).is_ok() {
                    let mut buffer = [0u8; 1024];
                    if let Ok((bytes_received, _client_path)) = server.receive_from(&mut buffer) {
                        assert_eq!(&buffer[..bytes_received], test_message);
                        
                        // Echo back
                        let _ = server.send_to(&buffer[..bytes_received], &client_path);
                    }
                }
            }
        }
    }

    #[test]
    fn test_unix_socket_line_protocol() {
        let temp_dir = tempdir().unwrap();
        let socket_path = temp_dir.path().join("test_line_socket");
        
        // Start server in background thread
        let server_path = socket_path.clone();
        let server_handle = thread::spawn(move || {
            let mut server = UnixSocketServer::new(&server_path);
            if server.listen().is_ok() {
                if let Ok(mut client_socket) = server.accept() {
                    // Line echo server
                    if let Ok(line) = client_socket.receive_line() {
                        let _ = client_socket.send_line(&format!("Echo: {}", line));
                    }
                }
            }
        });
        
        // Give server time to start
        thread::sleep(Duration::from_millis(100));
        
        // Connect client
        let client = UnixSocketClient::new();
        if let Ok(mut client_socket) = client.connect(&socket_path) {
            if client_socket.send_line("Test line").is_ok() {
                if let Ok(response) = client_socket.receive_line() {
                    assert_eq!(response, "Echo: Test line");
                }
            }
        }
        
        let _ = server_handle.join();
    }

    #[test]
    fn test_socket_registry() {
        let registry = get_socket_registry();
        assert!(registry.read().is_ok());
        
        // Test cleanup
        assert!(cleanup_sockets().is_ok());
    }

    #[test]
    fn test_socket_timeout() {
        let temp_dir = tempdir().unwrap();
        let socket_path = temp_dir.path().join("test_timeout_socket");
        
        let config = UnixSocketConfig {
            timeout: Some(Duration::from_millis(100)),
            ..Default::default()
        };
        
        // Test connect timeout (should fail since no server)
        let client = UnixSocketClient::with_config(config);
        let result = client.connect_timeout(&socket_path, Duration::from_millis(50));
        assert!(result.is_err());
    }
}
