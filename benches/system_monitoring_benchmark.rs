/// Performance benchmarks for system monitoring module
/// 
/// Measures the overhead and performance characteristics of various
/// monitoring operations to ensure they're suitable for production use.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use cursed::stdlib::system::monitoring::{
    SystemMonitor, monitor_system, get_resource_usage, get_performance_metrics,
    get_cpu_usage, get_memory_usage, get_disk_usage, get_network_statistics,
};
use std::time::Duration;

fn benchmark_monitor_creation(c: &mut Criterion) {
    c.bench_function("monitor_creation", |b| {
        b.iter(|| {
            let monitor = SystemMonitor::new();
            black_box(monitor);
        });
    });
}

fn benchmark_monitor_creation_with_cache(c: &mut Criterion) {
    let mut group = c.benchmark_group("monitor_creation_with_cache");
    
    for cache_duration_ms in [100, 500, 1000, 5000].iter() {
        group.bench_with_input(
            BenchmarkId::new("cache_duration", cache_duration_ms),
            cache_duration_ms,
            |b, &duration_ms| {
                b.iter(|| {
                    let monitor = SystemMonitor::with_cache_duration(Duration::from_millis(duration_ms));
                    black_box(monitor);
                });
            },
        );
    }
    group.finish();
}

fn benchmark_resource_usage_collection(c: &mut Criterion) {
    c.bench_function("get_resource_usage", |b| {
        b.iter(|| {
            let usage = get_resource_usage().unwrap();
            black_box(usage);
        });
    });
}

fn benchmark_individual_metrics(c: &mut Criterion) {
    let mut group = c.benchmark_group("individual_metrics");
    
    group.bench_function("cpu_usage", |b| {
        b.iter(|| {
            let cpu = get_cpu_usage().unwrap();
            black_box(cpu);
        });
    });
    
    group.bench_function("memory_usage", |b| {
        b.iter(|| {
            let memory = get_memory_usage().unwrap();
            black_box(memory);
        });
    });
    
    group.bench_function("disk_usage", |b| {
        b.iter(|| {
            let disk = get_disk_usage().unwrap();
            black_box(disk);
        });
    });
    
    group.bench_function("network_statistics", |b| {
        b.iter(|| {
            let network = get_network_statistics().unwrap();
            black_box(network);
        });
    });
    
    group.finish();
}

fn benchmark_performance_metrics(c: &mut Criterion) {
    c.bench_function("get_performance_metrics", |b| {
        b.iter(|| {
            let metrics = get_performance_metrics().unwrap();
            black_box(metrics);
        });
    });
}

fn benchmark_monitor_with_caching(c: &mut Criterion) {
    let mut group = c.benchmark_group("caching_performance");
    
    // Test different cache durations
    for cache_duration_ms in [100, 500, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("cached_calls", cache_duration_ms),
            cache_duration_ms,
            |b, &duration_ms| {
                let monitor = SystemMonitor::with_cache_duration(Duration::from_millis(duration_ms));
                b.iter(|| {
                    // First call should populate cache
                    let usage1 = monitor.get_resource_usage().unwrap();
                    // Second call should use cache
                    let usage2 = monitor.get_resource_usage().unwrap();
                    black_box((usage1, usage2));
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_concurrent_monitoring(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_monitoring");
    
    for thread_count in [1, 2, 4, 8].iter() {
        group.bench_with_input(
            BenchmarkId::new("threads", thread_count),
            thread_count,
            |b, &threads| {
                b.iter(|| {
                    let monitor = std::sync::Arc::new(SystemMonitor::new());
                    let mut handles = vec![];
                    
                    for _ in 0..threads {
                        let monitor_clone = std::sync::Arc::clone(&monitor);
                        let handle = std::thread::spawn(move || {
                            for _ in 0..10 {
                                let usage = monitor_clone.get_resource_usage().unwrap();
                                black_box(usage);
                            }
                        });
                        handles.push(handle);
                    }
                    
                    for handle in handles {
                        handle.join().unwrap();
                    }
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_cache_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_operations");
    
    let monitor = SystemMonitor::new();
    
    group.bench_function("cache_clear", |b| {
        b.iter(|| {
            // Populate cache first
            let _ = monitor.get_resource_usage();
            // Clear cache
            let result = monitor.clear_cache();
            black_box(result);
        });
    });
    
    group.bench_function("cache_stats", |b| {
        b.iter(|| {
            let stats = monitor.get_cache_stats().unwrap();
            black_box(stats);
        });
    });
    
    group.finish();
}

fn benchmark_system_info_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("system_info");
    
    group.bench_function("system_info_summary", |b| {
        b.iter(|| {
            let info = cursed::stdlib::system::monitoring::get_system_info_summary().unwrap();
            black_box(info);
        });
    });
    
    group.bench_function("process_info_current", |b| {
        let current_pid = std::process::id();
        b.iter(|| {
            let info = cursed::stdlib::system::monitoring::get_process_info(current_pid).unwrap();
            black_box(info);
        });
    });
    
    group.bench_function("top_processes_cpu", |b| {
        b.iter(|| {
            let processes = cursed::stdlib::system::monitoring::get_top_processes_by_cpu(10).unwrap();
            black_box(processes);
        });
    });
    
    group.bench_function("top_processes_memory", |b| {
        b.iter(|| {
            let processes = cursed::stdlib::system::monitoring::get_top_processes_by_memory(10).unwrap();
            black_box(processes);
        });
    });
    
    group.finish();
}

fn benchmark_monitoring_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("monitoring_overhead");
    
    // Baseline: empty function call
    group.bench_function("baseline_empty", |b| {
        b.iter(|| {
            black_box(());
        });
    });
    
    // Monitor creation overhead
    group.bench_function("monitor_new", |b| {
        b.iter(|| {
            let monitor = SystemMonitor::new();
            black_box(monitor);
        });
    });
    
    // Single metric vs full resource usage
    let monitor = SystemMonitor::new();
    group.bench_function("single_cpu_metric", |b| {
        b.iter(|| {
            let cpu = monitor.get_cpu_usage().unwrap();
            black_box(cpu);
        });
    });
    
    group.bench_function("full_resource_usage", |b| {
        b.iter(|| {
            let usage = monitor.get_resource_usage().unwrap();
            black_box(usage);
        });
    });
    
    group.finish();
}

fn benchmark_repeated_calls(c: &mut Criterion) {
    let mut group = c.benchmark_group("repeated_calls");
    
    let monitor = SystemMonitor::new();
    
    // Test repeated calls without caching
    group.bench_function("repeated_no_cache", |b| {
        b.iter(|| {
            for _ in 0..10 {
                let usage = monitor.get_resource_usage().unwrap();
                black_box(usage);
            }
        });
    });
    
    // Test repeated calls with aggressive caching
    let cached_monitor = SystemMonitor::with_cache_duration(Duration::from_secs(10));
    group.bench_function("repeated_with_cache", |b| {
        b.iter(|| {
            for _ in 0..10 {
                let usage = cached_monitor.get_resource_usage().unwrap();
                black_box(usage);
            }
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_monitor_creation,
    benchmark_monitor_creation_with_cache,
    benchmark_resource_usage_collection,
    benchmark_individual_metrics,
    benchmark_performance_metrics,
    benchmark_monitor_with_caching,
    benchmark_concurrent_monitoring,
    benchmark_cache_operations,
    benchmark_system_info_operations,
    benchmark_monitoring_overhead,
    benchmark_repeated_calls
);

criterion_main!(benches);
