/// OSR and Tiered Compilation Demonstration
/// 
/// This example demonstrates the On-Stack Replacement (OSR) and Tiered Compilation
/// features of the CURSED JIT compilation system.

use cursed::codegen::llvm::{
    jit_compilation::{JitCompilationInterface, JitCompilationConfig, create_optimized_jit_interface},
    jit_engine::CursedJitEngine,
    osr::{OSRManager, OSRConfig, DeoptimizationReason},
    tiered_compilation::{TieredCompilationManager, TieredCompilationConfig, CompilationTier},
    LlvmCodeGenerator,
};
use cursed::error::Error;
use std::time::{Duration, Instant};
use inkwell::{context::Context, OptimizationLevel};

fn main() -> Result<(), Error> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("🚀 CURSED JIT: OSR and Tiered Compilation Demo");
    println!("{}", "=".repeat(60));
    
    demo_osr_functionality()?;
    demo_tiered_compilation()?;
    demo_integrated_jit_compilation()?;
    demo_performance_analysis()?;
    
    Ok(())
}

/// Demonstrate OSR (On-Stack Replacement) functionality
fn demo_osr_functionality() -> Result<(), Error> {
    println!("\n🔄 OSR (On-Stack Replacement) Demo");
    println!("{}", "-".repeat(40));
    
    let context = Context::create();
    let config = OSRConfig {
        enable_loop_osr: true,
        enable_function_osr: true,
        osr_trigger_threshold: 10,
        enable_deoptimization: true,
        enable_speculative_optimizations: true,
        ..OSRConfig::default()
    };
    
    let mut osr_manager = OSRManager::new(&context, config);
    
    // Simulate function execution with OSR opportunities
    println!("📊 Testing OSR trigger conditions...");
    
    let function_name = "hot_loop_function";
    
    // Simulate increasing execution count
    for execution_count in 1..=20 {
        if osr_manager.should_trigger_osr(function_name, execution_count) {
            println!("✅ OSR triggered at execution count: {}", execution_count);
            break;
        } else if execution_count % 5 == 0 {
            println!("⏳ Execution count: {} (not yet ready for OSR)", execution_count);
        }
    }
    
    // Demonstrate deoptimization scenarios
    println!("\n🔀 Testing deoptimization scenarios...");
    
    let deopt_scenarios = vec![
        ("type_mismatch_func", DeoptimizationReason::TypeAssumptionViolated),
        ("speculative_func", DeoptimizationReason::SpeculativeOptimizationFailed),
        ("control_flow_func", DeoptimizationReason::ControlFlowAssumptionViolated),
        ("memory_layout_func", DeoptimizationReason::MemoryLayoutAssumptionViolated),
        ("external_dep_func", DeoptimizationReason::ExternalDependencyChanged),
    ];
    
    for (func_name, reason) in deopt_scenarios {
        osr_manager.trigger_deoptimization(func_name, reason.clone())?;
        println!("🔄 Deoptimized '{}' due to: {:?}", func_name, reason);
    }
    
    // Display OSR statistics
    let stats = osr_manager.get_stats();
    println!("\n📈 OSR Statistics:");
    println!("  Total OSR replacements: {}", stats.total_osr_replacements);
    println!("  Successful transitions: {}", stats.successful_transitions);
    println!("  Failed transitions: {}", stats.failed_transitions);
    println!("  Deoptimizations: {}", stats.deoptimizations);
    
    if stats.total_osr_replacements > 0 {
        let success_rate = (stats.successful_transitions as f64 / stats.total_osr_replacements as f64) * 100.0;
        println!("  Success rate: {:.2}%", success_rate);
    }
    
    println!("  Average preparation time: {:.2}ms", stats.avg_preparation_time.as_millis());
    println!("  Average transition time: {:.2}ms", stats.avg_transition_time.as_millis());
    
    Ok(())
}

/// Demonstrate Tiered Compilation functionality
fn demo_tiered_compilation() -> Result<(), Error> {
    println!("\n🎯 Tiered Compilation Demo");
    println!("{}", "-".repeat(40));
    
    let context = Context::create();
    let config = TieredCompilationConfig {
        enable_auto_promotion: true,
        enable_background_compilation: true,
        enable_profiling_guided_optimization: true,
        ..TieredCompilationConfig::default()
    };
    
    let mut tiered_manager = TieredCompilationManager::new(&context, config)?;
    
    // Register test functions
    let test_functions = vec![
        "interpreter_function",
        "basic_jit_function", 
        "optimized_function",
        "highly_optimized_function",
        "speculative_function",
    ];
    
    println!("📝 Registering functions for tiered compilation...");
    for function in &test_functions {
        tiered_manager.register_function(function)?;
        println!("  ✅ Registered: {}", function);
    }
    
    // Simulate different execution patterns
    println!("\n⚡ Simulating execution patterns...");
    
    let execution_patterns = vec![
        ("interpreter_function", 5, Duration::from_millis(10)),
        ("basic_jit_function", 50, Duration::from_millis(8)),
        ("optimized_function", 200, Duration::from_millis(5)),
        ("highly_optimized_function", 1000, Duration::from_millis(3)),
        ("speculative_function", 5000, Duration::from_millis(2)),
    ];
    
    for (function, executions, exec_time) in execution_patterns {
        println!("  🏃 Executing '{}' {} times...", function, executions);
        
        for _ in 0..executions {
            tiered_manager.record_execution(function, exec_time)?;
        }
        
        let current_tier = tiered_manager.get_function_tier(function);
        println!("    Current tier: {:?}", current_tier);
        
        if let Some(profile) = tiered_manager.get_function_profile(function) {
            println!("    Execution count: {}", profile.execution_count);
            println!("    Average time: {:.2}ms", profile.avg_execution_time.as_millis());
        }
    }
    
    // Demonstrate hot path detection
    println!("\n🔥 Detecting hot paths...");
    for function in &test_functions {
        let hot_paths = tiered_manager.detect_hot_paths(function)?;
        if !hot_paths.is_empty() {
            println!("  🌶️  Hot paths in '{}':", function);
            for hot_path in &hot_paths {
                println!("    - {} (frequency: {}, potential: {:.2})", 
                    hot_path.segment_id, 
                    hot_path.execution_frequency,
                    hot_path.optimization_potential
                );
            }
        }
    }
    
    // Demonstrate optimization opportunities
    println!("\n🔧 Identifying optimization opportunities...");
    for tier in [CompilationTier::BasicJIT, CompilationTier::OptimizedJIT, CompilationTier::HighlyOptimizedJIT] {
        println!("  Opportunities for {:?}:", tier);
        
        let opportunities = tiered_manager.identify_optimization_opportunities("optimized_function", tier)?;
        for opportunity in &opportunities {
            println!("    - Type: {:?}", opportunity.optimization_type);
            println!("      Improvement: {:.2}x", opportunity.potential_improvement);
            println!("      Cost: {:.2}ms", opportunity.compilation_cost.as_millis());
            println!("      Confidence: {:.2}", opportunity.confidence_score);
        }
    }
    
    // Display tiered compilation statistics
    let stats = tiered_manager.get_stats();
    println!("\n📊 Tiered Compilation Statistics:");
    println!("  Functions per tier:");
    for (tier, count) in &stats.functions_per_tier {
        println!("    {:?}: {}", tier, count);
    }
    println!("  Total promotions: {}", stats.total_promotions);
    println!("  Total demotions: {}", stats.total_demotions);
    println!("  Background compilations: {}", stats.background_compilations);
    
    Ok(())
}

/// Demonstrate integrated JIT compilation with OSR and Tiered Compilation
fn demo_integrated_jit_compilation() -> Result<(), Error> {
    println!("\n🚀 Integrated JIT Compilation Demo");
    println!("{}", "-".repeat(40));
    
    let context = Context::create();
    
    // Create JIT interface with OSR and tiered compilation enabled
    let mut interface = create_optimized_jit_interface(&context)?;
    
    // Override config for demo purposes
    let mut config = interface.get_config().clone();
    config.enable_osr = true;
    config.enable_tiered_compilation = true;
    config.hot_path_threshold = 10;
    config.enable_dynamic_recompilation = true;
    config.enable_background_compilation = true;
    
    interface.update_config(config);
    
    println!("⚙️  Configuration:");
    println!("  OSR enabled: {}", interface.get_config().enable_osr);
    println!("  Tiered compilation enabled: {}", interface.get_config().enable_tiered_compilation);
    println!("  Hot path threshold: {}", interface.get_config().hot_path_threshold);
    
    // Compile test functions
    let functions = vec![
        ("fibonacci", ""),
        ("factorial", ""),
        ("matrix_multiply", ""),
        ("string_processing", ""),
        ("numerical_computation", ""),
    ];
    
    println!("\n📝 Compiling functions...");
    for (function, source) in &functions {
        let start_time = Instant::now();
        interface.compile_function(function, source)?;
        let compile_time = start_time.elapsed();
        println!("  ✅ Compiled '{}' in {:.2}ms", function, compile_time.as_millis());
    }
    
    // Execute functions with different patterns
    println!("\n⚡ Executing functions with different patterns...");
    
    // Cold functions (few executions)
    for _ in 0..3 {
        interface.execute_function("fibonacci")?;
    }
    println!("  ❄️  'fibonacci' executed 3 times (cold)");
    
    // Warm functions (moderate executions)
    for _ in 0..15 {
        interface.execute_function("factorial")?;
    }
    println!("  🔥 'factorial' executed 15 times (warm)");
    
    // Hot functions (many executions)
    for i in 0..50 {
        interface.execute_function("matrix_multiply")?;
        
        // Show tier progression
        if i == 9 || i == 24 || i == 49 {
            let tier = interface.get_tiered_manager().get_function_tier("matrix_multiply");
            println!("    Tier after {} executions: {:?}", i + 1, tier);
        }
    }
    println!("  🌶️  'matrix_multiply' executed 50 times (hot)");
    
    // Very hot functions (intensive executions)
    for _ in 0..100 {
        interface.execute_function("numerical_computation")?;
    }
    println!("  🔥🔥 'numerical_computation' executed 100 times (very hot)");
    
    // Test OSR preparation
    println!("\n🔄 Testing OSR preparation...");
    interface.prepare_osr_for_function("matrix_multiply")?;
    println!("  ✅ OSR prepared for 'matrix_multiply'");
    
    // Generate comprehensive report
    println!("\n📊 Comprehensive Performance Report:");
    let report = interface.generate_comprehensive_report();
    println!("{}", report);
    
    Ok(())
}

/// Demonstrate performance analysis and optimization
fn demo_performance_analysis() -> Result<(), Error> {
    println!("\n📈 Performance Analysis Demo");
    println!("{}", "-".repeat(40));
    
    let context = Context::create();
    let mut interface = create_optimized_jit_interface(&context)?;
    
    // Set up for performance testing
    let mut config = interface.get_config().clone();
    config.hot_path_threshold = 5;
    config.enable_performance_monitoring = true;
    interface.update_config(config);
    
    let benchmark_function = "performance_test_function";
    interface.compile_function(benchmark_function, "")?;
    
    // Perform performance measurement
    println!("🏃 Running performance benchmarks...");
    
    let measurements = vec![10, 50, 100, 200, 500];
    
    for &iterations in &measurements {
        let start_time = Instant::now();
        
        for _ in 0..iterations {
            interface.execute_function(benchmark_function)?;
        }
        
        let total_time = start_time.elapsed();
        let avg_time = total_time / iterations;
        
        let tier = interface.get_tiered_manager().get_function_tier(benchmark_function);
        
        println!("  📊 {} iterations: total={:.2}ms, avg={:.3}ms, tier={:?}", 
            iterations, 
            total_time.as_millis(), 
            avg_time.as_micros() as f64 / 1000.0,
            tier
        );
    }
    
    // Test function profiling
    println!("\n🔍 Function profiling...");
    let avg_time = interface.profile_function_execution(benchmark_function, 100)?;
    println!("  Average execution time over 100 iterations: {:.3}ms", avg_time.as_micros() as f64 / 1000.0);
    
    // Check hot paths
    let hot_paths = interface.get_hot_paths();
    if !hot_paths.is_empty() {
        println!("\n🔥 Hot paths detected:");
        for (i, path) in hot_paths.iter().enumerate() {
            let tier = interface.get_tiered_manager().get_function_tier(path);
            println!("  {}. {} (tier: {:?})", i + 1, path, tier);
        }
    }
    
    // Final statistics
    let stats = interface.get_stats();
    println!("\n📈 Final Statistics:");
    println!("  Total JIT compilations: {}", stats.total_jit_compilations);
    println!("  Hot path optimizations: {}", stats.hot_path_optimizations);
    println!("  Background compilations: {}", stats.background_compilations);
    println!("  Average compilation time: {:.2}ms", stats.avg_compilation_time.as_millis());
    
    if stats.osr_stats.total_osr_replacements > 0 {
        println!("  OSR replacements: {}", stats.osr_stats.total_osr_replacements);
        let success_rate = (stats.osr_stats.successful_transitions as f64 / stats.osr_stats.total_osr_replacements as f64) * 100.0;
        println!("  OSR success rate: {:.2}%", success_rate);
    }
    
    println!("  Tier promotions: {}", stats.tiered_stats.total_promotions);
    println!("  Tier demotions: {}", stats.tiered_stats.total_demotions);
    
    // Performance improvement analysis
    if stats.performance_improvement_percent > 0.0 {
        println!("  Performance improvement: {:.2}%", stats.performance_improvement_percent);
    }
    
    Ok(())
}

/// Helper function to demonstrate configuration scenarios
#[allow(dead_code)]
fn demo_configuration_scenarios() -> Result<(), Error> {
    println!("\n⚙️  Configuration Scenarios Demo");
    println!("{}", "-".repeat(40));
    
    let context = Context::create();
    
    // Scenario 1: Development configuration
    println!("🛠️  Development Configuration:");
    let _dev_interface = cursed::codegen::llvm::jit_compilation::create_debug_jit_interface(&context)?;
    println!("  - OSR disabled for predictable debugging");
    println!("  - Tiered compilation disabled");
    println!("  - Lower optimization levels");
    println!("  - Detailed logging enabled");
    
    // Scenario 2: Production configuration
    println!("\n🚀 Production Configuration:");
    let _prod_interface = cursed::codegen::llvm::jit_compilation::create_optimized_jit_interface(&context)?;
    println!("  - OSR enabled for maximum performance");
    println!("  - Tiered compilation enabled");
    println!("  - Aggressive optimization levels");
    println!("  - Background compilation enabled");
    
    // Scenario 3: Custom configuration
    println!("\n🎛️  Custom Configuration:");
    let jit_engine = CursedJitEngine::new_with_default_config(&context)?;
    let codegen = LlvmCodeGenerator::new()?;
    
    let custom_config = JitCompilationConfig {
        hot_path_threshold: 25,
        enable_osr: true,
        enable_tiered_compilation: true,
        enable_dynamic_recompilation: true,
        enable_background_compilation: false, // Disable for control
        hot_path_optimization_level: OptimizationLevel::Aggressive,
        regular_optimization_level: OptimizationLevel::Default,
        compilation_timeout: Duration::from_secs(10),
        max_parallel_compilations: 2,
        enable_pgo: true,
        ..JitCompilationConfig::default()
    };
    
    let _custom_interface = JitCompilationInterface::new(&context, jit_engine, codegen, custom_config)?;
    println!("  - Custom hot path threshold: 25");
    println!("  - OSR enabled, background compilation disabled");
    println!("  - Profile-guided optimization enabled");
    println!("  - Limited parallel compilations: 2");
    
    Ok(())
}

/// Stress test for the system
#[allow(dead_code)]
fn stress_test() -> Result<(), Error> {
    println!("\n💪 Stress Test");
    println!("{}", "-".repeat(40));
    
    let context = Context::create();
    let mut interface = create_optimized_jit_interface(&context)?;
    
    // Create many functions
    let function_count = 20;
    let execution_count = 100;
    
    println!("📝 Creating {} functions...", function_count);
    for i in 0..function_count {
        let function_name = format!("stress_test_function_{}", i);
        interface.compile_function(&function_name, "")?;
    }
    
    println!("⚡ Executing each function {} times...", execution_count);
    let start_time = Instant::now();
    
    for i in 0..function_count {
        let function_name = format!("stress_test_function_{}", i);
        for _ in 0..execution_count {
            interface.execute_function(&function_name)?;
        }
    }
    
    let total_time = start_time.elapsed();
    let total_executions = function_count * execution_count;
    let avg_time = total_time / total_executions as u32;
    
    println!("✅ Stress test completed:");
    println!("  Total functions: {}", function_count);
    println!("  Total executions: {}", total_executions);
    println!("  Total time: {:.2}s", total_time.as_secs_f64());
    println!("  Average time per execution: {:.3}ms", avg_time.as_micros() as f64 / 1000.0);
    
    let stats = interface.get_stats();
    println!("  Tier promotions during stress test: {}", stats.tiered_stats.total_promotions);
    println!("  OSR transitions during stress test: {}", stats.osr_stats.total_osr_replacements);
    
    Ok(())
}
