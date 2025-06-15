# Distributed Compilation System for CURSED

A comprehensive distributed compilation infrastructure that enables scaling CURSED compilation across multiple machines with intelligent work distribution, fault tolerance, and optimization.

## Overview

The distributed compilation system provides:

- **Intelligent Work Distribution**: Smart load balancing across worker nodes
- **Network Optimization**: Compression, connection pooling, and bandwidth management
- **Fault Tolerance**: Automatic failover and retry mechanisms
- **Compilation Caching**: Distributed cache with multiple eviction strategies
- **Performance Monitoring**: Real-time metrics and optimization
- **Production Ready**: Comprehensive error handling and logging

## Quick Start

### Basic Setup

```rust
use cursed::optimization::distributed::*;

// Create configuration
let config = DistributedConfig {
    coordinator_port: 9000,
    max_workers: 16,
    chunk_size: 4,
    network_timeout: Duration::from_secs(30),
    fault_tolerance: true,
    caching_enabled: true,
    load_balancing: LoadBalancingStrategy::AdaptiveWeighted,
    monitoring_enabled: true,
    ..DistributedConfig::default()
};

// Create and start system
let mut system = DistributedCompilationSystem::new(config)?;
system.start().await?;

// Add worker nodes
let worker = WorkerNode::new(
    "192.168.1.100:9001".parse()?,
    WorkerCapabilities::detect_local()?
);
system.add_worker(worker).await?;

// Submit compilation job
let job = CompilationJob::new(
    vec!["main.csd".to_string(), "lib.csd".to_string()],
    "x86_64-unknown-linux-gnu".to_string(),
    OutputType::Object,
);

let result = system.compile_distributed(job).await?;
```

### Running the Demo

```bash
# Run the comprehensive demo
make distributed-compile-demo

# Or directly with cargo
cargo run --example distributed_compilation_demo
```

## Core Components

### 1. Distributed Compiler (`distributed_compiler.rs`)

Main coordinator for managing distributed compilation:

```rust
let mut compiler = DistributedCompiler::new(CompilerConfig {
    max_concurrent_jobs: 32,
    job_timeout: Duration::from_secs(300),
    retry_attempts: 3,
    chunk_size: 4,
    enable_monitoring: true,
})?;

// Submit single job
let result = compiler.submit_job(job).await?;

// Submit batch of jobs
let results = compiler.submit_batch(jobs).await?;
```

**Features:**
- Job chunking for large compilations
- Automatic retry on failures
- Performance monitoring
- Batch processing support

### 2. Worker Node Management (`worker_node.rs`)

Manages remote worker nodes:

```rust
let manager = WorkerNodeManager::new(WorkerConfig {
    heartbeat_interval: Duration::from_secs(30),
    response_timeout: Duration::from_secs(10),
    max_missed_heartbeats: 3,
    auto_discovery: true,
    ..WorkerConfig::default()
})?;

// Register workers
manager.register_worker(worker).await?;

// Find best worker for job
let best_worker = manager.find_best_worker(
    "x86_64-unknown-linux-gnu",
    Some("rust")
).await?;
```

**Features:**
- Automatic capability detection
- Health monitoring
- Performance scoring
- Network discovery

### 3. Load Balancer (`load_balancer.rs`)

Intelligent work distribution:

```rust
let mut lb = LoadBalancer::new(
    LoadBalancingStrategy::AdaptiveWeighted,
    max_workers
)?;

// Select best worker for job
let selection = lb.select_worker(&job).await?;

// Record job completion for learning
lb.record_job_completion(
    &job_id,
    &worker_id,
    duration,
    success
).await?;
```

**Strategies:**
- **RoundRobin**: Simple cycling through workers
- **LeastLoaded**: Select worker with lowest current load
- **PerformanceBased**: Consider historical performance
- **AdaptiveWeighted**: Combine multiple factors
- **WorkStealing**: Dynamic load redistribution

### 4. Network Optimizer (`network_optimizer.rs`)

Optimizes network communications:

```rust
let optimizer = NetworkOptimizer::new(NetworkConfig {
    compression: CompressionConfig {
        enabled: true,
        level: CompressionLevel::Balanced,
        algorithm: CompressionAlgorithm::LZ4,
        ..CompressionConfig::default()
    },
    bandwidth: BandwidthConfig {
        max_bandwidth_per_worker: 10_000_000, // 10 MB/s
        adaptive_enabled: true,
        ..BandwidthConfig::default()
    },
    ..NetworkConfig::default()
})?;

// Send optimized message
optimizer.send_message(&destination, message).await?;
```

**Features:**
- Multiple compression algorithms (LZ4, Zstd, Gzip, Brotli)
- Connection pooling
- Bandwidth management
- Quality of Service (QoS)

### 5. Compilation Cache (`compilation_cache.rs`)

Distributed caching system:

```rust
let cache = CompilationCache::new(CacheStrategy::LruWithTtl {
    max_entries: 10000,
    ttl: Duration::from_hours(24),
})?;

// Cache compilation result
cache.put(job.cache_key(), result).await?;

// Retrieve from cache
if let Some(cached) = cache.get(&job.cache_key()).await? {
    return Ok(cached.output);
}
```

**Strategies:**
- **LruWithTtl**: Least Recently Used with Time-To-Live
- **LFU**: Least Frequently Used
- **SizeLimited**: Size-based eviction
- **Adaptive**: Dynamic strategy selection

## Configuration

### System Configuration

```rust
let config = DistributedConfig {
    // Network settings
    coordinator_port: 9000,
    network_timeout: Duration::from_secs(30),
    
    // Scaling settings
    max_workers: 16,
    chunk_size: 4,
    
    // Features
    fault_tolerance: true,
    caching_enabled: true,
    monitoring_enabled: true,
    
    // Load balancing
    load_balancing: LoadBalancingStrategy::AdaptiveWeighted,
    
    // Network optimization
    network_config: NetworkConfig {
        compression: CompressionConfig {
            enabled: true,
            level: CompressionLevel::Balanced,
            min_size_bytes: 1024,
            algorithm: CompressionAlgorithm::LZ4,
        },
        connection_pool: ConnectionPoolConfig {
            max_connections_per_worker: 4,
            max_total_connections: 64,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300),
            keep_alive_interval: Duration::from_secs(60),
            multiplexing_enabled: true,
        },
        bandwidth: BandwidthConfig {
            max_bandwidth_per_worker: 10_000_000,
            total_bandwidth_limit: 100_000_000,
            adaptive_enabled: true,
            monitoring_interval: Duration::from_secs(1),
            qos: QosConfig::default(),
        },
        monitoring_enabled: true,
    },
};
```

### Worker Configuration

```rust
let worker_config = WorkerConfig {
    heartbeat_interval: Duration::from_secs(30),
    response_timeout: Duration::from_secs(10),
    max_missed_heartbeats: 3,
    metrics_interval: Duration::from_secs(5),
    auto_discovery: true,
    discovery_interval: Duration::from_secs(60),
};
```

## Performance Optimization

### Expected Speedup

The system typically achieves:

- **2-5x speedup** for medium projects (50-200 files)
- **3-8x speedup** for large projects (200+ files)
- **Linear scaling** up to 16 workers
- **Cache hit rates** of 60-90% in typical development

### Performance Factors

1. **Network Latency**: Lower latency = better performance
2. **Worker Capabilities**: More cores/memory = higher throughput
3. **Job Size**: Larger jobs benefit more from distribution
4. **Cache Efficiency**: High cache hit rates reduce work
5. **Load Balance Quality**: Even distribution prevents bottlenecks

### Monitoring

```rust
// Get current statistics
let stats = system.get_statistics();
println!("Speedup: {:.1}x", stats.speedup_factor);
println!("Cache hit rate: {:.1}%", stats.cache_hit_rate * 100.0);
println!("Active workers: {}", stats.active_workers);

// Generate performance report
let report = system.generate_performance_report();
println!("{}", report);
```

## Testing

### Running Tests

```bash
# All tests
make distributed-compile-test

# Specific test categories
make distributed-compile-test-unit
make distributed-compile-test-integration
make distributed-compile-test-performance
make distributed-compile-fault-tolerance

# Performance benchmarks
make distributed-compile-benchmark

# Coverage analysis
make distributed-compile-coverage
```

### Test Categories

1. **Unit Tests**: Individual component validation
2. **Integration Tests**: End-to-end system testing
3. **Performance Tests**: Benchmarking and optimization
4. **Fault Tolerance Tests**: Error handling and recovery
5. **Edge Case Tests**: Boundary conditions and error cases

### Performance Benchmarks

```bash
# Load balancer performance (should complete <100ms for 1000 selections)
make distributed-compile-load-balancer-tests

# Cache performance (should handle 1000 operations <1s)
make distributed-compile-cache-tests

# Network optimization benchmarks
make distributed-compile-network-tests
```

## Fault Tolerance

### Error Handling

The system handles various failure scenarios:

1. **Worker Failures**: Automatic failover to healthy workers
2. **Network Issues**: Retry with exponential backoff
3. **Timeout Handling**: Configurable timeouts with graceful degradation
4. **Partial Failures**: Completion tracking and retry mechanisms

### Configuration

```rust
let config = DistributedConfig {
    fault_tolerance: true,
    network_timeout: Duration::from_secs(30),
    // ... other settings
};

let compiler_config = CompilerConfig {
    retry_attempts: 3,
    job_timeout: Duration::from_secs(300),
    // ... other settings
};
```

### Recovery Mechanisms

- **Automatic Retry**: Failed jobs are retried automatically
- **Worker Health Monitoring**: Unhealthy workers are marked offline
- **Graceful Degradation**: System continues with reduced capacity
- **State Recovery**: Compilation state is preserved across failures

## Production Deployment

### System Requirements

**Coordinator Node:**
- 4+ CPU cores
- 8+ GB RAM
- 100+ GB storage
- Gigabit network

**Worker Nodes:**
- 2+ CPU cores per concurrent job
- 4+ GB RAM per concurrent job
- 50+ GB storage
- Fast network connection

### Network Setup

```bash
# Configure firewall
sudo ufw allow 9000/tcp  # Coordinator port
sudo ufw allow 9001/tcp  # Worker port

# Ensure workers can reach coordinator
ping coordinator-ip

# Test network bandwidth
iperf3 -s  # On coordinator
iperf3 -c coordinator-ip  # On workers
```

### Monitoring and Logging

The system provides comprehensive logging:

```rust
// Enable structured logging
tracing_subscriber::fmt()
    .with_env_filter("cursed::optimization::distributed=debug")
    .init();

// Monitor performance
let stats = system.get_statistics();
tracing::info!(
    workers = stats.active_workers,
    speedup = stats.speedup_factor,
    cache_hit_rate = stats.cache_hit_rate,
    "System performance"
);
```

## Integration with Build System

### Makefile Integration

The system is fully integrated with the CURSED build system:

```bash
# Quick validation
make distributed-compile-quick

# Full test suite
make distributed-compile-all

# Individual components
make distributed-compile-worker-tests
make distributed-compile-load-balancer-tests
make distributed-compile-cache-tests

# Performance analysis
make distributed-compile-benchmark
make distributed-compile-coverage
```

### CI/CD Integration

```yaml
# Example GitHub Actions workflow
- name: Test Distributed Compilation
  run: |
    make distributed-compile-validate
    make distributed-compile-test
    make distributed-compile-coverage
```

## Advanced Usage

### Custom Load Balancing

```rust
// Implement custom strategy
pub struct CustomStrategy {
    // Custom state
}

impl LoadBalancingStrategy for CustomStrategy {
    fn select_worker(&self, job: &CompilationJob, workers: &[WorkerNode]) -> Option<String> {
        // Custom selection logic
    }
}

// Use custom strategy
let lb = LoadBalancer::new(LoadBalancingStrategy::Custom(Box::new(CustomStrategy::new())), 16)?;
```

### Custom Caching

```rust
// Implement custom cache strategy
let cache = CompilationCache::new(CacheStrategy::Custom {
    eviction_policy: Box::new(MyCustomEviction),
    storage_backend: Box::new(MyCustomStorage),
})?;
```

### Network Protocols

```rust
// Configure custom transport protocols
let network_config = NetworkConfig {
    protocol: ProtocolConfig {
        transport_mapping: {
            let mut mapping = HashMap::new();
            mapping.insert("job_assignment".to_string(), TransportProtocol::QUIC);
            mapping.insert("heartbeat".to_string(), TransportProtocol::UDP);
            mapping
        },
        serialization: SerializationFormat::ProtocolBuffers,
        // ... other settings
    },
    // ... other settings
};
```

## Troubleshooting

### Common Issues

**Connection Timeouts:**
```bash
# Check network connectivity
ping worker-ip
telnet worker-ip 9001

# Increase timeout
let config = NetworkConfig {
    connection_pool: ConnectionPoolConfig {
        connection_timeout: Duration::from_secs(60),
        // ...
    },
    // ...
};
```

**High Memory Usage:**
```bash
# Monitor memory
make distributed-compile-test-performance

# Reduce cache size
let cache = CompilationCache::new(CacheStrategy::SizeLimited {
    max_size_bytes: 100 * 1024 * 1024, // 100MB
})?;
```

**Poor Performance:**
```bash
# Check worker distribution
make distributed-compile-load-balancer-tests

# Enable detailed monitoring
let config = DistributedConfig {
    monitoring_enabled: true,
    // ...
};
```

### Debug Information

```rust
// Enable debug logging
std::env::set_var("RUST_LOG", "cursed::optimization::distributed=debug");

// Get detailed statistics
let stats = system.get_statistics();
let workers = system.get_workers().await?;
let report = system.generate_performance_report();

println!("Statistics: {:#?}", stats);
println!("Workers: {:#?}", workers);
println!("Report:\n{}", report);
```

## Future Enhancements

Planned improvements include:

1. **GPU Acceleration**: Support for GPU-accelerated compilation
2. **Cloud Integration**: Native support for cloud worker nodes
3. **Advanced Scheduling**: ML-based load prediction
4. **Cross-Platform**: Enhanced Windows and macOS support
5. **Kubernetes Integration**: Native K8s deployment support

## Contributing

To contribute to the distributed compilation system:

1. Run the full test suite: `make distributed-compile-all`
2. Add tests for new features: `tests/distributed_compilation_test.rs`
3. Update documentation: `docs/distributed_compilation.md`
4. Follow the coding standards in existing modules

## License

This distributed compilation system is part of the CURSED programming language project and follows the same license terms.
