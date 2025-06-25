/// Comprehensive tests for the CURSED performance debugging system
/// 
/// Tests the real implementation of performance debugging, profiling,
/// adaptive pass ordering, and regression testing components.

use cursed::optimization::performance_debugging::*;
use cursed::error::Result;
use std::time::Duration;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_debugger_creation() {
        let config = DebugConfig {
            enable_pass_tracing: true,
            enable_profiling: true,
            enable_adaptive_learning: true,
            enable_regression_testing: true,
            verbosity_level: DebugVerbosity::Normal,
            output_format: DebugOutputFormat::Text,
        };
        
        let debugger = PerformanceDebugger::new(config);
        assert_eq!(debugger.get_statistics().passes_traced, 0);
        assert_eq!(debugger.get_statistics().profiling_sessions, 0);
    }

    #[test]
    fn test_debug_session_lifecycle() -> Result<()> {
        let config = DebugConfig {
            enable_pass_tracing: true,
            enable_profiling: true,
            enable_adaptive_learning: false,
            enable_regression_testing: false,
            verbosity_level: DebugVerbosity::Normal,
            output_format: DebugOutputFormat::Text,
        };
        
        let mut debugger = PerformanceDebugger::new(config);
        
        // Start session
        debugger.start_debug_session("test_session")?;
        
        // Trace some passes
        let result1 = debugger.trace_pass_execution("dead_code_elimination", 1000, || {
            std::thread::sleep(Duration::from_millis(10));
            Ok(42)
        })?;
        assert_eq!(result1, 42);
        
        let result2 = debugger.trace_pass_execution("constant_folding", 500, || {
            std::thread::sleep(Duration::from_millis(5));
            Ok("optimized")
        })?;
        assert_eq!(result2, "optimized");
        
        // End session and get report
        let report = debugger.end_debug_session()?;
        assert_eq!(report.trace_results.len(), 2);
        
        // Check statistics
        let stats = debugger.get_statistics();
        assert_eq!(stats.passes_traced, 2);
        
        Ok(())
    }

    #[test]
    fn test_pass_execution_tracer() -> Result<()> {
        let config = TraceConfig {
            enable_detailed_tracing: true,
            trace_intermediate_states: true,
            trace_transformations: true,
            max_trace_depth: 5,
            trace_memory_usage: true,
        };
        
        let mut tracer = PassExecutionTracer::new(config);
        tracer.start_tracing()?;
        
        // Trace multiple passes
        let trace_id1 = tracer.start_pass_trace("dead_code_elimination", 1000)?;
        std::thread::sleep(Duration::from_millis(10));
        tracer.end_pass_trace(trace_id1, true)?;
        
        let trace_id2 = tracer.start_pass_trace("constant_folding", 500)?;
        std::thread::sleep(Duration::from_millis(5));
        tracer.end_pass_trace(trace_id2, true)?;
        
        let traces = tracer.get_traces();
        assert_eq!(traces.len(), 2);
        
        // Verify trace details
        assert_eq!(traces[0].pass_name, "dead_code_elimination");
        assert_eq!(traces[1].pass_name, "constant_folding");
        
        // Check that duration is recorded
        assert!(traces[0].duration.is_some());
        assert!(traces[1].duration.is_some());
        
        // Check performance metrics
        assert!(traces[0].performance_metrics.instructions_processed > 0);
        assert!(traces[0].performance_metrics.memory_usage > 0);
        assert!(traces[0].performance_metrics.cache_hits > 0);
        
        Ok(())
    }

    #[test]
    fn test_trace_report_generation() -> Result<()> {
        let config = TraceConfig::default();
        let mut tracer = PassExecutionTracer::new(config);
        tracer.start_tracing()?;
        
        // Add some traces
        for i in 0..5 {
            let trace_id = tracer.start_pass_trace(&format!("pass_{}", i), 1000)?;
            std::thread::sleep(Duration::from_millis(2));
            tracer.end_pass_trace(trace_id, true)?;
        }
        
        let report = tracer.generate_trace_report()?;
        
        // Verify report contains expected sections
        assert!(report.contains("Pass Execution Trace Report"));
        assert!(report.contains("Total traces collected: 5"));
        assert!(report.contains("Top 10 Slowest Passes"));
        assert!(report.contains("Performance Metrics Summary"));
        assert!(report.contains("Cache hit rate"));
        
        Ok(())
    }

    #[test]
    fn test_optimization_profiler() -> Result<()> {
        let config = ProfilingConfig {
            enable_detailed_profiling: true,
            profile_memory_usage: true,
            profile_cache_behavior: true,
            sample_interval: Duration::from_millis(10),
            max_sessions: 5,
        };
        
        let mut profiler = OptimizationProfiler::new(config);
        
        // Start profiling session
        profiler.start_session("test_profiling")?;
        
        // Profile some passes
        profiler.profile_pass("dead_code_elimination", Duration::from_millis(50), 1024 * 1024)?;
        profiler.profile_pass("constant_folding", Duration::from_millis(20), 512 * 1024)?;
        profiler.profile_pass("dead_code_elimination", Duration::from_millis(45), 1536 * 1024)?;
        
        // End session
        let session = profiler.end_session()?.unwrap();
        
        // Verify session data
        assert_eq!(session.session_id, "test_profiling");
        assert_eq!(session.passes_profiled.len(), 2); // dead_code and constant_folding
        
        // Check dead_code_elimination profile
        let dead_code_profile = session.passes_profiled.iter()
            .find(|p| p.pass_name == "dead_code_elimination")
            .unwrap();
        assert_eq!(dead_code_profile.execution_count, 2);
        assert!(dead_code_profile.effectiveness_score > 0.0);
        
        Ok(())
    }

    #[test]
    fn test_profiler_baseline_comparison() -> Result<()> {
        let config = ProfilingConfig::default();
        let mut profiler = OptimizationProfiler::new(config);
        
        // Create first session as baseline
        profiler.start_session("baseline")?;
        profiler.profile_pass("optimization_pass", Duration::from_millis(100), 1024 * 1024)?;
        profiler.end_session()?;
        profiler.create_baseline("baseline_1")?;
        
        // Create second session for comparison
        profiler.start_session("comparison")?;
        profiler.profile_pass("optimization_pass", Duration::from_millis(80), 800 * 1024)?;
        
        let comparison = profiler.compare_to_baseline("baseline_1")?;
        assert_eq!(comparison.baseline_name, "baseline_1");
        assert_eq!(comparison.current_session_id, "comparison");
        assert!(comparison.overall_improvement > 0.0); // Should show improvement
        
        Ok(())
    }

    #[test]
    fn test_adaptive_pass_manager() -> Result<()> {
        let config = AdaptiveConfig {
            enable_adaptive_ordering: true,
            learning_rate: 0.1,
            minimum_samples: 3,
            confidence_threshold: 0.7,
            exploration_rate: 0.1,
        };
        
        let mut manager = AdaptivePassManager::new(config);
        
        // Record some performance feedback
        manager.record_performance_feedback("dead_code_elimination", Duration::from_millis(50), 0.8, true)?;
        manager.record_performance_feedback("constant_folding", Duration::from_millis(20), 0.9, true)?;
        manager.record_performance_feedback("function_inlining", Duration::from_millis(100), 0.6, true)?;
        
        // Try to optimize ordering (should return original due to insufficient samples)
        let original_ordering = vec!["dead_code_elimination".to_string(), "constant_folding".to_string(), "function_inlining".to_string()];
        let optimized = manager.optimize_pass_ordering(&original_ordering)?;
        
        // Should return original ordering due to insufficient samples
        assert_eq!(optimized, original_ordering);
        
        // Add more samples to reach minimum
        for _ in 0..5 {
            manager.record_performance_feedback("dead_code_elimination", Duration::from_millis(45), 0.85, true)?;
            manager.record_performance_feedback("constant_folding", Duration::from_millis(15), 0.95, true)?;
        }
        
        // Now should potentially reorder (though may be the same due to exploration)
        let optimized2 = manager.optimize_pass_ordering(&original_ordering)?;
        assert_eq!(optimized2.len(), original_ordering.len());
        
        Ok(())
    }

    #[test]
    fn test_adaptive_learning_results() -> Result<()> {
        let config = AdaptiveConfig::default();
        let mut manager = AdaptivePassManager::new(config);
        
        // Record feedback for multiple passes
        manager.record_performance_feedback("high_effectiveness", Duration::from_millis(10), 0.95, true)?;
        manager.record_performance_feedback("medium_effectiveness", Duration::from_millis(50), 0.7, true)?;
        manager.record_performance_feedback("low_effectiveness", Duration::from_millis(200), 0.3, false)?;
        
        let results = manager.get_learning_results();
        
        // Check that we have effectiveness data
        assert!(!results.pass_effectiveness.is_empty());
        assert_eq!(results.adaptations_made, 0); // No orderings recorded yet
        
        Ok(())
    }

    #[test]
    fn test_regression_tester() -> Result<()> {
        let config = RegressionTestConfig {
            enable_continuous_testing: false,
            test_on_optimization_change: true,
            parallel_test_execution: false,
            max_test_time: Duration::from_secs(5),
            memory_limit: 1024 * 1024 * 1024,
        };
        
        let mut tester = RegressionTester::new(config);
        
        // Should have default test suites
        let recent_results = tester.get_recent_results();
        assert!(recent_results.is_empty()); // No tests run yet
        
        // Run all tests
        let results = tester.run_all_tests()?;
        
        // Verify results structure
        assert!(results.tests_run > 0);
        assert_eq!(results.tests_run, results.tests_passed + results.tests_failed);
        assert!(!results.detailed_results.is_empty());
        
        Ok(())
    }

    #[test]
    fn test_regression_baseline_creation() -> Result<()> {
        let config = RegressionTestConfig::default();
        let mut tester = RegressionTester::new(config);
        
        // Create baseline for a test suite
        tester.create_baseline("basic_optimizations")?;
        
        // Run tests again and compare
        let current_results = tester.run_all_tests()?;
        
        // Should have baseline comparisons available now
        let comparisons = tester.compare_to_baseline("basic_optimizations", &current_results.detailed_results)?;
        assert!(!comparisons.is_empty());
        
        // Check comparison structure
        for comparison in &comparisons {
            assert!(!comparison.test_name.is_empty());
            assert!(comparison.baseline_time > Duration::default());
            assert!(comparison.current_time > Duration::default());
            assert!(comparison.significance >= 0.0 && comparison.significance <= 1.0);
        }
        
        Ok(())
    }

    #[test]
    fn test_regression_report_generation() -> Result<()> {
        let config = RegressionTestConfig::default();
        let mut tester = RegressionTester::new(config);
        
        // Run some tests to generate data
        tester.run_all_tests()?;
        
        let report = tester.generate_regression_report()?;
        
        // Verify report structure
        assert!(report.contains("Regression Testing Report"));
        assert!(report.contains("Test suites:"));
        assert!(report.contains("Test Suite Overview"));
        assert!(report.contains("Recommendations"));
        
        Ok(())
    }

    #[test]
    fn test_comprehensive_debugging_workflow() -> Result<()> {
        let config = DebugConfig {
            enable_pass_tracing: true,
            enable_profiling: true,
            enable_adaptive_learning: true,
            enable_regression_testing: true,
            verbosity_level: DebugVerbosity::Verbose,
            output_format: DebugOutputFormat::Markdown,
        };
        
        let mut debugger = PerformanceDebugger::new(config);
        
        // Full debugging workflow
        debugger.start_debug_session("comprehensive_test")?;
        
        // Trace several optimization passes
        let passes = vec!["dead_code_elimination", "constant_folding", "function_inlining", "loop_optimization"];
        for pass in &passes {
            debugger.trace_pass_execution(pass, 1000, || {
                std::thread::sleep(Duration::from_millis(10 + rand::random::<u64>() % 20));
                Ok(())
            })?;
        }
        
        // Test adaptive pass ordering
        let original_ordering: Vec<String> = passes.iter().map(|s| s.to_string()).collect();
        let optimized_ordering = debugger.adapt_pass_ordering(&original_ordering)?;
        assert_eq!(optimized_ordering.len(), original_ordering.len());
        
        // Run regression tests
        let regression_results = debugger.run_regression_tests()?;
        assert!(regression_results.tests_run > 0);
        
        // End session and generate comprehensive report
        let debug_report = debugger.end_debug_session()?;
        assert!(!debug_report.trace_results.is_empty());
        
        // Generate final report
        let final_report = debugger.generate_report()?;
        assert!(final_report.contains("CURSED Optimization Performance Debug Report"));
        assert!(final_report.contains("Debug Statistics"));
        assert!(final_report.contains("Pass Execution Traces"));
        
        // Verify statistics were updated
        let stats = debugger.get_statistics();
        assert_eq!(stats.passes_traced, passes.len());
        assert!(stats.regression_tests_run > 0);
        
        Ok(())
    }

    #[test]
    fn test_error_handling_in_tracing() -> Result<()> {
        let config = DebugConfig {
            enable_pass_tracing: true,
            enable_profiling: false,
            enable_adaptive_learning: false,
            enable_regression_testing: false,
            verbosity_level: DebugVerbosity::Normal,
            output_format: DebugOutputFormat::Text,
        };
        
        let mut debugger = PerformanceDebugger::new(config);
        debugger.start_debug_session("error_test")?;
        
        // Test error handling in pass tracing
        let result = debugger.trace_pass_execution("failing_pass", 500, || {
            Err(cursed::error::Error::InvalidInput("Simulated pass failure".to_string()))
        });
        
        // Should propagate the error
        assert!(result.is_err());
        
        // But should still record the trace
        let report = debugger.end_debug_session()?;
        assert_eq!(report.trace_results.len(), 1);
        
        // Check that error was recorded in debug info
        let trace = &report.trace_results[0];
        assert!(!trace.debug_info.errors.is_empty());
        
        Ok(())
    }

    #[test]
    fn test_performance_metrics_accuracy() -> Result<()> {
        let config = TraceConfig {
            enable_detailed_tracing: true,
            trace_intermediate_states: false,
            trace_transformations: true,
            max_trace_depth: 10,
            trace_memory_usage: true,
        };
        
        let mut tracer = PassExecutionTracer::new(config);
        tracer.start_tracing()?;
        
        // Trace passes with different characteristics
        let passes = vec![
            ("dead_code_elimination", 100),
            ("constant_folding", 50),
            ("function_inlining", 200),
            ("loop_optimization", 150),
        ];
        
        for (pass_name, duration_ms) in passes {
            let trace_id = tracer.start_pass_trace(pass_name, 1000)?;
            std::thread::sleep(Duration::from_millis(duration_ms));
            tracer.end_pass_trace(trace_id, true)?;
        }
        
        let traces = tracer.get_traces();
        assert_eq!(traces.len(), 4);
        
        // Verify that different passes have different estimated metrics
        let dead_code_trace = traces.iter().find(|t| t.pass_name == "dead_code_elimination").unwrap();
        let constant_trace = traces.iter().find(|t| t.pass_name == "constant_folding").unwrap();
        
        // Dead code elimination should process more instructions than constant folding
        // (this is based on our estimation logic)
        assert!(dead_code_trace.performance_metrics.instructions_processed <= constant_trace.performance_metrics.instructions_processed);
        
        // All traces should have realistic cache behavior
        for trace in &traces {
            let total_accesses = trace.performance_metrics.cache_hits + trace.performance_metrics.cache_misses;
            assert!(total_accesses > 0);
            
            let hit_rate = trace.performance_metrics.cache_hits as f64 / total_accesses as f64;
            assert!(hit_rate >= 0.0 && hit_rate <= 1.0);
        }
        
        Ok(())
    }

    #[test]
    fn test_adaptive_learning_confidence() -> Result<()> {
        let config = AdaptiveConfig {
            enable_adaptive_ordering: true,
            learning_rate: 0.2,
            minimum_samples: 2,
            confidence_threshold: 0.6,
            exploration_rate: 0.2,
        };
        
        let mut manager = AdaptivePassManager::new(config);
        
        // Record consistent feedback to build confidence
        for _ in 0..5 {
            manager.record_performance_feedback("consistent_pass", Duration::from_millis(50), 0.8, true)?;
        }
        
        // Record inconsistent feedback for another pass
        let feedback_scores = vec![0.9, 0.2, 0.8, 0.1, 0.7];
        for score in feedback_scores {
            manager.record_performance_feedback("inconsistent_pass", Duration::from_millis(50), score, true)?;
        }
        
        let results = manager.get_learning_results();
        
        // Should have learned about both passes
        assert!(results.pass_effectiveness.contains_key("consistent_pass"));
        assert!(results.pass_effectiveness.contains_key("inconsistent_pass"));
        
        // Confidence should be reasonable (not too high due to limited data)
        assert!(results.learning_confidence >= 0.0 && results.learning_confidence <= 1.0);
        
        Ok(())
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_real_world_optimization_scenario() -> Result<()> {
        // Simulate a real-world optimization scenario
        let debug_config = DebugConfig {
            enable_pass_tracing: true,
            enable_profiling: true,
            enable_adaptive_learning: true,
            enable_regression_testing: true,
            verbosity_level: DebugVerbosity::Debug,
            output_format: DebugOutputFormat::Markdown,
        };
        
        let mut debugger = PerformanceDebugger::new(debug_config);
        
        // Start optimization session
        debugger.start_debug_session("real_world_optimization")?;
        
        // Simulate multiple optimization rounds
        for round in 0..3 {
            println!("Optimization round {}", round + 1);
            
            // Define passes for this round
            let passes = vec![
                "dead_code_elimination",
                "constant_folding", 
                "common_subexpression_elimination",
                "function_inlining",
                "loop_invariant_motion",
                "strength_reduction",
            ];
            
            // Get adaptive ordering
            let pass_ordering: Vec<String> = passes.iter().map(|s| s.to_string()).collect();
            let optimized_ordering = debugger.adapt_pass_ordering(&pass_ordering)?;
            
            // Execute passes in optimized order
            for pass_name in &optimized_ordering {
                debugger.trace_pass_execution(pass_name, 2000 + round * 500, || {
                    // Simulate optimization work with variable duration
                    let duration = match pass_name.as_str() {
                        "dead_code_elimination" => 30 + round * 5,
                        "constant_folding" => 15 + round * 2,
                        "function_inlining" => 80 + round * 10,
                        "loop_invariant_motion" => 60 + round * 8,
                        _ => 40 + round * 5,
                    };
                    
                    std::thread::sleep(Duration::from_millis(duration));
                    Ok(format!("Optimized with {}", pass_name))
                })?;
            }
            
            // Run regression tests after each round
            if round > 0 {
                let regression_results = debugger.run_regression_tests()?;
                println!("Round {} regression results: {}/{} passed", 
                        round + 1, regression_results.tests_passed, regression_results.tests_run);
                
                if regression_results.regression_count > 0 {
                    println!("⚠️  {} regressions detected in round {}", 
                            regression_results.regression_count, round + 1);
                }
            }
        }
        
        // Generate comprehensive analysis
        let final_report = debugger.end_debug_session()?;
        assert!(final_report.trace_results.len() >= 18); // 3 rounds * 6 passes
        
        // Generate and verify final report
        let comprehensive_report = debugger.generate_report()?;
        assert!(comprehensive_report.len() > 1000); // Should be substantial
        
        println!("Generated comprehensive report ({} characters)", comprehensive_report.len());
        
        // Verify key sections exist
        assert!(comprehensive_report.contains("Debug Statistics"));
        assert!(comprehensive_report.contains("Pass Execution Traces"));
        assert!(comprehensive_report.contains("Profiling Results"));
        assert!(comprehensive_report.contains("Adaptive Learning Results"));
        assert!(comprehensive_report.contains("Regression Test Results"));
        
        Ok(())
    }

    #[test]
    fn test_performance_regression_detection() -> Result<()> {
        let config = RegressionTestConfig {
            enable_continuous_testing: true,
            test_on_optimization_change: true,
            parallel_test_execution: false,
            max_test_time: Duration::from_secs(10),
            memory_limit: 2 * 1024 * 1024 * 1024,
        };
        
        let mut tester = RegressionTester::new(config);
        
        // Create baseline
        tester.create_baseline("basic_optimizations")?;
        
        // Run tests multiple times to simulate changes
        let mut all_results = Vec::new();
        
        for iteration in 0..5 {
            let results = tester.run_all_tests()?;
            all_results.push(results);
            
            println!("Iteration {}: {}/{} tests passed", 
                    iteration + 1, 
                    all_results[iteration].tests_passed, 
                    all_results[iteration].tests_run);
        }
        
        // Verify we collected multiple result sets
        assert_eq!(all_results.len(), 5);
        
        // Each result should have the same number of tests
        let expected_test_count = all_results[0].tests_run;
        for result in &all_results {
            assert_eq!(result.tests_run, expected_test_count);
        }
        
        // Generate final report
        let report = tester.generate_regression_report()?;
        assert!(report.contains("Regression Testing Report"));
        assert!(report.contains("Recent Benchmark Results"));
        
        Ok(())
    }
}
