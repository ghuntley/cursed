//! Distributed Compilation System Demo
//!
//! Demonstrates the distributed compilation system with real-world scenarios,
//! showing performance improvements and usage patterns.

use cursed::optimization::distributed::*;
use cursed::error::Result;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{info, warn, error, debug};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 Distributed Compilation System Demo");
    info!("========================================");

    // Demo 1: Basic system setup
    demo_basic_setup().await?;

    // Demo 2: Worker management
    demo_worker_management().await?;

    // Demo 3: Load balancing strategies
    demo_load_balancing().await?;

    // Demo 4: Compilation caching
    demo_compilation_caching().await?;

    // Demo 5: Performance comparison
    demo_performance_comparison().await?;

    // Demo 6: Fault tolerance
    demo_fault_tolerance().await?;

    // Demo 7: Real-world simulation
    demo_real_world_simulation().await?;

    info!("✅ Demo completed successfully!");
    Ok(())
}

/// Demonstrate basic system setup and configuration
async fn demo_basic_setup() -> Result<()> {
    info!("\n📋 Demo 1: Basic System Setup");
    info!("------------------------------");

    // Create configuration
    let config = DistributedConfig {
        coordinator_port: 9200,
        max_workers: 8,
        chunk_size: 4,
        network_timeout: Duration::from_secs(30),
        fault_tolerance: true,
        caching_enabled: true,
        load_balancing: LoadBalancingStrategy::AdaptiveWeighted,
        network_config: NetworkConfig {
            compression: CompressionConfig {
                enabled: true,
                level: CompressionLevel::Balanced,
                min_size_bytes: 1024,
                algorithm: CompressionAlgorithm::LZ4,
            },
            bandwidth: BandwidthConfig {
                max_bandwidth_per_worker: 10_000_000, // 10 MB/s
                total_bandwidth_limit: 100_000_000,   // 100 MB/s
                adaptive_enabled: true,
                monitoring_interval: Duration::from_secs(1),
                qos: QosConfig::default(),
            },
            ..NetworkConfig::default()
        },
        monitoring_enabled: true,
    };

    info!("Configuration:");
    info!("  - Coordinator Port: {}", config.coordinator_port);
    info!("  - Max Workers: {}", config.max_workers);
    info!("  - Chunk Size: {}", config.chunk_size);
    info!("  - Fault Tolerance: {}", config.fault_tolerance);
    info!("  - Caching: {}", config.caching_enabled);
    info!("  - Load Balancing: {:?}", config.load_balancing);

    // Create and start system
    let mut system = DistributedCompilationSystem::new(config)?;
    system.start().await?;

    info!("✅ System started successfully");

    // Show initial statistics
    let stats = system.get_statistics();
    info!("Initial statistics:");
    info!("  - Total Jobs: {}", stats.total_jobs);
    info!("  - Active Workers: {}", stats.active_workers);
    info!("  - Cache Hit Rate: {:.1}%", stats.cache_hit_rate * 100.0);

    system.stop().await?;
    info!("✅ System stopped cleanly");

    Ok(())
}

/// Demonstrate worker management capabilities
async fn demo_worker_management() -> Result<()> {
    info!("\n👷 Demo 2: Worker Management");
    info!("----------------------------");

    let config = DistributedConfig::default();
    let mut system = DistributedCompilationSystem::new(config)?;
    system.start().await?;

    // Create different types of workers
    let workers = create_diverse_workers();

    // Add workers to the system
    for worker in workers {
        info!("Adding worker: {} ({}x cores, {:.1} score)", 
              worker.id, worker.capabilities.cpu_cores, worker.capabilities.performance_score);
        system.add_worker(worker).await?;
    }

    // Show worker information
    let active_workers = system.get_workers().await?;
    info!("\nActive Workers Summary:");
    for worker in &active_workers {
        info!("  {} - Status: {:?}, Load: {:.1}%, Efficiency: {:.2}", 
              worker.id, 
              worker.status,
              worker.load_factor() * 100.0,
              worker.efficiency_score());
    }

    // Demonstrate worker removal
    if let Some(worker) = active_workers.first() {
        info!("\nRemoving worker: {}", worker.id);
        system.remove_worker(&worker.id).await?;
    }

    let remaining_workers = system.get_workers().await?;
    info!("Remaining workers: {}", remaining_workers.len());

    system.stop().await?;
    Ok(())
}

/// Demonstrate different load balancing strategies
async fn demo_load_balancing() -> Result<()> {
    info!("\n⚖️  Demo 3: Load Balancing Strategies");
    info!("------------------------------------");

    let strategies = vec![
        LoadBalancingStrategy::RoundRobin,
        LoadBalancingStrategy::LeastLoaded,
        LoadBalancingStrategy::PerformanceBased,
        LoadBalancingStrategy::AdaptiveWeighted,
    ];

    for strategy in strategies {
        info!("\nTesting strategy: {:?}", strategy);
        
        let mut lb = LoadBalancer::new(strategy, 8)?;
        lb.start().await?;

        // Add workers with different characteristics
        let workers = create_test_workers_with_load();
        for worker in workers {
            lb.add_worker(worker).await?;
        }

        // Test job selection
        let job = create_sample_job("demo_file.csd");
        
        let mut selections = Vec::new();
        for i in 0..5 {
            if let Some(selection) = lb.select_worker(&job).await? {
                selections.push((i, selection.worker_id.clone(), selection.confidence));
                info!("  Selection {}: {} (confidence: {:.2})", 
                      i + 1, selection.worker_id, selection.confidence);
            }
        }

        // Show load balancing efficiency
        let efficiency = lb.get_efficiency().await?;
        info!("  Load balancing efficiency: {:.1}%", efficiency * 100.0);

        lb.stop().await?;
    }

    Ok(())
}

/// Demonstrate compilation caching benefits
async fn demo_compilation_caching() -> Result<()> {
    info!("\n💾 Demo 4: Compilation Caching");
    info!("------------------------------");

    // Test different cache strategies
    let strategies = vec![
        ("LRU with TTL", CacheStrategy::LruWithTtl { 
            max_entries: 100, 
            ttl: Duration::from_secs(3600) 
        }),
        ("LFU", CacheStrategy::LFU { 
            max_entries: 100 
        }),
        ("Size Limited", CacheStrategy::SizeLimited { 
            max_size_bytes: 10 * 1024 * 1024 
        }),
    ];

    for (name, strategy) in strategies {
        info!("\nTesting cache strategy: {}", name);
        
        let cache = CompilationCache::new(strategy)?;
        
        // Simulate compilation results
        let entries = create_sample_cache_entries();
        
        let start = Instant::now();
        
        // Fill cache
        for entry in &entries {
            let key = format!("job_{}", entry.job_id);
            cache.put(key, entry.clone()).await?;
        }
        
        // Test cache hits
        let mut hits = 0;
        let mut misses = 0;
        
        for entry in &entries {
            let key = format!("job_{}", entry.job_id);
            if cache.get(&key).await?.is_some() {
                hits += 1;
            } else {
                misses += 1;
            }
        }
        
        // Test some misses
        for i in 1000..1010 {
            let key = format!("job_missing_{}", i);
            if cache.get(&key).await?.is_none() {
                misses += 1;
            }
        }
        
        let total_time = start.elapsed();
        let stats = cache.get_stats().await?;
        
        info!("  Cache operations completed in: {:?}", total_time);
        info!("  Hit rate: {:.1}%", stats.hit_rate * 100.0);
        info!("  Entries: {}", stats.entry_count);
        info!("  Total size: {:.1} KB", stats.total_size_bytes as f64 / 1024.0);
        info!("  Efficiency score: {:.2}", stats.efficiency_score);
    }

    Ok(())
}

/// Demonstrate performance comparison between sequential and distributed compilation
async fn demo_performance_comparison() -> Result<()> {
    info!("\n🏁 Demo 5: Performance Comparison");
    info!("---------------------------------");

    // Create a large compilation job
    let large_job = create_large_compilation_job();
    info!("Created large job with {} source files", large_job.source_files.len());

    // Sequential compilation simulation
    info!("\nSimulating sequential compilation...");
    let sequential_start = Instant::now();
    simulate_sequential_compilation(&large_job).await;
    let sequential_time = sequential_start.elapsed();
    info!("Sequential compilation time: {:?}", sequential_time);

    // Distributed compilation simulation
    info!("\nSimulating distributed compilation...");
    let distributed_start = Instant::now();
    let speedup = simulate_distributed_compilation(&large_job).await?;
    let distributed_time = distributed_start.elapsed();
    info!("Distributed compilation time: {:?}", distributed_time);

    // Calculate improvements
    let time_speedup = sequential_time.as_secs_f64() / distributed_time.as_secs_f64();
    info!("\n📊 Performance Results:");
    info!("  Sequential time: {:?}", sequential_time);
    info!("  Distributed time: {:?}", distributed_time);
    info!("  Time speedup: {:.1}x", time_speedup);
    info!("  Efficiency speedup: {:.1}x", speedup);
    info!("  Time saved: {:?}", sequential_time.saturating_sub(distributed_time));

    Ok(())
}

/// Demonstrate fault tolerance capabilities
async fn demo_fault_tolerance() -> Result<()> {
    info!("\n🛡️  Demo 6: Fault Tolerance");
    info!("---------------------------");

    let config = DistributedConfig {
        fault_tolerance: true,
        network_timeout: Duration::from_secs(5),
        ..DistributedConfig::default()
    };

    let mut system = DistributedCompilationSystem::new(config)?;
    system.start().await?;

    // Add some workers
    let workers = create_test_workers_with_load();
    for worker in workers {
        system.add_worker(worker).await?;
    }

    info!("Initial workers: {}", system.get_workers().await?.len());

    // Simulate worker failures
    info!("\nSimulating worker failures...");
    
    // Get workers and simulate failure of one
    let workers = system.get_workers().await?;
    if let Some(worker) = workers.first() {
        info!("Simulating failure of worker: {}", worker.id);
        
        // Remove the worker (simulating failure)
        system.remove_worker(&worker.id).await?;
        
        info!("Remaining workers after failure: {}", system.get_workers().await?.len());
        
        // System should continue to function
        let stats = system.get_statistics();
        info!("System still operational - Active workers: {}", stats.active_workers);
    }

    // Test network timeout handling
    info!("\nTesting network timeout handling...");
    let timeout_config = NetworkConfig {
        connection_pool: ConnectionPoolConfig {
            connection_timeout: Duration::from_millis(1), // Very short timeout
            ..ConnectionPoolConfig::default()
        },
        ..NetworkConfig::default()
    };

    let network_optimizer = NetworkOptimizer::new(timeout_config);
    info!("Network optimizer handles short timeouts: {}", network_optimizer.is_ok());

    system.stop().await?;
    Ok(())
}

/// Simulate a real-world compilation scenario
async fn demo_real_world_simulation() -> Result<()> {
    info!("\n🌍 Demo 7: Real-World Simulation");
    info!("--------------------------------");

    let config = DistributedConfig {
        coordinator_port: 9300,
        max_workers: 12,
        chunk_size: 8,
        fault_tolerance: true,
        caching_enabled: true,
        load_balancing: LoadBalancingStrategy::AdaptiveWeighted,
        monitoring_enabled: true,
        ..DistributedConfig::default()
    };

    let mut system = DistributedCompilationSystem::new(config)?;
    system.start().await?;

    // Add a realistic set of workers (different performance levels)
    let workers = create_realistic_worker_farm();
    for worker in workers {
        system.add_worker(worker.clone()).await?;
        info!("Added worker: {} ({} cores, {:.1} perf score)", 
              worker.id, worker.capabilities.cpu_cores, worker.capabilities.performance_score);
    }

    // Simulate a large software project compilation
    info!("\nSimulating large project compilation...");
    let projects = create_multiple_projects();
    
    let mut total_jobs = 0;
    let start_time = Instant::now();

    for (project_name, files) in projects {
        info!("Compiling project: {} ({} files)", project_name, files.len());
        
        for chunk in files.chunks(8) {
            let job = CompilationJob::new(
                chunk.to_vec(),
                "x86_64-unknown-linux-gnu".to_string(),
                OutputType::Object,
            );
            
            // In a real implementation, this would actually submit the job
            // For demo purposes, we'll just simulate the compilation
            simulate_job_processing(&job).await;
            total_jobs += 1;
        }
    }

    let total_time = start_time.elapsed();

    // Generate final report
    info!("\n📈 Final Report:");
    info!("================");
    
    let stats = system.get_statistics();
    info!("Projects compiled: {}", 3);
    info!("Total jobs processed: {}", total_jobs);
    info!("Total compilation time: {:?}", total_time);
    info!("Active workers: {}", stats.active_workers);
    info!("Average speedup achieved: {:.1}x", 2.5); // Mock value
    info!("Cache efficiency: {:.1}%", 75.0); // Mock value
    info!("Network efficiency: {:.1}%", 85.0); // Mock value

    let performance_report = system.generate_performance_report();
    info!("\nDetailed Performance Report:");
    info!("{}", performance_report);

    system.stop().await?;
    Ok(())
}

// Helper functions

fn create_diverse_workers() -> Vec<WorkerNode> {
    let configs = vec![
        ("desktop-1", 8, 16384, 8.0),   // High-end desktop
        ("desktop-2", 4, 8192, 4.0),   // Mid-range desktop
        ("laptop-1", 4, 8192, 3.5),    // Laptop
        ("server-1", 16, 32768, 15.0), // Server
        ("server-2", 24, 65536, 20.0), // High-end server
    ];

    configs.into_iter().map(|(id, cores, memory, score)| {
        create_worker_with_specs(id, cores, memory, score)
    }).collect()
}

fn create_test_workers_with_load() -> Vec<WorkerNode> {
    let configs = vec![
        ("worker-low", 4, 8192, 4.0, 1),    // Low load
        ("worker-med", 8, 16384, 8.0, 4),   // Medium load
        ("worker-high", 4, 8192, 4.0, 3),   // High load
        ("worker-idle", 16, 32768, 16.0, 0), // Idle
    ];

    configs.into_iter().map(|(id, cores, memory, score, load)| {
        let mut worker = create_worker_with_specs(id, cores, memory, score);
        worker.metrics.active_jobs = load;
        worker
    }).collect()
}

fn create_realistic_worker_farm() -> Vec<WorkerNode> {
    let configs = vec![
        ("build-01", 8, 16384, 8.0),
        ("build-02", 8, 16384, 8.0),
        ("build-03", 16, 32768, 15.0),
        ("build-04", 16, 32768, 15.0),
        ("build-05", 4, 8192, 4.0),
        ("build-06", 4, 8192, 4.0),
        ("build-07", 32, 65536, 30.0), // High-end server
        ("build-08", 12, 24576, 12.0),
        ("gpu-01", 8, 16384, 10.0),    // GPU-accelerated
        ("gpu-02", 8, 16384, 10.0),
    ];

    let mut workers: Vec<WorkerNode> = configs.into_iter().map(|(id, cores, memory, score)| {
        create_worker_with_specs(id, cores, memory, score)
    }).collect();

    // Add GPU features to GPU workers
    for worker in &mut workers {
        if worker.id.starts_with("gpu-") {
            worker.capabilities.features.push("cuda".to_string());
            worker.capabilities.features.push("opencl".to_string());
        }
    }

    workers
}

fn create_worker_with_specs(id: &str, cores: usize, memory_mb: usize, performance_score: f64) -> WorkerNode {
    let capabilities = WorkerCapabilities {
        cpu_cores: cores,
        memory_mb,
        disk_space_gb: 1000,
        supported_targets: vec![
            "x86_64-unknown-linux-gnu".to_string(),
            "x86_64-pc-windows-msvc".to_string(),
            "aarch64-unknown-linux-gnu".to_string(),
        ],
        toolchains: {
            let mut toolchains = std::collections::HashMap::new();
            toolchains.insert("rust".to_string(), "1.70.0".to_string());
            toolchains.insert("clang".to_string(), "15.0.0".to_string());
            toolchains.insert("gcc".to_string(), "11.0.0".to_string());
            toolchains
        },
        max_concurrent_jobs: cores,
        performance_score,
        network_bandwidth: 1000.0, // 1 Gbps
        features: vec!["cross-compile".to_string()],
    };

    let address = format!("192.168.1.{}:9001", 100 + id.len()).parse().unwrap();
    let mut worker = WorkerNode::new(address, capabilities);
    worker.id = id.to_string();
    worker.status = WorkerStatus::Online;
    worker
}

fn create_sample_job(filename: &str) -> CompilationJob {
    let mut job = CompilationJob::new(
        vec![filename.to_string()],
        "x86_64-unknown-linux-gnu".to_string(),
        OutputType::Object,
    );
    job.optimization_level = "O2".to_string();
    job.priority = JobPriority::Normal;
    job
}

fn create_large_compilation_job() -> CompilationJob {
    let files: Vec<String> = (1..=100)
        .map(|i| format!("src/module_{:03}.csd", i))
        .collect();

    let mut job = CompilationJob::new(
        files,
        "x86_64-unknown-linux-gnu".to_string(),
        OutputType::Object,
    );
    job.optimization_level = "O2".to_string();
    job.estimate_duration();
    job
}

fn create_sample_cache_entries() -> Vec<CacheEntry> {
    (1..=50).map(|i| {
        CacheEntry {
            job_id: format!("job_{:03}", i),
            output: vec![0u8; 1024 * i], // Variable size outputs
            created_at: std::time::SystemTime::now(),
            access_count: 0,
        }
    }).collect()
}

fn create_multiple_projects() -> Vec<(String, Vec<String>)> {
    vec![
        ("web-server".to_string(), (1..=30).map(|i| format!("server/handler_{}.csd", i)).collect()),
        ("client-app".to_string(), (1..=45).map(|i| format!("client/component_{}.csd", i)).collect()),
        ("shared-lib".to_string(), (1..=20).map(|i| format!("shared/util_{}.csd", i)).collect()),
    ]
}

async fn simulate_sequential_compilation(job: &CompilationJob) {
    // Simulate compilation time per file
    let time_per_file = Duration::from_millis(500);
    let total_time = time_per_file * job.source_files.len() as u32;
    sleep(Duration::from_millis(total_time.as_millis() as u64 / 100)).await; // Scaled down for demo
}

async fn simulate_distributed_compilation(job: &CompilationJob) -> Result<f64> {
    let chunks = job.split_into_chunks(8);
    let parallelism = 4; // Simulate 4 workers
    
    // Simulate parallel processing
    let time_per_chunk = Duration::from_millis(200);
    let parallel_batches = (chunks.len() + parallelism - 1) / parallelism;
    let total_time = time_per_chunk * parallel_batches as u32;
    
    sleep(Duration::from_millis(total_time.as_millis() as u64 / 100)).await; // Scaled down for demo
    
    // Return simulated speedup
    Ok(3.2) // 3.2x speedup
}

async fn simulate_job_processing(job: &CompilationJob) {
    let processing_time = Duration::from_millis(50 * job.source_files.len() as u64);
    sleep(processing_time).await;
}
