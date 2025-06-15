/// Profile-Guided Optimization (PGO) Example
/// 
/// Demonstrates how to use the CURSED compiler's PGO system for optimizing
/// performance-critical code through profile collection and analysis.

use cursed::optimization::pgo::{
    PgoManager, PgoConfig, InstrumentationMode, CollectionMode, 
    OptimizationStrategy, ProfileData
};
use cursed::optimization::optimization_manager::OptimizationManager;
use cursed::optimization::config::{OptimizationConfig, OptimizationLevel};
use std::path::PathBuf;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 CURSED Profile-Guided Optimization (PGO) Example");
    println!("========================================================");
    
    // Example CURSED source code with different performance characteristics
    let source_code = r#"
// Hot function - called frequently in tight loops
slay compute_fibonacci(n: i32) -> i32 {
    lowkey (n <= 1) {
        return n;
    }
    return compute_fibonacci(n - 1) + compute_fibonacci(n - 2);
}

// Medium function - called moderately  
slay process_array(arr: [i32; 100]) -> i32 {
    sus sum = 0;
    lowkey (sus i = 0; i < 100; i++) {
        sum += arr[i] * 2;
        // Vectorizable loop - good candidate for SIMD
    }
    return sum;
}

// Cold function - rarely called
slay error_handler(error_code: i32) {
    println("Error occurred: {}", error_code);
    // This should be optimized for size, not speed
}

// Main function with representative workload
slay main() {
    // Hot path - compute fibonacci many times
    lowkey (sus i = 0; i < 1000; i++) {
        facts fib = compute_fibonacci(15);
        
        // Medium usage - process arrays occasionally  
        lowkey (i % 10 == 0) {
            sus test_array: [i32; 100];
            lowkey (sus j = 0; j < 100; j++) {
                test_array[j] = j;
            }
            facts result = process_array(test_array);
        }
        
        // Cold path - error handling rarely triggered
        lowkey (i % 500 == 0) {
            error_handler(404);
        }
    }
}
"#;

    // Step 1: Create PGO configuration
    println!("\n📋 Step 1: Configure PGO System");
    let pgo_config = PgoConfig {
        enabled: true,
        profile_data_dir: PathBuf::from("example_pgo_profiles"),
        instrumentation_mode: InstrumentationMode::Frontend,
        collection_mode: CollectionMode::CountersAndSampling,
        optimization_strategy: OptimizationStrategy::Balanced,
        hot_function_threshold: 0.1,  // Functions using >10% of execution time
        cold_function_threshold: 0.01, // Functions using <1% of execution time
        min_execution_count: 10,
        enable_indirect_call_promotion: true,
        enable_value_profiling: true,
        enable_control_flow_profiling: true,
        ..Default::default()
    };
    
    println!("✓ PGO enabled with balanced optimization strategy");
    println!("✓ Frontend instrumentation mode selected");
    println!("✓ Hot threshold: {}%, Cold threshold: {}%", 
             pgo_config.hot_function_threshold * 100.0,
             pgo_config.cold_function_threshold * 100.0);

    // Step 2: Create PGO Manager
    println!("\n🔧 Step 2: Initialize PGO Manager");
    let mut pgo_manager = PgoManager::new(pgo_config)?;
    println!("✓ PGO manager created successfully");

    // Step 3: Start profiling session
    println!("\n📊 Step 3: Start Profile Collection Session");
    let session_id = pgo_manager.start_session(Some("fibonacci_example".to_string()))?;
    println!("✓ Started PGO session: {}", session_id);

    // Step 4: Generate instrumented code
    println!("\n🔨 Step 4: Generate Instrumented Code");
    let instrumented_code = pgo_manager.generate_instrumented_code(source_code, "main")?;
    println!("✓ Source code instrumented for profiling");
    println!("  Original size: {} bytes", source_code.len());
    println!("  Instrumented size: {} bytes", instrumented_code.len());
    println!("  Size increase: {:.1}%", 
             (instrumented_code.len() as f64 / source_code.len() as f64 - 1.0) * 100.0);

    // Step 5: Simulate profile collection (in real usage, you'd compile and run the instrumented code)
    println!("\n⚡ Step 5: Simulate Profile Data Collection");
    simulate_profile_collection(&mut pgo_manager, &session_id)?;

    // Step 6: Stop profiling and collect data
    println!("\n📈 Step 6: Stop Profiling and Collect Data");
    let pgo_session = pgo_manager.stop_session()?;
    println!("✓ Profile collection completed");
    println!("  Session duration: {:?}", pgo_session.start_time.elapsed());
    println!("  Session status: {:?}", pgo_session.status);

    // Step 7: Analyze profile data and generate recommendations
    println!("\n🔍 Step 7: Analyze Profile Data");
    let recommendations = pgo_manager.analyze_and_recommend(&session_id)?;
    
    println!("✓ Profile analysis completed");
    println!("  Hot functions identified: {}", recommendations.hot_functions.len());
    println!("  Cold functions identified: {}", recommendations.cold_functions.len());
    println!("  Optimization opportunities: {}", recommendations.optimization_opportunities.len());

    // Display hot functions
    if !recommendations.hot_functions.is_empty() {
        println!("\n🔥 Hot Functions (candidates for aggressive optimization):");
        for (i, hot_func) in recommendations.hot_functions.iter().take(5).enumerate() {
            println!("  {}. {} - executed {} times ({:.1}% of total time)",
                     i + 1, hot_func.name, hot_func.execution_count, hot_func.time_percentage);
        }
    }

    // Display cold functions
    if !recommendations.cold_functions.is_empty() {
        println!("\n❄️  Cold Functions (candidates for size optimization):");
        for (i, cold_func) in recommendations.cold_functions.iter().take(5).enumerate() {
            println!("  {}. {}", i + 1, cold_func);
        }
    }

    // Display optimization opportunities
    if !recommendations.optimization_opportunities.is_empty() {
        println!("\n💡 Optimization Opportunities:");
        for (i, opportunity) in recommendations.optimization_opportunities.iter().take(5).enumerate() {
            println!("  {}. {} on {} - Expected improvement: {:.1}% (confidence: {:.0}%)",
                     i + 1, 
                     opportunity.optimization_type.to_string(),
                     opportunity.target,
                     opportunity.expected_improvement,
                     opportunity.confidence * 100.0);
        }
    }

    // Display recommended compiler flags
    if !recommendations.recommended_flags.is_empty() {
        println!("\n🎯 Recommended Compiler Flags:");
        for flag in &recommendations.recommended_flags {
            println!("  {}", flag);
        }
    }

    // Step 8: Integrate with optimization manager
    println!("\n🚄 Step 8: Apply Optimizations with Optimization Manager");
    let opt_config = OptimizationConfig {
        optimization_level: OptimizationLevel::Aggressive,
        enable_profiling: true,
        profile_data_dir: Some(PathBuf::from("example_pgo_profiles")),
        profile_guided: true,
        ..Default::default()
    };

    let mut optimization_manager = OptimizationManager::new(opt_config)?;
    let optimization_result = optimization_manager.optimize_complete(source_code)?;

    println!("✓ Complete optimization workflow finished");
    println!("  Optimization passes applied: {}", optimization_result.passes_applied.len());
    println!("  Performance improvement: {:.1}%", optimization_result.performance_improvement);
    println!("  Optimization time: {:?}", optimization_result.optimization_time);
    println!("  Success: {}", optimization_result.success);

    if !optimization_result.passes_applied.is_empty() {
        println!("\n📋 Applied Optimization Passes:");
        for (i, pass) in optimization_result.passes_applied.iter().enumerate() {
            println!("  {}. {}", i + 1, pass);
        }
    }

    // Step 9: Show PGO statistics
    println!("\n📊 Step 9: PGO Performance Statistics");
    let pgo_stats = pgo_manager.get_statistics();
    println!("  Sessions completed: {}", pgo_stats.sessions_completed);
    println!("  Total optimizations applied: {}", pgo_stats.total_optimizations_applied);
    println!("  Average performance improvement: {:.1}%", pgo_stats.average_performance_improvement);
    println!("  Profile data size: {} bytes", pgo_stats.profile_data_size);
    println!("  Instrumentation overhead: {:.2}%", pgo_stats.instrumentation_overhead);

    // Step 10: Demonstrate different optimization strategies
    println!("\n🎛️  Step 10: Demonstration of Different Optimization Strategies");
    
    let strategies = [
        ("Speed-Focused", OptimizationStrategy::Speed),
        ("Size-Focused", OptimizationStrategy::Size),
        ("Balanced", OptimizationStrategy::Balanced),
        ("Custom", OptimizationStrategy::Custom {
            speed_weight: 0.6,
            size_weight: 0.3,
            compilation_time_weight: 0.1,
            power_weight: 0.0,
        }),
    ];

    for (name, strategy) in &strategies {
        println!("\n  {} Strategy:", name);
        
        let strategy_config = PgoConfig {
            enabled: true,
            optimization_strategy: strategy.clone(),
            profile_data_dir: PathBuf::from("strategy_test"),
            ..Default::default()
        };

        if let Ok(strategy_manager) = PgoManager::new(strategy_config) {
            let strategy_recs = strategy_manager.analyze_and_recommend(&session_id);
            if let Ok(recs) = strategy_recs {
                println!("    Optimization opportunities: {}", recs.optimization_opportunities.len());
                if let Some(first_opp) = recs.optimization_opportunities.first() {
                    println!("    Primary recommendation: {} on {}",
                             first_opp.optimization_type.to_string(),
                             first_opp.target);
                }
            }
        }
    }

    // Cleanup
    println!("\n🧹 Cleanup");
    let _ = std::fs::remove_dir_all("example_pgo_profiles");
    let _ = std::fs::remove_dir_all("strategy_test");
    println!("✓ Cleaned up temporary files");

    println!("\n🎉 PGO Example Completed Successfully!");
    println!("\nKey Takeaways:");
    println!("- PGO can provide significant performance improvements (10-30% typical)");
    println!("- Hot functions benefit from aggressive optimizations (inlining, vectorization)");
    println!("- Cold functions should be optimized for size to reduce binary bloat");
    println!("- Profile-guided decisions are more effective than static analysis alone");
    println!("- Different strategies can be used based on optimization goals");

    Ok(())
}

/// Simulate profile data collection by creating realistic execution profiles
fn simulate_profile_collection(pgo_manager: &mut PgoManager, session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("  Simulating execution with representative workload...");
    
    // Simulate profile collection timing
    let start_time = Instant::now();
    
    // Create realistic profile data that matches our source code
    let mut profile_data = ProfileData::new();
    
    // Hot function: compute_fibonacci - called frequently
    profile_data.add_function_execution("compute_fibonacci".to_string(), 15000); // Called many times
    
    // Medium function: process_array - called occasionally  
    profile_data.add_function_execution("process_array".to_string(), 100); // Called 100 times (1000/10)
    
    // Cold function: error_handler - called rarely
    profile_data.add_function_execution("error_handler".to_string(), 2); // Called 2 times (1000/500)
    
    // Main function
    profile_data.add_function_execution("main".to_string(), 1);
    
    // Add basic block execution data (simulated)
    profile_data.add_basic_block_execution("compute_fibonacci:entry".to_string(), 15000);
    profile_data.add_basic_block_execution("compute_fibonacci:recursive_case".to_string(), 12000);
    profile_data.add_basic_block_execution("compute_fibonacci:base_case".to_string(), 3000);
    
    profile_data.add_basic_block_execution("process_array:loop".to_string(), 10000); // 100 * 100 iterations
    profile_data.add_basic_block_execution("main:fibonacci_loop".to_string(), 1000);
    profile_data.add_basic_block_execution("main:array_processing".to_string(), 100);
    profile_data.add_basic_block_execution("main:error_handling".to_string(), 2);
    
    // Add edge execution data (control flow)
    profile_data.add_edge_execution("main:fibonacci_loop->compute_fibonacci".to_string(), 1000);
    profile_data.add_edge_execution("main:array_processing->process_array".to_string(), 100);
    profile_data.add_edge_execution("main:error_handling->error_handler".to_string(), 2);
    
    // Add some execution time
    profile_data.total_execution_time = Duration::from_millis(2500); // 2.5 seconds execution
    
    // Save the simulated profile data
    pgo_manager.save_profile_data(session_id, &profile_data)?;
    
    let collection_time = start_time.elapsed();
    println!("  ✓ Profile data collection simulated in {:?}", collection_time);
    println!("  ✓ Functions profiled: {}", profile_data.function_counts.len());
    println!("  ✓ Basic blocks profiled: {}", profile_data.basic_block_counts.len());
    println!("  ✓ Edges profiled: {}", profile_data.edge_counts.len());
    println!("  ✓ Total execution events: {}", profile_data.total_function_executions());
    
    Ok(())
}

// Helper trait for displaying optimization types
trait OptimizationTypeDisplay {
    fn to_string(&self) -> String;
}

impl OptimizationTypeDisplay for cursed::optimization::pgo::OptimizationType {
    fn to_string(&self) -> String {
        match self {
            cursed::optimization::pgo::OptimizationType::FunctionInlining => "Function Inlining".to_string(),
            cursed::optimization::pgo::OptimizationType::LoopOptimization => "Loop Optimization".to_string(),
            cursed::optimization::pgo::OptimizationType::VectorizationOptimization => "Vectorization".to_string(),
            cursed::optimization::pgo::OptimizationType::BranchPrediction => "Branch Prediction".to_string(),
            cursed::optimization::pgo::OptimizationType::IndirectCallPromotion => "Indirect Call Promotion".to_string(),
            cursed::optimization::pgo::OptimizationType::ValueSpecialization => "Value Specialization".to_string(),
            cursed::optimization::pgo::OptimizationType::CodeLayout => "Code Layout".to_string(),
            cursed::optimization::pgo::OptimizationType::RegisterAllocation => "Register Allocation".to_string(),
            cursed::optimization::pgo::OptimizationType::DeadCodeElimination => "Dead Code Elimination".to_string(),
            cursed::optimization::pgo::OptimizationType::ConstantPropagation => "Constant Propagation".to_string(),
        }
    }
}
