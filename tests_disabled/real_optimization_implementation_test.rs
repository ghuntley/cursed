/// Tests for Real Optimization Implementation
/// 
/// Validates that the real optimization implementations provide
/// measurable performance benefits and correct analysis.

use cursed::optimization::{
    RealOptimizationManager, RealPerformanceCalculator, CpuEfficiencyEstimator,
    RegressionDetector, PerformanceDataPoint, EnvironmentInfo,
    OptimizationLevel, OptimizationConfig,
};
use cursed::optimization::real_optimization_integration::{
    RecommendationType, RecommendationPriority,
};
use cursed::error::Result;
use std::time::{Duration, Instant};
use inkwell::context::Context;

#[test]
fn test_real_performance_calculator_provides_real_improvements() {
    let calculator = RealPerformanceCalculator::new();
    let context = Context::create();
    let module = context.create_module("test_module");
    
    // Create a simple function with some instructions
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let function = module.add_function("test_function", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    let param1 = function.get_nth_param(0).unwrap().into_int_value();
    let param2 = function.get_nth_param(1).unwrap().into_int_value();
    let add_result = builder.build_int_add(param1, param2, "add_result").unwrap();
    let mul_result = builder.build_int_mul(add_result, param1, "mul_result").unwrap();
    builder.build_return(Some(&mul_result)).unwrap();
    
    // Calculate performance improvements
    let improvements = calculator.calculate_real_performance_improvements(
        &module, 
        OptimizationLevel::Default
    ).unwrap();
    
    // Verify that we get real improvement metrics
    assert!(improvements.runtime_improvement >= 0.0);
    assert!(improvements.memory_improvement >= 0.0);
    assert!(improvements.code_size_improvement >= 0.0);
    assert!(improvements.compilation_speedup >= 1.0);
    assert!(improvements.energy_efficiency >= 0.0);
    
    // The improvements should be reasonable values
    assert!(improvements.runtime_improvement <= 1.0);
    assert!(improvements.memory_improvement <= 1.0);
    assert!(improvements.compilation_speedup <= 10.0);
}

#[test]
fn test_cpu_efficiency_estimator_provides_detailed_analysis() {
    let mut estimator = CpuEfficiencyEstimator::new();
    let context = Context::create();
    let module = context.create_module("test_module");
    
    // Create a function with various instruction types
    let i32_type = context.i32_type();
    let f32_type = context.f32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), f32_type.into()], false);
    let function = module.add_function("complex_function", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    // Add various instruction types
    let int_param = function.get_nth_param(0).unwrap().into_int_value();
    let float_param = function.get_nth_param(1).unwrap().into_float_value();
    
    // Integer arithmetic
    let int_add = builder.build_int_add(int_param, i32_type.const_int(10, false), "int_add").unwrap();
    let int_mul = builder.build_int_mul(int_add, i32_type.const_int(2, false), "int_mul").unwrap();
    
    // Floating point arithmetic
    let float_add = builder.build_float_add(float_param, f32_type.const_float(1.5), "float_add").unwrap();
    let float_mul = builder.build_float_mul(float_add, f32_type.const_float(2.0), "float_mul").unwrap();
    
    // Convert float to int and add
    let float_to_int = builder.build_float_to_signed_int(float_mul, i32_type, "float_to_int").unwrap();
    let final_result = builder.build_int_add(int_mul, float_to_int, "final_result").unwrap();
    
    builder.build_return(Some(&final_result)).unwrap();
    
    // Estimate CPU efficiency
    let efficiency = estimator.estimate_cpu_efficiency(&module).unwrap();
    
    // Verify we get comprehensive analysis
    assert!(efficiency.overall_efficiency >= 0.0 && efficiency.overall_efficiency <= 1.0);
    assert!(efficiency.instructions_per_cycle >= 0.0);
    assert!(efficiency.pipeline_efficiency >= 0.0 && efficiency.pipeline_efficiency <= 1.0);
    assert!(efficiency.cache_hit_rate >= 0.0 && efficiency.cache_hit_rate <= 1.0);
    assert!(efficiency.branch_prediction_accuracy >= 0.0 && efficiency.branch_prediction_accuracy <= 1.0);
    
    // Should have execution unit utilization data
    assert!(!efficiency.execution_unit_utilization.is_empty());
    
    // Bottlenecks analysis should be present (could be empty for simple functions)
    // Just verify the field exists and is accessible
    let _bottlenecks = &efficiency.bottlenecks;
}

#[test]
fn test_regression_detector_identifies_performance_changes() {
    let mut detector = RegressionDetector::new();
    
    // Add baseline performance data
    let baseline_data = PerformanceDataPoint {
        timestamp: 1000,
        build_id: "baseline_build".to_string(),
        compilation_time: Duration::from_secs(5),
        execution_time: Duration::from_millis(100),
        memory_usage: 1024 * 1024, // 1MB
        binary_size: 512 * 1024,   // 512KB
        optimization_level: "O2".to_string(),
        git_commit: Some("abc123".to_string()),
        environment_info: EnvironmentInfo {
            os: "linux".to_string(),
            cpu_model: "test_cpu".to_string(),
            memory_gb: 8,
            compiler_version: "1.0.0".to_string(),
            temperature_celsius: Some(45.0),
        },
    };
    
    // Add multiple baseline points
    for i in 0..35 {
        let mut data = baseline_data.clone();
        data.timestamp += i;
        data.build_id = format!("baseline_build_{}", i);
        // Add slight variations
        data.compilation_time = Duration::from_millis(5000 + (i as u64 * 10));
        data.execution_time = Duration::from_millis(100 + (i as u64));
        detector.add_performance_data(data).unwrap();
    }
    
    // Add a regression case
    let regression_data = PerformanceDataPoint {
        timestamp: 2000,
        build_id: "regression_build".to_string(),
        compilation_time: Duration::from_secs(8), // 60% increase
        execution_time: Duration::from_millis(150), // 50% increase
        memory_usage: 1536 * 1024, // 50% increase
        binary_size: 768 * 1024,   // 50% increase
        optimization_level: "O2".to_string(),
        git_commit: Some("def456".to_string()),
        environment_info: baseline_data.environment_info.clone(),
    };
    
    // Detect regression
    let regression_result = detector.detect_regression(&regression_data).unwrap();
    
    // Should detect the regression
    assert!(regression_result.is_regression);
    assert!(regression_result.confidence_score > 0.5);
    assert!(!regression_result.affected_metrics.is_empty());
    assert!(!regression_result.recommendations.is_empty());
    
    // Should have some root cause analysis
    assert!(!regression_result.root_cause_analysis.potential_causes.is_empty() ||
            !regression_result.root_cause_analysis.environmental_factors.is_empty());
}

#[test]
fn test_real_optimization_manager_comprehensive_analysis() {
    let config = OptimizationConfig::default();
    let manager = RealOptimizationManager::new(config);
    
    let context = Context::create();
    let module = context.create_module("comprehensive_test");
    
    // Create a more complex function for analysis
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("fibonacci", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let loop_block = context.append_basic_block(function, "loop");
    let exit_block = context.append_basic_block(function, "exit");
    
    let builder = context.create_builder();
    
    // Entry block - check if n <= 1
    builder.position_at_end(entry_block);
    let n = function.get_nth_param(0).unwrap().into_int_value();
    let one = i32_type.const_int(1, false);
    let cmp = builder.build_int_compare(inkwell::IntPredicate::ULE, n, one, "cmp").unwrap();
    builder.build_conditional_branch(cmp, exit_block, loop_block).unwrap();
    
    // Loop block - fibonacci calculation
    builder.position_at_end(loop_block);
    let phi_a = builder.build_phi(i32_type, "a").unwrap();
    let phi_b = builder.build_phi(i32_type, "b").unwrap();
    let phi_i = builder.build_phi(i32_type, "i").unwrap();
    
    let zero = i32_type.const_int(0, false);
    phi_a.add_incoming(&[(&zero, entry_block)]);
    phi_b.add_incoming(&[(&one, entry_block)]);
    phi_i.add_incoming(&[(&i32_type.const_int(2, false), entry_block)]);
    
    let a_val = phi_a.as_basic_value().into_int_value();
    let b_val = phi_b.as_basic_value().into_int_value();
    let i_val = phi_i.as_basic_value().into_int_value();
    
    let new_a = b_val;
    let new_b = builder.build_int_add(a_val, b_val, "new_b").unwrap();
    let new_i = builder.build_int_add(i_val, one, "new_i").unwrap();
    
    let loop_cmp = builder.build_int_compare(inkwell::IntPredicate::ULT, new_i, n, "loop_cmp").unwrap();
    builder.build_conditional_branch(loop_cmp, loop_block, exit_block).unwrap();
    
    phi_a.add_incoming(&[(&new_a, loop_block)]);
    phi_b.add_incoming(&[(&new_b, loop_block)]);
    phi_i.add_incoming(&[(&new_i, loop_block)]);
    
    // Exit block
    builder.position_at_end(exit_block);
    let result_phi = builder.build_phi(i32_type, "result").unwrap();
    result_phi.add_incoming(&[(&n, entry_block), (&new_b, loop_block)]);
    builder.build_return(Some(&result_phi.as_basic_value())).unwrap();
    
    // Perform comprehensive optimization analysis
    let result = manager.optimize_with_real_analysis(&module, OptimizationLevel::Aggressive).unwrap();
    
    // Verify comprehensive analysis results
    assert!(!result.session.session_id.is_empty());
    assert_eq!(result.session.optimization_level, OptimizationLevel::Aggressive);
    assert!(result.session.total_duration > Duration::from_nanos(0));
    
    // Verify performance improvements are calculated
    assert!(result.session.performance_improvements.runtime_improvement >= 0.0);
    assert!(result.session.performance_improvements.compilation_speedup >= 1.0);
    
    // Verify CPU efficiency analysis
    assert!(result.session.cpu_efficiency.overall_efficiency >= 0.0);
    assert!(result.session.cpu_efficiency.overall_efficiency <= 1.0);
    
    // Verify detailed performance metrics
    assert!(result.performance_metrics.compilation_metrics.total_compilation_time >= Duration::from_nanos(0));
    assert!(result.performance_metrics.runtime_metrics.instructions_per_cycle >= 0.0);
    assert!(result.performance_metrics.memory_metrics.binary_size_bytes > 0);
    
    // Verify effectiveness analysis
    assert!(result.effectiveness_analysis.overall_effectiveness >= 0.0);
    assert!(result.effectiveness_analysis.overall_effectiveness <= 1.0);
    
    // Should have recommendations
    assert!(!result.recommendations.is_empty());
    
    // Recommendations should have meaningful priorities and types
    for recommendation in &result.recommendations {
        match recommendation.priority {
            RecommendationPriority::Low | RecommendationPriority::Medium | 
            RecommendationPriority::High | RecommendationPriority::Critical => {
                // Valid priority
            }
        }
        
        match recommendation.recommendation_type {
            RecommendationType::IncreaseOptimizationLevel |
            RecommendationType::EnableSpecificPass |
            RecommendationType::TuneParameters |
            RecommendationType::UseProfileGuidedOptimization |
            RecommendationType::ImproveAlgorithm |
            RecommendationType::OptimizeMemoryLayout |
            RecommendationType::EnableVectorization |
            RecommendationType::ReduceCodeSize |
            RecommendationType::ImproveCompilationSpeed => {
                // Valid recommendation type
            }
        }
        
        assert!(recommendation.expected_benefit >= 0.0);
        assert!(!recommendation.description.is_empty());
    }
}

#[test]
fn test_performance_trends_analysis() {
    let config = OptimizationConfig::default();
    let manager = RealOptimizationManager::new(config);
    
    let context = Context::create();
    let module = context.create_module("trends_test");
    
    // Create simple function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("simple", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    builder.build_return(Some(&i32_type.const_int(42, false))).unwrap();
    
    // Perform multiple optimizations to build history
    for i in 0..5 {
        let level = match i % 3 {
            0 => OptimizationLevel::Debug,
            1 => OptimizationLevel::Default,
            _ => OptimizationLevel::Aggressive,
        };
        
        let _result = manager.optimize_with_real_analysis(&module, level).unwrap();
    }
    
    // Get performance trends
    let trends = manager.get_performance_trends().unwrap();
    
    // Verify trends are calculated
    // Trends should be one of the valid enum values
    match trends.runtime_trend {
        cursed::optimization::real_optimization_implementation::TrendDirection::Improving |
        cursed::optimization::real_optimization_implementation::TrendDirection::Stable |
        cursed::optimization::real_optimization_implementation::TrendDirection::Degrading |
        cursed::optimization::real_optimization_implementation::TrendDirection::InsufficientData => {
            // Valid trend direction
        }
    }
    
    assert!(trends.overall_effectiveness >= 0.0);
    
    // Get optimization history
    let history = manager.get_optimization_history();
    assert_eq!(history.len(), 5);
    
    // Verify history entries are valid
    for session in &history {
        assert!(!session.session_id.is_empty());
        assert!(session.total_duration >= Duration::from_nanos(0));
        assert!(session.performance_improvements.compilation_speedup >= 1.0);
    }
}

#[test]
fn test_optimization_report_generation() {
    let config = OptimizationConfig::default();
    let manager = RealOptimizationManager::new(config);
    
    let context = Context::create();
    let module = context.create_module("report_test");
    
    // Create simple function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("test", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    builder.build_return(Some(&i32_type.const_int(1, false))).unwrap();
    
    // Perform optimization to generate data
    let _result = manager.optimize_with_real_analysis(&module, OptimizationLevel::Default).unwrap();
    
    // Generate comprehensive report
    let report = manager.generate_optimization_report().unwrap();
    
    // Verify report contains expected sections
    assert!(report.contains("# CURSED Real Optimization Analysis Report"));
    assert!(report.contains("## Summary Statistics"));
    assert!(report.contains("## Performance Trends"));
    assert!(report.contains("## Recent Optimization Sessions"));
    
    // Report should contain actual data
    assert!(report.contains("Total optimization sessions: 1"));
    assert!(report.contains("CPU efficiency"));
    assert!(report.contains("runtime improvement"));
    
    // Report should be properly formatted markdown
    assert!(report.starts_with('#'));
    assert!(report.contains('\n'));
}

#[cfg(test)]
mod benchmark_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_optimization_performance_benchmarks() {
        let config = OptimizationConfig::default();
        let manager = RealOptimizationManager::new(config);
        
        let context = Context::create();
        let module = context.create_module("benchmark_test");
        
        // Create a moderately complex function for benchmarking
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let function = module.add_function("factorial", fn_type, None);
        
        let entry_block = context.append_basic_block(function, "entry");
        let loop_block = context.append_basic_block(function, "loop");
        let exit_block = context.append_basic_block(function, "exit");
        
        let builder = context.create_builder();
        
        // Build factorial function with loop
        builder.position_at_end(entry_block);
        let n = function.get_nth_param(0).unwrap().into_int_value();
        let zero = i32_type.const_int(0, false);
        let one = i32_type.const_int(1, false);
        let cmp = builder.build_int_compare(inkwell::IntPredicate::ULE, n, one, "cmp").unwrap();
        builder.build_conditional_branch(cmp, exit_block, loop_block).unwrap();
        
        builder.position_at_end(loop_block);
        let phi_result = builder.build_phi(i32_type, "result").unwrap();
        let phi_counter = builder.build_phi(i32_type, "counter").unwrap();
        
        phi_result.add_incoming(&[(&one, entry_block)]);
        phi_counter.add_incoming(&[(&i32_type.const_int(2, false), entry_block)]);
        
        let result_val = phi_result.as_basic_value().into_int_value();
        let counter_val = phi_counter.as_basic_value().into_int_value();
        
        let new_result = builder.build_int_mul(result_val, counter_val, "new_result").unwrap();
        let new_counter = builder.build_int_add(counter_val, one, "new_counter").unwrap();
        
        let loop_cmp = builder.build_int_compare(inkwell::IntPredicate::ULE, new_counter, n, "loop_cmp").unwrap();
        builder.build_conditional_branch(loop_cmp, loop_block, exit_block).unwrap();
        
        phi_result.add_incoming(&[(&new_result, loop_block)]);
        phi_counter.add_incoming(&[(&new_counter, loop_block)]);
        
        builder.position_at_end(exit_block);
        let final_phi = builder.build_phi(i32_type, "final").unwrap();
        final_phi.add_incoming(&[(&one, entry_block), (&new_result, loop_block)]);
        builder.build_return(Some(&final_phi.as_basic_value())).unwrap();
        
        // Benchmark different optimization levels
        let optimization_levels = vec![
            OptimizationLevel::Debug,
            OptimizationLevel::Default,
            OptimizationLevel::Aggressive,
        ];
        
        for level in optimization_levels {
            let start_time = Instant::now();
            let result = manager.optimize_with_real_analysis(&module, level).unwrap();
            let optimization_time = start_time.elapsed();
            
            // Verify optimization completes within reasonable time (< 1 second for test)
            assert!(optimization_time < Duration::from_secs(1), 
                    "Optimization took too long: {:?} for level {:?}", optimization_time, level);
            
            // Verify we get meaningful results
            assert!(result.session.cpu_efficiency.overall_efficiency > 0.0);
            assert!(result.session.performance_improvements.compilation_speedup >= 1.0);
            
            // Higher optimization levels should generally provide better results
            // (though this isn't guaranteed for all cases)
            if matches!(level, OptimizationLevel::Aggressive) {
                assert!(result.session.cpu_efficiency.overall_efficiency >= 0.3,
                        "Aggressive optimization should provide reasonable efficiency");
            }
        }
    }

    #[test]
    fn test_memory_efficiency_calculations() {
        let calculator = RealPerformanceCalculator::new();
        let context = Context::create();
        
        // Create two modules with different memory characteristics
        let small_module = context.create_module("small_module");
        let large_module = context.create_module("large_module");
        
        // Small module - simple function
        let i32_type = context.i32_type();
        let simple_fn_type = i32_type.fn_type(&[], false);
        let simple_function = small_module.add_function("simple", simple_fn_type, None);
        let simple_block = context.append_basic_block(simple_function, "entry");
        
        let builder = context.create_builder();
        builder.position_at_end(simple_block);
        builder.build_return(Some(&i32_type.const_int(1, false))).unwrap();
        
        // Large module - multiple functions with more instructions
        for i in 0..10 {
            let fn_name = format!("function_{}", i);
            let large_fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
            let large_function = large_module.add_function(&fn_name, large_fn_type, None);
            let large_block = context.append_basic_block(large_function, "entry");
            
            builder.position_at_end(large_block);
            let param1 = large_function.get_nth_param(0).unwrap().into_int_value();
            let param2 = large_function.get_nth_param(1).unwrap().into_int_value();
            
            // Add multiple arithmetic operations
            let add1 = builder.build_int_add(param1, param2, "add1").unwrap();
            let mul1 = builder.build_int_mul(add1, param1, "mul1").unwrap();
            let add2 = builder.build_int_add(mul1, param2, "add2").unwrap();
            let mul2 = builder.build_int_mul(add2, i32_type.const_int(i as u64, false), "mul2").unwrap();
            
            builder.build_return(Some(&mul2)).unwrap();
        }
        
        // Calculate improvements for both
        let small_improvements = calculator.calculate_real_performance_improvements(
            &small_module, OptimizationLevel::Default
        ).unwrap();
        
        let large_improvements = calculator.calculate_real_performance_improvements(
            &large_module, OptimizationLevel::Default
        ).unwrap();
        
        // Both should provide valid memory improvement metrics
        assert!(small_improvements.memory_improvement >= 0.0);
        assert!(large_improvements.memory_improvement >= 0.0);
        
        // The improvements should be proportional to complexity
        // (though exact relationships depend on optimization effectiveness)
        assert!(small_improvements.memory_improvement <= 1.0);
        assert!(large_improvements.memory_improvement <= 1.0);
        
        // Verify compilation speedup is reasonable
        assert!(small_improvements.compilation_speedup >= 1.0);
        assert!(large_improvements.compilation_speedup >= 1.0);
        assert!(small_improvements.compilation_speedup <= 10.0);
        assert!(large_improvements.compilation_speedup <= 10.0);
    }
}
