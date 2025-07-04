use crate::error::{Result, CursedError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use uuid::Uuid;

/// Capabilities of a worker node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerCapabilities {
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub supported_targets: Vec<String>,
    pub max_parallel_jobs: u32,
    pub network_bandwidth_mbps: u32,
}

impl Default for WorkerCapabilities {
    fn default() -> Self {
        Self {
            cpu_cores: 8,
            memory_gb: 16,
            supported_targets: vec!["x86_64-unknown-linux-gnu".to_string()],
            max_parallel_jobs: 4,
            network_bandwidth_mbps: 1000,
        }
    }
}

/// Status of a worker node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkerStatus {
    Online,
    Offline,
    Busy,
    Maintenance,
    Error(String),
}

/// Performance metrics for a worker node
#[derive(Debug, Clone)]
pub struct WorkerMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub active_jobs: u32,
    pub completed_jobs: u64,
    pub failed_jobs: u64,
    pub average_job_duration: Duration,
    pub last_heartbeat: Instant,
    pub network_latency_ms: u32,
}

impl Default for WorkerMetrics {
    fn default() -> Self {
        Self {
            cpu_usage_percent: 0.0,
            memory_usage_percent: 0.0,
            active_jobs: 0,
            completed_jobs: 0,
            failed_jobs: 0,
            average_job_duration: Duration::from_secs(0),
            last_heartbeat: Instant::now(),
            network_latency_ms: 0,
        }
    }
}

/// A worker node in the distributed system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerNode {
    pub id: String,
    pub hostname: String,
    pub port: u16,
    pub capabilities: WorkerCapabilities,
    pub status: WorkerStatus,
    #[serde(skip)]
    pub metrics: WorkerMetrics,
    pub tags: HashMap<String, String>,
}

impl WorkerNode {
    pub fn new(hostname: String, port: u16) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            hostname,
            port,
            capabilities: WorkerCapabilities::default(),
            status: WorkerStatus::Online,
            metrics: WorkerMetrics::default(),
            tags: HashMap::new(),
        }
    }

    pub fn with_capabilities(mut self, capabilities: WorkerCapabilities) -> Self {
        self.capabilities = capabilities;
        self
    }

    pub fn with_tags(mut self, tags: HashMap<String, String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn is_available(&self) -> bool {
        matches!(self.status, WorkerStatus::Online) && 
        self.metrics.active_jobs < self.capabilities.max_parallel_jobs
    }

    pub fn can_handle_job(&self, target: &str) -> bool {
        self.capabilities.supported_targets.contains(&target.to_string())
    }

    pub fn get_load_factor(&self) -> f64 {
        if self.capabilities.max_parallel_jobs == 0 {
            return 1.0;
        }
        
        let job_load = self.metrics.active_jobs as f64 / self.capabilities.max_parallel_jobs as f64;
        let cpu_load = self.metrics.cpu_usage_percent / 100.0;
        let memory_load = self.metrics.memory_usage_percent / 100.0;
        
        // Weighted average of different load factors
        (job_load * 0.4 + cpu_load * 0.3 + memory_load * 0.3).min(1.0)
    }

    pub fn update_metrics(&mut self, metrics: WorkerMetrics) {
        self.metrics = metrics;
    }

    pub fn set_status(&mut self, status: WorkerStatus) {
        self.status = status;
    }

    pub fn increment_active_jobs(&mut self) {
        self.metrics.active_jobs += 1;
    }

    pub fn decrement_active_jobs(&mut self) {
        if self.metrics.active_jobs > 0 {
            self.metrics.active_jobs -= 1;
        }
    }

    pub fn record_job_completion(&mut self, duration: Duration, success: bool) {
        if success {
            self.metrics.completed_jobs += 1;
        } else {
            self.metrics.failed_jobs += 1;
        }

        // Update average job duration using exponential moving average
        let alpha = 0.1;
        let new_avg = self.metrics.average_job_duration.as_secs_f64() * (1.0 - alpha) 
                     + duration.as_secs_f64() * alpha;
        self.metrics.average_job_duration = Duration::from_secs_f64(new_avg);
    }

    pub fn get_success_rate(&self) -> f64 {
        let total_jobs = self.metrics.completed_jobs + self.metrics.failed_jobs;
        if total_jobs == 0 {
            return 1.0;
        }
        self.metrics.completed_jobs as f64 / total_jobs as f64
    }

    pub fn get_address(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }

    pub fn update_heartbeat(&mut self) {
        self.metrics.last_heartbeat = Instant::now();
    }

    pub fn is_healthy(&self) -> bool {
        let heartbeat_age = self.metrics.last_heartbeat.elapsed();
        heartbeat_age < Duration::from_secs(30) && // Heartbeat within 30 seconds
        matches!(self.status, WorkerStatus::Online) &&
        self.get_success_rate() > 0.5 // At least 50% success rate
    }
}

/// Worker node manager for handling multiple workers
#[derive(Debug)]
pub struct WorkerNodeManager {
    workers: HashMap<String, WorkerNode>,
    health_check_interval: Duration,
}

impl WorkerNodeManager {
    pub fn new() -> Self {
        Self {
            workers: HashMap::new(),
            health_check_interval: Duration::from_secs(10),
        }
    }

    pub fn add_worker(&mut self, worker: WorkerNode) -> Result<()> {
        tracing::info!("Adding worker: {} ({})", worker.id, worker.get_address());
        self.workers.insert(worker.id.clone(), worker);
        Ok(())
    }

    pub fn remove_worker(&mut self, worker_id: &str) -> Result<()> {
        if let Some(worker) = self.workers.remove(worker_id) {
            tracing::info!("Removed worker: {} ({})", worker.id, worker.get_address());
            Ok(())
        } else {
            Err(CursedError::system_error(&format!("Worker not found: {}", worker_id)))
        }
    }

    pub fn get_worker(&self, worker_id: &str) -> Option<&WorkerNode> {
        self.workers.get(worker_id)
    }

    pub fn get_worker_mut(&mut self, worker_id: &str) -> Option<&mut WorkerNode> {
        self.workers.get_mut(worker_id)
    }

    pub fn get_available_workers(&self) -> Vec<&WorkerNode> {
        self.workers.values()
            .filter(|w| w.is_available())
            .collect()
    }

    pub fn get_workers_for_target(&self, target: &str) -> Vec<&WorkerNode> {
        self.workers.values()
            .filter(|w| w.can_handle_job(target))
            .collect()
    }

    pub fn get_least_loaded_worker(&self) -> Option<&WorkerNode> {
        self.get_available_workers()
            .into_iter()
            .min_by(|a, b| a.get_load_factor().partial_cmp(&b.get_load_factor()).unwrap())
    }

    pub fn get_worker_count(&self) -> usize {
        self.workers.len()
    }

    pub fn get_healthy_worker_count(&self) -> usize {
        self.workers.values()
            .filter(|w| w.is_healthy())
            .count()
    }

    pub fn perform_health_check(&mut self) -> Result<()> {
        let mut unhealthy_workers = Vec::new();

        for (id, worker) in &mut self.workers {
            if !worker.is_healthy() {
                tracing::warn!("Worker {} is unhealthy", id);
                worker.set_status(WorkerStatus::Error("Health check failed".to_string()));
                unhealthy_workers.push(id.clone());
            }
        }

        if !unhealthy_workers.is_empty() {
            tracing::warn!("Found {} unhealthy workers", unhealthy_workers.len());
        }

        Ok(())
    }

    pub fn get_total_capacity(&self) -> u32 {
        self.workers.values()
            .map(|w| w.capabilities.max_parallel_jobs)
            .sum()
    }

    pub fn get_current_utilization(&self) -> f64 {
        let total_capacity = self.get_total_capacity();
        if total_capacity == 0 {
            return 0.0;
        }

        let total_active_jobs: u32 = self.workers.values()
            .map(|w| w.metrics.active_jobs)
            .sum();

        total_active_jobs as f64 / total_capacity as f64
    }
}

impl Default for WorkerNodeManager {
    fn default() -> Self {
        Self::new()
    }
}
