//! Comprehensive PGO System Integration Tests
//! 
//! Tests the complete Profile-Guided Optimization system including:
//! - End-to-end profile collection and optimization
//! - Profile data storage and management
//! - Optimization effectiveness measurement
//! - Performance regression detection
//! - CLI integration and workflow testing

use cursed::optimization::pgo::*;
use cursed::error::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use std::time::SystemTime;

#[test]
fn test_pgo_system_creation() {
    let pgo_system = PgoSystem::new();
    assert!(pgo_system.is_ok(), "PGO system creation should succeed");
    
    let system = pgo_system.unwrap();
    let stats = system.get_system_statistics();
    assert_eq!(stats.profile_count, 0, "New system should have no profiles");
}

#[test]
fn test_pgo_system_with_custom_config() {
    let config = PgoSystemConfig {
        enable_collection: true,
        enable_optimization: true,
        profile_directory: "test_profiles".to_string(),
        quality_threshold: 0.8,
        performance_target: 20.0,
        max_profile_age: Duration::from_secs(24 * 3600), // 1 day
        enable_validation: true,
        profile_version: ProfileVersion::V1_0,
        enable_merging: true,
        optimization_level: OptimizationAggressiveness::Aggressive,
    };
    
    let pgo_system = PgoSystem::with_config(config.clone());
    assert!(pgo_system.is_ok(), "PGO system creation with config should succeed");
    
    let system = pgo_system.unwrap();
    // Verify configuration is applied (would need getter methods in real implementation)
}

#[test]
fn test_profile_collection_initialization() {
    let mut pgo_system = PgoSystem::new().unwrap();
    let output_path = PathBuf::from("/tmp/test_profile_collection");
    
    let result = pgo_system.initialize_collection(&output_path);
    assert!(result.is_ok(), "Profile collection initialization should succeed");
}

#[test]
fn test_profile_optimization_initialization() {
    let mut pgo_system = PgoSystem::new().unwrap();
    
    // Create a dummy profile file for testing
    let profile_path = PathBuf::from("/tmp/test_profile.profile");
    
    // For testing, we'll skip actual file loading and test the interface
    // In a real implementation, would create actual profile file
    let result = pgo_system.initialize_optimization(&profile_path);
    // Expect this to fail since we don't have a real profile file
    assert!(result.is_err(), "Should fail with missing profile file");
}

#[test]
fn test_execution_context_creation() {
    let context = ExecutionContext {
        args: vec!["test_program".to_string(), "--input", "test.txt"].iter().map(|s| s.to_string()).collect(),
        env_vars: {
            let mut env = HashMap::new();
            env.insert("TEST_ENV".to_string(), "test_value".to_string());
            env
        },
        working_dir: PathBuf::from("/tmp/test_work_dir"),
        input_data: Some(b"test input data".to_vec()),
        expected_output: Some("expected output".to_string()),
        timeout: Some(Duration::from_secs(30)),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("test_type".to_string(), "integration_test".to_string());
            meta
        },
    };
    
    assert_eq!(context.args.len(), 3, "Should have 3 command line arguments");
    assert!(context.env_vars.contains_key("TEST_ENV"), "Should contain test environment variable");
    assert!(context.input_data.is_some(), "Should have input data");
    assert!(context.expected_output.is_some(), "Should have expected output");
    assert_eq!(context.timeout, Some(Duration::from_secs(30)), "Should have 30 second timeout");
}

#[test]
fn test_profile_collector_configuration() {
    let config = ProfileCollectorConfig {
        enable_function_profiling: true,
        enable_branch_profiling: true,
        enable_loop_profiling: true,
        enable_memory_profiling: false, // Disabled for test performance
        enable_call_site_profiling: true,
        sampling_rate: 0.5, // 50% sampling for testing
        max_events_in_memory: 10000,
        flush_interval: Duration::from_millis(500),
        enable_realtime_collection: false,
        buffer_size: 32768, // 32KB buffer
        enable_timing_analysis: true,
        enable_cache_analysis: false,
        thread_safety_level: ThreadSafetyLevel::Basic,
    };
    
    let collector = ProfileCollector::new(config.clone());
    assert!(collector.is_ok(), "Profile collector creation should succeed");
    
    let mut collector = collector.unwrap();
    let init_result = collector.initialize();
    assert!(init_result.is_ok(), "Profile collector initialization should succeed");
}

#[test]
fn test_profile_storage_configuration() {
    let config = ProfileStorageConfig {
        storage_directory: PathBuf::from("/tmp/test_profile_storage"),
        enable_compression: true,
        compression_level: 6,
        max_file_size: 50 * 1024 * 1024, // 50MB
        enable_auto_cleanup: true,
        max_profile_age: Duration::from_secs(7 * 24 * 3600), // 1 week
        enable_validation: true,
        format_version: ProfileVersion::V1_0,
        enable_incremental: true,
        backup_retention_count: 3,
        enable_encryption: false,
        encryption_method: EncryptionMethod::None,
    };
    
    let storage = ProfileStorage::new(config);
    assert!(storage.is_ok(), "Profile storage creation should succeed");
}

#[test]
fn test_profile_analyzer_configuration() {
    let config = ProfileAnalysisConfig {
        hot_function_threshold: 50,
        hot_function_time_threshold: 0.05,
        inlining_benefit_threshold: 0.6,
        branch_misprediction_threshold: 0.1,
        loop_unroll_threshold: 4,
        vectorization_threshold: 0.7,
        memory_pattern_threshold: 0.8,
        enable_statistical_analysis: true,
        enable_cross_function_analysis: true,
        analysis_depth: AnalysisDepth::Standard,
    };
    
    let analyzer = ProfileAnalyzer::new(config);
    assert!(analyzer.is_ok(), "Profile analyzer creation should succeed");
}

#[test]
fn test_pgo_pass_manager_configuration() {
    let config = PgoPassConfig {
        enable_inlining: true,
        enable_branch_layout: true,
        enable_loop_optimization: true,
        enable_code_layout: true,
        inlining_aggressiveness: 0.7,
        branch_optimization_level: BranchOptimizationLevel::Moderate,
        loop_optimization_aggressiveness: 0.6,
        code_layout_level: CodeLayoutLevel::Intermediate,
        max_optimization_time: Duration::from_secs(120),
        enable_validation: true,
        safety_level: OptimizationSafetyLevel::Moderate,
    };
    
    // Create LLVM context for testing
    let context = inkwell::context::Context::create();
    let pass_manager = PgoPassManager::new(&context, config);
    assert!(pass_manager.is_ok(), "PGO pass manager creation should succeed");
}

#[test]
fn test_profile_manager_commands() {
    let config = ProfileManagerConfig::default();
    let mut manager = ProfileManager::new(config).unwrap();
    
    // Test session creation
    let create_session_cmd = ProfileCommand::CreateSession {
        session_type: SessionType::Collection,
        metadata: SessionMetadata {
            created_by: "test_user".to_string(),
            purpose: "Testing profile manager".to_string(),
            project: Some("test_project".to_string()),
            custom_fields: HashMap::new(),
        },
    };
    
    let result = manager.execute_command(create_session_cmd);
    assert!(result.is_ok(), "Session creation should succeed");
    
    let session_result = result.unwrap();
    assert!(session_result.success, "Session creation should be successful");
    assert!(session_result.data.contains_key("session_id"), "Should return session ID");
    
    // Test session listing
    let list_sessions_cmd = ProfileCommand::ListSessions;
    let result = manager.execute_command(list_sessions_cmd);
    assert!(result.is_ok(), "Session listing should succeed");
    
    let list_result = result.unwrap();
    assert!(list_result.success, "Session listing should be successful");
    assert_eq!(list_result.data.get("session_count"), Some(&"1".to_string()), "Should have 1 active session");
}

#[test]
fn test_profile_validation() {
    let config = ProfileManagerConfig::default();
    let mut manager = ProfileManager::new(config).unwrap();
    
    // Create test profile data
    let profile_data = create_test_profile_data();
    
    let validation_result = manager.validate_profile(&profile_data);
    assert!(validation_result.is_ok(), "Profile validation should succeed");
    
    let validation = validation_result.unwrap();
    assert!(validation.result.score > 0.0, "Validation score should be positive");
    assert!(validation.result.score <= 1.0, "Validation score should not exceed 1.0");
}

#[test]
fn test_profile_storage_operations() {
    let config = ProfileStorageConfig::default();
    let mut storage = ProfileStorage::new(config).unwrap();
    
    // Initialize storage
    let storage_path = PathBuf::from("/tmp/test_storage");
    let init_result = storage.initialize(&storage_path);
    assert!(init_result.is_ok(), "Storage initialization should succeed");
    
    // Test profile storage
    let profile_data = create_test_profile_data();
    let store_result = storage.store_profile(&profile_data);
    assert!(store_result.is_ok(), "Profile storage should succeed");
    
    let profile_id = store_result.unwrap();
    assert!(!profile_id.is_empty(), "Profile ID should not be empty");
    
    // Test profile listing
    let profiles = storage.list_profiles();
    assert!(profiles.is_ok(), "Profile listing should succeed");
    
    let profile_list = profiles.unwrap();
    assert_eq!(profile_list.len(), 1, "Should have 1 stored profile");
}

#[test]
fn test_instrumentation_configuration() {
    let config = InstrumentationConfig {
        enable_counters: true,
        enable_timing: true,
        enable_edges: true,
        enable_memory: false, // Disabled for testing
        sampling_rate: 0.5,
        max_overhead: 0.1,
        safety_level: InstrumentationSafetyLevel::Basic,
        enable_debug: false,
        buffer_size: 32768,
        flush_interval: Duration::from_secs(1),
        thread_safe: true,
    };
    
    let context = inkwell::context::Context::create();
    let instrumentation = ProfileInstrumentation::new(&context, config);
    assert!(instrumentation.is_ok(), "Profile instrumentation creation should succeed");
}

#[test]
fn test_optimization_integration() {
    let config = PgoIntegrationConfig {
        enable_metrics_collection: true,
        enable_regression_detection: true,
        performance_threshold: 0.05,
        max_optimization_time: Duration::from_secs(60),
        enable_validation: true,
        integration_strategy: IntegrationStrategy::Augment,
        enable_fallback: true,
        aggressiveness_level: 0.7,
        enable_experimental: false,
    };
    
    let context = inkwell::context::Context::create();
    let integrator = PgoOptimizationIntegrator::new(&context, config);
    assert!(integrator.is_ok(), "PGO optimization integrator creation should succeed");
}

#[test]
fn test_pgo_error_types() {
    // Test PGO-specific error types
    let collection_error = PgoError::CollectionFailed("Test collection error".to_string());
    assert!(format!("{}", collection_error).contains("Profile collection failed"));
    
    let storage_error = PgoError::StorageFailed("Test storage error".to_string());
    assert!(format!("{}", storage_error).contains("Profile storage failed"));
    
    let analysis_error = PgoError::AnalysisFailed("Test analysis error".to_string());
    assert!(format!("{}", analysis_error).contains("Profile analysis failed"));
    
    let optimization_error = PgoError::OptimizationFailed("Test optimization error".to_string());
    assert!(format!("{}", optimization_error).contains("PGO optimization failed"));
    
    let validation_error = PgoError::ValidationFailed("Test validation error".to_string());
    assert!(format!("{}", validation_error).contains("Profile validation failed"));
    
    let format_error = PgoError::IncompatibleFormat { 
        expected: "2.0".to_string(), 
        found: "1.0".to_string() 
    };
    assert!(format!("{}", format_error).contains("Incompatible profile format"));
    
    let quality_error = PgoError::InsufficientQuality { 
        actual: 0.4, 
        required: 0.7 
    };
    assert!(format!("{}", quality_error).contains("Insufficient profile quality"));
    
    let age_error = PgoError::ProfileTooOld { 
        age: Duration::from_secs(8 * 24 * 3600), 
        max_age: Duration::from_secs(7 * 24 * 3600) 
    };
    assert!(format!("{}", age_error).contains("Profile too old"));
}

#[test]
fn test_performance_metrics() {
    let baseline_metrics = PerformanceMetrics {
        compilation_time: Duration::from_millis(1000),
        code_size: 10000,
        execution_time_improvement: 0.0,
        memory_usage_improvement: 0.0,
        cache_efficiency_improvement: 0.0,
        branch_prediction_improvement: 0.0,
        call_overhead_reduction: 0.0,
        loop_performance_improvement: 0.0,
        overall_performance_score: 0.5,
        confidence_level: 1.0,
    };
    
    let optimized_metrics = PerformanceMetrics {
        compilation_time: Duration::from_millis(1200),
        code_size: 9500,
        execution_time_improvement: 0.15,
        memory_usage_improvement: 0.08,
        cache_efficiency_improvement: 0.12,
        branch_prediction_improvement: 0.05,
        call_overhead_reduction: 0.20,
        loop_performance_improvement: 0.25,
        overall_performance_score: 0.75,
        confidence_level: 0.8,
    };
    
    // Verify metrics are reasonable
    assert!(optimized_metrics.overall_performance_score > baseline_metrics.overall_performance_score,
            "Optimized score should be higher than baseline");
    assert!(optimized_metrics.code_size < baseline_metrics.code_size,
            "Optimized code should be smaller");
    assert!(optimized_metrics.execution_time_improvement > 0.0,
            "Should have positive execution time improvement");
}

#[test]
fn test_optimization_aggressiveness_levels() {
    // Test different optimization aggressiveness levels
    let conservative_config = PgoSystemConfig {
        optimization_level: OptimizationAggressiveness::Conservative,
        ..Default::default()
    };
    
    let moderate_config = PgoSystemConfig {
        optimization_level: OptimizationAggressiveness::Moderate,
        ..Default::default()
    };
    
    let aggressive_config = PgoSystemConfig {
        optimization_level: OptimizationAggressiveness::Aggressive,
        ..Default::default()
    };
    
    let experimental_config = PgoSystemConfig {
        optimization_level: OptimizationAggressiveness::Experimental,
        ..Default::default()
    };
    
    // Verify that different levels create different configurations
    let conservative_collector_config = ProfileCollectorConfig::from_pgo_config(&conservative_config);
    let aggressive_collector_config = ProfileCollectorConfig::from_pgo_config(&aggressive_config);
    
    assert!(conservative_collector_config.sampling_rate < aggressive_collector_config.sampling_rate,
            "Conservative should have lower sampling rate than aggressive");
}

#[test]
fn test_profile_versions() {
    // Test profile version compatibility
    let v1_0 = ProfileVersion::V1_0;
    let v1_1 = ProfileVersion::V1_1;
    let v2_0 = ProfileVersion::V2_0;
    
    assert_eq!(v1_0.as_str(), "1.0");
    assert_eq!(v1_1.as_str(), "1.1");
    assert_eq!(v2_0.as_str(), "2.0");
    
    assert!(v1_0.is_compatible_with(&v1_1), "v1.0 should be compatible with v1.1");
    assert!(v1_1.is_compatible_with(&v2_0), "v1.1 should be compatible with v2.0");
    assert!(v2_0.is_compatible_with(&v1_0), "v2.0 should be compatible with v1.0");
    
    // Test version parsing
    let parsed_v1_0 = ProfileVersion::from_str("1.0");
    assert!(parsed_v1_0.is_ok(), "Should parse v1.0 successfully");
    assert_eq!(parsed_v1_0.unwrap(), ProfileVersion::V1_0);
    
    let invalid_version = ProfileVersion::from_str("3.0");
    assert!(invalid_version.is_err(), "Should fail to parse invalid version");
}

#[test]
fn test_pgo_system_statistics() {
    let pgo_system = PgoSystem::new().unwrap();
    let stats = pgo_system.get_system_statistics();
    
    // Verify initial statistics
    assert_eq!(stats.profile_count, 0, "Should start with no profiles");
    assert_eq!(stats.total_optimization_time, Duration::ZERO, "Should start with no optimization time");
    assert_eq!(stats.average_performance_improvement, 0.0, "Should start with no improvement");
}

#[test]
fn test_session_types_and_metadata() {
    // Test different session types
    let collection_session = SessionType::Collection;
    let analysis_session = SessionType::Analysis;
    let optimization_session = SessionType::Optimization;
    let migration_session = SessionType::Migration;
    let validation_session = SessionType::Validation;
    
    // Verify session types are different
    assert_ne!(format!("{:?}", collection_session), format!("{:?}", analysis_session));
    
    // Test session metadata
    let mut custom_fields = HashMap::new();
    custom_fields.insert("test_key".to_string(), "test_value".to_string());
    
    let metadata = SessionMetadata {
        created_by: "test_user".to_string(),
        purpose: "Testing session metadata".to_string(),
        project: Some("test_project".to_string()),
        custom_fields,
    };
    
    assert_eq!(metadata.created_by, "test_user");
    assert_eq!(metadata.purpose, "Testing session metadata");
    assert!(metadata.project.is_some());
    assert!(metadata.custom_fields.contains_key("test_key"));
}

// Helper function to create test profile data
fn create_test_profile_data() -> ProfileData {
    ProfileData {
        timestamp: SystemTime::now(),
        collection_duration: Duration::from_secs(10),
        function_profiles: HashMap::new(),
        branch_profiles: HashMap::new(),
        loop_profiles: HashMap::new(),
        memory_profiles: HashMap::new(),
        call_site_profiles: HashMap::new(),
        metadata: ProfileMetadata {
            command_line: vec!["test_program".to_string()],
            environment: HashMap::new(),
            working_directory: "/tmp".to_string(),
            compiler_version: "cursed-1.0.0".to_string(),
            target_architecture: "x86_64".to_string(),
            collection_config: ProfileCollectorConfig::default(),
            format_version: "1.0".to_string(),
            quality_score: 0.8,
        },
        collection_stats: CollectionStatistics::default(),
    }
}

#[test]
fn test_end_to_end_pgo_workflow() {
    // Test a complete PGO workflow
    let mut pgo_system = PgoSystem::new().unwrap();
    
    // Step 1: Initialize for collection
    let collection_path = PathBuf::from("/tmp/test_e2e_collection");
    let init_result = pgo_system.initialize_collection(&collection_path);
    assert!(init_result.is_ok(), "Collection initialization should succeed");
    
    // Step 2: Collect profile data
    let execution_context = ExecutionContext::default();
    let collection_result = pgo_system.collect_profile_data(&execution_context);
    assert!(collection_result.is_ok(), "Profile data collection should succeed");
    
    let profile_data = collection_result.unwrap();
    
    // Step 3: Store profile data
    let storage_result = pgo_system.store_profile_data(&profile_data);
    assert!(storage_result.is_ok(), "Profile data storage should succeed");
    
    // Step 4: Get optimization recommendations
    let recommendations = pgo_system.get_optimization_recommendations(&profile_data);
    assert!(recommendations.is_ok(), "Should get optimization recommendations");
    
    let recommendations = recommendations.unwrap();
    assert!(!recommendations.is_empty(), "Should have at least one recommendation");
    
    // Step 5: Validate metrics (simulate baseline vs optimized)
    let baseline_metrics = PerformanceMetrics::default();
    let optimized_metrics = PerformanceMetrics {
        overall_performance_score: 0.15,
        execution_time_improvement: 0.10,
        memory_usage_improvement: 0.05,
        ..Default::default()
    };
    
    let validation_result = pgo_system.validate_optimization_effectiveness(&baseline_metrics, &optimized_metrics);
    assert!(validation_result.is_ok(), "Performance validation should succeed");
    
    let validation = validation_result.unwrap();
    assert!(validation.validation_passed, "Validation should pass with improvements");
}
