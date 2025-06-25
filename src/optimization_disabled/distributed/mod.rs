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
    pub coordinator_port: u16,
    /// Maximum number of worker nodes
    pub max_workers: usize,
    /// Task distribution chunk size
    pub chunk_size: usize,
    /// Network timeout for communications
    pub network_timeout: Duration,
    /// Enable fault tolerance features
    pub fault_tolerance: bool,
    /// Enable compilation caching
    pub caching_enabled: bool,
    /// Load balancing strategy
    pub load_balancing: LoadBalancingStrategy,
    /// Network optimization settings
    pub network_config: NetworkConfig,
    /// Enable performance monitoring
    pub monitoring_enabled: bool,
}

impl Default for DistributedConfig {
    fn default() -> Self {
        Self {
            coordinator_port: 9000,
            max_workers: 16,
            chunk_size: 4,
            network_timeout: Duration::from_secs(30),
            fault_tolerance: true,
            caching_enabled: true,
            load_balancing: LoadBalancingStrategy::AdaptiveWeighted,
            network_config: NetworkConfig::default(),
            monitoring_enabled: true,
        }
    }
}

/// Statistics for distributed compilation performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedStats {
    /// Total compilation jobs processed
    pub total_jobs: usize,
    /// Successfully completed jobs
    pub completed_jobs: usize,
    /// Failed jobs
    pub failed_jobs: usize,
    /// Average job duration
    pub average_duration: Duration,
    /// Total time saved vs sequential compilation
    pub time_saved: Duration,
    /// Speedup factor achieved
    pub speedup_factor: f64,
    /// Network overhead
    pub network_overhead: Duration,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Number of active workers
    pub active_workers: usize,
    /// Load balancing efficiency
    pub load_balance_efficiency: f64,
    /// Fault recovery operations
    pub fault_recoveries: usize,
}

impl Default for DistributedStats {
    fn default() -> Self {
        Self {
            total_jobs: 0,
            completed_jobs: 0,
            failed_jobs: 0,
            average_duration: Duration::ZERO,
            time_saved: Duration::ZERO,
            speedup_factor: 1.0,
            network_overhead: Duration::ZERO,
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
    compiler: distributed_compiler::DistributedCompiler,
    load_balancer: load_balancer::LoadBalancer,
    network_optimizer: network_optimizer::NetworkOptimizer,
    cache: compilation_cache::CompilationCache,
    stats: DistributedStats,
}

impl DistributedCompilationSystem {
    /// Create a new distributed compilation system
    pub fn new(config: DistributedConfig) -> Result<Self> {
        let compiler = distributed_compiler::DistributedCompiler::new(
            distributed_compiler::CompilerConfig {
                max_concurrent_jobs: config.max_workers * 2,
                job_timeout: config.network_timeout,
                retry_attempts: if config.fault_tolerance { 3 } else { 1 },
                chunk_size: config.chunk_size,
                enable_monitoring: config.monitoring_enabled,
            }
        )?;

        let load_balancer = load_balancer::LoadBalancer::new(
            config.load_balancing.clone(),
            config.max_workers,
        )?;

        let network_optimizer = network_optimizer::NetworkOptimizer::new(
            config.network_config.clone()
        )?;

        let cache = if config.caching_enabled {
            compilation_cache::CompilationCache::new(
                compilation_cache::CacheStrategy::LruWithTtl {
                    max_entries: 10000,
                    ttl: Duration::from_hours(24),
                }
            )?
        } else {
            compilation_cache::CompilationCache::disabled()
        };

        Ok(Self {
            config,
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

        tracing::info!(
            workers = self.config.max_workers,
            port = self.config.coordinator_port,
            "Distributed compilation system started"
        );

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
        use tracing::instrument;

        #[instrument(skip(self, job))]
        async fn inner_compile(
            system: &mut DistributedCompilationSystem,
            job: CompilationJob,
        ) -> Result<Vec<u8>> {
            let start_time = std::time::Instant::now();

            // Check cache first
            if let Some(cached_result) = system.cache.get(&job.cache_key()).await? {
                system.stats.cache_hit_rate = 
                    (system.stats.cache_hit_rate * system.stats.total_jobs as f64 + 1.0) 
                    / (system.stats.total_jobs + 1) as f64;
                tracing::debug!("Cache hit for job {}", job.id);
                return Ok(cached_result.output);
            }

            // Submit job to distributed compiler
            let result = system.compiler.submit_job(job.clone()).await?;

            // Cache successful results
            if result.success {
                system.cache.put(
                    job.cache_key(),
                    compilation_cache::CacheEntry {
                        job_id: job.id.clone(),
                        output: result.output.clone(),
                        created_at: std::time::SystemTime::now(),
                        access_count: 1,
                    }
                ).await?;
            }

            // Update statistics
            let duration = start_time.elapsed();
            system.update_stats(&job, &result, duration).await?;

            if result.success {
                Ok(result.output)
            } else {
                Err(crate::error::CursedError::system_error(&format!(
                    "Compilation failed: {}", 
                    result.error_message.unwrap_or_else(|| "Unknown error".to_string())
                )))
            }
        }

        inner_compile(self, job).await
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
        let compiler_config = distributed_compiler::CompilerConfig {
            max_concurrent_jobs: new_config.max_workers * 2,
            job_timeout: new_config.network_timeout,
            retry_attempts: if new_config.fault_tolerance { 3 } else { 1 },
            chunk_size: new_config.chunk_size,
            enable_monitoring: new_config.monitoring_enabled,
        };
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
        self.stats.active_workers = self.load_balancer.active_worker_count().await?;
        self.stats.load_balance_efficiency = self.load_balancer.get_efficiency().await?;
        self.stats.network_overhead = self.network_optimizer.get_overhead().await?;

        Ok(())
    }

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
"#,
            self.stats.total_jobs,
            self.stats.completed_jobs,
            if self.stats.total_jobs > 0 { 
                self.stats.completed_jobs as f64 / self.stats.total_jobs as f64 * 100.0 
            } else { 0.0 },
            self.stats.failed_jobs,
            if self.stats.total_jobs > 0 { 
                self.stats.failed_jobs as f64 / self.stats.total_jobs as f64 * 100.0 
            } else { 0.0 },
            self.stats.average_duration.as_secs_f64(),
            self.stats.speedup_factor,
            self.stats.time_saved.as_secs_f64(),
            self.stats.cache_hit_rate * 100.0,
            self.stats.active_workers,
            self.stats.load_balance_efficiency * 100.0,
            self.stats.network_overhead.as_secs_f64(),
            self.stats.fault_recoveries,
            if self.stats.total_jobs > 0 { 
                self.stats.completed_jobs as f64 / self.stats.total_jobs as f64 * 100.0 
            } else { 0.0 },
        )
    }
}

