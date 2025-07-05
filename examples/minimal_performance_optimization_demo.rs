use cursed::optimization::{
    config::{OptimizationConfig, OptimizationLevel, OptimizationProfile},
    performance_integration::{
        ImplementationEffort, IntegratedOptimizationResults, ProjectCharacteristics,
        PerformanceIntegrationSystem, PerformanceIntegrationConfig, PerformanceTargets
    },
    build_integration::{create_build_optimizer_from_args_with_performance, BuildContext},
    benchmarking::BenchmarkResults,
};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 CURSED Performance Optimization Demo");
    println!("=======================================");
    
    // Create basic configuration
    let config = OptimizationConfig::release();
    let project_root = std::env::current_dir()?;
    let output_dir = project_root.join("target");
    let target = "x86_64-unknown-linux-gnu".to_string();
    let profile = "release".to_string();
    let features = vec!["optimization".to_string()];
    let workspace_path = project_root.clone();
    let performance_config = PerformanceIntegrationConfig::default();

    // Create build optimizer
    let source_files = vec![project_root.join("src/main.rs")];
    let optimizer = create_build_optimizer_from_args_with_performance(
        project_root.clone(),
        source_files,
        Some(output_dir),
        Some(target),
        false,  // debug
        true,   // release
        false,  // verbose
        true,   // enable_performance
    )?;

    println!("✅ Build optimizer created successfully");

    // Analyze project characteristics
    let characteristics = ProjectCharacteristics::analyze_project(&project_root.to_string_lossy())?;
    
    println!("\n📊 Project Analysis:");
    println!("  📝 Lines of code: {}", characteristics.total_loc);
    println!("  🔧 Functions: {}", characteristics.function_count);
    println!("  📦 Modules: {}", characteristics.module_count);
    println!("  📚 Dependencies: {}", characteristics.dependency_count);
    println!("  🏗️  Build time estimate: {:.1}s", characteristics.typical_build_time_seconds);

    // Show implementation effort levels
    println!("\n⚡ Implementation Effort Levels:");
    match ImplementationEffort::Low {
        ImplementationEffort::Low => println!("  🟢 Low effort features available"),
        ImplementationEffort::Medium => println!("  🟡 Medium effort features available"),
        ImplementationEffort::High => println!("  🔴 High effort features available"),
    }

    // Test benchmark results
    let mut benchmark_results = BenchmarkResults::new("compilation_benchmark".to_string());
    benchmark_results.iterations = 10;
    benchmark_results.average_time = std::time::Duration::from_millis(150);
    benchmark_results.throughput = 6.67;

    println!("\n📈 Benchmark Results:");
    println!("  📊 Test: {}", benchmark_results.name);
    println!("  🔄 Iterations: {}", benchmark_results.iterations);
    println!("  ⏱️  Average time: {:.2}ms", benchmark_results.average_time.as_millis());
    println!("  🚀 Throughput: {:.2} ops/sec", benchmark_results.throughput);

    // Show optimization results
    let mut optimization_results = IntegratedOptimizationResults::new();
    optimization_results.optimization_score = 0.85;
    optimization_results.performance_improvement = 1.3;
    optimization_results.code_size_reduction = 0.9;
    optimization_results.memory_reduction = 0.8;

    println!("\n🎯 Optimization Results:");
    println!("  📊 Score: {:.1}%", optimization_results.optimization_score * 100.0);
    println!("  🚀 Performance improvement: {:.1}x", optimization_results.performance_improvement);
    println!("  📏 Code size reduction: {:.1}%", (1.0 - optimization_results.code_size_reduction) * 100.0);
    println!("  💾 Memory reduction: {:.1}%", (1.0 - optimization_results.memory_reduction) * 100.0);

    // Show recommendations
    let recommendations = characteristics.get_recommendations();
    if !recommendations.is_empty() {
        println!("\n💡 Optimization Recommendations:");
        for (i, rec) in recommendations.iter().enumerate() {
            println!("  {}. {}", i + 1, rec);
        }
    }

    println!("\n✨ Demo completed successfully!");
    Ok(())
}
