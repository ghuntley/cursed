//! Comprehensive tests for the CURSED optimization system
//! 
//! This module contains tests for all optimization passes and functionality.

use super::*;
use crate::error::Result;
use std::time::Duration;
use std::collections::HashMap;

/// Test optimization configuration
#[test]
fn test_optimization_config() {
    let config = OptimizationConfig::new(OptimizationLevel::Aggressive);
    assert_eq!(config.level, OptimizationLevel::Aggressive);
    assert!(config.vectorize);
    assert!(config.parallel_codegen);
    
    let debug_config = OptimizationConfig::debug();
    assert_eq!(debug_config.level, OptimizationLevel::None);
    assert!(debug_config.debug_info);
    assert!(!debug_config.lto);
    
    let release_config = OptimizationConfig::release();
    assert_eq!(release_config.level, OptimizationLevel::Aggressive);
    assert!(!release_config.debug_info);
    assert!(release_config.lto);
}

/// Test optimization level conversion
#[test]
fn test_optimization_level_conversion() {
    assert_eq!(OptimizationLevel::from("0"), OptimizationLevel::None);
    assert_eq!(OptimizationLevel::from("1"), OptimizationLevel::Less);
    assert_eq!(OptimizationLevel::from("2"), OptimizationLevel::Default);
    assert_eq!(OptimizationLevel::from("3"), OptimizationLevel::Aggressive);
    assert_eq!(OptimizationLevel::from("s"), OptimizationLevel::Size);
    assert_eq!(OptimizationLevel::from("z"), OptimizationLevel::SizeZ);
}

/// Test optimization profiles
#[test]
fn test_optimization_profiles() {
    let dev_profile = OptimizationProfile::development();
    assert_eq!(dev_profile.name, "Development");
    assert!(dev_profile.estimated_build_time_factor < 1.0);
    
    let prod_profile = OptimizationProfile::production();
    assert_eq!(prod_profile.name, "Production");
    assert!(prod_profile.estimated_performance_gain > 1.0);
    
    let size_profile = OptimizationProfile::size_optimized();
    assert_eq!(size_profile.name, "Size");
    assert!(size_profile.is_suitable_for("embedded"));
    
    let balanced_profile = OptimizationProfile::balanced();
    assert_eq!(balanced_profile.name, "Balanced");
    assert!(balanced_profile.is_suitable_for("general"));
}

/// Test CLI optimization level parsing
#[test]
fn test_cli_optimization_level_parsing() -> Result<()> {
    let config_0 = OptimizationConfig::from_cli_level("0")?;
    assert_eq!(config_0.level, OptimizationLevel::None);
    
    let config_1 = OptimizationConfig::from_cli_level("1")?;
    assert_eq!(config_1.level, OptimizationLevel::Less);
    
    let config_2 = OptimizationConfig::from_cli_level("2")?;
    assert_eq!(config_2.level, OptimizationLevel::Default);
    
    let config_3 = OptimizationConfig::from_cli_level("3")?;
    assert_eq!(config_3.level, OptimizationLevel::Aggressive);
    
    let config_s = OptimizationConfig::from_cli_level("s")?;
    assert_eq!(config_s.level, OptimizationLevel::Size);
    
    let config_z = OptimizationConfig::from_cli_level("z")?;
    assert_eq!(config_z.level, OptimizationLevel::SizeZ);
    
    // Test invalid level
    let result = OptimizationConfig::from_cli_level("invalid");
    assert!(result.is_err());
    
    Ok(())
}

/// Test benchmarking configuration
#[test]
fn test_benchmarking_config() {
    let config = OptimizationConfig::for_benchmarking();
    assert_eq!(config.level, OptimizationLevel::Aggressive);
    assert!(config.vectorize);
    assert!(config.lto);
    assert!(config.enable_profiling);
    assert!(config.parallel_workers > 16);
    assert!(config.max_cache_size > 4 * 1024 * 1024 * 1024); // > 4GB
}

/// Test optimization configuration validation
#[test]
fn test_optimization_config_validation() {
    let mut config = OptimizationConfig::default();
    assert!(config.validate().is_ok());
    
    // Test invalid inline threshold
    config.inline_threshold = 20000;
    assert!(config.validate().is_err());
    
    // Test invalid unroll threshold
    config.inline_threshold = 100; // Reset
    config.unroll_threshold = 2000;
    assert!(config.validate().is_err());
    
    // Test empty target feature
    config.unroll_threshold = 100; // Reset
    config.target_features.push("".to_string());
    assert!(config.validate().is_err());
    
    // Test empty pass name
    config.target_features.clear(); // Reset
    config.custom_passes.push("".to_string());
    assert!(config.validate().is_err());
}

/// Test optimization flags generation
#[test]
fn test_optimization_flags_generation() {
    let config_o0 = OptimizationConfig::new(OptimizationLevel::None);
    let flags_o0 = config_o0.get_optimization_flags();
    assert!(flags_o0.contains(&"-O0".to_string()));
    
    let config_o3 = OptimizationConfig::new(OptimizationLevel::Aggressive);
    let flags_o3 = config_o3.get_optimization_flags();
    assert!(flags_o3.contains(&"-O3".to_string()));
    
    let mut config_lto = OptimizationConfig::new(OptimizationLevel::Default);
    config_lto.enable_lto();
    let flags_lto = config_lto.get_optimization_flags();
    assert!(flags_lto.contains(&"-flto".to_string()));
    
    let mut config_debug = OptimizationConfig::new(OptimizationLevel::Default);
    config_debug.enable_debug_info();
    let flags_debug = config_debug.get_optimization_flags();
    assert!(flags_debug.contains(&"-g".to_string()));
}

/// Test inlining and unrolling decisions
#[test]
fn test_inlining_and_unrolling_decisions() {
    let config_none = OptimizationConfig::new(OptimizationLevel::None);
    assert!(!config_none.should_inline(100));
    assert!(!config_none.should_unroll(50));
    
    let config_aggressive = OptimizationConfig::new(OptimizationLevel::Aggressive);
    assert!(config_aggressive.should_inline(100));
    assert!(config_aggressive.should_unroll(50));
    
    let config_size = OptimizationConfig::new(OptimizationLevel::Size);
    assert!(config_size.should_inline(25)); // Small function
    assert!(!config_size.should_inline(300)); // Large function
    assert!(!config_size.should_unroll(50)); // No unrolling for size
}

/// Test environment-based configuration
#[test]
fn test_environment_based_configuration() {
    // Test default (development) environment
    let config_dev = OptimizationConfig::for_development();
    assert_eq!(config_dev.level, OptimizationLevel::Less);
    assert!(config_dev.debug_info);
    assert!(!config_dev.lto);
    
    // Test production configuration
    let config_prod = OptimizationConfig::for_production();
    assert_eq!(config_prod.level, OptimizationLevel::Aggressive);
    assert!(!config_prod.debug_info);
    assert!(config_prod.lto);
    assert!(config_prod.profile_guided);
}

/// Test cache configuration
#[test]
fn test_cache_configuration() {
    let config = OptimizationConfig::default();
    assert!(config.is_cache_enabled());
    assert!(config.cache_size_mb() > 0);
    
    let mut config_no_cache = OptimizationConfig::default();
    config_no_cache.set_max_cache_size(0);
    assert!(!config_no_cache.is_cache_enabled());
    
    let mut config_large_cache = OptimizationConfig::default();
    config_large_cache.set_max_cache_size(8 * 1024 * 1024 * 1024); // 8GB
    assert_eq!(config_large_cache.cache_size_mb(), 8192);
}

/// Test workspace configuration
#[test]
fn test_workspace_configuration() {
    let mut config = OptimizationConfig::default();
    config.set_workspace_dir("/tmp/test_workspace".to_string());
    
    assert_eq!(config.workspace_dir, "/tmp/test_workspace");
    assert_eq!(config.workspace_path(), std::path::PathBuf::from("/tmp/test_workspace"));
}

/// Test parallel worker configuration
#[test]
fn test_parallel_worker_configuration() {
    let mut config = OptimizationConfig::default();
    config.parallel_workers = 8;
    config.enable_parallel = true;
    assert_eq!(config.effective_workers(), 8);
    
    config.enable_parallel = false;
    assert_eq!(config.effective_workers(), 1);
}

/// Test argument parsing
#[test]
fn test_argument_parsing() -> Result<()> {
    let args = vec!["--optimize".to_string()];
    let config = OptimizationConfig::from_args(&args)?;
    assert_eq!(config.level, OptimizationLevel::Default);
    
    let args = vec!["--opt-level=3".to_string()];
    let config = OptimizationConfig::from_args(&args)?;
    assert_eq!(config.level, OptimizationLevel::Aggressive);
    
    let args = vec!["--debug".to_string()];
    let config = OptimizationConfig::from_args(&args)?;
    assert_eq!(config.level, OptimizationLevel::None);
    assert!(config.debug_info);
    
    Ok(())
}

/// Test end-to-end optimization with CURSED code
#[test]
fn test_end_to_end_optimization() -> Result<()> {
    // Create a temporary directory for testing with unique name
    let temp_dir = std::env::temp_dir().join(format!("cursed_test_optimization_{}", std::process::id()));
    if temp_dir.exists() {
        std::fs::remove_dir_all(&temp_dir).ok();
    }
    std::fs::create_dir_all(&temp_dir).unwrap();
    
    // Create required subdirectories that the package manager expects
    std::fs::create_dir_all(&temp_dir.join("target").join("packages")).unwrap();
    std::fs::create_dir_all(&temp_dir.join("target").join("temp")).unwrap();
    std::fs::create_dir_all(&temp_dir.join("target").join("packages").join("installed")).unwrap();
    
    // Set up a temporary working directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    
    let source = r#"
        vibez.spill("Testing optimization")
        sus x normie = 10 + 20
        sus y normie = x * 2
        vibez.spill("Result: " + y)
    "#;
    
    // Test different optimization levels
    let result = (|| -> Result<()> {
        for level in &["0", "1", "2", "3"] {
            let ir = crate::compile_to_ir_with_optimization(source, Some(level))?;
            
            // Verify IR was generated
            assert!(!ir.is_empty());
            
            println!("✓ Optimization level {} generated IR successfully", level);
        }
        Ok(())
    })();
    
    // Cleanup: return to original directory
    std::env::set_current_dir(original_dir).ok();
    std::fs::remove_dir_all(&temp_dir).ok();
    
    result
}

/// Test optimization performance characteristics
#[test]
fn test_optimization_performance_characteristics() {
    let profiles = OptimizationProfile::all_profiles();
    assert_eq!(profiles.len(), 4);
    
    // Development profile should be fastest to compile
    let dev_profile = profiles.iter().find(|p| p.name == "Development").unwrap();
    assert!(dev_profile.estimated_build_time_factor < 1.0);
    
    // Production profile should have best performance
    let prod_profile = profiles.iter().find(|p| p.name == "Production").unwrap();
    assert!(prod_profile.estimated_performance_gain > 1.5);
    
    // Size profile should be balanced
    let size_profile = profiles.iter().find(|p| p.name == "Size").unwrap();
    assert!(size_profile.estimated_performance_gain >= 1.0);
}

/// Test profile finding by name
#[test]
fn test_profile_finding_by_name() {
    assert!(OptimizationProfile::by_name("development").is_some());
    assert!(OptimizationProfile::by_name("dev").is_some());
    assert!(OptimizationProfile::by_name("production").is_some());
    assert!(OptimizationProfile::by_name("prod").is_some());
    assert!(OptimizationProfile::by_name("size").is_some());
    assert!(OptimizationProfile::by_name("min").is_some());
    assert!(OptimizationProfile::by_name("balanced").is_some());
    assert!(OptimizationProfile::by_name("default").is_some());
    assert!(OptimizationProfile::by_name("nonexistent").is_none());
}

/// Test LLVM optimization config conversion
#[test]
fn test_llvm_optimization_config_conversion() {
    let config = OptimizationConfig::new(OptimizationLevel::Aggressive);
    let llvm_config = config.to_llvm_config();
    
    // Test conversion works
    assert!(llvm_config.vectorize_loops);
    assert!(llvm_config.inline_functions);
    assert!(llvm_config.enable_parallel_optimization);
    assert!(llvm_config.enable_caching);
}
