/// CURSED Optimization System Showcase
/// 
/// Demonstrates the comprehensive optimization capabilities of the CURSED compiler
/// including parallel compilation, incremental builds, caching, and performance analysis.

use cursed::optimization::*;
use cursed::optimization::config::*;
use cursed::optimization::profiler::*;
use cursed::optimization::analysis::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tempfile::TempDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 CURSED Optimization System Showcase");
    println!("=====================================\n");

    // Initialize tracing for better output
    tracing_subscriber::fmt()
        .with_env_filter("cursed=info,optimization_showcase=info")
        .init();

    // Create temporary directory for demo
    let temp_dir = TempDir::new()?;
    println!("📁 Working directory: {}", temp_dir.path().display());

    // Part 1: Configuration Showcase
    println!("\n🔧 Part 1: Configuration System");
    println!("================================");
    showcase_configuration(&temp_dir)?;

    // Part 2: Parallel Compilation
    println!("\n⚡ Part 2: Parallel Compilation");
    println!("===============================");
    showcase_parallel_compilation(&temp_dir)?;

    // Part 3: Incremental Builds
    println!("\n🔄 Part 3: Incremental Builds");
    println!("==============================");
    showcase_incremental_builds(&temp_dir)?;

    // Part 4: Compilation Cache
    println!("\n💾 Part 4: Compilation Cache");
    println!("============================");
    showcase_compilation_cache(&temp_dir)?;

    // Part 5: Performance Profiling
    println!("\n📈 Part 5: Performance Profiling");
    println!("=================================");
    showcase_performance_profiling(&temp_dir)?;

    // Part 6: Performance Analysis
    println!("\n📊 Part 6: Performance Analysis");
    println!("===============================");
    showcase_performance_analysis(&temp_dir)?;

    // Part 7: Optimization Passes
    println!("\n🔍 Part 7: Optimization Passes");
    println!("===============================");
    showcase_optimization_passes()?;

    // Part 8: Complete Integration
    println!("\n🎯 Part 8: Complete Integration");
    println!("===============================");
    showcase_complete_integration(&temp_dir)?;

    println!("\n🎉 Optimization System Showcase Complete!");
    println!("==========================================");
    println!("The CURSED optimization system provides:");
    println!("✅ Parallel compilation with work-stealing scheduler");
    println!("✅ Incremental builds with dependency tracking");
    println!("✅ Intelligent compilation cache with eviction policies");
    println!("✅ Comprehensive performance profiling");
    println!("✅ Detailed performance analysis and recommendations");
    println!("✅ Custom CURSED-specific optimization passes");
    println!("✅ Complete integration with CLI tools");

    Ok(())
}

fn showcase_configuration(temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating optimization configuration system...");

    // Create different optimization profiles
    let profiles = [
        ("Development", OptimizationProfile::Development),
        ("Release", OptimizationProfile::Release),
        ("Debug", OptimizationProfile::Debug),
        ("Size", OptimizationProfile::Size),
        ("Performance", OptimizationProfile::Performance),
    ];

    for (name, profile) in profiles {
        let config = profile.to_config();
        println!("📋 {} Profile:", name);
        println!("   Optimization Level: {}", config.optimization_level.as_str());
        println!("   Debug Mode: {}", config.debug_mode);
        println!("   Parallel Workers: {}", config.parallel_workers);
        println!("   Enable Profiling: {}", config.enable_profiling);
        println!("   LTO Enabled: {}", config.llvm_passes.enable_link_time_optimization);
    }

    // Test configuration from command line arguments
    let args = OptimizationArgs {
        optimization_level: Some("O3".to_string()),
        parallel_workers: Some(8),
        enable_profiling: Some(true),
        target_cpu: Some("native".to_string()),
        cache_directory: Some(temp_dir.path().to_path_buf()),
        ..Default::default()
    };

    let config = OptimizationConfig::from_args(&args)?;
    println!("\n🎛️  Custom Configuration:");
    println!("   Optimization Level: {}", config.optimization_level.as_str());
    println!("   Parallel Workers: {}", config.parallel_workers);
    println!("   Cache Directory: {}", config.cache_dir().display());
    println!("   Target CPU: {:?}", config.target_cpu);

    // Test configuration validation
    match config.validate() {
        Ok(()) => println!("   ✅ Configuration is valid"),
        Err(e) => println!("   ❌ Configuration error: {}", e),
    }

    Ok(())
}

fn showcase_parallel_compilation(temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating parallel compilation system...");

    let mut compiler = ParallelCompiler::new(4);
    println!("🔧 Created parallel compiler with {} workers", compiler.worker_count);

    // Start the compiler
    compiler.start()?;
    println!("✅ Parallel compiler started");

    // Create mock compilation jobs
    let jobs = create_mock_compilation_jobs(temp_dir, 10)?;
    println!("📝 Created {} compilation jobs", jobs.len());

    // Add jobs to the queue
    for job in jobs {
        compiler.add_job(job)?;
    }

    let stats = compiler.get_stats();
    println!("📊 Jobs queued: {}", stats.jobs_queued);

    // Wait for completion
    let start_time = Instant::now();
    let results = compiler.wait_for_completion(Some(Duration::from_secs(30)))?;
    let duration = start_time.elapsed();

    println!("⏱️  Compilation completed in {:?}", duration);
    println!("📈 Results:");
    println!("   Total jobs: {}", results.len());
    println!("   Successful: {}", results.iter().filter(|r| r.success).count());
    println!("   Failed: {}", results.iter().filter(|r| !r.success).count());

    let final_stats = compiler.get_stats();
    println!("📊 Final Statistics:");
    println!("   Jobs completed: {}", final_stats.jobs_completed);
    println!("   Jobs failed: {}", final_stats.jobs_failed);
    println!("   Total time: {:?}", final_stats.total_compilation_time);
    println!("   Worker utilization: {:.1}%", final_stats.worker_utilization * 100.0);

    compiler.stop()?;
    println!("🛑 Parallel compiler stopped");

    Ok(())
}

fn showcase_incremental_builds(temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating incremental build system...");

    let mut builder = IncrementalBuilder::new(temp_dir.path())?;
    println!("🔧 Created incremental builder");

    // Create test source files
    let source_files = create_test_source_files(temp_dir, 5)?;
    println!("📁 Created {} test source files", source_files.len());

    // Initial build detection
    let detection = builder.detect_changes(&source_files)?;
    println!("🔍 Initial change detection:");
    println!("   New files: {}", detection.files_added.len());
    println!("   Rebuild required: {}", detection.rebuild_required);

    // Update dependency information
    for (i, file) in source_files.iter().enumerate() {
        let mut dependencies = std::collections::HashSet::new();
        if i > 0 {
            dependencies.insert(source_files[i - 1].clone());
        }
        
        builder.update_dependency_info(
            file,
            dependencies,
            vec!["--optimize".to_string()],
            Some(file.with_extension("o")),
        )?;
    }

    println!("📊 Dependency graph updated");

    // Get compilation order
    let files_set: std::collections::HashSet<_> = source_files.iter().cloned().collect();
    let compilation_order = builder.get_compilation_order(&files_set)?;
    
    println!("🔢 Compilation order:");
    for (i, file) in compilation_order.iter().enumerate() {
        println!("   {}. {}", i + 1, file.file_name().unwrap().to_string_lossy());
    }

    // Simulate file modification
    std::thread::sleep(Duration::from_millis(100));
    std::fs::write(&source_files[2], "// Modified content\nfacts y = 99;")?;

    // Detect changes after modification
    let detection2 = builder.detect_changes(&source_files)?;
    println!("🔍 Change detection after modification:");
    println!("   Changed files: {}", detection2.files_changed.len());
    println!("   Dependencies affected: {}", detection2.dependencies_changed.len());

    let cache_stats = builder.get_cache_stats();
    println!("📊 Cache Statistics:");
    for (key, value) in cache_stats {
        println!("   {}: {}", key, value);
    }

    Ok(())
}

fn showcase_compilation_cache(temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating compilation cache system...");

    let mut cache = CompilationCache::new(&temp_dir.path().join("cache"))?;
    println!("🔧 Created compilation cache");

    // Create test source file
    let source_file = temp_dir.path().join("cache_test.csd");
    std::fs::write(&source_file, "facts cached_value = 42;")?;

    // Test different cache types
    let cache_types = [
        CacheType::CompiledObject,
        CacheType::LlvmIr,
        CacheType::AstSerialized,
        CacheType::PreprocessedSource,
    ];

    for cache_type in cache_types {
        let key = cache.generate_key(
            &source_file,
            &[],
            &["--optimize".to_string()],
            cache_type,
        )?;

        let test_data = format!("Test data for {:?}", cache_type);
        cache.store(
            &key,
            test_data.as_bytes(),
            &source_file,
            &[],
            &["--optimize".to_string()],
            cache_type,
        )?;

        println!("💾 Stored {} data", cache_type.file_extension());
    }

    // Test retrieval
    let key = cache.generate_key(
        &source_file,
        &[],
        &["--optimize".to_string()],
        CacheType::CompiledObject,
    )?;

    if cache.contains(&key) {
        let data = cache.retrieve(&key)?.unwrap();
        println!("📖 Retrieved data: {}", String::from_utf8_lossy(&data));
    }

    let stats = cache.get_stats();
    println!("📊 Cache Statistics:");
    for (key, value) in stats {
        println!("   {}: {}", key, value);
    }

    Ok(())
}

fn showcase_performance_profiling(_temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating performance profiling system...");

    let profiler = PerformanceProfiler::new();
    profiler.start_session("showcase_session");
    println!("🔧 Started profiling session");

    // Simulate compilation phases
    let phases = [
        ("parsing", ProfileCategory::Parsing, 50),
        ("type_checking", ProfileCategory::TypeChecking, 80),
        ("optimization", ProfileCategory::Optimization, 120),
        ("code_generation", ProfileCategory::CodeGeneration, 90),
        ("linking", ProfileCategory::Linking, 30),
    ];

    for (phase_name, category, duration_ms) in phases {
        println!("⏳ Running {} phase...", phase_name);
        
        profiler.start_timer("showcase_session", phase_name);
        std::thread::sleep(Duration::from_millis(duration_ms));
        
        let mut metadata = HashMap::new();
        metadata.insert("duration_ms".to_string(), duration_ms.to_string());
        metadata.insert("category".to_string(), format!("{:?}", category));
        
        profiler.end_timer_with_metadata("showcase_session", phase_name, category, metadata);
    }

    // Add some additional operations
    for i in 0..5 {
        let op_name = format!("file_operation_{}", i);
        profiler.start_timer("showcase_session", &op_name);
        std::thread::sleep(Duration::from_millis(10 + i * 5));
        profiler.end_timer("showcase_session", &op_name, ProfileCategory::FileIO);
    }

    println!("📊 Profiling Summary:");
    profiler.print_summary("showcase_session");

    // Get category breakdown
    let breakdown = profiler.get_category_breakdown("showcase_session");
    println!("\n📈 Category Breakdown:");
    for (category, duration) in breakdown {
        println!("   {:?}: {:?}", category, duration);
    }

    // Get slowest operations
    let slowest = profiler.get_slowest_operations("showcase_session", 3);
    println!("\n🐌 Slowest Operations:");
    for (i, op) in slowest.iter().enumerate() {
        println!("   {}. {}: {:?}", i + 1, op.name, op.duration);
    }

    let _ = profiler.end_session();
    println!("✅ Profiling session completed");

    Ok(())
}

fn showcase_performance_analysis(_temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating performance analysis system...");

    let profiler = PerformanceProfiler::new();
    let mut analyzer = PerformanceAnalyzer::new();

    profiler.start_session("analysis_demo");

    // Create a realistic compilation scenario
    profiler.record_point(
        "analysis_demo",
        "slow_parsing",
        ProfileCategory::Parsing,
        Duration::from_millis(2000), // Slow operation
        HashMap::new(),
    );

    profiler.record_point(
        "analysis_demo",
        "normal_typecheck",
        ProfileCategory::TypeChecking,
        Duration::from_millis(500),
        HashMap::new(),
    );

    profiler.record_point(
        "analysis_demo",
        "fast_codegen",
        ProfileCategory::CodeGeneration,
        Duration::from_millis(100),
        HashMap::new(),
    );

    // Analyze the performance
    let report = analyzer.analyze(&profiler, "analysis_demo")?;

    println!("📊 Analysis Report:");
    println!("   Overall Score: {:.1}/100", report.summary.overall_score);
    println!("   Total Time: {:?}", report.summary.total_compilation_time);
    println!("   Files Processed: {}", report.summary.files_processed);
    println!("   Cache Hit Rate: {:.1}%", report.summary.cache_hit_rate);

    println!("\n🚨 Bottlenecks Identified:");
    for (i, bottleneck) in report.bottlenecks.iter().enumerate() {
        println!("   {}. {} ({:.1}% impact)", 
            i + 1, bottleneck.operation, bottleneck.impact_score);
        println!("      Category: {:?}", bottleneck.category);
        println!("      Time: {:?}", bottleneck.time_spent);
    }

    println!("\n💡 Recommendations:");
    for (i, rec) in report.recommendations.iter().enumerate() {
        println!("   {}. {} ({:?} Priority)", i + 1, rec.title, rec.priority);
        println!("      Estimated Improvement: {:.1}%", rec.estimated_improvement);
        println!("      {}", rec.description);
    }

    println!("\n📈 Trends:");
    println!("   Compilation Time: {:?}", report.trends.compilation_time_trend);
    println!("   Memory Usage: {:?}", report.trends.memory_usage_trend);
    println!("   Regression Detected: {}", report.trends.regression_detected);

    Ok(())
}

fn showcase_optimization_passes() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating optimization passes system...");

    let mut passes = CursedOptimizationPasses::new();
    println!("🔧 Created optimization pass manager");

    // List available passes
    let pass_list = passes.list_passes();
    println!("📋 Available Optimization Passes:");
    for (name, description) in pass_list {
        println!("   • {}: {}", name, description);
    }

    // Create a mock AST for testing
    // Note: This would normally be a real AST from parsing
    let mut mock_ast = cursed::ast::Program {
        modules: vec![], // Empty for demo
    };

    // Run all optimization passes
    println!("\n🔄 Running optimization passes...");
    let stats = passes.run_all(&mut mock_ast)?;

    println!("📊 Optimization Statistics:");
    println!("   Passes Run: {}", stats.passes_run);
    println!("   Total Time: {:?}", stats.total_time);
    println!("   Transformations by Pass:");
    for (pass_name, count) in stats.transformations_applied {
        if count > 0 {
            println!("     {}: {}", pass_name, count);
        }
    }

    Ok(())
}

fn showcase_complete_integration(temp_dir: &TempDir) -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating complete optimization system integration...");

    // Create comprehensive configuration
    let config = OptimizationConfig {
        optimization_level: OptimizationLevel::Aggressive,
        parallel_workers: 4,
        enable_parallel: true,
        enable_incremental: true,
        enable_profiling: true,
        cache_directory: Some(temp_dir.path().join("integration_cache")),
        generate_reports: true,
        verbose_optimization: true,
        ..Default::default()
    };

    println!("🔧 Configuration:");
    println!("   Level: {}", config.optimization_level.as_str());
    println!("   Workers: {}", config.effective_workers());
    println!("   Cache: {}", config.cache_dir().display());

    // Create optimization system
    let system = Arc::new(OptimizationSystem::new(config)?);
    println!("✅ Optimization system created");

    // Start optimization session
    let session = OptimizationSession::new(system.clone(), "integration_demo".to_string());
    println!("🚀 Started optimization session: {}", session.id());

    // Simulate complete compilation workflow
    let profiler = session.system().profiler();
    
    // Phase 1: Project analysis
    profiler.start_timer(session.id(), "project_analysis");
    std::thread::sleep(Duration::from_millis(50));
    profiler.end_timer(session.id(), "project_analysis", ProfileCategory::Other);

    // Phase 2: Dependency resolution
    profiler.start_timer(session.id(), "dependency_resolution");
    std::thread::sleep(Duration::from_millis(80));
    profiler.end_timer(session.id(), "dependency_resolution", ProfileCategory::Other);

    // Phase 3: Parallel compilation
    let parallel_compiler = session.system().parallel_compiler();
    println!("⚡ Parallel compilation with {} workers", parallel_compiler.worker_count);

    // Phase 4: Incremental build check
    let incremental_builder = session.system().incremental_builder();
    println!("🔄 Incremental builder cache: {}", incremental_builder.cache_directory.display());

    // Phase 5: Cache utilization
    let cache_stats = session.system().cache_stats();
    println!("💾 Cache statistics:");
    for (key, value) in cache_stats {
        println!("     {}: {}", key, value);
    }

    // Generate final result
    let mut result = OptimizationResult::success();
    result.set_timing(session.duration());
    result.set_memory_usage(1024 * 1024 * 50); // 50MB
    result.set_cache_stats(85, 15);
    result.set_processing_stats(20, 45);
    result.set_improvements(12.5, 25.8);

    println!("\n🎯 Integration Results:");
    result.print_summary();

    println!("✅ Complete integration demonstration finished");

    Ok(())
}

// Helper functions

fn create_mock_compilation_jobs(
    temp_dir: &TempDir,
    count: usize,
) -> Result<Vec<CompilationJob>, Box<dyn std::error::Error>> {
    use cursed::optimization::parallel::{CompilationJob, JobPriority};
    use std::time::Instant;

    let mut jobs = Vec::new();
    
    for i in 0..count {
        let source_file = temp_dir.path().join(format!("file_{}.csd", i));
        let output_file = temp_dir.path().join(format!("file_{}.o", i));
        
        // Create source file
        std::fs::write(&source_file, format!("facts value_{} = {};", i, i * 10))?;
        
        let job = CompilationJob {
            id: format!("job_{}", i),
            source_path: source_file,
            output_path: output_file,
            dependencies: Vec::new(),
            priority: if i < 3 { JobPriority::High } else { JobPriority::Normal },
            compile_flags: vec!["--optimize".to_string()],
            created_at: Instant::now(),
        };
        
        jobs.push(job);
    }
    
    Ok(jobs)
}

fn create_test_source_files(
    temp_dir: &TempDir,
    count: usize,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    
    for i in 0..count {
        let file_path = temp_dir.path().join(format!("source_{}.csd", i));
        let content = format!(
            "// Source file {}\nfacts value_{} = {};\nslay display_value(val: i32) -> string {{\n    return \"{{\"}};val{{\"}}\";\n}}",
            i, i, i * 100
        );
        
        std::fs::write(&file_path, content)?;
        files.push(file_path);
    }
    
    Ok(files)
}
