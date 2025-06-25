// Worker Node Management for Distributed Compilation
//
// Manages remote worker nodes that perform actual compilation tasks.
// Handles worker registration, capability detection, health monitoring, and task assignment.

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
/// Capabilities and specifications of a worker node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerCapabilities {
    /// Number of CPU cores available
    /// Available memory in MB
    /// Available disk space in MB
    /// Supported target architectures
    /// Available compiler toolchains
    /// Maximum concurrent jobs
    /// Performance benchmark score
    /// Network bandwidth (Mbps)
    /// Specialized features (GPU compilation, etc.)
/// Current status of a worker node
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkerStatus {
    /// Worker is online and available
    /// Worker is online but busy with tasks
    /// Worker is offline or unreachable
    /// Worker is in maintenance mode
    /// Worker has encountered an error
    /// Worker is starting up
    /// Worker is shutting down
/// Performance and health metrics for a worker node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerMetrics {
    /// Current CPU utilization (0.0 - 1.0)
    /// Current memory utilization (0.0 - 1.0)
    /// Current disk utilization (0.0 - 1.0)
    /// Current network utilization (0.0 - 1.0)
    /// Number of active compilation jobs
    /// Total jobs completed
    /// Total jobs failed
    /// Average job completion time
    /// Total uptime
    /// Last update timestamp
    /// Current load average
    /// Temperature metrics (for hardware monitoring)
impl Default for WorkerMetrics {
    fn default() -> Self {
        Self {
        }
    }
/// Configuration for worker node management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerConfig {
    /// How often to send heartbeats
    /// Timeout for worker responses
    /// Number of missed heartbeats before marking offline
    /// Metrics update interval
    /// Enable automatic worker discovery
    /// Discovery broadcast interval
impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Messages exchanged between coordinator and workers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkerMessage {
    /// Worker registration request
    RegisterRequest {
    /// Registration acknowledgment
    RegisterResponse {
    /// Heartbeat from worker
    Heartbeat {
    /// Heartbeat acknowledgment
    HeartbeatAck {
    /// Job assignment to worker
    JobAssignment {
    /// Job completion notification
    JobCompletion {
    /// Job failure notification
    JobFailure {
    /// Shutdown request
    Shutdown {
    /// Capability update
    CapabilityUpdate {
    /// Status update
    StatusUpdate {
    /// Discovery broadcast
    DiscoveryBroadcast {
    /// Discovery response
    DiscoveryResponse {
/// Worker node manager
pub struct WorkerNodeManager {
impl WorkerNode {
    /// Create a new worker node
    pub fn new(
    ) -> Self {
        Self {
        }
    }

    /// Check if worker is available for new jobs
    pub fn is_available(&self) -> bool {
        matches!(self.status, WorkerStatus::Online) &&
        self.metrics.active_jobs < self.capabilities.max_concurrent_jobs
    /// Get current load factor (0.0 = no load, 1.0 = fully loaded)
    pub fn load_factor(&self) -> f64 {
        if self.capabilities.max_concurrent_jobs == 0 {
            return 1.0;
        }
        self.metrics.active_jobs as f64 / self.capabilities.max_concurrent_jobs as f64
    /// Calculate worker efficiency score
    pub fn efficiency_score(&self) -> f64 {
        let base_score = self.capabilities.performance_score;
        let load_penalty = self.load_factor() * 0.5; // Reduce score based on load
        let reliability = if self.metrics.completed_jobs + self.metrics.failed_jobs > 0 {
            self.metrics.completed_jobs as f64 / (self.metrics.completed_jobs + self.metrics.failed_jobs) as f64
        } else {
            1.0
        
        base_score * (1.0 - load_penalty) * reliability
    /// Check if worker supports a target architecture
    pub fn supports_target(&self, target: &str) -> bool {
        self.capabilities.supported_targets.contains(&target.to_string())
    /// Check if worker has a specific toolchain
    pub fn has_toolchain(&self, toolchain: &str) -> bool {
        self.capabilities.toolchains.contains_key(toolchain)
    /// Update worker metrics
    pub fn update_metrics(&mut self, new_metrics: WorkerMetrics) {
        self.metrics = new_metrics;
        self.metrics.last_updated = SystemTime::now();
        self.last_heartbeat = SystemTime::now();
    /// Check if worker heartbeat is stale
    pub fn is_heartbeat_stale(&self, max_age: Duration) -> bool {
        self.last_heartbeat
            .elapsed()
            .unwrap_or(Duration::MAX)
            > max_age
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
            network_bandwidth: 100.0, // Default 100 Mbps
        })
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
    /// Get available disk space
    fn get_disk_space() -> Result<usize> {
        use std::fs;
        
        // Try to get disk space for current directory
        if let Ok(metadata) = fs::metadata(".") {
            // This is a simplified implementation
            // In practice, you'd use platform-specific APIs
            return Ok(100 * 1024); // 100GB default
        Ok(50 * 1024) // 50GB fallback
    /// Detect supported target architectures
    fn detect_targets() -> Vec<String> {
        vec![
        ]
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
        toolchains
    /// Check if a command exists in PATH
    fn command_exists(command: &str) -> bool {
        std::process::Command::new("which")
            .arg(command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    /// Calculate performance score based on hardware
    fn calculate_performance_score(cpu_cores: usize, memory_mb: usize) -> f64 {
        let cpu_score = cpu_cores as f64 * 1.0;
        let memory_score = (memory_mb as f64 / 1024.0) * 0.5; // GB * 0.5
        
        cpu_score + memory_score
    /// Detect special features
    fn detect_features() -> Vec<String> {
        let mut features = Vec::new();
        
        // Check for GPU support
        if Self::command_exists("nvidia-smi") {
            features.push("cuda".to_string());
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
        })
    /// Start the worker manager
    #[instrument(skip(self))]
    pub async fn start(&mut self) -> Result<()> {
        self.is_running.store(true, std::sync::atomic::Ordering::Relaxed);

        // Start discovery if enabled
        if self.config.auto_discovery {
            self.start_discovery().await?;
        // Start heartbeat monitoring
        self.start_heartbeat_monitor().await?;

        info!("Worker node manager started");
        Ok(())
    /// Stop the worker manager
    #[instrument(skip(self))]
    pub async fn stop(&mut self) -> Result<()> {
        self.is_running.store(false, std::sync::atomic::Ordering::Relaxed);

        // Send shutdown messages to all workers
        self.shutdown_all_workers().await?;

        info!("Worker node manager stopped");
        Ok(())
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
        info!(worker_id, "Worker registered");
        Ok(worker_id)
    /// Unregister a worker node
    #[instrument(skip(self))]
    pub async fn unregister_worker(&self, worker_id: &str) -> Result<()> {
        {
            let mut workers = self.workers.lock()
                .map_err(|_| CursedError::system_error("Failed to lock workers"))?;
            workers.remove(worker_id);
        info!(worker_id, "Worker unregistered");
        Ok(())
    /// Get all registered workers
    pub async fn get_all_workers(&self) -> Result<Vec<WorkerNode>> {
        let workers = self.workers.lock()
            .map_err(|_| CursedError::system_error("Failed to lock workers"))?;
        Ok(workers.values().cloned().collect())
    /// Get workers by status
    pub async fn get_workers_by_status(&self, status: WorkerStatus) -> Result<Vec<WorkerNode>> {
        let workers = self.workers.lock()
            .map_err(|_| CursedError::system_error("Failed to lock workers"))?;
        Ok(workers
            .values()
            .filter(|w| w.status == status)
            .cloned()
            .collect())
    /// Get available workers for new jobs
    pub async fn get_available_workers(&self) -> Result<Vec<WorkerNode>> {
        let workers = self.workers.lock()
            .map_err(|_| CursedError::system_error("Failed to lock workers"))?;
        Ok(workers
            .values()
            .filter(|w| w.is_available())
            .cloned()
            .collect())
    /// Find best worker for a specific job
    pub async fn find_best_worker(
    ) -> Result<Option<WorkerNode>> {
        let workers = self.workers.lock()
            .map_err(|_| CursedError::system_error("Failed to lock workers"))?;

        let mut best_worker = None;
        let mut best_score = 0.0;

        for worker in workers.values() {
            if !worker.is_available() || !worker.supports_target(target) {
                continue;
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
    /// Update worker metrics
    #[instrument(skip(self, metrics))]
    pub async fn update_worker_metrics(
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
    /// Update worker status
    #[instrument(skip(self))]
    pub async fn update_worker_status(
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
    /// Start heartbeat monitoring
    async fn start_heartbeat_monitor(&self) -> Result<()> {
        let manager = self.clone_for_tasks();
        tokio::spawn(async move {
            manager.run_heartbeat_monitor().await;
        });

        info!("Heartbeat monitor started");
        Ok(())
    /// Run discovery broadcast loop
    async fn run_discovery_broadcast(&self) {
        while self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            if let Err(e) = self.broadcast_discovery().await {
                warn!(error = ?e, "Discovery broadcast failed");
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

            let serialized = bincode::serialize(&message)
                .map_err(|e| CursedError::system_error(&format!("Serialization failed: {}", e)))?;

            // Broadcast to common subnets
            let broadcast_addresses = [
            ];

            for addr_str in &broadcast_addresses {
                if let Ok(addr) = addr_str.parse::<SocketAddr>() {
                    let _ = socket.send_to(&serialized, addr).await;
                }
            }

            debug!("Discovery broadcast sent");
        Ok(())
    /// Run heartbeat monitoring loop
    async fn run_heartbeat_monitor(&self) {
        while self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            if let Err(e) = self.check_worker_heartbeats().await {
                warn!(error = ?e, "Heartbeat check failed");
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
        // Mark stale workers as offline
        for worker_id in stale_workers {
            self.update_worker_status(&worker_id, WorkerStatus::Offline).await?;
            warn!(worker_id, "Worker marked offline due to stale heartbeat");
        Ok(())
    /// Shutdown all workers gracefully
    async fn shutdown_all_workers(&self) -> Result<()> {
        let workers = self.get_all_workers().await?;
        
        for worker in workers {
            if let Err(e) = self.send_shutdown_message(&worker).await {
                warn!(worker_id = worker.id, error = ?e, "Failed to send shutdown message");
            }
        }

        Ok(())
    /// Send shutdown message to a worker
    async fn send_shutdown_message(&self, worker: &WorkerNode) -> Result<()> {
        let timeout_duration = Duration::from_secs(10);
        
        match timeout(timeout_duration, TcpStream::connect(worker.address)).await {
            Ok(Ok(mut stream)) => {
                let message = WorkerMessage::Shutdown {

                let serialized = bincode::serialize(&message)
                    .map_err(|e| CursedError::system_error(&format!("Serialization failed: {}", e)))?;

                if let Err(e) = stream.write_all(&serialized).await {
                    return Err(CursedError::system_error(&format!("Failed to send shutdown: {}", e)));
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
    /// Clone for async tasks
    fn clone_for_tasks(&self) -> WorkerManagerHandle {
        WorkerManagerHandle {
        }
    }
/// Handle for async tasks
#[derive(Debug)]
struct WorkerManagerHandle {
impl WorkerManagerHandle {
    async fn run_discovery_broadcast(&self) {
        while self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            if let Err(e) = self.broadcast_discovery().await {
                warn!(error = ?e, "Discovery broadcast failed");
            tokio::time::sleep(self.config.discovery_interval).await;
        }
    }

    async fn run_heartbeat_monitor(&self) {
        while self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            if let Err(e) = self.check_worker_heartbeats().await {
                warn!(error = ?e, "Heartbeat check failed");
            tokio::time::sleep(self.config.heartbeat_interval).await;
        }
    }

    async fn broadcast_discovery(&self) -> Result<()> {
        let local_address = std::env::var("CURSED_COORDINATOR_ADDRESS")
            .unwrap_or_else(|_| "127.0.0.1:9000".to_string())
            .parse()
            .unwrap_or_else(|_| "127.0.0.1:9000".parse().unwrap());
            
        let message = WorkerMessage::DiscoveryBroadcast {

        let serialized = bincode::serialize(&message)
            .map_err(|e| CursedError::system_error(&format!("Serialization failed: {}", e)))?;

        // Create UDP socket for broadcasting
        let socket = UdpSocket::bind("0.0.0.0:0").await
            .map_err(|e| CursedError::system_error(&format!("Failed to bind discovery socket: {}", e)))?;
        
        socket.set_broadcast(true)
            .map_err(|e| CursedError::system_error(&format!("Failed to enable broadcast: {}", e)))?;

        // Broadcast to common subnets
        let broadcast_addresses = [
        ];

        for addr_str in &broadcast_addresses {
            if let Ok(addr) = addr_str.parse::<SocketAddr>() {
                let _ = socket.send_to(&serialized, addr).await;
            }
        }

        debug!("Discovery broadcast sent");
        Ok(())
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
        Ok(())
    }
}

