//! Worker Node Management for Distributed Compilation
//!
//! Manages remote worker nodes that perform actual compilation tasks.
//! Handles worker registration, capability detection, health monitoring, and task assignment.

use crate::error::{CursedError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};
use tokio::net::{TcpStream, UdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::timeout;
use tracing::{debug, error, info, instrument, warn};
use uuid::Uuid;

/// A worker node in the distributed compilation system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerNode {
    pub id: String,
    pub address: SocketAddr,
    pub capabilities: WorkerCapabilities,
    pub status: WorkerStatus,
    pub metrics: WorkerMetrics,
    pub last_heartbeat: SystemTime,
    pub registration_time: SystemTime,
    pub version: String,
    pub tags: HashMap<String, String>,
}

/// Capabilities and specifications of a worker node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerCapabilities {
    /// Number of CPU cores available
    pub cpu_cores: usize,
    /// Available memory in MB
    pub memory_mb: usize,
    /// Available disk space in MB
    pub disk_space_mb: usize,
    /// Supported target architectures
    pub supported_targets: Vec<String>,
    /// Available compiler toolchains
    pub toolchains: HashMap<String, String>,
    /// Maximum concurrent jobs
    pub max_concurrent_jobs: usize,
    /// Performance benchmark score
    pub performance_score: f64,
    /// Network bandwidth (Mbps)
    pub network_bandwidth: f64,
    /// Specialized features (GPU compilation, etc.)
    pub features: Vec<String>,
}

/// Current status of a worker node
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkerStatus {
    /// Worker is online and available
    Online,
    /// Worker is online but busy with tasks
    Busy,
    /// Worker is offline or unreachable
    Offline,
    /// Worker is in maintenance mode
    Maintenance,
    /// Worker has encountered an error
    Error { message: String },
    /// Worker is starting up
    Starting,
    /// Worker is shutting down
    Stopping,
}

/// Performance and health metrics for a worker node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerMetrics {
    /// Current CPU utilization (0.0 - 1.0)
    pub cpu_usage: f64,
    /// Current memory utilization (0.0 - 1.0)
    pub memory_usage: f64,
    /// Current disk utilization (0.0 - 1.0)
    pub disk_usage: f64,
    /// Current network utilization (0.0 - 1.0)
    pub network_usage: f64,
    /// Number of active compilation jobs
    pub active_jobs: usize,
    /// Total jobs completed
    pub completed_jobs: usize,
    /// Total jobs failed
    pub failed_jobs: usize,
    /// Average job completion time
    pub average_job_time: Duration,
    /// Total uptime
    pub uptime: Duration,
    /// Last update timestamp
    pub last_updated: SystemTime,
    /// Current load average
    pub load_average: f64,
    /// Temperature metrics (for hardware monitoring)
    pub temperature_celsius: Option<f64>,
}

impl Default for WorkerMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_usage: 0.0,
            network_usage: 0.0,
            active_jobs: 0,
            completed_jobs: 0,
            failed_jobs: 0,
            average_job_time: Duration::ZERO,
            uptime: Duration::ZERO,
            last_updated: SystemTime::now(),
            load_average: 0.0,
            temperature_celsius: None,
        }
    }
}

/// Configuration for worker node management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerConfig {
    /// How often to send heartbeats
    pub heartbeat_interval: Duration,
    /// Timeout for worker responses
    pub response_timeout: Duration,
    /// Number of missed heartbeats before marking offline
    pub max_missed_heartbeats: usize,
    /// Metrics update interval
    pub metrics_interval: Duration,
    /// Enable automatic worker discovery
    pub auto_discovery: bool,
    /// Discovery broadcast interval
    pub discovery_interval: Duration,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval: Duration::from_secs(30),
            response_timeout: Duration::from_secs(10),
            max_missed_heartbeats: 3,
            metrics_interval: Duration::from_secs(5),
            auto_discovery: true,
            discovery_interval: Duration::from_secs(60),
        }
    }
}

/// Messages exchanged between coordinator and workers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkerMessage {
    /// Worker registration request
    RegisterRequest {
        capabilities: WorkerCapabilities,
        version: String,
        tags: HashMap<String, String>,
    },
    /// Registration acknowledgment
    RegisterResponse {
        accepted: bool,
        coordinator_id: String,
        assigned_id: String,
    },
    /// Heartbeat from worker
    Heartbeat {
        worker_id: String,
        metrics: WorkerMetrics,
        status: WorkerStatus,
    },
    /// Heartbeat acknowledgment
    HeartbeatAck {
        coordinator_time: SystemTime,
    },
    /// Job assignment to worker
    JobAssignment {
        job: super::distributed_compiler::CompilationJob,
    },
    /// Job completion notification
    JobCompletion {
        job_id: String,
        result: super::distributed_compiler::CompilationResult,
    },
    /// Job failure notification
    JobFailure {
        job_id: String,
        error: String,
        retry_possible: bool,
    },
    /// Shutdown request
    Shutdown {
        graceful: bool,
        timeout: Duration,
    },
    /// Capability update
    CapabilityUpdate {
        capabilities: WorkerCapabilities,
    },
    /// Status update
    StatusUpdate {
        status: WorkerStatus,
        message: Option<String>,
    },
    /// Discovery broadcast
    DiscoveryBroadcast {
        coordinator_id: String,
        coordinator_address: SocketAddr,
    },
    /// Discovery response
    DiscoveryResponse {
        worker_id: String,
        worker_address: SocketAddr,
        capabilities: WorkerCapabilities,
    },
}

/// Worker node manager
pub struct WorkerNodeManager {
    config: WorkerConfig,
    workers: Arc<Mutex<HashMap<String, WorkerNode>>>,
    coordinator_id: String,
    discovery_socket: Option<UdpSocket>,
    is_running: Arc<std::sync::atomic::AtomicBool>,
}

impl WorkerNode {
    /// Create a new worker node
    pub fn new(
        address: SocketAddr,
        capabilities: WorkerCapabilities,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            address,
            capabilities,
            status: WorkerStatus::Starting,
            metrics: WorkerMetrics::default(),
            last_heartbeat: SystemTime::now(),
            registration_time: SystemTime::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            tags: HashMap::new(),
        }
    }

    /// Check if worker is available for new jobs
    pub fn is_available(&self) -> bool {
        matches!(self.status, WorkerStatus::Online) &&
        self.metrics.active_jobs < self.capabilities.max_concurrent_jobs
    }

    /// Get current load factor (0.0 = no load, 1.0 = fully loaded)
    pub fn load_factor(&self) -> f64 {
        if self.capabilities.max_concurrent_jobs == 0 {
            return 1.0;
        }
        self.metrics.active_jobs as f64 / self.capabilities.max_concurrent_jobs as f64
    }

    /// Calculate worker efficiency score
    pub fn efficiency_score(&self) -> f64 {
        let base_score = self.capabilities.performance_score;
        let load_penalty = self.load_factor() * 0.5; // Reduce score based on load
        let reliability = if self.metrics.completed_jobs + self.metrics.failed_jobs > 0 {
            self.metrics.completed_jobs as f64 / (self.metrics.completed_jobs + self.metrics.failed_jobs) as f64
        } else {
            1.0
        };
        
        base_score * (1.0 - load_penalty) * reliability
    }

    /// Check if worker supports a target architecture
    pub fn supports_target(&self, target: &str) -> bool {
        self.capabilities.supported_targets.contains(&target.to_string())
    }

    /// Check if worker has a specific toolchain
    pub fn has_toolchain(&self, toolchain: &str) -> bool {
        self.capabilities.toolchains.contains_key(toolchain)
    }

    /// Update worker metrics
    pub fn update_metrics(&mut self, new_metrics: WorkerMetrics) {
        self.metrics = new_metrics;
        self.metrics.last_updated = SystemTime::now();
        self.last_heartbeat = SystemTime::now();
    }

    /// Check if worker heartbeat is stale
    pub fn is_heartbeat_stale(&self, max_age: Duration) -> bool {
        self.last_heartbeat
            .elapsed()
            .unwrap_or(Duration::MAX)
            > max_age
    }

    /// Get worker age since registration
    pub fn age(&self) -> Duration {
        self.registration_time
            .elapsed()
            .unwrap_or(Duration::ZERO)
    }
}

impl WorkerCapabilities {
    /// Create capabilities for the current machine
    pub fn detect_local() -> Result<Self> {
        let cpu_cores = num_cpus::get();
        
        // Get memory information
        let memory_mb = Self::get_memory_info()?;
        
        // Get disk space information
        let disk_space_mb = Self::get_disk_space()?;
        
        // Detect supported targets
        let supported_targets = Self::detect_targets();
        
        // Detect available toolchains
        let toolchains = Self::detect_toolchains();
        
        // Calculate performance score
        let performance_score = Self::calculate_performance_score(cpu_cores, memory_mb);
        
        Ok(Self {
            cpu_cores,
            memory_mb,
            disk_space_mb,
            supported_targets,
            toolchains,
            max_concurrent_jobs: cpu_cores,
            performance_score,
            network_bandwidth: 100.0, // Default 100 Mbps
            features: Self::detect_features(),
        })
    }

    /// Get system memory information
    fn get_memory_info() -> Result<usize> {
        // Simplified memory detection
        #[cfg(target_os = "linux")]
        {
            use std::fs;
            if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
                for line in meminfo.split("\n") {
                    if line.starts_with("MemTotal:") {
                        if let Some(kb_str) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = kb_str.parse::<usize>() {
                                return Ok(kb / 1024); // Convert KB to MB
                            }
                        }
                    }
                }
            }
        }
        
        // Default fallback
        Ok(8192) // 8GB default
    }

    /// Get available disk space
    fn get_disk_space() -> Result<usize> {
        use std::fs;
        
        // Try to get disk space for current directory
        if let Ok(metadata) = fs::metadata(".") {
            // This is a simplified implementation
            // In practice, you'd use platform-specific APIs
            return Ok(100 * 1024); // 100GB default
        }
        
        Ok(50 * 1024) // 50GB fallback
    }

    /// Detect supported target architectures
    fn detect_targets() -> Vec<String> {
        vec![
            "x86_64-unknown-linux-gnu".to_string(),
            "x86_64-pc-windows-msvc".to_string(),
            "x86_64-apple-darwin".to_string(),
            "aarch64-unknown-linux-gnu".to_string(),
            "wasm32-unknown-unknown".to_string(),
        ]
    }

    /// Detect available toolchains
    fn detect_toolchains() -> HashMap<String, String> {
        let mut toolchains = HashMap::new();
        
        // Check for common compilers
        if Self::command_exists("rustc") {
            toolchains.insert("rust".to_string(), "1.70.0".to_string());
        }
        if Self::command_exists("clang") {
            toolchains.insert("clang".to_string(), "15.0.0".to_string());
        }
        if Self::command_exists("gcc") {
            toolchains.insert("gcc".to_string(), "11.0.0".to_string());
        }
        
        toolchains
    }

    /// Check if a command exists in PATH
    fn command_exists(command: &str) -> bool {
        std::process::Command::new("which")
            .arg(command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Calculate performance score based on hardware
    fn calculate_performance_score(cpu_cores: usize, memory_mb: usize) -> f64 {
        let cpu_score = cpu_cores as f64 * 1.0;
        let memory_score = (memory_mb as f64 / 1024.0) * 0.5; // GB * 0.5
        
        cpu_score + memory_score
    }

    /// Detect special features
    fn detect_features() -> Vec<String> {
        let mut features = Vec::new();
        
        // Check for GPU support
        if Self::command_exists("nvidia-smi") {
            features.push("cuda".to_string());
        }
        
        // Check for cross-compilation support
        features.push("cross-compile".to_string());
        
        features
    }
}

impl WorkerNodeManager {
    /// Create a new worker node manager
    #[instrument]
    pub fn new(config: WorkerConfig) -> Result<Self> {
        Ok(Self {
            config,
            workers: Arc::new(Mutex::new(HashMap::new())),
            coordinator_id: Uuid::new_v4().to_string(),
            discovery_socket: None,
            is_running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        })
    }

    /// Start the worker manager
    #[instrument(skip(self))]
    pub async fn start(&mut self) -> Result<()> {
        self.is_running.store(true, std::sync::atomic::Ordering::Relaxed);

        // Start discovery if enabled
        if self.config.auto_discovery {
            self.start_discovery().await?;
        }

        // Start heartbeat monitoring
        self.start_heartbeat_monitor().await?;

        info!("Worker node manager started");
        Ok(())
    }

    /// Stop the worker manager
    #[instrument(skip(self))]
    pub async fn stop(&mut self) -> Result<()> {
        self.is_running.store(false, std::sync::atomic::Ordering::Relaxed);

        // Send shutdown messages to all workers
        self.shutdown_all_workers().await?;

        info!("Worker node manager stopped");
        Ok(())
    }

    /// Register a new worker node
    #[instrument(skip(self, worker))]
    pub async fn register_worker(&self, mut worker: WorkerNode) -> Result<String> {
        let worker_id = worker.id.clone();
        worker.registration_time = SystemTime::now();
        worker.last_heartbeat = SystemTime::now();
        worker.status = WorkerStatus::Online;

        {
            let mut workers = self.workers.lock()
                .map_err(|_| CursedError::system_error("Failed to lock workers"))?;
            workers.insert(worker_id.clone(), worker);
        }

        info!(worker_id, "Worker registered");
        Ok(worker_id)
    }

    /// Unregister a worker node
    #[instrument(skip(self))]
    pub async fn unregister_worker(&self, worker_id: &str) -> Result<()> {
        {
            let mut workers = self.workers.lock()
                .map_err(|_| CursedError::system_error("Failed to lock workers"))?;
            workers.remove(worker_id);
        }

        info!(worker_id, "Worker unregistered");
        Ok(())
    }

    /// Get all registered workers
    pub async fn get_all_workers(&self) -> Result<Vec<WorkerNode>> {
        let workers = self.workers.lock()
            .map_err(|_| CursedError::system_error("Failed to lock workers"))?;
        Ok(workers.values().cloned().collect())
    }

    /// Get workers by status
    pub async fn get_workers_by_status(&self, status: WorkerStatus) -> Result<Vec<WorkerNode>> {
        let workers = self.workers.lock()
            .map_err(|_| CursedError::system_error("Failed to lock workers"))?;
        Ok(workers
            .values()
            .filter(|w| w.status == status)
            .cloned()
            .collect())
    }

    /// Get available workers for new jobs
    pub async fn get_available_workers(&self) -> Result<Vec<WorkerNode>> {
        let workers = self.workers.lock()
            .map_err(|_| CursedError::system_error("Failed to lock workers"))?;
        Ok(workers
            .values()
            .filter(|w| w.is_available())
            .cloned()
            .collect())
    }

    /// Find best worker for a specific job
    pub async fn find_best_worker(
        &self,
        target: &str,
        toolchain: Option<&str>,
    ) -> Result<Option<WorkerNode>> {
        let workers = self.workers.lock()
            .map_err(|_| CursedError::system_error("Failed to lock workers"))?;

        let mut best_worker = None;
        let mut best_score = 0.0;

        for worker in workers.values() {
            if !worker.is_available() || !worker.supports_target(target) {
                continue;
            }

            if let Some(tc) = toolchain {
                if !worker.has_toolchain(tc) {
                    continue;
                }
            }

            let score = worker.efficiency_score();
            if score > best_score {
                best_score = score;
                best_worker = Some(worker.clone());
            }
        }

        Ok(best_worker)
    }

    /// Update worker metrics
    #[instrument(skip(self, metrics))]
    pub async fn update_worker_metrics(
        &self,
        worker_id: &str,
        metrics: WorkerMetrics,
    ) -> Result<()> {
        {
            let mut workers = self.workers.lock()
                .map_err(|_| CursedError::system_error("Failed to lock workers"))?;
            
            if let Some(worker) = workers.get_mut(worker_id) {
                worker.update_metrics(metrics);
            } else {
                return Err(CursedError::system_error("Worker not found"));
            }
        }

        debug!(worker_id, "Worker metrics updated");
        Ok(())
    }

    /// Update worker status
    #[instrument(skip(self))]
    pub async fn update_worker_status(
        &self,
        worker_id: &str,
        status: WorkerStatus,
    ) -> Result<()> {
        {
            let mut workers = self.workers.lock()
                .map_err(|_| CursedError::system_error("Failed to lock workers"))?;
            
            if let Some(worker) = workers.get_mut(worker_id) {
                worker.status = status;
                worker.last_heartbeat = SystemTime::now();
            } else {
                return Err(CursedError::system_error("Worker not found"));
            }
        }

        debug!(worker_id, "Worker status updated");
        Ok(())
    }

    /// Start worker discovery service
    async fn start_discovery(&mut self) -> Result<()> {
        let socket = UdpSocket::bind("0.0.0.0:0").await
            .map_err(|e| CursedError::system_error(&format!("Failed to bind discovery socket: {}", e)))?;
        
        socket.set_broadcast(true)
            .map_err(|e| CursedError::system_error(&format!("Failed to enable broadcast: {}", e)))?;

        self.discovery_socket = Some(socket);

        // Start discovery broadcast task
        let manager = self.clone_for_tasks();
        tokio::spawn(async move {
            manager.run_discovery_broadcast().await;
        });

        info!("Worker discovery service started");
        Ok(())
    }

    /// Start heartbeat monitoring
    async fn start_heartbeat_monitor(&self) -> Result<()> {
        let manager = self.clone_for_tasks();
        tokio::spawn(async move {
            manager.run_heartbeat_monitor().await;
        });

        info!("Heartbeat monitor started");
        Ok(())
    }

    /// Run discovery broadcast loop
    async fn run_discovery_broadcast(&self) {
        while self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            if let Err(e) = self.broadcast_discovery().await {
                warn!(error = ?e, "Discovery broadcast failed");
            }

            tokio::time::sleep(self.config.discovery_interval).await;
        }
    }

    /// Broadcast discovery message
    async fn broadcast_discovery(&self) -> Result<()> {
        if let Some(socket) = &self.discovery_socket {
            let local_address = std::env::var("CURSED_COORDINATOR_ADDRESS")
                .unwrap_or_else(|_| "127.0.0.1:9000".to_string())
                .parse()
                .unwrap_or_else(|_| "127.0.0.1:9000".parse().unwrap());
                
            let message = WorkerMessage::DiscoveryBroadcast {
                coordinator_id: self.coordinator_id.clone(),
                coordinator_address: local_address,
            };

            let serialized = bincode::serialize(&message)
                .map_err(|e| CursedError::system_error(&format!("Serialization failed: {}", e)))?;

            // Broadcast to common subnets
            let broadcast_addresses = [
                "192.168.1.255:9001",
                "192.168.0.255:9001",
                "10.0.0.255:9001",
                "172.16.255.255:9001",
            ];

            for addr_str in &broadcast_addresses {
                if let Ok(addr) = addr_str.parse::<SocketAddr>() {
                    let _ = socket.send_to(&serialized, addr).await;
                }
            }

            debug!("Discovery broadcast sent");
        }

        Ok(())
    }

    /// Run heartbeat monitoring loop
    async fn run_heartbeat_monitor(&self) {
        while self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            if let Err(e) = self.check_worker_heartbeats().await {
                warn!(error = ?e, "Heartbeat check failed");
            }

            tokio::time::sleep(self.config.heartbeat_interval).await;
        }
    }

    /// Check worker heartbeats and mark stale workers as offline
    async fn check_worker_heartbeats(&self) -> Result<()> {
        let max_heartbeat_age = self.config.heartbeat_interval
            * self.config.max_missed_heartbeats as u32;

        let mut stale_workers = Vec::new();

        {
            let workers = self.workers.lock()
                .map_err(|_| CursedError::system_error("Failed to lock workers"))?;
            
            for (worker_id, worker) in workers.iter() {
                if worker.is_heartbeat_stale(max_heartbeat_age) {
                    stale_workers.push(worker_id.clone());
                }
            }
        }

        // Mark stale workers as offline
        for worker_id in stale_workers {
            self.update_worker_status(&worker_id, WorkerStatus::Offline).await?;
            warn!(worker_id, "Worker marked offline due to stale heartbeat");
        }

        Ok(())
    }

    /// Shutdown all workers gracefully
    async fn shutdown_all_workers(&self) -> Result<()> {
        let workers = self.get_all_workers().await?;
        
        for worker in workers {
            if let Err(e) = self.send_shutdown_message(&worker).await {
                warn!(worker_id = worker.id, error = ?e, "Failed to send shutdown message");
            }
        }

        Ok(())
    }

    /// Send shutdown message to a worker
    async fn send_shutdown_message(&self, worker: &WorkerNode) -> Result<()> {
        let timeout_duration = Duration::from_secs(10);
        
        match timeout(timeout_duration, TcpStream::connect(worker.address)).await {
            Ok(Ok(mut stream)) => {
                let message = WorkerMessage::Shutdown {
                    graceful: true,
                    timeout: Duration::from_secs(30),
                };

                let serialized = bincode::serialize(&message)
                    .map_err(|e| CursedError::system_error(&format!("Serialization failed: {}", e)))?;

                if let Err(e) = stream.write_all(&serialized).await {
                    return Err(CursedError::system_error(&format!("Failed to send shutdown: {}", e)));
                }

                debug!(worker_id = worker.id, "Shutdown message sent");
            }
            Ok(Err(e)) => {
                return Err(CursedError::system_error(&format!("Connection failed: {}", e)));
            }
            Err(_) => {
                return Err(CursedError::system_error("Connection timeout"));
            }
        }

        Ok(())
    }

    /// Clone for async tasks
    fn clone_for_tasks(&self) -> WorkerManagerHandle {
        WorkerManagerHandle {
            config: self.config.clone(),
            workers: self.workers.clone(),
            coordinator_id: self.coordinator_id.clone(),
            is_running: self.is_running.clone(),
        }
    }
}

/// Handle for async tasks
#[derive(Debug)]
struct WorkerManagerHandle {
    config: WorkerConfig,
    workers: Arc<Mutex<HashMap<String, WorkerNode>>>,
    coordinator_id: String,
    is_running: Arc<std::sync::atomic::AtomicBool>,
}

impl WorkerManagerHandle {
    async fn run_discovery_broadcast(&self) {
        while self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            if let Err(e) = self.broadcast_discovery().await {
                warn!(error = ?e, "Discovery broadcast failed");
            }

            tokio::time::sleep(self.config.discovery_interval).await;
        }
    }

    async fn run_heartbeat_monitor(&self) {
        while self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            if let Err(e) = self.check_worker_heartbeats().await {
                warn!(error = ?e, "Heartbeat check failed");
            }

            tokio::time::sleep(self.config.heartbeat_interval).await;
        }
    }

    async fn broadcast_discovery(&self) -> Result<()> {
        let local_address = std::env::var("CURSED_COORDINATOR_ADDRESS")
            .unwrap_or_else(|_| "127.0.0.1:9000".to_string())
            .parse()
            .unwrap_or_else(|_| "127.0.0.1:9000".parse().unwrap());
            
        let message = WorkerMessage::DiscoveryBroadcast {
            coordinator_id: self.coordinator_id.clone(),
            coordinator_address: local_address,
        };

        let serialized = bincode::serialize(&message)
            .map_err(|e| CursedError::system_error(&format!("Serialization failed: {}", e)))?;

        // Create UDP socket for broadcasting
        let socket = UdpSocket::bind("0.0.0.0:0").await
            .map_err(|e| CursedError::system_error(&format!("Failed to bind discovery socket: {}", e)))?;
        
        socket.set_broadcast(true)
            .map_err(|e| CursedError::system_error(&format!("Failed to enable broadcast: {}", e)))?;

        // Broadcast to common subnets
        let broadcast_addresses = [
            "192.168.1.255:9001",
            "192.168.0.255:9001", 
            "10.0.0.255:9001",
            "172.16.255.255:9001",
        ];

        for addr_str in &broadcast_addresses {
            if let Ok(addr) = addr_str.parse::<SocketAddr>() {
                let _ = socket.send_to(&serialized, addr).await;
            }
        }

        debug!("Discovery broadcast sent");
        Ok(())
    }

    async fn check_worker_heartbeats(&self) -> Result<()> {
        let max_heartbeat_age = self.config.heartbeat_interval
            * self.config.max_missed_heartbeats as u32;

        let mut stale_workers = Vec::new();

        {
            let workers = self.workers.lock()
                .map_err(|_| CursedError::system_error("Failed to lock workers"))?;
            
            for (worker_id, worker) in workers.iter() {
                if worker.is_heartbeat_stale(max_heartbeat_age) {
                    stale_workers.push(worker_id.clone());
                }
            }
        }

        // Mark stale workers as offline
        for worker_id in stale_workers {
            // Update worker status to offline
            {
                let mut workers = self.workers.lock()
                    .map_err(|_| CursedError::system_error("Failed to lock workers"))?;
                
                if let Some(worker) = workers.get_mut(&worker_id) {
                    worker.status = WorkerStatus::Offline;
                }
            }
            warn!(worker_id, "Worker marked offline due to stale heartbeat");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[tokio::test]
    async fn test_worker_creation() {
        let capabilities = WorkerCapabilities {
            cpu_cores: 4,
            memory_mb: 8192,
            disk_space_mb: 100000,
            supported_targets: vec!["x86_64-unknown-linux-gnu".to_string()],
            toolchains: HashMap::new(),
            max_concurrent_jobs: 4,
            performance_score: 4.0,
            network_bandwidth: 100.0,
            features: Vec::new(),
        };

        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9001);
        let worker = WorkerNode::new(address, capabilities);

        assert!(!worker.id.is_empty());
        assert_eq!(worker.address, address);
        assert_eq!(worker.capabilities.cpu_cores, 4);
    }

    #[tokio::test]
    async fn test_worker_availability() {
        let capabilities = WorkerCapabilities {
            cpu_cores: 4,
            memory_mb: 8192,
            disk_space_mb: 100000,
            supported_targets: vec!["x86_64-unknown-linux-gnu".to_string()],
            toolchains: HashMap::new(),
            max_concurrent_jobs: 2,
            performance_score: 4.0,
            network_bandwidth: 100.0,
            features: Vec::new(),
        };

        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9001);
        let mut worker = WorkerNode::new(address, capabilities);
        worker.status = WorkerStatus::Online;
        worker.metrics.active_jobs = 1;

        assert!(worker.is_available());

        worker.metrics.active_jobs = 2;
        assert!(!worker.is_available());
    }

    #[tokio::test]
    async fn test_load_factor() {
        let capabilities = WorkerCapabilities {
            cpu_cores: 4,
            memory_mb: 8192,
            disk_space_mb: 100000,
            supported_targets: vec!["x86_64-unknown-linux-gnu".to_string()],
            toolchains: HashMap::new(),
            max_concurrent_jobs: 4,
            performance_score: 4.0,
            network_bandwidth: 100.0,
            features: Vec::new(),
        };

        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9001);
        let mut worker = WorkerNode::new(address, capabilities);
        worker.metrics.active_jobs = 2;

        assert_eq!(worker.load_factor(), 0.5);
    }

    #[tokio::test]
    async fn test_manager_creation() {
        let config = WorkerConfig::default();
        let manager = WorkerNodeManager::new(config);
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_worker_registration() {
        let config = WorkerConfig::default();
        let manager = WorkerNodeManager::new(config).unwrap();

        let capabilities = WorkerCapabilities {
            cpu_cores: 4,
            memory_mb: 8192,
            disk_space_mb: 100000,
            supported_targets: vec!["x86_64-unknown-linux-gnu".to_string()],
            toolchains: HashMap::new(),
            max_concurrent_jobs: 4,
            performance_score: 4.0,
            network_bandwidth: 100.0,
            features: Vec::new(),
        };

        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9001);
        let worker = WorkerNode::new(address, capabilities);
        let worker_id = worker.id.clone();

        let result = manager.register_worker(worker).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), worker_id);

        let workers = manager.get_all_workers().await.unwrap();
        assert_eq!(workers.len(), 1);
        assert_eq!(workers[0].id, worker_id);
    }
}
