/// Production-ready Unix domain socket transport implementation
/// 
/// This module provides a high-performance, thread-safe Unix domain socket
/// transport with connection pooling, comprehensive error handling, and
/// integration with the existing IPC infrastructure.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicU64, AtomicUsize, Ordering}};
use std::time::{Duration, Instant, SystemTime};
use std::thread;
use std::io::{Read, Write, ErrorKind};
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{debug, info, warn, error, instrument, span, Level};

#[cfg(unix)]
use std::os::unix::net::{UnixStream, UnixListener, UnixDatagram};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};

use crate::stdlib::ipc::{
    IpcResult, IpcError, 
    communication_error_detailed, connection_failed, timeout_error, resource_error,
    system_error
};
use super::traits::{
    Transport, TransportConnection, TransportListener, StreamTransport, DatagramTransport
};
use super::pool::{TransportPool, PoolConfig, PoolStatistics};

/// Unix domain socket transport configuration
#[derive(Debug, Clone)]
pub struct UnixSocketConfig {
    pub socket_path: PathBuf,
    pub socket_type: UnixSocketType,
    pub permissions: u32,
    pub buffer_size: usize,
    pub read_timeout: Option<Duration>,
    pub write_timeout: Option<Duration>,
    pub nonblocking: bool,
    pub reuse_addr: bool,
    pub backlog: i32,
    pub enable_credentials: bool,
    pub enable_abstract_namespace: bool,
    pub cleanup_on_drop: bool,
    pub max_message_size: usize,
}

impl UnixSocketConfig {
    pub fn new<P: AsRef<Path>>(path: P, socket_type: UnixSocketType) -> Self {
        Self {
            socket_path: path.as_ref().to_path_buf(),
            socket_type,
            permissions: 0o755,
            buffer_size: 8192,
            read_timeout: Some(Duration::from_secs(30)),
            write_timeout: Some(Duration::from_secs(30)),
            nonblocking: false,
            reuse_addr: true,
            backlog: 128,
            enable_credentials: false,
            enable_abstract_namespace: false,
            cleanup_on_drop: true,
            max_message_size: 1024 * 1024, // 1MB
        }
    }

    pub fn stream<P: AsRef<Path>>(path: P) -> Self {
        Self::new(path, UnixSocketType::Stream)
    }

    pub fn datagram<P: AsRef<Path>>(path: P) -> Self {
        Self::new(path, UnixSocketType::Datagram)
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.read_timeout = Some(timeout);
        self.write_timeout = Some(timeout);
        self
    }

    pub fn with_nonblocking(mut self) -> Self {
        self.nonblocking = true;
        self
    }

    pub fn with_credentials(mut self) -> Self {
        self.enable_credentials = true;
        self
    }

    pub fn with_abstract_namespace(mut self) -> Self {
        self.enable_abstract_namespace = true;
        self
    }

    pub fn validate(&self) -> IpcResult<()> {
        if self.buffer_size == 0 {
            return Err(resource_error("Buffer size cannot be zero"));
        }
        if self.max_message_size == 0 {
            return Err(resource_error("Max message size cannot be zero"));
        }
        if self.backlog < 1 {
            return Err(resource_error("Backlog must be at least 1"));
        }
        Ok(())
    }
}

/// Unix socket type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnixSocketType {
    Stream,      // SOCK_STREAM - reliable, connection-oriented
    Datagram,    // SOCK_DGRAM - unreliable, connectionless
    Sequential,  // SOCK_SEQPACKET - reliable, connection-oriented, preserves message boundaries
}

/// Unix domain socket transport
#[derive(Debug)]
pub struct UnixSocketTransport {
    config: UnixSocketConfig,
    statistics: Arc<Mutex<TransportStatistics>>,
}

/// Unix socket connection wrapper
#[derive(Debug)]
pub struct UnixSocketConnection {
    stream: UnixStream,
    config: UnixSocketConfig,
    remote_addr: String,
    statistics: Arc<Mutex<ConnectionStatistics>>,
    created_at: Instant,
    last_activity: Arc<Mutex<Instant>>,
}

/// Unix socket listener wrapper
#[derive(Debug)]
pub struct UnixSocketListener {
    listener: UnixListener,
    config: UnixSocketConfig,
    local_addr: String,
    statistics: Arc<Mutex<ListenerStatistics>>,
    connection_count: Arc<AtomicUsize>,
}

/// Unix socket datagram implementation
#[derive(Debug)]
pub struct UnixSocketDatagram {
    socket: UnixDatagram,
    config: UnixSocketConfig,
    local_addr: String,
    statistics: Arc<Mutex<DatagramStatistics>>,
}

/// Transport statistics
#[derive(Debug, Clone)]
pub struct TransportStatistics {
    pub connections_created: u64,
    pub connections_failed: u64,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
    pub total_messages: u64,
    pub errors: u64,
    pub average_latency: Duration,
    pub peak_throughput: f64,
    pub active_connections: usize,
    pub peak_connections: usize,
}

impl TransportStatistics {
    fn new() -> Self {
        Self {
            connections_created: 0,
            connections_failed: 0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
            total_messages: 0,
            errors: 0,
            average_latency: Duration::from_micros(0),
            peak_throughput: 0.0,
            active_connections: 0,
            peak_connections: 0,
        }
    }
}

/// Connection statistics
#[derive(Debug, Clone)]
pub struct ConnectionStatistics {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub errors: u64,
    pub last_activity: SystemTime,
    pub latency_sum: Duration,
    pub latency_count: u64,
}

impl ConnectionStatistics {
    fn new() -> Self {
        Self {
            bytes_sent: 0,
            bytes_received: 0,
            messages_sent: 0,
            messages_received: 0,
            errors: 0,
            last_activity: SystemTime::now(),
            latency_sum: Duration::from_micros(0),
            latency_count: 0,
        }
    }

    fn record_send(&mut self, bytes: usize, latency: Duration) {
        self.bytes_sent += bytes as u64;
        self.messages_sent += 1;
        self.last_activity = SystemTime::now();
        self.latency_sum += latency;
        self.latency_count += 1;
    }

    fn record_receive(&mut self, bytes: usize, latency: Duration) {
        self.bytes_received += bytes as u64;
        self.messages_received += 1;
        self.last_activity = SystemTime::now();
        self.latency_sum += latency;
        self.latency_count += 1;
    }

    fn record_error(&mut self) {
        self.errors += 1;
        self.last_activity = SystemTime::now();
    }

    fn average_latency(&self) -> Duration {
        if self.latency_count > 0 {
            self.latency_sum / self.latency_count as u32
        } else {
            Duration::from_micros(0)
        }
    }
}

/// Listener statistics
#[derive(Debug, Clone)]
pub struct ListenerStatistics {
    pub connections_accepted: u64,
    pub connection_failures: u64,
    pub total_uptime: Duration,
    pub peak_connections: usize,
    pub current_connections: usize,
}

impl ListenerStatistics {
    fn new() -> Self {
        Self {
            connections_accepted: 0,
            connection_failures: 0,
            total_uptime: Duration::from_secs(0),
            peak_connections: 0,
            current_connections: 0,
        }
    }
}

/// Datagram statistics
#[derive(Debug, Clone)]
pub struct DatagramStatistics {
    pub datagrams_sent: u64,
    pub datagrams_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub errors: u64,
    pub last_activity: SystemTime,
}

impl DatagramStatistics {
    fn new() -> Self {
        Self {
            datagrams_sent: 0,
            datagrams_received: 0,
            bytes_sent: 0,
            bytes_received: 0,
            errors: 0,
            last_activity: SystemTime::now(),
        }
    }
}

/// Performance metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub average_latency: Duration,
    pub throughput_bytes_per_sec: f64,
    pub messages_per_sec: f64,
    pub error_rate: f64,
    pub connection_success_rate: f64,
}

impl UnixSocketTransport {
    /// Create a new Unix socket transport
    #[instrument]
    pub fn new(config: UnixSocketConfig) -> IpcResult<Self> {
        config.validate()?;

        #[cfg(not(unix))]
        {
            return Err(communication_error_detailed(
                "unix_socket",
                "new",
                "Unix domain sockets not supported on this platform"
            ));
        }

        let transport = Self {
            config,
            statistics: Arc::new(Mutex::new(TransportStatistics::new())),
        };

        info!(
            transport_type = "unix_socket",
            socket_path = ?transport.config.socket_path,
            socket_type = ?transport.config.socket_type,
            "Created Unix socket transport"
        );

        Ok(transport)
    }

    /// Get transport statistics
    pub fn get_statistics(&self) -> TransportStatistics {
        self.statistics.lock()
            .map(|stats| stats.clone())
            .unwrap_or_else(|_| TransportStatistics::new())
    }

    /// Clean up socket file if it exists
    fn cleanup_socket_file(path: &Path) -> IpcResult<()> {
        if path.exists() {
            fs::remove_file(path)
                .map_err(|e| system_error(
                    e.raw_os_error().unwrap_or(-1),
                    &format!("Failed to remove socket file: {}", path.display())
                ))?;
            debug!("Removed existing socket file: {}", path.display());
        }
        Ok(())
    }

    /// Set socket permissions
    fn set_socket_permissions(path: &Path, permissions: u32) -> IpcResult<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(path)
                .map_err(|e| system_error(
                    e.raw_os_error().unwrap_or(-1),
                    "Failed to get socket file metadata"
                ))?
                .permissions();
            perms.set_mode(permissions);
            fs::set_permissions(path, perms)
                .map_err(|e| system_error(
                    e.raw_os_error().unwrap_or(-1),
                    "Failed to set socket file permissions"
                ))?;
        }
        Ok(())
    }
}

impl Transport for UnixSocketTransport {
    type Connection = UnixSocketConnection;
    type Listener = UnixSocketListener;

    #[instrument(skip(self))]
    fn connect(&self, address: &str) -> IpcResult<Self::Connection> {
        let span = span!(Level::DEBUG, "unix_socket_connect", address = address);
        let _enter = span.enter();

        #[cfg(unix)]
        {
            let start_time = Instant::now();
            
            match UnixStream::connect(address) {
                Ok(stream) => {
                    // Configure the stream
                    if let Some(timeout) = self.config.read_timeout {
                        stream.set_read_timeout(Some(timeout))
                            .map_err(|e| communication_error_detailed(
                                "unix_socket", "connect", &format!("Failed to set read timeout: {}", e)
                            ))?;
                    }

                    if let Some(timeout) = self.config.write_timeout {
                        stream.set_write_timeout(Some(timeout))
                            .map_err(|e| communication_error_detailed(
                                "unix_socket", "connect", &format!("Failed to set write timeout: {}", e)
                            ))?;
                    }

                    if self.config.nonblocking {
                        stream.set_nonblocking(true)
                            .map_err(|e| communication_error_detailed(
                                "unix_socket", "connect", &format!("Failed to set nonblocking: {}", e)
                            ))?;
                    }

                    // Update statistics
                    if let Ok(mut stats) = self.statistics.lock() {
                        stats.connections_created += 1;
                        stats.active_connections += 1;
                        if stats.active_connections > stats.peak_connections {
                            stats.peak_connections = stats.active_connections;
                        }
                    }

                    let connection = UnixSocketConnection {
                        stream,
                        config: self.config.clone(),
                        remote_addr: address.to_string(),
                        statistics: Arc::new(Mutex::new(ConnectionStatistics::new())),
                        created_at: start_time,
                        last_activity: Arc::new(Mutex::new(Instant::now())),
                    };

                    debug!(
                        address = address,
                        duration_us = start_time.elapsed().as_micros(),
                        "Successfully connected to Unix socket"
                    );

                    Ok(connection)
                }
                Err(e) => {
                    // Update failure statistics
                    if let Ok(mut stats) = self.statistics.lock() {
                        stats.connections_failed += 1;
                        stats.errors += 1;
                    }

                    error!(
                        address = address,
                        error = %e,
                        "Failed to connect to Unix socket"
                    );

                    Err(connection_failed(address, &e.to_string()))
                }
            }
        }

        #[cfg(not(unix))]
        {
            Err(communication_error_detailed(
                "unix_socket",
                "connect",
                "Unix domain sockets not supported on this platform"
            ))
        }
    }

    #[instrument(skip(self))]
    fn bind(&self, address: &str) -> IpcResult<Self::Listener> {
        let span = span!(Level::DEBUG, "unix_socket_bind", address = address);
        let _enter = span.enter();

        #[cfg(unix)]
        {
            let path = Path::new(address);
            
            // Clean up existing socket file
            if self.config.cleanup_on_drop {
                Self::cleanup_socket_file(path)?;
            }

            match UnixListener::bind(path) {
                Ok(listener) => {
                    // Set permissions
                    Self::set_socket_permissions(path, self.config.permissions)?;

                    // Configure the listener
                    if self.config.nonblocking {
                        listener.set_nonblocking(true)
                            .map_err(|e| communication_error_detailed(
                                "unix_socket", "bind", &format!("Failed to set nonblocking: {}", e)
                            ))?;
                    }

                    let socket_listener = UnixSocketListener {
                        listener,
                        config: self.config.clone(),
                        local_addr: address.to_string(),
                        statistics: Arc::new(Mutex::new(ListenerStatistics::new())),
                        connection_count: Arc::new(AtomicUsize::new(0)),
                    };

                    info!(
                        address = address,
                        permissions = format!("{:o}", self.config.permissions),
                        "Successfully bound Unix socket listener"
                    );

                    Ok(socket_listener)
                }
                Err(e) => {
                    error!(
                        address = address,
                        error = %e,
                        "Failed to bind Unix socket listener"
                    );

                    Err(communication_error_detailed(
                        "unix_socket",
                        "bind",
                        &format!("Failed to bind to {}: {}", address, e)
                    ))
                }
            }
        }

        #[cfg(not(unix))]
        {
            Err(communication_error_detailed(
                "unix_socket",
                "bind",
                "Unix domain sockets not supported on this platform"
            ))
        }
    }

    fn is_available() -> bool {
        cfg!(unix)
    }

    fn name(&self) -> &'static str {
        "unix_socket"
    }
}

impl StreamTransport for UnixSocketTransport {
    fn max_message_size(&self) -> usize {
        self.config.max_message_size
    }

    fn preserves_message_boundaries(&self) -> bool {
        self.config.socket_type == UnixSocketType::Sequential
    }
}

impl TransportConnection for UnixSocketConnection {
    #[instrument(skip(self, buffer))]
    fn read(&mut self, buffer: &mut [u8]) -> IpcResult<usize> {
        let start_time = Instant::now();
        
        match self.stream.read(buffer) {
            Ok(bytes_read) => {
                let latency = start_time.elapsed();
                
                // Update statistics
                if let Ok(mut stats) = self.statistics.lock() {
                    stats.record_receive(bytes_read, latency);
                }
                
                // Update last activity
                if let Ok(mut last_activity) = self.last_activity.lock() {
                    *last_activity = Instant::now();
                }

                debug!(
                    bytes_read = bytes_read,
                    latency_us = latency.as_micros(),
                    "Read data from Unix socket"
                );

                Ok(bytes_read)
            }
            Err(e) => {
                // Update error statistics
                if let Ok(mut stats) = self.statistics.lock() {
                    stats.record_error();
                }

                error!(
                    error = %e,
                    remote_addr = %self.remote_addr,
                    "Failed to read from Unix socket"
                );

                Err(communication_error_detailed(
                    "unix_socket",
                    "read",
                    &e.to_string()
                ))
            }
        }
    }

    #[instrument(skip(self, data))]
    fn write(&mut self, data: &[u8]) -> IpcResult<usize> {
        let start_time = Instant::now();
        
        match self.stream.write(data) {
            Ok(bytes_written) => {
                let latency = start_time.elapsed();
                
                // Update statistics
                if let Ok(mut stats) = self.statistics.lock() {
                    stats.record_send(bytes_written, latency);
                }
                
                // Update last activity
                if let Ok(mut last_activity) = self.last_activity.lock() {
                    *last_activity = Instant::now();
                }

                debug!(
                    bytes_written = bytes_written,
                    latency_us = latency.as_micros(),
                    "Wrote data to Unix socket"
                );

                Ok(bytes_written)
            }
            Err(e) => {
                // Update error statistics
                if let Ok(mut stats) = self.statistics.lock() {
                    stats.record_error();
                }

                error!(
                    error = %e,
                    remote_addr = %self.remote_addr,
                    bytes_attempted = data.len(),
                    "Failed to write to Unix socket"
                );

                Err(communication_error_detailed(
                    "unix_socket",
                    "write",
                    &e.to_string()
                ))
            }
        }
    }

    fn flush(&mut self) -> IpcResult<()> {
        self.stream.flush()
            .map_err(|e| communication_error_detailed(
                "unix_socket",
                "flush",
                &e.to_string()
            ))
    }

    fn close(&mut self) -> IpcResult<()> {
        debug!(remote_addr = %self.remote_addr, "Closing Unix socket connection");
        
        // The UnixStream will be closed when dropped
        // Update last activity
        if let Ok(mut last_activity) = self.last_activity.lock() {
            *last_activity = Instant::now();
        }
        
        Ok(())
    }

    fn is_active(&self) -> bool {
        // For Unix sockets, we can check if the peer is still connected
        // by attempting a zero-byte write (this is a common technique)
        match self.stream.write(&[]) {
            Ok(_) => true,
            Err(e) => match e.kind() {
                ErrorKind::BrokenPipe | ErrorKind::ConnectionAborted | ErrorKind::ConnectionReset => false,
                _ => true, // Assume active for other errors
            }
        }
    }

    fn remote_address(&self) -> Option<String> {
        Some(self.remote_addr.clone())
    }

    fn set_read_timeout(&mut self, timeout: Option<Duration>) -> IpcResult<()> {
        self.stream.set_read_timeout(timeout)
            .map_err(|e| communication_error_detailed(
                "unix_socket",
                "set_read_timeout",
                &e.to_string()
            ))
    }

    fn set_write_timeout(&mut self, timeout: Option<Duration>) -> IpcResult<()> {
        self.stream.set_write_timeout(timeout)
            .map_err(|e| communication_error_detailed(
                "unix_socket",
                "set_write_timeout",
                &e.to_string()
            ))
    }

    fn try_clone(&self) -> IpcResult<Box<dyn TransportConnection>> {
        match self.stream.try_clone() {
            Ok(cloned_stream) => {
                let cloned_connection = UnixSocketConnection {
                    stream: cloned_stream,
                    config: self.config.clone(),
                    remote_addr: self.remote_addr.clone(),
                    statistics: self.statistics.clone(),
                    created_at: self.created_at,
                    last_activity: self.last_activity.clone(),
                };
                Ok(Box::new(cloned_connection))
            }
            Err(e) => Err(communication_error_detailed(
                "unix_socket",
                "clone",
                &e.to_string()
            ))
        }
    }
}

impl Read for UnixSocketConnection {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let result = self.stream.read(buf);
        
        // Update last activity on successful read
        if result.is_ok() {
            if let Ok(mut last_activity) = self.last_activity.lock() {
                *last_activity = Instant::now();
            }
        }
        
        result
    }
}

impl Write for UnixSocketConnection {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let result = self.stream.write(buf);
        
        // Update last activity on successful write
        if result.is_ok() {
            if let Ok(mut last_activity) = self.last_activity.lock() {
                *last_activity = Instant::now();
            }
        }
        
        result
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}

impl TransportListener for UnixSocketListener {
    type Connection = UnixSocketConnection;

    #[instrument(skip(self))]
    fn accept(&mut self) -> IpcResult<Self::Connection> {
        let span = span!(Level::DEBUG, "unix_socket_accept", local_addr = %self.local_addr);
        let _enter = span.enter();

        match self.listener.accept() {
            Ok((stream, _addr)) => {
                // Configure the accepted stream
                if let Some(timeout) = self.config.read_timeout {
                    let _ = stream.set_read_timeout(Some(timeout));
                }

                if let Some(timeout) = self.config.write_timeout {
                    let _ = stream.set_write_timeout(Some(timeout));
                }

                if self.config.nonblocking {
                    let _ = stream.set_nonblocking(true);
                }

                // Update statistics
                if let Ok(mut stats) = self.statistics.lock() {
                    stats.connections_accepted += 1;
                    stats.current_connections += 1;
                    if stats.current_connections > stats.peak_connections {
                        stats.peak_connections = stats.current_connections;
                    }
                }

                let count = self.connection_count.fetch_add(1, Ordering::Relaxed) + 1;

                let connection = UnixSocketConnection {
                    stream,
                    config: self.config.clone(),
                    remote_addr: format!("unix:unknown:{}", count),
                    statistics: Arc::new(Mutex::new(ConnectionStatistics::new())),
                    created_at: Instant::now(),
                    last_activity: Arc::new(Mutex::new(Instant::now())),
                };

                debug!(
                    local_addr = %self.local_addr,
                    connection_count = count,
                    "Accepted new Unix socket connection"
                );

                Ok(connection)
            }
            Err(e) => {
                // Update failure statistics
                if let Ok(mut stats) = self.statistics.lock() {
                    stats.connection_failures += 1;
                }

                error!(
                    local_addr = %self.local_addr,
                    error = %e,
                    "Failed to accept Unix socket connection"
                );

                Err(communication_error_detailed(
                    "unix_socket",
                    "accept",
                    &e.to_string()
                ))
            }
        }
    }

    fn set_nonblocking(&mut self, nonblocking: bool) -> IpcResult<()> {
        self.listener.set_nonblocking(nonblocking)
            .map_err(|e| communication_error_detailed(
                "unix_socket",
                "set_nonblocking",
                &e.to_string()
            ))
    }

    fn local_address(&self) -> Option<String> {
        Some(self.local_addr.clone())
    }

    fn close(&mut self) -> IpcResult<()> {
        debug!(local_addr = %self.local_addr, "Closing Unix socket listener");
        
        // Clean up socket file if configured to do so
        if self.config.cleanup_on_drop {
            let path = Path::new(&self.local_addr);
            if path.exists() {
                let _ = fs::remove_file(path);
                debug!("Removed socket file: {}", path.display());
            }
        }
        
        Ok(())
    }
}

impl Drop for UnixSocketListener {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

/// Unix socket connection pool
pub type UnixSocketPool = TransportPool<UnixSocketConnection>;

/// Connection pool for Unix sockets
pub struct ConnectionPool {
    pool: UnixSocketPool,
}

impl ConnectionPool {
    /// Create a new connection pool
    pub fn new(transport: Arc<UnixSocketTransport>, config: PoolConfig) -> IpcResult<Self> {
        let pool = TransportPool::new(transport, config)?;
        Ok(Self { pool })
    }

    /// Get a connection from the pool
    pub fn get_connection(&self, address: &str) -> IpcResult<super::pool::PooledConnection<UnixSocketConnection>> {
        self.pool.get_connection(address)
    }

    /// Get pool statistics
    pub fn get_statistics(&self) -> PoolStatistics {
        self.pool.get_statistics()
    }

    /// Shutdown the pool
    pub fn shutdown(&self) -> IpcResult<()> {
        self.pool.shutdown()
    }
}

// Global state management
lazy_static::lazy_static! {
    static ref GLOBAL_TRANSPORT_STATS: Arc<Mutex<HashMap<String, TransportStatistics>>> = 
        Arc::new(Mutex::new(HashMap::new()));
    
    static ref ACTIVE_CONNECTIONS: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    static ref TOTAL_BYTES_TRANSFERRED: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    static ref ERROR_COUNT: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
}

/// Module-level functions for global management

/// Initialize Unix transport subsystem
pub fn initialize_unix_transport() -> IpcResult<()> {
    info!("Initializing Unix domain socket transport subsystem");
    Ok(())
}

/// Cleanup Unix transport subsystem
pub fn cleanup_unix_transport() -> IpcResult<()> {
    info!("Cleaning up Unix domain socket transport subsystem");
    
    // Clear global statistics
    GLOBAL_TRANSPORT_STATS.lock().unwrap().clear();
    ACTIVE_CONNECTIONS.store(0, Ordering::Relaxed);
    TOTAL_BYTES_TRANSFERRED.store(0, Ordering::Relaxed);
    ERROR_COUNT.store(0, Ordering::Relaxed);
    
    Ok(())
}

/// Get active connection count
pub fn get_active_connection_count() -> usize {
    ACTIVE_CONNECTIONS.load(Ordering::Relaxed)
}

/// Get total bytes transferred
pub fn get_total_bytes_transferred() -> u64 {
    TOTAL_BYTES_TRANSFERRED.load(Ordering::Relaxed)
}

/// Get error count
pub fn get_error_count() -> u64 {
    ERROR_COUNT.load(Ordering::Relaxed)
}

/// Get performance metrics
pub fn get_performance_metrics() -> PerformanceMetrics {
    PerformanceMetrics {
        average_latency: Duration::from_micros(0),
        throughput_bytes_per_sec: 0.0,
        messages_per_sec: 0.0,
        error_rate: 0.0,
        connection_success_rate: 1.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_unix_socket_config() {
        let config = UnixSocketConfig::stream("/tmp/test.sock")
            .with_buffer_size(16384)
            .with_timeout(Duration::from_secs(10))
            .with_nonblocking()
            .with_credentials();

        assert_eq!(config.socket_path, PathBuf::from("/tmp/test.sock"));
        assert_eq!(config.socket_type, UnixSocketType::Stream);
        assert_eq!(config.buffer_size, 16384);
        assert_eq!(config.read_timeout, Some(Duration::from_secs(10)));
        assert!(config.nonblocking);
        assert!(config.enable_credentials);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_unix_socket_config_validation() {
        let invalid_config = UnixSocketConfig::stream("/tmp/test.sock")
            .with_buffer_size(0);
        assert!(invalid_config.validate().is_err());

        let mut invalid_config2 = UnixSocketConfig::stream("/tmp/test.sock");
        invalid_config2.max_message_size = 0;
        assert!(invalid_config2.validate().is_err());

        let mut invalid_config3 = UnixSocketConfig::stream("/tmp/test.sock");
        invalid_config3.backlog = 0;
        assert!(invalid_config3.validate().is_err());
    }

    #[test]
    fn test_socket_types() {
        assert_eq!(UnixSocketType::Stream, UnixSocketType::Stream);
        assert_eq!(UnixSocketType::Datagram, UnixSocketType::Datagram);
        assert_eq!(UnixSocketType::Sequential, UnixSocketType::Sequential);
        assert_ne!(UnixSocketType::Stream, UnixSocketType::Datagram);
    }

    #[test]
    fn test_transport_availability() {
        let available = UnixSocketTransport::is_available();
        #[cfg(unix)]
        assert!(available);
        #[cfg(not(unix))]
        assert!(!available);
    }

    #[test]
    fn test_statistics() {
        let mut stats = TransportStatistics::new();
        assert_eq!(stats.connections_created, 0);
        assert_eq!(stats.total_bytes_sent, 0);

        stats.connections_created += 1;
        stats.total_bytes_sent += 1024;
        assert_eq!(stats.connections_created, 1);
        assert_eq!(stats.total_bytes_sent, 1024);

        let mut conn_stats = ConnectionStatistics::new();
        conn_stats.record_send(512, Duration::from_millis(10));
        assert_eq!(conn_stats.bytes_sent, 512);
        assert_eq!(conn_stats.messages_sent, 1);
        assert!(conn_stats.average_latency() >= Duration::from_millis(10));

        conn_stats.record_receive(256, Duration::from_millis(5));
        assert_eq!(conn_stats.bytes_received, 256);
        assert_eq!(conn_stats.messages_received, 1);
    }

    #[test]
    fn test_global_functions() {
        assert!(initialize_unix_transport().is_ok());
        assert_eq!(get_active_connection_count(), 0);
        assert_eq!(get_total_bytes_transferred(), 0);
        assert_eq!(get_error_count(), 0);
        assert!(cleanup_unix_transport().is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn test_unix_socket_transport_creation() {
        let config = UnixSocketConfig::stream("/tmp/test_transport.sock");
        let result = UnixSocketTransport::new(config);
        assert!(result.is_ok());

        let transport = result.unwrap();
        assert_eq!(transport.name(), "unix_socket");
        assert!(UnixSocketTransport::is_available());

        let stats = transport.get_statistics();
        assert_eq!(stats.connections_created, 0);
    }

    #[cfg(unix)]
    #[test]
    fn test_socket_cleanup() {
        use std::fs::File;
        
        let socket_path = "/tmp/test_cleanup.sock";
        
        // Create a dummy file
        File::create(socket_path).unwrap();
        assert!(Path::new(socket_path).exists());
        
        // Test cleanup
        UnixSocketTransport::cleanup_socket_file(Path::new(socket_path)).unwrap();
        assert!(!Path::new(socket_path).exists());
    }

    #[cfg(unix)]
    #[test]
    fn test_basic_connect_fail() {
        let config = UnixSocketConfig::stream("/nonexistent/path/test.sock");
        let transport = UnixSocketTransport::new(config).unwrap();
        
        let result = transport.connect("/nonexistent/path/test.sock");
        assert!(result.is_err());
        
        let stats = transport.get_statistics();
        assert_eq!(stats.connections_failed, 1);
    }

    #[cfg(unix)]
    #[test]
    fn integration_test_socket_communication() {
        let socket_path = "/tmp/test_integration.sock";
        let config = UnixSocketConfig::stream(socket_path);
        let transport = Arc::new(UnixSocketTransport::new(config).unwrap());
        
        // Start server in a separate thread
        let server_transport = transport.clone();
        let server_handle = thread::spawn(move || {
            let mut listener = server_transport.bind(socket_path).unwrap();
            let mut connection = listener.accept().unwrap();
            
            let mut buffer = [0u8; 1024];
            let bytes_read = connection.read(&mut buffer).unwrap();
            assert!(bytes_read > 0);
            
            connection.write(b"Hello from server").unwrap();
            connection.flush().unwrap();
        });
        
        // Give server time to start
        thread::sleep(Duration::from_millis(100));
        
        // Connect as client
        let mut client_connection = transport.connect(socket_path).unwrap();
        client_connection.write(b"Hello from client").unwrap();
        client_connection.flush().unwrap();
        
        let mut buffer = [0u8; 1024];
        let bytes_read = client_connection.read(&mut buffer).unwrap();
        assert!(bytes_read > 0);
        assert_eq!(&buffer[..bytes_read], b"Hello from server");
        
        // Wait for server to complete
        server_handle.join().unwrap();
        
        // Cleanup
        let _ = fs::remove_file(socket_path);
    }
}
