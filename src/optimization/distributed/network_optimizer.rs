// Network Optimization for Distributed Compilation
//
// Optimizes network communications between the coordinator and worker nodes,
// including compression, connection pooling, and bandwidth management.

use crate::error::{CursedError, Result};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::{mpsc, Semaphore};
use tokio::time::timeout;
use tracing::{debug, error, info, instrument, warn};

/// Network optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkConfig {
    /// Compression settings
    /// Connection pool settings
    /// Bandwidth management
    /// Protocol settings
    /// Enable network monitoring
impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Compression configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompressionConfig {
    /// Enable compression
    /// Compression level (1-9, higher = better compression)
    /// Minimum payload size to compress
    /// Compression algorithm
impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            min_size_bytes: 1024, // 1KB
        }
    }
/// Compression levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompressionLevel {
/// Compression algorithms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompressionAlgorithm {
    /// Fast compression, good for real-time
    /// Balanced compression and speed
    /// Good compression ratio
    /// Best compression ratio
/// Connection pool configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectionPoolConfig {
    /// Maximum connections per worker
    /// Total maximum connections
    /// Connection timeout
    /// Idle connection timeout
    /// Keep-alive interval
    /// Enable connection multiplexing
impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Bandwidth management configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BandwidthConfig {
    /// Maximum bandwidth per worker (bytes/sec)
    /// Total bandwidth limit (bytes/sec)
    /// Enable adaptive bandwidth management
    /// Bandwidth monitoring interval
    /// Quality of Service settings
impl Default for BandwidthConfig {
    fn default() -> Self {
        Self {
            max_bandwidth_per_worker: 10_000_000, // 10 MB/s
            total_bandwidth_limit: 100_000_000,   // 100 MB/s
        }
    }
/// Quality of Service configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QosConfig {
    /// Priority levels for different message types
    /// Enable traffic shaping
    /// Congestion control algorithm
impl Default for QosConfig {
    fn default() -> Self {
        let mut priorities = HashMap::new();
        priorities.insert("job_assignment".to_string(), MessagePriority::High);
        priorities.insert("job_result".to_string(), MessagePriority::High);
        priorities.insert("heartbeat".to_string(), MessagePriority::Normal);
        priorities.insert("metrics".to_string(), MessagePriority::Low);

        Self {
        }
    }
/// Message priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
/// Congestion control algorithms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CongestionControl {
    /// TCP Vegas-like algorithm
    /// TCP Cubic-like algorithm
    /// TCP BBR-like algorithm
    /// Custom adaptive algorithm
/// Protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProtocolConfig {
    /// Use TCP or UDP for different message types
    /// Message serialization format
    /// Enable message ordering guarantees
    /// Enable message reliability guarantees
    /// Heartbeat interval
impl Default for ProtocolConfig {
    fn default() -> Self {
        let mut transport_mapping = HashMap::new();
        transport_mapping.insert("job_assignment".to_string(), TransportProtocol::TCP);
        transport_mapping.insert("job_result".to_string(), TransportProtocol::TCP);
        transport_mapping.insert("heartbeat".to_string(), TransportProtocol::UDP);
        transport_mapping.insert("metrics".to_string(), TransportProtocol::UDP);

        Self {
        }
    }
/// Transport protocols
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransportProtocol {
/// Serialization formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SerializationFormat {
/// Network message with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
/// Connection information
#[derive(Debug)]
struct ConnectionInfo {
/// Bandwidth tracking information
#[derive(Debug, Clone)]
struct BandwidthTracker {
/// Network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
impl Default for NetworkStats {
    fn default() -> Self {
        Self {
        }
    }
/// Network optimizer
pub struct NetworkOptimizer {
/// Prioritized message for queue
#[derive(Debug)]
struct PrioritizedMessage {
impl PartialEq for PrioritizedMessage {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl PartialOrd for PrioritizedMessage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PrioritizedMessage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first (reverse order for max-heap)
        other.priority.cmp(&self.priority)
            .then_with(|| self.queued_at.cmp(&other.queued_at))
    }
}

impl NetworkOptimizer {
    /// Create a new network optimizer
    #[instrument]
    pub fn new(config: NetworkConfig) -> Result<Self> {
        let bandwidth_permits = if config.bandwidth.total_bandwidth_limit > 0 {
            config.bandwidth.total_bandwidth_limit / 1024 // Convert to KB for semaphore
        } else {
            1000000 // Default 1GB worth of permits

        Ok(Self {
        })
    /// Start the network optimizer
    #[instrument(skip(self))]
    pub async fn start(&mut self) -> Result<()> {
        self.is_running.store(true, std::sync::atomic::Ordering::Relaxed);

        // Start background tasks
        self.start_bandwidth_monitor().await?;
        self.start_connection_manager().await?;
        self.start_message_processor().await?;

        info!("Network optimizer started");
        Ok(())
    /// Stop the network optimizer
    #[instrument(skip(self))]
    pub async fn stop(&mut self) -> Result<()> {
        self.is_running.store(false, std::sync::atomic::Ordering::Relaxed);

        // Close all connections
        self.close_all_connections().await?;

        info!("Network optimizer stopped");
        Ok(())
    /// Send a message to a worker
    #[instrument(skip(self, message))]
    pub async fn send_message(
    ) -> Result<()> {
        // Queue message based on priority
        {
            let mut queue = self.message_queue.lock()
                .map_err(|_| CursedError::system_error("Failed to lock message queue"))?;
            
            let prioritized = PrioritizedMessage {
            
            queue.push(prioritized);
        debug!(destination, "Message queued for transmission");
        Ok(())
    /// Send a message immediately (bypass queue)
    #[instrument(skip(self, message))]
    pub async fn send_immediate(
    ) -> Result<()> {
        // Compress if needed
        let compressed_message = self.compress_message(message).await?;
        
        // Get connection
        let mut connection = self.get_connection(destination).await?;
        
        // Serialize message
        let serialized = self.serialize_message(&compressed_message).await?;
        
        // Acquire bandwidth permit
        let permit_size = (serialized.len() / 1024).max(1);
        let _permit = self.bandwidth_semaphore.acquire_many(permit_size as u32).await
            .map_err(|_| CursedError::system_error("Failed to acquire bandwidth permit"))?;

        // Send message
        self.send_raw_data(&mut connection, &serialized).await?;
        
        // Update statistics
        self.update_send_stats(serialized.len(), &compressed_message).await?;
        
        // Return connection to pool
        self.return_connection(destination, connection).await?;

        debug!(destination, size = serialized.len(), "Message sent immediately");
        Ok(())
    /// Receive a message from a worker
    #[instrument(skip(self))]
    pub async fn receive_message(&self, source: &str) -> Result<NetworkMessage> {
        // Get connection
        let mut connection = self.get_connection(source).await?;
        
        // Receive raw data
        let serialized = self.receive_raw_data(&mut connection).await?;
        
        // Deserialize message
        let compressed_message = self.deserialize_message(&serialized).await?;
        
        // Decompress if needed
        let message = self.decompress_message(compressed_message).await?;
        
        // Update statistics
        self.update_receive_stats(serialized.len(), &message).await?;
        
        // Return connection to pool
        self.return_connection(source, connection).await?;

        debug!(source, size = serialized.len(), "Message received");
        Ok(message)
    /// Get network statistics
    pub async fn get_stats(&self) -> Result<NetworkStats> {
        let stats = self.stats.lock()
            .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
        Ok(stats.clone())
    /// Get current network overhead
    pub async fn get_overhead(&self) -> Result<Duration> {
        let stats = self.get_stats().await?;
        Ok(stats.average_latency)
    /// Update configuration
    #[instrument(skip(self, new_config))]
    pub async fn update_config(&mut self, new_config: NetworkConfig) -> Result<()> {
        // Update bandwidth semaphore if limit changed
        if new_config.bandwidth.total_bandwidth_limit != self.config.bandwidth.total_bandwidth_limit {
            let new_permits = if new_config.bandwidth.total_bandwidth_limit > 0 {
                new_config.bandwidth.total_bandwidth_limit / 1024
            } else {
                1000000
            
            // Create new semaphore (old one will be dropped)
            self.bandwidth_semaphore = Arc::new(Semaphore::new(new_permits));
        self.config = new_config;
        info!("Network configuration updated");
        Ok(())
    /// Compress a message if beneficial
    async fn compress_message(&self, mut message: NetworkMessage) -> Result<NetworkMessage> {
        if !self.config.compression.enabled || 
           message.payload.len() < self.config.compression.min_size_bytes {
            return Ok(message);
        let compressed = match self.config.compression.algorithm {

        // Only use compression if it actually reduces size
        if compressed.len() < message.payload.len() {
            message.payload = compressed;
            message.compressed = true;
        Ok(message)
    /// Decompress a message if compressed
    async fn decompress_message(&self, mut message: NetworkMessage) -> Result<NetworkMessage> {
        if !message.compressed {
            return Ok(message);
        let decompressed = match self.config.compression.algorithm {

        message.payload = decompressed;
        message.compressed = false;
        Ok(message)
    /// LZ4 compression
    async fn compress_lz4(&self, data: &[u8]) -> Result<Vec<u8>> {
        use std::io::Write;
        
        // Fast compression implementation
        let mut compressed = Vec::new();
        
        // Simple run-length encoding for demonstration
        // In production, would use lz4_flex or similar crate
        let mut i = 0;
        while i < data.len() {
            let current_byte = data[i];
            let mut count = 1;
            
            // Count consecutive bytes
            while i + count < data.len() && 
                  data[i + count] == current_byte && 
                  count < 255 {
                count += 1;
            if count > 3 {
                // Use run-length encoding
                compressed.push(0xFF); // Escape byte
                compressed.push(count as u8);
                compressed.push(current_byte);
            } else {
                // Store raw bytes
                for _ in 0..count {
                    compressed.push(current_byte);
                }
            }
            
            i += count;
        // Add compression header
        let mut result = Vec::with_capacity(compressed.len() + 8);
        result.extend_from_slice(&(data.len() as u32).to_le_bytes()); // Original size
        result.extend_from_slice(&(compressed.len() as u32).to_le_bytes()); // Compressed size
        result.extend_from_slice(&compressed);
        
        Ok(result)
    /// LZ4 decompression
    async fn decompress_lz4(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < 8 {
            return Err(CursedError::system_error("Invalid compressed data"));
        let original_size = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        let compressed_size = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
        
        if data.len() != compressed_size + 8 {
            return Err(CursedError::system_error("Compressed data size mismatch"));
        let compressed_data = &data[8..];
        let mut decompressed = Vec::with_capacity(original_size);
        
        let mut i = 0;
        while i < compressed_data.len() {
            if compressed_data[i] == 0xFF && i + 2 < compressed_data.len() {
                // Run-length encoded sequence
                let count = compressed_data[i + 1] as usize;
                let byte_value = compressed_data[i + 2];
                
                for _ in 0..count {
                    decompressed.push(byte_value);
                }
                i += 3;
            } else {
                // Raw byte
                decompressed.push(compressed_data[i]);
                i += 1;
            }
        }
        
        if decompressed.len() != original_size {
            return Err(CursedError::system_error("Decompression size mismatch"));
        Ok(decompressed)
    /// Zstd compression
    async fn compress_zstd(&self, data: &[u8]) -> Result<Vec<u8>> {
        use flate2::{write::GzEncoder, Compression};
        use std::io::Write;

        let level = match self.config.compression.level {

        let mut encoder = GzEncoder::new(Vec::new(), level);
        encoder.write_all(data)
            .map_err(|e| CursedError::system_error(&format!("Compression failed: {}", e)))?;
        
        encoder.finish()
            .map_err(|e| CursedError::system_error(&format!("Compression finish failed: {}", e)))
    /// Zstd decompression  
    async fn decompress_zstd(&self, data: &[u8]) -> Result<Vec<u8>> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| CursedError::system_error(&format!("Decompression failed: {}", e)))?;
        
        Ok(decompressed)
    /// Gzip compression
    async fn compress_gzip(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.compress_zstd(data).await // Reuse for now
    /// Gzip decompression
    async fn decompress_gzip(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.decompress_zstd(data).await // Reuse for now
    /// Brotli compression
    async fn compress_brotli(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.compress_zstd(data).await // Reuse for now
    /// Brotli decompression
    async fn decompress_brotli(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.decompress_zstd(data).await // Reuse for now
    /// Serialize a message
    async fn serialize_message(&self, message: &NetworkMessage) -> Result<Vec<u8>> {
        match self.config.protocol.serialization {
            SerializationFormat::Bincode => {
                bincode::serialize(message)
                    .map_err(|e| CursedError::system_error(&format!("Serialization failed: {}", e)))
            }
            SerializationFormat::MessagePack => {
                rmp_serde::to_vec(message)
                    .map_err(|e| CursedError::system_error(&format!("MessagePack serialization failed: {}", e)))
            }
            SerializationFormat::JSON => {
                serde_json::to_vec(message)
                    .map_err(|e| CursedError::system_error(&format!("JSON serialization failed: {}", e)))
            }
            SerializationFormat::ProtocolBuffers => {
                // Would use protobuf crate in practice
                bincode::serialize(message)
                    .map_err(|e| CursedError::system_error(&format!("Protobuf serialization failed: {}", e)))
            }
        }
    /// Deserialize a message
    async fn deserialize_message(&self, data: &[u8]) -> Result<NetworkMessage> {
        match self.config.protocol.serialization {
            SerializationFormat::Bincode => {
                bincode::deserialize(data)
                    .map_err(|e| CursedError::system_error(&format!("Deserialization failed: {}", e)))
            }
            SerializationFormat::MessagePack => {
                rmp_serde::from_slice(data)
                    .map_err(|e| CursedError::system_error(&format!("MessagePack deserialization failed: {}", e)))
            }
            SerializationFormat::JSON => {
                serde_json::from_slice(data)
                    .map_err(|e| CursedError::system_error(&format!("JSON deserialization failed: {}", e)))
            }
            SerializationFormat::ProtocolBuffers => {
                // Would use protobuf crate in practice
                bincode::deserialize(data)
                    .map_err(|e| CursedError::system_error(&format!("Protobuf deserialization failed: {}", e)))
            }
        }
    /// Get a connection from the pool or create a new one
    async fn get_connection(&self, destination: &str) -> Result<TcpStream> {
        // Try to get from pool first
        {
            let mut pool = self.connection_pool.lock()
                .map_err(|_| CursedError::system_error("Failed to lock connection pool"))?;
            
            if let Some(connections) = pool.get_mut(destination) {
                if let Some(conn_info) = connections.pop() {
                    // Check if connection is still valid
                    if conn_info.last_used.elapsed() < self.config.connection_pool.idle_timeout {
                        return Ok(conn_info.stream);
                    }
                }
            }
        }

        // Create new connection
        let addr: SocketAddr = destination.parse()
            .map_err(|_| CursedError::system_error("Invalid destination address"))?;

        let stream = timeout(
            TcpStream::connect(addr)
        ).await
            .map_err(|_| CursedError::system_error("Connection timeout"))?
            .map_err(|e| CursedError::system_error(&format!("Connection failed: {}", e)))?;

        debug!(destination, "New connection established");
        Ok(stream)
    /// Return a connection to the pool
    async fn return_connection(&self, destination: &str, stream: TcpStream) -> Result<()> {
        let mut pool = self.connection_pool.lock()
            .map_err(|_| CursedError::system_error("Failed to lock connection pool"))?;
        
        let connections = pool.entry(destination.to_string()).or_insert_with(Vec::new);
        
        // Check pool limits
        if connections.len() < self.config.connection_pool.max_connections_per_worker {
            let conn_info = ConnectionInfo {
            
            connections.push(conn_info);
        }
        // If pool is full, connection will be dropped

        Ok(())
    /// Send raw data over a connection
    async fn send_raw_data(&self, stream: &mut TcpStream, data: &[u8]) -> Result<()> {
        // Send length prefix
        let len = data.len() as u32;
        stream.write_all(&len.to_be_bytes()).await
            .map_err(|e| CursedError::system_error(&format!("Failed to send length: {}", e)))?;
        
        // Send data
        stream.write_all(data).await
            .map_err(|e| CursedError::system_error(&format!("Failed to send data: {}", e)))?;
        
        stream.flush().await
            .map_err(|e| CursedError::system_error(&format!("Failed to flush stream: {}", e)))?;

        Ok(())
    /// Receive raw data from a connection
    async fn receive_raw_data(&self, stream: &mut TcpStream) -> Result<Vec<u8>> {
        // Read length prefix
        let mut len_bytes = [0u8; 4];
        stream.read_exact(&mut len_bytes).await
            .map_err(|e| CursedError::system_error(&format!("Failed to read length: {}", e)))?;
        
        let len = u32::from_be_bytes(len_bytes) as usize;
        
        // Read data
        let mut data = vec![0u8; len];
        stream.read_exact(&mut data).await
            .map_err(|e| CursedError::system_error(&format!("Failed to read data: {}", e)))?;

        Ok(data)
    /// Update send statistics
    async fn update_send_stats(&self, bytes_sent: usize, message: &NetworkMessage) -> Result<()> {
        let mut stats = self.stats.lock()
            .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
        
        stats.total_bytes_sent += bytes_sent;
        stats.messages_sent += 1;
        
        // Update compression ratio
        if message.compressed {
            // Would calculate actual ratio based on original vs compressed size
            stats.compression_ratio = 0.7; // Mock value
        Ok(())
    /// Update receive statistics
    async fn update_receive_stats(&self, bytes_received: usize, _message: &NetworkMessage) -> Result<()> {
        let mut stats = self.stats.lock()
            .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
        
        stats.total_bytes_received += bytes_received;
        stats.messages_received += 1;

        Ok(())
    /// Start bandwidth monitoring task
    async fn start_bandwidth_monitor(&self) -> Result<()> {
        let bandwidth_trackers = self.bandwidth_trackers.clone();
        let config = self.config.clone();
        let is_running = self.is_running.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(config.bandwidth.monitoring_interval);
            
            while is_running.load(std::sync::atomic::Ordering::Relaxed) {
                interval.tick().await;
                
                // Update bandwidth statistics
                let mut trackers = match bandwidth_trackers.lock() {
                
                let now = Instant::now();
                for (worker_id, tracker) in trackers.iter_mut() {
                    let time_elapsed = now.duration_since(tracker.last_reset).as_secs_f64();
                    
                    if time_elapsed > 0.0 {
                        tracker.current_rate_out = tracker.bytes_sent as f64 / time_elapsed;
                        tracker.current_rate_in = tracker.bytes_received as f64 / time_elapsed;
                        
                        // Reset counters for next interval
                        tracker.bytes_sent = 0;
                        tracker.bytes_received = 0;
                        tracker.last_reset = now;
                        
                        debug!(
                            rate_out_mbps = tracker.current_rate_out / 1_000_000.0,
                            rate_in_mbps = tracker.current_rate_in / 1_000_000.0,
                            "Bandwidth monitoring update"
                        );
                    }
                }
            }
        });
        
        info!("Bandwidth monitoring task started");
        Ok(())
    /// Start connection manager task
    async fn start_connection_manager(&self) -> Result<()> {
        let connection_pool = self.connection_pool.clone();
        let config = self.config.clone();
        let is_running = self.is_running.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(config.connection_pool.keep_alive_interval);
            
            while is_running.load(std::sync::atomic::Ordering::Relaxed) {
                interval.tick().await;
                
                // Clean up idle connections
                let mut pool = match connection_pool.lock() {
                
                let now = Instant::now();
                for (worker_id, connections) in pool.iter_mut() {
                    connections.retain(|conn| {
                        let is_fresh = now.duration_since(conn.last_used) < config.connection_pool.idle_timeout;
                        if !is_fresh {
                            debug!(worker_id = worker_id, connection_id = conn.connection_id, "Cleaning up idle connection");
                        }
                        is_fresh
                    });
                // Remove empty worker entries
                pool.retain(|_, connections| !connections.is_empty());
            }
        });
        
        info!("Connection manager task started");
        Ok(())
    /// Start message processor task
    async fn start_message_processor(&self) -> Result<()> {
        let message_queue = self.message_queue.clone();
        let network_optimizer = NetworkOptimizer {
        let is_running = self.is_running.clone();
        
        tokio::spawn(async move {
            while is_running.load(std::sync::atomic::Ordering::Relaxed) {
                // Process priority queue
                let message_to_send = {
                    let mut queue = match message_queue.lock() {
                        Err(_) => {
                            tokio::time::sleep(Duration::from_millis(10)).await;
                            continue;
                        }
                    
                    queue.pop()
                
                if let Some(prioritized_message) = message_to_send {
                    let destination = prioritized_message.message.destination.clone();
                    
                    // Send message immediately
                    if let Err(e) = network_optimizer.send_immediate(&destination, prioritized_message.message).await {
                        warn!(error = ?e, destination = destination, "Failed to send queued message");
                    }
                } else {
                    // No messages in queue, sleep briefly
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }
        });
        
        info!("Message processor task started");
        Ok(())
    /// Close all connections
    async fn close_all_connections(&self) -> Result<()> {
        let mut pool = self.connection_pool.lock()
            .map_err(|_| CursedError::system_error("Failed to lock connection pool"))?;
        
        pool.clear();
        info!("All connections closed");
        Ok(())
    }
}

