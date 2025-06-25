//! Comprehensive tests for the CURSED performance optimization system
//! 
//! Validates that optimizations work correctly, improve performance,
//! and don't break functionality.

use std::time::{Duration, Instant};
use std::path::PathBuf;
use cursed::optimization::{
    comprehensive_optimization_enablement::{
        ComprehensiveOptimizationSystem, ComprehensiveOptimizationConfig,
        OptimizationResults, AdaptiveOptimizationLevel,
    },
    OptimizationLevel,
};

#[test]
fn test_comprehensive_optimization_system_initialization() {
    let system = ComprehensiveOptimizationSystem::new();
    assert!(system.is_ok(), "Failed to create comprehensive optimization system");
    
    let system = system.unwrap();
    
    // Verify all optimizations are enabled by default
    assert!(system.config.enable_function_inlining);
    assert!(system.config.enable_vectorization);
    assert!(system.config.enable_loop_unrolling);
    assert!(system.config.enable_common_subexpression_elimination);
    assert!(system.config.enable_tail_call_optimization);
    assert!(system.config.enable_link_time_optimization);
    assert!(system.config.enable_interprocedural_analysis);
    assert!(system.config.enable_profile_guided_optimization);
    assert!(system.config.enable_memory_layout_optimization);
    assert!(system.config.enable_advanced_vectorization);
    assert!(system.config.enable_loop_fusion);
    assert!(system.config.enable_prefetch_insertion);
    assert!(system.config.enable_numa_optimization);
}

#[test]
fn test_optimization_level_configurations() {
    // Test O0 (Debug) configuration
    let debug_config = ComprehensiveOptimizationConfig::debug_config();
    assert_eq!(debug_config.optimization_level, OptimizationLevel::None);
    assert!(!debug_config.enable_function_inlining);
    assert!(!debug_config.enable_vectorization);
    assert!(!debug_config.enable_loop_unrolling);
    assert!(debug_config.enable_common_subexpression_elimination); // Basic CSE should remain
    assert!(debug_config.enable_parallel_compilation); // Compilation speed features should remain
    assert!(debug_config.enable_caching_mechanisms);
    
    // Test O1 (Basic) configuration
    let basic_config = ComprehensiveOptimizationConfig::basic_config();
    assert_eq!(basic_config.optimization_level, OptimizationLevel::Basic);
    assert!(basic_config.enable_function_inlining);
    assert!(basic_config.enable_vectorization);
    assert!(basic_config.enable_loop_unrolling);
    assert!(basic_config.enable_common_subexpression_elimination);
    assert!(basic_config.enable_tail_call_optimization);
    assert!(!basic_config.enable_link_time_optimization); // Should skip LTO for faster compilation
    
    // Test O2 (Standard) configuration
    let standard_config = ComprehensiveOptimizationConfig::standard_config();
    assert_eq!(standard_config.optimization_level, OptimizationLevel::Default);
    assert!(standard_config.enable_function_inlining);
    assert!(standard_config.enable_vectorization);
    assert!(standard_config.enable_link_time_optimization);
    assert!(standard_config.enable_profile_guided_optimization);
    assert!(standard_config.enable_memory_layout_optimization);
    assert!(standard_config.enable_advanced_vectorization);
    
    // Test O3 (Aggressive) configuration
    let aggressive_config = ComprehensiveOptimizationConfig::aggressive_config();
    assert_eq!(aggressive_config.optimization_level, OptimizationLevel::Aggressive);
    assert!(aggressive_config.enable_function_inlining);
    assert!(aggressive_config.enable_vectorization);
    assert!(aggressive_config.enable_loop_unrolling);
    assert!(aggressive_config.enable_link_time_optimization);
    assert!(aggressive_config.enable_numa_optimization);
    
    // Test Os (Size) configuration
    let size_config = ComprehensiveOptimizationConfig::size_config();
    assert_eq!(size_config.optimization_level, OptimizationLevel::Size);
    assert!(!size_config.enable_function_inlining); // Should disable size-increasing opts
    assert!(!size_config.enable_vectorization);
    assert!(!size_config.enable_loop_unrolling);
    assert!(size_config.enable_link_time_optimization); // LTO helps with size
    assert!(size_config.enable_common_subexpression_elimination);
}

#[test]
fn test_adaptive_optimization_selection() {
    let adaptive_levels = ComprehensiveOptimizationConfig::create_default_adaptive_levels();
    let mut adaptive_engine = cursed::optimization::comprehensive_optimization_enablement::AdaptiveOptimizationEngine::new(adaptive_levels);
    
    // Test hot path code (lots of loops and math)
    let hot_code = r#"
        lowkey (sus i = 0; i < 1000; i++) {
            periodt;
            x = x * 2 + 3;
            y = y / 4 + 1;
            z = z * x + y;
        }
        lowkey (sus j = 0; j < 500; j++) {
            periodt;
            result = result + j * 2;
        }
    "#;
    let level = adaptive_engine.analyze_and_select_optimization(hot_code);
    assert_eq!(level, OptimizationLevel::Aggressive, "Hot path code should use aggressive optimization");
    
    // Test simple code (basic assignments)
    let simple_code = r#"
        sus x = 5;
        facts y = 10;
        sus z = x + y;
        facts result = z;
    "#;
    let level = adaptive_engine.analyze_and_select_optimization(simple_code);
    assert_eq!(level, OptimizationLevel::Basic, "Simple code should use basic optimization");
    
    // Test function-intensive code
    let function_code = r#"
        slay calculate_fibonacci(n) {
            lowkey (sus i = 0; i < n; i++) {
                periodt;
                slay helper_function(i);
            }
        }
        slay helper_function(x) {
            sus result = x * x;
            slay another_function(result);
        }
    "#;
    let level = adaptive_engine.analyze_and_select_optimization(function_code);
    assert!(
        level == OptimizationLevel::Default || level == OptimizationLevel::Aggressive,
        "Function-intensive code should use default or aggressive optimization"
    );
}

#[test]
fn test_optimization_performance_improvement() {
    let mut system = ComprehensiveOptimizationSystem::new().unwrap();
    
    // Test various code patterns and verify improvements
    let test_cases = vec![
        (
            "Hot loop code",
            r#"
                lowkey (sus i = 0; i < 10000; i++) {
                    periodt;
                    x = x * 2 + 3;
                    y = y * x + 1;
                }
            "#,
            0.4, // Expected minimum improvement
        ),
        (
            "Mathematical computation",
            r#"
                sus x = 5.0;
                facts y = x * x + 2.0 * x + 1.0;
                sus z = y / (x + 1.0);
                facts result = z * z + y * y;
            "#,
            0.2, // Expected minimum improvement
        ),
        (
            "Function calls",
            r#"
                slay fibonacci(n) {
                    bestie (n <= 1) { 
                        cap n; 
                    } flex { 
                        cap slay fibonacci(n-1) + slay fibonacci(n-2); 
                    }
                }
                sus result = slay fibonacci(10);
            "#,
            0.3, // Expected minimum improvement
        ),
    ];
    
    for (name, code, min_improvement) in test_cases {
        let target_path = PathBuf::from("/tmp/test_optimization");
        let results = system.optimize_source_code(code, &target_path);
        
        assert!(results.is_ok(), "Optimization failed for {}: {:?}", name, results.err());
        
        let results = results.unwrap();
        assert!(
            results.overall_improvement >= min_improvement,
            "Insufficient optimization improvement for {}: got {:.1}%, expected >= {:.1}%",
            name,
            results.overall_improvement * 100.0,
            min_improvement * 100.0
        );
        
        println!(
            "{}: {:.1}% overall improvement ({} optimizations applied)",
            name,
            results.overall_improvement * 100.0,
            results.optimizations_applied
        );
    }
}

#[test]
fn test_parallel_compilation_efficiency() {
    let config = ComprehensiveOptimizationConfig {
        enable_parallel_compilation: true,
        max_parallel_jobs: 4,
        ..ComprehensiveOptimizationConfig::default()
    };
    
    let mut system = ComprehensiveOptimizationSystem::with_config(config).unwrap();
    
    let large_code = r#"
        slay function1() { /* some code */ }
        slay function2() { /* some code */ }
        slay function3() { /* some code */ }
        slay function4() { /* some code */ }
        slay function5() { /* some code */ }
    "#.repeat(10); // Create a larger codebase
    
    let target_path = PathBuf::from("/tmp/test_parallel");
    let results = system.optimize_source_code(&large_code, &target_path).unwrap();
    
    // Parallel compilation should show efficiency gains
    assert!(
        results.parallel_efficiency >= 0.7,
        "Parallel compilation efficiency too low: {:.1}%",
        results.parallel_efficiency * 100.0
    );
    
    assert!(
        results.compilation_time_improvement >= 0.4,
        "Parallel compilation time improvement too low: {:.1}%",
        results.compilation_time_improvement * 100.0
    );
}

#[test]
fn test_caching_mechanisms() {
    let config = ComprehensiveOptimizationConfig {
        enable_caching_mechanisms: true,
        cache_directory: Some(PathBuf::from("/tmp/cursed-cache-test")),
        ..ComprehensiveOptimizationConfig::default()
    };
    
    let mut system = ComprehensiveOptimizationSystem::with_config(config).unwrap();
    
    let code = r#"
        slay test_function() {
            sus x = 42;
            facts y = x * 2;
            cap y;
        }
    "#;
    
    let target_path = PathBuf::from("/tmp/test_caching");
    
    // First compilation - should populate cache
    let results1 = system.optimize_source_code(code, &target_path).unwrap();
    
    // Second compilation - should benefit from cache
    let results2 = system.optimize_source_code(code, &target_path).unwrap();
    
    // Cache hit rate should be high on second compilation
    assert!(
        results2.cache_hit_rate >= 0.5,
        "Cache hit rate too low: {:.1}%",
        results2.cache_hit_rate * 100.0
    );
}

#[test]
fn test_optimization_effectiveness_measurement() {
    let mut system = ComprehensiveOptimizationSystem::new().unwrap();
    
    // Perform multiple optimizations
    let test_codes = vec![
        "sus x = 5; facts y = x * 2;",
        "lowkey (sus i = 0; i < 100; i++) { periodt; x = x + 1; }",
        "slay test() { cap 42; } sus result = slay test();",
    ];
    
    for code in test_codes {
        let target_path = PathBuf::from("/tmp/test_effectiveness");
        let _ = system.optimize_source_code(code, &target_path).unwrap();
    }
    
    let stats = system.get_performance_statistics();
    
    assert!(stats.total_compilations > 0);
    assert!(stats.average_improvement > 0.0);
    assert!(stats.average_compilation_time > Duration::ZERO);
    
    // Generate performance report
    let report = system.generate_performance_report().unwrap();
    assert!(report.contains("CURSED Comprehensive Performance Optimization Report"));
    assert!(report.contains("Total Compilations:"));
    assert!(report.contains("Average Performance Improvement:"));
    assert!(report.contains("Optimizations Enabled:"));
    
    println!("Performance Report:\n{}", report);
}

#[test]
fn test_different_optimization_levels_produce_different_results() {
    let test_code = r#"
        lowkey (sus i = 0; i < 1000; i++) {
            periodt;
            x = x * 2 + 3;
            y = y + x;
        }
        slay compute(n) {
            sus result = 0;
            lowkey (sus j = 0; j < n; j++) {
                periodt;
                result = result + j * j;
            }
            cap result;
        }
    "#;
    
    let configs = vec![
        ("Debug", ComprehensiveOptimizationConfig::debug_config()),
        ("Basic", ComprehensiveOptimizationConfig::basic_config()),
        ("Standard", ComprehensiveOptimizationConfig::standard_config()),
        ("Aggressive", ComprehensiveOptimizationConfig::aggressive_config()),
        ("Size", ComprehensiveOptimizationConfig::size_config()),
    ];
    
    let mut results = Vec::new();
    
    for (name, config) in configs {
        let mut system = ComprehensiveOptimizationSystem::with_config(config).unwrap();
        let target_path = PathBuf::from("/tmp/test_levels");
        let result = system.optimize_source_code(test_code, &target_path).unwrap();
        
        results.push((name, result));
        
        println!(
            "{} optimization: {:.1}% improvement ({} optimizations)",
            name,
            result.overall_improvement * 100.0,
            result.optimizations_applied
        );
    }
    
    // Verify that more aggressive optimization levels produce better results
    let debug_improvement = results[0].1.overall_improvement;
    let basic_improvement = results[1].1.overall_improvement;
    let standard_improvement = results[2].1.overall_improvement;
    let aggressive_improvement = results[3].1.overall_improvement;
    
    assert!(
        basic_improvement > debug_improvement,
        "Basic optimization should be better than debug"
    );
    assert!(
        standard_improvement > basic_improvement,
        "Standard optimization should be better than basic"
    );
    assert!(
        aggressive_improvement > standard_improvement,
        "Aggressive optimization should be better than standard"
    );
}

#[test]
fn test_optimization_timeout_handling() {
    let config = ComprehensiveOptimizationConfig {
        optimization_timeout: Duration::from_millis(1), // Very short timeout
        ..ComprehensiveOptimizationConfig::default()
    };
    
    let mut system = ComprehensiveOptimizationSystem::with_config(config).unwrap();
    
    let complex_code = r#"
        // Complex nested code that might take time to optimize
        lowkey (sus i = 0; i < 1000; i++) {
            periodt;
            lowkey (sus j = 0; j < 100; j++) {
                periodt;
                lowkey (sus k = 0; k < 50; k++) {
                    periodt;
                    x = x * i + j - k;
                }
            }
        }
    "#.repeat(10);
    
    let target_path = PathBuf::from("/tmp/test_timeout");
    let result = system.optimize_source_code(&complex_code, &target_path);
    
    // Should complete successfully even with short timeout (graceful degradation)
    assert!(result.is_ok(), "Optimization should handle timeout gracefully");
}

#[test]
fn test_memory_layout_optimization() {
    let config = ComprehensiveOptimizationConfig {
        enable_memory_layout_optimization: true,
        ..ComprehensiveOptimizationConfig::default()
    };
    
    let mut system = ComprehensiveOptimizationSystem::with_config(config).unwrap();
    
    let memory_intensive_code = r#"
        squad Person {
            name: string,
            age: i32,
            email: string,
            phone: string,
        }
        
        sus people = [Person; 1000];
        lowkey (sus i = 0; i < 1000; i++) {
            periodt;
            people[i] = Person {
                name: "John",
                age: 25,
                email: "john@example.com",
                phone: "123-456-7890",
            };
        }
    "#;
    
    let target_path = PathBuf::from("/tmp/test_memory_layout");
    let results = system.optimize_source_code(memory_intensive_code, &target_path).unwrap();
    
    assert!(
        results.memory_usage_improvement > 0.0,
        "Memory layout optimization should show improvement"
    );
}

#[test]
fn test_vectorization_improvements() {
    let config = ComprehensiveOptimizationConfig {
        enable_vectorization: true,
        enable_advanced_vectorization: true,
        ..ComprehensiveOptimizationConfig::default()
    };
    
    let mut system = ComprehensiveOptimizationSystem::with_config(config).unwrap();
    
    let vectorizable_code = r#"
        sus array1 = [f64; 1000];
        sus array2 = [f64; 1000];
        sus result = [f64; 1000];
        
        lowkey (sus i = 0; i < 1000; i++) {
            periodt;
            array1[i] = i as f64;
            array2[i] = (i * 2) as f64;
        }
        
        lowkey (sus i = 0; i < 1000; i++) {
            periodt;
            result[i] = array1[i] * array2[i] + 1.0;
        }
    "#;
    
    let target_path = PathBuf::from("/tmp/test_vectorization");
    let results = system.optimize_source_code(vectorizable_code, &target_path).unwrap();
    
    assert!(
        results.vectorization_improvement > 0.0,
        "Vectorization should show improvement for vectorizable code"
    );
}

#[test]
fn test_link_time_optimization() {
    let config = ComprehensiveOptimizationConfig {
        enable_link_time_optimization: true,
        ..ComprehensiveOptimizationConfig::default()
    };
    
    let mut system = ComprehensiveOptimizationSystem::with_config(config).unwrap();
    
    let multi_function_code = r#"
        slay helper1(x: i32) -> i32 {
            cap x * 2;
        }
        
        slay helper2(x: i32) -> i32 {
            cap slay helper1(x) + 1;
        }
        
        slay main() -> i32 {
            sus result = 0;
            lowkey (sus i = 0; i < 100; i++) {
                periodt;
                result = result + slay helper2(i);
            }
            cap result;
        }
    "#;
    
    let target_path = PathBuf::from("/tmp/test_lto");
    let results = system.optimize_source_code(multi_function_code, &target_path).unwrap();
    
    assert!(
        results.lto_improvement > 0.0,
        "Link-time optimization should show improvement for multi-function code"
    );
}

#[test]
fn test_profile_guided_optimization_integration() {
    let config = ComprehensiveOptimizationConfig {
        enable_profile_guided_optimization: true,
        profile_data_directory: Some(PathBuf::from("/tmp/pgo-test")),
        ..ComprehensiveOptimizationConfig::default()
    };
    
    let system_result = ComprehensiveOptimizationSystem::with_config(config);
    
    // PGO system should initialize successfully
    assert!(system_result.is_ok(), "PGO system should initialize successfully");
    
    let mut system = system_result.unwrap();
    assert!(system.pgo_system.is_some(), "PGO system should be enabled");
    
    let profile_code = r#"
        slay frequently_called() {
            sus result = 0;
            lowkey (sus i = 0; i < 1000; i++) {
                periodt;
                result = result + i;
            }
            cap result;
        }
        
        slay main() {
            lowkey (sus j = 0; j < 100; j++) {
                periodt;
                sus _ = slay frequently_called();
            }
        }
    "#;
    
    let target_path = PathBuf::from("/tmp/test_pgo");
    let results = system.optimize_source_code(profile_code, &target_path).unwrap();
    
    assert!(
        results.pgo_improvement > 0.0,
        "Profile-guided optimization should show improvement"
    );
}

#[test]
fn test_regression_detection() {
    let mut system = ComprehensiveOptimizationSystem::new().unwrap();
    
    // Simulate multiple compilation rounds
    let base_code = "sus x = 5; facts y = x * 2;";
    let target_path = PathBuf::from("/tmp/test_regression");
    
    // First compilation
    let results1 = system.optimize_source_code(base_code, &target_path).unwrap();
    
    // Second compilation (should be similar)
    let results2 = system.optimize_source_code(base_code, &target_path).unwrap();
    
    // Results should be consistent (no major regression)
    let improvement_diff = (results1.overall_improvement - results2.overall_improvement).abs();
    assert!(
        improvement_diff < 0.1,
        "Optimization results should be consistent across compilations"
    );
    
    let stats = system.get_performance_statistics();
    assert!(stats.total_compilations >= 2);
}

#[test]
fn test_smart_optimization_selection() {
    let config = ComprehensiveOptimizationConfig {
        enable_smart_optimization_selection: true,
        ..ComprehensiveOptimizationConfig::default()
    };
    
    let mut system = ComprehensiveOptimizationSystem::with_config(config).unwrap();
    
    // Test different code patterns should select different optimization strategies
    let patterns = vec![
        ("simple", "sus x = 5;"),
        ("mathematical", "sus x = 5.0 * 3.14159 + 2.71828 / 1.41421;"),
        ("loop_heavy", "lowkey (sus i = 0; i < 10000; i++) { periodt; x = x + 1; }"),
        ("function_heavy", "slay f1() {} slay f2() {} slay f3() {} slay f4() {}"),
    ];
    
    let mut results = Vec::new();
    
    for (name, code) in patterns {
        let target_path = PathBuf::from(format!("/tmp/test_smart_{}", name));
        let result = system.optimize_source_code(code, &target_path).unwrap();
        results.push((name, result));
    }
    
    // Different patterns should produce different optimization strategies
    // (in practice, this would be reflected in different optimization choices)
    for (name, result) in &results {
        println!(
            "Smart optimization for {}: {:.1}% improvement",
            name,
            result.overall_improvement * 100.0
        );
    }
    
    // At minimum, all should show some improvement
    for (name, result) in &results {
        assert!(
            result.overall_improvement > 0.0,
            "Smart optimization should improve performance for {} pattern",
            name
        );
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_end_to_end_optimization_pipeline() {
        // Test the complete optimization pipeline with a realistic CURSED program
        let realistic_program = r#"
            // Fibonacci calculation with memoization
            sus memo: [i32; 100] = [0; 100];
            
            slay fibonacci(n: i32) -> i32 {
                bestie (n <= 1) {
                    cap n;
                } flex bestie (memo[n as usize] != 0) {
                    cap memo[n as usize];
                } flex {
                    sus result = slay fibonacci(n - 1) + slay fibonacci(n - 2);
                    memo[n as usize] = result;
                    cap result;
                }
            }
            
            slay main() {
                lowkey (sus i = 0; i < 20; i++) {
                    periodt;
                    sus fib = slay fibonacci(i);
                    // Process fibonacci number
                    lowkey (sus j = 0; j < fib; j++) {
                        periodt;
                        sus computation = j * j + 2 * j + 1;
                    }
                }
            }
        "#;
        
        let mut system = ComprehensiveOptimizationSystem::new().unwrap();
        let target_path = PathBuf::from("/tmp/test_end_to_end");
        let results = system.optimize_source_code(realistic_program, &target_path).unwrap();
        
        // Comprehensive program should benefit significantly from optimization
        assert!(
            results.overall_improvement >= 0.3,
            "Realistic program should show significant optimization improvement: got {:.1}%",
            results.overall_improvement * 100.0
        );
        
        assert!(
            results.optimizations_applied >= 8,
            "Multiple optimizations should be applied to realistic program"
        );
        
        // Specific optimization improvements should be present
        assert!(results.function_inlining_improvement > 0.0);
        assert!(results.loop_optimization_improvement > 0.0);
        assert!(results.vectorization_improvement > 0.0);
        
        println!("End-to-end optimization results:");
        println!("  Overall improvement: {:.1}%", results.overall_improvement * 100.0);
        println!("  Function inlining: {:.1}%", results.function_inlining_improvement * 100.0);
        println!("  Loop optimization: {:.1}%", results.loop_optimization_improvement * 100.0);
        println!("  Vectorization: {:.1}%", results.vectorization_improvement * 100.0);
        println!("  Link-time optimization: {:.1}%", results.lto_improvement * 100.0);
        println!("  Profile-guided optimization: {:.1}%", results.pgo_improvement * 100.0);
        println!("  Cache hit rate: {:.1}%", results.cache_hit_rate * 100.0);
        println!("  Parallel efficiency: {:.1}%", results.parallel_efficiency * 100.0);
        println!("  Optimizations applied: {}", results.optimizations_applied);
        
        // Generate and validate performance report
        let report = system.generate_performance_report().unwrap();
        assert!(report.len() > 0);
        assert!(report.contains("CURSED Comprehensive Performance Optimization Report"));
        
        let stats = system.get_performance_statistics();
        assert!(stats.total_compilations >= 1);
        assert!(stats.average_improvement > 0.0);
    }
}
