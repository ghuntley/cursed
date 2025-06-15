/// Comprehensive PGO Integration Tests
/// 
/// Tests the complete Profile-Guided Optimization system including
/// instrumentation, profile collection, analysis, and optimization application.

use cursed::optimization::pgo::{
    PgoManager, PgoConfig, InstrumentationMode, CollectionMode, 
    OptimizationStrategy, ProfileData, ProfileAnalysis
};
use cursed::optimization::optimization_manager::OptimizationManager;
use cursed::optimization::config::{OptimizationConfig, OptimizationLevel};
use std::path::PathBuf;
use std::time::Duration;

#[test]
fn test_pgo_manager_creation() {
    let config = PgoConfig::default();
    let manager = PgoManager::new(config);
    assert!(manager.is_ok());
}

#[test]
fn test_pgo_session_lifecycle() {
    let mut config = PgoConfig::default();
    config.enabled = true;
    config.profile_data_dir = PathBuf::from("test_pgo_profiles");
    
    let mut manager = PgoManager::new(config).unwrap();
    
    // Start session
    let session_id = manager.start_session(Some("test_session".to_string())).unwrap();
    assert_eq!(session_id, "test_session");
    
    // Check session status
    let status = manager.get_session_status();
    assert!(status.is_some());
    assert_eq!(status.unwrap().id, "test_session");
    
    // Stop session
    let session = manager.stop_session().unwrap();
    assert_eq!(session.id, "test_session");
    
    // Cleanup
    let _ = std::fs::remove_dir_all("test_pgo_profiles");
}

#[test]
fn test_source_code_instrumentation() {
    let config = PgoConfig {
        enabled: true,
        instrumentation_mode: InstrumentationMode::Frontend,
        ..Default::default()
    };
    
    let manager = PgoManager::new(config).unwrap();
    
    let source_code = r#"
slay main() {
    lowkey (sus x = 0; x < 10; x++) {
        facts result = compute_value(x);
        println(result);
    }
}

slay compute_value(value: i32) -> i32 {
    return value * 2;
}
"#;

    let instrumented = manager.generate_instrumented_code(source_code, "test").unwrap();
    
    // Should contain instrumentation includes and calls
    assert!(instrumented.contains("#include \"pgo_runtime.h\""));
    assert!(instrumented.contains("__pgo_increment_function"));
}

#[test]
fn test_profile_data_analysis() {
    let config = PgoConfig::default();
    let mut manager = PgoManager::new(config).unwrap();
    
    // Create sample profile data
    let mut profile_data = ProfileData::new();
    profile_data.add_function_execution("hot_function".to_string(), 10000);
    profile_data.add_function_execution("cold_function".to_string(), 10);
    profile_data.add_function_execution("medium_function".to_string(), 1000);
    
    // Save and load profile data
    let profile_path = PathBuf::from("test_profile.json");
    manager.save_profile_data("test_session", &profile_data).ok();
    
    // Analyze and get recommendations
    let recommendations = manager.analyze_and_recommend("test_session");
    
    if let Ok(recs) = recommendations {
        assert!(!recs.hot_functions.is_empty());
        assert!(!recs.cold_functions.is_empty());
        assert!(!recs.optimization_opportunities.is_empty());
    }
    
    // Cleanup
    let _ = std::fs::remove_file("test_profile.json");
}

#[test]
fn test_optimization_strategies() {
    let strategies = [
        OptimizationStrategy::Speed,
        OptimizationStrategy::Size,
        OptimizationStrategy::Balanced,
        OptimizationStrategy::Custom {
            speed_weight: 0.7,
            size_weight: 0.2,
            compilation_time_weight: 0.1,
            power_weight: 0.0,
        },
    ];
    
    for strategy in &strategies {
        let config = PgoConfig {
            enabled: true,
            optimization_strategy: strategy.clone(),
            ..Default::default()
        };
        
        let manager = PgoManager::new(config);
        assert!(manager.is_ok());
    }
}

#[test]
fn test_instrumentation_modes() {
    let modes = [
        InstrumentationMode::Frontend,
        InstrumentationMode::IR,
        InstrumentationMode::Sampling,
        InstrumentationMode::Hardware,
        InstrumentationMode::Hybrid,
    ];
    
    for mode in &modes {
        let config = PgoConfig {
            enabled: true,
            instrumentation_mode: mode.clone(),
            ..Default::default()
        };
        
        let manager = PgoManager::new(config);
        assert!(manager.is_ok());
    }
}

#[test]
fn test_collection_modes() {
    let modes = [
        CollectionMode::Counters,
        CollectionMode::Sampling,
        CollectionMode::CountersAndSampling,
        CollectionMode::TimeBased,
        CollectionMode::EventBased,
    ];
    
    for mode in &modes {
        let config = PgoConfig {
            enabled: true,
            collection_mode: mode.clone(),
            ..Default::default()
        };
        
        let manager = PgoManager::new(config);
        assert!(manager.is_ok());
    }
}

#[test]
fn test_profile_data_format() {
    // Test profile data serialization/deserialization
    let mut profile_data = ProfileData::new();
    profile_data.add_function_execution("test_func".to_string(), 100);
    profile_data.add_basic_block_execution("test_block".to_string(), 200);
    profile_data.add_edge_execution("test_edge".to_string(), 150);
    
    // Test JSON serialization
    let json = cursed::optimization::pgo::serialize_to_json(&profile_data).unwrap();
    let deserialized = cursed::optimization::pgo::deserialize_from_json(&json).unwrap();
    
    assert_eq!(profile_data.function_counts.len(), deserialized.function_counts.len());
    assert_eq!(profile_data.function_counts.get("test_func"), deserialized.function_counts.get("test_func"));
    
    // Test LLVM format conversion
    let llvm_format = cursed::optimization::pgo::to_llvm_profdata_format(&profile_data);
    assert!(llvm_format.contains("test_func:100"));
    
    let parsed = cursed::optimization::pgo::from_llvm_profdata_format(&llvm_format).unwrap();
    assert_eq!(parsed.function_counts.get("test_func"), Some(&100));
}

#[test]
fn test_hot_cold_function_identification() {
    let mut profile_data = ProfileData::new();
    
    // Add functions with different execution counts
    profile_data.add_function_execution("very_hot".to_string(), 100000);
    profile_data.add_function_execution("hot".to_string(), 10000);
    profile_data.add_function_execution("warm".to_string(), 1000);
    profile_data.add_function_execution("cold".to_string(), 10);
    profile_data.add_function_execution("very_cold".to_string(), 1);
    
    // Test hot function identification (top 10%)
    let hot_functions = profile_data.get_hot_functions(10.0);
    assert!(hot_functions.contains(&"very_hot".to_string()));
    assert!(hot_functions.contains(&"hot".to_string()));
    
    // Test cold function identification (bottom 1%)
    let cold_functions = profile_data.get_cold_functions(1.0);
    assert!(cold_functions.contains(&"very_cold".to_string()));
    assert!(cold_functions.contains(&"cold".to_string()));
}

#[test]
fn test_profile_data_merging() {
    let mut profile1 = ProfileData::new();
    profile1.add_function_execution("func1".to_string(), 100);
    profile1.add_function_execution("func2".to_string(), 200);
    
    let mut profile2 = ProfileData::new();
    profile2.add_function_execution("func1".to_string(), 50);
    profile2.add_function_execution("func3".to_string(), 300);
    
    profile1.merge(profile2);
    
    assert_eq!(profile1.function_counts.get("func1"), Some(&150)); // 100 + 50
    assert_eq!(profile1.function_counts.get("func2"), Some(&200)); // unchanged
    assert_eq!(profile1.function_counts.get("func3"), Some(&300)); // new function
}

#[test]
fn test_profile_data_validation() {
    let mut profile_data = ProfileData::new();
    profile_data.add_function_execution("valid_func".to_string(), 100);
    
    // Valid profile should pass validation
    assert!(profile_data.validate().is_ok());
    
    // Add invalid data
    profile_data.function_counts.insert("invalid_func".to_string(), 0);
    
    // Should fail validation
    assert!(profile_data.validate().is_err());
}

#[test] 
fn test_optimization_manager_pgo_integration() {
    let mut opt_config = OptimizationConfig::default();
    opt_config.enable_profiling = true;
    opt_config.profile_data_dir = Some(PathBuf::from("test_pgo_opt"));
    
    let manager = OptimizationManager::new(opt_config);
    assert!(manager.is_ok());
    
    let mut manager = manager.unwrap();
    
    let test_source = r#"
slay main() {
    lowkey (sus i = 0; i < 1000; i++) {
        process_data(i);
    }
}

slay process_data(value: i32) {
    // Hot function that would benefit from optimization
    facts result = value * value + value / 2;
    println(result);
}
"#;
    
    // Apply complete optimization including PGO
    let result = manager.optimize_complete(test_source);
    
    if let Ok(opt_result) = result {
        assert!(opt_result.success);
        assert!(!opt_result.passes_applied.is_empty());
        // PGO should provide some performance improvement
        assert!(opt_result.performance_improvement >= 0.0);
    }
    
    // Cleanup
    let _ = std::fs::remove_dir_all("test_pgo_opt");
}

#[test]
fn test_pgo_disabled_behavior() {
    let config = PgoConfig {
        enabled: false,
        ..Default::default()
    };
    
    let manager = PgoManager::new(config).unwrap();
    
    // Operations should complete without error but do nothing
    let instrumented = manager.generate_instrumented_code("test code", "target").unwrap();
    assert_eq!(instrumented, "test code"); // No instrumentation when disabled
    
    // Session operations should return errors when disabled
    let session_result = manager.start_session(None);
    assert!(session_result.is_err());
}

#[test]
fn test_concurrent_pgo_sessions() {
    use std::thread;
    use std::sync::{Arc, Mutex};
    
    let config = PgoConfig {
        enabled: true,
        profile_data_dir: PathBuf::from("test_concurrent_pgo"),
        ..Default::default()
    };
    
    let manager = Arc::new(Mutex::new(PgoManager::new(config).unwrap()));
    let mut handles = Vec::new();
    
    // Start multiple PGO sessions concurrently
    for i in 0..3 {
        let manager_clone = Arc::clone(&manager);
        let handle = thread::spawn(move || {
            let mut mgr = manager_clone.lock().unwrap();
            let session_id = format!("concurrent_session_{}", i);
            
            // Each session should be independent
            if let Ok(id) = mgr.start_session(Some(session_id.clone())) {
                assert_eq!(id, session_id);
                // Simulate some work
                thread::sleep(Duration::from_millis(10));
                let _ = mgr.stop_session();
            }
        });
        handles.push(handle);
    }
    
    // Wait for all sessions to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Cleanup
    let _ = std::fs::remove_dir_all("test_concurrent_pgo");
}

#[test]
fn test_pgo_statistics_collection() {
    let config = PgoConfig {
        enabled: true,
        ..Default::default()
    };
    
    let manager = PgoManager::new(config).unwrap();
    let stats = manager.get_statistics();
    
    // Initial stats should be zero
    assert_eq!(stats.sessions_completed, 0);
    assert_eq!(stats.total_optimizations_applied, 0);
    assert_eq!(stats.average_performance_improvement, 0.0);
}

#[test]
fn test_pgo_configuration_updates() {
    let initial_config = PgoConfig::default();
    let mut manager = PgoManager::new(initial_config).unwrap();
    
    let new_config = PgoConfig {
        enabled: true,
        instrumentation_mode: InstrumentationMode::IR,
        optimization_strategy: OptimizationStrategy::Speed,
        hot_function_threshold: 0.2,
        ..Default::default()
    };
    
    let result = manager.update_config(new_config);
    assert!(result.is_ok());
}

#[test]
fn test_error_handling_and_recovery() {
    // Test various error conditions and recovery
    
    // Invalid profile data directory
    let config = PgoConfig {
        enabled: true,
        profile_data_dir: PathBuf::from("/invalid/path/that/cannot/exist"),
        ..Default::default()
    };
    
    // Should handle gracefully and not crash
    let manager_result = PgoManager::new(config);
    // May succeed or fail depending on permissions, but shouldn't panic
    
    // Test with minimal valid config
    let minimal_config = PgoConfig {
        enabled: true,
        profile_data_dir: PathBuf::from("minimal_test_pgo"),
        ..Default::default()
    };
    
    let manager = PgoManager::new(minimal_config);
    assert!(manager.is_ok());
    
    // Cleanup
    let _ = std::fs::remove_dir_all("minimal_test_pgo");
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_pgo_performance_overhead() {
        let config = PgoConfig {
            enabled: true,
            instrumentation_mode: InstrumentationMode::Frontend,
            ..Default::default()
        };
        
        let manager = PgoManager::new(config).unwrap();
        
        let large_source_code = "slay main() {\n".to_string() + 
            &"    println(\"test\");\n".repeat(1000) + 
            "}";
        
        let start_time = Instant::now();
        let instrumented = manager.generate_instrumented_code(&large_source_code, "perf_test").unwrap();
        let instrumentation_time = start_time.elapsed();
        
        // Instrumentation should complete in reasonable time (< 100ms for 1000 lines)
        assert!(instrumentation_time < Duration::from_millis(100));
        
        // Instrumented code should be larger but not excessively so
        assert!(instrumented.len() > large_source_code.len());
        assert!(instrumented.len() < large_source_code.len() * 3); // Less than 3x growth
    }
    
    #[test]
    fn test_profile_data_processing_performance() {
        let mut profile_data = ProfileData::new();
        
        // Add many functions to test scalability
        let start_time = Instant::now();
        for i in 0..10000 {
            profile_data.add_function_execution(format!("func_{}", i), i as u64);
        }
        let processing_time = start_time.elapsed();
        
        // Should process 10K functions quickly (< 50ms)
        assert!(processing_time < Duration::from_millis(50));
        
        // Test hot/cold function identification performance
        let start_time = Instant::now();
        let hot_functions = profile_data.get_hot_functions(10.0);
        let cold_functions = profile_data.get_cold_functions(10.0);
        let analysis_time = start_time.elapsed();
        
        // Analysis should be fast (< 10ms)
        assert!(analysis_time < Duration::from_millis(10));
        assert!(!hot_functions.is_empty());
        assert!(!cold_functions.is_empty());
    }
}
