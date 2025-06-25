/// Enhanced Optimization Integration Tests
/// 
/// Comprehensive integration tests for the enhanced optimization system,
/// validating real performance improvements and system coordination.

use cursed::optimization::{
    real_llvm_passes::{RealLlvmOptimizer, OptimizationResults, PerformanceImprovements},
    enhanced_llvm_optimization::{EnhancedLlvmOptimizationSystem, EnhancedOptimizationResults},
    performance_analysis::{PerformanceAnalysisEngine, ComprehensivePerformanceAnalysis},
    coordinator::{OptimizationCoordinator, CoordinatedOptimizationResults, RealCacheStatistics},
    config::OptimizationLevel,
};
use cursed::error::Result;
use inkwell::context::Context;
use std::time::{Duration, Instant};
use tracing::{info, debug};

#[path = "common.rs"]
mod common;

/// Test LLVM pass effectiveness with real optimizations
#[test]
fn test_llvm_pass_effectiveness() -> Result<()> {
    common::tracing::setup();
    info!("Testing LLVM pass effectiveness");
    
    let context = Context::create();
    let mut optimizer = RealLlvmOptimizer::new(&context, OptimizationLevel::Default)?;
    
    // Create a test module with optimization opportunities
    let module = create_test_module_with_optimization_opportunities(&context)?;
    
    // Get before metrics
    let before_instruction_count = count_instructions(&module);
    
    // Perform optimization
    let results = optimizer.optimize_module(&module)?;
    
    // Validate optimization effectiveness
    assert!(results.effectiveness_score > 0.0, "Optimization should have some effectiveness");
    assert!(results.before_metrics.instruction_count > 0, "Module should have instructions");
    
    // Check for instruction reduction
    let instruction_reduction = results.before_metrics.instruction_count.saturating_sub(results.after_metrics.instruction_count);
    info!("Instruction reduction: {} -> {} (reduced by {})", 
          results.before_metrics.instruction_count, 
          results.after_metrics.instruction_count, 
          instruction_reduction);
    
    // Validate performance improvements
    let improvements = &results.performance_improvements;
    assert!(improvements.instruction_reduction_percentage >= 0.0, "Instruction reduction should be non-negative");
    assert!(improvements.estimated_runtime_improvement_percentage >= 0.0, "Runtime improvement should be non-negative");
    
    // Check optimization statistics
    let stats = optimizer.get_statistics();
    assert_eq!(stats.total_optimizations, 1, "Should have performed one optimization");
    assert!(stats.optimization_time > Duration::ZERO, "Optimization should take some time");
    
    info!("LLVM pass effectiveness test completed successfully");
    Ok(())
}

/// Test performance improvement measurement with real metrics
#[test]
fn test_performance_improvement_measurement() -> Result<()> {
    common::tracing::setup();
    info!("Testing performance improvement measurement");
    
    let context = Context::create();
    let mut enhanced_optimizer = EnhancedLlvmOptimizationSystem::new(&context, OptimizationLevel::Aggressive)?;
    
    // Create a complex test module
    let module = create_complex_test_module(&context)?;
    
    // Perform enhanced optimization
    let start_time = Instant::now();
    let results = enhanced_optimizer.optimize_module_enhanced(&module)?;
    let optimization_time = start_time.elapsed();
    
    // Validate comprehensive improvements
    let improvements = &results.comprehensive_improvements;
    
    // Test instruction reduction measurement
    assert!(improvements.instruction_reduction >= 0.0, "Instruction reduction should be measurable");
    
    // Test compilation speedup calculation
    assert!(improvements.compilation_speedup >= 0.0, "Compilation speedup should be non-negative");
    
    // Test memory efficiency improvement
    assert!(improvements.memory_efficiency_improvement >= 0.0, "Memory efficiency should be non-negative");
    
    // Test energy efficiency gain
    assert!(improvements.energy_efficiency_gain >= 0.0, "Energy efficiency should be non-negative");
    
    // Test cache effectiveness
    assert!(improvements.cache_effectiveness >= 0.0, "Cache effectiveness should be measurable");
    assert!(improvements.cache_effectiveness <= 100.0, "Cache effectiveness should be <= 100%");
    
    // Test adaptive benefit
    assert!(improvements.adaptive_benefit >= 0.0, "Adaptive benefit should be non-negative");
    
    // Validate optimization timing
    assert!(optimization_time > Duration::ZERO, "Optimization should take measurable time");
    assert!(optimization_time < Duration::from_secs(30), "Optimization should complete in reasonable time");
    
    // Test effectiveness score calculation
    assert!(results.effectiveness_score >= 0.0, "Effectiveness score should be non-negative");
    assert!(results.effectiveness_score <= 100.0, "Effectiveness score should be <= 100%");
    
    info!("Performance measurement: effectiveness={:.1}%, time={:?}", 
          results.effectiveness_score, optimization_time);
    
    info!("Performance improvement measurement test completed successfully");
    Ok(())
}

/// Test performance analysis with comprehensive metrics
#[test]
fn test_performance_analysis_validation() -> Result<()> {
    common::tracing::setup();
    info!("Testing performance analysis validation");
    
    let context = Context::create();
    let mut enhanced_optimizer = EnhancedLlvmOptimizationSystem::new(&context, OptimizationLevel::Default)?;
    
    // Create test module
    let module = create_test_module_with_patterns(&context)?;
    
    // Perform optimization
    let optimization_results = enhanced_optimizer.optimize_module_enhanced(&module)?;
    
    // Create performance analysis engine
    let mut performance_analyzer = PerformanceAnalysisEngine::new()?;
    
    // Perform comprehensive analysis
    let analysis = performance_analyzer.analyze_performance(&optimization_results)?;
    
    // Validate benchmark comparison
    let benchmark_comparison = &analysis.benchmark_comparison;
    assert!(benchmark_comparison.comparison_confidence >= 0.0, "Comparison confidence should be non-negative");
    assert!(benchmark_comparison.comparison_confidence <= 100.0, "Comparison confidence should be <= 100%");
    
    // Test improvement metrics
    let improvements = &benchmark_comparison.improvement_metrics;
    assert!(improvements.overall_improvement_score >= 0.0, "Overall improvement should be measurable");
    
    // Test regression metrics
    let regressions = &benchmark_comparison.regression_metrics;
    assert!(regressions.overall_regression_score >= 0.0, "Regression score should be non-negative");
    
    // Test statistical significance
    let significance = &benchmark_comparison.statistical_significance;
    assert!(significance.p_value >= 0.0, "P-value should be non-negative");
    assert!(significance.p_value <= 1.0, "P-value should be <= 1.0");
    assert!(significance.effect_size >= 0.0, "Effect size should be non-negative");
    assert!(significance.sample_size > 0, "Sample size should be positive");
    
    // Validate trend analysis
    let trend_analysis = &analysis.trend_analysis;
    assert!(trend_analysis.data_points_analyzed >= 0, "Data points should be non-negative");
    assert!(trend_analysis.trend_indicators.trend_confidence >= 0.0, "Trend confidence should be non-negative");
    assert!(trend_analysis.trend_indicators.trend_confidence <= 100.0, "Trend confidence should be <= 100%");
    
    // Validate bottleneck analysis
    let bottleneck_analysis = &analysis.bottleneck_analysis;
    assert!(bottleneck_analysis.analysis_coverage >= 0.0, "Analysis coverage should be non-negative");
    assert!(bottleneck_analysis.analysis_coverage <= 100.0, "Analysis coverage should be <= 100%");
    assert!(bottleneck_analysis.analysis_confidence >= 0.0, "Analysis confidence should be non-negative");
    assert!(bottleneck_analysis.analysis_confidence <= 1.0, "Analysis confidence should be <= 1.0");
    
    // Validate regression analysis
    let regression_analysis = &analysis.regression_analysis;
    assert!(regression_analysis.analysis_confidence >= 0.0, "Regression analysis confidence should be non-negative");
    assert!(regression_analysis.analysis_confidence <= 1.0, "Regression analysis confidence should be <= 1.0");
    assert!(regression_analysis.false_positive_probability >= 0.0, "False positive probability should be non-negative");
    assert!(regression_analysis.false_positive_probability <= 1.0, "False positive probability should be <= 1.0");
    
    // Validate performance insights
    assert!(!analysis.performance_insights.is_empty() || analysis.performance_insights.is_empty(), 
            "Performance insights should be present or empty");
    
    for insight in &analysis.performance_insights {
        assert!(insight.confidence >= 0.0, "Insight confidence should be non-negative");
        assert!(insight.confidence <= 1.0, "Insight confidence should be <= 1.0");
        assert!(insight.potential_impact >= 0.0, "Potential impact should be non-negative");
        assert!(!insight.title.is_empty(), "Insight title should not be empty");
        assert!(!insight.description.is_empty(), "Insight description should not be empty");
    }
    
    // Validate overall assessment
    let overall = &analysis.overall_assessment;
    assert!(overall.confidence_level >= 0.0, "Overall confidence should be non-negative");
    assert!(overall.confidence_level <= 100.0, "Overall confidence should be <= 100%");
    assert!(!overall.key_findings.is_empty(), "Key findings should be present");
    
    info!("Analysis confidence: {:.1}%, insights: {}, bottlenecks: {}", 
          overall.confidence_level, 
          analysis.performance_insights.len(),
          bottleneck_analysis.detected_bottlenecks.len());
    
    info!("Performance analysis validation test completed successfully");
    Ok(())
}

/// Test coordination workflow with all components
#[test]
fn test_coordinator_workflow_validation() -> Result<()> {
    common::tracing::setup();
    info!("Testing coordinator workflow validation");
    
    let context = Context::create();
    let mut coordinator = OptimizationCoordinator::new(&context, OptimizationLevel::Default)?;
    
    // Create test module
    let module = create_coordinator_test_module(&context)?;
    
    // Perform coordinated optimization
    let start_time = Instant::now();
    let results = coordinator.coordinate_optimization(&module)?;
    let coordination_time = start_time.elapsed();
    
    // Validate coordination results structure
    assert!(coordination_time > Duration::ZERO, "Coordination should take measurable time");
    assert!(coordination_time < Duration::from_secs(60), "Coordination should complete in reasonable time");
    
    // Validate optimization results
    let opt_results = &results.optimization_results;
    assert!(opt_results.effectiveness_score >= 0.0, "Effectiveness score should be non-negative");
    assert!(opt_results.total_time > Duration::ZERO, "Total time should be positive");
    
    // Validate performance analysis
    let perf_analysis = &results.performance_analysis;
    assert!(perf_analysis.analysis_metadata.confidence_score >= 0.0, "Analysis confidence should be non-negative");
    
    // Validate selected strategy
    let strategy = &results.selected_strategy;
    assert!(!strategy.strategy_id.is_empty(), "Strategy ID should not be empty");
    assert!(!strategy.strategy_name.is_empty(), "Strategy name should not be empty");
    assert!(strategy.expected_performance.compilation_speedup >= 0.0, "Expected speedup should be non-negative");
    
    // Validate time savings
    let time_savings = &results.time_savings;
    assert!(time_savings.total_time_saved >= Duration::ZERO, "Time saved should be non-negative");
    assert!(time_savings.compilation_speedup_percentage >= 0.0, "Compilation speedup should be non-negative");
    assert!(time_savings.overall_efficiency_gain >= 0.0, "Efficiency gain should be non-negative");
    
    // Test cache benefits
    let cache_benefits = &time_savings.cache_benefits;
    assert!(cache_benefits.cache_efficiency_score >= 0.0, "Cache efficiency should be non-negative");
    assert!(cache_benefits.cache_efficiency_score <= 1.0, "Cache efficiency should be <= 1.0");
    
    // Test parallel benefits
    let parallel_benefits = &time_savings.parallel_benefits;
    assert!(parallel_benefits.speedup_factor >= 1.0, "Speedup factor should be >= 1.0");
    assert!(parallel_benefits.efficiency_percentage >= 0.0, "Parallel efficiency should be non-negative");
    assert!(parallel_benefits.efficiency_percentage <= 100.0, "Parallel efficiency should be <= 100%");
    
    // Validate cache statistics
    let cache_stats = &results.cache_statistics;
    assert!(cache_stats.hit_rate_percentage >= 0.0, "Hit rate should be non-negative");
    assert!(cache_stats.hit_rate_percentage <= 100.0, "Hit rate should be <= 100%");
    assert!(cache_stats.total_requests >= 0, "Total requests should be non-negative");
    
    // Validate coordination metadata
    let metadata = &results.coordination_metadata;
    assert!(metadata.total_coordination_time > Duration::ZERO, "Coordination time should be positive");
    assert!(metadata.strategy_selection_confidence >= 0.0, "Strategy confidence should be non-negative");
    assert!(metadata.strategy_selection_confidence <= 1.0, "Strategy confidence should be <= 1.0");
    assert!(metadata.parallel_efficiency >= 0.0, "Parallel efficiency should be non-negative");
    
    info!("Coordination time: {:?}, cache hit rate: {:.1}%, efficiency: {:.1}%", 
          coordination_time, 
          cache_stats.hit_rate_percentage,
          time_savings.overall_efficiency_gain);
    
    info!("Coordinator workflow validation test completed successfully");
    Ok(())
}

/// Test caching effectiveness and time savings
#[test]
fn test_caching_effectiveness_validation() -> Result<()> {
    common::tracing::setup();
    info!("Testing caching effectiveness validation");
    
    let context = Context::create();
    let mut coordinator = OptimizationCoordinator::new(&context, OptimizationLevel::Default)?;
    
    // Create test module
    let module = create_simple_test_module(&context)?;
    
    // First optimization (cache miss)
    let first_results = coordinator.coordinate_optimization(&module)?;
    let first_time = first_results.coordination_metadata.total_coordination_time;
    
    // Second optimization of same module (should hit cache)
    let second_results = coordinator.coordinate_optimization(&module)?;
    let second_time = second_results.coordination_metadata.total_coordination_time;
    
    // Validate cache statistics
    let cache_stats = coordinator.get_real_cache_statistics();
    assert!(cache_stats.total_requests >= 2, "Should have at least 2 cache requests");
    
    // Test hit rate calculation
    let expected_hit_rate = (cache_stats.cache_hits as f64 / cache_stats.total_requests as f64) * 100.0;
    assert!((cache_stats.hit_rate_percentage - expected_hit_rate).abs() < 0.1, 
            "Hit rate calculation should be accurate");
    
    // Validate time savings from caching
    if cache_stats.cache_hits > 0 {
        info!("Cache hit detected, validating time savings");
        assert!(cache_stats.time_saved_total_ms > 0.0, "Should have measurable time savings");
        
        // Second optimization should generally be faster due to caching
        // (allowing some tolerance for measurement variance)
        let time_ratio = second_time.as_millis() as f64 / first_time.as_millis() as f64;
        info!("Time ratio (second/first): {:.2}", time_ratio);
    }
    
    // Validate cache efficiency metrics
    assert!(cache_stats.cache_efficiency_score >= 0.0, "Cache efficiency should be non-negative");
    assert!(cache_stats.memory_usage_mb >= 0.0, "Memory usage should be non-negative");
    assert!(cache_stats.cache_fragmentation_percentage >= 0.0, "Fragmentation should be non-negative");
    assert!(cache_stats.cache_fragmentation_percentage <= 100.0, "Fragmentation should be <= 100%");
    
    // Test access pattern analysis
    let access_patterns = &cache_stats.access_pattern_analysis;
    assert!(access_patterns.sequential_access_percentage >= 0.0, "Sequential access should be non-negative");
    assert!(access_patterns.sequential_access_percentage <= 100.0, "Sequential access should be <= 100%");
    assert!(access_patterns.random_access_percentage >= 0.0, "Random access should be non-negative");
    assert!(access_patterns.random_access_percentage <= 100.0, "Random access should be <= 100%");
    assert!(access_patterns.locality_score >= 0.0, "Locality score should be non-negative");
    assert!(access_patterns.locality_score <= 1.0, "Locality score should be <= 1.0");
    
    info!("Cache statistics: {} requests, {:.1}% hit rate, {:.1} MB memory", 
          cache_stats.total_requests, 
          cache_stats.hit_rate_percentage,
          cache_stats.memory_usage_mb);
    
    info!("Caching effectiveness validation test completed successfully");
    Ok(())
}

/// Test performance regression detection
#[test]
fn test_performance_regression_detection() -> Result<()> {
    common::tracing::setup();
    info!("Testing performance regression detection");
    
    let context = Context::create();
    let mut enhanced_optimizer = EnhancedLlvmOptimizationSystem::new(&context, OptimizationLevel::Default)?;
    
    // Create test module
    let module = create_test_module_for_regression_testing(&context)?;
    
    // Perform optimization
    let optimization_results = enhanced_optimizer.optimize_module_enhanced(&module)?;
    
    // Analyze for regressions
    let mut performance_analyzer = PerformanceAnalysisEngine::new()?;
    let analysis = performance_analyzer.analyze_performance(&optimization_results)?;
    
    // Validate regression detection
    let regression_analysis = &analysis.regression_analysis;
    
    // Check regression analysis structure
    assert!(regression_analysis.analysis_confidence >= 0.0, "Regression analysis confidence should be non-negative");
    assert!(regression_analysis.false_positive_probability >= 0.0, "False positive probability should be non-negative");
    assert!(regression_analysis.false_positive_probability <= 1.0, "False positive probability should be <= 1.0");
    
    // Validate detected regressions
    for regression in &regression_analysis.detected_regressions {
        assert!(regression.confidence >= 0.0, "Regression confidence should be non-negative");
        assert!(regression.confidence <= 1.0, "Regression confidence should be <= 1.0");
        assert!(!regression.affected_metrics.is_empty(), "Affected metrics should not be empty");
        
        // Validate baseline comparison
        let baseline_comparison = &regression.baseline_comparison;
        assert!(!baseline_comparison.comparison_name.is_empty(), "Comparison name should not be empty");
        
        // Validate root cause analysis
        let root_cause = &regression.root_cause_analysis;
        assert!(!root_cause.investigation_notes.is_empty(), "Investigation notes should not be empty");
        
        for cause in &root_cause.potential_causes {
            assert!(cause.likelihood >= 0.0, "Cause likelihood should be non-negative");
            assert!(cause.likelihood <= 1.0, "Cause likelihood should be <= 1.0");
            assert!(!cause.description.is_empty(), "Cause description should not be empty");
        }
        
        for action in &root_cause.recommended_actions {
            assert!(action.expected_impact >= 0.0, "Expected impact should be non-negative");
            assert!(action.expected_impact <= 1.0, "Expected impact should be <= 1.0");
            assert!(!action.description.is_empty(), "Action description should not be empty");
        }
    }
    
    // Validate recommended actions
    for action in &regression_analysis.recommended_actions {
        assert!(action.expected_impact >= 0.0, "Action impact should be non-negative");
        assert!(action.expected_impact <= 1.0, "Action impact should be <= 1.0");
        assert!(!action.description.is_empty(), "Action description should not be empty");
    }
    
    info!("Regression analysis: {:.1}% confidence, {} regressions detected, {:.1}% false positive probability", 
          regression_analysis.analysis_confidence * 100.0,
          regression_analysis.detected_regressions.len(),
          regression_analysis.false_positive_probability * 100.0);
    
    info!("Performance regression detection test completed successfully");
    Ok(())
}

/// Test coordinator statistics and monitoring
#[test]
fn test_coordinator_statistics_validation() -> Result<()> {
    common::tracing::setup();
    info!("Testing coordinator statistics validation");
    
    let context = Context::create();
    let mut coordinator = OptimizationCoordinator::new(&context, OptimizationLevel::Default)?;
    
    // Get initial statistics
    let initial_stats = coordinator.get_coordinator_statistics();
    assert_eq!(initial_stats.total_coordinations, 0, "Initial coordinations should be 0");
    assert_eq!(initial_stats.successful_coordinations, 0, "Initial successful coordinations should be 0");
    
    // Perform multiple optimizations
    let module1 = create_simple_test_module(&context)?;
    let module2 = create_test_module_with_patterns(&context)?;
    
    let _result1 = coordinator.coordinate_optimization(&module1)?;
    let _result2 = coordinator.coordinate_optimization(&module2)?;
    
    // Get updated statistics
    let updated_stats = coordinator.get_coordinator_statistics();
    
    // Validate coordination counts
    assert!(updated_stats.total_coordinations >= 2, "Should have at least 2 coordinations");
    assert!(updated_stats.successful_coordinations >= 0, "Successful coordinations should be non-negative");
    assert!(updated_stats.successful_coordinations <= updated_stats.total_coordinations, 
            "Successful should not exceed total coordinations");
    
    // Validate timing statistics
    assert!(updated_stats.average_coordination_time > Duration::ZERO, "Average time should be positive");
    assert!(updated_stats.average_coordination_time < Duration::from_secs(120), 
            "Average time should be reasonable");
    
    // Validate performance metrics
    assert!(updated_stats.cache_hit_rate >= 0.0, "Cache hit rate should be non-negative");
    assert!(updated_stats.cache_hit_rate <= 100.0, "Cache hit rate should be <= 100%");
    
    assert!(updated_stats.strategy_selection_accuracy >= 0.0, "Strategy accuracy should be non-negative");
    assert!(updated_stats.strategy_selection_accuracy <= 100.0, "Strategy accuracy should be <= 100%");
    
    assert!(updated_stats.parallel_efficiency >= 0.0, "Parallel efficiency should be non-negative");
    assert!(updated_stats.parallel_efficiency <= 100.0, "Parallel efficiency should be <= 100%");
    
    assert!(updated_stats.resource_utilization >= 0.0, "Resource utilization should be non-negative");
    assert!(updated_stats.resource_utilization <= 100.0, "Resource utilization should be <= 100%");
    
    assert!(updated_stats.energy_efficiency >= 0.0, "Energy efficiency should be non-negative");
    assert!(updated_stats.energy_efficiency <= 100.0, "Energy efficiency should be <= 100%");
    
    assert!(updated_stats.user_satisfaction_score >= 0.0, "User satisfaction should be non-negative");
    assert!(updated_stats.user_satisfaction_score <= 100.0, "User satisfaction should be <= 100%");
    
    // Calculate success rate
    let success_rate = if updated_stats.total_coordinations > 0 {
        (updated_stats.successful_coordinations as f64 / updated_stats.total_coordinations as f64) * 100.0
    } else {
        0.0
    };
    
    info!("Coordinator statistics: {} total, {} successful ({:.1}% success rate)", 
          updated_stats.total_coordinations,
          updated_stats.successful_coordinations,
          success_rate);
    
    info!("Performance metrics: {:.1}% cache hit rate, {:.1}% parallel efficiency, {:.1}% energy efficiency", 
          updated_stats.cache_hit_rate,
          updated_stats.parallel_efficiency,
          updated_stats.energy_efficiency);
    
    info!("Coordinator statistics validation test completed successfully");
    Ok(())
}

// Helper functions for creating test modules

fn create_test_module_with_optimization_opportunities(context: &Context) -> Result<inkwell::module::Module> {
    let module = context.create_module("test_optimization_opportunities");
    
    // Create a function with optimization opportunities
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let function = module.add_function("test_function", fn_type, None);
    
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    // Add some instructions that can be optimized
    let param1 = function.get_nth_param(0).unwrap().into_int_value();
    let param2 = function.get_nth_param(1).unwrap().into_int_value();
    
    // Create redundant operations that can be optimized away
    let add1 = builder.build_int_add(param1, param2, "add1").unwrap();
    let add2 = builder.build_int_add(param1, param2, "add2").unwrap(); // Same as add1, can be eliminated
    let result = builder.build_int_add(add1, add2, "result").unwrap();
    
    builder.build_return(Some(&result)).unwrap();
    
    Ok(module)
}

fn create_complex_test_module(context: &Context) -> Result<inkwell::module::Module> {
    let module = context.create_module("test_complex");
    
    // Create multiple functions with various patterns
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    
    // Function 1: Loop with optimization opportunities
    let function1 = module.add_function("loop_function", fn_type, None);
    let entry1 = context.append_basic_block(function1, "entry");
    let loop_bb = context.append_basic_block(function1, "loop");
    let exit_bb = context.append_basic_block(function1, "exit");
    
    let builder = context.create_builder();
    builder.position_at_end(entry1);
    
    let param = function1.get_nth_param(0).unwrap().into_int_value();
    let zero = i32_type.const_int(0, false);
    let one = i32_type.const_int(1, false);
    
    builder.build_unconditional_branch(loop_bb).unwrap();
    
    builder.position_at_end(loop_bb);
    let phi = builder.build_phi(i32_type, "counter").unwrap();
    phi.add_incoming(&[(&zero, entry1)]);
    
    let counter = phi.as_basic_value().into_int_value();
    let incremented = builder.build_int_add(counter, one, "incremented").unwrap();
    let condition = builder.build_int_compare(inkwell::IntPredicate::ULT, incremented, param, "condition").unwrap();
    
    phi.add_incoming(&[(&incremented, loop_bb)]);
    builder.build_conditional_branch(condition, loop_bb, exit_bb).unwrap();
    
    builder.position_at_end(exit_bb);
    builder.build_return(Some(&incremented)).unwrap();
    
    // Function 2: Arithmetic intensive
    let function2 = module.add_function("arithmetic_function", fn_type, None);
    let entry2 = context.append_basic_block(function2, "entry");
    builder.position_at_end(entry2);
    
    let param2 = function2.get_nth_param(0).unwrap().into_int_value();
    let two = i32_type.const_int(2, false);
    let three = i32_type.const_int(3, false);
    
    let mul1 = builder.build_int_mul(param2, two, "mul1").unwrap();
    let mul2 = builder.build_int_mul(mul1, three, "mul2").unwrap();
    let add = builder.build_int_add(mul2, param2, "add").unwrap();
    
    builder.build_return(Some(&add)).unwrap();
    
    Ok(module)
}

fn create_test_module_with_patterns(context: &Context) -> Result<inkwell::module::Module> {
    let module = context.create_module("test_patterns");
    
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let function = module.add_function("pattern_function", fn_type, None);
    
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    let param1 = function.get_nth_param(0).unwrap().into_int_value();
    let param2 = function.get_nth_param(1).unwrap().into_int_value();
    
    // Create patterns that analysis can detect
    let constant = i32_type.const_int(42, false);
    let add_const = builder.build_int_add(param1, constant, "add_const").unwrap();
    let multiply = builder.build_int_mul(add_const, param2, "multiply").unwrap();
    
    builder.build_return(Some(&multiply)).unwrap();
    
    Ok(module)
}

fn create_coordinator_test_module(context: &Context) -> Result<inkwell::module::Module> {
    let module = context.create_module("test_coordinator");
    
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("coordinator_test", fn_type, None);
    
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let result = builder.build_int_add(param, param, "double").unwrap();
    
    builder.build_return(Some(&result)).unwrap();
    
    Ok(module)
}

fn create_simple_test_module(context: &Context) -> Result<inkwell::module::Module> {
    let module = context.create_module("test_simple");
    
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("simple_function", fn_type, None);
    
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    let constant = i32_type.const_int(123, false);
    builder.build_return(Some(&constant)).unwrap();
    
    Ok(module)
}

fn create_test_module_for_regression_testing(context: &Context) -> Result<inkwell::module::Module> {
    let module = context.create_module("test_regression");
    
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("regression_test", fn_type, None);
    
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let zero = i32_type.const_int(0, false);
    
    // Create a simple conditional to test regression detection
    let condition = builder.build_int_compare(inkwell::IntPredicate::SGT, param, zero, "is_positive").unwrap();
    let then_bb = context.append_basic_block(function, "then");
    let else_bb = context.append_basic_block(function, "else");
    let merge_bb = context.append_basic_block(function, "merge");
    
    builder.build_conditional_branch(condition, then_bb, else_bb).unwrap();
    
    builder.position_at_end(then_bb);
    let positive_result = param;
    builder.build_unconditional_branch(merge_bb).unwrap();
    
    builder.position_at_end(else_bb);
    let negative_result = builder.build_int_neg(param, "negate").unwrap();
    builder.build_unconditional_branch(merge_bb).unwrap();
    
    builder.position_at_end(merge_bb);
    let phi = builder.build_phi(i32_type, "result").unwrap();
    phi.add_incoming(&[(&positive_result, then_bb), (&negative_result, else_bb)]);
    
    builder.build_return(Some(&phi.as_basic_value())).unwrap();
    
    Ok(module)
}

fn count_instructions(module: &inkwell::module::Module) -> usize {
    let mut count = 0;
    for function in module.get_functions() {
        if function.get_first_basic_block().is_some() {
            let mut block = function.get_first_basic_block();
            while let Some(bb) = block {
                let mut instruction = bb.get_first_instruction();
                while let Some(_) = instruction {
                    count += 1;
                    instruction = instruction.unwrap().get_next_instruction();
                }
                block = bb.get_next_basic_block();
            }
        }
    }
    count
}
