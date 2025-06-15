/// LTO Performance Benchmarks
/// 
/// Comprehensive benchmarks for the CURSED Link-Time Optimization system,
/// measuring performance improvements and compilation time overhead.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use cursed::optimization::lto::{
    LtoOptimizer, LtoConfig, LtoLevel, LtoCompilationUnit, LtoStatistics
};
use cursed::error::Result;
use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Duration;

/// Generate test compilation units for benchmarking
fn generate_test_units(count: usize, size_per_unit: usize) -> Vec<LtoCompilationUnit> {
    let mut units = Vec::new();
    
    for i in 0..count {
        let mut unit = LtoCompilationUnit::new(
            format!("benchmark_unit_{}", i),
            PathBuf::from(format!("benchmark_unit_{}.bc", i))
        );
        
        // Add functions with realistic patterns
        for j in 0..10 {
            unit.exported_functions.insert(format!("unit_{}_function_{}", i, j));
        }
        
        // Add some cross-module dependencies
        if i > 0 {
            unit.dependencies.push(format!("benchmark_unit_{}", i - 1));
        }
        
        // Add globals
        for k in 0..3 {
            unit.exported_globals.insert(format!("unit_{}_global_{}", i, k));
        }
        
        unit.size_estimate = size_per_unit;
        units.push(unit);
    }
    
    units
}

/// Benchmark LTO optimizer creation
fn bench_lto_optimizer_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("lto_optimizer_creation");
    
    for level in [LtoLevel::None, LtoLevel::Thin, LtoLevel::Full] {
        group.bench_with_input(
            BenchmarkId::new("creation", level.as_str()),
            &level,
            |b, &level| {
                b.iter(|| {
                    let config = LtoConfig {
                        level,
                        ..Default::default()
                    };
                    black_box(LtoOptimizer::new(config).unwrap())
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark compilation unit addition
fn bench_compilation_unit_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("compilation_unit_addition");
    
    for unit_count in [10, 50, 100, 200] {
        group.throughput(Throughput::Elements(unit_count as u64));
        group.bench_with_input(
            BenchmarkId::new("add_units", unit_count),
            &unit_count,
            |b, &unit_count| {
                b.iter_batched(
                    || {
                        let config = LtoConfig {
                            level: LtoLevel::Thin,
                            ..Default::default()
                        };
                        let optimizer = LtoOptimizer::new(config).unwrap();
                        let units = generate_test_units(unit_count, 1000);
                        (optimizer, units)
                    },
                    |(mut optimizer, units)| {
                        for unit in units {
                            black_box(optimizer.add_compilation_unit(unit));
                        }
                        black_box(optimizer)
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    
    group.finish();
}

/// Benchmark cross-module analysis
fn bench_cross_module_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("cross_module_analysis");
    
    for unit_count in [5, 20, 50, 100] {
        group.throughput(Throughput::Elements(unit_count as u64));
        group.bench_with_input(
            BenchmarkId::new("analysis", unit_count),
            &unit_count,
            |b, &unit_count| {
                b.iter_batched(
                    || {
                        let config = LtoConfig {
                            level: LtoLevel::Thin,
                            enable_cross_module_inlining: true,
                            enable_whole_program_dce: true,
                            enable_global_variable_optimization: true,
                            enable_cross_module_constant_propagation: true,
                            ..Default::default()
                        };
                        let mut optimizer = LtoOptimizer::new(config).unwrap();
                        let units = generate_test_units(unit_count, 2000);
                        for unit in units {
                            optimizer.add_compilation_unit(unit);
                        }
                        optimizer
                    },
                    |mut optimizer| {
                        // Run optimization which includes analysis
                        black_box(optimizer.optimize().unwrap())
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    
    group.finish();
}

/// Benchmark Thin LTO optimization
fn bench_thin_lto_optimization(c: &mut Criterion) {
    let mut group = c.benchmark_group("thin_lto_optimization");
    group.measurement_time(Duration::from_secs(30));
    
    for (unit_count, size_per_unit) in [(10, 500), (25, 1000), (50, 1500)] {
        group.throughput(Throughput::Elements(unit_count as u64));
        group.bench_with_input(
            BenchmarkId::new("thin_lto", format!("{}x{}", unit_count, size_per_unit)),
            &(unit_count, size_per_unit),
            |b, &(unit_count, size_per_unit)| {
                b.iter_batched(
                    || {
                        let config = LtoConfig {
                            level: LtoLevel::Thin,
                            max_worker_threads: 4,
                            thin_lto_partition_threshold: 1000,
                            enable_cross_module_inlining: true,
                            enable_whole_program_dce: true,
                            ..Default::default()
                        };
                        let mut optimizer = LtoOptimizer::new(config).unwrap();
                        let units = generate_test_units(unit_count, size_per_unit);
                        for unit in units {
                            optimizer.add_compilation_unit(unit);
                        }
                        optimizer
                    },
                    |mut optimizer| {
                        black_box(optimizer.optimize().unwrap())
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    
    group.finish();
}

/// Benchmark Full LTO optimization
fn bench_full_lto_optimization(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_lto_optimization");
    group.measurement_time(Duration::from_secs(45));
    
    for unit_count in [5, 15, 30] {
        group.throughput(Throughput::Elements(unit_count as u64));
        group.bench_with_input(
            BenchmarkId::new("full_lto", unit_count),
            &unit_count,
            |b, &unit_count| {
                b.iter_batched(
                    || {
                        let config = LtoConfig {
                            level: LtoLevel::Full,
                            enable_cross_module_inlining: true,
                            enable_whole_program_dce: true,
                            enable_global_variable_optimization: true,
                            enable_cross_module_constant_propagation: true,
                            enable_devirtualization: true,
                            ..Default::default()
                        };
                        let mut optimizer = LtoOptimizer::new(config).unwrap();
                        let units = generate_test_units(unit_count, 2000);
                        for unit in units {
                            optimizer.add_compilation_unit(unit);
                        }
                        optimizer
                    },
                    |mut optimizer| {
                        black_box(optimizer.optimize().unwrap())
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    
    group.finish();
}

/// Benchmark LTO configuration comparison
fn bench_lto_level_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("lto_level_comparison");
    group.measurement_time(Duration::from_secs(20));
    
    let unit_count = 20;
    let size_per_unit = 1000;
    
    for level in [LtoLevel::None, LtoLevel::Thin, LtoLevel::Full] {
        group.bench_with_input(
            BenchmarkId::new("level_comparison", level.as_str()),
            &level,
            |b, &level| {
                b.iter_batched(
                    || {
                        let config = LtoConfig {
                            level,
                            enable_cross_module_inlining: level != LtoLevel::None,
                            enable_whole_program_dce: level != LtoLevel::None,
                            enable_global_variable_optimization: level != LtoLevel::None,
                            ..Default::default()
                        };
                        let mut optimizer = LtoOptimizer::new(config).unwrap();
                        let units = generate_test_units(unit_count, size_per_unit);
                        for unit in units {
                            optimizer.add_compilation_unit(unit);
                        }
                        optimizer
                    },
                    |mut optimizer| {
                        black_box(optimizer.optimize().unwrap())
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    
    group.finish();
}

/// Benchmark scalability with worker threads (Thin LTO)
fn bench_worker_thread_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("worker_thread_scalability");
    group.measurement_time(Duration::from_secs(25));
    
    let unit_count = 40;
    let size_per_unit = 1000;
    
    for worker_count in [1, 2, 4, 8] {
        group.bench_with_input(
            BenchmarkId::new("workers", worker_count),
            &worker_count,
            |b, &worker_count| {
                b.iter_batched(
                    || {
                        let config = LtoConfig {
                            level: LtoLevel::Thin,
                            max_worker_threads: worker_count,
                            thin_lto_partition_threshold: 800,
                            enable_cross_module_inlining: true,
                            enable_whole_program_dce: true,
                            ..Default::default()
                        };
                        let mut optimizer = LtoOptimizer::new(config).unwrap();
                        let units = generate_test_units(unit_count, size_per_unit);
                        for unit in units {
                            optimizer.add_compilation_unit(unit);
                        }
                        optimizer
                    },
                    |mut optimizer| {
                        black_box(optimizer.optimize().unwrap())
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    
    group.finish();
}

/// Benchmark partition threshold impact (Thin LTO)
fn bench_partition_threshold_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("partition_threshold_impact");
    
    let unit_count = 30;
    let size_per_unit = 1200;
    
    for threshold in [500, 1000, 2000, 4000] {
        group.bench_with_input(
            BenchmarkId::new("threshold", threshold),
            &threshold,
            |b, &threshold| {
                b.iter_batched(
                    || {
                        let config = LtoConfig {
                            level: LtoLevel::Thin,
                            max_worker_threads: 4,
                            thin_lto_partition_threshold: threshold,
                            enable_cross_module_inlining: true,
                            enable_whole_program_dce: true,
                            ..Default::default()
                        };
                        let mut optimizer = LtoOptimizer::new(config).unwrap();
                        let units = generate_test_units(unit_count, size_per_unit);
                        for unit in units {
                            optimizer.add_compilation_unit(unit);
                        }
                        optimizer
                    },
                    |mut optimizer| {
                        black_box(optimizer.optimize().unwrap())
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    
    group.finish();
}

/// Benchmark LTO statistics collection overhead
fn bench_statistics_collection(c: &mut Criterion) {
    let mut group = c.benchmark_group("statistics_collection");
    
    for (enable_profiling, unit_count) in [(false, 50), (true, 50)] {
        group.bench_with_input(
            BenchmarkId::new("stats", if enable_profiling { "enabled" } else { "disabled" }),
            &(enable_profiling, unit_count),
            |b, &(enable_profiling, unit_count)| {
                b.iter_batched(
                    || {
                        let config = LtoConfig {
                            level: LtoLevel::Thin,
                            enable_profiling,
                            enable_cross_module_inlining: true,
                            enable_whole_program_dce: true,
                            ..Default::default()
                        };
                        let mut optimizer = LtoOptimizer::new(config).unwrap();
                        let units = generate_test_units(unit_count, 1000);
                        for unit in units {
                            optimizer.add_compilation_unit(unit);
                        }
                        optimizer
                    },
                    |mut optimizer| {
                        let result = optimizer.optimize().unwrap();
                        black_box(result.statistics)
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    
    group.finish();
}

/// Benchmark complex optimization scenarios
fn bench_complex_optimization_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_optimization_scenarios");
    group.measurement_time(Duration::from_secs(40));
    
    // Scenario 1: Many small modules (web application style)
    group.bench_function("web_app_scenario", |b| {
        b.iter_batched(
            || {
                let config = LtoConfig {
                    level: LtoLevel::Thin,
                    max_worker_threads: 4,
                    thin_lto_partition_threshold: 500,
                    enable_cross_module_inlining: true,
                    enable_whole_program_dce: true,
                    enable_global_variable_optimization: true,
                    ..Default::default()
                };
                let mut optimizer = LtoOptimizer::new(config).unwrap();
                let units = generate_test_units(60, 300); // Many small modules
                for unit in units {
                    optimizer.add_compilation_unit(unit);
                }
                optimizer
            },
            |mut optimizer| {
                black_box(optimizer.optimize().unwrap())
            },
            criterion::BatchSize::SmallInput,
        );
    });
    
    // Scenario 2: Few large modules (system programming style)
    group.bench_function("system_app_scenario", |b| {
        b.iter_batched(
            || {
                let config = LtoConfig {
                    level: LtoLevel::Full,
                    enable_cross_module_inlining: true,
                    enable_whole_program_dce: true,
                    enable_global_variable_optimization: true,
                    enable_cross_module_constant_propagation: true,
                    enable_devirtualization: true,
                    ..Default::default()
                };
                let mut optimizer = LtoOptimizer::new(config).unwrap();
                let units = generate_test_units(8, 4000); // Few large modules
                for unit in units {
                    optimizer.add_compilation_unit(unit);
                }
                optimizer
            },
            |mut optimizer| {
                black_box(optimizer.optimize().unwrap())
            },
            criterion::BatchSize::SmallInput,
        );
    });
    
    // Scenario 3: Mixed module sizes (real-world style)
    group.bench_function("mixed_scenario", |b| {
        b.iter_batched(
            || {
                let config = LtoConfig {
                    level: LtoLevel::Thin,
                    max_worker_threads: 6,
                    thin_lto_partition_threshold: 1500,
                    enable_cross_module_inlining: true,
                    enable_whole_program_dce: true,
                    enable_global_variable_optimization: true,
                    enable_cross_module_constant_propagation: true,
                    ..Default::default()
                };
                let mut optimizer = LtoOptimizer::new(config).unwrap();
                
                // Generate mixed-size modules
                let mut units = Vec::new();
                for i in 0..30 {
                    let size = match i % 4 {
                        0 => 200,   // Small modules
                        1 => 800,   // Medium modules
                        2 => 2000,  // Large modules
                        3 => 5000,  // Very large modules
                        _ => 1000,
                    };
                    units.extend(generate_test_units(1, size));
                }
                
                for unit in units {
                    optimizer.add_compilation_unit(unit);
                }
                optimizer
            },
            |mut optimizer| {
                black_box(optimizer.optimize().unwrap())
            },
            criterion::BatchSize::SmallInput,
        );
    });
    
    group.finish();
}

/// Benchmark memory usage patterns
fn bench_memory_usage_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage_patterns");
    
    for (caching_enabled, unit_count) in [(false, 40), (true, 40)] {
        group.bench_with_input(
            BenchmarkId::new("caching", if caching_enabled { "enabled" } else { "disabled" }),
            &(caching_enabled, unit_count),
            |b, &(caching_enabled, unit_count)| {
                b.iter_batched(
                    || {
                        let config = LtoConfig {
                            level: LtoLevel::Thin,
                            enable_caching: caching_enabled,
                            enable_cross_module_inlining: true,
                            enable_whole_program_dce: true,
                            ..Default::default()
                        };
                        let mut optimizer = LtoOptimizer::new(config).unwrap();
                        let units = generate_test_units(unit_count, 1500);
                        for unit in units {
                            optimizer.add_compilation_unit(unit);
                        }
                        optimizer
                    },
                    |mut optimizer| {
                        black_box(optimizer.optimize().unwrap())
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    lto_benches,
    bench_lto_optimizer_creation,
    bench_compilation_unit_addition,
    bench_cross_module_analysis,
    bench_thin_lto_optimization,
    bench_full_lto_optimization,
    bench_lto_level_comparison,
    bench_worker_thread_scalability,
    bench_partition_threshold_impact,
    bench_statistics_collection,
    bench_complex_optimization_scenarios,
    bench_memory_usage_patterns
);

criterion_main!(lto_benches);
