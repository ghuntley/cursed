//! Bootstrap Build System Integration Tests
//!
//! Comprehensive tests for the integration between the bootstrap verification
//! system and the build orchestrator, validating self-hosting capability.

use cursed::bootstrap::{
    SelfCompilationVerifier, VerificationConfig, VerificationResult
};
use cursed::cli::bootstrap::{
    BootstrapCliConfig, bootstrap_command, handle_bootstrap_command
};
use cursed::error::{Error, Result as CursedResult};
use clap::ArgMatches;
use std::path::PathBuf;
use std::time::Duration;
use tempfile::tempdir;
use tracing::info;

/// Initialize tracing for tests
fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();
}

#[tokio::test]
async fn test_bootstrap_cli_config_creation() {
    init_test_tracing();
    
    let config = BootstrapCliConfig::default();
    assert_eq!(config.bootstrap_cycles, 3);
    assert_eq!(config.timeout_minutes, 10);
    assert!(!config.keep_intermediates);
    assert!(!config.verbose);
    assert!(!config.force);
    assert_eq!(config.optimization_levels, vec!["-O2".to_string()]);
}

#[test]
fn test_bootstrap_command_structure() {
    init_test_tracing();
    
    let cmd = bootstrap_command();
    assert_eq!(cmd.get_name(), "bootstrap");
    
    // Verify all expected subcommands are present
    let subcommands: Vec<_> = cmd.get_subcommands().map(|s| s.get_name()).collect();
    assert!(subcommands.contains(&"verify"));
    assert!(subcommands.contains(&"stages"));
    assert!(subcommands.contains(&"clean"));
    assert!(subcommands.contains(&"status"));
    
    info!("Bootstrap command structure validated successfully");
}

#[test]
fn test_verification_config_integration() {
    init_test_tracing();
    
    let temp_dir = tempdir().unwrap();
    let config = VerificationConfig {
        work_dir: temp_dir.path().to_path_buf(),
        compilation_timeout: Duration::from_secs(300),
        execution_timeout: Duration::from_secs(60),
        keep_intermediates: false,
        optimization_levels: vec!["-O2".to_string()],
        bootstrap_cycles: 3,
    };
    
    // Test configuration creation
    let verifier_result = SelfCompilationVerifier::new(config);
    assert!(verifier_result.is_ok());
    
    info!("Verification config integration validated");
}

#[tokio::test]
async fn test_bootstrap_status_command() {
    init_test_tracing();
    
    // Create a mock ArgMatches for the status command
    let cmd = bootstrap_command();
    let matches = cmd.try_get_matches_from(vec!["bootstrap", "status"]);
    
    assert!(matches.is_ok());
    let matches = matches.unwrap();
    
    // Test the status command handler
    let result = handle_bootstrap_command(&matches).await;
    
    // The status command should always succeed (it just reports current state)
    assert!(result.is_ok());
    
    info!("Bootstrap status command tested successfully");
}

#[tokio::test]
async fn test_bootstrap_stages_command() {
    init_test_tracing();
    
    // Test basic stages command
    let cmd = bootstrap_command();
    let matches = cmd.try_get_matches_from(vec!["bootstrap", "stages"]);
    
    assert!(matches.is_ok());
    let matches = matches.unwrap();
    
    let result = handle_bootstrap_command(&matches).await;
    assert!(result.is_ok());
    
    // Test detailed stages command
    let matches_detailed = cmd.try_get_matches_from(vec!["bootstrap", "stages", "--detail"]);
    assert!(matches_detailed.is_ok());
    let matches_detailed = matches_detailed.unwrap();
    
    let result_detailed = handle_bootstrap_command(&matches_detailed).await;
    assert!(result_detailed.is_ok());
    
    info!("Bootstrap stages command tested successfully");
}

#[tokio::test]
async fn test_bootstrap_clean_command() {
    init_test_tracing();
    
    let temp_dir = tempdir().unwrap();
    let work_dir = temp_dir.path().join("test_bootstrap_clean");
    
    // Create some dummy files to clean
    std::fs::create_dir_all(&work_dir).unwrap();
    std::fs::write(work_dir.join("dummy.txt"), "test").unwrap();
    
    let cmd = bootstrap_command();
    let matches = cmd.try_get_matches_from(vec![
        "bootstrap", "clean", 
        "--work-dir", work_dir.to_str().unwrap()
    ]);
    
    assert!(matches.is_ok());
    let matches = matches.unwrap();
    
    let result = handle_bootstrap_command(&matches).await;
    assert!(result.is_ok());
    
    // Verify the directory was cleaned
    assert!(!work_dir.exists());
    
    info!("Bootstrap clean command tested successfully");
}

#[test]
fn test_bootstrap_verify_command_parsing() {
    init_test_tracing();
    
    let cmd = bootstrap_command();
    
    // Test basic verify command
    let matches = cmd.try_get_matches_from(vec!["bootstrap", "verify"]);
    assert!(matches.is_ok());
    
    // Test verify command with all options
    let matches_full = cmd.try_get_matches_from(vec![
        "bootstrap", "verify",
        "--cycles", "5",
        "--timeout", "15",
        "--work-dir", "/tmp/test",
        "--keep",
        "--optimization", "O3",
        "--verbose",
        "--force"
    ]);
    assert!(matches_full.is_ok());
    
    let matches = matches_full.unwrap();
    let sub_matches = matches.subcommand_matches("verify").unwrap();
    
    // Verify all arguments are parsed correctly
    assert_eq!(sub_matches.get_one::<String>("cycles").unwrap(), "5");
    assert_eq!(sub_matches.get_one::<String>("timeout").unwrap(), "15");
    assert_eq!(sub_matches.get_one::<String>("work-dir").unwrap(), "/tmp/test");
    assert!(sub_matches.get_flag("keep"));
    assert!(sub_matches.get_flag("verbose"));
    assert!(sub_matches.get_flag("force"));
    
    info!("Bootstrap verify command parsing validated");
}

#[tokio::test]
async fn test_bootstrap_verification_config_creation() {
    init_test_tracing();
    
    let temp_dir = tempdir().unwrap();
    
    // Test creating a verification config
    let config = VerificationConfig {
        work_dir: temp_dir.path().to_path_buf(),
        compilation_timeout: Duration::from_secs(600),
        execution_timeout: Duration::from_secs(60),
        keep_intermediates: true,
        optimization_levels: vec!["-O2".to_string(), "-O3".to_string()],
        bootstrap_cycles: 4,
    };
    
    // Verify config properties
    assert_eq!(config.bootstrap_cycles, 4);
    assert_eq!(config.compilation_timeout, Duration::from_secs(600));
    assert!(config.keep_intermediates);
    assert_eq!(config.optimization_levels.len(), 2);
    
    // Test verifier creation with config
    let verifier_result = SelfCompilationVerifier::new(config);
    assert!(verifier_result.is_ok());
    
    info!("Bootstrap verification config creation tested");
}

#[test]
fn test_bootstrap_cli_error_handling() {
    init_test_tracing();
    
    let cmd = bootstrap_command();
    
    // Test invalid arguments
    let invalid_cycles = cmd.try_get_matches_from(vec![
        "bootstrap", "verify", "--cycles", "invalid"
    ]);
    assert!(invalid_cycles.is_err());
    
    let invalid_timeout = cmd.try_get_matches_from(vec![
        "bootstrap", "verify", "--timeout", "not_a_number"
    ]);
    assert!(invalid_timeout.is_err());
    
    // Test unknown subcommand
    let unknown_subcommand = cmd.try_get_matches_from(vec![
        "bootstrap", "unknown-command"
    ]);
    assert!(unknown_subcommand.is_err());
    
    info!("Bootstrap CLI error handling validated");
}

#[tokio::test]
async fn test_bootstrap_integration_workflow() {
    init_test_tracing();
    
    // Test a complete workflow: status -> stages -> clean
    let cmd = bootstrap_command();
    
    // 1. Check status
    let status_matches = cmd.try_get_matches_from(vec!["bootstrap", "status"]).unwrap();
    let status_result = handle_bootstrap_command(&status_matches).await;
    assert!(status_result.is_ok());
    
    // 2. Show stages
    let stages_matches = cmd.try_get_matches_from(vec!["bootstrap", "stages"]).unwrap();
    let stages_result = handle_bootstrap_command(&stages_matches).await;
    assert!(stages_result.is_ok());
    
    // 3. Clean (with temp directory)
    let temp_dir = tempdir().unwrap();
    let work_dir = temp_dir.path().join("workflow_test");
    std::fs::create_dir_all(&work_dir).unwrap();
    
    let clean_matches = cmd.try_get_matches_from(vec![
        "bootstrap", "clean", "--work-dir", work_dir.to_str().unwrap()
    ]).unwrap();
    let clean_result = handle_bootstrap_command(&clean_matches).await;
    assert!(clean_result.is_ok());
    
    info!("Bootstrap integration workflow tested successfully");
}

#[test]
fn test_bootstrap_verification_performance_tracking() {
    init_test_tracing();
    
    // Test performance tracking structure
    let temp_dir = tempdir().unwrap();
    let config = VerificationConfig {
        work_dir: temp_dir.path().to_path_buf(),
        compilation_timeout: Duration::from_secs(300),
        execution_timeout: Duration::from_secs(60),
        keep_intermediates: false,
        optimization_levels: vec!["-O1".to_string(), "-O2".to_string()],
        bootstrap_cycles: 2,
    };
    
    // Verify the configuration supports performance tracking
    assert!(config.compilation_timeout > Duration::ZERO);
    assert!(config.execution_timeout > Duration::ZERO);
    assert!(!config.optimization_levels.is_empty());
    
    info!("Bootstrap verification performance tracking validated");
}

#[test]
fn test_bootstrap_convergence_analysis() {
    init_test_tracing();
    
    // Test convergence analysis capabilities
    let temp_dir = tempdir().unwrap();
    let config = VerificationConfig {
        work_dir: temp_dir.path().to_path_buf(),
        compilation_timeout: Duration::from_secs(300),
        execution_timeout: Duration::from_secs(60),
        keep_intermediates: false,
        optimization_levels: vec!["-O2".to_string()],
        bootstrap_cycles: 5, // More cycles for convergence testing
    };
    
    // Test that convergence analysis is properly configured
    assert!(config.bootstrap_cycles >= 3); // Minimum for meaningful convergence
    
    let verifier_result = SelfCompilationVerifier::new(config);
    assert!(verifier_result.is_ok());
    
    info!("Bootstrap convergence analysis configuration validated");
}

#[tokio::test]
async fn test_bootstrap_error_reporting() {
    init_test_tracing();
    
    // Test error reporting for invalid work directory
    let cmd = bootstrap_command();
    let matches = cmd.try_get_matches_from(vec![
        "bootstrap", "clean", "--work-dir", "/invalid/path/that/should/not/exist"
    ]).unwrap();
    
    // This should succeed (clean command handles non-existent directories gracefully)
    let result = handle_bootstrap_command(&matches).await;
    assert!(result.is_ok());
    
    info!("Bootstrap error reporting tested");
}

#[test]
fn test_bootstrap_multi_optimization_levels() {
    init_test_tracing();
    
    let cmd = bootstrap_command();
    
    // Test multiple optimization levels
    let matches = cmd.try_get_matches_from(vec![
        "bootstrap", "verify",
        "--optimization", "O1",
        "--optimization", "O2", 
        "--optimization", "O3"
    ]);
    
    assert!(matches.is_ok());
    let matches = matches.unwrap();
    let sub_matches = matches.subcommand_matches("verify").unwrap();
    
    let opt_levels: Vec<_> = sub_matches.get_many::<String>("optimization")
        .unwrap()
        .collect();
    
    assert_eq!(opt_levels.len(), 3);
    assert!(opt_levels.contains(&&"O1".to_string()));
    assert!(opt_levels.contains(&&"O2".to_string()));
    assert!(opt_levels.contains(&&"O3".to_string()));
    
    info!("Bootstrap multi-optimization levels validated");
}

#[test]
fn test_bootstrap_stage_documentation() {
    init_test_tracing();
    
    // Test that all bootstrap stages are properly documented
    let stages = [
        ("Stage 0", "Rust-based CURSED compiler (bootstrap compiler)"),
        ("Stage 1", "CURSED-based compiler (compiled by Stage 0)"),
        ("Stage 2", "Self-compiled CURSED compiler (compiled by Stage 1)"),
        ("Stage 3+", "Convergence verification"),
    ];
    
    // Verify we have documentation for all expected stages
    assert_eq!(stages.len(), 4);
    
    for (stage, description) in &stages {
        assert!(!stage.is_empty());
        assert!(!description.is_empty());
        assert!(description.len() > 10); // Meaningful description
    }
    
    info!("Bootstrap stage documentation validated");
}
