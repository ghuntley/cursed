/// Unix domain sockets implementation for CURSED IPC
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::time::Duration;
use crate::stdlib::ipc::error::{IpcError, IpcResult};

#[cfg(unix)]
use std::os::unix::net::{UnixStream, UnixListener};

/// Socket configuration
#[derive(Debug, Clone)]
pub struct SocketConfig {
    pub address: SocketAddress,
    pub socket_type: SocketType,
    pub buffer_size: usize,
    pub timeout: Option<Duration>,
    pub permissions: u32,
}

impl SocketConfig {
    pub fn new(address: SocketAddress) -> Self {
        Self {
            address,
            socket_type: SocketType::Stream,
            buffer_size: 8192,
            timeout: None,
            permissions: 0o600,
        }
    }

    pub fn with_type(mut self, socket_type: SocketType) -> Self {
        self.socket_type = socket_type;
        self
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_permissions(mut self, permissions: u32) -> Self {
        self.permissions = permissions;
        self
    }
}

/// Socket address for Unix domain sockets
#[derive(Debug, Clone, PartialEq)]
pub enum SocketAddress {
    /// Filesystem path-based socket
    Path(PathBuf),
    /// Abstract namespace socket (Linux only)
    Abstract(String),
}

impl SocketAddress {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        SocketAddress::Path(path.as_ref().to_path_buf())
    }

    pub fn from_abstract<S: AsRef<str>>(name: S) -> Self {
        SocketAddress::Abstract(name.as_ref().to_string())
    }

    pub fn as_path(&self) -> Option<&Path> {
        match self {
            SocketAddress::Path(path) => Some(path),
            SocketAddress::Abstract(_) => None,
        }
    }

    pub fn as_abstract(&self) -> Option<&str> {
        match self {
            SocketAddress::Path(_) => None,
            SocketAddress::Abstract(name) => Some(name),
        }
    }
}

/// Socket type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SocketType {
    /// Stream socket (reliable, ordered)
    Stream,
    /// Datagram socket (unreliable, unordered)
    Datagram,
}

/// Unix domain socket wrapper
pub struct UnixSocket {
    config: SocketConfig,
    #[cfg(unix)]
    stream: Option<UnixStream>,
    reader: Option<BufReader<Box<dyn Read + Send>>>,
    writer: Option<BufWriter<Box<dyn Write + Send>>>,
}

impl UnixSocket {
    /// Connect to a Unix domain socket
    pub fn connect(address: SocketAddress) -> IpcResult<Self> {
        let config = SocketConfig::new(address);
        Self::connect_with_config(config)
    }

    /// Connect with custom configuration
    pub fn connect_with_config(config: SocketConfig) -> IpcResult<Self> {
        #[cfg(unix)]
        {
            let stream = match &config.address {
                SocketAddress::Path(path) => {
                    UnixStream::connect(path).map_err(IpcError::from)?
                }
                SocketAddress::Abstract(name) => {
                    // Abstract sockets on Linux
                    #[cfg(target_os = "linux")]
                    {
                        use std::os::unix::net::SocketAddr;
                        let addr = SocketAddr::from_abstract_name(name.as_bytes())
                            .map_err(|e| IpcError::InvalidInput(format!("Invalid abstract socket name: {}", e)))?;
                        UnixStream::connect_addr(&addr).map_err(IpcError::from)?
                    }
                    #[cfg(not(target_os = "linux"))]
                    {
                        return Err(IpcError::InvalidOperation("Abstract sockets not supported on this platform".to_string()));
                    }
                }
            };

            // Set timeout if specified
            if let Some(timeout) = config.timeout {
                stream.set_read_timeout(Some(timeout)).map_err(IpcError::from)?;
                stream.set_write_timeout(Some(timeout)).map_err(IpcError::from)?;
            }

            Ok(Self {
                config,
                stream: Some(stream),
                reader: None,
                writer: None,
            })
        }

        #[cfg(not(unix))]
        {
            Err(IpcError::InvalidOperation("Unix domain sockets not supported on this platform".to_string()))
        }
    }

    /// Bind to a Unix domain socket address
    pub fn bind(address: SocketAddress) -> IpcResult<UnixListener> {
        #[cfg(unix)]
        {
            // Remove existing socket file if it exists
            if let SocketAddress::Path(path) = &address {
                if path.exists() {
                    std::fs::remove_file(path).map_err(IpcError::from)?;
                }
            }

            let listener = match &address {
                SocketAddress::Path(path) => {
                    UnixListener::bind(path).map_err(IpcError::from)?
                }
                SocketAddress::Abstract(name) => {
                    #[cfg(target_os = "linux")]
                    {
                        use std::os::unix::net::SocketAddr;
                        let addr = SocketAddr::from_abstract_name(name.as_bytes())
                            .map_err(|e| IpcError::InvalidInput(format!("Invalid abstract socket name: {}", e)))?;
                        UnixListener::bind_addr(&addr).map_err(IpcError::from)?
                    }
                    #[cfg(not(target_os = "linux"))]
                    {
                        return Err(IpcError::InvalidOperation("Abstract sockets not supported on this platform".to_string()));
                    }
                }
            };

            // Register with IPC registry
            let addr_str = match &address {
                SocketAddress::Path(path) => path.to_string_lossy().to_string(),
                SocketAddress::Abstract(name) => format!("@{}", name),
            };
            crate::stdlib::ipc::register_socket(addr_str, address)?;

            Ok(listener)
        }

        #[cfg(not(unix))]
        {
            Err(IpcError::InvalidOperation("Unix domain sockets not supported on this platform".to_string()))
        }
    }

    /// Send data through the socket
    pub fn send(&mut self, data: &[u8]) -> IpcResult<usize> {
        #[cfg(unix)]
        {
            match &mut self.stream {
                Some(stream) => {
                    crate::stdlib::ipc::increment_operations();
                    stream.write(data).map_err(|e| {
                        crate::stdlib::ipc::increment_failed_operations();
                        IpcError::from(e)
                    })
                }
                None => {
                    crate::stdlib::ipc::increment_failed_operations();
                    Err(IpcError::InvalidOperation("Socket not connected".to_string()))
                }
            }
        }

        #[cfg(not(unix))]
        {
            Err(IpcError::InvalidOperation("Unix domain sockets not supported on this platform".to_string()))
        }
    }

    /// Send a string through the socket
    pub fn send_string(&mut self, s: &str) -> IpcResult<usize> {
        self.send(s.as_bytes())
    }

    /// Receive data from the socket
    pub fn receive(&mut self, buffer: &mut [u8]) -> IpcResult<usize> {
        #[cfg(unix)]
        {
            match &mut self.stream {
                Some(stream) => {
                    crate::stdlib::ipc::increment_operations();
                    stream.read(buffer).map_err(|e| {
                        crate::stdlib::ipc::increment_failed_operations();
                        IpcError::from(e)
                    })
                }
                None => {
                    crate::stdlib::ipc::increment_failed_operations();
                    Err(IpcError::InvalidOperation("Socket not connected".to_string()))
                }
            }
        }

        #[cfg(not(unix))]
        {
            Err(IpcError::InvalidOperation("Unix domain sockets not supported on this platform".to_string()))
        }
    }

    /// Receive a string from the socket
    pub fn receive_string(&mut self, max_size: usize) -> IpcResult<String> {
        let mut buffer = vec![0u8; max_size];
        let size = self.receive(&mut buffer)?;
        buffer.truncate(size);
        String::from_utf8(buffer).map_err(|e| IpcError::InvalidInput(format!("Invalid UTF-8: {}", e)))
    }

    /// Shutdown the socket
    pub fn shutdown(&mut self) -> IpcResult<()> {
        #[cfg(unix)]
        {
            if let Some(stream) = &self.stream {
                stream.shutdown(std::net::Shutdown::Both).map_err(IpcError::from)?;
            }
        }
        Ok(())
    }

    /// Get the socket address
    pub fn address(&self) -> &SocketAddress {
        &self.config.address
    }

    /// Check if socket is connected
    pub fn is_connected(&self) -> bool {
        #[cfg(unix)]
        {
            self.stream.is_some()
        }

        #[cfg(not(unix))]
        {
            false
        }
    }
}

impl Drop for UnixSocket {
    fn drop(&mut self) {
        let _ = self.shutdown();
        
        // Unregister from IPC registry
        let addr_str = match &self.config.address {
            SocketAddress::Path(path) => path.to_string_lossy().to_string(),
            SocketAddress::Abstract(name) => format!("@{}", name),
        };
        let _ = crate::stdlib::ipc::unregister_socket(&addr_str);
    }
}

/// Create a socket pair (connected sockets)
pub fn create_socket_pair() -> IpcResult<(UnixSocket, UnixSocket)> {
    #[cfg(unix)]
    {
        let (stream1, stream2) = UnixStream::pair().map_err(IpcError::from)?;
        
        let socket1 = UnixSocket {
            config: SocketConfig::new(SocketAddress::Abstract("pair1".to_string())),
            stream: Some(stream1),
            reader: None,
            writer: None,
        };
        
        let socket2 = UnixSocket {
            config: SocketConfig::new(SocketAddress::Abstract("pair2".to_string())),
            stream: Some(stream2),
            reader: None,
            writer: None,
        };
        
        Ok((socket1, socket2))
    }

    #[cfg(not(unix))]
    {
        Err(IpcError::InvalidOperation("Socket pairs not supported on this platform".to_string()))
    }
}

/// Remove a socket file
pub fn remove_socket<P: AsRef<Path>>(path: P) -> IpcResult<()> {
    let path = path.as_ref();
    if path.exists() {
        std::fs::remove_file(path).map_err(IpcError::from)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_socket_address() {
        let addr = SocketAddress::from_path("/tmp/test.sock");
        assert!(addr.as_path().is_some());
        assert!(addr.as_abstract().is_none());

        let addr = SocketAddress::from_abstract("test");
        assert!(addr.as_path().is_none());
        assert!(addr.as_abstract().is_some());
        assert_eq!(addr.as_abstract().unwrap(), "test");
    }

    #[test]
    fn test_socket_config() {
        let addr = SocketAddress::from_path("/tmp/test.sock");
        let config = SocketConfig::new(addr)
            .with_type(SocketType::Datagram)
            .with_buffer_size(4096)
            .with_timeout(Duration::from_secs(5))
            .with_permissions(0o644);

        assert_eq!(config.socket_type, SocketType::Datagram);
        assert_eq!(config.buffer_size, 4096);
        assert_eq!(config.timeout, Some(Duration::from_secs(5)));
        assert_eq!(config.permissions, 0o644);
    }

    #[cfg(unix)]
    #[test]
    fn test_socket_pair() {
        let result = create_socket_pair();
        assert!(result.is_ok());

        let (mut sock1, mut sock2) = result.unwrap();
        assert!(sock1.is_connected());
        assert!(sock2.is_connected());

        // Test communication
        let message = "Hello from socket 1";
        assert!(sock1.send_string(message).is_ok());

        let received = sock2.receive_string(1024);
        assert!(received.is_ok());
        assert_eq!(received.unwrap(), message);
    }

    #[cfg(unix)]
    #[test]
    fn test_socket_bind_connect() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let addr = SocketAddress::from_path(&socket_path);

        // Bind listener
        let listener = UnixSocket::bind(addr.clone());
        assert!(listener.is_ok());

        // Connect to socket
        let client = UnixSocket::connect(addr);
        assert!(client.is_ok());

        let client = client.unwrap();
        assert!(client.is_connected());
    }

    #[test]
    fn test_error_handling() {
        // Test connecting to non-existent socket
        let addr = SocketAddress::from_path("/non/existent/socket");
        let result = UnixSocket::connect(addr);
        assert!(result.is_err());
    }
}
