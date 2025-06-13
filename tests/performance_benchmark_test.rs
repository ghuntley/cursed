/// Performance Benchmark Tests
/// 
/// Comprehensive performance benchmarks for different optimization levels,
/// parallel compilation, and compilation pipeline performance.

#[path = "common.rs"]
pub mod common;

use cursed::codegen::llvm::optimization::{
    OptimizationManager, OptimizationLevel, OptimizationConfig
};
use cursed::profiling::performance::{
    PerformanceMonitor, CompilationPhase, ReportFormat, ReportConfig
};
use cursed::core::performance_pipeline::{
    PerformancePipeline, CompilationJob, utils as pipeline_utils
};
use inkwell::context::Context;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

/// Performance benchmark results
#[derive(Debug, Clone)]
struct BenchmarkResult {
    test_name: String,
    duration: Duration,
    optimization_level: OptimizationLevel,
    code_size_before: usize,
    code_size_after: usize,
    passes_run: usize,
}

impl BenchmarkResult {
    fn compression_ratio(&self) -> f64 {
        if self.code_size_before > 0 {
            self.code_size_after as f64 / self.code_size_before as f64
        } else {
            1.0
        }
    }
    
    fn optimization_efficiency(&self) -> f64 {
        if self.passes_run > 0 && self.duration.as_nanos() > 0 {
            self.passes_run as f64 / self.duration.as_secs_f64()
        } else {
            0.0
        }
    }
}

/// Create a test LLVM module with various constructs for optimization testing
fn create_test_module(context: &Context, complexity: usize) -> inkwell::module::Module {
    let module = context.create_module("benchmark_test");
    let builder = context.create_builder();
    
    let i32_type = context.i32_type();
    let i64_type = context.i64_type();
    let f64_type = context.f64_type();
    
    // Create functions with varying complexity
    for i in 0..complexity {
        let fn_name = format!("test_function_{}", i);
        let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
        let function = module.add_function(&fn_name, fn_type, None);
        
        let entry_bb = context.append_basic_block(function, "entry");
        let loop_bb = context.append_basic_block(function, "loop");
        let exit_bb = context.append_basic_block(function, "exit");
        
        builder.position_at_end(entry_bb);
        let param1 = function.get_nth_param(0).unwrap();
        let param2 = function.get_nth_param(1).unwrap();
        
        // Create some computation that can be optimized
        let sum = builder.build_int_add(param1.into_int_value(), param2.into_int_value(), "sum").unwrap();
        let mul = builder.build_int_mul(sum, i32_type.const_int(2, false), "mul").unwrap();
        
        // Create a loop for more optimization opportunities
        let loop_var = builder.build_alloca(i32_type, "loop_var").unwrap();
        builder.build_store(loop_var, i32_type.const_int(0, false)).unwrap();
        builder.build_unconditional_branch(loop_bb).unwrap();
        
        builder.position_at_end(loop_bb);
        let current_val = builder.build_load(i32_type, loop_var, "current").unwrap();
        let incremented = builder.build_int_add(
            current_val.into_int_value(),
            i32_type.const_int(1, false),
            "inc"
        ).unwrap();
        builder.build_store(loop_var, incremented).unwrap();
        
        let condition = builder.build_int_compare(
            inkwell::IntPredicate::ULT,
            incremented,
            i32_type.const_int(10, false),
            "cond"
        ).unwrap();
        
        builder.build_conditional_branch(condition, loop_bb, exit_bb).unwrap();
        
        builder.position_at_end(exit_bb);
        let final_result = builder.build_int_add(mul, incremented, "result").unwrap();
        builder.build_return(Some(&final_result)).unwrap();
    }
    
    // Add some global variables
    for i in 0..complexity / 2 {
        let global_name = format!("global_var_{}", i);
        module.add_global(i64_type, None, &global_name);
    }
    
    module
}

#[test]
fn benchmark_optimization_levels() {
    init_tracing!();
    
    let context = Context::create();
    let complexity = 20; // Number of functions to create
    let module = create_test_module(&context, complexity);
    
    let levels = vec![
        OptimizationLevel::None,
        OptimizationLevel::Less,
        OptimizationLevel::Default,
        OptimizationLevel::Aggressive,
        OptimizationLevel::Size,
        OptimizationLevel::SizeAggressive,
    ];
    
    let mut results = Vec::new();
    
    for level in levels {
        let config = OptimizationConfig {
            level,
            ..Default::default()
        };
        
        let mut manager = OptimizationManager::new(&context, config);
        
        // Get initial code size
        let code_before = module.print_to_string().to_string();
        let size_before = code_before.len();
        
        let start_time = Instant::now();
        
        // Initialize and optimize
        manager.initialize(&module).unwrap();
        manager.optimize_module(&module).unwrap();
        
        let duration = start_time.elapsed();
        
        // Get final code size and stats
        let code_after = module.print_to_string().to_string();
        let size_after = code_after.len();
        let stats = manager.get_stats();
        
        let result = BenchmarkResult {
            test_name: format!("optimization_level_{}", level.as_str()),
            duration,
            optimization_level: level,
            code_size_before: size_before,
            code_size_after: size_after,
            passes_run: stats.passes_run,
        };
        
        tracing::info!(
            "Level {}: {:.2}ms, {} passes, {:.1}% size reduction, {:.1} passes/sec",
            level.as_str(),
            duration.as_secs_f64() * 1000.0,
            result.passes_run,
            (1.0 - result.compression_ratio()) * 100.0,
            result.optimization_efficiency()
        );
        
        results.push(result);
    }
    
    // Print summary
    println!("\n📊 Optimization Level Benchmark Summary:");
    println!("═══════════════════════════════════════════════════════════");
    println!("{:<8} {:<10} {:<8} {:<12} {:<15}", "Level", "Time(ms)", "Passes", "Size Red(%)", "Efficiency");
    println!("───────────────────────────────────────────────────────────");
    
    for result in &results {
        println!("{:<8} {:<10.2} {:<8} {:<12.1} {:<15.1}",
            result.optimization_level.as_str(),
            result.duration.as_secs_f64() * 1000.0,
            result.passes_run,
            (1.0 - result.compression_ratio()) * 100.0,
            result.optimization_efficiency()
        );
    }
    
    // Verify that higher optimization levels generally take more time but provide better results
    let o0_result = results.iter().find(|r| r.optimization_level == OptimizationLevel::None).unwrap();
    let o3_result = results.iter().find(|r| r.optimization_level == OptimizationLevel::Aggressive).unwrap();
    
    assert!(o3_result.passes_run >= o0_result.passes_run, "O3 should run more passes than O0");
    // Note: We don't assert on duration because O0 might actually take time due to LLVM setup overhead
    
    tracing::info!("✅ Optimization level benchmarks completed successfully");
}

#[test]
fn benchmark_module_sizes() {
    init_tracing!();
    
    let context = Context::create();
    let sizes = vec![5, 10, 25, 50, 100];
    
    let mut results = Vec::new();
    
    for size in sizes {
        let module = create_test_module(&context, size);
        let config = OptimizationConfig {
            level: OptimizationLevel::Default,
            ..Default::default()
        };
        
        let mut manager = OptimizationManager::new(&context, config);
        
        let start_time = Instant::now();
        manager.initialize(&module).unwrap();
        manager.optimize_module(&module).unwrap();
        let duration = start_time.elapsed();
        
        let stats = manager.get_stats();
        
        let result = BenchmarkResult {
            test_name: format!("module_size_{}", size),
            duration,
            optimization_level: OptimizationLevel::Default,
            code_size_before: stats.code_size_before,
            code_size_after: stats.code_size_after,
            passes_run: stats.passes_run,
        };
        
        tracing::info!(
            "Size {}: {:.2}ms, {} functions, {:.1} functions/sec",
            size,
            duration.as_secs_f64() * 1000.0,
            size,
            size as f64 / duration.as_secs_f64()
        );
        
        results.push(result);
    }
    
    // Print summary
    println!("\n📊 Module Size Scaling Benchmark:");
    println!("═══════════════════════════════════════════════════");
    println!("{:<12} {:<10} {:<12} {:<15}", "Functions", "Time(ms)", "Throughput", "Linear Factor");
    println!("───────────────────────────────────────────────────");
    
    let baseline = &results[0];
    for result in &results {
        let functions = result.test_name.split('_').last().unwrap().parse::<usize>().unwrap();
        let throughput = functions as f64 / result.duration.as_secs_f64();
        let linear_factor = result.duration.as_secs_f64() / baseline.duration.as_secs_f64() / 
                           (functions as f64 / 5.0); // 5 is baseline size
        
        println!("{:<12} {:<10.2} {:<12.1} {:<15.2}",
            functions,
            result.duration.as_secs_f64() * 1000.0,
            throughput,
            linear_factor
        );
    }
    
    // Verify scaling characteristics - compilation time should scale reasonably with module size
    let small_result = &results[0];
    let large_result = &results[results.len() - 1];
    
    let size_ratio = 100.0 / 5.0; // Large size / small size
    let time_ratio = large_result.duration.as_secs_f64() / small_result.duration.as_secs_f64();
    
    // Time scaling should be roughly linear to quadratic (not exponential)
    assert!(time_ratio < size_ratio * size_ratio, 
           "Compilation time scaling should be better than O(n²)");
    
    tracing::info!("✅ Module size scaling benchmarks completed successfully");
}

#[test]
fn benchmark_performance_monitor_overhead() {
    init_tracing!();
    
    let iterations = 1000;
    
    // Benchmark without performance monitoring
    let start_time = Instant::now();
    for i in 0..iterations {
        // Simulate some work
        std::thread::sleep(Duration::from_micros(10));
        let _result = i * 2 + 1;
    }
    let baseline_duration = start_time.elapsed();
    
    // Benchmark with performance monitoring
    let monitor = PerformanceMonitor::new();
    let start_time = Instant::now();
    
    for i in 0..iterations {
        monitor.start_phase(CompilationPhase::Lexing).unwrap();
        
        // Simulate some work
        std::thread::sleep(Duration::from_micros(10));
        let _result = i * 2 + 1;
        
        monitor.record_file_processed(CompilationPhase::Lexing, 10);
        monitor.end_phase(CompilationPhase::Lexing).unwrap();
    }
    
    let monitored_duration = start_time.elapsed();
    
    let overhead_ratio = monitored_duration.as_secs_f64() / baseline_duration.as_secs_f64();
    let overhead_percentage = (overhead_ratio - 1.0) * 100.0;
    
    tracing::info!(
        "Performance monitor overhead: {:.1}% ({:.2}ms baseline, {:.2}ms monitored)",
        overhead_percentage,
        baseline_duration.as_secs_f64() * 1000.0,
        monitored_duration.as_secs_f64() * 1000.0
    );
    
    // Verify overhead is reasonable (should be less than 50% for this micro-benchmark)
    assert!(overhead_percentage < 50.0, 
           "Performance monitor overhead should be reasonable");
    
    // Test report generation performance
    let report_start = Instant::now();
    let _report = monitor.generate_report().unwrap();
    let report_duration = report_start.elapsed();
    
    tracing::info!(
        "Report generation time: {:.2}ms for {} iterations",
        report_duration.as_secs_f64() * 1000.0,
        iterations
    );
    
    // Report generation should be fast
    assert!(report_duration.as_millis() < 100, 
           "Report generation should be fast");
    
    tracing::info!("✅ Performance monitor overhead benchmarks completed successfully");
}

#[tokio::test]
async fn benchmark_parallel_compilation() {
    init_tracing!();
    
    // Create test compilation jobs
    let job_counts = vec![1, 5, 10, 20];
    
    for job_count in job_counts {
        let jobs: Vec<CompilationJob> = (0..job_count).map(|i| {
            CompilationJob {
                id: i,
                file_path: PathBuf::from(format!("test_{}.csd", i)),
                source_code: format!("sus x_{} = {};", i, i * 42),
                dependencies: Vec::new(),
                priority: 0,
            }
        }).collect();
        
        // Test with different thread counts
        let thread_counts = vec![1, 2, 4];
        
        for thread_count in thread_counts {
            let (mut parallel_config, incremental_config, progress_config) = pipeline_utils::dev_config();
            parallel_config.num_threads = thread_count;
            
            let mut pipeline = PerformancePipeline::new(
                parallel_config,
                incremental_config,
                progress_config,
            );
            
            let start_time = Instant::now();
            let results = pipeline.compile_files(jobs.clone()).await.unwrap();
            let duration = start_time.elapsed();
            
            assert_eq!(results.len(), job_count);
            
            let throughput = job_count as f64 / duration.as_secs_f64();
            
            tracing::info!(
                "Jobs: {}, Threads: {}, Time: {:.2}ms, Throughput: {:.1} jobs/sec",
                job_count,
                thread_count,
                duration.as_secs_f64() * 1000.0,
                throughput
            );
        }
    }
    
    tracing::info!("✅ Parallel compilation benchmarks completed successfully");
}

#[test]
fn benchmark_optimization_pass_combinations() {
    init_tracing!();
    
    let context = Context::create();
    let module = create_test_module(&context, 15);
    
    let configurations = vec![
        ("Vectorization Only", OptimizationConfig {
            level: OptimizationLevel::Default,
            vectorize_loops: true,
            vectorize_slp: true,
            unroll_loops: false,
            merge_functions: false,
            inline_functions: false,
            ..Default::default()
        }),
        ("Inlining Only", OptimizationConfig {
            level: OptimizationLevel::Default,
            vectorize_loops: false,
            vectorize_slp: false,
            unroll_loops: false,
            merge_functions: false,
            inline_functions: true,
            ..Default::default()
        }),
        ("Loop Optimizations", OptimizationConfig {
            level: OptimizationLevel::Default,
            vectorize_loops: true,
            vectorize_slp: false,
            unroll_loops: true,
            merge_functions: false,
            inline_functions: false,
            ..Default::default()
        }),
        ("All Optimizations", OptimizationConfig {
            level: OptimizationLevel::Default,
            vectorize_loops: true,
            vectorize_slp: true,
            unroll_loops: true,
            merge_functions: true,
            inline_functions: true,
            ..Default::default()
        }),
    ];
    
    println!("\n📊 Optimization Pass Combination Benchmark:");
    println!("═══════════════════════════════════════════════════════════");
    println!("{:<20} {:<10} {:<8} {:<12}", "Configuration", "Time(ms)", "Passes", "Size Red(%)");
    println!("───────────────────────────────────────────────────────────");
    
    for (name, config) in configurations {
        let mut manager = OptimizationManager::new(&context, config);
        
        let code_before = module.print_to_string().to_string();
        let size_before = code_before.len();
        
        let start_time = Instant::now();
        manager.initialize(&module).unwrap();
        manager.optimize_module(&module).unwrap();
        let duration = start_time.elapsed();
        
        let code_after = module.print_to_string().to_string();
        let size_after = code_after.len();
        let stats = manager.get_stats();
        
        let size_reduction = if size_before > 0 {
            (1.0 - (size_after as f64 / size_before as f64)) * 100.0
        } else {
            0.0
        };
        
        println!("{:<20} {:<10.2} {:<8} {:<12.1}",
            name,
            duration.as_secs_f64() * 1000.0,
            stats.passes_run,
            size_reduction
        );
    }
    
    tracing::info!("✅ Optimization pass combination benchmarks completed successfully");
}

#[test] 
fn benchmark_report_generation_formats() {
    init_tracing!();
    
    let monitor = PerformanceMonitor::new();
    
    // Add comprehensive timing data
    let phases = vec![
        CompilationPhase::Lexing,
        CompilationPhase::Parsing, 
        CompilationPhase::TypeChecking,
        CompilationPhase::CodeGeneration,
        CompilationPhase::LlvmCodegen,
        CompilationPhase::Optimization,
        CompilationPhase::Linking,
    ];
    
    for phase in phases {
        monitor.start_phase(phase.clone()).unwrap();
        std::thread::sleep(Duration::from_millis(5));
        monitor.record_file_processed(phase.clone(), 50 + (phase as usize) * 10);
        if phase as usize % 3 == 0 {
            monitor.record_error(phase.clone());
        }
        monitor.end_phase(phase).unwrap();
    }
    
    let formats = vec![
        ReportFormat::Table,
        ReportFormat::Json,
        ReportFormat::Csv,
        ReportFormat::Summary,
        ReportFormat::Graph,
    ];
    
    println!("\n📊 Report Generation Performance:");
    println!("═══════════════════════════════════════════════");
    println!("{:<10} {:<12} {:<15}", "Format", "Time(ms)", "Size(chars)");
    println!("───────────────────────────────────────────────");
    
    for format in formats {
        let mut config = ReportConfig::default();
        config.format = format.clone();
        
        let mut format_monitor = PerformanceMonitor::with_config(config);
        
        // Copy data to format-specific monitor
        for phase in &[
            CompilationPhase::Lexing,
            CompilationPhase::Parsing,
            CompilationPhase::TypeChecking,
        ] {
            format_monitor.start_phase(phase.clone()).unwrap();
            std::thread::sleep(Duration::from_millis(1));
            format_monitor.record_file_processed(phase.clone(), 25);
            format_monitor.end_phase(phase.clone()).unwrap();
        }
        
        let start_time = Instant::now();
        let report = format_monitor.generate_report().unwrap();
        let duration = start_time.elapsed();
        
        println!("{:<10} {:<12.2} {:<15}",
            format!("{:?}", format),
            duration.as_secs_f64() * 1000.0,
            report.len()
        );
        
        // Verify report is not empty and contains expected content
        assert!(!report.is_empty(), "Report should not be empty");
        
        match format {
            ReportFormat::Json => {
                assert!(report.starts_with('{') || report.starts_with('['), "JSON should start with {{ or [");
            }
            ReportFormat::Csv => {
                assert!(report.contains(','), "CSV should contain commas");
            }
            ReportFormat::Table | ReportFormat::Summary | ReportFormat::Graph => {
                assert!(report.contains("Lexing") || report.contains("compilation"), 
                       "Text formats should contain phase names or compilation info");
            }
        }
    }
    
    tracing::info!("✅ Report generation format benchmarks completed successfully");
}
