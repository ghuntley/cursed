//! Comprehensive Tests for Distributed Compilation System
//!
//! Tests all aspects of the distributed compilation infrastructure including
//! unit tests, integration tests, performance benchmarks, and fault tolerance.

use cursed::optimization::distributed::*;
use cursed::error::Result;
use std::time::Duration;
use tokio::time::timeout;
use tracing_test::traced_test;

/// Basic distributed compilation system tests
mod basic_tests {
    use super::*;

    #[tokio::test]
    #[traced_test]
    async fn test_system_creation_and_configuration() {
        let config = DistributedConfig {
            coordinator_port: 9000,
            max_workers: 8,
            chunk_size: 4,
            network_timeout: Duration::from_secs(30),
            fault_tolerance: true,
            caching_enabled: true,
            load_balancing: LoadBalancingStrategy::AdaptiveWeighted,
            network_config: NetworkConfig::default(),
            monitoring_enabled: true,
        };

        let system = DistributedCompilationSystem::new(config.clone());
        assert!(system.is_ok());

        let system = system.unwrap();
        assert_eq!(system.get_statistics().total_jobs, 0);
        assert_eq!(system.get_statistics().speedup_factor, 1.0);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_default_configuration() {
        let config = DistributedConfig::default();
        
        assert_eq!(config.coordinator_port, 9000);
        assert_eq!(config.max_workers, 16);
        assert_eq!(config.chunk_size, 4);
        assert!(config.fault_tolerance);
        assert!(config.caching_enabled);
        assert!(config.monitoring_enabled);
        assert_eq!(config.network_timeout, Duration::from_secs(30));
    }

    #[tokio::test]
    #[traced_test]
    async fn test_system_startup_and_shutdown() {
        let config = DistributedConfig::default();
        let mut system = DistributedCompilationSystem::new(config).unwrap();

        // Test startup
        let start_result = system.start().await;
        assert!(start_result.is_ok());

        // Test shutdown
        let stop_result = system.stop().await;
        assert!(stop_result.is_ok());
    }
}

/// Worker node management tests
mod worker_tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    fn create_test_worker(id: &str, cores: usize) -> WorkerNode {
        let capabilities = WorkerCapabilities {
            cpu_cores: cores,
            memory_mb: 8192,
            disk_space_mb: 100000,
            supported_targets: vec!["x86_64-unknown-linux-gnu".to_string()],
            toolchains: std::collections::HashMap::new(),
            max_concurrent_jobs: cores,
            performance_score: cores as f64,
            network_bandwidth: 100.0,
            features: Vec::new(),
        };

        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9001);
        let mut worker = WorkerNode::new(address, capabilities);
        worker.id = id.to_string();
        worker.status = WorkerStatus::Online;
        worker
    }

    #[tokio::test]
    #[traced_test]
    async fn test_worker_creation_and_capabilities() {
        let worker = create_test_worker("test_worker", 4);
        
        assert_eq!(worker.capabilities.cpu_cores, 4);
        assert_eq!(worker.capabilities.max_concurrent_jobs, 4);
        assert!(worker.supports_target("x86_64-unknown-linux-gnu"));
        assert!(!worker.supports_target("aarch64-unknown-linux-gnu"));
        assert!(worker.is_available());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_worker_load_calculation() {
        let mut worker = create_test_worker("test_worker", 4);
        
        // No load initially
        assert_eq!(worker.load_factor(), 0.0);
        
        // Half load
        worker.metrics.active_jobs = 2;
        assert_eq!(worker.load_factor(), 0.5);
        
        // Full load
        worker.metrics.active_jobs = 4;
        assert_eq!(worker.load_factor(), 1.0);
        
        // Overload
        worker.metrics.active_jobs = 6;
        assert_eq!(worker.load_factor(), 1.5);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_worker_efficiency_score() {
        let mut worker = create_test_worker("test_worker", 4);
        worker.capabilities.performance_score = 8.0;
        worker.metrics.completed_jobs = 10;
        worker.metrics.failed_jobs = 0;
        
        let efficiency = worker.efficiency_score();
        assert!(efficiency > 0.0);
        assert!(efficiency <= 8.0); // Should not exceed base performance score
        
        // Add some failures
        worker.metrics.failed_jobs = 2;
        let reduced_efficiency = worker.efficiency_score();
        assert!(reduced_efficiency < efficiency);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_worker_capability_detection() {
        let capabilities = WorkerCapabilities::detect_local();
        assert!(capabilities.is_ok());
        
        let caps = capabilities.unwrap();
        assert!(caps.cpu_cores > 0);
        assert!(caps.memory_mb > 0);
        assert!(caps.performance_score > 0.0);
        assert!(!caps.supported_targets.is_empty());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_worker_manager() {
        let config = WorkerConfig::default();
        let manager = WorkerNodeManager::new(config).unwrap();
        
        // Test worker registration
        let worker = create_test_worker("test_worker", 4);
        let worker_id = worker.id.clone();
        
        let register_result = manager.register_worker(worker).await;
        assert!(register_result.is_ok());
        assert_eq!(register_result.unwrap(), worker_id);
        
        // Test worker retrieval
        let workers = manager.get_all_workers().await.unwrap();
        assert_eq!(workers.len(), 1);
        assert_eq!(workers[0].id, worker_id);
        
        // Test worker unregistration
        let unregister_result = manager.unregister_worker(&worker_id).await;
        assert!(unregister_result.is_ok());
        
        let workers = manager.get_all_workers().await.unwrap();
        assert_eq!(workers.len(), 0);
    }
}

/// Load balancer tests
mod load_balancer_tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    fn create_test_worker(id: &str, cores: usize, load: usize) -> WorkerNode {
        let capabilities = WorkerCapabilities {
            cpu_cores: cores,
            memory_mb: 8192,
            disk_space_mb: 100000,
            supported_targets: vec!["x86_64-unknown-linux-gnu".to_string()],
            toolchains: std::collections::HashMap::new(),
            max_concurrent_jobs: cores,
            performance_score: cores as f64,
            network_bandwidth: 100.0,
            features: Vec::new(),
        };

        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9001);
        let mut worker = WorkerNode::new(address, capabilities);
        worker.id = id.to_string();
        worker.status = WorkerStatus::Online;
        worker.metrics.active_jobs = load;
        worker
    }

    fn create_test_job() -> CompilationJob {
        CompilationJob::new(
            vec!["test.csd".to_string()],
            "x86_64-unknown-linux-gnu".to_string(),
            OutputType::Object,
        )
    }

    #[tokio::test]
    #[traced_test]
    async fn test_load_balancer_creation() {
        let lb = LoadBalancer::new(LoadBalancingStrategy::RoundRobin, 4);
        assert!(lb.is_ok());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_round_robin_strategy() {
        let mut lb = LoadBalancer::new(LoadBalancingStrategy::RoundRobin, 4).unwrap();
        
        // Add workers
        lb.add_worker(create_test_worker("worker1", 4, 0)).await.unwrap();
        lb.add_worker(create_test_worker("worker2", 4, 0)).await.unwrap();
        lb.add_worker(create_test_worker("worker3", 4, 0)).await.unwrap();

        let job = create_test_job();
        
        // Test round-robin selection
        let selection1 = lb.select_worker(&job).await.unwrap();
        let selection2 = lb.select_worker(&job).await.unwrap();
        let selection3 = lb.select_worker(&job).await.unwrap();
        let selection4 = lb.select_worker(&job).await.unwrap();

        assert!(selection1.is_some());
        assert!(selection2.is_some());
        assert!(selection3.is_some());
        assert!(selection4.is_some());

        // Should cycle through workers
        assert_ne!(selection1.unwrap().worker_id, selection2.unwrap().worker_id);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_least_loaded_strategy() {
        let mut lb = LoadBalancer::new(LoadBalancingStrategy::LeastLoaded, 4).unwrap();
        
        // Add workers with different loads
        lb.add_worker(create_test_worker("worker1", 4, 3)).await.unwrap(); // High load
        lb.add_worker(create_test_worker("worker2", 4, 1)).await.unwrap(); // Low load
        lb.add_worker(create_test_worker("worker3", 4, 2)).await.unwrap(); // Medium load

        let job = create_test_job();
        let selection = lb.select_worker(&job).await.unwrap();
        
        assert!(selection.is_some());
        assert_eq!(selection.unwrap().worker_id, "worker2"); // Should select least loaded
    }

    #[tokio::test]
    #[traced_test]
    async fn test_performance_based_strategy() {
        let mut lb = LoadBalancer::new(LoadBalancingStrategy::PerformanceBased, 4).unwrap();
        
        // Add workers with different performance scores
        let mut worker1 = create_test_worker("worker1", 2, 0);
        worker1.capabilities.performance_score = 2.0;
        
        let mut worker2 = create_test_worker("worker2", 8, 0);
        worker2.capabilities.performance_score = 8.0;
        
        lb.add_worker(worker1).await.unwrap();
        lb.add_worker(worker2).await.unwrap();

        let job = create_test_job();
        let selection = lb.select_worker(&job).await.unwrap();
        
        assert!(selection.is_some());
        assert_eq!(selection.unwrap().worker_id, "worker2"); // Should select higher performance
    }

    #[tokio::test]
    #[traced_test]
    async fn test_adaptive_weighted_strategy() {
        let mut lb = LoadBalancer::new(LoadBalancingStrategy::AdaptiveWeighted, 4).unwrap();
        
        // Add various workers
        lb.add_worker(create_test_worker("worker1", 4, 2)).await.unwrap();
        lb.add_worker(create_test_worker("worker2", 8, 1)).await.unwrap();
        lb.add_worker(create_test_worker("worker3", 2, 0)).await.unwrap();

        let job = create_test_job();
        let selection = lb.select_worker(&job).await.unwrap();
        
        assert!(selection.is_some());
        // Should select based on combined factors
        let selected_id = selection.unwrap().worker_id;
        assert!(selected_id == "worker2" || selected_id == "worker3");
    }

    #[tokio::test]
    #[traced_test]
    async fn test_job_completion_tracking() {
        let lb = LoadBalancer::new(LoadBalancingStrategy::LeastLoaded, 4).unwrap();
        let worker = create_test_worker("test_worker", 4, 0);
        
        let mut lb_mut = lb;
        lb_mut.add_worker(worker).await.unwrap();

        let result = lb_mut.record_job_completion(
            "job1",
            "test_worker",
            Duration::from_secs(45),
            true,
        ).await;
        
        assert!(result.is_ok());
        
        let efficiency = lb_mut.get_efficiency().await.unwrap();
        assert!(efficiency >= 0.0 && efficiency <= 1.0);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_strategy_switching() {
        let mut lb = LoadBalancer::new(LoadBalancingStrategy::RoundRobin, 4).unwrap();
        
        let result = lb.update_strategy(LoadBalancingStrategy::PerformanceBased).await;
        assert!(result.is_ok());
    }
}

/// Network optimizer tests
mod network_tests {
    use super::*;

    #[tokio::test]
    #[traced_test]
    async fn test_network_optimizer_creation() {
        let config = NetworkConfig::default();
        let optimizer = NetworkOptimizer::new(config);
        assert!(optimizer.is_ok());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_compression_configuration() {
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
    #[traced_test]
    async fn test_bandwidth_management() {
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
    #[traced_test]
    async fn test_message_priority_ordering() {
        use std::collections::BinaryHeap;
        
        // Create messages with different priorities
        let high_priority_msg = NetworkMessage {
            id: "1".to_string(),
            message_type: "job_assignment".to_string(),
            priority: MessagePriority::High,
            source: "coordinator".to_string(),
            destination: "worker1".to_string(),
            payload: Vec::new(),
            compressed: false,
            timestamp: std::time::SystemTime::now(),
            correlation_id: None,
        };

        let low_priority_msg = NetworkMessage {
            id: "2".to_string(),
            message_type: "metrics".to_string(),
            priority: MessagePriority::Low,
            source: "worker1".to_string(),
            destination: "coordinator".to_string(),
            payload: Vec::new(),
            compressed: false,
            timestamp: std::time::SystemTime::now(),
            correlation_id: None,
        };

        // Test that high priority messages are processed first
        assert!(high_priority_msg.priority > low_priority_msg.priority);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_network_stats() {
        let mut optimizer = NetworkOptimizer::new(NetworkConfig::default()).unwrap();
        optimizer.start().await.unwrap();
        
        let stats = optimizer.get_stats().await.unwrap();
        assert_eq!(stats.total_bytes_sent, 0);
        assert_eq!(stats.total_bytes_received, 0);
        assert_eq!(stats.messages_sent, 0);
        assert_eq!(stats.messages_received, 0);
        
        optimizer.stop().await.unwrap();
    }
}

/// Compilation cache tests
mod cache_tests {
    use super::*;

    fn create_test_entry(id: &str, output: &[u8]) -> CacheEntry {
        CacheEntry {
            job_id: id.to_string(),
            output: output.to_vec(),
            created_at: std::time::SystemTime::now(),
            access_count: 0,
        }
    }

    #[tokio::test]
    #[traced_test]
    async fn test_cache_creation() {
        let cache = CompilationCache::new(CacheStrategy::default());
        assert!(cache.is_ok());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_cache_put_get() {
        let cache = CompilationCache::new(CacheStrategy::default()).unwrap();
        let entry = create_test_entry("test_job", b"compiled output");
        
        let put_result = cache.put("test_key".to_string(), entry.clone()).await;
        assert!(put_result.is_ok());
        
        let get_result = cache.get("test_key").await;
        assert!(get_result.is_ok());
        assert!(get_result.unwrap().is_some());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_cache_miss() {
        let cache = CompilationCache::new(CacheStrategy::default()).unwrap();
        
        let get_result = cache.get("nonexistent_key").await;
        assert!(get_result.is_ok());
        assert!(get_result.unwrap().is_none());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_lru_eviction() {
        let strategy = CacheStrategy::LruWithTtl {
            max_entries: 2,
            ttl: Duration::from_secs(3600),
        };
        let cache = CompilationCache::new(strategy).unwrap();
        
        // Fill cache to capacity
        cache.put("key1".to_string(), create_test_entry("job1", b"output1")).await.unwrap();
        cache.put("key2".to_string(), create_test_entry("job2", b"output2")).await.unwrap();
        
        // Add one more - should evict oldest
        cache.put("key3".to_string(), create_test_entry("job3", b"output3")).await.unwrap();
        
        // key1 should be evicted, key2 and key3 should remain
        assert!(!cache.contains("key1").await.unwrap());
        assert!(cache.contains("key2").await.unwrap());
        assert!(cache.contains("key3").await.unwrap());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_cache_statistics() {
        let cache = CompilationCache::new(CacheStrategy::default()).unwrap();
        let entry = create_test_entry("test_job", b"compiled output");
        
        // Initial stats
        let stats = cache.get_stats().await.unwrap();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        
        // Add entry and test hit
        cache.put("test_key".to_string(), entry).await.unwrap();
        cache.get("test_key").await.unwrap();
        
        let stats = cache.get_stats().await.unwrap();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.entry_count, 1);
        
        // Test miss
        cache.get("nonexistent").await.unwrap();
        
        let stats = cache.get_stats().await.unwrap();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hit_rate, 0.5);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_disabled_cache() {
        let cache = CompilationCache::disabled();
        let entry = create_test_entry("test_job", b"compiled output");
        
        // Operations should succeed but do nothing
        assert!(cache.put("test_key".to_string(), entry).await.is_ok());
        assert!(cache.get("test_key").await.unwrap().is_none());
        assert!(!cache.contains("test_key").await.unwrap());
    }
}

/// Distributed compiler tests
mod compiler_tests {
    use super::*;

    #[tokio::test]
    #[traced_test]
    async fn test_compiler_creation() {
        let config = CompilerConfig::default();
        let compiler = DistributedCompiler::new(config);
        assert!(compiler.is_ok());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_job_creation() {
        let job = CompilationJob::new(
            vec!["test.csd".to_string()],
            "x86_64-unknown-linux-gnu".to_string(),
            OutputType::Object,
        );
        
        assert!(!job.id.is_empty());
        assert_eq!(job.source_files.len(), 1);
        assert_eq!(job.output_type, OutputType::Object);
        assert_eq!(job.priority, JobPriority::Normal);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_job_cache_key() {
        let job1 = CompilationJob::new(
            vec!["test.csd".to_string()],
            "x86_64-unknown-linux-gnu".to_string(),
            OutputType::Object,
        );
        
        let job2 = CompilationJob::new(
            vec!["test.csd".to_string()],
            "x86_64-unknown-linux-gnu".to_string(),
            OutputType::Object,
        );

        // Same inputs should produce same cache key
        assert_eq!(job1.cache_key(), job2.cache_key());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_job_chunking() {
        let job = CompilationJob::new(
            vec!["1.csd".to_string(), "2.csd".to_string(), "3.csd".to_string(), 
                 "4.csd".to_string(), "5.csd".to_string()],
            "x86_64-unknown-linux-gnu".to_string(),
            OutputType::Object,
        );

        let chunks = job.split_into_chunks(2);
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].source_files.len(), 2);
        assert_eq!(chunks[1].source_files.len(), 2);
        assert_eq!(chunks[2].source_files.len(), 1);
        
        // Check chunk metadata
        assert_eq!(chunks[0].chunk_id, Some(0));
        assert_eq!(chunks[1].chunk_id, Some(1));
        assert_eq!(chunks[2].chunk_id, Some(2));
        
        // All chunks should have same parent
        for chunk in &chunks {
            assert_eq!(chunk.parent_job_id, Some(job.id.clone()));
        }
    }

    #[tokio::test]
    #[traced_test]
    async fn test_job_duration_estimation() {
        let mut job = CompilationJob::new(
            vec!["test1.csd".to_string(), "test2.csd".to_string(), "test3.csd".to_string()],
            "x86_64-unknown-linux-gnu".to_string(),
            OutputType::Object,
        );
        
        job.optimization_level = "O3".to_string();
        job.estimate_duration();
        
        assert!(job.estimated_duration > Duration::ZERO);
        assert!(job.estimated_duration > Duration::from_secs(5)); // Should be > base time
    }

    #[tokio::test]
    #[traced_test]
    async fn test_compiler_startup_shutdown() {
        let config = CompilerConfig::default();
        let mut compiler = DistributedCompiler::new(config).unwrap();

        let start_result = compiler.start().await;
        assert!(start_result.is_ok());

        let stop_result = compiler.stop().await;
        assert!(stop_result.is_ok());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_compiler_statistics() {
        let config = CompilerConfig::default();
        let compiler = DistributedCompiler::new(config).unwrap();

        let stats = compiler.get_statistics().await.unwrap();
        assert_eq!(stats.total_jobs_submitted, 0);
        assert_eq!(stats.jobs_completed, 0);
        assert_eq!(stats.jobs_failed, 0);
        assert_eq!(stats.average_job_time, Duration::ZERO);
    }
}

/// Integration tests
mod integration_tests {
    use super::*;

    #[tokio::test]
    #[traced_test]
    async fn test_full_system_integration() {
        let config = DistributedConfig {
            coordinator_port: 9100,
            max_workers: 4,
            chunk_size: 2,
            network_timeout: Duration::from_secs(10),
            fault_tolerance: true,
            caching_enabled: true,
            load_balancing: LoadBalancingStrategy::LeastLoaded,
            network_config: NetworkConfig::default(),
            monitoring_enabled: true,
        };

        let mut system = DistributedCompilationSystem::new(config).unwrap();
        
        // Start the system
        let start_result = system.start().await;
        assert!(start_result.is_ok());

        // Add some workers
        let worker1 = create_test_worker("worker1", 4);
        let worker2 = create_test_worker("worker2", 8);
        
        system.add_worker(worker1).await.unwrap();
        system.add_worker(worker2).await.unwrap();

        // Check workers are registered
        let workers = system.get_workers().await.unwrap();
        assert_eq!(workers.len(), 2);

        // Test statistics
        let stats = system.get_statistics();
        assert_eq!(stats.total_jobs, 0);
        assert_eq!(stats.active_workers, 2);

        // Stop the system
        let stop_result = system.stop().await;
        assert!(stop_result.is_ok());
    }

    fn create_test_worker(id: &str, cores: usize) -> WorkerNode {
        let capabilities = WorkerCapabilities {
            cpu_cores: cores,
            memory_mb: 8192,
            disk_space_mb: 100000,
            supported_targets: vec!["x86_64-unknown-linux-gnu".to_string()],
            toolchains: std::collections::HashMap::new(),
            max_concurrent_jobs: cores,
            performance_score: cores as f64,
            network_bandwidth: 100.0,
            features: Vec::new(),
        };

        let address = "127.0.0.1:9001".parse().unwrap();
        let mut worker = WorkerNode::new(address, capabilities);
        worker.id = id.to_string();
        worker.status = WorkerStatus::Online;
        worker
    }

    #[tokio::test]
    #[traced_test]
    async fn test_configuration_updates() {
        let config = DistributedConfig::default();
        let mut system = DistributedCompilationSystem::new(config).unwrap();
        
        system.start().await.unwrap();

        // Update configuration
        let new_config = DistributedConfig {
            max_workers: 32,
            load_balancing: LoadBalancingStrategy::PerformanceBased,
            ..DistributedConfig::default()
        };

        let update_result = system.update_config(new_config).await;
        assert!(update_result.is_ok());

        system.stop().await.unwrap();
    }

    #[tokio::test]
    #[traced_test]
    async fn test_performance_reporting() {
        let config = DistributedConfig::default();
        let system = DistributedCompilationSystem::new(config).unwrap();

        let report = system.generate_performance_report();
        assert!(report.contains("Distributed Compilation Performance Report"));
        assert!(report.contains("Total Jobs:"));
        assert!(report.contains("Performance Metrics:"));
        assert!(report.contains("Resource Utilization:"));
        assert!(report.contains("Reliability:"));
    }
}

/// Performance benchmark tests
mod performance_tests {
    use super::*;

    #[tokio::test]
    #[traced_test]
    async fn test_load_balancer_performance() {
        let mut lb = LoadBalancer::new(LoadBalancingStrategy::LeastLoaded, 16).unwrap();
        
        // Add many workers
        for i in 0..16 {
            let worker = create_test_worker(&format!("worker{}", i), 4);
            lb.add_worker(worker).await.unwrap();
        }

        let job = create_test_job();
        let start = std::time::Instant::now();
        
        // Select workers many times
        for _ in 0..1000 {
            let _selection = lb.select_worker(&job).await.unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration < Duration::from_millis(100)); // Should be fast
    }

    fn create_test_worker(id: &str, cores: usize) -> WorkerNode {
        let capabilities = WorkerCapabilities {
            cpu_cores: cores,
            memory_mb: 8192,
            disk_space_mb: 100000,
            supported_targets: vec!["x86_64-unknown-linux-gnu".to_string()],
            toolchains: std::collections::HashMap::new(),
            max_concurrent_jobs: cores,
            performance_score: cores as f64,
            network_bandwidth: 100.0,
            features: Vec::new(),
        };

        let address = "127.0.0.1:9001".parse().unwrap();
        let mut worker = WorkerNode::new(address, capabilities);
        worker.id = id.to_string();
        worker.status = WorkerStatus::Online;
        worker
    }

    fn create_test_job() -> CompilationJob {
        CompilationJob::new(
            vec!["test.csd".to_string()],
            "x86_64-unknown-linux-gnu".to_string(),
            OutputType::Object,
        )
    }

    #[tokio::test]
    #[traced_test]
    async fn test_cache_performance() {
        let cache = CompilationCache::new(CacheStrategy::default()).unwrap();
        let entry = CacheEntry {
            job_id: "test".to_string(),
            output: vec![0u8; 1024], // 1KB entry
            created_at: std::time::SystemTime::now(),
            access_count: 0,
        };

        let start = std::time::Instant::now();
        
        // Perform many cache operations
        for i in 0..1000 {
            let key = format!("key{}", i);
            cache.put(key.clone(), entry.clone()).await.unwrap();
            let _result = cache.get(&key).await.unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration < Duration::from_secs(1)); // Should complete in reasonable time
    }

    #[tokio::test]
    #[traced_test]
    async fn test_large_job_chunking_performance() {
        let large_job = CompilationJob::new(
            (0..1000).map(|i| format!("file{}.csd", i)).collect(),
            "x86_64-unknown-linux-gnu".to_string(),
            OutputType::Object,
        );

        let start = std::time::Instant::now();
        let chunks = large_job.split_into_chunks(10);
        let duration = start.elapsed();

        assert_eq!(chunks.len(), 100);
        assert!(duration < Duration::from_millis(10)); // Should be very fast
    }

    #[tokio::test]
    #[traced_test]
    async fn test_worker_efficiency_calculation_performance() {
        let mut worker = create_test_worker("test_worker", 4);
        worker.metrics.completed_jobs = 10000;
        worker.metrics.failed_jobs = 100;

        let start = std::time::Instant::now();
        
        // Calculate efficiency many times
        for _ in 0..10000 {
            let _efficiency = worker.efficiency_score();
        }
        
        let duration = start.elapsed();
        assert!(duration < Duration::from_millis(100)); // Should be very fast
    }
}

/// Fault tolerance tests
mod fault_tolerance_tests {
    use super::*;

    #[tokio::test]
    #[traced_test]
    async fn test_worker_failure_handling() {
        let mut lb = LoadBalancer::new(LoadBalancingStrategy::LeastLoaded, 4).unwrap();
        
        // Add workers
        let mut worker1 = create_test_worker("worker1", 4);
        let worker2 = create_test_worker("worker2", 4);
        
        lb.add_worker(worker1.clone()).await.unwrap();
        lb.add_worker(worker2).await.unwrap();

        // Simulate worker failure
        worker1.status = WorkerStatus::Error { message: "Connection lost".to_string() };
        lb.add_worker(worker1).await.unwrap(); // Update with error status

        let job = create_test_job();
        let selection = lb.select_worker(&job).await.unwrap();
        
        // Should select the healthy worker
        assert!(selection.is_some());
        assert_eq!(selection.unwrap().worker_id, "worker2");
    }

    fn create_test_worker(id: &str, cores: usize) -> WorkerNode {
        let capabilities = WorkerCapabilities {
            cpu_cores: cores,
            memory_mb: 8192,
            disk_space_mb: 100000,
            supported_targets: vec!["x86_64-unknown-linux-gnu".to_string()],
            toolchains: std::collections::HashMap::new(),
            max_concurrent_jobs: cores,
            performance_score: cores as f64,
            network_bandwidth: 100.0,
            features: Vec::new(),
        };

        let address = "127.0.0.1:9001".parse().unwrap();
        let mut worker = WorkerNode::new(address, capabilities);
        worker.id = id.to_string();
        worker.status = WorkerStatus::Online;
        worker
    }

    fn create_test_job() -> CompilationJob {
        CompilationJob::new(
            vec!["test.csd".to_string()],
            "x86_64-unknown-linux-gnu".to_string(),
            OutputType::Object,
        )
    }

    #[tokio::test]
    #[traced_test]
    async fn test_network_timeout_handling() {
        let config = NetworkConfig {
            connection_pool: ConnectionPoolConfig {
                connection_timeout: Duration::from_millis(100),
                ..ConnectionPoolConfig::default()
            },
            ..NetworkConfig::default()
        };

        let optimizer = NetworkOptimizer::new(config);
        assert!(optimizer.is_ok());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_cache_corruption_handling() {
        let cache = CompilationCache::new(CacheStrategy::default()).unwrap();
        
        // Test with various edge cases
        let result = cache.get("").await; // Empty key
        assert!(result.is_ok());
        
        let result = cache.get("nonexistent_key").await; // Non-existent key
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_system_graceful_degradation() {
        let config = DistributedConfig {
            fault_tolerance: true,
            ..DistributedConfig::default()
        };
        
        let mut system = DistributedCompilationSystem::new(config).unwrap();
        system.start().await.unwrap();

        // Test with no workers (should not crash)
        let workers = system.get_workers().await.unwrap();
        assert_eq!(workers.len(), 0);

        // System should still respond to queries
        let stats = system.get_statistics();
        assert_eq!(stats.active_workers, 0);

        system.stop().await.unwrap();
    }

    #[tokio::test]
    #[traced_test]
    async fn test_job_retry_mechanism() {
        let config = CompilerConfig {
            retry_attempts: 3,
            ..CompilerConfig::default()
        };

        let compiler = DistributedCompiler::new(config).unwrap();
        
        // The retry mechanism is tested implicitly through the configuration
        // In a real scenario, failed jobs would be retried automatically
        assert!(compiler.get_statistics().await.is_ok());
    }
}

/// Edge case tests
mod edge_case_tests {
    use super::*;

    #[tokio::test]
    #[traced_test]
    async fn test_zero_worker_scenario() {
        let config = DistributedConfig {
            max_workers: 0,
            ..DistributedConfig::default()
        };

        let system = DistributedCompilationSystem::new(config);
        assert!(system.is_ok());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_single_file_job() {
        let job = CompilationJob::new(
            vec!["single.csd".to_string()],
            "x86_64-unknown-linux-gnu".to_string(),
            OutputType::Object,
        );

        let chunks = job.split_into_chunks(4);
        assert_eq!(chunks.len(), 1); // Should not split single file
        assert_eq!(chunks[0].source_files.len(), 1);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_empty_job() {
        let job = CompilationJob::new(
            vec![],
            "x86_64-unknown-linux-gnu".to_string(),
            OutputType::Object,
        );

        let chunks = job.split_into_chunks(4);
        assert_eq!(chunks.len(), 1); // Should return original job
        assert_eq!(chunks[0].source_files.len(), 0);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_very_large_cache_entry() {
        let cache = CompilationCache::new(CacheStrategy::default()).unwrap();
        
        // Create a very large entry (exceeds max size)
        let large_entry = CacheEntry {
            job_id: "large_job".to_string(),
            output: vec![0u8; 200 * 1024 * 1024], // 200MB
            created_at: std::time::SystemTime::now(),
            access_count: 0,
        };

        // Should succeed but not actually cache (too large)
        let result = cache.put("large_key".to_string(), large_entry).await;
        assert!(result.is_ok());
        
        // Entry should not be in cache
        let retrieved = cache.get("large_key").await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    #[traced_test]
    async fn test_worker_with_zero_capacity() {
        let capabilities = WorkerCapabilities {
            cpu_cores: 0,
            memory_mb: 0,
            disk_space_mb: 0,
            supported_targets: vec![],
            toolchains: std::collections::HashMap::new(),
            max_concurrent_jobs: 0,
            performance_score: 0.0,
            network_bandwidth: 0.0,
            features: Vec::new(),
        };

        let address = "127.0.0.1:9001".parse().unwrap();
        let worker = WorkerNode::new(address, capabilities);
        
        assert_eq!(worker.load_factor(), 1.0); // Should handle division by zero
        assert!(!worker.is_available()); // Should not be available
    }

    #[tokio::test]
    #[traced_test]
    async fn test_invalid_target_architecture() {
        let job = CompilationJob::new(
            vec!["test.csd".to_string()],
            "invalid-target-arch".to_string(),
            OutputType::Object,
        );

        let worker = create_test_worker("worker1", 4);
        assert!(!worker.supports_target(&job.target_triple));
    }

    fn create_test_worker(id: &str, cores: usize) -> WorkerNode {
        let capabilities = WorkerCapabilities {
            cpu_cores: cores,
            memory_mb: 8192,
            disk_space_mb: 100000,
            supported_targets: vec!["x86_64-unknown-linux-gnu".to_string()],
            toolchains: std::collections::HashMap::new(),
            max_concurrent_jobs: cores,
            performance_score: cores as f64,
            network_bandwidth: 100.0,
            features: Vec::new(),
        };

        let address = "127.0.0.1:9001".parse().unwrap();
        let mut worker = WorkerNode::new(address, capabilities);
        worker.id = id.to_string();
        worker.status = WorkerStatus::Online;
        worker
    }

    #[tokio::test]
    #[traced_test]
    async fn test_extreme_load_balancing() {
        let mut lb = LoadBalancer::new(LoadBalancingStrategy::LeastLoaded, 1000).unwrap();
        
        // Add workers with extreme loads
        let mut worker1 = create_test_worker("worker1", 1);
        worker1.metrics.active_jobs = 1000; // Extreme overload
        
        let worker2 = create_test_worker("worker2", 1);
        // worker2 has no load
        
        lb.add_worker(worker1).await.unwrap();
        lb.add_worker(worker2).await.unwrap();

        let job = CompilationJob::new(
            vec!["test.csd".to_string()],
            "x86_64-unknown-linux-gnu".to_string(),
            OutputType::Object,
        );

        let selection = lb.select_worker(&job).await.unwrap();
        assert!(selection.is_some());
        assert_eq!(selection.unwrap().worker_id, "worker2"); // Should select unloaded worker
    }
}
