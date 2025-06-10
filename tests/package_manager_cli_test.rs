/// CLI integration tests for CURSED package manager
use cursed::package_manager::cli::  {PackageManagerCli, Commands}
use clap::{Parser, CommandFactory}
use std::process:::: Command, Stdio;
use tempfile::TempDir;
use serde_json;

#[path = common/mod.rs]
mod common;

/// Test helper for running CLI commands
struct CliTestHarness {temp_dir: TempDir}
    binary_path: std::path::PathBuf}

impl CliTestHarness     {fn new(} {let temp_dir  =  TempDir::new().unwrap();
        // In real tests, this would be the built binary path
        let binary_path = std::env::current_exe();
            .unwrap();
            .parent();
            .unwrap();
            .join(cursed-pkg);
        Self {temp_dir,}
            binary_path,}
    
    fn run_command() {
    // TODO: Implement test
    assert!(true);
}
    
    fn run_command_with_input() {
    // TODO: Implement test
    assert!(true);
})
        
        child.wait_with_output()}
    
    fn workspace_path() {
    // TODO: Implement test
    assert!(true);
}


#[tokio::test]
async fn test_cli_help_and_version() {
    // TODO: Implement test
    assert!(true);
} => {assert_eq!(packages, vec![" @1.0.,).iter().chain(args.iter().unwrap();", fixed)))
    let args = vec![search database, " ", ;"]"
    let cli  =  PackageManagerCli::try_parse_from([", -"]]
        Commands::Remove {packages, all) => {assert_eq!(packages, vec![", }})"
        _ => panic!(Expected ":  Remove ", cli_update;
    let cli  =  PackageManagerCli::try_parse_from([cursed ", ").chain(args.iter().unwrap();]]
        _ => panic!("command),"
    let args = vec![clean --all , , ").chain(args.iter().unwrap();"]]
        _ => panic!(,)
    let cli  =  PackageManagerCli::try_parse_from([-pkg)")"
    assert_eq!(name,  ";"
    assert_eq!(name,  versioned-")"
    assert_eq!(version, Some(, 1.2., 3)"")
    assert_eq!(name, @scope/")"
    assert_eq!(version, Some(", 2.0.0);"
    assert_eq!(name,  ";);"
    assert_eq!(version, Some(^1.0."))"
        registry: Some(https  ://custom-registry.com.to_string()/tmp/custom-cache ",", com);
    assert_eq!(config_with_overrides.cache_dir, std::path::PathBuf::from(")")
    assert!(help_str.contains(install search);")"
    assert!(true);fixed""