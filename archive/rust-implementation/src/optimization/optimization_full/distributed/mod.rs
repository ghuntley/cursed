// Comprehensive Distributed Compilation System for CURSED
//
// This module provides a production-ready distributed compilation infrastructure
// with intelligent work distribution, fault tolerance, and optimization.

use crate::error::Result;
use crate::error::CursedError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// Core modules
pub mod distributed_compiler;
pub mod worker_node;
pub mod load_balancer;
pub mod network_optimizer;
pub mod compilation_cache;

// Re-export main types
pub use distributed_compiler::{DistributedCompiler, CompilerConfig, CompilationJob};
pub use worker_node::{WorkerNode, WorkerCapabilities, WorkerStatus, WorkerMetrics};
pub use load_balancer::{LoadBalancer, LoadBalancingStrategy, NodeSelection};
pub use network_optimizer::{NetworkOptimizer, NetworkConfig, CompressionLevel};
pub use compilation_cache::{CompilationCache, CacheEntry, CacheStrategy};

/// Configuration for the distributed compilation system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedConfig {
    /// Coordinator port for managing workers
    /// Maximum number of worker nodes
    /// Task distribution chunk size
    /// Network timeout for communications
    /// Enable fault tolerance features
    /// Enable compilation caching
    /// Load balancing strategy
    /// Network optimization settings
    /// Enable performance monitoring
impl Default for DistributedConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Statistics for distributed compilation performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedStats {
    /// Total compilation jobs processed
    /// Successfully completed jobs
    /// Failed jobs
    /// Average job duration
    /// Total time saved vs sequential compilation
    /// Speedup factor achieved
    /// Network overhead
    /// Cache hit rate
    /// Number of active workers
    /// Load balancing efficiency
    /// Fault recovery operations
impl Default for DistributedStats {
    fn default() -> Self {
        Self {
        }
    }
/// Main distributed compilation system coordinator
#[derive(Debug)]
pub struct DistributedCompilationSystem {
impl DistributedCompilationSystem {
    /// Create a new distributed compilation system
    pub fn new(config: DistributedConfig) -> Result<Self> {
        let compiler = distributed_compiler::DistributedCompiler::new(
            distributed_compiler::CompilerConfig {
            }
        )?;

        let load_balancer = load_balancer::LoadBalancer::new(
        )?;

        let network_optimizer = network_optimizer::NetworkOptimizer::new(
            config.network_config.clone()
        )?;

        let cache = if config.caching_enabled {
            compilation_cache::CompilationCache::new(
                compilation_cache::CacheStrategy::LruWithTtl {
                }
            )?
        } else {
            compilation_cache::CompilationCache::disabled()

        Ok(Self {
        })
    /// Start the distributed compilation system
    pub async fn start(&mut self) -> Result<()> {
        tracing::info!("Starting distributed compilation system");

        // Start the compiler subsystem
        self.compiler.start().await?;

        // Initialize load balancer
        self.load_balancer.start().await?;

        // Start network optimization
        self.network_optimizer.start().await?;

        // Initialize cache if enabled
        if self.config.caching_enabled {
            self.cache.initialize().await?;
        tracing::info!(
            "Distributed compilation system started"
        );

        Ok(())
    /// Stop the distributed compilation system
    pub async fn stop(&mut self) -> Result<()> {
        tracing::info!("Stopping distributed compilation system");

        // Stop all subsystems
        self.compiler.stop().await?;
        self.load_balancer.stop().await?;
        self.network_optimizer.stop().await?;
        self.cache.shutdown().await?;

        tracing::info!("Distributed compilation system stopped");
        Ok(())
    /// Submit a compilation job for distributed processing
    pub async fn compile_distributed(&mut self, job: CompilationJob) -> Result<Vec<u8>> {
        use tracing::instrument;

        #[instrument(skip(self, job))]
        async fn inner_compile(
        ) -> Result<Vec<u8>> {
            let start_time = std::time::Instant::now();

            // Check cache first
            if let Some(cached_result) = system.cache.get(&job.cache_key()).await? {
                system.stats.cache_hit_rate = 
                    (system.stats.cache_hit_rate * system.stats.total_jobs as f64 + 1.0) 
                    / (system.stats.total_jobs + 1) as f64;
                tracing::debug!("Cache hit for job {}", job.id);
                return Ok(cached_result.output);
            // Submit job to distributed compiler
            let result = system.compiler.submit_job(job.clone()).await?;

            // Cache successful results
            if result.success {
                system.cache.put(
                    compilation_cache::CacheEntry {
                    }
                ).await?;
            // Update statistics
            let duration = start_time.elapsed();
            system.update_stats(&job, &result, duration).await?;

            if result.success {
                Ok(result.output)
            } else {
                Err(crate::error::CursedError::system_error(&format!(
                    result.error_message.unwrap_or_else(|| "Unknown error".to_string())
                )))
            }
        }

        inner_compile(self, job).await
    /// Get current system statistics
    pub fn get_statistics(&self) -> &DistributedStats {
        &self.stats
    /// Get active worker nodes
    pub async fn get_workers(&self) -> Result<Vec<worker_node::WorkerNode>> {
        self.load_balancer.get_active_workers().await
    /// Add a new worker node
    pub async fn add_worker(&mut self, worker: worker_node::WorkerNode) -> Result<()> {
        self.load_balancer.add_worker(worker).await
    /// Remove a worker node
    pub async fn remove_worker(&mut self, worker_id: &str) -> Result<()> {
        self.load_balancer.remove_worker(worker_id).await
    /// Update system configuration
    pub async fn update_config(&mut self, new_config: DistributedConfig) -> Result<()> {
        tracing::info!("Updating distributed compilation configuration");
        
        // Update load balancer configuration
        if new_config.load_balancing != self.config.load_balancing {
            self.load_balancer.update_strategy(new_config.load_balancing.clone()).await?;
        // Update network optimization
        if new_config.network_config != self.config.network_config {
            self.network_optimizer.update_config(new_config.network_config.clone()).await?;
        // Update compiler configuration
        let compiler_config = distributed_compiler::CompilerConfig {
        self.compiler.update_config(compiler_config).await?;

        self.config = new_config;
        Ok(())
    /// Update system statistics
    async fn update_stats(
    ) -> Result<()> {
        self.stats.total_jobs += 1;
        
        if result.success {
            self.stats.completed_jobs += 1;
        } else {
            self.stats.failed_jobs += 1;
        // Update average duration using exponential moving average
        let alpha = 0.1;
        let new_avg = self.stats.average_duration.as_secs_f64() * (1.0 - alpha) 
                     + duration.as_secs_f64() * alpha;
        self.stats.average_duration = Duration::from_secs_f64(new_avg);

        // Calculate speedup factor based on estimated sequential time
        let estimated_sequential_time = job.estimated_duration;
        if estimated_sequential_time > Duration::ZERO {
            self.stats.speedup_factor = estimated_sequential_time.as_secs_f64() / duration.as_secs_f64();
            self.stats.time_saved += estimated_sequential_time.saturating_sub(duration);
        // Update other metrics
        self.stats.active_workers = self.load_balancer.active_worker_count().await?;
        self.stats.load_balance_efficiency = self.load_balancer.get_efficiency().await?;
        self.stats.network_overhead = self.network_optimizer.get_overhead().await?;

        Ok(())
    /// Generate performance report
    pub fn generate_performance_report(&self) -> String {
        format!(
            r#"Distributed Compilation Performance Report
========================================

Total Jobs: {}
Completed: {} ({:.1}%)
Failed: {} ({:.1}%)

Performance Metrics:
- Average Duration: {:.2}s
- Speedup Factor: {:.1}x
- Time Saved: {:.1}s
- Cache Hit Rate: {:.1}%

Resource Utilization:
- Active Workers: {}
- Load Balance Efficiency: {:.1}%
- Network Overhead: {:.2}s

Reliability:
- Fault Recoveries: {}
- Success Rate: {:.1}%
            if self.stats.total_jobs > 0 { 
                self.stats.completed_jobs as f64 / self.stats.total_jobs as f64 * 100.0 
            if self.stats.total_jobs > 0 { 
                self.stats.failed_jobs as f64 / self.stats.total_jobs as f64 * 100.0 
            if self.stats.total_jobs > 0 { 
                self.stats.completed_jobs as f64 / self.stats.total_jobs as f64 * 100.0 
        )
    }
}

