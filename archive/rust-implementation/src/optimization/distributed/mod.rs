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
    pub coordinator_port: u16,
    pub max_worker_nodes: usize,
    pub chunk_size: usize,
    pub network_timeout: Duration,
    pub fault_tolerance_enabled: bool,
    pub caching_enabled: bool,
    pub load_balancing: load_balancer::LoadBalancingStrategy,
    pub network_config: network_optimizer::NetworkConfig,
    pub performance_monitoring_enabled: bool,
}

impl Default for DistributedConfig {
    fn default() -> Self {
        Self {
            coordinator_port: 8080,
            max_worker_nodes: 10,
            chunk_size: 1024,
            network_timeout: Duration::from_secs(30),
            fault_tolerance_enabled: true,
            caching_enabled: true,
            load_balancing: load_balancer::LoadBalancingStrategy::LeastLoaded,
            network_config: network_optimizer::NetworkConfig::default(),
            performance_monitoring_enabled: true,
        }
    }
}

/// Statistics for distributed compilation performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedStats {
    pub total_jobs: u64,
    pub completed_jobs: u64,
    pub failed_jobs: u64,
    pub average_duration: Duration,
    pub time_saved: Duration,
    pub speedup_factor: f64,
    pub network_overhead: Duration,
    pub cache_hit_rate: f64,
    pub active_workers: u32,
    pub load_balance_efficiency: f64,
    pub fault_recoveries: u64,
}

impl Default for DistributedStats {
    fn default() -> Self {
        Self {
            total_jobs: 0,
            completed_jobs: 0,
            failed_jobs: 0,
            average_duration: Duration::from_secs(0),
            time_saved: Duration::from_secs(0),
            speedup_factor: 1.0,
            network_overhead: Duration::from_secs(0),
            cache_hit_rate: 0.0,
            active_workers: 0,
            load_balance_efficiency: 0.0,
            fault_recoveries: 0,
        }
    }
}

/// Main distributed compilation system coordinator
#[derive(Debug)]
pub struct DistributedCompilationSystem {
    config: DistributedConfig,
    compiler: DistributedCompiler,
    load_balancer: LoadBalancer,
    network_optimizer: NetworkOptimizer,
    cache: CompilationCache,
    stats: DistributedStats,
}

impl DistributedCompilationSystem {
    /// Create a new distributed compilation system
    pub fn new(config: DistributedConfig) -> Result<Self> {
        let compiler = distributed_compiler::DistributedCompiler::new(
            distributed_compiler::CompilerConfig::default()
        )?;

        let load_balancer = load_balancer::LoadBalancer::new(
            config.load_balancing.clone()
        )?;

        let network_optimizer = network_optimizer::NetworkOptimizer::new(
            config.network_config.clone()
        )?;

        let cache = if config.caching_enabled {
            compilation_cache::CompilationCache::new(
                compilation_cache::CacheStrategy::LruWithTtl {
                    max_entries: 1000,
                    ttl: Duration::from_secs(24 * 60 * 60),
                }
            )?
        } else {
            compilation_cache::CompilationCache::disabled()
        };

        Ok(Self {
            config: config.clone(),
            compiler,
            load_balancer,
            network_optimizer,
            cache,
            stats: DistributedStats::default(),
        })
    }

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
        }

        tracing::info!("Distributed compilation system started");

        Ok(())
    }

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
    }

    /// Submit a compilation job for distributed processing
    pub async fn compile_distributed(&mut self, job: CompilationJob) -> Result<Vec<u8>> {
        let start_time = std::time::Instant::now();

        // Check cache first
        if let Some(cached_result) = self.cache.get(&job.cache_key()).await? {
            self.stats.cache_hit_rate = 
                (self.stats.cache_hit_rate * self.stats.total_jobs as f64 + 1.0) 
                / (self.stats.total_jobs + 1) as f64;
            tracing::debug!("Cache hit for job {}", job.id);
            return Ok(cached_result.output);
        }

        // Submit job to distributed compiler
        let result = self.compiler.submit_job(job.clone()).await?;

        // Cache successful results
        if result.success {
            let cache_entry = compilation_cache::CacheEntry::new(
                job.cache_key(),
                result.output.clone(),
                compilation_cache::CacheMetadata {
                    source_files: job.source_files.iter().map(|s| std::path::PathBuf::from(s)).collect(),
                    optimization_level: job.optimization_level,
                    target_platform: job.target_platform.clone(),
                    compiler_version: env!("CARGO_PKG_VERSION").to_string(),
                    compilation_time: result.compilation_time,
                    worker_id: result.worker_id.clone(),
                }
            );
            self.cache.put(cache_entry).await?;
        }

        // Update statistics
        let duration = start_time.elapsed();
        self.update_stats(&job, &result, duration).await?;

        if result.success {
            Ok(result.output)
        } else {
            Err(CursedError::system_error(&format!(
                "Compilation failed: {}", 
                result.error_message.unwrap_or_else(|| "Unknown error".to_string())
            )))
        }
    }

    /// Get current system statistics
    pub fn get_statistics(&self) -> &DistributedStats {
        &self.stats
    }

    /// Get active worker nodes
    pub async fn get_workers(&self) -> Result<Vec<worker_node::WorkerNode>> {
        self.load_balancer.get_active_workers().await
    }

    /// Add a new worker node
    pub async fn add_worker(&mut self, worker: worker_node::WorkerNode) -> Result<()> {
        self.load_balancer.add_worker(worker).await
    }

    /// Remove a worker node
    pub async fn remove_worker(&mut self, worker_id: &str) -> Result<()> {
        self.load_balancer.remove_worker(worker_id).await
    }

    /// Update system configuration
    pub async fn update_config(&mut self, new_config: DistributedConfig) -> Result<()> {
        tracing::info!("Updating distributed compilation configuration");
        
        // Update load balancer configuration
        if new_config.load_balancing != self.config.load_balancing {
            self.load_balancer.update_strategy(new_config.load_balancing.clone()).await?;
        }

        // Update network optimization
        if new_config.network_config != self.config.network_config {
            self.network_optimizer.update_config(new_config.network_config.clone()).await?;
        }

        // Update compiler configuration
        let compiler_config = distributed_compiler::CompilerConfig::default();
        self.compiler.update_config(compiler_config).await?;

        self.config = new_config;
        Ok(())
    }

    /// Update system statistics
    async fn update_stats(
        &mut self,
        job: &CompilationJob,
        result: &distributed_compiler::CompilationResult,
        duration: Duration,
    ) -> Result<()> {
        self.stats.total_jobs += 1;
        
        if result.success {
            self.stats.completed_jobs += 1;
        } else {
            self.stats.failed_jobs += 1;
        }

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
        }

        // Update other metrics
        self.stats.active_workers = self.load_balancer.active_worker_count().await? as u32;
        self.stats.load_balance_efficiency = self.load_balancer.get_efficiency().await?;
        self.stats.network_overhead = self.network_optimizer.get_overhead().await?;

        Ok(())
    }

    /// Generate performance report
    pub fn generate_performance_report(&self) -> String {
        let completed_rate = if self.stats.total_jobs > 0 { 
            self.stats.completed_jobs as f64 / self.stats.total_jobs as f64 * 100.0 
        } else { 
            0.0 
        };

        let failed_rate = if self.stats.total_jobs > 0 { 
            self.stats.failed_jobs as f64 / self.stats.total_jobs as f64 * 100.0 
        } else { 
            0.0 
        };

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
- Success Rate: {:.1}%"#,
            self.stats.total_jobs,
            self.stats.completed_jobs,
            completed_rate,
            self.stats.failed_jobs,
            failed_rate,
            self.stats.average_duration.as_secs_f64(),
            self.stats.speedup_factor,
            self.stats.time_saved.as_secs_f64(),
            self.stats.cache_hit_rate * 100.0,
            self.stats.active_workers,
            self.stats.load_balance_efficiency * 100.0,
            self.stats.network_overhead.as_secs_f64(),
            self.stats.fault_recoveries,
            completed_rate
        )
    }
}
