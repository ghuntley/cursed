use crate::error::CursedError;
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

// Placeholder imports disabled
    system_error
// };

use super::traits::{
    Transport, TransportConnection, TransportListener, StreamTransport, DatagramTransport
// };

use super::pool::{TransportPool, PoolConfig, PoolStatistics};

/// Unix domain socket transport configuration
#[derive(Debug, Clone)]
pub struct UnixSocketConfig {
impl UnixSocketConfig {
    pub fn new<P: AsRef<Path>>(path: P, socket_type: UnixSocketType) -> Self {
        Self {
            max_message_size: 1024 * 1024, // 1MB
        }
    }

    pub fn stream<P: AsRef<Path>>(path: P) -> Self {
        Self::new(path, UnixSocketType::Stream)
    pub fn datagram<P: AsRef<Path>>(path: P) -> Self {
        Self::new(path, UnixSocketType::Datagram)
    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.read_timeout = Some(timeout);
        self.write_timeout = Some(timeout);
        self
    pub fn with_nonblocking(mut self) -> Self {
        self.nonblocking = true;
        self
    pub fn with_credentials(mut self) -> Self {
        self.enable_credentials = true;
        self
    pub fn with_abstract_namespace(mut self) -> Self {
        self.enable_abstract_namespace = true;
        self
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
/// Unix domain socket transport
#[derive(Debug)]
pub struct UnixSocketTransport {
/// Unix socket connection wrapper
#[derive(Debug)]
pub struct UnixSocketConnection {
/// Unix socket listener wrapper
#[derive(Debug)]
pub struct UnixSocketListener {
/// Unix socket datagram implementation
#[derive(Debug)]
pub struct UnixSocketDatagram {
/// Transport statistics
#[derive(Debug, Clone)]
pub struct TransportStatistics {
impl TransportStatistics {
    fn new() -> Self {
        Self {
        }
    }
/// Connection statistics
#[derive(Debug, Clone)]
pub struct ConnectionStatistics {
impl ConnectionStatistics {
    fn new() -> Self {
        Self {
        }
    }

    fn record_send(&mut self, bytes: usize, latency: Duration) {
        self.bytes_sent += bytes as u64;
        self.messages_sent += 1;
        self.last_activity = SystemTime::now();
        self.latency_sum += latency;
        self.latency_count += 1;
    fn record_receive(&mut self, bytes: usize, latency: Duration) {
        self.bytes_received += bytes as u64;
        self.messages_received += 1;
        self.last_activity = SystemTime::now();
        self.latency_sum += latency;
        self.latency_count += 1;
    fn record_error(&mut self) {
        self.errors += 1;
        self.last_activity = SystemTime::now();
    fn average_latency(&self) -> Duration {
        if self.latency_count > 0 {
            self.latency_sum / self.latency_count as u32
        } else {
            Duration::from_micros(0)
        }
    }
/// Listener statistics
#[derive(Debug, Clone)]
pub struct ListenerStatistics {
impl ListenerStatistics {
    fn new() -> Self {
        Self {
        }
    }
/// Datagram statistics
#[derive(Debug, Clone)]
pub struct DatagramStatistics {
impl DatagramStatistics {
    fn new() -> Self {
        Self {
        }
    }
/// Performance metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
impl UnixSocketTransport {
    /// Create a new Unix socket transport
    #[instrument]
    pub fn new(config: UnixSocketConfig) -> IpcResult<Self> {
        config.validate()?;

        #[cfg(not(unix))]
        {
            return Err(communication_error_detailed(
                "Unix domain sockets not supported on this platform"
            ));
        let transport = Self {

        info!(
            "Created Unix socket transport"
        );

        Ok(transport)
    /// Get transport statistics
    pub fn get_statistics(&self) -> TransportStatistics {
        self.statistics.lock()
            .map(|stats| stats.clone())
            .unwrap_or_else(|_| TransportStatistics::new())
    /// Clean up socket file if it exists
    fn cleanup_socket_file(path: &Path) -> IpcResult<()> {
        if path.exists() {
            fs::remove_file(path)
                .map_err(|e| system_error(
                    &format!("Failed to remove socket file: {}", path.display())
                ))?;
            debug!("Removed existing socket file: {}", path.display());
        }
        Ok(())
    /// Set socket permissions
    fn set_socket_permissions(path: &Path, permissions: u32) -> IpcResult<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(path)
                .map_err(|e| system_error(
                    "Failed to get socket file metadata"
                ))?
                .permissions();
            perms.set_mode(permissions);
            fs::set_permissions(path, perms)
                .map_err(|e| system_error(
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
                    if let Some(timeout) = self.config.write_timeout {
                        stream.set_write_timeout(Some(timeout))
                            .map_err(|e| communication_error_detailed(
                                "unix_socket", "connect", &format!("Failed to set write timeout: {}", e)
                            ))?;
                    if self.config.nonblocking {
                        stream.set_nonblocking(true)
                            .map_err(|e| communication_error_detailed(
                                "unix_socket", "connect", &format!("Failed to set nonblocking: {}", e)
                            ))?;
                    // Update statistics
                    if let Ok(mut stats) = self.statistics.lock() {
                        stats.connections_created += 1;
                        stats.active_connections += 1;
                        if stats.active_connections > stats.peak_connections {
                            stats.peak_connections = stats.active_connections;
                        }
                    }

                    let connection = UnixSocketConnection {

                    debug!(
                        "Successfully connected to Unix socket"
                    );

                    Ok(connection)
                }
                Err(e) => {
                    // Update failure statistics
                    if let Ok(mut stats) = self.statistics.lock() {
                        stats.connections_failed += 1;
                        stats.errors += 1;
                    error!(
                        "Failed to connect to Unix socket"
                    );

                    Err(connection_failed(address, &e.to_string()))
                }
            }
        #[cfg(not(unix))]
        {
            Err(communication_error_detailed(
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
                    let socket_listener = UnixSocketListener {

                    info!(
                        "Successfully bound Unix socket listener"
                    );

                    Ok(socket_listener)
                }
                Err(e) => {
                    error!(
                        "Failed to bind Unix socket listener"
                    );

                    Err(communication_error_detailed(
                        &format!("Failed to bind to {}: {}", address, e)
                    ))
                }
            }
        #[cfg(not(unix))]
        {
            Err(communication_error_detailed(
                "Unix domain sockets not supported on this platform"
            ))
        }
    }

    fn is_available() -> bool {
        cfg!(unix)
    fn name(&self) -> &'static str {
        "unix_socket"
    }
}

impl StreamTransport for UnixSocketTransport {
    fn max_message_size(&self) -> usize {
        self.config.max_message_size
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
                // Update last activity
                if let Ok(mut last_activity) = self.last_activity.lock() {
                    *last_activity = Instant::now();
                debug!(
                    "Read data from Unix socket"
                );

                Ok(bytes_read)
            }
            Err(e) => {
                // Update error statistics
                if let Ok(mut stats) = self.statistics.lock() {
                    stats.record_error();
                error!(
                    "Failed to read from Unix socket"
                );

                Err(communication_error_detailed(
                    &e.to_string()
                ))
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
                // Update last activity
                if let Ok(mut last_activity) = self.last_activity.lock() {
                    *last_activity = Instant::now();
                debug!(
                    "Wrote data to Unix socket"
                );

                Ok(bytes_written)
            }
            Err(e) => {
                // Update error statistics
                if let Ok(mut stats) = self.statistics.lock() {
                    stats.record_error();
                error!(
                    "Failed to write to Unix socket"
                );

                Err(communication_error_detailed(
                    &e.to_string()
                ))
            }
        }
    fn flush(&mut self) -> IpcResult<()> {
        self.stream.flush()
            .map_err(|e| communication_error_detailed(
                &e.to_string()
            ))
    fn close(&mut self) -> IpcResult<()> {
        debug!(remote_addr = %self.remote_addr, "Closing Unix socket connection");
        
        // The UnixStream will be closed when dropped
        // Update last activity
        if let Ok(mut last_activity) = self.last_activity.lock() {
            *last_activity = Instant::now();
        Ok(())
    fn is_active(&self) -> bool {
        // For Unix sockets, we can check if the peer is still connected
        // by attempting a zero-byte write (this is a common technique)
        match self.stream.write(&[]) {
            Err(e) => match e.kind() {
                _ => true, // Assume active for other errors
            }
        }
    fn remote_address(&self) -> Option<String> {
        Some(self.remote_addr.clone())
    fn set_read_timeout(&mut self, timeout: Option<Duration>) -> IpcResult<()> {
        self.stream.set_read_timeout(timeout)
            .map_err(|e| communication_error_detailed(
                &e.to_string()
            ))
    fn set_write_timeout(&mut self, timeout: Option<Duration>) -> IpcResult<()> {
        self.stream.set_write_timeout(timeout)
            .map_err(|e| communication_error_detailed(
                &e.to_string()
            ))
    fn try_clone(&self) -> IpcResult<Box<dyn TransportConnection>> {
        match self.stream.try_clone() {
            Ok(cloned_stream) => {
                let cloned_connection = UnixSocketConnection {
                Ok(Box::new(cloned_connection))
            }
            Err(e) => Err(communication_error_detailed(
                &e.to_string()
            ))
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
                if let Some(timeout) = self.config.write_timeout {
                    let _ = stream.set_write_timeout(Some(timeout));
                if self.config.nonblocking {
                    let _ = stream.set_nonblocking(true);
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

                debug!(
                    "Accepted new Unix socket connection"
                );

                Ok(connection)
            }
            Err(e) => {
                // Update failure statistics
                if let Ok(mut stats) = self.statistics.lock() {
                    stats.connection_failures += 1;
                error!(
                    "Failed to accept Unix socket connection"
                );

                Err(communication_error_detailed(
                    &e.to_string()
                ))
            }
        }
    fn set_nonblocking(&mut self, nonblocking: bool) -> IpcResult<()> {
        self.listener.set_nonblocking(nonblocking)
            .map_err(|e| communication_error_detailed(
                &e.to_string()
            ))
    fn local_address(&self) -> Option<String> {
        Some(self.local_addr.clone())
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
impl ConnectionPool {
    /// Create a new connection pool
    pub fn new(transport: Arc<UnixSocketTransport>, config: PoolConfig) -> IpcResult<Self> {
        let pool = TransportPool::new(transport, config)?;
        Ok(Self { pool })
    /// Get a connection from the pool
    pub fn get_connection(&self, address: &str) -> IpcResult<super::pool::PooledConnection<UnixSocketConnection>> {
        self.pool.get_connection(address)
    /// Get pool statistics
    pub fn get_statistics(&self) -> PoolStatistics {
        self.pool.get_statistics()
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
/// Module-level functions for global management

/// Initialize Unix transport subsystem
pub fn initialize_unix_transport() -> IpcResult<()> {
    info!("Initializing Unix domain socket transport subsystem");
    Ok(())
/// Cleanup Unix transport subsystem
pub fn cleanup_unix_transport() -> IpcResult<()> {
    info!("Cleaning up Unix domain socket transport subsystem");
    
    // Clear global statistics
    GLOBAL_TRANSPORT_STATS.lock().unwrap().clear();
    ACTIVE_CONNECTIONS.store(0, Ordering::Relaxed);
    TOTAL_BYTES_TRANSFERRED.store(0, Ordering::Relaxed);
    ERROR_COUNT.store(0, Ordering::Relaxed);
    
    Ok(())
/// Get active connection count
pub fn get_active_connection_count() -> usize {
    ACTIVE_CONNECTIONS.load(Ordering::Relaxed)
/// Get total bytes transferred
pub fn get_total_bytes_transferred() -> u64 {
    TOTAL_BYTES_TRANSFERRED.load(Ordering::Relaxed)
/// Get error count
pub fn get_error_count() -> u64 {
    ERROR_COUNT.load(Ordering::Relaxed)
/// Get performance metrics
pub fn get_performance_metrics() -> PerformanceMetrics {
    PerformanceMetrics {
    }
}

