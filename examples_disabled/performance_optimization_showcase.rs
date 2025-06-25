/// Performance Optimization System Showcase
/// 
/// Demonstrates the comprehensive performance optimization capabilities
/// of the CURSED compiler including advanced LLVM passes, PGO, build
/// optimization, and performance monitoring.

use cursed::optimization::comprehensive_performance_system::*;
use cursed::optimization::config::{OptimizationConfig, OptimizationLevel};
use cursed::optimization::pgo::{PgoConfig, InstrumentationMode, OptimizationStrategy};
use cursed::error::Result;
use std::path::PathBuf;
use std::time::Instant;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("🚀 CURSED Performance Optimization System Showcase");
    println!("==================================================\n");
    
    // Create LLVM context
    let context = inkwell::context::Context::create();
    
    // Demonstrate different optimization levels
    demonstrate_optimization_levels(&context).await?;
    
    // Demonstrate profile-guided optimization
    demonstrate_pgo(&context).await?;
    
    // Demonstrate build performance optimization
    demonstrate_build_optimization(&context).await?;
    
    // Demonstrate performance monitoring
    demonstrate_performance_monitoring(&context).await?;
    
    // Demonstrate comprehensive benchmarking
    demonstrate_benchmarking(&context).await?;
    
    // Demonstrate optimization recommendations
    demonstrate_optimization_recommendations(&context).await?;
    
    println!("\n✅ Performance optimization showcase completed!");
    println!("🔍 Check the generated reports and metrics for detailed analysis.");
    
    Ok(())
}

async fn demonstrate_optimization_levels(context: &inkwell::context::Context) -> Result<()> {
    println!("📊 Demonstrating Optimization Levels");
    println!("------------------------------------");
    
    let optimization_levels = [
        (OptimizationLevel::None, "O0 - No optimization"),
        (OptimizationLevel::Less, "O1 - Basic optimization"),
        (OptimizationLevel::Default, "O2 - Standard optimization"),
        (OptimizationLevel::Aggressive, "O3 - Aggressive optimization"),
        (OptimizationLevel::Size, "Os - Size optimization"),
        (OptimizationLevel::SizeAggressive, "Oz - Aggressive size optimization"),
    ];
    
    for (level, description) in optimization_levels {
        println!("\n🔧 Testing {}", description);
        
        let mut config = PerformanceConfig::default();
        config.optimization_level = level.clone();
        config.enable_function_inlining = true;
        config.enable_dead_code_elimination = true;
        config.enable_constant_propagation = true;
        config.enable_loop_optimization = true;
        
        let mut performance_system = ComprehensivePerformanceSystem::new(context, config)?;
        
        // Create test module
        let module = context.create_module(&format!("test_module_{:?}", level));
        let source_files = vec![PathBuf::from("showcase_test.csd")];
        
        let start_time = Instant::now();
        let results = performance_system.optimize_module(&module, &source_files).await?;
        let duration = start_time.elapsed();
        
        println!("  ⏱️  Compilation time: {:?}", duration);
        
        if let Some(llvm_results) = &results.llvm_optimization_results {
            println!("  🔄 Passes applied: {}", llvm_results.passes_applied);
            println!("  📈 Performance improvement: {:.2}%", llvm_results.estimated_performance_improvement);
            println!("  📉 Code size reduction: {:.2}%", llvm_results.code_size_reduction);
            println!("  🔀 Functions inlined: {}", llvm_results.functions_inlined);
            println!("  🗑️  Dead code eliminated: {}", llvm_results.dead_code_eliminated);
        }
        
        if let Some(metrics) = &results.performance_metrics {
            println!("  💾 Memory usage: {} KB", metrics.memory_usage / 1024);
            println!("  🧠 CPU usage: {:.1}%", metrics.cpu_usage_percentage);
        }
    }
    
    Ok(())
}

async fn demonstrate_pgo(context: &inkwell::context::Context) -> Result<()> {
    println!("\n\n📈 Demonstrating Profile-Guided Optimization (PGO)");
    println!("--------------------------------------------------");
    
    let mut config = PerformanceConfig::default();
    config.optimization_level = OptimizationLevel::Aggressive;
    config.enable_pgo = true;
    config.pgo_config = PgoConfig {
        enabled: true,
        profile_data_dir: PathBuf::from("showcase_pgo_profiles"),
        instrumentation_mode: InstrumentationMode::Frontend,
        optimization_strategy: OptimizationStrategy::Speed,
        hot_function_threshold: 0.1,
        enable_indirect_call_promotion: true,
        enable_value_profiling: true,
        enable_control_flow_profiling: true,
        ..PgoConfig::default()
    };
    
    let mut performance_system = ComprehensivePerformanceSystem::new(context, config)?;
    
    println!("🔬 Instrumenting code for profile collection...");
    
    let module = context.create_module("pgo_showcase_module");
    let source_files = vec![PathBuf::from("pgo_showcase.csd")];
    
    let start_time = Instant::now();
    let results = performance_system.optimize_module(&module, &source_files).await?;
    let duration = start_time.elapsed();
    
    println!("⏱️  PGO compilation time: {:?}", duration);
    
    if let Some(pgo_results) = &results.pgo_results {
        println!("📊 PGO Session ID: {}", pgo_results.session_id);
        println!("📈 Instrumentation overhead: {:.2}%", pgo_results.instrumentation_overhead);
        println!("🎯 Optimizations applied: {}", pgo_results.optimizations_applied.len());
        
        let recommendations = &pgo_results.recommendations;
        println!("🔥 Hot functions identified: {}", recommendations.hot_functions.len());
        println!("❄️  Cold functions identified: {}", recommendations.cold_functions.len());
        println!("🎯 Optimization opportunities: {}", recommendations.optimization_opportunities.len());
        
        for opportunity in &recommendations.optimization_opportunities {
            println!("  • {} - {:.1}% improvement expected", 
                    opportunity.target, opportunity.expected_improvement);
        }
    }
    
    Ok(())
}

async fn demonstrate_build_optimization(context: &inkwell::context::Context) -> Result<()> {
    println!("\n\n🏗️  Demonstrating Build Performance Optimization");
    println!("-----------------------------------------------");
    
    let mut config = PerformanceConfig::default();
    config.enable_incremental_compilation = true;
    config.enable_parallel_compilation = true;
    config.max_parallel_jobs = 4;
    config.enable_compilation_caching = true;
    config.cache_directory = PathBuf::from("showcase_cache");
    
    let mut performance_system = ComprehensivePerformanceSystem::new(context, config)?;
    
    // Simulate multiple source files
    let source_files: Vec<PathBuf> = (0..5)
        .map(|i| PathBuf::from(format!("build_showcase_{}.csd", i)))
        .collect();
    
    println!("📁 Compiling {} source files...", source_files.len());
    
    // First compilation (cold cache)
    let module = context.create_module("build_showcase_module");
    
    println!("\n🥶 Cold compilation (no cache):");
    let start_time = Instant::now();
    let results1 = performance_system.optimize_module(&module, &source_files).await?;
    let cold_time = start_time.elapsed();
    
    println!("  ⏱️  Time: {:?}", cold_time);
    println!("  💾 Cache hit: {}", results1.cache_hit);
    
    if let Some(dep_analysis) = &results1.dependency_analysis {
        println!("  📊 Files analyzed: {}", dep_analysis.files_analyzed);
        println!("  🔗 Dependencies found: {}", dep_analysis.dependencies_found);
        println!("  ♻️  Incremental possible: {}", dep_analysis.incremental_compilation_possible);
    }
    
    // Second compilation (warm cache)
    println!("\n🔥 Warm compilation (with cache):");
    let start_time = Instant::now();
    let results2 = performance_system.optimize_module(&module, &source_files).await?;
    let warm_time = start_time.elapsed();
    
    println!("  ⏱️  Time: {:?}", warm_time);
    println!("  💾 Cache hit: {}", results2.cache_hit);
    
    let speedup = cold_time.as_secs_f64() / warm_time.as_secs_f64();
    println!("  🚀 Speedup: {:.2}x", speedup);
    
    // Get build performance statistics
    let statistics = performance_system.get_performance_statistics();
    println!("\n📈 Build Performance Statistics:");
    println!("  🏗️  Total compilations: {}", statistics.total_compilations);
    println!("  ⏱️  Average time: {:?}", statistics.average_compilation_time);
    println!("  💾 Cache hit rate: {:.1}%", statistics.cache_hit_rate * 100.0);
    
    Ok(())
}

async fn demonstrate_performance_monitoring(context: &inkwell::context::Context) -> Result<()> {
    println!("\n\n📊 Demonstrating Performance Monitoring");
    println!("---------------------------------------");
    
    let mut config = PerformanceConfig::default();
    config.enable_performance_monitoring = true;
    config.collect_memory_usage = true;
    config.collect_compilation_time = true;
    
    let mut performance_system = ComprehensivePerformanceSystem::new(context, config)?;
    
    println!("🔍 Monitoring performance across multiple compilations...");
    
    // Perform several compilations to collect monitoring data
    let module = context.create_module("monitoring_showcase_module");
    
    for i in 1..=5 {
        println!("\n📊 Compilation #{}", i);
        
        let source_files = vec![PathBuf::from(format!("monitoring_test_{}.csd", i))];
        
        let start_time = Instant::now();
        let results = performance_system.optimize_module(&module, &source_files).await?;
        let duration = start_time.elapsed();
        
        println!("  ⏱️  Time: {:?}", duration);
        
        if let Some(metrics) = &results.performance_metrics {
            println!("  💾 Memory: {} MB", metrics.memory_usage / (1024 * 1024));
            println!("  🧠 CPU: {:.1}%", metrics.cpu_usage_percentage);
            println!("  🔄 Cache ops: {}", metrics.cache_operations);
            println!("  ⚙️  Parallel jobs: {}", metrics.parallel_jobs_used);
        }
    }
    
    // Get comprehensive statistics
    let statistics = performance_system.get_performance_statistics();
    
    println!("\n📈 Performance Monitoring Summary:");
    println!("  📊 Total compilations: {}", statistics.total_compilations);
    println!("  ⏱️  Average time: {:?}", statistics.average_compilation_time);
    println!("  🎯 Optimization effectiveness: {:.1}%", statistics.optimization_effectiveness * 100.0);
    
    if let Some(memory_stats) = &statistics.memory_usage_stats {
        println!("  💾 Memory usage:");
        println!("    Average: {} MB", memory_stats.average_memory_usage / (1024 * 1024));
        println!("    Peak: {} MB", memory_stats.peak_memory_usage / (1024 * 1024));
        println!("    Samples: {}", memory_stats.samples_count);
    }
    
    match statistics.recent_performance_trend {
        PerformanceTrend::Improving => println!("  📈 Trend: Improving"),
        PerformanceTrend::Stable => println!("  📊 Trend: Stable"),
        PerformanceTrend::Degrading => println!("  📉 Trend: Degrading"),
    }
    
    Ok(())
}

async fn demonstrate_benchmarking(context: &inkwell::context::Context) -> Result<()> {
    println!("\n\n🏁 Demonstrating Comprehensive Benchmarking");
    println!("-------------------------------------------");
    
    let mut config = PerformanceConfig::default();
    config.enable_benchmarking = true;
    config.benchmark_iterations = 5;
    config.benchmark_warmup_iterations = 2;
    
    let mut performance_system = ComprehensivePerformanceSystem::new(context, config)?;
    
    println!("🏃 Running comprehensive benchmarks...");
    
    let start_time = Instant::now();
    let benchmark_results = performance_system.run_benchmarks("showcase_benchmark").await?;
    let total_time = start_time.elapsed();
    
    println!("\n📊 Benchmark Results:");
    println!("  🏁 Total duration: {:?}", total_time);
    
    // Compilation benchmarks
    let comp_bench = &benchmark_results.compilation_benchmarks;
    println!("\n🏗️  Compilation Performance:");
    println!("  ⏱️  Total time: {:?}", comp_bench.total_time);
    println!("  📄 Files compiled: {}", comp_bench.files_compiled);
    println!("  ⚡ Avg time per file: {:?}", comp_bench.average_time_per_file);
    println!("  💾 Cache utilization: {:.1}%", comp_bench.cache_utilization * 100.0);
    
    // Runtime benchmarks
    let runtime_bench = &benchmark_results.runtime_benchmarks;
    println!("\n🚀 Runtime Performance:");
    println!("  ⚡ Execution time: {:?}", runtime_bench.execution_time);
    println!("  💾 Memory usage: {} MB", runtime_bench.memory_usage / (1024 * 1024));
    println!("  🧠 CPU usage: {:.1}%", runtime_bench.cpu_usage);
    println!("  📈 Throughput: {:.1} ops/sec", runtime_bench.throughput);
    
    // Optimization benchmarks
    let opt_bench = &benchmark_results.optimization_benchmarks;
    println!("\n🔧 Optimization Performance:");
    println!("  🔄 Passes executed: {}", opt_bench.passes_executed);
    println!("  📉 Code size reduction: {:.1}%", opt_bench.code_size_reduction);
    println!("  📈 Performance improvement: {:.1}%", opt_bench.performance_improvement);
    println!("  🎯 Effectiveness: {:.1}%", opt_bench.optimization_effectiveness * 100.0);
    
    // System metrics
    let sys_metrics = &benchmark_results.system_metrics;
    println!("\n💻 System Metrics:");
    println!("  💾 Total memory: {} GB", sys_metrics.total_memory / (1024 * 1024 * 1024));
    println!("  💾 Available memory: {} GB", sys_metrics.available_memory / (1024 * 1024 * 1024));
    println!("  🧠 CPU cores: {}", sys_metrics.cpu_cores);
    println!("  🔧 CPU usage: {:.1}%", sys_metrics.cpu_usage);
    
    Ok(())
}

async fn demonstrate_optimization_recommendations(context: &inkwell::context::Context) -> Result<()> {
    println!("\n\n💡 Demonstrating Optimization Recommendations");
    println!("---------------------------------------------");
    
    // Configure system with suboptimal settings to generate recommendations
    let mut config = PerformanceConfig::default();
    config.enable_pgo = false; // Disabled to trigger recommendation
    config.enable_parallel_compilation = false; // Disabled to trigger recommendation
    config.enable_compilation_caching = false; // Disabled to trigger recommendation
    
    let mut performance_system = ComprehensivePerformanceSystem::new(context, config)?;
    
    // Perform some compilations to generate data for recommendations
    let module = context.create_module("recommendations_showcase_module");
    
    for i in 1..=3 {
        let source_files = vec![PathBuf::from(format!("rec_test_{}.csd", i))];
        let _results = performance_system.optimize_module(&module, &source_files).await?;
    }
    
    println!("🔍 Analyzing performance and generating recommendations...");
    
    let recommendations = performance_system.generate_optimization_recommendations();
    
    println!("\n💡 Optimization Recommendations:");
    
    if recommendations.is_empty() {
        println!("  ✅ No recommendations - system is already well optimized!");
    } else {
        for (i, rec) in recommendations.iter().enumerate() {
            println!("\n  {}. {:?} Priority - {:?}", i + 1, rec.priority, rec.category);
            println!("     📝 {}", rec.description);
            println!("     📈 Expected improvement: {:.1}%", rec.expected_improvement);
            
            match rec.action {
                OptimizationAction::EnablePGO => {
                    println!("     🎯 Action: Enable Profile-Guided Optimization");
                }
                OptimizationAction::EnableParallelCompilation => {
                    println!("     🎯 Action: Enable parallel compilation");
                }
                OptimizationAction::AdjustCacheSettings => {
                    println!("     🎯 Action: Optimize compilation caching");
                }
                OptimizationAction::ReduceMemoryUsage => {
                    println!("     🎯 Action: Reduce memory usage");
                }
                OptimizationAction::IncreaseOptimizationLevel => {
                    println!("     🎯 Action: Increase optimization level");
                }
                OptimizationAction::EnableIncrementalCompilation => {
                    println!("     🎯 Action: Enable incremental compilation");
                }
            }
        }
        
        // Show potential cumulative improvement
        let total_improvement: f64 = recommendations.iter()
            .map(|r| r.expected_improvement)
            .sum();
        
        println!("\n🚀 Potential cumulative improvement: {:.1}%", total_improvement);
    }
    
    // Export performance data
    println!("\n📊 Exporting performance data for analysis...");
    let export_path = PathBuf::from("showcase_performance_data.json");
    performance_system.export_performance_data(&export_path)?;
    println!("  💾 Performance data exported to: {:?}", export_path);
    
    Ok(())
}
