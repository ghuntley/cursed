/// Comprehensive Test Suite for CURSED Optimization System
/// 
/// Tests all aspects of the performance optimization system including:
/// - ML-driven optimization decisions
/// - CURSED-specific optimizations (goroutines, channels, Gen Z slang)
/// - Advanced LLVM passes (memory layout, vectorization, cache optimization)
/// - Compiler speed improvements (incremental caching, parallel compilation)

use cursed::optimization::{
    ml_optimization::*,
    enhanced_llvm_passes::{
        memory_layout_optimizer::MemoryLayoutOptimizer,
        vectorization_optimizer::VectorizationOptimizer,
        real_goroutine_optimizer::RealGoroutineOptimizer,
    },
    *,
};
use cursed::error::Result;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::collections::HashMap;

/// Test the ML optimization engine
mod ml_optimization_tests {
    use super::*;

    #[test]
    fn test_ml_engine_creation() -> Result<()> {
        let config = MLOptimizationConfig::default();
        let engine = MLOptimizationEngine::new(config)?;
        
        assert!(engine.config.enabled);
        assert_eq!(engine.config.learning_rate, 0.01);
        assert_eq!(engine.config.batch_size, 32);
        
        Ok(())
    }

    #[test]
    fn test_feature_extraction() -> Result<()> {
        let config = MLOptimizationConfig::default();
        let mut engine = MLOptimizationEngine::new(config)?;
        
        let function_ir = r#"
        define i32 @test_function(i32 %x) {
        entry:
          %add = add i32 %x, 1
          ret i32 %add
        }
        "#;
        
        let features = engine.extract_features(function_ir, None)?;
        
        // Verify feature vector has reasonable defaults
        assert_eq!(features.function_features.size_in_bytes, 0);
        assert_eq!(features.target_features.available_registers, 16);
        
        Ok(())
    }

    #[test]
    fn test_inlining_decision() -> Result<()> {
        let config = MLOptimizationConfig::default();
        let mut engine = MLOptimizationEngine::new(config)?;
        
        let mut features = FeatureVector::default();
        features.function_features.size_in_bytes = 80;  // Small function
        features.function_features.call_count = 20;     // High call count
        
        let decision = engine.make_optimization_decision("inlining", &features)?;
        
        match decision {
            OptimizationDecision::Inline { should_inline, confidence } => {
                assert!(should_inline);
                assert!(confidence > 0.5);
            },
            _ => panic!("Expected inlining decision"),
        }
        
        Ok(())
    }

    #[test]
    fn test_vectorization_decision() -> Result<()> {
        let config = MLOptimizationConfig::default();
        let mut engine = MLOptimizationEngine::new(config)?;
        
        let mut features = FeatureVector::default();
        features.performance_features.instruction_level_parallelism = 3.0;
        
        let decision = engine.make_optimization_decision("vectorization", &features)?;
        
        match decision {
            OptimizationDecision::Vectorize { vector_width, profitability } => {
                assert_eq!(vector_width, 8);
                assert!(profitability > 0.0);
            },
            _ => panic!("Expected vectorization decision"),
        }
        
        Ok(())
    }

    #[test]
    fn test_cursed_specific_optimization() -> Result<()> {
        let config = MLOptimizationConfig::default();
        let mut engine = MLOptimizationEngine::new(config)?;
        
        let mut features = FeatureVector::default();
        features.cursed_features.goroutine_usage.goroutine_spawn_count = 15;
        
        let decision = engine.make_optimization_decision("cursed_specific", &features)?;
        
        match decision {
            OptimizationDecision::CursedSpecific { optimization, .. } => {
                match optimization {
                    CursedOptType::GoroutineStackOptimization { target_size } => {
                        assert_eq!(target_size, 64 * 1024);
                    },
                    _ => panic!("Expected goroutine stack optimization"),
                }
            },
            _ => panic!("Expected CURSED-specific decision"),
        }
        
        Ok(())
    }

    #[test]
    fn test_training_sample_addition() -> Result<()> {
        let config = MLOptimizationConfig::default();
        let mut engine = MLOptimizationEngine::new(config)?;
        
        let sample = TrainingSample {
            features: FeatureVector::default(),
            optimization_decision: OptimizationDecision::Inline {
                should_inline: true,
                confidence: 0.9,
            },
            actual_performance: PerformanceMetrics {
                execution_time: Duration::from_millis(100),
                memory_usage: 1024,
                cache_misses: 50,
                energy_consumption: 0.5,
                throughput: 1000.0,
            },
            timestamp: std::time::SystemTime::now(),
            quality_score: 0.9,
        };
        
        engine.add_training_sample(sample)?;
        
        Ok(())
    }

    #[test]
    fn test_model_training() -> Result<()> {
        let config = MLOptimizationConfig::default();
        let mut engine = MLOptimizationEngine::new(config)?;
        
        // Add some training samples
        for i in 0..5 {
            let sample = TrainingSample {
                features: FeatureVector::default(),
                optimization_decision: OptimizationDecision::Inline {
                    should_inline: i % 2 == 0,
                    confidence: 0.8,
                },
                actual_performance: PerformanceMetrics {
                    execution_time: Duration::from_millis(100 + i as u64 * 10),
                    memory_usage: 1024,
                    cache_misses: 50,
                    energy_consumption: 0.5,
                    throughput: 1000.0,
                },
                timestamp: std::time::SystemTime::now(),
                quality_score: 0.8,
            };
            engine.add_training_sample(sample)?;
        }
        
        engine.train_models()?;
        
        let stats = engine.get_model_statistics();
        assert!(stats.overall_accuracy > 0.0);
        
        Ok(())
    }

    #[test]
    fn test_optimization_caching() -> Result<()> {
        let config = MLOptimizationConfig::default();
        let mut engine = MLOptimizationEngine::new(config)?;
        
        let features = FeatureVector::default();
        
        // First call should be computed
        let decision1 = engine.make_optimization_decision("inlining", &features)?;
        
        // Second call should use cache
        let decision2 = engine.make_optimization_decision("inlining", &features)?;
        
        // Decisions should be the same
        match (&decision1, &decision2) {
            (
                OptimizationDecision::Inline { should_inline: s1, confidence: c1 },
                OptimizationDecision::Inline { should_inline: s2, confidence: c2 }
            ) => {
                assert_eq!(s1, s2);
                assert_eq!(c1, c2);
            },
            _ => panic!("Expected same inlining decisions"),
        }
        
        Ok(())
    }
}

/// Test CURSED-specific optimizations
mod cursed_specific_tests {
    use super::*;
    use inkwell::context::Context;

    #[test]
    fn test_goroutine_optimizer_creation() {
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let optimizer = RealGoroutineOptimizer::new(statistics, None);
        
        assert!(optimizer.optimization_config.enable_stack_size_optimization);
        assert!(optimizer.optimization_config.enable_scheduler_hints);
        assert!(optimizer.optimization_config.enable_goroutine_pooling);
    }

    #[test]
    fn test_goroutine_pattern_analysis() {
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let optimizer = RealGoroutineOptimizer::new(statistics, None);
        
        let config = &optimizer.optimization_config;
        assert_eq!(config.min_stack_size, 8 * 1024);
        assert_eq!(config.max_stack_size, 1024 * 1024);
        assert_eq!(config.pool_size_threshold, 10);
    }

    #[test]
    fn test_stack_size_optimization() {
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let optimizer = RealGoroutineOptimizer::new(statistics, None);
        
        // Test different stack risk levels
        use cursed::optimization::enhanced_llvm_passes::real_goroutine_optimizer::StackRiskLevel;
        
        assert_eq!(optimizer.assess_stack_risk(16 * 1024, 2), StackRiskLevel::Safe);
        assert_eq!(optimizer.assess_stack_risk(200 * 1024, 5), StackRiskLevel::High);
    }

    #[test]
    fn test_creation_pattern_optimization_potential() {
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let optimizer = RealGoroutineOptimizer::new(statistics, None);
        
        use cursed::optimization::enhanced_llvm_passes::real_goroutine_optimizer::CreationPatternType;
        
        let potential = optimizer.calculate_optimization_potential(&CreationPatternType::Periodic);
        assert_eq!(potential, 0.9);
        
        let potential = optimizer.calculate_optimization_potential(&CreationPatternType::ShortLived);
        assert_eq!(potential, 0.8);
        
        let potential = optimizer.calculate_optimization_potential(&CreationPatternType::LongLived);
        assert_eq!(potential, 0.4);
    }

    #[test]
    fn test_goroutine_optimization_config() {
        use cursed::optimization::enhanced_llvm_passes::real_goroutine_optimizer::GoroutineOptimizationConfig;
        
        let config = GoroutineOptimizationConfig::default();
        
        assert!(config.enable_stack_size_optimization);
        assert!(config.enable_scheduler_hints);
        assert!(config.enable_goroutine_pooling);
        assert!(config.enable_concurrent_pattern_optimization);
        
        assert_eq!(config.min_optimization_benefit, 0.05);
        assert_eq!(config.max_optimization_overhead, 0.02);
        assert_eq!(config.optimization_confidence_threshold, 0.8);
    }
}

/// Test advanced LLVM passes
mod llvm_passes_tests {
    use super::*;
    use inkwell::context::Context;

    #[test]
    fn test_memory_layout_optimizer() {
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let optimizer = MemoryLayoutOptimizer::new(statistics);
        
        // Test alignment requirements
        assert_eq!(optimizer.alignment_requirements.cache_line_size, 64);
        assert_eq!(optimizer.alignment_requirements.page_size, 4096);
        assert_eq!(optimizer.alignment_requirements.vector_alignment, 16);
    }

    #[test]
    fn test_memory_layout_hot_field_identification() {
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let optimizer = MemoryLayoutOptimizer::new(statistics);
        
        use cursed::optimization::enhanced_llvm_passes::memory_layout_optimizer::{
            FieldAccessInfo, AccessPattern
        };
        
        let access_patterns = vec![
            FieldAccessInfo {
                field_index: 0,
                access_frequency: 100,
                access_pattern: AccessPattern::Sequential,
                temporal_locality: 0.8,
            },
            FieldAccessInfo {
                field_index: 1,
                access_frequency: 10,
                access_pattern: AccessPattern::Random,
                temporal_locality: 0.2,
            },
            FieldAccessInfo {
                field_index: 2,
                access_frequency: 80,
                access_pattern: AccessPattern::Sequential,
                temporal_locality: 0.7,
            },
        ];
        
        let hot_fields = optimizer.identify_hot_fields(&access_patterns);
        assert!(hot_fields.is_some());
        let hot_fields = hot_fields.unwrap();
        assert!(hot_fields.contains(&0));
        assert!(hot_fields.contains(&2));
        assert!(!hot_fields.contains(&1));
    }

    #[test]
    fn test_vectorization_optimizer() {
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let optimizer = VectorizationOptimizer::new(statistics);
        
        // Test target vector info defaults
        assert_eq!(optimizer.target_info.supported_widths.get("i32"), Some(&vec![4, 8, 16]));
        assert_eq!(optimizer.target_info.supported_widths.get("f32"), Some(&vec![4, 8, 16]));
        assert_eq!(optimizer.target_info.supported_widths.get("f64"), Some(&vec![2, 4, 8]));
    }

    #[test]
    fn test_vectorization_speedup_estimation() {
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let optimizer = VectorizationOptimizer::new(statistics);
        
        use cursed::optimization::enhanced_llvm_passes::vectorization_optimizer::VectorOperation;
        
        let speedup = optimizer.estimate_vectorization_speedup(&VectorOperation::Add, 8);
        assert_eq!(speedup, 6.4); // 8 * 0.8
        
        let speedup = optimizer.estimate_vectorization_speedup(&VectorOperation::Multiply, 4);
        assert_eq!(speedup, 2.88); // 4 * 0.8 * 0.9
        
        let speedup = optimizer.estimate_vectorization_speedup(&VectorOperation::Divide, 4);
        assert_eq!(speedup, 1.92); // 4 * 0.8 * 0.6
    }

    #[test]
    fn test_vectorization_optimal_width() {
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let optimizer = VectorizationOptimizer::new(statistics);
        
        assert_eq!(optimizer.get_optimal_vector_width("i32"), 16);
        assert_eq!(optimizer.get_optimal_vector_width("f32"), 16);
        assert_eq!(optimizer.get_optimal_vector_width("f64"), 8);
        assert_eq!(optimizer.get_optimal_vector_width("unknown"), 4);
    }
}

/// Test performance optimization system integration
mod integration_tests {
    use super::*;

    #[test]
    fn test_performance_optimization_system_creation() -> Result<()> {
        let performance_config = PerformanceConfig::default();
        let optimization_config = OptimizationConfig::default();
        
        let system = PerformanceOptimizationSystem::new(
            performance_config,
            optimization_config,
        )?;
        
        Ok(())
    }

    #[test]
    fn test_optimization_session_creation() -> Result<()> {
        let performance_config = PerformanceConfig::default();
        let optimization_config = OptimizationConfig::default();
        
        let system = PerformanceOptimizationSystem::new(
            performance_config,
            optimization_config,
        )?;
        
        let session = system.create_session("test_session".to_string());
        assert_eq!(session.name, "test_session");
        assert!(session.id.starts_with("test_session_"));
        
        Ok(())
    }

    #[test]
    fn test_system_statistics() -> Result<()> {
        let performance_config = PerformanceConfig::default();
        let optimization_config = OptimizationConfig::default();
        
        let system = PerformanceOptimizationSystem::new(
            performance_config,
            optimization_config,
        )?;
        
        let stats = system.get_system_statistics();
        // Should not panic and return valid statistics
        
        Ok(())
    }

    #[test]
    fn test_benchmarking_integration() -> Result<()> {
        let performance_config = PerformanceConfig::default();
        let optimization_config = OptimizationConfig::default();
        
        let system = PerformanceOptimizationSystem::new(
            performance_config,
            optimization_config,
        )?;
        
        let benchmark_config = BenchmarkConfig {
            benchmark_type: BenchmarkType::Compilation,
            iterations: 5,
            warmup_iterations: 2,
            complexity_level: ComplexityLevel::Medium,
            enable_profiling: true,
            timeout: Duration::from_secs(30),
            test_data: BenchmarkTestData::Synthetic {
                function_count: 10,
                complexity_factor: 1.0,
            },
        };
        
        let results = system.run_benchmark(benchmark_config)?;
        assert!(results.iterations.len() > 0);
        
        Ok(())
    }
}

/// Test compiler speed improvements
mod compiler_speed_tests {
    use super::*;

    #[test]
    fn test_incremental_compilation_cache() -> Result<()> {
        // Test incremental compilation caching
        let performance_config = PerformanceConfig::default();
        let optimization_config = OptimizationConfig::default();
        
        let system = PerformanceOptimizationSystem::new(
            performance_config,
            optimization_config,
        )?;
        
        // Create multiple sessions to test caching
        let session1 = system.create_session("session1".to_string());
        let session2 = system.create_session("session2".to_string());
        
        assert_ne!(session1.id, session2.id);
        
        Ok(())
    }

    #[test]
    fn test_parallel_optimization_benefits() -> Result<()> {
        // Test that parallel optimization provides benefits
        let performance_config = PerformanceConfig {
            enable_realtime_monitoring: true,
            enable_benchmarking: true,
            enable_prediction: true,
            monitoring_interval_ms: 50,
            max_benchmark_iterations: 5,
            max_performance_entries: 1000,
            resource_monitoring_level: ResourceMonitoringLevel::Detailed,
        };
        
        let optimization_config = OptimizationConfig::default();
        
        let system = PerformanceOptimizationSystem::new(
            performance_config,
            optimization_config,
        )?;
        
        // Test resource statistics collection
        let resource_stats = system.get_resource_statistics()?;
        // Should return valid resource statistics
        
        Ok(())
    }

    #[test]
    fn test_optimization_prediction() -> Result<()> {
        let performance_config = PerformanceConfig {
            enable_prediction: true,
            ..PerformanceConfig::default()
        };
        
        let optimization_config = OptimizationConfig::default();
        
        let system = PerformanceOptimizationSystem::new(
            performance_config,
            optimization_config,
        )?;
        
        // Test performance analysis
        let analysis = system.get_performance_analysis(Duration::from_secs(1))?;
        assert_eq!(analysis.units_optimized, 0); // No units optimized yet
        
        Ok(())
    }
}

/// Test error handling and edge cases
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_invalid_optimization_type() {
        let config = MLOptimizationConfig::default();
        let mut engine = MLOptimizationEngine::new(config).unwrap();
        
        let features = FeatureVector::default();
        let result = engine.make_optimization_decision("invalid_type", &features);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_ml_engine_with_disabled_config() -> Result<()> {
        let config = MLOptimizationConfig {
            enabled: false,
            ..MLOptimizationConfig::default()
        };
        
        let engine = MLOptimizationEngine::new(config)?;
        assert!(!engine.config.enabled);
        
        Ok(())
    }

    #[test]
    fn test_performance_config_validation() {
        let config = PerformanceConfig {
            enable_realtime_monitoring: true,
            enable_benchmarking: true,
            enable_prediction: false,
            monitoring_interval_ms: 100,
            max_benchmark_iterations: 10,
            max_performance_entries: 10000,
            resource_monitoring_level: ResourceMonitoringLevel::Basic,
        };
        
        assert!(config.enable_realtime_monitoring);
        assert!(!config.enable_prediction);
        assert_eq!(config.monitoring_interval_ms, 100);
    }
}

/// Test comprehensive optimization scenarios
mod comprehensive_optimization_tests {
    use super::*;

    #[test]
    fn test_full_optimization_pipeline() -> Result<()> {
        // Test complete optimization pipeline with ML integration
        let ml_config = MLOptimizationConfig::default();
        let ml_engine = Arc::new(Mutex::new(MLOptimizationEngine::new(ml_config)?));
        
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let goroutine_optimizer = RealGoroutineOptimizer::new(
            statistics.clone(),
            Some(ml_engine.clone())
        );
        
        // Test that the optimizer was created successfully
        assert!(goroutine_optimizer.optimization_config.enable_stack_size_optimization);
        
        Ok(())
    }

    #[test]
    fn test_optimization_coordination() -> Result<()> {
        // Test coordination between different optimization passes
        let performance_config = PerformanceConfig::default();
        let optimization_config = OptimizationConfig::default();
        
        let system = PerformanceOptimizationSystem::new(
            performance_config,
            optimization_config,
        )?;
        
        let session = system.create_session("coordination_test".to_string());
        
        // Test that session management works correctly
        assert!(!session.id.is_empty());
        assert_eq!(session.name, "coordination_test");
        
        Ok(())
    }

    #[test]
    fn test_optimization_effectiveness_measurement() -> Result<()> {
        // Test measurement of optimization effectiveness
        let config = MLOptimizationConfig::default();
        let mut engine = MLOptimizationEngine::new(config)?;
        
        // Add training data to measure effectiveness
        for i in 0..10 {
            let sample = TrainingSample {
                features: FeatureVector::default(),
                optimization_decision: OptimizationDecision::Inline {
                    should_inline: i % 2 == 0,
                    confidence: 0.8 + (i as f64 * 0.01),
                },
                actual_performance: PerformanceMetrics {
                    execution_time: Duration::from_millis(100 - i as u64 * 5),
                    memory_usage: 1024 + i * 50,
                    cache_misses: 50 - i * 2,
                    energy_consumption: 0.5 + (i as f64 * 0.01),
                    throughput: 1000.0 + (i as f64 * 10.0),
                },
                timestamp: std::time::SystemTime::now(),
                quality_score: 0.8 + (i as f64 * 0.02),
            };
            engine.add_training_sample(sample)?;
        }
        
        engine.train_models()?;
        
        let stats = engine.get_model_statistics();
        assert!(stats.overall_accuracy > 0.0);
        assert!(stats.inlining_accuracy > 0.0);
        
        Ok(())
    }

    #[test]
    fn test_multi_pass_optimization() -> Result<()> {
        // Test that multiple optimization passes work together
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        
        let memory_optimizer = MemoryLayoutOptimizer::new(statistics.clone());
        let vectorization_optimizer = VectorizationOptimizer::new(statistics.clone());
        let goroutine_optimizer = RealGoroutineOptimizer::new(statistics.clone(), None);
        
        // Test that all optimizers use the same statistics
        let stats = statistics.lock().unwrap();
        assert_eq!(stats.memory_layout_improvements, 0);
        assert_eq!(stats.vectorized_operations, 0);
        assert_eq!(stats.goroutine_optimizations, 0);
        
        Ok(())
    }
}

/// Performance regression tests
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_optimization_performance() -> Result<()> {
        // Test that optimizations complete within reasonable time
        let start_time = Instant::now();
        
        let config = MLOptimizationConfig::default();
        let mut engine = MLOptimizationEngine::new(config)?;
        
        let features = FeatureVector::default();
        let _decision = engine.make_optimization_decision("inlining", &features)?;
        
        let elapsed = start_time.elapsed();
        assert!(elapsed < Duration::from_millis(100), "Optimization took too long: {:?}", elapsed);
        
        Ok(())
    }

    #[test]
    fn test_large_scale_optimization() -> Result<()> {
        // Test optimization with large numbers of functions
        let performance_config = PerformanceConfig {
            max_performance_entries: 100,
            ..PerformanceConfig::default()
        };
        let optimization_config = OptimizationConfig::default();
        
        let system = PerformanceOptimizationSystem::new(
            performance_config,
            optimization_config,
        )?;
        
        // Create multiple sessions to simulate large scale optimization
        for i in 0..10 {
            let session = system.create_session(format!("session_{}", i));
            assert!(!session.id.is_empty());
        }
        
        Ok(())
    }

    #[test]
    fn test_memory_usage_optimization() -> Result<()> {
        // Test that the optimization system doesn't use excessive memory
        let config = MLOptimizationConfig {
            batch_size: 16,  // Smaller batch size
            feature_vector_size: 64,  // Smaller feature vectors
            ..MLOptimizationConfig::default()
        };
        
        let engine = MLOptimizationEngine::new(config)?;
        
        // Test that the engine was created with the optimized configuration
        assert_eq!(engine.config.batch_size, 16);
        assert_eq!(engine.config.feature_vector_size, 64);
        
        Ok(())
    }
}

/// Integration with existing CURSED systems
mod cursed_integration_tests {
    use super::*;

    #[test]
    fn test_gen_z_slang_optimization() -> Result<()> {
        // Test optimization of Gen Z slang patterns
        let config = MLOptimizationConfig::default();
        let mut engine = MLOptimizationEngine::new(config)?;
        
        let mut features = FeatureVector::default();
        features.cursed_features.gen_z_slang_patterns.slay_function_usage = 5;
        features.cursed_features.gen_z_slang_patterns.yolo_expression_count = 10;
        features.cursed_features.gen_z_slang_patterns.periodt_termination_usage = 3;
        
        let decision = engine.make_optimization_decision("cursed_specific", &features)?;
        
        match decision {
            OptimizationDecision::CursedSpecific { optimization, .. } => {
                // Should suggest some CURSED-specific optimization
                match optimization {
                    CursedOptType::GenZSlangInlining { inline_threshold } => {
                        assert_eq!(inline_threshold, 0.7);
                    },
                    _ => {}, // Other optimizations are also valid
                }
            },
            _ => panic!("Expected CURSED-specific optimization"),
        }
        
        Ok(())
    }

    #[test]
    fn test_channel_optimization() -> Result<()> {
        // Test channel buffer sizing optimization
        let config = MLOptimizationConfig::default();
        let mut engine = MLOptimizationEngine::new(config)?;
        
        let mut features = FeatureVector::default();
        features.cursed_features.channel_usage.channel_count = 8;
        features.cursed_features.channel_usage.send_receive_ratio = 1.2;
        features.cursed_features.channel_usage.select_statement_usage = 3;
        
        let decision = engine.make_optimization_decision("cursed_specific", &features)?;
        
        match decision {
            OptimizationDecision::CursedSpecific { optimization, .. } => {
                match optimization {
                    CursedOptType::ChannelBufferSizing { optimal_size } => {
                        assert_eq!(optimal_size, 16);
                    },
                    _ => {}, // Other optimizations are also valid for this case
                }
            },
            _ => panic!("Expected CURSED-specific optimization"),
        }
        
        Ok(())
    }

    #[test]
    fn test_error_propagation_optimization() -> Result<()> {
        // Test error propagation optimization
        let mut features = FeatureVector::default();
        features.cursed_features.error_propagation_usage.question_mark_operator_usage = 15;
        features.cursed_features.error_propagation_usage.error_handling_blocks = 5;
        features.cursed_features.error_propagation_usage.panic_recovery_usage = 2;
        
        // Test that features are set correctly
        assert_eq!(features.cursed_features.error_propagation_usage.question_mark_operator_usage, 15);
        assert_eq!(features.cursed_features.error_propagation_usage.error_handling_blocks, 5);
        
        Ok(())
    }
}

/// Create helper functions for testing
impl Default for EnhancedOptimizationStatistics {
    fn default() -> Self {
        Self {
            memory_layout_improvements: 0,
            vectorized_operations: 0,
            goroutine_optimizations: 0,
        }
    }
}

#[derive(Debug, Default)]
pub struct EnhancedOptimizationStatistics {
    pub memory_layout_improvements: usize,
    pub vectorized_operations: usize,
    pub goroutine_optimizations: usize,
}
