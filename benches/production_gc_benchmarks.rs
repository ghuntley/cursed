/// Production Garbage Collector Performance Benchmarks
/// 
/// This benchmark suite measures the performance characteristics of the
/// production garbage collector under various workloads and usage patterns.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;

use cursed::memory::{
    production_gc::{ProductionGarbageCollector, ProductionGcConfig},
    pressure_detection::PressureLevel,
    object_store::Storable,
    gc::CollectionTrigger,
};

/// Benchmark test object
#[derive(Debug, Clone)]
struct BenchObject {
    data: Vec<u8>,
    id: u64,
    refs: Vec<u64>,
}

impl Storable for BenchObject {
    fn size_hint(&self) -> usize {
        std::mem::size_of::<Self>() + self.data.len() + self.refs.len() * 8
    }
    
    fn type_name(&self) -> &'static str {
        "BenchObject"
    }
}

impl BenchObject {
    fn new(size: usize, id: u64) -> Self {
        Self {
            data: vec![0u8; size],
            id,
            refs: Vec::new(),
        }
    }
    
    fn with_refs(size: usize, id: u64, ref_count: usize) -> Self {
        Self {
            data: vec![0u8; size],
            id,
            refs: (0..ref_count).map(|i| id + i as u64).collect(),
        }
    }
}

/// Create optimized benchmark configuration
fn create_bench_config() -> ProductionGcConfig {
    let mut config = ProductionGcConfig::default();
    
    // Larger heap for performance testing
    config.initial_heap_size = 64 * 1024 * 1024;  // 64MB
    config.max_heap_size = 512 * 1024 * 1024;     // 512MB
    
    // Optimized collection thresholds
    config.gc_config.young_gen_threshold = 0.8;
    config.gc_config.old_gen_threshold = 0.9;
    config.emergency_threshold = 0.95;
    
    // Disable background collection for consistent benchmarks
    config.enable_auto_collection = false;
    config.enable_profiling = false; // Reduce overhead
    
    // Optimize for performance
    config.collection_threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    
    config
}

/// Benchmark basic allocation performance
fn bench_allocation_performance(c: &mut Criterion) {
    let config = create_bench_config();
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    let mut group = c.benchmark_group("allocation_performance");
    
    // Test different allocation sizes
    for size in [64, 256, 1024, 4096, 16384].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("allocate", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let obj = BenchObject::new(size, 1);
                    black_box(gc.allocate(obj).unwrap())
                })
            },
        );
    }
    
    group.finish();
}

/// Benchmark allocation throughput
fn bench_allocation_throughput(c: &mut Criterion) {
    let config = create_bench_config();
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    let mut group = c.benchmark_group("allocation_throughput");
    group.throughput(Throughput::Elements(1000));
    
    group.bench_function("throughput_1000_objects", |b| {
        b.iter(|| {
            let mut objects = Vec::new();
            for i in 0..1000 {
                let obj = BenchObject::new(256, i);
                if let Ok(ptr) = gc.allocate(obj) {
                    objects.push(ptr);
                }
            }
            black_box(objects)
        })
    });
    
    group.finish();
}

/// Benchmark garbage collection performance
fn bench_collection_performance(c: &mut Criterion) {
    let config = create_bench_config();
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    // Pre-allocate objects to create garbage
    let mut objects = Vec::new();
    for i in 0..1000 {
        let obj = BenchObject::new(512, i);
        if let Ok(ptr) = gc.allocate(obj) {
            objects.push(ptr);
        }
    }
    
    // Drop some objects to create garbage
    objects.truncate(500);
    
    let mut group = c.benchmark_group("collection_performance");
    
    group.bench_function("manual_collection", |b| {
        b.iter(|| {
            black_box(gc.collect().unwrap())
        })
    });
    
    group.bench_function("emergency_collection", |b| {
        b.iter(|| {
            black_box(gc.collect_with_trigger(CollectionTrigger::Emergency).unwrap())
        })
    });
    
    group.finish();
}

/// Benchmark mixed workload performance
fn bench_mixed_workload(c: &mut Criterion) {
    let config = create_bench_config();
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    let mut group = c.benchmark_group("mixed_workload");
    
    group.bench_function("allocate_and_collect", |b| {
        b.iter(|| {
            // Allocation phase
            let mut objects = Vec::new();
            for i in 0..500 {
                let obj = BenchObject::new(256, i);
                if let Ok(ptr) = gc.allocate(obj) {
                    objects.push(ptr);
                }
            }
            
            // Collection phase
            let _stats = gc.collect().unwrap();
            
            black_box(objects)
        })
    });
    
    group.finish();
}

/// Benchmark concurrent allocation
fn bench_concurrent_allocation(c: &mut Criterion) {
    let config = create_bench_config();
    let gc = Arc::new(ProductionGarbageCollector::new(config).unwrap());
    
    let mut group = c.benchmark_group("concurrent_allocation");
    
    for thread_count in [1, 2, 4, 8].iter() {
        group.bench_with_input(
            BenchmarkId::new("threads", thread_count),
            thread_count,
            |b, &thread_count| {
                b.iter(|| {
                    let mut handles = Vec::new();
                    
                    for _ in 0..thread_count {
                        let gc_clone = gc.clone();
                        let handle = thread::spawn(move || {
                            let mut local_objects = Vec::new();
                            for i in 0..100 {
                                let obj = BenchObject::new(256, i);
                                if let Ok(ptr) = gc_clone.allocate(obj) {
                                    local_objects.push(ptr);
                                }
                            }
                            local_objects
                        });
                        handles.push(handle);
                    }
                    
                    let mut all_objects = Vec::new();
                    for handle in handles {
                        all_objects.extend(handle.join().unwrap());
                    }
                    
                    black_box(all_objects)
                })
            },
        );
    }
    
    group.finish();
}

/// Benchmark memory pressure scenarios
fn bench_memory_pressure(c: &mut Criterion) {
    let mut config = create_bench_config();
    config.max_heap_size = 8 * 1024 * 1024; // Smaller heap for pressure testing
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    let mut group = c.benchmark_group("memory_pressure");
    
    group.bench_function("high_pressure_allocation", |b| {
        b.iter(|| {
            // Fill up memory to create pressure
            let mut objects = Vec::new();
            loop {
                let obj = BenchObject::new(1024, objects.len() as u64);
                match gc.allocate(obj) {
                    Ok(ptr) => objects.push(ptr),
                    Err(_) => break, // Out of memory
                }
                
                if objects.len() > 5000 {
                    break; // Prevent infinite loop
                }
            }
            
            // Force collection under pressure
            let _stats = gc.collect().unwrap();
            
            black_box(objects)
        })
    });
    
    group.finish();
}

/// Benchmark different object sizes
fn bench_object_sizes(c: &mut Criterion) {
    let config = create_bench_config();
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    let mut group = c.benchmark_group("object_sizes");
    
    let sizes = [
        ("tiny", 16),
        ("small", 64),
        ("medium", 256),
        ("large", 1024),
        ("huge", 4096),
        ("giant", 16384),
        ("massive", 65536),
    ];
    
    for (name, size) in sizes.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("size", name),
            size,
            |b, &size| {
                b.iter(|| {
                    let obj = BenchObject::new(size, 1);
                    black_box(gc.allocate(obj).unwrap())
                })
            },
        );
    }
    
    group.finish();
}

/// Benchmark collection algorithms
fn bench_collection_algorithms(c: &mut Criterion) {
    let config = create_bench_config();
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    // Pre-allocate mixed objects
    let mut objects = Vec::new();
    for i in 0..1000 {
        let size = if i % 10 == 0 { 2048 } else { 256 };
        let obj = BenchObject::new(size, i);
        if let Ok(ptr) = gc.allocate(obj) {
            objects.push(ptr);
        }
    }
    
    let mut group = c.benchmark_group("collection_algorithms");
    
    let triggers = [
        ("manual", CollectionTrigger::Manual),
        ("allocation_pressure", CollectionTrigger::AllocationPressure),
        ("heap_utilization", CollectionTrigger::HeapUtilization),
        ("periodic", CollectionTrigger::Periodic),
    ];
    
    for (name, trigger) in triggers.iter() {
        group.bench_with_input(
            BenchmarkId::new("trigger", name),
            trigger,
            |b, &trigger| {
                b.iter(|| {
                    black_box(gc.collect_with_trigger(trigger).unwrap())
                })
            },
        );
    }
    
    group.finish();
}

/// Benchmark allocation patterns
fn bench_allocation_patterns(c: &mut Criterion) {
    let config = create_bench_config();
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    let mut group = c.benchmark_group("allocation_patterns");
    
    // Sequential allocation pattern
    group.bench_function("sequential", |b| {
        b.iter(|| {
            let mut objects = Vec::new();
            for i in 0..1000 {
                let obj = BenchObject::new(256, i);
                if let Ok(ptr) = gc.allocate(obj) {
                    objects.push(ptr);
                }
            }
            black_box(objects)
        })
    });
    
    // Random size allocation pattern
    group.bench_function("random_sizes", |b| {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        b.iter(|| {
            let mut objects = Vec::new();
            for i in 0..1000 {
                let mut hasher = DefaultHasher::new();
                i.hash(&mut hasher);
                let size = 64 + (hasher.finish() % 4000) as usize;
                
                let obj = BenchObject::new(size, i);
                if let Ok(ptr) = gc.allocate(obj) {
                    objects.push(ptr);
                }
            }
            black_box(objects)
        })
    });
    
    // Batched allocation pattern
    group.bench_function("batched", |b| {
        b.iter(|| {
            let mut all_objects = Vec::new();
            for batch in 0..10 {
                let mut batch_objects = Vec::new();
                for i in 0..100 {
                    let obj = BenchObject::new(256, batch * 100 + i);
                    if let Ok(ptr) = gc.allocate(obj) {
                        batch_objects.push(ptr);
                    }
                }
                all_objects.push(batch_objects);
                
                // Force collection between batches
                let _ = gc.collect();
            }
            black_box(all_objects)
        })
    });
    
    group.finish();
}

/// Benchmark long-running scenarios
fn bench_long_running(c: &mut Criterion) {
    let config = create_bench_config();
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    let mut group = c.benchmark_group("long_running");
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("sustained_allocation", |b| {
        b.iter(|| {
            let start = Instant::now();
            let mut objects = Vec::new();
            let mut allocation_count = 0;
            
            // Run for a fixed time period
            while start.elapsed() < Duration::from_millis(100) {
                let obj = BenchObject::new(512, allocation_count);
                match gc.allocate(obj) {
                    Ok(ptr) => {
                        objects.push(ptr);
                        allocation_count += 1;
                        
                        // Periodically drop some objects
                        if allocation_count % 100 == 0 {
                            objects.truncate(objects.len() / 2);
                        }
                        
                        // Periodic collection
                        if allocation_count % 500 == 0 {
                            let _ = gc.collect();
                        }
                    }
                    Err(_) => {
                        // Collection on allocation failure
                        let _ = gc.collect();
                        objects.clear();
                    }
                }
            }
            
            black_box((allocation_count, objects))
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_allocation_performance,
    bench_allocation_throughput,
    bench_collection_performance,
    bench_mixed_workload,
    bench_concurrent_allocation,
    bench_memory_pressure,
    bench_object_sizes,
    bench_collection_algorithms,
    bench_allocation_patterns,
    bench_long_running
);

criterion_main!(benches);
