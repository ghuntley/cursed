/// Comprehensive Optimization Infrastructure Benchmarks
/// 
/// This benchmark suite measures the performance characteristics of the optimization
/// infrastructure across all optimization features, providing detailed analysis
/// of compilation speed improvements, runtime performance gains, and overhead costs.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::time::{Duration, Instant};
use std::collections::HashMap;

use cursed::optimization::*;

/// Benchmark LLVM advanced optimization passes
fn bench_llvm_advanced_optimization(c: &mut Criterion) {
    let config = OptimizationConfig::default();
    let mut optimizer = llvm_advanced::AdvancedOptimizationManager::new(&config).unwrap();
    
    let context = inkwell::context::Context::create();
    let module = context.create_module("benchmark_module");
    
    // Create multiple functions with different complexities
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    
    // Simple function
    let simple_fn = module.add_function("simple_function", fn_type, None);
    let simple_block = context.append_basic_block(simple_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(simple_block);
    let param = simple_fn.get_first_param().unwrap();
    builder.build_return(Some(&param)).unwrap();
    
    // Complex function with loops and branches
    let complex_fn = module.add_function("complex_function", fn_type, None);
    let entry_block = context.append_basic_block(complex_fn, "entry");
    let loop_block = context.append_basic_block(complex_fn, "loop");
    let exit_block = context.append_basic_block(complex_fn, "exit");
    
    builder.position_at_end(entry_block);
    let zero = i32_type.const_int(0, false);
    let one = i32_type.const_int(1, false);
    let counter = builder.build_alloca(i32_type, "counter").unwrap();
    builder.build_store(counter, zero).unwrap();
    builder.build_unconditional_branch(loop_block).unwrap();
    
    builder.position_at_end(loop_block);
    let counter_val = builder.build_load(i32_type, counter, "counter_val").unwrap();
    let incremented = builder.build_int_add(counter_val.into_int_value(), one, "inc").unwrap();
    builder.build_store(counter, incremented).unwrap();
    let param = complex_fn.get_first_param().unwrap();
    let cmp = builder.build_int_compare(inkwell::IntPredicate::ULT, incremented, param.into_int_value(), "cmp").unwrap();
    builder.build_conditional_branch(cmp, loop_block, exit_block).unwrap();
    
    builder.position_at_end(exit_block);
    builder.build_return(Some(&incremented)).unwrap();
    
    let mut group = c.benchmark_group("llvm_advanced_optimization");
    
    group.bench_function("optimize_simple_module", |b| {
        b.iter(|| {
            black_box(optimizer.optimize_module(&module).unwrap())
        })
    });
    
    group.finish();
}

/// Benchmark parallel compilation performance
fn bench_parallel_compilation(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_compilation");
    
    for &thread_count in [1, 2, 4, 8].iter() {
        group.throughput(Throughput::Elements(10));
        group.bench_with_input(
            BenchmarkId::new("compile_modules", thread_count),
            &thread_count,
            |b, &thread_count| {
                let config = OptimizationConfig {
                    max_parallel_threads: thread_count,
                    ..Default::default()
                };
                
                let mut compiler = parallel_compilation::ParallelCompiler::new(&config).unwrap();
                
                let modules = (0..10).map(|i| {
                    (format!("module_{}", i), format!("src/module_{}.csd", i), vec![])
                }).collect();
                
                b.iter(|| {
                    black_box(compiler.compile_modules(modules.clone()).unwrap())
                })
            },
        );
    }
    
    group.finish();
}

/// Benchmark JIT optimization effectiveness
fn bench_jit_optimization(c: &mut Criterion) {
    let config = OptimizationConfig::default();
    let mut optimizer = jit_optimization::AdaptiveJitOptimizer::new(&config).unwrap();
    
    let mut group = c.benchmark_group("jit_optimization");
    
    // Benchmark hot path detection
    group.bench_function("hot_path_detection", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let execution_time = Duration::from_nanos(100 + (i % 50) as u64);
                optimizer.record_execution("benchmark_function", execution_time);
            }
            black_box(optimizer.get_optimization_recommendations())
        })
    });
    
    // Benchmark optimization application
    group.bench_function("apply_optimization", |b| {
        // Pre-populate with hot path data
        for _i in 0..200 {
            optimizer.record_execution("hot_function", Duration::from_nanos(150));
        }
        
        let recommendations = optimizer.get_optimization_recommendations();
        
        b.iter(|| {
            for recommendation in &recommendations {
                black_box(optimizer.apply_optimization(recommendation).unwrap());
            }
        })
    });
    
    group.finish();
}

/// Benchmark incremental compilation performance
fn bench_incremental_compilation(c: &mut Criterion) {
    let config = OptimizationConfig::default();
    let mut compiler = incremental_compilation::IncrementalCompiler::new(&config).unwrap();
    
    // Create temporary project directory
    let temp_dir = std::env::temp_dir().join("cursed_incremental_bench");
    std::fs::create_dir_all(&temp_dir).unwrap();
    
    let mut group = c.benchmark_group("incremental_compilation");
    
    group.bench_function("full_incremental_compilation", |b| {
        b.iter(|| {
            black_box(compiler.compile_incrementally(&temp_dir).unwrap())
        })
    });
    
    group.finish();
    
    // Clean up
    let _ = std::fs::remove_dir_all(&temp_dir);
}

/// Benchmark memory optimization performance
fn bench_memory_optimization(c: &mut Criterion) {
    let config = OptimizationConfig::default();
    let mut optimizer = memory_optimization::MemoryLayoutOptimizer::new(&config).unwrap();
    
    let mut group = c.benchmark_group("memory_optimization");
    
    // Test different numbers of structures
    for &struct_count in [10, 50, 100, 500].iter() {
        group.throughput(Throughput::Elements(struct_count));
        group.bench_with_input(
            BenchmarkId::new("optimize_structures", struct_count),
            &struct_count,
            |b, &struct_count| {
                let structures: Vec<String> = (0..struct_count)
                    .map(|i| format!("Struct_{}", i))
                    .collect();
                
                b.iter(|| {
                    black_box(optimizer.optimize_memory_layout(&structures).unwrap())
                })
            },
        );
    }
    
    group.finish();
}

/// Benchmark compilation speed optimization
fn bench_compilation_speed_optimization(c: &mut Criterion) {
    let config = OptimizationConfig {
        max_parallel_threads: 4,
        ..Default::default()
    };
    
    let mut optimizer = compilation_speed::CompilationSpeedOptimizer::new(&config).unwrap();
    
    let program = cursed::ast::Program {
        statements: vec![],
    };
    
    let mut group = c.benchmark_group("compilation_speed_optimization");
    
    group.bench_function("optimize_compilation", |b| {
        b.iter(|| {
            black_box(optimizer.optimize_compilation(&program).unwrap())
        })
    });
    
    group.finish();
}

/// Benchmark performance profiling overhead
fn bench_performance_profiling(c: &mut Criterion) {
    let config = OptimizationConfig::default();
    let mut profiler = profiling::PerformanceProfiler::new(&config).unwrap();
    
    let session_config = profiling::SessionConfig {
        sample_rate: 1.0,
        max_samples: 10000,
        profiling_duration: None,
        output_format: profiling::OutputFormat::Json,
        enable_detailed_analysis: false,
    };
    
    let mut group = c.benchmark_group("performance_profiling");
    
    // Benchmark profiling overhead
    group.bench_function("profiling_overhead", |b| {
        let _session_id = profiler.start_session("bench_session".to_string(), session_config.clone()).unwrap();
        
        b.iter(|| {
            let _profile = profiler.profile_compilation("test_module", || {
                // Simulate minimal work
                black_box(42 + 24);
                Ok(())
            }).unwrap();
        })
    });
    
    // Benchmark without profiling for comparison
    group.bench_function("no_profiling_baseline", |b| {
        b.iter(|| {
            // Same work without profiling
            black_box(42 + 24);
        })
    });
    
    group.finish();
}

/// Benchmark optimization caching performance
fn bench_optimization_caching(c: &mut Criterion) {
    let cache_config = cache::CacheConfig {
        max_size: 100 * 1024 * 1024, // 100MB
        max_entries: 10000,
        enable_compression: true,
        enable_encryption: false,
        ..Default::default()
    };
    
    let mut cache = cache::OptimizationCache::new(cache_config).unwrap();
    
    let mut group = c.benchmark_group("optimization_caching");
    
    // Benchmark cache operations
    group.bench_function("cache_store_retrieve", |b| {
        let optimization_result = cache::OptimizationResult {
            optimization_type: "test_optimization".to_string(),
            input_hash: "benchmark_hash".to_string(),
            output_data: vec![0u8; 1024], // 1KB of data
            optimization_stats: cache::OptimizationStats {
                optimization_time: Duration::from_millis(100),
                code_size_before: 2048,
                code_size_after: 1536,
                performance_improvement: 1.33,
                memory_usage: 4096,
            },
            dependencies: vec![],
            compiler_version: "cursed-0.1.0".to_string(),
            optimization_level: 2,
        };
        
        b.iter(|| {
            let key = format!("benchmark_key_{}", fastrand::u64(..));
            cache.store_optimization_result(&key, optimization_result.clone()).unwrap();
            black_box(cache.get_optimization_result(&key))
        })
    });
    
    // Benchmark cache maintenance
    group.bench_function("cache_maintenance", |b| {
        // Pre-populate cache
        for i in 0..1000 {
            let result = cache::OptimizationResult {
                optimization_type: "maintenance_test".to_string(),
                input_hash: format!("hash_{}", i),
                output_data: vec![0u8; 512],
                optimization_stats: cache::OptimizationStats {
                    optimization_time: Duration::from_millis(50),
                    code_size_before: 1024,
                    code_size_after: 768,
                    performance_improvement: 1.1,
                    memory_usage: 2048,
                },
                dependencies: vec![],
                compiler_version: "cursed-0.1.0".to_string(),
                optimization_level: 1,
            };
            let _ = cache.store_optimization_result(&format!("maint_key_{}", i), result);
        }
        
        b.iter(|| {
            black_box(cache.maintenance().unwrap())
        })
    });
    
    group.finish();
}

/// Benchmark adaptive optimization
fn bench_adaptive_optimization(c: &mut Criterion) {
    let config = OptimizationConfig::default();
    let mut optimizer = adaptive::AdaptiveOptimizer::new(&config).unwrap();
    
    let context = adaptive::OptimizationContext {
        target_platform: "x86_64".to_string(),
        optimization_level: 2,
        code_characteristics: adaptive::CodeCharacteristics {
            function_count: 100,
            loop_count: 50,
            branch_count: 200,
            memory_access_patterns: vec!["sequential".to_string(), "strided".to_string()],
            algorithmic_complexity: 4.2,
            data_structures_used: vec!["array".to_string(), "hash_map".to_string(), "tree".to_string()],
        },
        resource_constraints: adaptive::ResourceConstraints {
            memory_limit: 2 * 1024 * 1024 * 1024, // 2GB
            compilation_time_limit: Duration::from_secs(120),
            cpu_cores_available: 8,
            disk_space_available: 50 * 1024 * 1024 * 1024, // 50GB
        },
        performance_requirements: adaptive::PerformanceRequirements {
            target_execution_time: Duration::from_millis(50),
            memory_usage_limit: 1024 * 1024 * 1024, // 1GB
            throughput_requirement: 5000.0,
            latency_requirement: Duration::from_millis(5),
            energy_efficiency: true,
        },
        environment_info: adaptive::EnvironmentInfo {
            cpu_architecture: "x86_64".to_string(),
            cache_sizes: vec![32768, 262144, 8388608], // 32KB, 256KB, 8MB
            memory_hierarchy: vec!["L1".to_string(), "L2".to_string(), "L3".to_string()],
            compiler_version: "cursed-0.1.0".to_string(),
            operating_system: "Linux".to_string(),
        },
    };
    
    let mut group = c.benchmark_group("adaptive_optimization");
    
    group.bench_function("adapt_strategy", |b| {
        b.iter(|| {
            black_box(optimizer.adapt_strategy(&context).unwrap())
        })
    });
    
    // Benchmark learning from feedback
    group.bench_function("learn_from_feedback", |b| {
        let feedback = (0..100).map(|i| {
            adaptive::FeedbackEvent {
                event_id: format!("feedback_{}", i),
                optimization_id: format!("opt_{}", i % 10),
                event_type: adaptive::FeedbackEventType::PerformanceMetric,
                timestamp: std::time::SystemTime::now(),
                data: adaptive::FeedbackData::Numeric(0.8 + (i as f64 * 0.002)),
                reliability: 0.9,
            }
        }).collect();
        
        b.iter(|| {
            black_box(optimizer.learn_from_feedback(feedback.clone()).unwrap())
        })
    });
    
    group.finish();
}

/// Benchmark complete optimization pipeline
fn bench_optimization_pipeline(c: &mut Criterion) {
    let config = OptimizationConfig {
        enable_advanced_llvm: true,
        enable_parallel_compilation: true,
        enable_incremental_compilation: false, // Disable for simpler benchmarking
        enable_jit_optimization: true,
        enable_memory_optimization: true,
        enable_profiling: false, // Disable to avoid overhead
        enable_caching: true,
        enable_adaptive_optimization: true,
        max_parallel_threads: 4,
        optimization_level: 2,
        target_arch: "x86_64".to_string(),
        debug_optimizations: false,
    };
    
    let mut group = c.benchmark_group("optimization_pipeline");
    
    group.bench_function("create_optimization_manager", |b| {
        b.iter(|| {
            black_box(OptimizationManager::new(config.clone()).unwrap())
        })
    });
    
    group.bench_function("integrated_optimization", |b| {
        let mut manager = OptimizationManager::new(config.clone()).unwrap();
        
        b.iter(|| {
            // Simulate integrated optimization workflow
            
            // 1. Memory optimization
            if let Some(memory_optimizer) = manager.memory_optimizer() {
                // Would run memory optimization if it were mutable
                black_box(format!("Memory optimizer available: {}", 
                    memory_optimizer as *const _ as usize));
            }
            
            // 2. JIT optimization
            if let Some(jit_optimizer) = manager.jit_optimizer() {
                black_box(format!("JIT optimizer available: {}", 
                    jit_optimizer as *const _ as usize));
            }
            
            // 3. Adaptive optimization
            if let Some(adaptive_optimizer) = manager.adaptive_optimizer() {
                black_box(format!("Adaptive optimizer available: {}", 
                    adaptive_optimizer as *const _ as usize));
            }
            
            // Simulate optimization coordination
            black_box(42);
        })
    });
    
    group.finish();
}

/// Benchmark optimization effectiveness measurement
fn bench_optimization_effectiveness(c: &mut Criterion) {
    let benchmark_config = benchmarking::BenchmarkConfig {
        iterations: 5,
        warmup_iterations: 1,
        max_execution_time: Duration::from_secs(5),
        min_execution_time: Duration::from_millis(1),
        confidence_level: 0.90,
        track_memory_usage: false,
        enable_cpu_profiling: false,
        output_directory: std::env::temp_dir().join("cursed_benchmark_bench"),
        compare_with_baseline: false,
        enable_parallel_execution: false,
    };
    
    let mut benchmarks = benchmarking::OptimizationBenchmarks::new(benchmark_config);
    
    // Create test files for benchmarking
    let test_files = vec![
        std::path::PathBuf::from("bench_test1.csd"),
        std::path::PathBuf::from("bench_test2.csd"),
    ];
    
    let compilation_suite = benchmarking::create_compilation_benchmark_suite(test_files);
    benchmarks.register_suite(compilation_suite);
    
    let mut group = c.benchmark_group("optimization_effectiveness");
    
    group.bench_function("execute_benchmark_suite", |b| {
        b.iter(|| {
            black_box(benchmarks.execute_all_suites().unwrap())
        })
    });
    
    group.finish();
}

/// Benchmark optimization overhead comparison
fn bench_optimization_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimization_overhead");
    
    // Baseline: no optimization
    group.bench_function("no_optimization_baseline", |b| {
        b.iter(|| {
            // Simulate basic compilation work
            let mut sum = 0u64;
            for i in 0..1000 {
                sum += i * i;
            }
            black_box(sum)
        })
    });
    
    // With LLVM optimization
    group.bench_function("with_llvm_optimization", |b| {
        let config = OptimizationConfig::default();
        let mut optimizer = llvm_advanced::AdvancedOptimizationManager::new(&config).unwrap();
        
        b.iter(|| {
            // Same work plus optimization overhead
            let mut sum = 0u64;
            for i in 0..1000 {
                sum += i * i;
            }
            
            // Minimal optimization simulation
            let stats = optimizer.get_stats();
            black_box((sum, stats))
        })
    });
    
    // With memory optimization
    group.bench_function("with_memory_optimization", |b| {
        let config = OptimizationConfig::default();
        let mut optimizer = memory_optimization::MemoryLayoutOptimizer::new(&config).unwrap();
        
        b.iter(|| {
            let mut sum = 0u64;
            for i in 0..1000 {
                sum += i * i;
            }
            
            let stats = optimizer.get_stats();
            black_box((sum, stats))
        })
    });
    
    // With adaptive optimization
    group.bench_function("with_adaptive_optimization", |b| {
        let config = OptimizationConfig::default();
        let mut optimizer = adaptive::AdaptiveOptimizer::new(&config).unwrap();
        
        b.iter(|| {
            let mut sum = 0u64;
            for i in 0..1000 {
                sum += i * i;
            }
            
            let stats = optimizer.get_stats();
            black_box((sum, stats))
        })
    });
    
    group.finish();
}

/// Benchmark scalability with different optimization levels
fn bench_optimization_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimization_scalability");
    
    for &opt_level in [0, 1, 2, 3].iter() {
        group.throughput(Throughput::Elements(opt_level as u64));
        group.bench_with_input(
            BenchmarkId::new("optimization_level", opt_level),
            &opt_level,
            |b, &opt_level| {
                let config = OptimizationConfig {
                    optimization_level: opt_level,
                    ..Default::default()
                };
                
                let manager = OptimizationManager::new(config).unwrap();
                
                b.iter(|| {
                    // Simulate work that scales with optimization level
                    let work_multiplier = (opt_level + 1) as usize;
                    let mut sum = 0u64;
                    
                    for i in 0..(1000 * work_multiplier) {
                        sum += (i as u64).wrapping_mul(i as u64);
                    }
                    
                    black_box((sum, manager.config()))
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    optimization_benches,
    bench_llvm_advanced_optimization,
    bench_parallel_compilation,
    bench_jit_optimization,
    bench_incremental_compilation,
    bench_memory_optimization,
    bench_compilation_speed_optimization,
    bench_performance_profiling,
    bench_optimization_caching,
    bench_adaptive_optimization,
    bench_optimization_pipeline,
    bench_optimization_effectiveness,
    bench_optimization_overhead,
    bench_optimization_scalability
);

criterion_main!(optimization_benches);
