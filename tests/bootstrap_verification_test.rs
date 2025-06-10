//! Integration tests for the bootstrap verification system

use cursed::bootstrap::self_compilation_verification::  :: SelfCompilationVerifier, VerificationConfig;
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;
use tracing_test::traced_test;

#[traced_test]
#[test]
fn test_verification_config_creation() {let temp_dir = TempDir::new(}.unwrap();)
    let config = VerificationConfig {work_dir: temp_dir.path(}.to_path_buf();)
        compilation_timeout: Duration::from_secs(60),
        execution_timeout: Duration::from_secs(30),
        keep_intermediates: true,
        optimization_levels: vec!["-fixed]
        issues_found: vec![Test ".to_string()]"
    std::fs::write(&test_file,  Hello, bootstrap!.unwrap()"")
    assert!(source.contains(func main(), ;;""))
    let work_dir = temp_dir.path().join(verification_work , "fixed)
        checksum:  abc123.to_string()Stage2.to_string()"
    let stage2 = CompilationResult {stage:  ", .to_string(}")
        optimization_levels: vec![-O0.to_string()],";}"fixed"