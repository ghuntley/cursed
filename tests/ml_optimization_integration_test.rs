/// Integration tests for ML-Guided Optimization System
/// 
/// Tests the complete ML optimization pipeline including feature extraction,
/// model training, prediction, data collection, and continuous learning.

use cursed::optimization::ml::*;
use cursed::optimization::ml::feature_extraction::*;
use cursed::optimization::ml::model_training::*;
use cursed::optimization::ml::prediction::*;
use cursed::optimization::ml::data_collection::*;
use cursed::optimization::ml::continuous_learning::*;
use cursed::error::Result;
use std::time::Duration;
use std::collections::HashMap;

#[test]
fn test_ml_optimization_coordinator_creation() -> Result<()> {
    let config = MLOptimizationConfig::default();
    let coordinator = MLOptimizationCoordinator::new(config)?;
    
    // Verify coordinator was created successfully
    assert!(coordinator.get_performance_statistics().is_ok());
    
    Ok(())
}

#[test]
fn test_feature_extraction_basic() -> Result<()> {
    let config = FeatureConfig::default();
    let mut extractor = FeatureExtractor::new(config)?;
    
    let sample_code = r#"
        slay fibonacci(sus n: i64) -> i64 {
            lowkey (n <= 1) {
                return n;
            }
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
    "#;
    
    let features = extractor.extract_features(sample_code, None, None)?;
    
    // Verify basic features were extracted
    assert!(features.syntax_features.function_count > 0);
    assert!(features.syntax_features.lines_of_code > 0);
    assert!(features.cursed_features.slang_features.slay_functions > 0);
    
    Ok(())
}

#[test]
fn test_feature_extraction_cursed_specific() -> Result<()> {
    let config = FeatureConfig::default();
    let mut extractor = FeatureExtractor::new(config)?;
    
    let cursed_code = r#"
        slay concurrent_worker() {
            stan process_data();
            facts channel_data = channel<i64>();
            yolo;
            periodt (sus i = 0; i < 10; i++) {
                channel_data <- i;
            }
        }
        
        collab Processor {
            slay process(sus data: i64) -> i64;
        }
        
        slay safe_operation() -> Result<i64, string> {
            facts result = risky_operation()?;
            Ok(result)
        }
    "#;
    
    let features = extractor.extract_features(cursed_code, None, None)?;
    
    // Verify CURSED-specific features
    assert!(features.cursed_features.goroutine_features.goroutine_spawns > 0);
    assert!(features.cursed_features.channel_features.channel_declarations > 0);
    assert!(features.cursed_features.slang_features.stan_goroutine_spawns > 0);
    assert!(features.cursed_features.slang_features.yolo_expressions > 0);
    assert!(features.cursed_features.slang_features.periodt_statements > 0);
    assert!(features.cursed_features.interface_features.interface_declarations > 0);
    assert!(features.cursed_features.error_handling_features.question_mark_operators > 0);
    
    Ok(())
}

#[test]
fn test_model_trainer_creation() -> Result<()> {
    let config = TrainingConfig::default();
    let trainer = ModelTrainer::new(config)?;
    
    // Verify trainer was created with all model types
    let statistics = trainer.get_training_statistics();
    assert_eq!(statistics.models_trained, 10); // All model types
    
    Ok(())
}

#[test]
fn test_model_training_with_sample_data() -> Result<()> {
    let config = TrainingConfig {
        max_epochs: 10, // Reduced for testing
        batch_size: 16,
        ..TrainingConfig::default()
    };
    let mut trainer = ModelTrainer::new(config)?;
    
    // Create sample training data
    let training_data = create_sample_training_data()?;
    
    // Train models
    trainer.train_all_models(&training_data)?;
    
    // Verify training completed
    let statistics = trainer.get_training_statistics();
    assert!(statistics.successful_updates > 0);
    assert!(statistics.total_training_time > Duration::from_secs(0));
    
    Ok(())
}

#[test]
fn test_prediction_engine() -> Result<()> {
    let config = PredictionConfig::default();
    let mut predictor = OptimizationPredictor::new(config)?;
    
    // Create mock trained models
    let trained_models = create_mock_trained_models()?;
    predictor.update_models(trained_models)?;
    
    // Test prediction
    let features = create_sample_feature_vector();
    let context = create_sample_prediction_context();
    
    let prediction = predictor.predict_optimization_strategy(&features, Some(&context))?;
    
    // Verify prediction structure
    assert!(prediction.confidence > 0.0);
    assert!(prediction.confidence <= 1.0);
    assert!(!prediction.explanation.model_reasoning.is_empty());
    assert!(prediction.prediction_time > Duration::from_nanos(0));
    
    Ok(())
}

#[test]
fn test_data_collection() -> Result<()> {
    let mut collector = PerformanceDataCollector::new()?;
    
    // Create sample data
    let source_code = "slay test() { return 42; }";
    let context = CompilationContext::default();
    let strategy = create_sample_optimization_strategy();
    let compilation_metrics = create_sample_compilation_metrics();
    let runtime_metrics = create_sample_runtime_metrics();
    
    // Record data
    collector.record_compilation_data(source_code, &context, &strategy, &compilation_metrics)?;
    collector.record_runtime_data(source_code, &context, &strategy, &runtime_metrics)?;
    
    // Get training data
    let training_data = collector.get_training_data()?;
    assert!(!training_data.is_empty());
    
    Ok(())
}

#[test]
fn test_continuous_learning_engine() -> Result<()> {
    let config = LearningConfig {
        min_samples_for_update: 5, // Reduced for testing
        ..LearningConfig::default()
    };
    let mut learning_engine = ContinuousLearningEngine::new(config)?;
    
    // Process some sample data
    for i in 0..10 {
        let source_code = format!("slay test_{i}() {{ return {i}; }}");
        let context = CompilationContext::default();
        let strategy = create_sample_optimization_strategy();
        let compilation_metrics = create_sample_compilation_metrics();
        let runtime_metrics = create_sample_runtime_metrics();
        
        learning_engine.process_new_data(
            &source_code,
            &context,
            &strategy,
            &compilation_metrics,
            &runtime_metrics,
        )?;
    }
    
    // Check if update is needed
    let should_update = learning_engine.should_update_models()?;
    assert!(should_update);
    
    // Verify statistics
    let statistics = learning_engine.get_statistics();
    assert!(statistics.total_learning_iterations > 0);
    
    Ok(())
}

#[test]
fn test_end_to_end_optimization_workflow() -> Result<()> {
    // Create ML optimization coordinator
    let config = MLOptimizationConfig::default();
    let mut coordinator = MLOptimizationCoordinator::new(config)?;
    
    // Sample code to optimize
    let source_code = r#"
        slay bubble_sort(sus arr: [i64; 100]) -> [i64; 100] {
            facts result = arr;
            periodt (sus i = 0; i < 100; i++) {
                periodt (sus j = 0; j < 100 - i - 1; j++) {
                    lowkey (result[j] > result[j + 1]) {
                        facts temp = result[j];
                        result[j] = result[j + 1];
                        result[j + 1] = temp;
                    }
                }
            }
            return result;
        }
    "#;
    
    let context = CompilationContext::default();
    
    // Get optimization recommendation
    let strategy = coordinator.recommend_optimization_strategy(source_code, &context)?;
    
    // Verify strategy was generated
    assert!(strategy.confidence > 0.0);
    assert!(!strategy.reasoning.is_empty());
    assert!(!strategy.enabled_passes.is_empty());
    
    // Simulate applying the optimization and recording results
    let compilation_metrics = CompilationMetrics {
        compilation_time: Duration::from_millis(150),
        memory_peak_usage: 1024 * 1024,
        binary_size: 8192,
        binary_size_change: -0.05, // 5% smaller
        optimization_passes_applied: vec!["dce".to_string(), "inline".to_string()],
        pass_execution_times: HashMap::new(),
        llvm_ir_size: 4096,
        assembly_size: 6144,
        linking_time: Duration::from_millis(25),
        errors_encountered: vec![],
        warnings_generated: vec![],
        cache_hit_rate: 0.8,
    };
    
    let runtime_metrics = RuntimeMetrics {
        execution_time: Duration::from_millis(50),
        execution_time_improvement: 0.15, // 15% faster
        memory_usage_peak: 512 * 1024,
        memory_usage_average: 256 * 1024,
        memory_usage_change: -0.1, // 10% less memory
        cpu_utilization: 0.75,
        cache_miss_rate: 0.03,
        branch_miss_rate: 0.02,
        page_faults: 50,
        context_switches: 10,
        system_calls: 100,
        energy_consumption: 5.0,
        energy_consumption_change: -0.08, // 8% less energy
        throughput: 200.0,
        latency_p50: Duration::from_millis(25),
        latency_p95: Duration::from_millis(40),
        latency_p99: Duration::from_millis(60),
        error_rate: 0.0,
    };
    
    // Record the outcome for learning
    coordinator.record_optimization_outcome(
        source_code,
        &context,
        &strategy,
        &compilation_metrics,
        &runtime_metrics,
    )?;
    
    // Get performance statistics
    let statistics = coordinator.get_performance_statistics()?;
    assert!(statistics.total_recommendations > 0);
    
    Ok(())
}

#[test]
fn test_optimization_strategy_explanation() -> Result<()> {
    let config = PredictionConfig {
        enable_explanation_generation: true,
        max_alternative_options: 2,
        ..PredictionConfig::default()
    };
    let mut predictor = OptimizationPredictor::new(config)?;
    
    // Create mock trained models
    let trained_models = create_mock_trained_models()?;
    predictor.update_models(trained_models)?;
    
    // Create feature vector with notable characteristics
    let mut features = create_sample_feature_vector();
    features.syntax_features.loop_count = 5;
    features.cursed_features.goroutine_features.goroutine_spawns = 3;
    features.cursed_features.channel_features.channel_declarations = 2;
    
    let context = create_sample_prediction_context();
    let prediction = predictor.predict_optimization_strategy(&features, Some(&context))?;
    
    // Verify explanation quality
    assert!(!prediction.explanation.primary_factors.is_empty());
    assert!(!prediction.explanation.model_reasoning.is_empty());
    assert!(!prediction.explanation.feature_importance.is_empty());
    
    // Verify alternative strategies
    assert!(!prediction.alternative_strategies.is_empty());
    assert!(prediction.alternative_strategies.len() <= 2);
    
    Ok(())
}

#[test]
fn test_performance_regression_detection() -> Result<()> {
    let mut collector = PerformanceDataCollector::new()?;
    
    // Establish baseline
    let baseline_compilation = CompilationMetrics {
        compilation_time: Duration::from_millis(100),
        memory_peak_usage: 1024 * 1024,
        binary_size: 8192,
        binary_size_change: 0.0,
        optimization_passes_applied: vec!["dce".to_string()],
        pass_execution_times: HashMap::new(),
        llvm_ir_size: 4096,
        assembly_size: 6144,
        linking_time: Duration::from_millis(20),
        errors_encountered: vec![],
        warnings_generated: vec![],
        cache_hit_rate: 0.8,
    };
    
    let baseline_runtime = RuntimeMetrics {
        execution_time: Duration::from_millis(100),
        execution_time_improvement: 0.0,
        memory_usage_peak: 512 * 1024,
        memory_usage_average: 256 * 1024,
        memory_usage_change: 0.0,
        cpu_utilization: 0.7,
        cache_miss_rate: 0.05,
        branch_miss_rate: 0.02,
        page_faults: 100,
        context_switches: 20,
        system_calls: 200,
        energy_consumption: 10.0,
        energy_consumption_change: 0.0,
        throughput: 100.0,
        latency_p50: Duration::from_millis(50),
        latency_p95: Duration::from_millis(80),
        latency_p99: Duration::from_millis(120),
        error_rate: 0.0,
    };
    
    collector.establish_baseline(
        "baseline_v1".to_string(),
        baseline_compilation,
        baseline_runtime,
    )?;
    
    // Simulate a regression
    let regression_runtime = RuntimeMetrics {
        execution_time_improvement: -0.2, // 20% slower - regression!
        memory_usage_change: 0.3, // 30% more memory - also bad
        ..create_sample_runtime_metrics()
    };
    
    let context = CompilationContext::default();
    let strategy = create_sample_optimization_strategy();
    let compilation_metrics = create_sample_compilation_metrics();
    
    collector.record_runtime_data(
        "test_regression",
        &context,
        &strategy,
        &regression_runtime,
    )?;
    
    // Verify data was recorded
    let training_data = collector.get_training_data()?;
    assert!(!training_data.is_empty());
    
    // Find the regression data point
    let regression_point = training_data.iter()
        .find(|point| point.runtime_metrics.execution_time_improvement < 0.0);
    assert!(regression_point.is_some());
    
    Ok(())
}

#[test]
fn test_cache_effectiveness() -> Result<()> {
    let config = PredictionConfig {
        cache_predictions: true,
        cache_size: 100,
        ..PredictionConfig::default()
    };
    let mut predictor = OptimizationPredictor::new(config)?;
    
    // Create mock trained models
    let trained_models = create_mock_trained_models()?;
    predictor.update_models(trained_models)?;
    
    let features = create_sample_feature_vector();
    let context = create_sample_prediction_context();
    
    // First prediction should be cache miss
    let start_time = std::time::Instant::now();
    let prediction1 = predictor.predict_optimization_strategy(&features, Some(&context))?;
    let first_duration = start_time.elapsed();
    
    // Second prediction should be cache hit (faster)
    let start_time = std::time::Instant::now();
    let prediction2 = predictor.predict_optimization_strategy(&features, Some(&context))?;
    let second_duration = start_time.elapsed();
    
    // Verify predictions are the same
    assert_eq!(prediction1.confidence, prediction2.confidence);
    assert_eq!(prediction1.optimization_strategy.optimization_level, 
               prediction2.optimization_strategy.optimization_level);
    
    // Second prediction should be faster (cached)
    assert!(second_duration <= first_duration);
    
    // Check cache statistics
    let statistics = predictor.get_statistics();
    assert!(statistics.cache_hits > 0 || statistics.cache_misses > 0);
    
    Ok(())
}

// Helper functions for creating test data

fn create_sample_training_data() -> Result<Vec<TrainingDataPoint>> {
    let mut training_data = Vec::new();
    
    for i in 0..20 {
        let features = create_sample_feature_vector();
        let context = CompilationContext::default();
        let strategy = create_sample_optimization_strategy();
        let compilation_metrics = create_sample_compilation_metrics();
        let runtime_metrics = create_sample_runtime_metrics();
        
        training_data.push(TrainingDataPoint {
            source_identifier: format!("test_source_{}", i),
            features,
            compilation_context: context,
            optimization_strategy: strategy,
            compilation_metrics,
            runtime_metrics,
            timestamp: std::time::SystemTime::now(),
            quality_score: 0.8 + (i as f64 * 0.01), // Varying quality
            validation_status: ValidationStatus::Valid,
        });
    }
    
    Ok(training_data)
}

fn create_mock_trained_models() -> Result<HashMap<ModelType, ModelParameters>> {
    let mut models = HashMap::new();
    
    for model_type in [
        ModelType::FunctionInlining,
        ModelType::LoopOptimization,
        ModelType::Vectorization,
        ModelType::RegisterAllocation,
        ModelType::MemoryOptimization,
        ModelType::GoroutineOptimization,
        ModelType::ChannelOptimization,
        ModelType::ErrorPropagationOptimization,
        ModelType::CompilerPassSelection,
        ModelType::OptimizationLevelPrediction,
    ] {
        let parameters = ModelParameters {
            weights: vec![vec![0.5; 128]],
            biases: vec![0.1],
            hyperparameters: HashMap::new(),
            architecture: ModelArchitecture {
                model_type: format!("{:?}", model_type),
                layer_sizes: vec![128, 64, 32, 1],
                activation_functions: vec!["relu".to_string(), "relu".to_string(), "sigmoid".to_string()],
                dropout_rates: vec![0.1, 0.2, 0.0],
            },
        };
        
        models.insert(model_type, parameters);
    }
    
    Ok(models)
}

fn create_sample_feature_vector() -> FeatureVector {
    FeatureVector {
        syntax_features: SyntaxFeatures {
            lines_of_code: 50,
            function_count: 3,
            loop_count: 2,
            conditional_count: 5,
            cyclomatic_complexity: 8.0,
            ..SyntaxFeatures::default()
        },
        semantic_features: SemanticFeatures::default(),
        performance_features: PerformanceFeatures {
            execution_frequency: 100.0,
            instruction_level_parallelism: 2.5,
            ..PerformanceFeatures::default()
        },
        target_features: TargetFeatures::default(),
        cursed_features: CursedSpecificFeatures {
            goroutine_features: GoroutineFeatures {
                goroutine_spawns: 2,
                ..GoroutineFeatures::default()
            },
            channel_features: ChannelFeatures {
                channel_declarations: 1,
                ..ChannelFeatures::default()
            },
            ..CursedSpecificFeatures::default()
        },
        context_features: ContextFeatures::default(),
    }
}

fn create_sample_prediction_context() -> PredictionContext {
    PredictionContext {
        compilation_target: CompilationTarget {
            target_arch: "x86_64".to_string(),
            target_os: "linux".to_string(),
            cpu_features: vec!["sse2".to_string(), "avx".to_string()],
            memory_model: "little_endian".to_string(),
            deployment_scenario: DeploymentScenario::Production,
        },
        performance_requirements: PerformanceRequirements {
            priority: PerformancePriority::Speed,
            latency_requirements: Some(Duration::from_millis(100)),
            throughput_requirements: Some(1000.0),
            memory_constraints: Some(1024 * 1024 * 1024), // 1GB
            energy_constraints: None,
        },
        resource_constraints: ResourceConstraints {
            max_compilation_time: Some(Duration::from_secs(300)),
            max_memory_usage: Some(4 * 1024 * 1024 * 1024), // 4GB
            parallel_compilation: true,
            available_cores: 8,
        },
        historical_performance: None,
    }
}

fn create_sample_optimization_strategy() -> OptimizationStrategy {
    OptimizationStrategy {
        optimization_level: OptimizationLevel::Speed,
        enabled_passes: vec![
            OptimizationPass::DeadCodeElimination,
            OptimizationPass::ConstantPropagation,
            OptimizationPass::Inlining { aggressiveness: 0.7 },
        ],
        pass_parameters: HashMap::new(),
        confidence: 0.85,
        reasoning: "Speed optimization with aggressive inlining".to_string(),
        estimated_performance_gain: 1.3,
    }
}

fn create_sample_compilation_metrics() -> CompilationMetrics {
    CompilationMetrics {
        compilation_time: Duration::from_millis(200),
        memory_peak_usage: 2 * 1024 * 1024, // 2MB
        binary_size: 16384, // 16KB
        binary_size_change: -0.02, // 2% smaller
        optimization_passes_applied: vec!["dce".to_string(), "inline".to_string()],
        pass_execution_times: {
            let mut times = HashMap::new();
            times.insert("dce".to_string(), Duration::from_millis(50));
            times.insert("inline".to_string(), Duration::from_millis(100));
            times
        },
        llvm_ir_size: 8192,
        assembly_size: 12288,
        linking_time: Duration::from_millis(30),
        errors_encountered: vec![],
        warnings_generated: vec!["unused variable".to_string()],
        cache_hit_rate: 0.75,
    }
}

fn create_sample_runtime_metrics() -> RuntimeMetrics {
    RuntimeMetrics {
        execution_time: Duration::from_millis(80),
        execution_time_improvement: 0.12, // 12% faster
        memory_usage_peak: 1024 * 1024, // 1MB
        memory_usage_average: 512 * 1024, // 512KB
        memory_usage_change: -0.05, // 5% less memory
        cpu_utilization: 0.8,
        cache_miss_rate: 0.04,
        branch_miss_rate: 0.015,
        page_faults: 25,
        context_switches: 5,
        system_calls: 150,
        energy_consumption: 8.0,
        energy_consumption_change: -0.06, // 6% less energy
        throughput: 125.0,
        latency_p50: Duration::from_millis(40),
        latency_p95: Duration::from_millis(70),
        latency_p99: Duration::from_millis(100),
        error_rate: 0.0,
    }
}
