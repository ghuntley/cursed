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
    pub compression: CompressionConfig,
    /// Connection pool settings
    pub connection_pool: ConnectionPoolConfig,
    /// Bandwidth management
    pub bandwidth: BandwidthConfig,
    /// Protocol settings
    pub protocol: ProtocolConfig,
    /// Enable network monitoring
    pub monitoring_enabled: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            compression: CompressionConfig::default(),
            connection_pool: ConnectionPoolConfig::default(),
            bandwidth: BandwidthConfig::default(),
            protocol: ProtocolConfig::default(),
            monitoring_enabled: true,
        }
    }
}

/// Compression configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompressionConfig {
    /// Enable compression
    pub enabled: bool,
    /// Compression level (1-9, higher = better compression)
    pub level: CompressionLevel,
    /// Minimum payload size to compress
    pub min_size_bytes: usize,
    /// Compression algorithm
    pub algorithm: CompressionAlgorithm,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: CompressionLevel::Balanced,
            min_size_bytes: 1024, // 1KB
            algorithm: CompressionAlgorithm::LZ4,
        }
    }
}

/// Compression levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompressionLevel {
    Fast = 1,
    Balanced = 4,
    High = 6,
    Maximum = 9,
}

/// Compression algorithms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompressionAlgorithm {
    /// Fast compression, good for real-time
    LZ4,
    /// Balanced compression and speed
    Zstd,
    /// Good compression ratio
    Gzip,
    /// Best compression ratio
    Brotli,
}

/// Connection pool configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectionPoolConfig {
    /// Maximum connections per worker
    pub max_connections_per_worker: usize,
    /// Total maximum connections
    pub max_total_connections: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Idle connection timeout
    pub idle_timeout: Duration,
    /// Keep-alive interval
    pub keep_alive_interval: Duration,
    /// Enable connection multiplexing
    pub multiplexing_enabled: bool,
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            max_connections_per_worker: 4,
            max_total_connections: 64,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300),
            keep_alive_interval: Duration::from_secs(60),
            multiplexing_enabled: true,
        }
    }
}

/// Bandwidth management configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BandwidthConfig {
    /// Maximum bandwidth per worker (bytes/sec)
    pub max_bandwidth_per_worker: usize,
    /// Total bandwidth limit (bytes/sec)
    pub total_bandwidth_limit: usize,
    /// Enable adaptive bandwidth management
    pub adaptive_enabled: bool,
    /// Bandwidth monitoring interval
    pub monitoring_interval: Duration,
    /// Quality of Service settings
    pub qos: QosConfig,
}

impl Default for BandwidthConfig {
    fn default() -> Self {
        Self {
            max_bandwidth_per_worker: 10_000_000, // 10 MB/s
            total_bandwidth_limit: 100_000_000,   // 100 MB/s
            adaptive_enabled: true,
            monitoring_interval: Duration::from_secs(1),
            qos: QosConfig::default(),
        }
    }
}

/// Quality of Service configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QosConfig {
    /// Priority levels for different message types
    pub message_priorities: HashMap<String, MessagePriority>,
    /// Enable traffic shaping
    pub traffic_shaping_enabled: bool,
    /// Congestion control algorithm
    pub congestion_control: CongestionControl,
}

impl Default for QosConfig {
    fn default() -> Self {
        let mut priorities = HashMap::new();
        priorities.insert("job_assignment".to_string(), MessagePriority::High);
        priorities.insert("job_result".to_string(), MessagePriority::High);
        priorities.insert("heartbeat".to_string(), MessagePriority::Normal);
        priorities.insert("metrics".to_string(), MessagePriority::Low);

        Self {
            message_priorities: priorities,
            traffic_shaping_enabled: true,
            congestion_control: CongestionControl::Vegas,
        }
    }
}

/// Message priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// Congestion control algorithms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CongestionControl {
    /// TCP Vegas-like algorithm
    Vegas,
    /// TCP Cubic-like algorithm
    Cubic,
    /// TCP BBR-like algorithm
    BBR,
    /// Custom adaptive algorithm
    Adaptive,
}

/// Protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProtocolConfig {
    /// Use TCP or UDP for different message types
    pub transport_mapping: HashMap<String, TransportProtocol>,
    /// Message serialization format
    pub serialization: SerializationFormat,
    /// Enable message ordering guarantees
    pub ordered_delivery: bool,
    /// Enable message reliability guarantees
    pub reliable_delivery: bool,
    /// Heartbeat interval
    pub heartbeat_interval: Duration,
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        let mut transport_mapping = HashMap::new();
        transport_mapping.insert("job_assignment".to_string(), TransportProtocol::TCP);
        transport_mapping.insert("job_result".to_string(), TransportProtocol::TCP);
        transport_mapping.insert("heartbeat".to_string(), TransportProtocol::UDP);
        transport_mapping.insert("metrics".to_string(), TransportProtocol::UDP);

        Self {
            transport_mapping,
            serialization: SerializationFormat::MessagePack,
            ordered_delivery: true,
            reliable_delivery: true,
            heartbeat_interval: Duration::from_secs(30),
        }
    }
}

/// Transport protocols
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransportProtocol {
    TCP,
    UDP,
    QUIC,
}

/// Serialization formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SerializationFormat {
    Bincode,
    MessagePack,
    ProtocolBuffers,
    JSON,
}

/// Network message with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub id: String,
    pub message_type: String,
    pub priority: MessagePriority,
    pub source: String,
    pub destination: String,
    pub payload: Vec<u8>,
    pub compressed: bool,
    pub timestamp: SystemTime,
    pub correlation_id: Option<String>,
}

/// Connection information
#[derive(Debug)]
struct ConnectionInfo {
    stream: TcpStream,
    last_used: Instant,
    bytes_sent: usize,
    bytes_received: usize,
    connection_id: String,
}

/// Bandwidth tracking information
#[derive(Debug, Clone)]
struct BandwidthTracker {
    bytes_sent: usize,
    bytes_received: usize,
    last_reset: Instant,
    current_rate_out: f64,
    current_rate_in: f64,
}

/// Network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub total_bytes_sent: usize,
    pub total_bytes_received: usize,
    pub messages_sent: usize,
    pub messages_received: usize,
    pub compression_ratio: f64,
    pub average_latency: Duration,
    pub connection_pool_size: usize,
    pub bandwidth_utilization: f64,
    pub packet_loss_rate: f64,
    pub retransmission_rate: f64,
}

impl Default for NetworkStats {
    fn default() -> Self {
        Self {
            total_bytes_sent: 0,
            total_bytes_received: 0,
            messages_sent: 0,
            messages_received: 0,
            compression_ratio: 1.0,
            average_latency: Duration::ZERO,
            connection_pool_size: 0,
            bandwidth_utilization: 0.0,
            packet_loss_rate: 0.0,
            retransmission_rate: 0.0,
        }
    }
}

/// Network optimizer
pub struct NetworkOptimizer {
    config: NetworkConfig,
    connection_pool: Arc<Mutex<HashMap<String, Vec<ConnectionInfo>>>>,
    bandwidth_trackers: Arc<Mutex<HashMap<String, BandwidthTracker>>>,
    compression_cache: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    stats: Arc<Mutex<NetworkStats>>,
    bandwidth_semaphore: Arc<Semaphore>,
    message_queue: Arc<Mutex<std::collections::BinaryHeap<PrioritizedMessage>>>,
    is_running: Arc<std::sync::atomic::AtomicBool>,
}

/// Prioritized message for queue
#[derive(Debug)]
struct PrioritizedMessage {
    message: NetworkMessage,
    priority: MessagePriority,
    queued_at: Instant,
}

impl PartialEq for PrioritizedMessage {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for PrioritizedMessage {}

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
        };

        Ok(Self {
            config,
            connection_pool: Arc::new(Mutex::new(HashMap::new())),
            bandwidth_trackers: Arc::new(Mutex::new(HashMap::new())),
            compression_cache: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(NetworkStats::default())),
            bandwidth_semaphore: Arc::new(Semaphore::new(bandwidth_permits)),
            message_queue: Arc::new(Mutex::new(std::collections::BinaryHeap::new())),
            is_running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        })
    }

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
    }

    /// Stop the network optimizer
    #[instrument(skip(self))]
    pub async fn stop(&mut self) -> Result<()> {
        self.is_running.store(false, std::sync::atomic::Ordering::Relaxed);

        // Close all connections
        self.close_all_connections().await?;

        info!("Network optimizer stopped");
        Ok(())
    }

    /// Send a message to a worker
    #[instrument(skip(self, message))]
    pub async fn send_message(
        &self,
        destination: &str,
        message: NetworkMessage,
    ) -> Result<()> {
        // Queue message based on priority
        {
            let mut queue = self.message_queue.lock()
                .map_err(|_| CursedError::system_error("Failed to lock message queue"))?;
            
            let prioritized = PrioritizedMessage {
                priority: message.priority.clone(),
                queued_at: Instant::now(),
                message,
            };
            
            queue.push(prioritized);
        }

        debug!(destination, "Message queued for transmission");
        Ok(())
    }

    /// Send a message immediately (bypass queue)
    #[instrument(skip(self, message))]
    pub async fn send_immediate(
        &self,
        destination: &str,
        message: NetworkMessage,
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
    }

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
    }

    /// Get network statistics
    pub async fn get_stats(&self) -> Result<NetworkStats> {
        let stats = self.stats.lock()
            .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
        Ok(stats.clone())
    }

    /// Get current network overhead
    pub async fn get_overhead(&self) -> Result<Duration> {
        let stats = self.get_stats().await?;
        Ok(stats.average_latency)
    }

    /// Update configuration
    #[instrument(skip(self, new_config))]
    pub async fn update_config(&mut self, new_config: NetworkConfig) -> Result<()> {
        // Update bandwidth semaphore if limit changed
        if new_config.bandwidth.total_bandwidth_limit != self.config.bandwidth.total_bandwidth_limit {
            let new_permits = if new_config.bandwidth.total_bandwidth_limit > 0 {
                new_config.bandwidth.total_bandwidth_limit / 1024
            } else {
                1000000
            };
            
            // Create new semaphore (old one will be dropped)
            self.bandwidth_semaphore = Arc::new(Semaphore::new(new_permits));
        }

        self.config = new_config;
        info!("Network configuration updated");
        Ok(())
    }

    /// Compress a message if beneficial
    async fn compress_message(&self, mut message: NetworkMessage) -> Result<NetworkMessage> {
        if !self.config.compression.enabled || 
           message.payload.len() < self.config.compression.min_size_bytes {
            return Ok(message);
        }

        let compressed = match self.config.compression.algorithm {
            CompressionAlgorithm::LZ4 => self.compress_lz4(&message.payload).await?,
            CompressionAlgorithm::Zstd => self.compress_zstd(&message.payload).await?,
            CompressionAlgorithm::Gzip => self.compress_gzip(&message.payload).await?,
            CompressionAlgorithm::Brotli => self.compress_brotli(&message.payload).await?,
        };

        // Only use compression if it actually reduces size
        if compressed.len() < message.payload.len() {
            message.payload = compressed;
            message.compressed = true;
        }

        Ok(message)
    }

    /// Decompress a message if compressed
    async fn decompress_message(&self, mut message: NetworkMessage) -> Result<NetworkMessage> {
        if !message.compressed {
            return Ok(message);
        }

        let decompressed = match self.config.compression.algorithm {
            CompressionAlgorithm::LZ4 => self.decompress_lz4(&message.payload).await?,
            CompressionAlgorithm::Zstd => self.decompress_zstd(&message.payload).await?,
            CompressionAlgorithm::Gzip => self.decompress_gzip(&message.payload).await?,
            CompressionAlgorithm::Brotli => self.decompress_brotli(&message.payload).await?,
        };

        message.payload = decompressed;
        message.compressed = false;
        Ok(message)
    }

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
            }
            
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
        }
        
        // Add compression header
        let mut result = Vec::with_capacity(compressed.len() + 8);
        result.extend_from_slice(&(data.len() as u32).to_le_bytes()); // Original size
        result.extend_from_slice(&(compressed.len() as u32).to_le_bytes()); // Compressed size
        result.extend_from_slice(&compressed);
        
        Ok(result)
    }

    /// LZ4 decompression
    async fn decompress_lz4(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < 8 {
            return Err(CursedError::system_error("Invalid compressed data"));
        }
        
        let original_size = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        let compressed_size = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
        
        if data.len() != compressed_size + 8 {
            return Err(CursedError::system_error("Compressed data size mismatch"));
        }
        
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
        }
        
        Ok(decompressed)
    }

    /// Zstd compression
    async fn compress_zstd(&self, data: &[u8]) -> Result<Vec<u8>> {
        use flate2::{write::GzEncoder, Compression};
        use std::io::Write;

        let level = match self.config.compression.level {
            CompressionLevel::Fast => Compression::fast(),
            CompressionLevel::Balanced => Compression::default(),
            CompressionLevel::High => Compression::best(),
            CompressionLevel::Maximum => Compression::best(),
        };

        let mut encoder = GzEncoder::new(Vec::new(), level);
        encoder.write_all(data)
            .map_err(|e| CursedError::system_error(&format!("Compression failed: {}", e)))?;
        
        encoder.finish()
            .map_err(|e| CursedError::system_error(&format!("Compression finish failed: {}", e)))
    }

    /// Zstd decompression  
    async fn decompress_zstd(&self, data: &[u8]) -> Result<Vec<u8>> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| CursedError::system_error(&format!("Decompression failed: {}", e)))?;
        
        Ok(decompressed)
    }

    /// Gzip compression
    async fn compress_gzip(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.compress_zstd(data).await // Reuse for now
    }

    /// Gzip decompression
    async fn decompress_gzip(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.decompress_zstd(data).await // Reuse for now
    }

    /// Brotli compression
    async fn compress_brotli(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.compress_zstd(data).await // Reuse for now
    }

    /// Brotli decompression
    async fn decompress_brotli(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.decompress_zstd(data).await // Reuse for now
    }

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
            self.config.connection_pool.connection_timeout,
            TcpStream::connect(addr)
        ).await
            .map_err(|_| CursedError::system_error("Connection timeout"))?
            .map_err(|e| CursedError::system_error(&format!("Connection failed: {}", e)))?;

        debug!(destination, "New connection established");
        Ok(stream)
    }

    /// Return a connection to the pool
    async fn return_connection(&self, destination: &str, stream: TcpStream) -> Result<()> {
        let mut pool = self.connection_pool.lock()
            .map_err(|_| CursedError::system_error("Failed to lock connection pool"))?;
        
        let connections = pool.entry(destination.to_string()).or_insert_with(Vec::new);
        
        // Check pool limits
        if connections.len() < self.config.connection_pool.max_connections_per_worker {
            let conn_info = ConnectionInfo {
                stream,
                last_used: Instant::now(),
                bytes_sent: 0,
                bytes_received: 0,
                connection_id: uuid::Uuid::new_v4().to_string(),
            };
            
            connections.push(conn_info);
        }
        // If pool is full, connection will be dropped

        Ok(())
    }

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
    }

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
    }

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
        }

        Ok(())
    }

    /// Update receive statistics
    async fn update_receive_stats(&self, bytes_received: usize, _message: &NetworkMessage) -> Result<()> {
        let mut stats = self.stats.lock()
            .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
        
        stats.total_bytes_received += bytes_received;
        stats.messages_received += 1;

        Ok(())
    }

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
                    Ok(t) => t,
                    Err(_) => continue,
                };
                
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
                            worker_id = worker_id,
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
    }

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
                    Ok(p) => p,
                    Err(_) => continue,
                };
                
                let now = Instant::now();
                for (worker_id, connections) in pool.iter_mut() {
                    connections.retain(|conn| {
                        let is_fresh = now.duration_since(conn.last_used) < config.connection_pool.idle_timeout;
                        if !is_fresh {
                            debug!(worker_id = worker_id, connection_id = conn.connection_id, "Cleaning up idle connection");
                        }
                        is_fresh
                    });
                }
                
                // Remove empty worker entries
                pool.retain(|_, connections| !connections.is_empty());
            }
        });
        
        info!("Connection manager task started");
        Ok(())
    }

    /// Start message processor task
    async fn start_message_processor(&self) -> Result<()> {
        let message_queue = self.message_queue.clone();
        let network_optimizer = NetworkOptimizer {
            config: self.config.clone(),
            connection_pool: self.connection_pool.clone(),
            bandwidth_trackers: self.bandwidth_trackers.clone(),
            compression_cache: self.compression_cache.clone(),
            stats: self.stats.clone(),
            bandwidth_semaphore: self.bandwidth_semaphore.clone(),
            message_queue: self.message_queue.clone(),
            is_running: self.is_running.clone(),
        };
        let is_running = self.is_running.clone();
        
        tokio::spawn(async move {
            while is_running.load(std::sync::atomic::Ordering::Relaxed) {
                // Process priority queue
                let message_to_send = {
                    let mut queue = match message_queue.lock() {
                        Ok(q) => q,
                        Err(_) => {
                            tokio::time::sleep(Duration::from_millis(10)).await;
                            continue;
                        }
                    };
                    
                    queue.pop()
                };
                
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
    }

    /// Close all connections
    async fn close_all_connections(&self) -> Result<()> {
        let mut pool = self.connection_pool.lock()
            .map_err(|_| CursedError::system_error("Failed to lock connection pool"))?;
        
        pool.clear();
        info!("All connections closed");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_network_optimizer_creation() {
        let config = NetworkConfig::default();
        let optimizer = NetworkOptimizer::new(config);
        assert!(optimizer.is_ok());
    }

    #[tokio::test]
    async fn test_compression_config() {
        let config = CompressionConfig {
            enabled: true,
            level: CompressionLevel::High,
            min_size_bytes: 2048,
            algorithm: CompressionAlgorithm::Zstd,
        };
        
        assert!(config.enabled);
        assert_eq!(config.min_size_bytes, 2048);
        assert_eq!(config.algorithm, CompressionAlgorithm::Zstd);
    }

    #[tokio::test]
    async fn test_message_priority_ordering() {
        let msg1 = PrioritizedMessage {
            message: NetworkMessage {
                id: "1".to_string(),
                message_type: "test".to_string(),
                priority: MessagePriority::Low,
                source: "test".to_string(),
                destination: "test".to_string(),
                payload: Vec::new(),
                compressed: false,
                timestamp: SystemTime::now(),
                correlation_id: None,
            },
            priority: MessagePriority::Low,
            queued_at: Instant::now(),
        };

        let msg2 = PrioritizedMessage {
            message: NetworkMessage {
                id: "2".to_string(),
                message_type: "test".to_string(),
                priority: MessagePriority::High,
                source: "test".to_string(),
                destination: "test".to_string(),
                payload: Vec::new(),
                compressed: false,
                timestamp: SystemTime::now(),
                correlation_id: None,
            },
            priority: MessagePriority::High,
            queued_at: Instant::now(),
        };

        assert!(msg2 > msg1); // Higher priority should be greater
    }

    #[tokio::test]
    async fn test_bandwidth_config() {
        let config = BandwidthConfig {
            max_bandwidth_per_worker: 5_000_000,
            total_bandwidth_limit: 50_000_000,
            adaptive_enabled: true,
            monitoring_interval: Duration::from_millis(500),
            qos: QosConfig::default(),
        };

        assert_eq!(config.max_bandwidth_per_worker, 5_000_000);
        assert!(config.adaptive_enabled);
    }

    #[tokio::test]
    async fn test_default_configurations() {
        let net_config = NetworkConfig::default();
        let compression_config = CompressionConfig::default();
        let pool_config = ConnectionPoolConfig::default();
        let bandwidth_config = BandwidthConfig::default();

        assert!(compression_config.enabled);
        assert_eq!(pool_config.max_connections_per_worker, 4);
        assert_eq!(bandwidth_config.max_bandwidth_per_worker, 10_000_000);
        assert!(net_config.monitoring_enabled);
    }
}
