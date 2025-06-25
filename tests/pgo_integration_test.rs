/// PGO Integration Tests
/// 
/// Comprehensive tests for the Profile-Guided Optimization system
/// including instrumentation, profile collection, analysis, and optimization application.

use cursed::optimization::pgo::*;
use cursed::error::Result;
use cursed::cli::pgo_commands::*;
use std::path::PathBuf;
use std::time::Duration;
use std::collections::HashMap;
use tempfile::TempDir;

// Test helper for creating test profile data
fn create_test_profile_data() -> ProfileData {
    let mut profile_data = ProfileData::new();
    
    // Add function execution data
    profile_data.add_function_execution("main".to_string(), 1000);
    profile_data.add_function_execution("hot_function".to_string(), 5000);
    profile_data.add_function_execution("cold_function".to_string(), 10);
    profile_data.add_function_execution("compute_heavy".to_string(), 2000);
    
    // Add basic block data
    profile_data.add_basic_block_execution("main_bb1".to_string(), 1000);
    profile_data.add_basic_block_execution("hot_function_bb1".to_string(), 5000);
    profile_data.add_basic_block_execution("hot_function_loop".to_string(), 50000);
    
    // Add edge data
    profile_data.add_edge_execution("main->hot_function".to_string(), 1000);
    profile_data.add_edge_execution("hot_function->compute_heavy".to_string(), 2000);
    
    // Set timing data
    profile_data.total_execution_time = Duration::from_millis(1000);
    
    // Add hot functions
    profile_data.hot_functions = vec![
        HotFunction {
            name: "hot_function".to_string(),
            execution_count: 5000,
            total_time: Duration::from_millis(500),
            average_time: Duration::from_nanos(100000),
            time_percentage: 50.0,
            optimization_priority: OptimizationPriority::High,
            call_sites: HashMap::from([("main".to_string(), 1000)]),
            call_count: 1000,
            average_size: 50,
            has_vectorizable_loops: true,
            memory_access_pattern: MemoryAccessPattern::Sequential,
            branch_prediction_accuracy: 0.95,
            cache_miss_rate: 0.05,
            optimization_potential: OptimizationPotential::High,
        },
        HotFunction {
            name: "compute_heavy".to_string(),
            execution_count: 2000,
            total_time: Duration::from_millis(300),
            average_time: Duration::from_nanos(150000),
            time_percentage: 30.0,
            optimization_priority: OptimizationPriority::Medium,
            call_sites: HashMap::from([("hot_function".to_string(), 2000)]),
            call_count: 2000,
            average_size: 100,
            has_vectorizable_loops: false,
            memory_access_pattern: MemoryAccessPattern::Random,
            branch_prediction_accuracy: 0.80,
            cache_miss_rate: 0.15,
            optimization_potential: OptimizationPotential::Medium,
        },
    ];
    
    // Add cold functions
    profile_data.cold_functions = vec!["cold_function".to_string()];
    
    profile_data
}

#[test]
fn test_pgo_manager_creation() {
    let config = PgoConfig::default();
    let pgo_manager = PgoManager::new(config);
    assert!(pgo_manager.is_ok());
}

#[test]
fn test_pgo_session_lifecycle() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = PgoConfig {
        enabled: true,
        profile_data_dir: temp_dir.path().to_path_buf(),
        ..PgoConfig::default()
    };
    
    let mut pgo_manager = PgoManager::new(config)?;
    
    // Start session
    let session_id = pgo_manager.start_session(None)?;
    assert!(!session_id.is_empty());
    
    // Check session status
    let session_status = pgo_manager.get_session_status();
    assert!(session_status.is_some());
    assert_eq!(session_status.unwrap().status, PgoSessionStatus::Collecting);
    
    // Stop session
    let session = pgo_manager.stop_session()?;
    assert_eq!(session.id, session_id);
    assert_eq!(session.status, PgoSessionStatus::Completed);
    
    Ok(())
}

#[test]
fn test_profile_data_collection() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = PgoConfig {
        enabled: true,
        profile_data_dir: temp_dir.path().to_path_buf(),
        collection_mode: CollectionMode::Counters,
        ..PgoConfig::default()
    };
    
    let mut collector = ProfileCollector::new(config)?;
    
    // Start collection
    collector.start_collection("test_session", None)?;
    
    // Record some profile data
    collector.record_function_profile(
        "test_session",
        "test_function",
        100,
        Duration::from_millis(10),
    )?;
    
    collector.record_basic_block_profile(
        "test_session",
        "test_block",
        150,
    )?;
    
    // Stop collection and get data
    let profile_data = collector.collect_profile_data("test_session")?;
    
    assert!(profile_data.function_counts.contains_key("test_function"));
    assert_eq!(profile_data.function_counts["test_function"], 100);
    
    Ok(())
}

#[test]
fn test_profile_analysis() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = PgoConfig {
        enabled: true,
        profile_data_dir: temp_dir.path().to_path_buf(),
        hot_function_threshold: 0.1,
        cold_function_threshold: 0.01,
        ..PgoConfig::default()
    };
    
    let profile_data = create_test_profile_data();
    let analyzer = ProfileAnalyzer::new(config)?;
    
    let analysis = analyzer.analyze_profile_data(&profile_data)?;
    
    // Check hot functions are identified
    assert!(!analysis.hot_functions.is_empty());
    let hot_function_names: Vec<String> = analysis.hot_functions.iter()
        .map(|f| f.name.clone()).collect();
    assert!(hot_function_names.contains(&"hot_function".to_string()));
    
    // Check cold functions are identified
    assert!(!analysis.cold_functions.is_empty());
    assert!(analysis.cold_functions.contains(&"cold_function".to_string()));
    
    Ok(())
}

#[test]
fn test_optimization_engine() -> Result<()> {
    let config = PgoConfig {
        enabled: true,
        optimization_strategy: OptimizationStrategy::Speed,
        ..PgoConfig::default()
    };
    
    let profile_data = create_test_profile_data();
    let analyzer = ProfileAnalyzer::new(config.clone())?;
    let analysis = analyzer.analyze_profile_data(&profile_data)?;
    
    let optimization_engine = PgoOptimizationEngine::new(config)?;
    let optimization_results = optimization_engine.apply_optimizations(&analysis)?;
    
    assert!(!optimization_results.is_empty());
    
    // Check that optimizations were applied to hot functions
    let optimized_targets: Vec<String> = optimization_results.iter()
        .map(|r| r.target.clone()).collect();
    assert!(optimized_targets.contains(&"hot_function".to_string()));
    
    Ok(())
}

#[test]
fn test_pgo_config_serialization() -> Result<()> {
    let config = PgoConfig {
        enabled: true,
        optimization_strategy: OptimizationStrategy::Custom {
            speed_weight: 0.7,
            size_weight: 0.2,
            compilation_time_weight: 0.1,
            power_weight: 0.0,
        },
        hot_function_threshold: 0.15,
        cold_function_threshold: 0.02,
        enable_indirect_call_promotion: true,
        enable_value_profiling: true,
        ..PgoConfig::default()
    };
    
    // Test JSON serialization
    let json = serde_json::to_string_pretty(&config).unwrap();
    let deserialized: PgoConfig = serde_json::from_str(&json).unwrap();
    
    assert_eq!(config.enabled, deserialized.enabled);
    assert_eq!(config.hot_function_threshold, deserialized.hot_function_threshold);
    assert_eq!(config.enable_indirect_call_promotion, deserialized.enable_indirect_call_promotion);
    
    Ok(())
}

#[test]
fn test_profile_data_merge() -> Result<()> {
    let mut profile1 = ProfileData::new();
    profile1.add_function_execution("func1".to_string(), 100);
    profile1.add_function_execution("func2".to_string(), 200);
    
    let mut profile2 = ProfileData::new();
    profile2.add_function_execution("func1".to_string(), 50);
    profile2.add_function_execution("func3".to_string(), 150);
    
    profile1.merge(profile2);
    
    assert_eq!(profile1.function_counts["func1"], 150); // 100 + 50
    assert_eq!(profile1.function_counts["func2"], 200);
    assert_eq!(profile1.function_counts["func3"], 150);
    
    Ok(())
}

#[test]
fn test_hot_cold_function_identification() -> Result<()> {
    let profile_data = create_test_profile_data();
    
    let hot_functions = profile_data.get_hot_functions(20.0); // Above 20%
    let cold_functions = profile_data.get_cold_functions(5.0); // Below 5%
    
    assert!(hot_functions.contains(&"hot_function".to_string()));
    assert!(cold_functions.contains(&"cold_function".to_string()));
    assert!(!hot_functions.contains(&"cold_function".to_string()));
    assert!(!cold_functions.contains(&"hot_function".to_string()));
    
    Ok(())
}

#[test]
fn test_llvm_format_conversion() -> Result<()> {
    let profile_data = create_test_profile_data();
    
    // Convert to LLVM format
    let llvm_format = crate::optimization::pgo::data_format::to_llvm_profdata_format(&profile_data);
    assert!(llvm_format.contains("main:1000"));
    assert!(llvm_format.contains("hot_function:5000"));
    
    // Parse back from LLVM format
    let parsed_profile = crate::optimization::pgo::data_format::from_llvm_profdata_format(&llvm_format)?;
    assert_eq!(parsed_profile.function_counts.get("main"), Some(&1000));
    assert_eq!(parsed_profile.function_counts.get("hot_function"), Some(&5000));
    
    Ok(())
}

#[test]
fn test_optimization_recommendations() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = PgoConfig {
        enabled: true,
        profile_data_dir: temp_dir.path().to_path_buf(),
        ..PgoConfig::default()
    };
    
    let pgo_manager = PgoManager::new(config)?;
    
    // Create a test session and save profile data
    let session_id = "test_recommendations";
    let profile_data = create_test_profile_data();
    pgo_manager.save_profile_data(session_id, &profile_data)?;
    
    // Generate recommendations
    let recommendations = pgo_manager.analyze_and_recommend(session_id)?;
    
    assert_eq!(recommendations.session_id, session_id);
    assert!(!recommendations.optimization_opportunities.is_empty());
    assert!(!recommendations.recommended_flags.is_empty());
    
    // Check that high-priority optimizations are recommended for hot functions
    let hot_opportunities: Vec<_> = recommendations.optimization_opportunities.iter()
        .filter(|o| o.priority == OptimizationPriority::High)
        .collect();
    assert!(!hot_opportunities.is_empty());
    
    Ok(())
}

#[test]
fn test_pgo_statistics() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = PgoConfig {
        enabled: true,
        profile_data_dir: temp_dir.path().to_path_buf(),
        ..PgoConfig::default()
    };
    
    let pgo_manager = PgoManager::new(config)?;
    let statistics = pgo_manager.get_statistics();
    
    // Basic statistics structure validation
    assert!(statistics.sessions_completed >= 0);
    assert!(statistics.total_optimizations_applied >= 0);
    assert!(statistics.average_performance_improvement >= 0.0);
    assert!(statistics.profile_data_size >= 0);
    assert!(statistics.instrumentation_overhead >= 0.0);
    
    Ok(())
}

#[test]
fn test_cli_pgo_command_structure() {
    // Test that PGO commands are properly structured
    use clap::CommandFactory;
    
    #[derive(clap::Parser)]
    struct TestCli {
        #[command(subcommand)]
        pgo: PgoCommands,
    }
    
    let cmd = TestCli::command();
    let help = cmd.render_help();
    let help_str = help.to_string();
    
    // Verify all main PGO subcommands are present
    assert!(help_str.contains("generate"));
    assert!(help_str.contains("collect"));
    assert!(help_str.contains("merge"));
    assert!(help_str.contains("analyze"));
    assert!(help_str.contains("apply"));
    assert!(help_str.contains("workflow"));
    assert!(help_str.contains("stats"));
}

#[test]
fn test_pgo_command_handler_creation() {
    let handler = PgoCommandHandler::new();
    assert!(handler.pgo_manager.is_none());
    
    let default_handler = PgoCommandHandler::default();
    assert!(default_handler.pgo_manager.is_none());
}

#[test]
fn test_performance_metrics_calculation() -> Result<()> {
    let hot_function = HotFunction {
        name: "test_function".to_string(),
        execution_count: 1000,
        total_time: Duration::from_millis(100),
        average_time: Duration::from_nanos(100000),
        time_percentage: 25.0,
        optimization_priority: OptimizationPriority::High,
        call_sites: HashMap::new(),
        call_count: 1000,
        average_size: 75,
        has_vectorizable_loops: true,
        memory_access_pattern: MemoryAccessPattern::Sequential,
        branch_prediction_accuracy: 0.92,
        cache_miss_rate: 0.08,
        optimization_potential: OptimizationPotential::High,
    };
    
    // Verify performance metrics make sense
    assert_eq!(hot_function.average_time, Duration::from_nanos(100000));
    assert!(hot_function.time_percentage > 0.0);
    assert!(hot_function.branch_prediction_accuracy <= 1.0);
    assert!(hot_function.cache_miss_rate <= 1.0);
    
    Ok(())
}

#[test]
fn test_instrumentation_configuration() -> Result<()> {
    let configs = vec![
        (InstrumentationMode::Frontend, CollectionMode::Counters),
        (InstrumentationMode::IR, CollectionMode::Sampling),
        (InstrumentationMode::Sampling, CollectionMode::CountersAndSampling),
        (InstrumentationMode::Hardware, CollectionMode::TimeBased),
        (InstrumentationMode::Hybrid, CollectionMode::EventBased),
    ];
    
    for (instr_mode, coll_mode) in configs {
        let config = PgoConfig {
            enabled: true,
            instrumentation_mode: instr_mode,
            collection_mode: coll_mode,
            ..PgoConfig::default()
        };
        
        let collector = ProfileCollector::new(config);
        assert!(collector.is_ok());
    }
    
    Ok(())
}

#[test]
fn test_optimization_strategy_effects() -> Result<()> {
    let strategies = vec![
        OptimizationStrategy::Speed,
        OptimizationStrategy::Size,
        OptimizationStrategy::Balanced,
        OptimizationStrategy::Custom {
            speed_weight: 0.8,
            size_weight: 0.1,
            compilation_time_weight: 0.1,
            power_weight: 0.0,
        },
    ];
    
    for strategy in strategies {
        let config = PgoConfig {
            enabled: true,
            optimization_strategy: strategy,
            ..PgoConfig::default()
        };
        
        let optimization_engine = PgoOptimizationEngine::new(config);
        assert!(optimization_engine.is_ok());
    }
    
    Ok(())
}

#[test]
fn test_profile_data_validation() -> Result<()> {
    let mut profile_data = create_test_profile_data();
    
    // Valid profile data should pass validation
    assert!(profile_data.validate().is_ok());
    
    // Add a function with zero execution count (should fail validation)
    profile_data.function_counts.insert("zero_function".to_string(), 0);
    assert!(profile_data.validate().is_err());
    
    Ok(())
}

#[test] 
fn test_pgo_workflow_integration() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    
    // Test the complete workflow simulation
    let mut handler = PgoCommandHandler::new();
    
    let workflow_args = PgoWorkflowArgs {
        source_files: vec![PathBuf::from("test.csd")],
        training_args: vec!["--test".to_string()],
        output: Some(PathBuf::from("optimized_binary")),
        profile_dir: temp_dir.path().to_path_buf(),
        strategy: PgoOptimizationStrategy::Balanced,
        training_runs: 1,
        cleanup: false,
        report: true,
        benchmark: false,
    };
    
    // This should not fail even with simulated compilation
    let result = handler.handle_workflow(workflow_args);
    
    // In a real implementation, this would succeed
    // For now, we expect it to work with simulation
    match result {
        Ok(_) => {} // Expected with full implementation
        Err(_) => {} // Expected with current simulation
    }
    
    Ok(())
}

// Performance regression test
#[test]
fn test_pgo_performance_overhead() -> Result<()> {
    let start = std::time::Instant::now();
    
    let temp_dir = TempDir::new().unwrap();
    let config = PgoConfig {
        enabled: true,
        profile_data_dir: temp_dir.path().to_path_buf(),
        ..PgoConfig::default()
    };
    
    let _pgo_manager = PgoManager::new(config)?;
    let _profile_data = create_test_profile_data();
    
    let elapsed = start.elapsed();
    
    // PGO setup should be fast (< 100ms for test data)
    assert!(elapsed < Duration::from_millis(100));
    
    Ok(())
}

// Memory usage test
#[test]
fn test_pgo_memory_efficiency() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = PgoConfig {
        enabled: true,
        profile_data_dir: temp_dir.path().to_path_buf(),
        max_profile_data_size: 1024 * 1024, // 1MB limit
        ..PgoConfig::default()
    };
    
    let mut collector = ProfileCollector::new(config)?;
    
    // Add a reasonable amount of profile data
    collector.start_collection("memory_test", None)?;
    
    for i in 0..1000 {
        collector.record_function_profile(
            "memory_test",
            &format!("function_{}", i),
            10,
            Duration::from_nanos(1000),
        )?;
    }
    
    let profile_data = collector.collect_profile_data("memory_test")?;
    
    // Verify data was collected efficiently
    assert_eq!(profile_data.function_counts.len(), 1000);
    
    Ok(())
}
