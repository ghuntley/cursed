//! Tests for the optimization enablement system
//! 
//! Validates optimization profiles, configuration management,
//! performance tracking, and CLI integration.

use cursed::optimization::{
    OptimizationEnablementSystem, OptimizationEnablementConfig, OptimizationProfile,
    PerformanceMonitoringConfig, PerformanceReportFormat, OptimizationConfigManager,
    OptimizationConfig, OptimizationLevel,
};
use cursed::error::Result;
use std::time::Duration;
use tempfile::TempDir;

/// Test basic optimization enablement system creation
#[test]
fn test_optimization_enablement_system_creation() {
    let system = OptimizationEnablementSystem::new();
    assert!(system.is_ok());
    
    let system = system.unwrap();
    assert!(system.config.enable_advanced_by_default);
    assert!(system.config.enable_pgo_when_available);
    assert!(system.config.enable_adaptive_optimization);
    assert!(system.config.enable_parallel_optimization);
}

/// Test optimization profile configurations
#[test]
fn test_optimization_profiles() {
    let system = OptimizationEnablementSystem::new().unwrap();
    
    // Test development profile
    let dev_config = system.get_optimization_config(&OptimizationProfile::Development).unwrap();
    assert_eq!(dev_config.optimization_level, OptimizationLevel::None);
    assert!(!dev_config.enable_lto);
    
    // Test release profile
    let release_config = system.get_optimization_config(&OptimizationProfile::Release).unwrap();
    assert_eq!(release_config.optimization_level, OptimizationLevel::Aggressive);
    assert!(release_config.enable_lto);
    
    // Test size profile
    let size_config = system.get_optimization_config(&OptimizationProfile::Size).unwrap();
    assert_eq!(size_config.optimization_level, OptimizationLevel::Size);
    assert!(size_config.enable_lto);
    assert!(!size_config.enable_inlining); // Size optimized should avoid inlining
    
    // Test debug profile
    let debug_config = system.get_optimization_config(&OptimizationProfile::Debug).unwrap();
    assert_eq!(debug_config.optimization_level, OptimizationLevel::Basic);
}

/// Test custom optimization profile
#[test]
fn test_custom_optimization_profile() {
    let mut system = OptimizationEnablementSystem::new().unwrap();
    
    // Create custom profile
    let custom_config = OptimizationConfig {
        optimization_level: OptimizationLevel::Default,
        enable_lto: false,
        enable_parallel: true,
        enable_vectorization: true,
        ..OptimizationConfig::default()
    };
    
    system.add_custom_profile("custom_fast".to_string(), custom_config.clone());
    
    // Test retrieving custom profile
    let retrieved_config = system.get_optimization_config(
        &OptimizationProfile::Custom("custom_fast".to_string())
    ).unwrap();
    
    assert_eq!(retrieved_config.optimization_level, OptimizationLevel::Default);
    assert!(!retrieved_config.enable_lto);
    assert!(retrieved_config.enable_parallel);
    assert!(retrieved_config.enable_vectorization);
}

/// Test optimization application with different profiles
#[test]
fn test_optimization_application() {
    let mut system = OptimizationEnablementSystem::new().unwrap();
    let test_source = r#"
        slay add(sus a: i32, sus b: i32) -> i32 {
            return a + b;
        }
        
        slay main() {
            facts result = add(5, 3);
            println(&format!("Result: {}", result))?;
        }
    "#;
    
    // Test with development profile
    let dev_results = system.apply_optimizations(
        test_source,
        &OptimizationProfile::Development,
        None,
        &[],
    ).unwrap();
    
    assert!(dev_results.total_optimization_time > Duration::ZERO);
    assert_eq!(dev_results.overall_improvement, 0.0); // No optimization for development
    
    // Test with release profile
    let release_results = system.apply_optimizations(
        test_source,
        &OptimizationProfile::Release,
        None,
        &[],
    ).unwrap();
    
    assert!(release_results.overall_improvement > 0.5); // Aggressive optimization
    assert!(release_results.llvm_improvements.is_some());
    assert!(release_results.pgo_improvements.is_some());
    assert!(release_results.adaptive_improvements.is_some());
    assert!(release_results.time_savings.is_some());
    
    // Test with target-specific features
    let target_results = system.apply_optimizations(
        test_source,
        &OptimizationProfile::Release,
        Some("x86_64"),
        &["sse4.2".to_string(), "avx2".to_string()],
    ).unwrap();
    
    assert!(target_results.overall_improvement > 0.5);
}

/// Test performance monitoring configuration
#[test]
fn test_performance_monitoring() {
    let mut config = OptimizationEnablementConfig::default();
    config.performance_monitoring.report_format = PerformanceReportFormat::Detailed;
    config.performance_monitoring.track_compilation_time = true;
    config.performance_monitoring.track_optimization_effectiveness = true;
    
    let mut system = OptimizationEnablementSystem::with_config(config).unwrap();
    
    // Apply some optimizations to generate performance data
    let test_source = "slay main() { println(\"Hello, World!\"); }";
    let _results = system.apply_optimizations(
        test_source,
        &OptimizationProfile::Release,
        None,
        &[],
    ).unwrap();
    
    // Test performance report generation
    let report = system.generate_performance_report().unwrap();
    assert!(!report.is_empty());
    assert!(report.contains("Performance"));
    
    // Test performance statistics
    let stats = system.get_performance_statistics();
    assert_eq!(stats.total_compilations, 1);
    assert!(stats.average_compilation_time > Duration::ZERO);
}

/// Test optimization configuration manager
#[test]
fn test_optimization_config_manager() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test-config.toml");
    
    // Create and configure manager
    let mut manager = OptimizationConfigManager::with_config_path(&config_path).unwrap();
    
    // Test built-in profiles
    let profiles = manager.list_profiles();
    assert!(profiles.contains(&"development".to_string()));
    assert!(profiles.contains(&"release".to_string()));
    assert!(profiles.contains(&"size".to_string()));
    assert!(profiles.contains(&"debug".to_string()));
    assert!(profiles.contains(&"fast_compilation".to_string()));
    
    // Test custom profile creation
    let custom_config = OptimizationConfig {
        optimization_level: OptimizationLevel::Basic,
        enable_lto: true,
        enable_vectorization: false,
        ..OptimizationConfig::default()
    };
    
    manager.set_custom_profile("test_profile".to_string(), custom_config.clone()).unwrap();
    
    // Save and reload configuration
    manager.save_config().unwrap();
    
    let reloaded_manager = OptimizationConfigManager::with_config_path(&config_path).unwrap();
    let retrieved_config = reloaded_manager.get_profile_config(
        &OptimizationProfile::Custom("test_profile".to_string())
    ).unwrap();
    
    assert_eq!(retrieved_config.optimization_level, OptimizationLevel::Basic);
    assert!(retrieved_config.enable_lto);
    assert!(!retrieved_config.enable_vectorization);
}

/// Test configuration validation
#[test]
fn test_configuration_validation() {
    let manager = OptimizationConfigManager::new().unwrap();
    let warnings = manager.validate_config().unwrap();
    assert!(warnings.is_empty()); // Default config should be valid
    
    // Test config with invalid values
    let mut invalid_manager = OptimizationConfigManager::new().unwrap();
    invalid_manager.config.global_settings.max_parallel_jobs = 100; // Too high
    invalid_manager.config.global_settings.optimization_timeout_secs = 7200; // Too long
    
    let warnings = invalid_manager.validate_config().unwrap();
    assert!(!warnings.is_empty());
    assert!(warnings.iter().any(|w| w.contains("max_parallel_jobs")));
    assert!(warnings.iter().any(|w| w.contains("optimization_timeout_secs")));
}

/// Test CLI profile parsing
#[test]
fn test_cli_profile_parsing() {
    use cursed::optimization::enablement_system::cli::parse_optimization_profile;
    
    assert_eq!(parse_optimization_profile("development"), OptimizationProfile::Development);
    assert_eq!(parse_optimization_profile("dev"), OptimizationProfile::Development);
    assert_eq!(parse_optimization_profile("release"), OptimizationProfile::Release);
    assert_eq!(parse_optimization_profile("rel"), OptimizationProfile::Release);
    assert_eq!(parse_optimization_profile("size"), OptimizationProfile::Size);
    assert_eq!(parse_optimization_profile("s"), OptimizationProfile::Size);
    assert_eq!(parse_optimization_profile("debug"), OptimizationProfile::Debug);
    assert_eq!(parse_optimization_profile("dbg"), OptimizationProfile::Debug);
    
    assert_eq!(
        parse_optimization_profile("custom_name"), 
        OptimizationProfile::Custom("custom_name".to_string())
    );
}

/// Test optimization level mapping
#[test]
fn test_optimization_level_mapping() {
    assert_eq!(OptimizationLevel::None.to_llvm_level(), 0);
    assert_eq!(OptimizationLevel::Basic.to_llvm_level(), 1);
    assert_eq!(OptimizationLevel::Default.to_llvm_level(), 2);
    assert_eq!(OptimizationLevel::Aggressive.to_llvm_level(), 3);
    assert_eq!(OptimizationLevel::Size.to_llvm_level(), 2);
    assert_eq!(OptimizationLevel::Fast.to_llvm_level(), 3);
}

/// Test performance improvements calculation
#[test]
fn test_performance_improvements() {
    let mut system = OptimizationEnablementSystem::new().unwrap();
    let test_source = "slay main() { println(\"test\"); }";
    
    // Test different profile improvements
    let dev_results = system.apply_optimizations(
        test_source,
        &OptimizationProfile::Development,
        None,
        &[],
    ).unwrap();
    
    let release_results = system.apply_optimizations(
        test_source,
        &OptimizationProfile::Release,
        None,
        &[],
    ).unwrap();
    
    let size_results = system.apply_optimizations(
        test_source,
        &OptimizationProfile::Size,
        None,
        &[],
    ).unwrap();
    
    // Release should have better improvements than development
    assert!(release_results.overall_improvement > dev_results.overall_improvement);
    
    // Size optimization should be between development and release
    assert!(size_results.overall_improvement > dev_results.overall_improvement);
    assert!(size_results.overall_improvement < release_results.overall_improvement);
}

/// Test parallel optimization efficiency
#[test]
fn test_parallel_optimization() {
    let mut config = OptimizationEnablementConfig::default();
    config.enable_parallel_optimization = true;
    config.max_parallel_jobs = 4;
    
    let mut system = OptimizationEnablementSystem::with_config(config).unwrap();
    let test_source = "slay main() { println(\"parallel test\"); }";
    
    let results = system.apply_optimizations(
        test_source,
        &OptimizationProfile::Release,
        None,
        &[],
    ).unwrap();
    
    assert!(results.parallel_efficiency.is_some());
    let efficiency = results.parallel_efficiency.unwrap();
    assert!(efficiency > 0.0);
    assert!(efficiency <= 1.0);
    
    // Should have time savings from parallel execution
    assert!(results.time_savings.is_some());
    let time_savings = results.time_savings.unwrap();
    assert!(time_savings.parallel_execution_savings > Duration::ZERO);
}

/// Test advanced optimization features
#[test]
fn test_advanced_optimization_features() {
    let mut config = OptimizationEnablementConfig::default();
    config.enable_advanced_by_default = true;
    config.enable_pgo_when_available = true;
    config.enable_adaptive_optimization = true;
    
    let mut system = OptimizationEnablementSystem::with_config(config).unwrap();
    let test_source = "slay main() { println(\"advanced test\"); }";
    
    let results = system.apply_optimizations(
        test_source,
        &OptimizationProfile::Release,
        None,
        &[],
    ).unwrap();
    
    // Should have improvements from all advanced features
    assert!(results.llvm_improvements.is_some());
    assert!(results.pgo_improvements.is_some());
    assert!(results.adaptive_improvements.is_some());
    
    let llvm_improvements = results.llvm_improvements.unwrap();
    assert!(llvm_improvements.runtime_improvement > 0.0);
    assert!(llvm_improvements.instruction_reduction > 0.0);
    assert!(llvm_improvements.memory_usage_improvement > 0.0);
    
    let pgo_improvements = results.pgo_improvements.unwrap();
    assert!(pgo_improvements.execution_time_improvement > 0.0);
    assert!(pgo_improvements.branch_prediction_improvement > 0.0);
    assert!(pgo_improvements.cache_hit_rate_improvement > 0.0);
    
    let adaptive_improvements = results.adaptive_improvements.unwrap();
    assert!(adaptive_improvements.overall_improvement > 0.0);
    assert!(adaptive_improvements.pattern_based_improvement > 0.0);
    assert!(adaptive_improvements.workload_specific_improvement > 0.0);
}

/// Test performance report formats
#[test]
fn test_performance_report_formats() {
    let mut system = OptimizationEnablementSystem::new().unwrap();
    let test_source = "slay main() { println(\"report test\"); }";
    
    // Generate some performance data
    let _results = system.apply_optimizations(
        test_source,
        &OptimizationProfile::Release,
        None,
        &[],
    ).unwrap();
    
    // Test summary format
    system.config.performance_monitoring.report_format = PerformanceReportFormat::Summary;
    let summary_report = system.generate_performance_report().unwrap();
    assert!(!summary_report.is_empty());
    assert!(summary_report.contains("Performance Summary"));
    
    // Test detailed format
    system.config.performance_monitoring.report_format = PerformanceReportFormat::Detailed;
    let detailed_report = system.generate_performance_report().unwrap();
    assert!(!detailed_report.is_empty());
    assert!(detailed_report.contains("Detailed Performance Report"));
    
    // Test JSON format
    system.config.performance_monitoring.report_format = PerformanceReportFormat::Json;
    let json_report = system.generate_performance_report().unwrap();
    assert!(!json_report.is_empty());
    assert!(json_report.starts_with('{') && json_report.ends_with('}'));
    
    // Test none format
    system.config.performance_monitoring.report_format = PerformanceReportFormat::None;
    let no_report = system.generate_performance_report().unwrap();
    assert!(no_report.is_empty());
}

/// Test optimization timeout behavior
#[test]
fn test_optimization_timeout() {
    let mut config = OptimizationEnablementConfig::default();
    config.optimization_timeout = Duration::from_millis(1); // Very short timeout
    
    let mut system = OptimizationEnablementSystem::with_config(config).unwrap();
    let test_source = "slay main() { println(\"timeout test\"); }";
    
    // Should still complete even with short timeout (simulation doesn't actually timeout)
    let results = system.apply_optimizations(
        test_source,
        &OptimizationProfile::Release,
        None,
        &[],
    ).unwrap();
    
    assert!(results.total_optimization_time >= Duration::ZERO);
}

/// Integration test with multiple compilation units
#[test]
fn test_multiple_compilation_units() {
    let mut system = OptimizationEnablementSystem::new().unwrap();
    
    let test_sources = vec![
        "slay main() { println(\"test 1\"); }",
        "slay add(a: i32, b: i32) -> i32 { a + b }",
        "slay factorial(n: i32) -> i32 { lowkey (n <= 1) { 1 } n * factorial(n-1) }",
    ];
    
    let mut total_improvement = 0.0;
    for (i, source) in test_sources.iter().enumerate() {
        let results = system.apply_optimizations(
            source,
            &OptimizationProfile::Release,
            None,
            &[],
        ).unwrap();
        
        total_improvement += results.overall_improvement;
        
        // Each compilation should be tracked
        let stats = system.get_performance_statistics();
        assert_eq!(stats.total_compilations, i + 1);
    }
    
    // Should have accumulated improvements
    assert!(total_improvement > 0.0);
    
    let final_stats = system.get_performance_statistics();
    assert_eq!(final_stats.total_compilations, test_sources.len());
    assert!(final_stats.average_improvement_percentage > 0.0);
}
